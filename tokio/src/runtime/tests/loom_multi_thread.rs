mod queue;
mod shutdown;
mod yield_now;
/// Full runtime loom tests. These are heavy tests and take significant time to
/// run on CI.
///
/// Use `LOOM_MAX_PREEMPTIONS=1` to do a "quick" run as a smoke test.
///
/// In order to speed up the C
use crate::runtime::tests::loom_oneshot as oneshot;
use crate::runtime::{self, Runtime};
use crate::{spawn, task};
use tokio_test::assert_ok;
use loom::sync::atomic::{AtomicBool, AtomicUsize};
use loom::sync::Arc;
use pin_project_lite::pin_project;
use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::sync::atomic::Ordering::{Relaxed, SeqCst};
use std::task::{ready, Context, Poll};
mod atomic_take {
    use loom::sync::atomic::AtomicBool;
    use std::mem::MaybeUninit;
    use std::sync::atomic::Ordering::SeqCst;
    pub(super) struct AtomicTake<T> {
        inner: MaybeUninit<T>,
        taken: AtomicBool,
    }
    impl<T> AtomicTake<T> {
        pub(super) fn new(value: T) -> Self {
            panic!("STUB: not implemented");
        }
        pub(super) fn take(&self) -> Option<T> {
            panic!("STUB: not implemented");
        }
    }
    impl<T> Drop for AtomicTake<T> {
        fn drop(&mut self) {
            panic!("STUB: not implemented");
        }
    }
}
#[derive(Clone)]
struct AtomicOneshot<T> {
    value: std::sync::Arc<atomic_take::AtomicTake<oneshot::Sender<T>>>,
}
impl<T> AtomicOneshot<T> {
    fn new(sender: oneshot::Sender<T>) -> Self {
        panic!("STUB: not implemented");
    }
    fn assert_send(&self, value: T) {
        panic!("STUB: not implemented");
    }
}
/// Tests are divided into groups to make the runs faster on CI.
mod group_a {
    use super::*;
    #[test]
    fn racy_shutdown() {
        loom::model(|| {
            let pool = mk_pool(1);
            pool.spawn(
                track(async {
                    crate::task::block_in_place(|| {});
                }),
            );
            pool.spawn(track(async {}));
            drop(pool);
        });
    }
    #[test]
    fn pool_multi_spawn() {
        loom::model(|| {
            let pool = mk_pool(2);
            let c1 = Arc::new(AtomicUsize::new(0));
            let (tx, rx) = oneshot::channel();
            let tx1 = AtomicOneshot::new(tx);
            let c2 = c1.clone();
            let tx2 = tx1.clone();
            pool.spawn(
                track(async move {
                    spawn(
                        track(async move {
                            if 1 == c1.fetch_add(1, Relaxed) {
                                tx1.assert_send(());
                            }
                        }),
                    );
                }),
            );
            pool.spawn(
                track(async move {
                    spawn(
                        track(async move {
                            if 1 == c2.fetch_add(1, Relaxed) {
                                tx2.assert_send(());
                            }
                        }),
                    );
                }),
            );
            rx.recv();
        });
    }
    fn only_blocking_inner(first_pending: bool) {
        panic!("STUB: not implemented");
    }
    #[test]
    fn only_blocking_without_pending() {
        only_blocking_inner(false)
    }
    #[test]
    fn only_blocking_with_pending() {
        only_blocking_inner(true)
    }
}
mod group_b {
    use super::*;
    fn blocking_and_regular_inner(first_pending: bool) {
        panic!("STUB: not implemented");
    }
    #[test]
    fn blocking_and_regular() {
        blocking_and_regular_inner(false);
    }
    #[test]
    fn blocking_and_regular_with_pending() {
        blocking_and_regular_inner(true);
    }
    #[test]
    fn join_output() {
        loom::model(|| {
            let rt = mk_pool(1);
            rt.block_on(async {
                let t = crate::spawn(track(async { "hello" }));
                let out = assert_ok!(t. await);
                assert_eq!("hello", out.into_inner());
            });
        });
    }
    #[test]
    fn poll_drop_handle_then_drop() {
        loom::model(|| {
            let rt = mk_pool(1);
            rt.block_on(async move {
                let mut t = crate::spawn(track(async { "hello" }));
                poll_fn(|cx| {
                        let _ = Pin::new(&mut t).poll(cx);
                        Poll::Ready(())
                    })
                    .await;
            });
        })
    }
    #[test]
    fn complete_block_on_under_load() {
        loom::model(|| {
            let pool = mk_pool(1);
            pool.block_on(async {
                crate::spawn(
                    track(async {
                        for _ in 0..2 {
                            task::yield_now().await;
                        }
                    }),
                );
                gated2(true).await
            });
        });
    }
    #[test]
    fn shutdown_with_notification() {
        use crate::sync::oneshot;
        loom::model(|| {
            let rt = mk_pool(2);
            let (done_tx, done_rx) = oneshot::channel::<()>();
            rt.spawn(
                track(async move {
                    let (tx, rx) = oneshot::channel::<()>();
                    crate::spawn(async move {
                        crate::task::spawn_blocking(move || {
                            let _ = tx.send(());
                        });
                        let _ = done_rx.await;
                    });
                    let _ = rx.await;
                    let _ = done_tx.send(());
                }),
            );
        });
    }
}
mod group_c {
    use super::*;
    #[test]
    fn pool_shutdown() {
        loom::model(|| {
            let pool = mk_pool(2);
            pool.spawn(
                track(async move {
                    gated2(true).await;
                }),
            );
            pool.spawn(
                track(async move {
                    gated2(false).await;
                }),
            );
            drop(pool);
        });
    }
}
mod group_d {
    use super::*;
    #[test]
    fn pool_multi_notify() {
        loom::model(|| {
            let pool = mk_pool(2);
            let c1 = Arc::new(AtomicUsize::new(0));
            let (done_tx, done_rx) = oneshot::channel();
            let done_tx1 = AtomicOneshot::new(done_tx);
            let done_tx2 = done_tx1.clone();
            let c2 = c1.clone();
            pool.spawn(
                track(async move {
                    multi_gated().await;
                    if 1 == c1.fetch_add(1, Relaxed) {
                        done_tx1.assert_send(());
                    }
                }),
            );
            pool.spawn(
                track(async move {
                    multi_gated().await;
                    if 1 == c2.fetch_add(1, Relaxed) {
                        done_tx2.assert_send(());
                    }
                }),
            );
            done_rx.recv();
        });
    }
}
fn mk_pool(num_threads: usize) -> Runtime {
    panic!("STUB: not implemented");
}
fn gated2(thread: bool) -> impl Future<Output = &'static str> {
    panic!("STUB: not implemented");
    #[allow(unreachable_code)] std::future::ready::<&'static str>(panic!())
}
async fn multi_gated() {
    panic!("STUB: not implemented");
}
fn track<T: Future>(f: T) -> Track<T> {
    panic!("STUB: not implemented");
}
pin_project! {
    struct Track < T > { #[pin] inner : T, arc : Arc < () >, }
}
impl<T> Track<T> {
    fn into_inner(self) -> T {
        panic!("STUB: not implemented");
    }
}
impl<T: Future> Future for Track<T> {
    type Output = Track<T::Output>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
