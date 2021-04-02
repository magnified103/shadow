/*
 * The Shadow Simulator
 * Copyright (c) 2010-2011, Rob Jansen
 * See LICENSE for licensing information
 */

#ifndef SHD_WORKER_H_
#define SHD_WORKER_H_

#include <glib.h>
#include <netinet/in.h>

#include "main/core/manager.h"
#include "main/core/scheduler/scheduler.h"
#include "main/core/support/definitions.h"
#include "main/core/support/options.h"
#include "main/core/work/task.h"
#include "main/host/host.h"
#include "main/routing/address.h"
#include "main/routing/dns.h"
#include "main/routing/packet.h"
#include "main/routing/topology.h"
#include "main/utility/count_down_latch.h"
#include "support/logger/log_level.h"

// A single worker thread.
typedef struct _Worker Worker;
// A pool of worker threads.
typedef struct _WorkerPool WorkerPool;
// Task to be executed on a worker thread.
typedef void (*WorkerPoolTaskFn)(void*);

// To be called by scheduler. Consumes `event`
void worker_runEvent(Event* event);
// To be called by worker thread
void worker_finish(GQueue* hosts);

// Create a workerpool with `nThreads` threads, allowing up to `nConcurrent` to
// run at a time.
WorkerPool* workerpool_new(Manager* manager, Scheduler* scheduler, int nThreads,
                           int nConcurrent);

// Begin executing taskFn(data) on each worker thread in the pool.
void workerpool_startTaskFn(WorkerPool* pool, WorkerPoolTaskFn taskFn,
                            void* data);
// Await completion of a taskFn on every thread in the pool.
void workerpool_awaitTaskFn(WorkerPool* pool);
int workerpool_getNWorkers(WorkerPool* pool);
// Signal worker threads to exit and wait for them to do so.
void workerpool_joinAll(WorkerPool* pool);
void workerpool_free(WorkerPool* pool);
pthread_t workerpool_getThread(WorkerPool* pool, int threadId);

int worker_getAffinity();
DNS* worker_getDNS();
Topology* worker_getTopology();
Options* worker_getOptions();
gboolean worker_scheduleTask(Task* task, SimulationTime nanoDelay);
void worker_sendPacket(Packet* packet);
gboolean worker_isAlive();

SimulationTime worker_getCurrentTime();
EmulatedTime worker_getEmulatedTime();

gboolean worker_isBootstrapActive();
guint32 worker_getNodeBandwidthUp(GQuark nodeID, in_addr_t ip);
guint32 worker_getNodeBandwidthDown(GQuark nodeID, in_addr_t ip);

gdouble worker_getLatency(GQuark sourceNodeID, GQuark destinationNodeID);
gint worker_getThreadID();
void worker_updateMinTimeJump(gdouble minPathLatency);
void worker_setCurrentTime(SimulationTime time);
gboolean worker_isFiltered(LogLevel level);

void worker_bootHosts(GQueue* hosts);
void worker_freeHosts(GQueue* hosts);

Host* worker_getActiveHost();
void worker_setActiveHost(Host* host);
Process* worker_getActiveProcess();
void worker_setActiveProcess(Process* proc);

void worker_incrementPluginError();

Address* worker_resolveIPToAddress(in_addr_t ip);
Address* worker_resolveNameToAddress(const gchar* name);

// Implementation for counting allocated objects. Do not use this function directly.
// Use worker_count_allocation instead from the call site.
void __worker_increment_object_alloc_counter(const char* object_name);

// Implementation for counting deallocated objects. Do not use this function directly.
// Use worker_count_deallocation instead from the call site.
void __worker_increment_object_dealloc_counter(const char* object_name);

// Increment a counter for the allocation of the object with the given name.
// This should be paired with an increment of the dealloc counter with the
// same name, otherwise we print a warning that a memory leak was detected.
#define worker_count_allocation(type) __worker_increment_object_alloc_counter(#type)

// Increment a counter for the deallocation of the object with the given name.
// This should be paired with an increment of the alloc counter with the
// same name, otherwise we print a warning that a memory leak was detected.
#define worker_count_deallocation(type) __worker_increment_object_dealloc_counter(#type)

// Aggregate the given syscall counts in a worker syscall counter.
void worker_add_syscall_counts(Counter* syscall_counts);

#endif /* SHD_WORKER_H_ */
