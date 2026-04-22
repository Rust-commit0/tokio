//! Current `backtrace::trace` + collector based backtrace implementation
//!
//! This implementation may eventually be extracted into a separate `tokio-taskdump` crate.
use std::ptr;
use crate::runtime::task::trace::{trace_with, Trace, TraceMeta};
/// Capture using the default `backtrace::trace`-based implementation.
#[inline(never)]
pub(super) fn capture<F, R>(f: F) -> (R, Trace)
where
    F: FnOnce() -> R,
{
    panic!("STUB: not implemented");
}
/// Capture a backtrace via `backtrace::trace` and collect it into `trace`.
pub(crate) fn trace_leaf(meta: &TraceMeta, trace: &mut Trace) {
    panic!("STUB: not implemented");
}
