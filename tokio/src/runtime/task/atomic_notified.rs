use crate::loom::sync::atomic::AtomicPtr;
use crate::runtime::task::{Header, Notified, RawTask};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::SeqCst;
/// An atomic cell which can contain a pointer to a [`Notified`] task.
///
/// This is similar to the `crate::util::AtomicCell` type, but specialized to
/// hold a task pointer --- this type "remembers" the task's scheduler generic
/// when a task is stored in the cell, so that the pointer can be turned back
/// into a [`Notified`] task with the correct generic type when it is retrieved.
pub(crate) struct AtomicNotified<S: 'static> {
    task: AtomicPtr<Header>,
    _scheduler: PhantomData<S>,
}
impl<S: 'static> AtomicNotified<S> {
    pub(crate) fn empty() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn swap(&self, task: Option<Notified<S>>) -> Option<Notified<S>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn take(&self) -> Option<Notified<S>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_some(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
unsafe impl<S: Send> Send for AtomicNotified<S> {}
unsafe impl<S: Send> Sync for AtomicNotified<S> {}
impl<S> Drop for AtomicNotified<S> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
