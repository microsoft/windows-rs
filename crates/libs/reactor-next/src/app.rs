use std::cell::RefCell;
use std::rc::Rc;

use super::*;
use crate::core::*;
use crate::native::*;

thread_local! {
    static HOST: RefCell<Option<LiveHost>> = const { RefCell::new(None) };
    static SCHEDULER_FAULT: RefCell<Option<windows_core::Error>> = const { RefCell::new(None) };
}

#[cfg(feature = "test")]
thread_local! {
    static LIVE_TEST_DISPATCHES: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_TEST_REARM: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

struct LiveHost {
    fault: Option<windows_core::Error>,
    pump: Box<dyn LivePump>,
}

trait LivePump {
    fn mount(&mut self) -> Result<(), PumpError>;
    fn dispatch_events(&mut self) -> Result<(), PumpError>;
    fn native_work_pending(&self) -> bool;
    fn schedule_retry(&self) -> Result<(), RuntimeError>;
    fn shutdown(&mut self);
    #[cfg(feature = "test")]
    fn live_inject_root_text(&mut self, value: &str) -> Result<(), RuntimeError>;
    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError>;
    #[cfg(feature = "test")]
    fn live_stale_remount(&mut self) -> bool;
    #[cfg(feature = "test")]
    fn live_rejection_then_retry(&self) -> bool;
    #[cfg(feature = "test")]
    fn live_mutate_then_fail(&mut self) -> bool;
}

impl<F> LivePump for RenderLoop<WinUiRuntime, F>
where
    F: FnMut(&mut Hooks) -> Element,
{
    fn mount(&mut self) -> Result<(), PumpError> {
        self.run()
    }

    fn dispatch_events(&mut self) -> Result<(), PumpError> {
        self.dispatch_events().map(|_| ())
    }

    fn native_work_pending(&self) -> bool {
        self.pump().native_work_pending()
    }

    fn schedule_retry(&self) -> Result<(), RuntimeError> {
        self.pump().runtime().schedule_retry()
    }

    fn shutdown(&mut self) {
        Self::shutdown(self);
        self.pump().runtime().close_scheduler();
    }

    #[cfg(feature = "test")]
    fn live_inject_root_text(&mut self, value: &str) -> Result<(), RuntimeError> {
        let root = self.pump().root().ok_or(RuntimeError::UnsupportedKind)?;
        let revision = self
            .pump()
            .event_revision(root, EventId::TextBoxTextChanged)
            .ok_or(RuntimeError::MissingSubscription(
                root,
                EventId::TextBoxTextChanged,
            ))?;
        self.pump().runtime().live_set_text(root, value)?;
        self.pump_mut().queue_event(QueuedEvent {
            node: root,
            event: EventId::TextBoxTextChanged,
            revision,
            payload: EventPayload::Str(value.into()),
        });
        self.pump().runtime().schedule_retry()
    }

    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError> {
        let root = self.pump().root().ok_or(RuntimeError::UnsupportedKind)?;
        self.pump().runtime().live_text(root)
    }

    #[cfg(feature = "test")]
    fn live_stale_remount(&mut self) -> bool {
        let calls = Rc::new(std::cell::Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let old_identity = self.pump().native_identity();
        self.pump_mut().runtime_mut().live_fail_next_structural();
        if !matches!(
            self.pump_mut().update(
                Button::new()
                    .on_click(move || callback_calls.set(callback_calls.get() + 1))
                    .into()
            ),
            Err(PumpError::RecoveredStructure(_))
        ) {
            return false;
        }
        let new_identity = self.pump().native_identity();
        let Some(root) = self.pump().root() else {
            return false;
        };
        let Some(revision) = self.pump().event_revision(root, EventId::ButtonClick) else {
            return false;
        };
        self.pump().runtime().live_queue_event(
            old_identity,
            root,
            EventId::ButtonClick,
            revision,
            EventPayload::Unit,
        );
        old_identity.window() == new_identity.window()
            && old_identity.realization_epoch() != new_identity.realization_epoch()
            && self.pump_mut().dispatch_events() == Ok(0)
            && calls.get() == 0
    }

    #[cfg(feature = "test")]
    fn live_rejection_then_retry(&self) -> bool {
        self.pump().runtime().live_reject_next_enqueue();
        self.pump().runtime().schedule_retry() == Err(RuntimeError::DispatcherRejected)
            && self.pump().runtime().schedule_retry().is_ok()
    }

    #[cfg(feature = "test")]
    fn live_mutate_then_fail(&mut self) -> bool {
        self.pump_mut()
            .runtime_mut()
            .live_fail_next_property_after_apply();
        if !matches!(
            self.pump_mut()
                .update(TextBox::new().text("mutated").into()),
            Err(PumpError::PropertyApplyFailed(_))
        ) {
            return false;
        }
        if self
            .pump()
            .runtime()
            .live_text(self.pump().root().unwrap())
            .as_deref()
            != Ok("mutated")
        {
            return false;
        }
        self.pump_mut()
            .update(TextBox::new().text("mutated").into())
            .is_ok()
            && !self.pump().retry_pending()
    }
}

pub fn bootstrap() -> windows_core::Result<()> {
    bootstrap_runtime()
}

pub struct App;

impl App {
    pub fn run<F>(root: F) -> windows_core::Result<()>
    where
        F: FnMut(&mut Hooks) -> Element + 'static,
    {
        initialize_ui_thread()?;
        let root = Rc::new(RefCell::new(Some(root)));
        let result = Rc::new(RefCell::new(Ok(())));
        let callback_result = Rc::clone(&result);

        let start = Application::Start(&ApplicationInitializationCallback::new(move |_| {
            let application = Rc::new(RefCell::new(None));
            let launch_application = Rc::clone(&application);
            let launch_result = Rc::clone(&callback_result);
            let launch_root = Rc::clone(&root);
            let on_launched = Box::new(move || {
                let launched = (|| {
                    let application = launch_application
                        .borrow_mut()
                        .take()
                        .ok_or_else(|| windows_core::Error::new(E_FAIL, "missing application"))?;
                    install_xaml_controls_resources(&application)?;
                    let root = launch_root.borrow_mut().take().unwrap();
                    let mut pump: Box<dyn LivePump> = Box::new(RenderLoop::new(
                        WinUiRuntime::with_application(application),
                        root,
                    ));
                    let property_fault = match pump.mount() {
                        Ok(()) => None,
                        Err(error) if error.recoverable() => Some(error),
                        Err(error) => return Err(pump_error(error)),
                    };
                    HOST.with(|host| {
                        *host.borrow_mut() = Some(LiveHost { fault: None, pump });
                    });
                    if let Some(error) = property_fault {
                        eprintln!("windows-reactor-next fault: {}", pump_error(error));
                        HOST.with(|host| {
                            host.borrow()
                                .as_ref()
                                .unwrap()
                                .pump
                                .schedule_retry()
                                .map_err(runtime_error)
                        })?;
                    }
                    Ok(())
                })();
                if let Err(error) = &launched {
                    *launch_result.borrow_mut() = Err(error.clone());
                    exit_ui_thread();
                }
                launched
            });
            match create_application(on_launched) {
                Ok(created) => *application.borrow_mut() = Some(created),
                Err(error) => {
                    *callback_result.borrow_mut() = Err(error);
                    exit_ui_thread();
                }
            }
        }));

        let callback_result = std::mem::replace(&mut *result.borrow_mut(), Ok(()));
        let host = HOST.with(|host| host.borrow_mut().take());
        let host_result = host.and_then(|host| host.fault).map_or(Ok(()), Err);
        let scheduler_result = SCHEDULER_FAULT
            .with(|fault| fault.borrow_mut().take())
            .map_or(Ok(()), Err);
        start
            .and(callback_result)
            .and(host_result)
            .and(scheduler_result)
    }
}

pub(crate) fn dispatch_native_events() {
    HOST.with(|host| {
        let Some(mut live) = host.borrow_mut().take() else {
            return;
        };
        #[cfg(feature = "test")]
        LIVE_TEST_DISPATCHES.with(|count| count.set(count.get().saturating_add(1)));
        let mut retry = false;
        if live.fault.is_none() {
            match live.pump.dispatch_events() {
                Ok(()) => retry = live.pump.native_work_pending(),
                Err(error) => {
                    let recoverable = error.recoverable();
                    let error = pump_error(error);
                    eprintln!("windows-reactor-next fault: {error}");
                    if recoverable {
                        retry = true;
                    } else {
                        live.fault = Some(error);
                        live.pump.shutdown();
                        exit_ui_thread();
                    }
                }
            }
        }
        #[cfg(feature = "test")]
        if LIVE_TEST_REARM.with(|rearm| rearm.replace(false))
            && let Err(error) = live.pump.schedule_retry()
        {
            live.fault = Some(runtime_error(error));
            live.pump.shutdown();
            exit_ui_thread();
        }
        if retry && let Err(error) = live.pump.schedule_retry() {
            live.fault = Some(runtime_error(error));
            live.pump.shutdown();
            exit_ui_thread();
        }
        *host.borrow_mut() = Some(live);
    });
}

#[cfg(feature = "test")]
pub fn schedule_live_controlled_repair_test(initial_success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let verify_dispatcher = dispatcher.clone();
    let edit = DispatcherQueueHandler::new(move || {
        let edited = HOST.with(|host| {
            host.borrow_mut()
                .as_mut()
                .map(|host| host.pump.live_inject_root_text("native"))
        });
        if !matches!(edited, Some(Ok(()))) {
            eprintln!("controlled repair fixture could not edit root: {edited:?}");
            std::process::exit(1);
        }
        if queue_live_repair_verification(verify_dispatcher.clone(), initial_success, 8).is_err() {
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &edit)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected controlled repair fixture",
        ))
    }
}

#[cfg(feature = "test")]
fn queue_live_repair_verification(
    dispatcher: DispatcherQueue,
    initial_success: bool,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let text = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .map(|host| host.pump.live_root_text())
        });
        let repaired = matches!(text.as_ref(), Some(Ok(value)) if value == "fixed");
        if initial_success && repaired {
            if schedule_live_scheduler_reentrancy_test().is_err() {
                std::process::exit(1);
            }
            return;
        }
        if attempts == 0 {
            eprintln!(
                "controlled repair fixture failed: resources={initial_success}, text={text:?}"
            );
            std::process::exit(1);
        }
        if queue_live_repair_verification(next_dispatcher.clone(), initial_success, attempts - 1)
            .is_err()
        {
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected controlled repair verification",
        ))
    }
}

#[cfg(feature = "test")]
fn schedule_live_scheduler_reentrancy_test() -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let verify_dispatcher = dispatcher.clone();
    let start = DispatcherQueueHandler::new(move || {
        LIVE_TEST_DISPATCHES.with(|count| count.set(0));
        LIVE_TEST_REARM.with(|rearm| rearm.set(true));
        let scheduled = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .is_some_and(|host| host.pump.live_rejection_then_retry())
        });
        if !scheduled || queue_live_reentrancy_verification(verify_dispatcher.clone(), 8).is_err() {
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &start)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected reentrancy fixture",
        ))
    }
}

#[cfg(feature = "test")]
fn queue_live_reentrancy_verification(
    dispatcher: DispatcherQueue,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        if LIVE_TEST_DISPATCHES.with(|count| count.get() >= 2) {
            finish_live_backend_test();
            return;
        }
        if attempts == 0
            || queue_live_reentrancy_verification(next_dispatcher.clone(), attempts - 1).is_err()
        {
            eprintln!("scheduler reentrancy fixture did not observe a rearmed dispatch");
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected reentrancy verification",
        ))
    }
}

#[cfg(feature = "test")]
fn finish_live_backend_test() {
    let Some(mut live) = HOST.with(|host| host.borrow_mut().take()) else {
        eprintln!("live backend fixture lost its host");
        std::process::exit(1);
    };
    if !live.pump.live_mutate_then_fail() {
        eprintln!("live backend fixture did not recover a mutate-then-fail setter");
        std::process::exit(1);
    }
    if !live.pump.live_stale_remount() {
        eprintln!("live backend fixture did not reject stale remount work");
        std::process::exit(1);
    }
    live.pump.shutdown();
    if !live_test_cleanup_ordered() {
        eprintln!("live backend fixture observed native reset before hook cleanup");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn pump_error(error: PumpError) -> windows_core::Error {
    windows_core::Error::new(E_FAIL, format!("{error:?}"))
}

fn runtime_error(error: RuntimeError) -> windows_core::Error {
    windows_core::Error::new(E_FAIL, format!("{error:?}"))
}

pub(crate) fn fail_native_scheduler(error: RuntimeError) {
    SCHEDULER_FAULT.with(|fault| {
        if fault.borrow().is_none() {
            *fault.borrow_mut() = Some(runtime_error(error));
        }
    });
    exit_ui_thread();
}
