use crate::loom::sync::{Arc, Mutex};
use loom::sync::Notify;
pub(crate) fn channel<T>() -> (Sender<T>, Receiver<T>) {
    panic!("STUB: not implemented");
}
pub(crate) struct Sender<T> {
    inner: Arc<Inner<T>>,
}
pub(crate) struct Receiver<T> {
    inner: Arc<Inner<T>>,
}
struct Inner<T> {
    notify: Notify,
    value: Mutex<Option<T>>,
}
impl<T> Sender<T> {
    pub(crate) fn send(self, value: T) {
        panic!("STUB: not implemented");
    }
}
impl<T> Receiver<T> {
    pub(crate) fn recv(self) -> T {
        panic!("STUB: not implemented");
    }
}
