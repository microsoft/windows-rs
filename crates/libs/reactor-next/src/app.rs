use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::*;
use crate::core::*;
use crate::native::*;

const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);

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
    _application: Application,
    closed_in_flight: HashSet<WindowToken>,
    fault: Option<windows_core::Error>,
    in_flight: HashSet<WindowToken>,
    #[cfg(feature = "test")]
    primary: WindowToken,
    windows: HashMap<WindowToken, Box<dyn LivePump>>,
}

impl LiveHost {
    #[cfg(feature = "test")]
    fn primary(&self) -> Option<&dyn LivePump> {
        self.windows.get(&self.primary).map(Box::as_ref)
    }

    #[cfg(feature = "test")]
    fn primary_mut(&mut self) -> Option<&mut (dyn LivePump + '_)> {
        match self.windows.get_mut(&self.primary) {
            Some(pump) => Some(pump.as_mut()),
            None => None,
        }
    }

    #[cfg(feature = "test")]
    fn primary_window_for_test(&self) -> Option<Window> {
        self.primary()?.live_window().ok()
    }

    #[cfg(feature = "test")]
    fn secondary(&self) -> Option<&dyn LivePump> {
        self.windows
            .iter()
            .find(|(token, _)| **token != self.primary)
            .map(|(_, pump)| pump.as_ref())
    }

    #[cfg(feature = "test")]
    fn secondary_window_for_test(&self) -> Option<Window> {
        let pump = self.secondary()?;
        if pump.live_root_text().as_deref() != Ok("second") || pump.schedule_retry().is_err() {
            return None;
        }
        pump.live_window().ok()
    }
}

trait LivePump {
    fn mount(&mut self) -> Result<(), PumpError>;
    fn dispatch_events(&mut self) -> Result<(), PumpError>;
    fn native_work_pending(&self) -> bool;
    fn schedule_retry(&self) -> Result<(), RuntimeError>;
    fn close_scheduler(&self);
    fn native_window_closed(&mut self);
    fn shutdown(&mut self);
    fn window_token(&self) -> WindowToken;
    #[cfg(feature = "test")]
    fn live_set_root_text(&self, _value: &str) -> Result<(), RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_root_text(&self) -> Result<String, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
    }
    #[cfg(feature = "test")]
    fn live_window(&self) -> Result<Window, RuntimeError> {
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
    fn live_component_text(&self) -> Result<String, RuntimeError> {
        Err(RuntimeError::UnsupportedKind)
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

    fn close_scheduler(&self) {
        self.pump.runtime().close_scheduler();
    }

    fn native_window_closed(&mut self) {
        self.pump.native_window_closed();
    }

    fn shutdown(&mut self) {
        self.pump.shutdown();
        self.pump.runtime().close_scheduler();
    }

    fn window_token(&self) -> WindowToken {
        self.pump.window_token()
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

    fn close_scheduler(&self) {
        self.pump().runtime().close_scheduler();
    }

    fn native_window_closed(&mut self) {
        Self::native_window_closed(self);
    }

    fn shutdown(&mut self) {
        Self::shutdown(self);
        self.pump().runtime().close_scheduler();
    }

    fn window_token(&self) -> WindowToken {
        self.pump().window_token()
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
    fn live_window(&self) -> Result<Window, RuntimeError> {
        self.pump().runtime().live_window()
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
    fn live_component_text(&self) -> Result<String, RuntimeError> {
        let native = self
            .pump()
            .root_native()
            .ok_or(RuntimeError::UnsupportedKind)?;
        self.pump().runtime().live_text(native)
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
            vec![Box::new(RenderLoop::new(
                WinUiRuntime::with_application(application),
                root,
            ))]
        })
    }

    pub fn run_windows<F, I>(roots: I) -> windows_core::Result<()>
    where
        F: FnMut(&mut Hooks) -> Element + 'static,
        I: IntoIterator<Item = F>,
    {
        let roots = roots.into_iter().collect::<Vec<_>>();
        if roots.is_empty() {
            return Err(windows_core::Error::new(
                E_INVALIDARG,
                "at least one window is required",
            ));
        }
        Self::run_with(move |application| {
            roots
                .into_iter()
                .map(|root| {
                    Box::new(RenderLoop::new(
                        WinUiRuntime::with_application(application.clone()),
                        root,
                    )) as Box<dyn LivePump>
                })
                .collect()
        })
    }

    pub fn run_component<C: Component>(props: C::Props) -> windows_core::Result<()> {
        Self::run_with(move |application| {
            vec![Box::new(ComponentLoop {
                pump: Pump::new(WinUiRuntime::with_application(application)),
                root: Some(View::component::<C>(props)),
            })]
        })
    }

    fn run_with(
        create_pumps: impl FnOnce(Application) -> Vec<Box<dyn LivePump>> + 'static,
    ) -> windows_core::Result<()> {
        initialize_ui_thread()?;
        let create_pumps = Rc::new(RefCell::new(Some(create_pumps)));
        let result = Rc::new(RefCell::new(Ok(())));
        let callback_result = Rc::clone(&result);

        let start = Application::Start(&ApplicationInitializationCallback::new(move |_| {
            let application = Rc::new(RefCell::new(None));
            let launch_application = Rc::clone(&application);
            let launch_result = Rc::clone(&callback_result);
            let launch_create_pumps = Rc::clone(&create_pumps);
            let on_launched = Box::new(move || {
                let launched: windows_core::Result<()> = (|| {
                    let application = launch_application
                        .borrow_mut()
                        .take()
                        .ok_or_else(|| windows_core::Error::new(E_FAIL, "missing application"))?;
                    install_xaml_controls_resources(&application)?;
                    let create_pumps = launch_create_pumps.borrow_mut().take().unwrap();
                    let mut pumps = create_pumps(application.clone()).into_iter();
                    let mut primary_pump = pumps.next().ok_or_else(|| {
                        windows_core::Error::new(E_INVALIDARG, "at least one window is required")
                    })?;
                    primary_pump.mount().map_err(pump_error)?;
                    let primary = primary_pump.window_token();
                    let mut windows = HashMap::with_capacity(pumps.len() + 1);
                    assert!(windows.insert(primary, primary_pump).is_none());
                    HOST.with(|host| {
                        *host.borrow_mut() = Some(LiveHost {
                            _application: application,
                            closed_in_flight: HashSet::new(),
                            fault: None,
                            in_flight: HashSet::new(),
                            #[cfg(feature = "test")]
                            primary,
                            windows,
                        });
                    });
                    let pumps = pumps.collect::<Vec<_>>();
                    if !pumps.is_empty() {
                        let dispatcher = DispatcherQueue::GetForCurrentThread()?;
                        let pumps = Rc::new(RefCell::new(Some(pumps)));
                        let mount = DispatcherQueueHandler::new(move || {
                            let Some(pumps) = pumps.borrow_mut().take() else {
                                return;
                            };
                            for mut pump in pumps {
                                if let Err(error) = pump.mount() {
                                    let error = pump_error(error);
                                    eprintln!(
                                        "windows-reactor-next additional window fault: {error}"
                                    );
                                    HOST.with(|host| {
                                        if let Some(host) = host.borrow_mut().as_mut() {
                                            host.fault = Some(error);
                                        }
                                    });
                                    exit_ui_thread();
                                    return;
                                }
                                let token = pump.window_token();
                                HOST.with(|host| {
                                    if let Some(host) = host.borrow_mut().as_mut() {
                                        assert!(host.windows.insert(token, pump).is_none());
                                    }
                                });
                            }
                        });
                        if !dispatcher
                            .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &mount)?
                        {
                            return Err(windows_core::Error::new(
                                E_FAIL,
                                "dispatcher rejected additional window mounting",
                            ));
                        }
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
        let host_result = host
            .and_then(|mut host| {
                for pump in host.windows.values_mut() {
                    pump.shutdown();
                }
                host.fault
            })
            .map_or(Ok(()), Err);
        let scheduler_result = SCHEDULER_FAULT
            .with(|fault| fault.borrow_mut().take())
            .map_or(Ok(()), Err);
        start
            .and(callback_result)
            .and(host_result)
            .and(scheduler_result)
    }
}

pub(crate) fn dispatch_native_events(token: WindowToken) {
    HOST.with(|host| {
        let Some(mut live) = ({
            let mut host = host.borrow_mut();
            let Some(host) = host.as_mut() else {
                return;
            };
            let live = host.windows.remove(&token);
            if live.is_some() {
                host.in_flight.insert(token);
            }
            live
        }) else {
            return;
        };
        #[cfg(feature = "test")]
        LIVE_TEST_DISPATCHES.with(|count| count.set(count.get().saturating_add(1)));
        let mut retry = false;
        let mut fault = None;
        match live.dispatch_events() {
            Ok(()) => retry = live.native_work_pending(),
            Err(error) => {
                let error = pump_error(error);
                eprintln!("windows-reactor-next fault: {error}");
                fault = Some(error);
                live.shutdown();
                exit_ui_thread();
            }
        }
        let closed = host
            .borrow()
            .as_ref()
            .is_some_and(|host| host.closed_in_flight.contains(&token));
        #[cfg(feature = "test")]
        if !closed
            && LIVE_TEST_REARM.with(|rearm| rearm.replace(false))
            && let Err(error) = live.schedule_retry()
        {
            fault = Some(runtime_error(error));
            live.shutdown();
            exit_ui_thread();
        }
        if !closed
            && retry
            && let Err(error) = live.schedule_retry()
        {
            fault = Some(runtime_error(error));
            live.shutdown();
            exit_ui_thread();
        }
        let mut finalize = None;
        if let Some(host) = host.borrow_mut().as_mut() {
            host.in_flight.remove(&token);
            let closed = host.closed_in_flight.remove(&token);
            if let Some(error) = fault {
                host.fault = Some(error);
            } else if closed {
                finalize = Some((live, host.windows.is_empty() && host.in_flight.is_empty()));
            } else {
                host.windows.insert(token, live);
            }
        }
        if let Some((live, empty)) = finalize {
            finalize_closed_window(live, empty);
        }
    });
}

pub(crate) fn dispatch_window_closed(token: WindowToken) {
    let (live, empty) = HOST.with(|host| {
        let mut host = host.borrow_mut();
        let Some(host) = host.as_mut() else {
            return (None, false);
        };
        if host.in_flight.contains(&token) {
            host.closed_in_flight.insert(token);
            return (None, false);
        }
        let live = host.windows.remove(&token);
        (live, host.windows.is_empty() && host.in_flight.is_empty())
    });
    if let Some(live) = live {
        finalize_closed_window(live, empty);
    }
}

fn finalize_closed_window(mut live: Box<dyn LivePump>, empty: bool) {
    live.close_scheduler();
    live.native_window_closed();
    let pending = Rc::new(RefCell::new(Some(live)));
    let pending_drop = Rc::clone(&pending);
    let drop_window = DispatcherQueueHandler::new(move || {
        drop(pending_drop.borrow_mut().take());
        if empty {
            #[cfg(feature = "test")]
            finish_live_closed_test();
            #[cfg(not(feature = "test"))]
            exit_ui_thread();
        }
    });
    let queued = DispatcherQueue::GetForCurrentThread().and_then(|dispatcher| {
        dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::High, &drop_window)
    });
    if !matches!(queued, Ok(true)) {
        eprintln!("windows-reactor-next could not finalize a closed window");
        std::process::abort();
    }
}

#[cfg(feature = "test")]
pub fn schedule_live_controlled_repair_test(initial_success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(150));
        let edit_dispatcher = dispatcher.clone();
        let edit = DispatcherQueueHandler::new(move || {
            let edited = HOST.with(|host| {
                host.borrow().as_ref().map(|host| {
                    (
                        host.primary()
                            .ok_or(RuntimeError::MissingApplication)
                            .and_then(|pump| pump.live_set_root_text("native")),
                        host.secondary()
                            .ok_or(RuntimeError::MissingApplication)
                            .and_then(|pump| pump.live_set_root_text("secondary-native")),
                    )
                })
            });
            if !matches!(edited, Some((Ok(()), Ok(())))) {
                eprintln!("controlled repair fixture could not edit both roots: {edited:?}");
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
            host.borrow().as_ref().map(|host| {
                (
                    host.primary().map(LivePump::live_root_text),
                    host.secondary().map(LivePump::live_root_text),
                )
            })
        });
        let repaired = matches!(
            text.as_ref(),
            Some((Some(Ok(primary)), Some(Ok(secondary))))
                if primary == "fixed" && secondary == "second"
        );
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
                .and_then(LiveHost::primary)
                .is_some_and(LivePump::live_rejection_then_retry)
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
    let window = HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::secondary_window_for_test)
    });
    if window.is_none_or(|window| window.Close().is_err()) {
        eprintln!("live backend fixture did not isolate and close its second window");
        std::process::exit(1);
    }
    let dispatcher = match DispatcherQueue::GetForCurrentThread() {
        Ok(dispatcher) => dispatcher,
        Err(error) => {
            eprintln!("window closure fixture has no dispatcher: {error}");
            std::process::exit(1);
        }
    };
    if queue_live_secondary_close_verification(dispatcher, 8).is_err() {
        std::process::exit(1);
    }
}

#[cfg(feature = "test")]
fn queue_live_secondary_close_verification(
    dispatcher: DispatcherQueue,
    attempts: u8,
) -> windows_core::Result<()> {
    let next_dispatcher = dispatcher.clone();
    let verify = DispatcherQueueHandler::new(move || {
        let closed = HOST.with(|host| {
            host.borrow().as_ref().is_some_and(|host| {
                host.windows.len() == 1
                    && host
                        .primary()
                        .is_some_and(|pump| pump.live_root_text().as_deref() == Ok("fixed"))
            })
        });
        if closed {
            continue_live_backend_test();
            return;
        }
        if attempts == 0
            || queue_live_secondary_close_verification(next_dispatcher.clone(), attempts - 1)
                .is_err()
        {
            eprintln!("live backend fixture retained its closed second window");
            std::process::exit(1);
        }
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &verify)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected window closure verification",
        ))
    }
}

#[cfg(feature = "test")]
fn continue_live_backend_test() {
    let Some(mut live) = HOST.with(|host| host.borrow_mut().take()) else {
        eprintln!("live backend fixture lost its host");
        std::process::exit(1);
    };
    if !live.primary_mut().is_some_and(LivePump::live_dense_reorder) {
        eprintln!("live backend fixture did not apply a dense keyed reorder");
        std::process::exit(1);
    }
    if !live
        .primary_mut()
        .is_some_and(LivePump::live_fragment_anchor)
    {
        eprintln!("live backend fixture did not apply empty and fragment transitions");
        std::process::exit(1);
    }
    if !live
        .primary_mut()
        .is_some_and(LivePump::live_component_update)
    {
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
                .and_then(LiveHost::primary)
                .is_some_and(LivePump::live_component_message_result)
        });
        if passed {
            finish_live_component_test();
            return;
        }
        if attempts == 0
            || queue_live_component_verification(next_dispatcher.clone(), attempts - 1).is_err()
        {
            let state = HOST.with(|host| {
                host.borrow().as_ref().map(|host| {
                    (
                        host.windows.len(),
                        host.primary().map(LivePump::live_component_text),
                        host.primary().map(LivePump::native_work_pending),
                    )
                })
            });
            eprintln!(
                "component scheduler fixture did not drain and rearm its message backlog: \
                 state={state:?}, dispatches={}",
                LIVE_TEST_DISPATCHES.with(|count| count.get()),
            );
            eprintln!(
                "component counts: creates={}, setups={}, cleanups={}",
                LIVE_COMPONENT_CREATES.with(|count| count.get()),
                LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get()),
                LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get()),
            );
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
    let window = HOST.with(|host| {
        host.borrow()
            .as_ref()
            .and_then(LiveHost::primary_window_for_test)
    });
    if window.is_none_or(|window| window.Close().is_err()) {
        eprintln!("component scheduler fixture could not close its primary window");
        std::process::exit(1);
    }
}

#[cfg(feature = "test")]
fn finish_live_closed_test() {
    if LIVE_COMPONENT_EFFECT_SETUPS.with(|count| count.get()) != 1
        || LIVE_COMPONENT_EFFECT_CLEANUPS.with(|count| count.get()) != 1
        || live_test_cleanup_count() != 1
    {
        eprintln!("live component effect setup or cleanup count was incorrect");
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
