"""
CLI tool for running simple shadow simulations.

Can be executed as `shadow-exec` after installing the package, or without
installing e.g. as
`PYTHONPATH=/reporoot/shadowtools/src python3 -m shadowtools.shadow_exec`.

Examples:

```
$ shadow-exec date
Sat Jan  1 00:00:00 GMT 2000
```

```
$ shadow-exec -- bash -c 'date; sleep 1000; date'
Sat Jan  1 00:00:00 GMT 2000
Sat Jan  1 00:16:40 GMT 2000
```
"""

import argparse
import enum
import subprocess
import shlex
import shutil
import sys
import tempfile
import textwrap
import yaml

from pathlib import Path
from typing import TextIO, BinaryIO, Final, Optional, List

import shadowtools.config as scfg


class PreserveChoice(enum.Enum):
    ALWAYS = enum.auto()
    NEVER = enum.auto()
    ON_ERROR = enum.auto()


def _main(
    progname: str,
    args: List[str],
    preserve: PreserveChoice = PreserveChoice.NEVER,
    temp_dir: Optional[Path] = None,
    stdout: BinaryIO = sys.stdout.buffer,
    stderr: TextIO = sys.stderr,
    shadow_bin: Path = Path("shadow"),
    model_unblocked_syscall_latency: bool = True,
) -> int:
    """
    Run a program under shadow.

    args:
    progname -- String prefix to use for output originating from this function.
    args -- List of arguments of program to be run under shadow.
    preserve -- Whether to save the temporary directory containing the raw
                simulation config and results.
    stdout -- Destination for the simulated program's merged stdout and stderr.
    stderr -- Destination for other "meta" output.
    shadow_bin -- Shadow binary basename or path.
    model_unblocked_syscall_latency -- Whether to set shadow's --model-unblocked-syscall-latency.
    """

    tmpdir = Path(tempfile.mkdtemp(prefix=f"{progname}-", dir=temp_dir))

    wrapper_script = textwrap.dedent(
        f"""
        set -euo pipefail

        # Change back to host working dir
        cd {shlex.quote(str(Path('.').resolve()))}

        # Run specified command, merging stderr to stdout
        exec {shlex.join(args)} 2>&1
        """
    )

    config = scfg.Config(
        general=scfg.General(
            # It'd be nice to set a higher stop-time here, but some simulations
            # (chutney) take a long time to fast-forward empty time after all
            # processes have exited.
            # TODO: investigate why this is and/or add a shadow feature to stop
            # early if all processes have exited.
            stop_time="100h",
            log_level="warning",
            heartbeat_interval=None,
            model_unblocked_syscall_latency=model_unblocked_syscall_latency,
        ),
        network=scfg.Network(graph=scfg.Graph(type="1_gbit_switch")),
        hosts={
            "host": scfg.Host(
                network_node_id=0,
                processes=[
                    scfg.Process(
                        path="bash",
                        args=[
                            "-c",
                            wrapper_script,
                        ],
                    )
                ],
            )
        },
    )
    config_path = tmpdir.joinpath("shadow.yaml")
    config_path.write_text(yaml.safe_dump(config))

    data_dir = tmpdir.joinpath("shadow.data")
    shadow_args = [
        f"--data-directory={data_dir}",
        "--progress=false",
        str(config_path),
    ]

    shadow_stdout_path = tmpdir.joinpath("shadow.stdout")
    shadow_stderr_path = tmpdir.joinpath("shadow.stderr")
    with shadow_stdout_path.open("w") as shadow_stdout_file, shadow_stderr_path.open(
        "w"
    ) as shadow_stderr_file:
        shadow_ps = subprocess.Popen(
            [str(shadow_bin)] + shadow_args,
            stdout=shadow_stdout_file,
            stderr=shadow_stderr_file,
        )
        simulated_stdout_file = None
        shadow_exited = False
        while True:
            processed_data = False

            # Try opening the simulated process's stdout if we haven't
            # successfully done so yet.
            if simulated_stdout_file is None:
                try:
                    simulated_stdout_file = open(
                        str(data_dir.joinpath("hosts/host/bash.1000.stdout")), "rb"
                    )
                except FileNotFoundError:
                    # Not created yet, presumably
                    pass

            # Pump data from sim stdout to our stdout
            data = None
            if simulated_stdout_file is not None:
                # Fairly arbitrary, but might as well avoid excessive memory
                # usage here.
                bufsize = 1_000_000
                data = simulated_stdout_file.read(bufsize)
            while data:
                processed_data = True
                count = stdout.write(data)
                data = data[count:]

            if not processed_data and shadow_exited:
                # Done
                break

            if not processed_data:
                # No data ready to handle right now.

                # Flush anything we've pumped so far.
                stdout.flush()
                try:
                    # Wait a bit for shadow to exit.
                    # We want this to be long enough to avoid burning CPU
                    # cycles, but short enough to keep latency of pumping data
                    # to stdout low.
                    timeout_secs = 1
                    shadow_ps.wait(timeout_secs)
                    # If we get here, then shadow exited.
                    # Mark as exited, but loop again in case more output has
                    # been written.
                    shadow_exited = True
                except subprocess.TimeoutExpired:
                    # shadow didn't exit. Loop to see if there's more data to
                    # process.
                    pass
    # if shadow failed, dump its stderr
    if shadow_ps.returncode:
        shadow_stderr = shadow_stderr_path.read_text()
        while shadow_stderr:
            count = stderr.write(textwrap.indent(shadow_stderr, "shadow: "))
            shadow_stderr = shadow_stderr[count:]

    # clean up temp files
    if preserve == PreserveChoice.ALWAYS or (
        preserve == PreserveChoice.ON_ERROR and shadow_ps.returncode
    ):
        print(f"{progname}: Preserving tmpdir {tmpdir}", file=stderr)
    else:
        shutil.rmtree(tmpdir)

    return shadow_ps.returncode


def __main__() -> None:
    """Raw main, suitable for use with `project.scripts` in `pyproject.toml`"""

    PROGNAME: Final[str] = "shadow-exec"

    parser = argparse.ArgumentParser(
        prog=PROGNAME,
        description=textwrap.dedent(
            f"""
            Executes the command `args` inside a single-host shadow simulation.

            Examples:
              {PROGNAME} date
              {PROGNAME} -- bash -c 'date; sleep 100; date'
            """
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "-p",
        "--preserve",
        choices=["always", "never", "on-error"],
        default="never",
        help="Whether to preserve the raw simulation config and result",
    )
    parser.add_argument(
        "-t",
        "--temp-dir",
        default=None,
        type=Path,
        help=(
            "Override default root directory for temporary files."
            + " If specified, must already exist."
            + " A fresh directory will be created here,"
            + " and by default deleted. See --preserve."
        ),
    )
    parser.add_argument(
        "--shadow-bin",
        default=Path("shadow"),
        type=Path,
        help="shadow binary basename or path",
    )
    if sys.version_info >= (3, 9):
        parser.add_argument(
            "--model-unblocked-syscall-latency",
            action=argparse.BooleanOptionalAction,
            default=True,
            help="set shadow's --model-unblocked-syscall-latency",
        )
    else:
        # No argparse.BooleanOptionalAction; emulate it.  Should function the
        # same, but the help text is a little less nice since the two flags
        # aren't collapsed together.
        parser.add_argument(
            "--model-unblocked-syscall-latency",
            action="store_true",
            help="set shadow's --model-unblocked-syscall-latency (default)",
            default=True,
        )
        parser.add_argument(
            "--no-model-unblocked-syscall-latency",
            action="store_false",
            dest="model_unblocked_syscall_latency",
            help="unset shadow's --model-unblocked-syscall-latency",
        )
    parser.add_argument("args", nargs="+", help="command and arguments to execute")
    res = parser.parse_args()
    rv = _main(
        progname=PROGNAME,
        args=res.args,
        # parser should have enforced a valid value here
        preserve=PreserveChoice[res.preserve.upper().translate({ord("-"): "_"})],
        shadow_bin=res.shadow_bin,
        temp_dir=res.temp_dir,
        model_unblocked_syscall_latency=res.model_unblocked_syscall_latency,
    )
    sys.exit(rv)


if __name__ == "__main__":
    __main__()
