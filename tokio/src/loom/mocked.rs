pub(crate) use loom::*;
pub(crate) mod sync {
    pub(crate) use loom::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
    #[derive(Debug)]
    pub(crate) struct Mutex<T>(loom::sync::Mutex<T>);
    #[allow(dead_code)]
    impl<T> Mutex<T> {
        #[inline]
        pub(crate) fn new(t: T) -> Mutex<T> {
            panic!("STUB: not implemented");
        }
        #[inline]
        #[track_caller]
        pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
            panic!("STUB: not implemented");
        }
        #[inline]
        pub(crate) fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
            panic!("STUB: not implemented");
        }
    }
    #[derive(Debug)]
    pub(crate) struct RwLock<T>(loom::sync::RwLock<T>);
    #[allow(dead_code)]
    impl<T> RwLock<T> {
        #[inline]
        pub(crate) fn new(t: T) -> Self {
            panic!("STUB: not implemented");
        }
        #[inline]
        pub(crate) fn read(&self) -> RwLockReadGuard<'_, T> {
            panic!("STUB: not implemented");
        }
        #[inline]
        pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
            panic!("STUB: not implemented");
        }
        #[inline]
        pub(crate) fn write(&self) -> RwLockWriteGuard<'_, T> {
            panic!("STUB: not implemented");
        }
        #[inline]
        pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
            panic!("STUB: not implemented");
        }
    }
    pub(crate) use loom::sync::*;
}
pub(crate) mod rand {
    pub(crate) fn seed() -> u64 {
        panic!("STUB: not implemented");
    }
}
pub(crate) mod sys {
    pub(crate) fn num_cpus() -> usize {
        panic!("STUB: not implemented");
    }
}
pub(crate) mod thread {
    pub use loom::lazy_static::AccessError;
    pub use loom::thread::*;
}
