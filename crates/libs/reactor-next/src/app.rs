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
    static LIVE_COMPONENT_CREATES: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_COMPONENT_EFFECT_SETUPS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
    static LIVE_COMPONENT_EFFECT_CLEANUPS: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
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
    fn live_set_root_text(&self, _value: &str) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_rejection_then_retry(&self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_component_update(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_component_message_result(&self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_dense_reorder(&mut self) -> bool {
        false
    }
    #[cfg(feature = "test")]
    fn live_fragment_anchor(&mut self) -> bool {
        false
    }
}

struct ComponentLoop {
    pump: Pump<WinUiRuntime>,
    root: Option<View>,
}

impl LivePump for ComponentLoop {
    fn mount(&mut self) -> Result<(), PumpError> {
        self.pump
            .mount_view(self.root.take().ok_or(PumpError::AlreadyMounted)?)
            .map(|_| ())
    }

    fn dispatch_events(&mut self) -> Result<(), PumpError> {
        self.pump.dispatch_events()?;
        self.pump.dispatch_components(64).map(|_| ())
    }

    fn native_work_pending(&self) -> bool {
        self.pump.native_work_pending()
    }

    fn schedule_retry(&self) -> Result<(), RuntimeError> {
        self.pump.runtime().schedule_retry()
    }

    fn shutdown(&mut self) {
        self.pump.shutdown();
        self.pump.runtime().close_scheduler();
    }
}

#[cfg(feature = "test")]
struct LiveTestComponent {
    messages: u8,
    text: String,
}

#[cfg(feature = "test")]
impl Component for LiveTestComponent {
    type Props = String;
    type Message = String;

    fn create(props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
        LIVE_COMPONENT_CREATES.with(|count| count.set(count.get().saturating_add(1)));
        Self {
            messages: 0,
            text: props.clone(),
        }
    }

    fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
        self.text.clone_from(props);
    }

    fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
        self.messages = self.messages.saturating_add(1);
        self.text = if self.messages == 65 {
            message
        } else {
            "pending".to_string()
        };
    }

    fn view(&self, context: &mut ViewContext<Self>) -> View {
        context.use_effect((), || {
            LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.set(count.get().saturating_add(1)));
            Some(Box::new(|| {
                LIVE_COMPONENT_EFFECT_CLEANUPS
                    .with(|count| count.set(count.get().saturating_add(1)));
            }))
        });
        let sender = context.sender();
        View::native(
            TextBox::new()
                .text(self.text.clone())
                .on_text_changed(move |value| {
                    for _ in 0..65 {
                        sender.send(value.clone());
                    }
                }),
        )
    }
}

impl<F> LivePump for RenderLoop<WinUiRuntime, F>
where
    F: FnMut(&mut Hooks) -> Element,
{
    fn mount(&mut self) -> Result<(), PumpError> {
        self.run()
    }

    fn dispatch_events(&mut self) -> Result<(), PumpError> {
        self.dispatch_events()?;
        self.pump_mut().dispatch_components(64).map(|_| ())
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
    fn live_set_root_text(&self, value: &str) -> Result<(), RuntimeError> {
        let root = self.pump().root().ok_or(RuntimeError::UnsupportedKind)?;
        self.pump().runtime().live_set_text(root, value)
    }

    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError> {
        let root = self.pump().root().ok_or(RuntimeError::UnsupportedKind)?;
        self.pump().runtime().live_text(root)
    }

    #[cfg(feature = "test")]
    fn live_rejection_then_retry(&self) -> bool {
        self.pump().runtime().live_reject_next_enqueue();
        self.pump().runtime().schedule_retry() == Err(RuntimeError::DispatcherRejected)
            && self.pump().runtime().schedule_retry().is_ok()
    }

    #[cfg(feature = "test")]
    fn live_component_update(&mut self) -> bool {
        LIVE_COMPONENT_CREATES.with(|count| count.set(0));
        LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.set(0));
        LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.set(0));
        if self
            .pump_mut()
            .update_view(View::component::<LiveTestComponent>(
                "component".to_string(),
            ))
            .is_err()
        {
            return false;
        }
        let Some(native) = self.pump().root_native() else {
            return false;
        };
        LIVE_COMPONENT_CREATES.with(|count| count.get() == 1)
            && LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get() == 1)
            && LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get() == 0)
            && self.pump().runtime().live_text(native).as_deref() == Ok("component")
            && self
                .pump()
                .runtime()
                .live_set_text(native, "message")
                .is_ok()
    }

    #[cfg(feature = "test")]
    fn live_component_message_result(&self) -> bool {
        let Some(native) = self.pump().root_native() else {
            return false;
        };
        LIVE_COMPONENT_CREATES.with(|count| count.get() == 1)
            && self.pump().runtime().live_text(native).as_deref() == Ok("message")
    }

    #[cfg(feature = "test")]
    fn live_dense_reorder(&mut self) -> bool {
        let labels = (0..512).map(|index| index.to_string()).collect::<Vec<_>>();
        let view =
            |labels: &[String]| {
                View::native(StackPanel::new().children(
                    labels.iter().map(|label| {
                        KeyedElement::new(label.clone(), TextBlock::new().text(label))
                    }),
                ))
            };
        if self.pump_mut().update_view(view(&labels)).is_err() {
            return false;
        }
        let reversed = labels.into_iter().rev().collect::<Vec<_>>();
        self.pump_mut().update_view(view(&reversed)).is_ok()
    }

    #[cfg(feature = "test")]
    fn live_fragment_anchor(&mut self) -> bool {
        let fragment = |reverse: bool| {
            let children = if reverse {
                [
                    KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                    KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                ]
            } else {
                [
                    KeyedView::new("a", View::native(TextBlock::new().text("A"))),
                    KeyedView::new("b", View::native(TextBlock::new().text("B"))),
                ]
            };
            View::children(
                StackPanel::new(),
                [
                    KeyedView::new("empty", View::Empty),
                    KeyedView::new("fragment", View::fragment(children)),
                ],
            )
        };

        self.pump_mut().update_view(View::Empty).is_ok()
            && self.pump_mut().update_view(fragment(false)).is_ok()
            && self.pump_mut().update_view(fragment(true)).is_ok()
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
        Self::run_with(move |application| {
            Box::new(RenderLoop::new(
                WinUiRuntime::with_application(application),
                root,
            ))
        })
    }

    pub fn run_component<C: Component>(props: C::Props) -> windows_core::Result<()> {
        Self::run_with(move |application| {
            Box::new(ComponentLoop {
                pump: Pump::new(WinUiRuntime::with_application(application)),
                root: Some(View::component::<C>(props)),
            })
        })
    }

    fn run_with(
        create_pump: impl FnOnce(Application) -> Box<dyn LivePump> + 'static,
    ) -> windows_core::Result<()> {
        initialize_ui_thread()?;
        let create_pump = Rc::new(RefCell::new(Some(create_pump)));
        let result = Rc::new(RefCell::new(Ok(())));
        let callback_result = Rc::clone(&result);

        let start = Application::Start(&ApplicationInitializationCallback::new(move |_| {
            let application = Rc::new(RefCell::new(None));
            let launch_application = Rc::clone(&application);
            let launch_result = Rc::clone(&callback_result);
            let launch_create_pump = Rc::clone(&create_pump);
            let on_launched = Box::new(move || {
                let launched: windows_core::Result<()> = (|| {
                    let application = launch_application
                        .borrow_mut()
                        .take()
                        .ok_or_else(|| windows_core::Error::new(E_FAIL, "missing application"))?;
                    install_xaml_controls_resources(&application)?;
                    let create_pump = launch_create_pump.borrow_mut().take().unwrap();
                    let mut pump = create_pump(application);
                    pump.mount().map_err(pump_error)?;
                    HOST.with(|host| {
                        *host.borrow_mut() = Some(LiveHost { fault: None, pump });
                    });
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
                    let error = pump_error(error);
                    eprintln!("windows-reactor-next fault: {error}");
                    live.fault = Some(error);
                    live.pump.shutdown();
                    exit_ui_thread();
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
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(150));
        let edit_dispatcher = dispatcher.clone();
        let edit = DispatcherQueueHandler::new(move || {
            let edited = HOST.with(|host| {
                host.borrow()
                    .as_ref()
                    .map(|host| host.pump.live_set_root_text("native"))
            });
            if !matches!(edited, Some(Ok(()))) {
                eprintln!("controlled repair fixture could not edit root: {edited:?}");
                std::process::exit(1);
            }
            let verify_dispatcher = edit_dispatcher.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                if queue_live_repair_verification(verify_dispatcher, initial_success, 8).is_err() {
                    std::process::exit(1);
                }
            });
        });
        if !matches!(
            dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &edit),
            Ok(true)
        ) {
            std::process::exit(1);
        }
    });
    Ok(())
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
    if !live.pump.live_dense_reorder() {
        eprintln!("live backend fixture did not apply a dense keyed reorder");
        std::process::exit(1);
    }
    if !live.pump.live_fragment_anchor() {
        eprintln!("live backend fixture did not apply empty and fragment transitions");
        std::process::exit(1);
    }
    if !live.pump.live_component_update() {
        eprintln!("live backend fixture did not apply a component structural update");
        std::process::exit(1);
    }
    let dispatcher = match DispatcherQueue::GetForCurrentThread() {
        Ok(dispatcher) => dispatcher,
        Err(error) => {
            eprintln!("component scheduler fixture has no dispatcher: {error}");
            std::process::exit(1);
        }
    };
    HOST.with(|host| *host.borrow_mut() = Some(live));
    if queue_live_component_verification(dispatcher, 8).is_err() {
        std::process::exit(1);
    }
}

#[cfg(feature = "test")]
fn queue_live_component_verification(
    dispatcher: DispatcherQueue,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let passed = HOST.with(|host| {
            host.borrow()
                .as_ref()
                .is_some_and(|host| host.pump.live_component_message_result())
        });
        if passed {
            finish_live_component_test();
            return;
        }
        if attempts == 0
            || queue_live_component_verification(next_dispatcher.clone(), attempts - 1).is_err()
        {
            eprintln!("component scheduler fixture did not drain and rearm its message backlog");
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected component scheduler verification",
        ))
    }
}

#[cfg(feature = "test")]
fn finish_live_component_test() {
    let Some(mut live) = HOST.with(|host| host.borrow_mut().take()) else {
        eprintln!("component scheduler fixture lost its host");
        std::process::exit(1);
    };
    live.pump.shutdown();
    if LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get()) != 1
        || LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get()) != 1
    {
        eprintln!("live component effect setup or cleanup count was incorrect");
        std::process::exit(1);
    }
    if !live_test_cleanup_ordered() {
        eprintln!("live backend fixture observed native reset before hook cleanup");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn pump_error(error: PumpError) -> windows_core::Error {
    match error {
        PumpError::NativeApplyFailed(error) => {
            eprintln!(
                "windows-reactor-next fatal native command failure at {}: {:?}",
                error.command, error.error
            );
            std::process::abort();
        }
        error => windows_core::Error::new(E_FAIL, format!("{error:?}")),
    }
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
