use super::{Entry, EntryHandle, WakeQueueEntry};
use crate::util::linked_list;
type EntryList = linked_list::LinkedList<WakeQueueEntry, Entry>;
/// A queue of entries that need to be woken up.
#[derive(Debug)]
pub(crate) struct WakeQueue {
    list: EntryList,
}
impl Drop for WakeQueue {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl WakeQueue {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - [`Entry::extra_pointers`] of `hdl` must not being used.
    pub(crate) unsafe fn push_front(&mut self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
    /// Wakes all entries in the wake queue.
    pub(crate) fn wake_all(mut self) {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests;
