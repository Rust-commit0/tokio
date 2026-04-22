use crate::sync::batch_semaphore::*;
use loom::future::block_on;
use loom::sync::atomic::AtomicUsize;
use loom::thread;
use std::future::{poll_fn, Future};
use std::pin::Pin;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Arc;
use std::task::Poll::Ready;
use std::task::{Context, Poll};
#[test]
fn basic_usage() {
    const NUM: usize = 2;
    struct Shared {
        semaphore: Semaphore,
        active: AtomicUsize,
    }
    async fn actor(shared: Arc<Shared>) {
        panic!("STUB: not implemented");
    }
    loom::model(|| {
        let shared = Arc::new(Shared {
            semaphore: Semaphore::new(NUM),
            active: AtomicUsize::new(0),
        });
        for _ in 0..NUM {
            let shared = shared.clone();
            thread::spawn(move || {
                block_on(actor(shared));
            });
        }
        block_on(actor(shared));
    });
}
#[test]
fn release() {
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::new(1));
        {
            let semaphore = semaphore.clone();
            thread::spawn(move || {
                block_on(semaphore.acquire(1)).unwrap();
                semaphore.release(1);
            });
        }
        block_on(semaphore.acquire(1)).unwrap();
        semaphore.release(1);
    });
}
#[test]
fn basic_closing() {
    const NUM: usize = 2;
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::new(1));
        for _ in 0..NUM {
            let semaphore = semaphore.clone();
            thread::spawn(move || {
                for _ in 0..2 {
                    block_on(semaphore.acquire(1)).map_err(|_| ())?;
                    semaphore.release(1);
                }
                Ok::<(), ()>(())
            });
        }
        semaphore.close();
    });
}
#[test]
fn concurrent_close() {
    const NUM: usize = 3;
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::new(1));
        for _ in 0..NUM {
            let semaphore = semaphore.clone();
            thread::spawn(move || {
                block_on(semaphore.acquire(1)).map_err(|_| ())?;
                semaphore.release(1);
                semaphore.close();
                Ok::<(), ()>(())
            });
        }
    });
}
#[test]
fn concurrent_cancel() {
    async fn poll_and_cancel(semaphore: Arc<Semaphore>) {
        panic!("STUB: not implemented");
    }
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::new(0));
        let t1 = {
            let semaphore = semaphore.clone();
            thread::spawn(move || block_on(poll_and_cancel(semaphore)))
        };
        let t2 = {
            let semaphore = semaphore.clone();
            thread::spawn(move || block_on(poll_and_cancel(semaphore)))
        };
        let t3 = {
            let semaphore = semaphore.clone();
            thread::spawn(move || block_on(poll_and_cancel(semaphore)))
        };
        t1.join().unwrap();
        semaphore.release(10);
        t2.join().unwrap();
        t3.join().unwrap();
    });
}
#[test]
fn batch() {
    let mut b = loom::model::Builder::new();
    b.preemption_bound = Some(1);
    b.check(|| {
        let semaphore = Arc::new(Semaphore::new(10));
        let active = Arc::new(AtomicUsize::new(0));
        let mut threads = vec![];
        for _ in 0..2 {
            let semaphore = semaphore.clone();
            let active = active.clone();
            threads
                .push(
                    thread::spawn(move || {
                        for n in &[4, 10, 8] {
                            block_on(semaphore.acquire(*n)).unwrap();
                            active.fetch_add(*n as usize, SeqCst);
                            let num_active = active.load(SeqCst);
                            assert!(num_active <= 10);
                            thread::yield_now();
                            active.fetch_sub(*n as usize, SeqCst);
                            semaphore.release(*n as usize);
                        }
                    }),
                );
        }
        for thread in threads.into_iter() {
            thread.join().unwrap();
        }
        assert_eq!(10, semaphore.available_permits());
    });
}
#[test]
fn release_during_acquire() {
    loom::model(|| {
        let semaphore = Arc::new(Semaphore::new(10));
        semaphore
            .try_acquire(8)
            .expect("try_acquire should succeed; semaphore uncontended");
        let semaphore2 = semaphore.clone();
        let thread = thread::spawn(move || block_on(semaphore2.acquire(4)).unwrap());
        semaphore.release(8);
        thread.join().unwrap();
        semaphore.release(4);
        assert_eq!(10, semaphore.available_permits());
    })
}
#[test]
fn concurrent_permit_updates() {
    loom::model(move || {
        let semaphore = Arc::new(Semaphore::new(5));
        let t1 = {
            let semaphore = semaphore.clone();
            thread::spawn(move || semaphore.release(3))
        };
        let t2 = {
            let semaphore = semaphore.clone();
            thread::spawn(move || {
                semaphore.try_acquire(1).expect("try_acquire should succeed")
            })
        };
        let t3 = {
            let semaphore = semaphore.clone();
            thread::spawn(move || semaphore.forget_permits(2))
        };
        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
        assert_eq!(semaphore.available_permits(), 5);
    })
}
