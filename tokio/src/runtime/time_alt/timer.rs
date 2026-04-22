use super::{EntryHandle, TempLocalContext};
use crate::runtime::scheduler::Handle as SchedulerHandle;
use crate::time::Instant;
use std::pin::Pin;
use std::task::{Context, Poll};
#[cfg(any(feature = "rt", feature = "rt-multi-thread"))]
use crate::util::error::RUNTIME_SHUTTING_DOWN_ERROR;
pub(crate) struct Timer {
    sched_handle: SchedulerHandle,
    /// The entry in the timing wheel.
    ///
    /// - `Some` if the timer is registered / pending / woken up / cancelling.
    /// - `None` if the timer is unregistered.
    entry: Option<EntryHandle>,
    /// The deadline for the timer.
    deadline: Instant,
}
impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Timer {
    #[track_caller]
    pub(crate) fn new(sched_hdl: SchedulerHandle, deadline: Instant) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn deadline(&self) -> Instant {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_elapsed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn register(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_elapsed(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn scheduler_handle(&self) -> &SchedulerHandle {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(crate) fn driver(&self) -> &crate::runtime::time::Handle {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(crate) fn clock(&self) -> &crate::time::Clock {
        panic!("STUB: not implemented");
    }
}
fn with_current_temp_local_context<F, R>(hdl: &SchedulerHandle, f: F) -> R
where
    F: FnOnce(Option<TempLocalContext<'_>>) -> R,
{
    panic!("STUB: not implemented");
}
fn push_from_remote(sched_hdl: &SchedulerHandle, entry_hdl: EntryHandle) {
    panic!("STUB: not implemented");
}
fn deadline_to_tick(sched_hdl: &SchedulerHandle, deadline: Instant) -> u64 {
    panic!("STUB: not implemented");
}
