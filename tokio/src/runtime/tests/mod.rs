#![cfg_attr(loom, warn(dead_code, unreachable_pub))]
use self::noop_scheduler::NoopSchedule;
use self::unowned_wrapper::unowned;
mod noop_scheduler {
    use crate::runtime::task::{self, Task, TaskHarnessScheduleHooks};
    /// `task::Schedule` implementation that does nothing, for testing.
    pub(crate) struct NoopSchedule;
    impl task::Schedule for NoopSchedule {
        fn release(&self, _task: &Task<Self>) -> Option<Task<Self>> {
            panic!("STUB: not implemented");
        }
        fn schedule(&self, _task: task::Notified<Self>) {
            panic!("STUB: not implemented");
        }
        fn hooks(&self) -> TaskHarnessScheduleHooks {
            panic!("STUB: not implemented");
        }
    }
}
mod unowned_wrapper {
    use crate::runtime::task::{Id, JoinHandle, Notified, SpawnLocation};
    use crate::runtime::tests::NoopSchedule;
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    #[track_caller]
    pub(crate) fn unowned<T>(task: T) -> (Notified<NoopSchedule>, JoinHandle<T::Output>)
    where
        T: std::future::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    #[cfg(not(all(tokio_unstable, feature = "tracing")))]
    #[track_caller]
    pub(crate) fn unowned<T>(task: T) -> (Notified<NoopSchedule>, JoinHandle<T::Output>)
    where
        T: std::future::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        panic!("STUB: not implemented");
    }
}
cfg_loom! {
    mod loom_blocking; mod loom_current_thread; mod loom_join_set; mod loom_local; mod
    loom_multi_thread; mod loom_oneshot; #[cfg(not(debug_assertions))]
    compile_error!("these tests require debug assertions to be enabled");
}
cfg_not_loom! {
    mod inject; mod queue; #[cfg(not(miri))] mod task_combinations; #[cfg(miri)] mod
    task;
}
