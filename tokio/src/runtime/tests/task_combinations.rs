use std::fmt;
use std::future::Future;
use std::panic;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::runtime::task::AbortHandle;
use crate::runtime::Builder;
use crate::sync::oneshot;
use crate::task::JoinHandle;
use futures::future::FutureExt;
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiRuntime {
    CurrentThread,
    Multi1,
    Multi2,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiLocalSet {
    Yes,
    No,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiTask {
    PanicOnRun,
    PanicOnDrop,
    PanicOnRunAndDrop,
    NoPanic,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiOutput {
    PanicOnDrop,
    NoPanic,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiJoinInterest {
    Polled,
    NotPolled,
}
#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiJoinHandle {
    DropImmediately = 1,
    DropFirstPoll = 2,
    DropAfterNoConsume = 3,
    DropAfterConsume = 4,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiAbort {
    NotAborted = 0,
    AbortedImmediately = 1,
    AbortedFirstPoll = 2,
    AbortedAfterFinish = 3,
    AbortedAfterConsumeOutput = 4,
}
#[derive(Copy, Clone, Debug, PartialEq)]
enum CombiAbortSource {
    JoinHandle,
    AbortHandle,
}
#[test]
#[cfg_attr(panic = "abort", ignore)]
fn test_combinations() {
    let mut rt = &[
        CombiRuntime::CurrentThread,
        CombiRuntime::Multi1,
        CombiRuntime::Multi2,
    ][..];
    if cfg!(miri) {
        rt = &[CombiRuntime::CurrentThread];
    }
    let ls = [CombiLocalSet::Yes, CombiLocalSet::No];
    let task = [
        CombiTask::NoPanic,
        CombiTask::PanicOnRun,
        CombiTask::PanicOnDrop,
        CombiTask::PanicOnRunAndDrop,
    ];
    let output = [CombiOutput::NoPanic, CombiOutput::PanicOnDrop];
    let ji = [CombiJoinInterest::Polled, CombiJoinInterest::NotPolled];
    let jh = [
        CombiJoinHandle::DropImmediately,
        CombiJoinHandle::DropFirstPoll,
        CombiJoinHandle::DropAfterNoConsume,
        CombiJoinHandle::DropAfterConsume,
    ];
    let abort = [
        CombiAbort::NotAborted,
        CombiAbort::AbortedImmediately,
        CombiAbort::AbortedFirstPoll,
        CombiAbort::AbortedAfterFinish,
        CombiAbort::AbortedAfterConsumeOutput,
    ];
    let ah = [
        None,
        Some(CombiJoinHandle::DropImmediately),
        Some(CombiJoinHandle::DropFirstPoll),
        Some(CombiJoinHandle::DropAfterNoConsume),
        Some(CombiJoinHandle::DropAfterConsume),
    ];
    for rt in rt.iter().copied() {
        for ls in ls.iter().copied() {
            for task in task.iter().copied() {
                for output in output.iter().copied() {
                    for ji in ji.iter().copied() {
                        for jh in jh.iter().copied() {
                            for abort in abort.iter().copied() {
                                for ah in ah.iter().copied() {
                                    test_combination(
                                        rt,
                                        ls,
                                        task,
                                        output,
                                        ji,
                                        jh,
                                        ah,
                                        abort,
                                        CombiAbortSource::JoinHandle,
                                    );
                                }
                                test_combination(
                                    rt,
                                    ls,
                                    task,
                                    output,
                                    ji,
                                    jh,
                                    None,
                                    abort,
                                    CombiAbortSource::AbortHandle,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
fn is_debug<T: fmt::Debug>(_: &T) {
    panic!("STUB: not implemented");
}
#[allow(clippy::too_many_arguments)]
fn test_combination(
    rt: CombiRuntime,
    ls: CombiLocalSet,
    task: CombiTask,
    output: CombiOutput,
    ji: CombiJoinInterest,
    jh: CombiJoinHandle,
    ah: Option<CombiJoinHandle>,
    abort: CombiAbort,
    abort_src: CombiAbortSource,
) {
    panic!("STUB: not implemented");
}
