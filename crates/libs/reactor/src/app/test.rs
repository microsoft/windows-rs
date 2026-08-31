use super::*;
use std::cell::RefCell;

#[derive(Default)]
pub(super) struct LiveTestState {
    pub(super) event_delivery_stage: usize,
    pub(super) event_delivery_observed: Option<Rc<std::cell::Cell<bool>>>,
    pub(super) event_delivery_waits: usize,
    pub(super) content_dialog_stage: usize,
    pub(super) content_dialog_waits: usize,
}

thread_local! {
    static DIAGNOSTICS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn record_live_diagnostic(message: String) {
    DIAGNOSTICS.with(|diagnostics| diagnostics.borrow_mut().push(message));
}

pub fn bring_live_virtual_index(index: usize) -> Result<(), RuntimeError> {
    HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::primary)
            .ok_or(RuntimeError::UnsupportedKind)?
            .live_bring_virtual_index(index)
    })
}

pub fn live_virtual_shell_counts() -> Result<(usize, usize), RuntimeError> {
    HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::primary)
            .ok_or(RuntimeError::UnsupportedKind)?
            .live_virtual_shell_counts()
    })
}

pub fn take_live_performance_times() -> (Vec<f64>, Vec<f64>) {
    let dispatch = LIVE_DISPATCH_TIMES_US.with(|times| std::mem::take(&mut *times.borrow_mut()));
    let native = HOST.with(|host| {
        host.borrow_mut()
            .as_mut()
            .and_then(LiveHost::primary_mut)
            .map_or_else(Vec::new, LivePump::take_live_native_apply_times)
    });
    (dispatch, native)
}

pub fn clear_live_performance_times() {
    LIVE_DISPATCH_TIMES_US.with(|times| times.borrow_mut().clear());
    HOST.with(|host| {
        if let Some(primary) = host.borrow_mut().as_mut().and_then(LiveHost::primary_mut) {
            primary.clear_live_native_apply_times();
        }
    });
}

pub fn take_live_diagnostics() -> Vec<String> {
    DIAGNOSTICS.with(|diagnostics| diagnostics.take())
}

pub fn schedule_live_event_subscription_count(
    completion: impl FnOnce(Result<usize, String>) + 'static,
) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let completion = RefCell::new(Some(completion));
    let handler = DispatcherQueueHandler::new(move || {
        let result = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .and_then(LiveHost::primary)
                .ok_or_else(|| "live primary window is unavailable".to_string())?
                .live_event_subscription_count()
                .map_err(|error| format!("{error:?}"))
        });
        if let Some(completion) = completion.take() {
            completion(result);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected live event subscription count request",
        ))
    }
}

pub fn schedule_live_window_handle(
    completion: impl FnOnce(Result<isize, String>) + 'static,
) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let completion = RefCell::new(Some(completion));
    let handler = DispatcherQueueHandler::new(move || {
        let result = HOST.with(|host| {
            let window = host
                .borrow()
                .as_ref()
                .and_then(LiveHost::primary)
                .and_then(|live| live.live_window().ok())
                .ok_or_else(|| "live primary window is unavailable".to_string())?;
            native_window_handle(&window).map_err(|error| error.to_string())
        });
        if let Some(completion) = completion.take() {
            completion(result);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected live window handle request",
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveProbe {
    ContentDialogLifecycle,
    ControlledFeedback,
    EventDelivery,
    EventRevokers,
}

pub fn schedule_live_probe(
    probe: LiveProbe,
    completion: impl Fn(Result<(), String>) + 'static,
) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let verify_dispatcher = dispatcher.clone();
    let completion: Rc<dyn Fn(Result<(), String>)> = Rc::new(completion);
    let handler = DispatcherQueueHandler::new(move || {
        if matches!(
            probe,
            LiveProbe::ContentDialogLifecycle | LiveProbe::EventDelivery
        ) {
            let result = live_staged_probe_step(probe);
            match result {
                Ok(true) => finish_live_probe(probe, Ok(()), Rc::clone(&completion)),
                Ok(false) => continue_live_staged_probe(
                    verify_dispatcher.clone(),
                    probe,
                    Rc::clone(&completion),
                ),
                Err(error) => finish_live_probe(probe, Err(error), Rc::clone(&completion)),
            }
            return;
        }
        let result = HOST.with(|host| {
            let mut host = host.borrow_mut();
            let Some(live) = host.as_mut().and_then(LiveHost::secondary_mut) else {
                return Err("live probe window is unavailable".to_string());
            };
            let passed = match probe {
                LiveProbe::ContentDialogLifecycle => unreachable!(),
                LiveProbe::ControlledFeedback => live.live_controlled_feedback_start(),
                LiveProbe::EventDelivery => unreachable!(),
                LiveProbe::EventRevokers => live.live_event_revokers(),
            };
            if !passed {
                return Err(format!("{probe:?} probe failed"));
            }
            Ok(())
        });
        if result.is_err() || matches!(probe, LiveProbe::EventDelivery | LiveProbe::EventRevokers) {
            finish_live_probe(probe, result, Rc::clone(&completion));
            return;
        }

        let input_dispatcher = verify_dispatcher.clone();
        let input_completion = Rc::clone(&completion);
        let input = move || {
            let applied = HOST.with(|host| {
                host.borrow_mut()
                    .as_mut()
                    .and_then(LiveHost::secondary_mut)
                    .is_some_and(LivePump::live_controlled_feedback_input)
            });
            if !applied {
                finish_live_probe(
                    probe,
                    Err(format!("{probe:?} native input failed")),
                    Rc::clone(&input_completion),
                );
                return;
            }
            let verify_completion = Rc::clone(&input_completion);
            let verify = move || {
                let passed = HOST.with(|host| {
                    host.borrow_mut()
                        .as_mut()
                        .and_then(LiveHost::secondary_mut)
                        .is_some_and(LivePump::live_controlled_feedback_finish)
                });
                finish_live_probe(
                    probe,
                    passed
                        .then_some(())
                        .ok_or_else(|| format!("{probe:?} probe failed")),
                    Rc::clone(&verify_completion),
                );
            };
            if let Err(error) = queue_live_delayed(input_dispatcher.clone(), verify) {
                input_completion(Err(format!("{probe:?} verification failed: {error}")));
            }
        };
        if let Err(error) = queue_live_delayed(verify_dispatcher.clone(), input) {
            completion(Err(format!("{probe:?} input scheduling failed: {error}")));
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected live probe",
        ))
    }
}

fn live_staged_probe_step(probe: LiveProbe) -> Result<bool, String> {
    HOST.with(|host| {
        let mut host = host.borrow_mut();
        let live = host
            .as_mut()
            .and_then(LiveHost::secondary_mut)
            .ok_or_else(|| "live probe window is unavailable".to_string())?;
        match probe {
            LiveProbe::ContentDialogLifecycle => live.live_content_dialog_lifecycle_step(),
            LiveProbe::EventDelivery => live.live_event_delivery_step(),
            _ => Err(format!("{probe:?} is not a staged probe")),
        }
    })
}

fn continue_live_staged_probe(
    dispatcher: DispatcherQueue,
    probe: LiveProbe,
    completion: Rc<dyn Fn(Result<(), String>)>,
) {
    let next_dispatcher = dispatcher.clone();
    let next_completion = Rc::clone(&completion);
    let step = move || {
        let result = live_staged_probe_step(probe);
        match result {
            Ok(true) => finish_live_probe(probe, Ok(()), Rc::clone(&next_completion)),
            Ok(false) => continue_live_staged_probe(
                next_dispatcher.clone(),
                probe,
                Rc::clone(&next_completion),
            ),
            Err(error) => {
                finish_live_probe(probe, Err(error), Rc::clone(&next_completion));
            }
        }
    };
    if let Err(error) = queue_live_delayed(dispatcher, step) {
        finish_live_probe(
            probe,
            Err(format!(
                "{probe:?} event delivery scheduling failed: {error}"
            )),
            completion,
        );
    }
}

fn finish_live_probe(
    probe: LiveProbe,
    result: Result<(), String>,
    completion: Rc<dyn Fn(Result<(), String>)>,
) {
    let window = HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::secondary)
            .and_then(|live| live.live_window().ok())
    });
    let result = result.and_then(|()| {
        window
            .ok_or_else(|| format!("{probe:?} window is unavailable"))?
            .Close()
            .map_err(|error| format!("{probe:?} window close failed: {error}"))
    });
    completion(result);
}
