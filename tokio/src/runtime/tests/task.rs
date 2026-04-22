use crate::runtime::task::{
    self, unowned, Id, JoinHandle, OwnedTasks, Schedule, SpawnLocation, Task,
    TaskHarnessScheduleHooks,
};
use crate::runtime::tests::NoopSchedule;
use std::collections::VecDeque;
use std::future::Future;
#[cfg(tokio_unstable)]
use std::panic::Location;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
struct AssertDropHandle {
    is_dropped: Arc<AtomicBool>,
}
impl AssertDropHandle {
    #[track_caller]
    fn assert_dropped(&self) {
        panic!("STUB: not implemented");
    }
    #[track_caller]
    fn assert_not_dropped(&self) {
        panic!("STUB: not implemented");
    }
}
struct AssertDrop {
    is_dropped: Arc<AtomicBool>,
}
impl AssertDrop {
    fn new() -> (Self, AssertDropHandle) {
        panic!("STUB: not implemented");
    }
}
impl Drop for AssertDrop {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
#[test]
fn create_drop1() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    drop(notified);
    handle.assert_not_dropped();
    drop(join);
    handle.assert_dropped();
}
#[test]
fn create_drop2() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    drop(join);
    handle.assert_not_dropped();
    drop(notified);
    handle.assert_dropped();
}
#[test]
fn drop_abort_handle1() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    let abort = join.abort_handle();
    drop(join);
    handle.assert_not_dropped();
    drop(notified);
    handle.assert_not_dropped();
    drop(abort);
    handle.assert_dropped();
}
#[test]
fn drop_abort_handle2() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    let abort = join.abort_handle();
    drop(notified);
    handle.assert_not_dropped();
    drop(abort);
    handle.assert_not_dropped();
    drop(join);
    handle.assert_dropped();
}
#[test]
fn drop_abort_handle_clone() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    let abort = join.abort_handle();
    let abort_clone = abort.clone();
    drop(join);
    handle.assert_not_dropped();
    drop(notified);
    handle.assert_not_dropped();
    drop(abort);
    handle.assert_not_dropped();
    drop(abort_clone);
    handle.assert_dropped();
}
#[test]
fn create_shutdown1() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    drop(join);
    handle.assert_not_dropped();
    notified.shutdown();
    handle.assert_dropped();
}
#[test]
fn create_shutdown2() {
    let (ad, handle) = AssertDrop::new();
    let (notified, join) = unowned(
        async {
            drop(ad);
            unreachable!()
        },
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    handle.assert_not_dropped();
    notified.shutdown();
    handle.assert_dropped();
    drop(join);
}
#[test]
fn unowned_poll() {
    let (task, _) = unowned(
        async {},
        NoopSchedule,
        Id::next(),
        SpawnLocation::capture(),
    );
    task.run();
}
#[test]
fn schedule() {
    with(|rt| {
        rt.spawn(async {
            crate::task::yield_now().await;
        });
        assert_eq!(2, rt.tick());
        rt.shutdown();
    })
}
#[test]
fn shutdown() {
    with(|rt| {
        rt.spawn(async {
            loop {
                crate::task::yield_now().await;
            }
        });
        rt.tick_max(1);
        rt.shutdown();
    })
}
#[test]
fn shutdown_immediately() {
    with(|rt| {
        rt.spawn(async {
            loop {
                crate::task::yield_now().await;
            }
        });
        rt.shutdown();
    })
}
#[test]
fn spawn_niche_in_task() {
    use std::future::poll_fn;
    use std::task::{Context, Poll, Waker};
    with(|rt| {
        let state = Arc::new(Mutex::new(State::new()));
        let mut subscriber = Subscriber::new(Arc::clone(&state), 1);
        rt.spawn(async move {
            subscriber.wait().await;
            subscriber.wait().await;
        });
        rt.spawn(async move {
            state.lock().unwrap().set_version(2);
            state.lock().unwrap().set_version(0);
        });
        rt.tick_max(10);
        assert!(rt.is_empty());
        rt.shutdown();
    });
    pub(crate) struct Subscriber {
        state: Arc<Mutex<State>>,
        observed_version: u64,
        waker_key: Option<usize>,
    }
    impl Subscriber {
        pub(crate) fn new(state: Arc<Mutex<State>>, version: u64) -> Self {
            panic!("STUB: not implemented");
        }
        pub(crate) async fn wait(&mut self) {
            panic!("STUB: not implemented");
        }
    }
    struct State {
        version: u64,
        wakers: Vec<Waker>,
    }
    impl State {
        pub(crate) fn new() -> Self {
            panic!("STUB: not implemented");
        }
        pub(crate) fn poll_update(
            &mut self,
            observed_version: &mut u64,
            waker_key: &mut Option<usize>,
            cx: &Context<'_>,
        ) -> Poll<Option<()>> {
            panic!("STUB: not implemented");
        }
        pub(crate) fn set_version(&mut self, version: u64) {
            panic!("STUB: not implemented");
        }
    }
}
#[test]
fn spawn_during_shutdown() {
    static DID_SPAWN: AtomicBool = AtomicBool::new(false);
    struct SpawnOnDrop(Runtime);
    impl Drop for SpawnOnDrop {
        fn drop(&mut self) {
            panic!("STUB: not implemented");
        }
    }
    with(|rt| {
        let rt2 = rt.clone();
        rt.spawn(async move {
            let _spawn_on_drop = SpawnOnDrop(rt2);
            loop {
                crate::task::yield_now().await;
            }
        });
        rt.tick_max(1);
        rt.shutdown();
    });
    assert!(DID_SPAWN.load(Ordering::SeqCst));
}
fn with(f: impl FnOnce(Runtime)) {
    panic!("STUB: not implemented");
}
#[derive(Clone)]
struct Runtime(Arc<Inner>);
struct Inner {
    core: Mutex<Core>,
    owned: OwnedTasks<Runtime>,
}
struct Core {
    queue: VecDeque<task::Notified<Runtime>>,
}
static CURRENT: Mutex<Option<Runtime>> = Mutex::new(None);
impl Runtime {
    #[track_caller]
    fn spawn<T>(&self, future: T) -> JoinHandle<T::Output>
    where
        T: 'static + Send + Future,
        T::Output: 'static + Send,
    {
        panic!("STUB: not implemented");
    }
    fn tick(&self) -> usize {
        panic!("STUB: not implemented");
    }
    fn tick_max(&self, max: usize) -> usize {
        panic!("STUB: not implemented");
    }
    fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn next_task(&self) -> task::Notified<Runtime> {
        panic!("STUB: not implemented");
    }
    fn shutdown(&self) {
        panic!("STUB: not implemented");
    }
}
impl Schedule for Runtime {
    fn release(&self, task: &Task<Self>) -> Option<Task<Self>> {
        panic!("STUB: not implemented");
    }
    fn schedule(&self, task: task::Notified<Self>) {
        panic!("STUB: not implemented");
    }
    fn hooks(&self) -> TaskHarnessScheduleHooks {
        panic!("STUB: not implemented");
    }
}
