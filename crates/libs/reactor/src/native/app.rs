use super::app_shim::{create_application, install_xaml_controls_resources};
use super::bindings::*;
use super::bootstrap;
use super::transient_menu::TransientMenuHost;
use super::{NativeWindow, WinUiAdapter};
use crate::{
    Callback, ColorScheme, Component, ComponentContext, ComponentHost, ComponentNode,
    ComponentUiServices, Menu, View, ViewContext, WindowPolicy, WindowPublication, WindowSize,
    component,
};
use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use windows_core::Interface;

windows_core::link!("kernel32.dll" "system" fn FindResourceW(module: *mut std::ffi::c_void, name: *const u16, resource_type: *const u16) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(name: *const u16) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LoadResource(module: *mut std::ffi::c_void, resource: *mut std::ffi::c_void) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn LockResource(resource: *mut std::ffi::c_void) -> *mut std::ffi::c_void);
windows_core::link!("kernel32.dll" "system" fn SizeofResource(module: *mut std::ffi::c_void, resource: *mut std::ffi::c_void) -> u32);

thread_local! {
    static APP_CALLBACKS: RefCell<HashMap<u64, Rc<dyn Fn() -> windows_core::Result<()>>>> =
        RefCell::new(HashMap::new());
    static APP_FAULT: RefCell<Option<windows_core::Error>> = const { RefCell::new(None) };
    static APP_LIFETIME: RefCell<Option<AppLifetime>> = const { RefCell::new(None) };
    static APP_TIMERS: RefCell<HashMap<u64, DispatcherTimerState>> = RefCell::new(HashMap::new());
    static APP_TRANSIENT_MENU: RefCell<Option<TransientMenuHost>> = const { RefCell::new(None) };
    static APP_COMPONENT_APPLICATION: RefCell<Option<LiveApplication>> = const { RefCell::new(None) };
}

static NEXT_APP_CALLBACK: AtomicU64 = AtomicU64::new(1);
static NEXT_APP_TIMER: AtomicU64 = AtomicU64::new(1);
const COMPONENT_DRAIN_BUDGET: usize = 64;
const WINDOW_WORK_BUDGET: usize = 16;
const WINDOW_WORK_CAPACITY: usize = 4_096;

struct AppLifetime {
    _resource: Box<dyn Any>,
    _application: Application,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

impl ScreenPoint {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct AppContext {
    dispatcher: DispatcherQueue,
    services: Arc<DispatcherComponentServices>,
    _ui_thread: PhantomData<Rc<()>>,
}

impl AppContext {
    fn new(dispatcher: DispatcherQueue) -> Self {
        Self {
            services: Arc::new(DispatcherComponentServices {
                dispatcher: dispatcher.clone(),
            }),
            dispatcher,
            _ui_thread: PhantomData,
        }
    }

    pub fn proxy(&self) -> AppProxy {
        AppProxy {
            dispatcher: self.dispatcher.clone(),
        }
    }

    pub(crate) fn ui_callback_once(
        callback: impl FnOnce() -> windows_core::Result<()> + 'static,
    ) -> windows_core::Result<AppCallback> {
        let dispatcher = DispatcherQueue::GetForCurrentThread()?;
        let proxy = AppProxy { dispatcher };
        let id = NEXT_APP_CALLBACK.fetch_add(1, Ordering::Relaxed);
        let callback = RefCell::new(Some(callback));
        APP_CALLBACKS.with(|callbacks| {
            assert!(
                callbacks
                    .borrow_mut()
                    .insert(
                        id,
                        Rc::new(move || {
                            APP_CALLBACKS.with(|callbacks| {
                                callbacks.borrow_mut().remove(&id);
                            });
                            callback.borrow_mut().take().unwrap()()
                        }),
                    )
                    .is_none()
            );
        });
        Ok(AppCallback { proxy, id })
    }

    pub fn component_services(&self) -> Arc<dyn crate::ComponentServices> {
        self.services.clone()
    }

    /// Registers application-lifetime work that can be invoked through the dispatcher.
    ///
    /// The registration remains active until application shutdown. Use component effects,
    /// observations, or other scoped handles for shorter-lived work.
    pub fn callback(
        &self,
        callback: impl Fn() -> windows_core::Result<()> + 'static,
    ) -> AppCallback {
        let id = NEXT_APP_CALLBACK.fetch_add(1, Ordering::Relaxed);
        APP_CALLBACKS.with(|callbacks| {
            assert!(
                callbacks
                    .borrow_mut()
                    .insert(id, Rc::new(callback))
                    .is_none()
            );
        });
        AppCallback {
            proxy: self.proxy(),
            id,
        }
    }

    pub fn exit(&self) -> windows_core::Result<()> {
        exit_application()
    }

    pub fn show_menu_at(&self, position: ScreenPoint, menu: Menu) -> windows_core::Result<()> {
        let handle = APP_TRANSIENT_MENU.with(|host| {
            let mut host = host.borrow_mut();
            if host.is_none() {
                *host = Some(TransientMenuHost::new(self.dispatcher.clone())?);
            }
            Ok::<_, windows_core::Error>(host.as_ref().unwrap().handle())
        })?;
        handle.show(position, menu)
    }

    pub fn open_component_window<C: Component>(&self, input: C::Input) -> windows_core::Result<()> {
        self.open_component_window_with_policy::<C>(input, WindowPolicy::new())
    }

    pub fn open_component_window_with_policy<C: Component>(
        &self,
        input: C::Input,
        policy: WindowPolicy,
    ) -> windows_core::Result<()> {
        APP_COMPONENT_APPLICATION.with(|application| {
            let mut application = application.borrow_mut();
            if application.is_none() {
                *application = Some(LiveApplication::new(self, false)?);
            }
            application
                .as_ref()
                .unwrap()
                .open(component::<C>("root", input), policy)
        })
    }
}

struct DispatcherTimerRegistration {
    cancelled: AtomicBool,
    dispatcher: DispatcherQueue,
    id: u64,
    ui_thread: u32,
}

impl crate::ComponentTimerRegistration for DispatcherTimerRegistration {
    fn cancel(&self) {
        if self.cancelled.swap(true, Ordering::AcqRel) {
            return;
        }
        if windows_threading::thread_id() == self.ui_thread {
            cancel_timer(self.id);
            return;
        }
        let id = self.id;
        let handler = DispatcherQueueHandler::new(move || cancel_timer(id));
        _ = self
            .dispatcher
            .TryEnqueueWithPriority(DispatcherQueuePriority::High, &handler);
    }
}

struct DispatcherComponentServices {
    dispatcher: DispatcherQueue,
}

struct DispatcherTimerState {
    callback: Option<Box<dyn FnOnce() + Send>>,
    tick: Option<windows_core::EventRevoker>,
    timer: DispatcherQueueTimer,
}

impl crate::ComponentServices for DispatcherComponentServices {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>) {
        windows_threading::submit(work);
    }

    fn set_timeout(
        &self,
        delay: Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn crate::ComponentTimerRegistration> {
        let id = NEXT_APP_TIMER.fetch_add(1, Ordering::Relaxed);
        let timer = self.dispatcher.CreateTimer().unwrap();
        timer
            .SetInterval(windows_time::TimeSpan::try_from(delay).unwrap())
            .unwrap();
        timer.SetIsRepeating(false).unwrap();
        let tick = timer.Tick(move |_, _| fire_timer(id)).unwrap();
        APP_TIMERS.with(|timers| {
            assert!(
                timers
                    .borrow_mut()
                    .insert(
                        id,
                        DispatcherTimerState {
                            callback: Some(callback),
                            tick: Some(tick),
                            timer: timer.clone(),
                        },
                    )
                    .is_none()
            );
        });
        timer.Start().unwrap();
        Arc::new(DispatcherTimerRegistration {
            cancelled: AtomicBool::new(false),
            dispatcher: self.dispatcher.clone(),
            id,
            ui_thread: windows_threading::thread_id(),
        })
    }
}

fn cancel_timer(id: u64) {
    APP_TIMERS.with(|timers| {
        if let Some(mut state) = timers.borrow_mut().remove(&id) {
            state.callback.take();
            _ = state.timer.Stop();
            state.tick.take();
        }
    });
}

fn fire_timer(id: u64) {
    APP_TIMERS.with(|timers| {
        let Some(mut state) = timers.borrow_mut().remove(&id) else {
            return;
        };
        _ = state.timer.Stop();
        state.tick.take();
        if let Some(callback) = state.callback.take() {
            callback();
        }
    });
}

#[derive(Clone)]
pub struct AppProxy {
    dispatcher: DispatcherQueue,
}

impl AppProxy {
    pub fn dispatch(
        &self,
        callback: impl FnOnce(&AppContext) -> windows_core::Result<()> + Send + 'static,
    ) -> windows_core::Result<()> {
        let callback = Arc::new(Mutex::new(Some(callback)));
        let invoke = Arc::clone(&callback);
        let dispatcher = self.dispatcher.clone();
        let handler = DispatcherQueueHandler::new(move || {
            let Some(callback) = invoke.lock().unwrap().take() else {
                return;
            };
            if let Err(error) = callback(&AppContext::new(dispatcher.clone())) {
                report_error(error);
            }
        });
        if self
            .dispatcher
            .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler)?
        {
            Ok(())
        } else {
            Err(windows_core::Error::new(
                E_FAIL,
                "application dispatcher rejected work",
            ))
        }
    }

    pub fn exit(&self) -> windows_core::Result<()> {
        self.dispatch(AppContext::exit)
    }
}

/// Application-lifetime dispatcher work registered by [`AppContext::callback`].
///
/// Clones refer to the same registration. Dropping the handles does not cancel it.
#[derive(Clone)]
pub struct AppCallback {
    proxy: AppProxy,
    id: u64,
}

impl AppCallback {
    pub fn invoke(&self) -> windows_core::Result<()> {
        let id = self.id;
        self.proxy
            .dispatch(move |_| invoke_registered_app_callback(id))
    }

    pub(crate) fn cancel(self) {
        APP_CALLBACKS.with(|callbacks| {
            callbacks.borrow_mut().remove(&self.id);
        });
    }
}

fn invoke_registered_app_callback(id: u64) -> windows_core::Result<()> {
    let callback = APP_CALLBACKS.with(|callbacks| callbacks.borrow().get(&id).map(Rc::clone));
    if let Some(callback) = callback {
        callback()?;
    }
    Ok(())
}

pub struct App;

struct StaticView;

impl Component for StaticView {
    type Input = View;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        input.clone()
    }
}

struct ComponentWindowState {
    host: ComponentHost<WinUiAdapter>,
    window: NativeWindow,
    lifecycle: ComponentWindowLifecycle,
    window_color_scheme: Option<(u64, Callback<ColorScheme>)>,
    window_size: Option<(u64, Callback<WindowSize>)>,
    window_observation_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ComponentWindowLifecycle {
    Open,
    Closing,
    Closed,
}

impl ComponentWindowLifecycle {
    fn begin_close(&mut self) -> bool {
        if *self != Self::Open {
            return false;
        }
        *self = Self::Closing;
        true
    }

    fn mark_closed(&mut self) {
        *self = Self::Closed;
    }
}

struct ComponentWindow {
    drain: AppCallback,
    state: Rc<RefCell<Option<ComponentWindowState>>>,
}

impl ComponentWindow {
    fn dispose(self) {
        self.drain.cancel();
        drop(self.state);
    }
}

enum WindowWork {
    Activate(u64),
    Close(u64),
    Closed(u64),
    Operation {
        window: u64,
        operation: Box<dyn FnOnce(*mut core::ffi::c_void)>,
    },
    Publish {
        publication: WindowPublication,
        window: u64,
    },
    Size {
        generation: u64,
        size: WindowSize,
        window: u64,
    },
    ColorScheme {
        generation: u64,
        scheme: ColorScheme,
        window: u64,
    },
    Open {
        policy: WindowPolicy,
        root: ComponentNode,
    },
}

#[derive(Default)]
struct LiveWindowServices {
    active: Cell<bool>,
    pending: RefCell<VecDeque<WindowWork>>,
    wake: RefCell<Option<AppCallback>>,
    wake_pending: Cell<bool>,
}

impl LiveWindowServices {
    fn activate(&self, wake: AppCallback) {
        self.active.set(true);
        *self.wake.borrow_mut() = Some(wake);
        if !self.pending.borrow().is_empty() {
            self.rearm();
        }
    }

    fn deactivate(&self) {
        self.active.set(false);
        self.pending.borrow_mut().clear();
        self.wake.borrow_mut().take();
        self.wake_pending.set(false);
    }

    fn has_pending(&self) -> bool {
        !self.pending.borrow().is_empty()
    }

    fn push(&self, work: WindowWork) -> bool {
        if !self.active.get() {
            return false;
        }
        {
            let mut pending = self.pending.borrow_mut();
            if pending.len() >= WINDOW_WORK_CAPACITY {
                return false;
            }
            pending.push_back(work);
        }
        self.rearm();
        true
    }

    fn push_durable(&self, work: WindowWork, matches: impl Fn(&WindowWork) -> bool) -> bool {
        if !self.active.get() {
            return false;
        }
        // Committed state cannot be rejected. Each caller matches one semantic owner so repeated
        // updates replace one bounded queue entry instead of growing with event volume.
        {
            let mut pending = self.pending.borrow_mut();
            if let Some(index) = pending.iter().rposition(matches) {
                pending.remove(index);
            }
            pending.push_back(work);
        }
        self.rearm();
        true
    }

    fn publish(&self, publication: WindowPublication, window: u64) -> bool {
        self.push_durable(
            WindowWork::Publish {
                publication,
                window,
            },
            |work| matches!(work, WindowWork::Publish { window: current, .. } if *current == window),
        )
    }

    fn size(&self, generation: u64, size: WindowSize, window: u64) -> bool {
        self.push_durable(
            WindowWork::Size {
                generation,
                size,
                window,
            },
            |work| {
                matches!(
                    work,
                    WindowWork::Size {
                        generation: current_generation,
                        window: current_window,
                        ..
                    } if *current_generation == generation && *current_window == window
                )
            },
        )
    }

    fn color_scheme(&self, generation: u64, scheme: ColorScheme, window: u64) -> bool {
        self.push_durable(
            WindowWork::ColorScheme {
                generation,
                scheme,
                window,
            },
            |work| {
                matches!(
                    work,
                    WindowWork::ColorScheme {
                        generation: current_generation,
                        window: current_window,
                        ..
                    } if *current_generation == generation && *current_window == window
                )
            },
        )
    }

    fn push_critical(&self, work: WindowWork) -> bool {
        if !self.active.get() {
            return false;
        }
        self.pending.borrow_mut().push_back(work);
        self.rearm();
        true
    }

    fn pop(&self) -> Option<WindowWork> {
        self.pending.borrow_mut().pop_front()
    }

    fn rearm(&self) {
        if self.wake_pending.replace(true) {
            return;
        }
        if let Some(wake) = self.wake.borrow().as_ref()
            && let Err(error) = wake.invoke()
        {
            self.wake_pending.set(false);
            report_error(error);
        }
    }

    fn begin_drain(&self) {
        self.wake_pending.set(false);
    }
}

struct LiveComponentWindowServices {
    application: Rc<LiveWindowServices>,
    window: u64,
}

impl ComponentUiServices for LiveComponentWindowServices {
    fn open_window(&self, root: ComponentNode, policy: WindowPolicy) -> bool {
        self.application.push(WindowWork::Open { policy, root })
    }

    fn publish_window(&self, publication: WindowPublication) {
        _ = self.application.publish(publication, self.window);
    }

    fn activate_window(&self) -> bool {
        self.application.push(WindowWork::Activate(self.window))
    }

    fn close_window(&self) -> bool {
        self.application.push(WindowWork::Close(self.window))
    }

    fn run_window(&self, operation: Box<dyn FnOnce(*mut core::ffi::c_void)>) -> bool {
        self.application.push(WindowWork::Operation {
            window: self.window,
            operation,
        })
    }
}

struct LiveApplicationState {
    application: AppProxy,
    context: AppContext,
    exit_when_empty: bool,
    next_window: u64,
    windows: HashMap<u64, ComponentWindow>,
}

struct LiveApplication {
    drain_requests: AppCallback,
    services: Rc<LiveWindowServices>,
    state: Rc<RefCell<LiveApplicationState>>,
}

impl LiveApplication {
    fn new(context: &AppContext, exit_when_empty: bool) -> windows_core::Result<Self> {
        let services = Rc::new(LiveWindowServices::default());
        let state = Rc::new(RefCell::new(LiveApplicationState {
            application: context.proxy(),
            context: context.clone(),
            exit_when_empty,
            next_window: 1,
            windows: HashMap::new(),
        }));
        let drain_state = Rc::clone(&state);
        let drain_services = Rc::clone(&services);
        let drain_requests =
            context.callback(move || drain_window_requests(&drain_state, &drain_services));
        services.activate(drain_requests.clone());
        Ok(Self {
            drain_requests,
            services,
            state,
        })
    }

    fn open(&self, root: ComponentNode, policy: WindowPolicy) -> windows_core::Result<()> {
        open_component_window(&self.state, &self.services, root, policy)
    }
}

impl Drop for LiveApplication {
    fn drop(&mut self) {
        self.services.deactivate();
        self.drain_requests.clone().cancel();
        let windows = std::mem::take(&mut self.state.borrow_mut().windows);
        for (_, window) in windows {
            window.dispose();
        }
    }
}

#[cfg(feature = "test")]
fn with_primary_component_window<T>(
    operation: impl FnOnce(&ComponentWindowState) -> Result<T, &'static str>,
) -> Result<T, &'static str> {
    APP_COMPONENT_APPLICATION.with(|application| {
        let application = application.borrow();
        let application = application.as_ref().ok_or("application is not running")?;
        let state = application.state.borrow();
        let window = state.windows.values().next().ok_or("window is not open")?;
        let window = window.state.borrow();
        let window = window.as_ref().ok_or("window is closed")?;
        operation(window)
    })
}

#[cfg(feature = "test")]
pub fn bring_live_virtual_index(index: usize) -> Result<(), &'static str> {
    with_primary_component_window(|window| {
        let collection = window
            .host
            .runtime()
            .graph()
            .objects()
            .find(|object| {
                window.host.runtime().graph().kind(*object)
                    == Some(crate::ObjectType::ItemsRepeater)
            })
            .ok_or("virtual collection is not mounted")?;
        window
            .host
            .runtime()
            .adapter()
            .realize_virtual_item(collection, index)
            .map_err(|_| "virtual index could not be realized")
    })
}

#[cfg(feature = "test")]
pub fn live_virtual_shell_counts() -> Result<(usize, usize), &'static str> {
    with_primary_component_window(|window| {
        Ok(window.host.runtime().adapter().virtual_shell_counts())
    })
}

#[cfg(feature = "test")]
pub struct LiveTickSubscription {
    _tick: windows_core::EventRevoker,
    timer: DispatcherQueueTimer,
}

#[cfg(feature = "test")]
impl Drop for LiveTickSubscription {
    fn drop(&mut self) {
        _ = self.timer.Stop();
    }
}

#[cfg(feature = "test")]
pub fn subscribe_live_tick(
    tick_callback: impl Fn() + 'static,
) -> windows_core::Result<LiveTickSubscription> {
    subscribe_live_interval(Duration::from_millis(16), tick_callback)
}

#[cfg(feature = "test")]
pub fn subscribe_live_interval(
    interval: Duration,
    tick_callback: impl Fn() + 'static,
) -> windows_core::Result<LiveTickSubscription> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let timer = dispatcher.CreateTimer()?;
    timer.SetInterval(windows_time::TimeSpan::try_from(interval).unwrap())?;
    timer.SetIsRepeating(true)?;
    let tick = timer.Tick(move |_, _| {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&tick_callback)).is_err() {
            std::process::abort();
        }
    })?;
    timer.Start()?;
    Ok(LiveTickSubscription { _tick: tick, timer })
}

#[cfg(feature = "test")]
#[must_use = "dropping the subscription stops rendering notifications"]
pub struct LiveRenderingSubscription {
    _rendering: windows_core::EventRevoker,
}

#[cfg(feature = "test")]
pub fn subscribe_live_rendering(
    rendering_callback: impl Fn() + 'static,
) -> windows_core::Result<LiveRenderingSubscription> {
    let rendering = CompositionTarget::Rendering(move |_, _| {
        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(&rendering_callback)).is_err() {
            std::process::abort();
        }
    })?;
    Ok(LiveRenderingSubscription {
        _rendering: rendering,
    })
}

#[cfg(feature = "test")]
pub fn schedule_live_test_exit(success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let handler = DispatcherQueueHandler::new(move || {
        std::process::exit(i32::from(!success));
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

impl App {
    /// Runs one static view in a native window.
    pub fn run(view: impl Into<View>) -> windows_core::Result<()> {
        Self::run_component::<StaticView>(view.into())
    }

    pub fn run_component<C: Component>(input: C::Input) -> windows_core::Result<()> {
        Self::run_component_with_policy::<C>(input, WindowPolicy::new())
    }

    pub fn run_component_with_policy<C: Component>(
        input: C::Input,
        policy: WindowPolicy,
    ) -> windows_core::Result<()> {
        Self::run_with(move |context| {
            let application = LiveApplication::new(context, true)?;
            application.open(component::<C>("root", input), policy)?;
            APP_COMPONENT_APPLICATION.with(|current| {
                assert!(current.borrow_mut().replace(application).is_none());
            });
            Ok(())
        })
    }

    pub fn run_with<T>(
        startup: impl FnOnce(&AppContext) -> windows_core::Result<T> + 'static,
    ) -> windows_core::Result<()>
    where
        T: 'static,
    {
        bootstrap_runtime()?;
        initialize_ui_thread()?;

        let startup = Rc::new(RefCell::new(Some(startup)));
        let result = Rc::new(RefCell::new(Ok(())));
        let callback_result = Rc::clone(&result);
        let start = Application::Start(&ApplicationInitializationCallback::new(move |_| {
            let application = Rc::new(RefCell::new(None));
            let launch_application = Rc::clone(&application);
            let launch_startup = Rc::clone(&startup);
            let launch_result = Rc::clone(&callback_result);
            let on_launched = Box::new(move || {
                let launched = (|| {
                    let application = launch_application
                        .borrow_mut()
                        .take()
                        .ok_or_else(|| windows_core::Error::new(E_FAIL, "missing application"))?;
                    install_xaml_controls_resources(&application)?;
                    application
                        .cast::<IApplication3>()?
                        .SetDispatcherShutdownMode(DispatcherShutdownMode::OnExplicitShutdown)?;
                    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
                    let startup = launch_startup.borrow_mut().take().ok_or_else(|| {
                        windows_core::Error::new(E_FAIL, "missing application startup")
                    })?;
                    let resource = startup(&AppContext::new(dispatcher))?;
                    APP_LIFETIME.with(|lifetime| {
                        *lifetime.borrow_mut() = Some(AppLifetime {
                            _resource: Box::new(resource),
                            _application: application,
                        });
                    });
                    Ok(())
                })();
                if let Err(error) = launched {
                    *launch_result.borrow_mut() = Err(error);
                    exit_ui_thread();
                }
                Ok(())
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
        APP_COMPONENT_APPLICATION.with(|application| drop(application.borrow_mut().take()));
        APP_CALLBACKS.with(|callbacks| callbacks.borrow_mut().clear());
        APP_TIMERS.with(|timers| timers.borrow_mut().clear());
        APP_TRANSIENT_MENU.with(|host| drop(host.borrow_mut().take()));
        APP_LIFETIME.with(|lifetime| drop(lifetime.borrow_mut().take()));
        let fault = APP_FAULT
            .with(|fault| fault.borrow_mut().take())
            .map_or(Ok(()), Err);
        start.and(callback_result).and(fault)
    }
}

fn open_component_window(
    application: &Rc<RefCell<LiveApplicationState>>,
    services: &Rc<LiveWindowServices>,
    root: ComponentNode,
    policy: WindowPolicy,
) -> windows_core::Result<()> {
    let id = {
        let mut application = application.borrow_mut();
        let id = application.next_window;
        application.next_window += 1;
        id
    };
    let context = application.borrow().context.clone();
    let state = Rc::new(RefCell::new(None::<ComponentWindowState>));
    let ui_services: Rc<dyn ComponentUiServices> = Rc::new(LiveComponentWindowServices {
        application: Rc::clone(services),
        window: id,
    });
    let mut host = ComponentHost::mount_with_all_services(
        WinUiAdapter::default(),
        context.component_services(),
        ui_services,
        [root],
    )?;
    let drain_state = Rc::clone(&state);
    let drain = context.callback(move || drain_component_window(&drain_state));
    let wake = drain.clone();
    host.set_waker(move || {
        if let Err(error) = wake.invoke() {
            report_error(error);
        }
    });
    let native_drain_state = Rc::downgrade(&state);
    let wake = drain.clone();
    host.set_native_event_wakers(
        move || {
            if let Some(native_drain_state) = native_drain_state.upgrade()
                && let Err(error) = drain_component_window(&native_drain_state)
            {
                report_error(error);
            }
        },
        move || {
            if let Err(error) = wake.invoke() {
                report_error(error);
            }
        },
    );
    let wake = drain.clone();
    host.set_continuation_waker(move || {
        if let Err(error) = wake.invoke() {
            report_error(error);
        }
    });
    let root = host.runtime().graph().root().unwrap();
    let mut window = match host
        .runtime()
        .adapter()
        .open_window_with_policy(root, &policy)
    {
        Ok(window) => window,
        Err(error) => {
            drain.cancel();
            return Err(error.into());
        }
    };
    let close_services = Rc::clone(services);
    let native_closed_state = Rc::downgrade(&state);
    if let Err(error) = window.set_closed(move || {
        if let Some(native_closed_state) = native_closed_state.upgrade()
            && let Some(state) = native_closed_state.borrow_mut().as_mut()
        {
            state.lifecycle.mark_closed();
        }
        close_services.push_critical(WindowWork::Closed(id));
        Ok(())
    }) {
        drain.cancel();
        return Err(error.into());
    }
    *state.borrow_mut() = Some(ComponentWindowState {
        host,
        window,
        lifecycle: ComponentWindowLifecycle::Open,
        window_color_scheme: None,
        window_size: None,
        window_observation_generation: 0,
    });
    assert!(
        application
            .borrow_mut()
            .windows
            .insert(id, ComponentWindow { drain, state },)
            .is_none()
    );
    Ok(())
}

fn drain_component_window(
    state: &Rc<RefCell<Option<ComponentWindowState>>>,
) -> windows_core::Result<()> {
    let mut state = state.borrow_mut();
    let state = state
        .as_mut()
        .ok_or_else(|| windows_core::Error::new(E_FAIL, "component window is not initialized"))?;
    state.host.drain(COMPONENT_DRAIN_BUDGET)?;
    Ok(())
}

fn drain_window_requests(
    application: &Rc<RefCell<LiveApplicationState>>,
    services: &Rc<LiveWindowServices>,
) -> windows_core::Result<()> {
    services.begin_drain();
    for _ in 0..WINDOW_WORK_BUDGET {
        let Some(work) = services.pop() else {
            break;
        };
        match work {
            WindowWork::Activate(id) => {
                let window = {
                    let application = application.borrow();
                    application.windows.get(&id).and_then(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref()?;
                        (state.lifecycle == ComponentWindowLifecycle::Open)
                            .then(|| state.window.clone())
                    })
                };
                if let Some(window) = window {
                    window.activate()?;
                }
            }
            WindowWork::Close(id) => {
                let window = {
                    let application = application.borrow();
                    application.windows.get(&id).and_then(|window| {
                        let mut state = window.state.borrow_mut();
                        let state = state.as_mut()?;
                        state.lifecycle.begin_close().then(|| state.window.clone())
                    })
                };
                if let Some(native) = window
                    && let Err(error) = native.close()
                {
                    let application = application.borrow();
                    if let Some(window) = application.windows.get(&id)
                        && let Some(state) = window.state.borrow_mut().as_mut()
                        && state.lifecycle == ComponentWindowLifecycle::Closing
                    {
                        state.lifecycle = ComponentWindowLifecycle::Open;
                    }
                    return Err(error.into());
                }
            }
            WindowWork::Closed(id) => {
                close_component_window(application, services, id)?;
            }
            WindowWork::Operation { window, operation } => {
                let native = {
                    let application = application.borrow();
                    application.windows.get(&window).and_then(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref()?;
                        (state.lifecycle == ComponentWindowLifecycle::Open)
                            .then(|| state.window.clone())
                    })
                };
                if let Some(native) = native {
                    operation(native.raw_handle()?);
                }
            }
            WindowWork::Publish {
                publication,
                window,
            } => {
                let application = application.borrow();
                let Some(window_state) = application.windows.get(&window) else {
                    continue;
                };
                let mut state = window_state.state.borrow_mut();
                let Some(state) = state.as_mut() else {
                    continue;
                };
                if state.lifecycle != ComponentWindowLifecycle::Open {
                    continue;
                }
                state.window_observation_generation =
                    state.window_observation_generation.wrapping_add(1);
                let generation = state.window_observation_generation;
                let size_observer = publication.on_size.as_ref().map(|_| {
                    let services = Rc::clone(services);
                    Rc::new(move |size| {
                        _ = services.size(generation, size, window);
                    }) as Rc<dyn Fn(WindowSize)>
                });
                let color_scheme_observer = publication.on_color_scheme.as_ref().map(|_| {
                    let services = Rc::clone(services);
                    Rc::new(move |scheme| {
                        _ = services.color_scheme(generation, scheme, window);
                    }) as Rc<dyn Fn(ColorScheme)>
                });
                state.window.apply_publication(
                    publication.title.as_deref(),
                    publication.visuals.as_ref(),
                    color_scheme_observer,
                    size_observer,
                )?;
                state.window_color_scheme = publication
                    .on_color_scheme
                    .map(|callback| (generation, callback));
                state.window_size = publication.on_size.map(|callback| (generation, callback));
            }
            WindowWork::ColorScheme {
                generation,
                scheme,
                window,
            } => {
                let callback = {
                    let application = application.borrow();
                    application.windows.get(&window).and_then(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref()?;
                        (state.lifecycle == ComponentWindowLifecycle::Open)
                            .then_some(state.window_color_scheme.as_ref())
                            .flatten()
                            .filter(|(current, _)| *current == generation)
                            .map(|(_, callback)| callback.clone())
                    })
                };
                if let Some(callback) = callback {
                    callback.call(scheme);
                }
            }
            WindowWork::Size {
                generation,
                size,
                window,
            } => {
                let callback = {
                    let application = application.borrow();
                    application.windows.get(&window).and_then(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref()?;
                        (state.lifecycle == ComponentWindowLifecycle::Open)
                            .then_some(state.window_size.as_ref())
                            .flatten()
                            .filter(|(current, _)| *current == generation)
                            .map(|(_, callback)| callback.clone())
                    })
                };
                if let Some(callback) = callback {
                    callback.call(size);
                }
            }
            WindowWork::Open { policy, root } => {
                open_component_window(application, services, root, policy)?;
            }
        }
    }
    if services.has_pending() {
        services.rearm();
    } else {
        exit_if_no_windows(application)?;
    }
    Ok(())
}

fn close_component_window(
    application: &Rc<RefCell<LiveApplicationState>>,
    services: &Rc<LiveWindowServices>,
    id: u64,
) -> windows_core::Result<()> {
    let (window, exit, proxy) = {
        let mut application = application.borrow_mut();
        let window = application.windows.remove(&id);
        let exit = application.exit_when_empty
            && application.windows.is_empty()
            && !services.has_pending();
        (window, exit, application.application.clone())
    };
    if let Some(window) = window {
        window.dispose();
    }
    if exit {
        proxy.exit()?;
    }
    Ok(())
}

fn exit_if_no_windows(application: &Rc<RefCell<LiveApplicationState>>) -> windows_core::Result<()> {
    let application = application.borrow();
    if application.exit_when_empty && application.windows.is_empty() {
        application.application.exit()?;
    }
    Ok(())
}

pub(super) fn report_error(error: windows_core::Error) {
    APP_FAULT.with(|fault| {
        if fault.borrow().is_none() {
            *fault.borrow_mut() = Some(error);
        }
    });
    exit_ui_thread();
}

fn initialize_ui_thread() -> windows_core::Result<()> {
    unsafe {
        _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    let result = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
    if result == RPC_E_CHANGED_MODE {
        return Err(windows_core::Error::new(
            RPC_E_CHANGED_MODE,
            "WinUI requires an STA thread",
        ));
    }
    result.ok()
}

fn is_packaged_process() -> windows_core::Result<bool> {
    let mut length = 0;
    let result = unsafe { GetCurrentPackageFullName(&mut length, windows_core::PWSTR::null()) };
    match result {
        ERROR_INSUFFICIENT_BUFFER => Ok(true),
        APPMODEL_ERROR_NO_PACKAGE => Ok(false),
        _ => Err(windows_core::HRESULT::from(windows_core::WIN32_ERROR(result as u32)).into()),
    }
}

fn bootstrap_runtime() -> windows_core::Result<()> {
    if is_packaged_process()? {
        return Ok(());
    }
    if self_contained_manifest_present() {
        return if self_contained_runtime_present() {
            Ok(())
        } else {
            Err(windows_core::Error::new(
                windows_core::HRESULT(0x8007007e_u32 as i32),
                "self-contained Windows App Runtime files are missing",
            ))
        };
    }
    bootstrap::bootstrap()
}

fn self_contained_runtime_present() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|parent| parent.to_path_buf()))
        .is_some_and(|parent| parent.join("Microsoft.WindowsAppRuntime.dll").is_file())
}

#[allow(clippy::manual_dangling_ptr)]
fn self_contained_manifest_present() -> bool {
    const SELF_CONTAINED_MARKER: &str = "windows-reactor-self-contained";
    let marker = SELF_CONTAINED_MARKER.as_bytes();

    unsafe {
        let module = GetModuleHandleW(std::ptr::null());
        if module.is_null() {
            return false;
        }
        let resource = FindResourceW(module, 1usize as *const u16, 24usize as *const u16);
        if resource.is_null() {
            return false;
        }
        let size = SizeofResource(module, resource) as usize;
        let loaded = LoadResource(module, resource);
        if loaded.is_null() {
            return false;
        }
        let data = LockResource(loaded).cast::<u8>();
        if data.is_null() {
            return false;
        }
        std::slice::from_raw_parts(data, size)
            .windows(marker.len())
            .any(|window| window == marker)
    }
}

fn exit_application() -> windows_core::Result<()> {
    let result = Application::Current().and_then(|application| application.Exit());
    if result.is_err() {
        unsafe {
            PostQuitMessage(0);
        }
    }

    result
}

fn exit_ui_thread() {
    let handler = DispatcherQueueHandler::new(|| {
        _ = exit_application();
    });
    let queued = DispatcherQueue::GetForCurrentThread().and_then(|dispatcher| {
        dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::High, &handler)
    });
    if !matches!(queued, Ok(true)) {
        unsafe {
            PostQuitMessage(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canceled_queued_app_callback_is_ignored() {
        let id = NEXT_APP_CALLBACK.fetch_add(1, Ordering::Relaxed);
        let invoked = Rc::new(Cell::new(false));
        let callback_invoked = Rc::clone(&invoked);
        APP_CALLBACKS.with(|callbacks| {
            callbacks.borrow_mut().insert(
                id,
                Rc::new(move || {
                    callback_invoked.set(true);
                    Ok(())
                }),
            );
            callbacks.borrow_mut().remove(&id);
        });

        invoke_registered_app_callback(id).unwrap();

        assert!(!invoked.get());
    }

    #[test]
    fn component_window_lifecycle_serializes_close() {
        let mut lifecycle = ComponentWindowLifecycle::Open;
        assert!(lifecycle.begin_close());
        assert_eq!(lifecycle, ComponentWindowLifecycle::Closing);
        assert!(!lifecycle.begin_close());
        lifecycle.mark_closed();
        assert_eq!(lifecycle, ComponentWindowLifecycle::Closed);
        assert!(!lifecycle.begin_close());
    }

    #[test]
    fn routine_window_work_is_coalesced_at_the_latest_position() {
        let services = LiveWindowServices::default();
        services.active.set(true);
        assert!(services.publish(
            WindowPublication {
                title: Some("first".to_string()),
                ..Default::default()
            },
            1,
        ));
        assert!(services.push(WindowWork::Activate(1)));
        assert!(services.publish(
            WindowPublication {
                title: Some("second".to_string()),
                ..Default::default()
            },
            1,
        ));
        assert!(services.size(
            7,
            WindowSize {
                width: 100.0,
                height: 100.0,
            },
            1,
        ));
        assert!(services.size(
            7,
            WindowSize {
                width: 200.0,
                height: 150.0,
            },
            1,
        ));

        assert!(matches!(services.pop(), Some(WindowWork::Activate(1))));
        assert!(matches!(
            services.pop(),
            Some(WindowWork::Publish {
                publication: WindowPublication {
                    title: Some(title),
                    ..
                },
                window: 1,
            }) if title == "second"
        ));
        assert!(matches!(
            services.pop(),
            Some(WindowWork::Size {
                generation: 7,
                size: WindowSize {
                    width: 200.0,
                    height: 150.0,
                },
                window: 1,
            })
        ));
        assert!(services.pop().is_none());
    }

    #[test]
    fn durable_window_work_survives_routine_queue_saturation() {
        let services = LiveWindowServices::default();
        services.active.set(true);
        for window in 0..WINDOW_WORK_CAPACITY {
            assert!(services.push(WindowWork::Activate(window as u64)));
        }
        assert!(!services.push(WindowWork::Activate(WINDOW_WORK_CAPACITY as u64)));

        for title in ["first", "latest"] {
            assert!(services.publish(
                WindowPublication {
                    title: Some(title.to_string()),
                    ..Default::default()
                },
                7,
            ));
        }
        for width in [100.0, 200.0] {
            assert!(services.size(
                11,
                WindowSize {
                    width,
                    height: 150.0,
                },
                7,
            ));
        }
        for scheme in [ColorScheme::Light, ColorScheme::Dark] {
            assert!(services.color_scheme(11, scheme, 7));
        }

        assert_eq!(services.pending.borrow().len(), WINDOW_WORK_CAPACITY + 3);
        for _ in 0..WINDOW_WORK_CAPACITY {
            assert!(matches!(services.pop(), Some(WindowWork::Activate(_))));
        }
        assert!(matches!(
            services.pop(),
            Some(WindowWork::Publish {
                publication: WindowPublication {
                    title: Some(title),
                    ..
                },
                window: 7,
            }) if title == "latest"
        ));
        assert!(matches!(
            services.pop(),
            Some(WindowWork::Size {
                generation: 11,
                size: WindowSize { width: 200.0, .. },
                window: 7,
            })
        ));
        assert!(matches!(
            services.pop(),
            Some(WindowWork::ColorScheme {
                generation: 11,
                scheme: ColorScheme::Dark,
                window: 7,
            })
        ));
        assert!(services.pop().is_none());
    }
}
