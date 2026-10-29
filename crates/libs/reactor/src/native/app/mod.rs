use super::*;
use bindings::Application;
use bindings::*;

mod application;

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
        callback: impl FnOnce() + 'static,
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
                            callback.borrow_mut().take().unwrap()();
                            Ok(())
                        }),
                    )
                    .is_none()
            );
        });
        Ok(AppCallback { proxy, id })
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
                *host = Some(TransientMenuHost::new(
                    self.dispatcher.clone(),
                    MenuTheme::Application,
                )?);
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

impl ComponentTimerRegistration for DispatcherTimerRegistration {
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

impl ComponentServices for DispatcherComponentServices {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>) {
        windows_threading::submit(work);
    }

    fn set_timeout(
        &self,
        delay: Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn ComponentTimerRegistration> {
        let id = NEXT_APP_TIMER.fetch_add(1, Ordering::Relaxed);
        let timer = self.dispatcher.CreateTimer().unwrap();
        timer
            .SetInterval(TimeSpan::try_from(delay).unwrap())
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
        if self.enqueue(callback)? {
            Ok(())
        } else {
            Err(windows_core::Error::new(
                E_FAIL,
                "application dispatcher rejected work",
            ))
        }
    }

    /// Returns `false` when the dispatcher is shutting down and rejects the work.
    fn enqueue(
        &self,
        callback: impl FnOnce(&AppContext) -> windows_core::Result<()> + Send + 'static,
    ) -> windows_core::Result<bool> {
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
        self.dispatcher
            .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler)
    }

    pub fn exit(&self) -> windows_core::Result<()> {
        self.dispatch(AppContext::exit)
    }

    /// Queues application exit from Reactor's own drains. `exit_application` falls back to
    /// `PostQuitMessage`, and a dispatcher that rejects the work is already shutting down.
    fn queue_exit(&self) {
        self.enqueue(|_| {
            _ = exit_application();
            Ok(())
        })
        .unwrap();
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

    /// Queues the callback from any thread. Errors from the callback reach `run` on the UI
    /// thread; a dispatcher that is shutting down drops the wake.
    pub(crate) fn wake(&self) {
        let id = self.id;
        self.proxy
            .enqueue(move |_| invoke_registered_app_callback(id))
            .unwrap();
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
    host: ComponentHost<SharedWinUiAdapter>,
    window: NativeWindow,
    lifecycle: Rc<Cell<ComponentWindowLifecycle>>,
    color_scheme_generation: Option<u64>,
    size_generation: Option<u64>,
    placement_generation: Option<u64>,
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
    Input {
        root: ComponentNode,
        window: u64,
    },
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
    Placement {
        generation: u64,
        placement: WindowPlacement,
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
struct WindowDispatch {
    active: Cell<bool>,
    deferred: RefCell<Vec<AppCallback>>,
}

impl WindowDispatch {
    fn callback(
        self: &Rc<Self>,
        context: &AppContext,
        work: impl Fn() -> windows_core::Result<()> + 'static,
    ) -> AppCallback {
        let dispatch = Rc::clone(self);
        let registration = Rc::new(std::cell::OnceCell::<AppCallback>::new());
        let wake = Rc::clone(&registration);
        let callback = context.callback(move || {
            let wake = wake.get().unwrap();
            dispatch.run(&work).unwrap_or_else(|| {
                let mut deferred = dispatch.deferred.borrow_mut();
                if !deferred.iter().any(|callback| callback.id == wake.id) {
                    deferred.push(wake.clone());
                }
                Ok(())
            })
        });
        assert!(registration.set(callback.clone()).is_ok());
        callback
    }

    /// Runs `work` unless an owner is already active, then wakes the callbacks deferred meanwhile.
    fn run<R>(&self, work: impl FnOnce() -> R) -> Option<R> {
        if self.active.replace(true) {
            return None;
        }
        // Native calls can pump a nested dispatcher loop. All window and application drivers
        // share this boundary so a nested wake cannot reenter an active owner.
        let result = work();
        self.active.set(false);
        let deferred = std::mem::take(&mut *self.deferred.borrow_mut());
        for callback in deferred {
            callback.wake();
        }
        Some(result)
    }
}

#[derive(Default)]
struct LiveWindowServices {
    active: Cell<bool>,
    pending: RefCell<VecDeque<WindowWork>>,
    wake: RefCell<Option<AppCallback>>,
    wake_pending: Cell<bool>,
    dispatch: Rc<WindowDispatch>,
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
        let retired = {
            let mut pending = self.pending.borrow_mut();
            let retired = pending
                .iter()
                .rposition(matches)
                .and_then(|index| pending.remove(index));
            pending.push_back(work);
            retired
        };
        drop(retired);
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

    fn update_input(&self, root: ComponentNode, window: u64) -> bool {
        self.push_durable(
            WindowWork::Input { root, window },
            |work| matches!(work, WindowWork::Input { window: current, .. } if *current == window),
        )
    }

    fn take_publication(&self, window: u64) -> Option<WindowPublication> {
        let mut pending = self.pending.borrow_mut();
        let index = pending.iter().position(
            |work| matches!(work, WindowWork::Publish { window: current, .. } if *current == window),
        )?;
        let WindowWork::Publish { publication, .. } = pending.remove(index).unwrap() else {
            unreachable!()
        };
        Some(publication)
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

    fn placement(&self, generation: u64, placement: WindowPlacement, window: u64) -> bool {
        self.push_durable(
            WindowWork::Placement {
                generation,
                placement,
                window,
            },
            |work| {
                matches!(
                    work,
                    WindowWork::Placement { generation: current_generation, window: current, .. }
                        if *current_generation == generation && *current == window
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
        if let Some(wake) = self.wake.borrow().as_ref() {
            wake.wake();
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
    changed: Option<AppCallback>,
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
            changed: None,
        }));
        let drain_state = Rc::clone(&state);
        let drain_services = Rc::clone(&services);
        let drain_requests = services.dispatch.callback(context, move || {
            drain_window_requests(&drain_state, &drain_services);
            Ok(())
        });
        services.activate(drain_requests.clone());
        Ok(Self {
            drain_requests,
            services,
            state,
        })
    }

    fn open(&self, root: ComponentNode, policy: WindowPolicy) -> windows_core::Result<()> {
        open_component_window(&self.state, &self.services, root, policy).map(|_| ())
    }
}

impl Drop for LiveApplication {
    fn drop(&mut self) {
        self.services.deactivate();
        self.drain_requests.clone().cancel();
        let windows = std::mem::take(&mut self.state.borrow_mut().windows);
        for (_, window) in windows {
            let native = window.state.borrow().as_ref().and_then(|state| {
                (state.lifecycle.get() != ComponentWindowLifecycle::Closed)
                    .then(|| state.window.clone())
            });
            if let Some(native) = native {
                native.close().unwrap();
            }
            window.dispose();
        }
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
                    let application = launch_application.borrow_mut().take().unwrap();
                    install_xaml_controls_resources(&application)?;
                    application
                        .cast::<IApplication3>()?
                        .SetDispatcherShutdownMode(DispatcherShutdownMode::OnExplicitShutdown)?;
                    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
                    let startup = launch_startup.borrow_mut().take().unwrap();
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
        #[cfg(windows_cast_diagnostics)]
        report_casts();
        start.and(callback_result).and(fault)
    }
}

/// Prints the casts made while the app ran, busiest call sites first, when building with
/// `RUSTFLAGS=--cfg windows_cast_diagnostics`.
#[cfg(windows_cast_diagnostics)]
fn report_casts() {
    let mut counts = windows_core::imp::take_cast_counts();
    counts.sort_by_key(|count| std::cmp::Reverse(count.count));
    let short = |name: &'static str| {
        let end = name.find('<').unwrap_or(name.len());
        let start = name[..end].rfind("::").map_or(0, |index| index + 2);
        &name[start..]
    };
    eprintln!(
        "windows-reactor: {} casts at {} call sites",
        counts.iter().map(|count| count.count).sum::<u64>(),
        counts.len()
    );
    for count in counts.iter().take(40) {
        eprintln!(
            "{:>8}  {} -> {}  {}",
            count.count,
            short(count.source),
            short(count.target),
            count.location
        );
    }
}

fn window_host(
    state: &mut Option<ComponentWindowState>,
) -> Option<&mut ComponentHost<SharedWinUiAdapter>> {
    state.as_mut().map(|state| &mut state.host)
}

fn open_component_window(
    application: &Rc<RefCell<LiveApplicationState>>,
    services: &Rc<LiveWindowServices>,
    root: ComponentNode,
    policy: WindowPolicy,
) -> windows_core::Result<u64> {
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
        SharedWinUiAdapter::default(),
        context.services.clone(),
        ui_services,
        [root],
    );
    let drain_state = Rc::clone(&state);
    let changed_application = Rc::downgrade(application);
    let drain = services.dispatch.callback(&context, move || {
        let mut drain = drain_state
            .borrow_mut()
            .as_mut()
            .unwrap()
            .host
            .begin_drain(COMPONENT_DRAIN_BUDGET);
        drive_host(&drain_state, window_host, |host| {
            host.drain_step(&mut drain)
        });
        if let Some(state) = drain_state.borrow_mut().as_mut() {
            state.host.end_drain(drain);
        }
        if let Some(application) = changed_application.upgrade()
            && let Some(changed) = &application.borrow().changed
        {
            changed.wake();
        }
        Ok(())
    });
    let wake = drain.clone();
    host.set_waker(move || wake.wake());
    let native_drain = drain.clone();
    let wake = drain.clone();
    host.set_native_event_wakers(
        move || invoke_registered_app_callback(native_drain.id).unwrap(),
        move || wake.wake(),
    );
    let wake = drain.clone();
    host.set_continuation_waker(move || wake.wake());
    let realize_state = Rc::downgrade(&state);
    let realize_dispatch = Rc::clone(&services.dispatch);
    host.set_realizer(move || {
        let Some(state) = realize_state.upgrade() else {
            return;
        };
        // Layout forced while this window applies a batch realizes rows within that batch. Layout
        // forced while the host or adapter is borrowed, or while another driver is active, keeps
        // the queued realization for the next drain.
        let Ok(applying) = state.try_borrow().map(|state| {
            state.as_ref().map(|state| {
                (
                    state.host.in_flight(),
                    state.host.runtime().adapter().is_available(),
                )
            })
        }) else {
            return;
        };
        let realize = || {
            drive_host(&state, window_host, ComponentHost::next_realization_batch);
            if let Some(state) = state.borrow().as_ref() {
                state.host.rearm_wake();
            }
        };
        match applying {
            Some((true, true)) => realize(),
            Some((false, true)) => _ = realize_dispatch.run(realize),
            _ => {}
        }
    });
    let root = host.runtime().graph().root().unwrap();
    let mut window = match host
        .runtime()
        .adapter()
        .create_window_with_policy(root, &policy)
    {
        Ok(window) => window,
        Err(error) => {
            drain.cancel();
            return Err(error.into());
        }
    };
    let close_services = Rc::clone(services);
    let lifecycle = Rc::new(Cell::new(ComponentWindowLifecycle::Open));
    let closed_lifecycle = Rc::clone(&lifecycle);
    if let Err(error) = window.set_closed(move || {
        let mut lifecycle = closed_lifecycle.get();
        lifecycle.mark_closed();
        closed_lifecycle.set(lifecycle);
        close_services.push_critical(WindowWork::Closed(id));
    }) {
        drain.cancel();
        window.close().unwrap();
        return Err(error.into());
    }
    let native = window.clone();
    *state.borrow_mut() = Some(ComponentWindowState {
        host,
        window,
        lifecycle,
        color_scheme_generation: None,
        size_generation: None,
        placement_generation: None,
    });
    let publication = services.take_publication(id).unwrap();
    let initialized = apply_component_window_publication(
        state.borrow_mut().as_mut().unwrap(),
        services,
        id,
        publication,
        true,
    );
    if let Err(error) = initialized {
        drain.cancel();
        native.close().unwrap();
        return Err(error);
    }
    assert!(
        application
            .borrow_mut()
            .windows
            .insert(id, ComponentWindow { drain, state },)
            .is_none()
    );
    if let Err(error) = native.activate() {
        if let Some(window) = application.borrow_mut().windows.remove(&id) {
            window.dispose();
        }
        native.close().unwrap();
        return Err(error.into());
    }
    Ok(id)
}

fn apply_component_window_publication(
    state: &mut ComponentWindowState,
    services: &Rc<LiveWindowServices>,
    window: u64,
    publication: WindowPublication,
    initial: bool,
) -> windows_core::Result<()> {
    let size_generation = publication.on_size.map(|observer| observer.generation);
    let color_scheme_generation = publication
        .on_color_scheme
        .map(|observer| observer.generation);
    let placement_generation = publication.on_placement.map(|observer| observer.generation);
    if state.size_generation != size_generation {
        state.size_generation = size_generation;
        let observer = size_generation.map(|generation| {
            let services = Rc::clone(services);
            Rc::new(move |size| {
                _ = services.size(generation, size, window);
            }) as Rc<dyn Fn(WindowSize)>
        });
        state.window.set_size_observer(observer)?;
    }
    state.window.apply_publication(
        publication.title.as_deref(),
        publication.visuals.as_ref(),
        initial,
    )?;
    if state.color_scheme_generation != color_scheme_generation {
        state.color_scheme_generation = color_scheme_generation;
        let observer = color_scheme_generation.map(|generation| {
            let services = Rc::clone(services);
            Rc::new(move |scheme| {
                _ = services.color_scheme(generation, scheme, window);
            }) as Rc<dyn Fn(ColorScheme)>
        });
        state.window.set_color_scheme_observer(observer)?;
    }
    if state.placement_generation != placement_generation {
        state.placement_generation = placement_generation;
        let observer = placement_generation.map(|generation| {
            let services = Rc::clone(services);
            Rc::new(move |placement| {
                _ = services.placement(generation, placement, window);
            }) as Rc<dyn Fn(WindowPlacement)>
        });
        state.window.set_placement_observer(observer)?;
    }
    Ok(())
}

fn drain_window_requests(
    application: &Rc<RefCell<LiveApplicationState>>,
    services: &Rc<LiveWindowServices>,
) {
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
                        (state.lifecycle.get() == ComponentWindowLifecycle::Open)
                            .then(|| state.window.clone())
                    })
                };
                if let Some(window) = window {
                    window.activate().unwrap();
                }
            }
            WindowWork::Close(id) => {
                let window = {
                    let application = application.borrow();
                    application.windows.get(&id).and_then(|window| {
                        let mut state = window.state.borrow_mut();
                        let state = state.as_mut()?;
                        let mut lifecycle = state.lifecycle.get();
                        let closing = lifecycle.begin_close();
                        state.lifecycle.set(lifecycle);
                        closing.then(|| state.window.clone())
                    })
                };
                if let Some(native) = window {
                    native.close().unwrap();
                }
            }
            WindowWork::Closed(id) => {
                close_component_window(application, services, id);
            }
            WindowWork::Input { root, window } => {
                let application = application.borrow();
                if let Some(window) = application.windows.get(&window) {
                    let mut state = window.state.borrow_mut();
                    if let Some(state) = state.as_mut()
                        && state.lifecycle.get() == ComponentWindowLifecycle::Open
                    {
                        state.host.update_root(root);
                    }
                }
            }
            WindowWork::Operation { window, operation } => {
                let native = {
                    let application = application.borrow();
                    application.windows.get(&window).and_then(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref()?;
                        (state.lifecycle.get() == ComponentWindowLifecycle::Open)
                            .then(|| state.window.clone())
                    })
                };
                if let Some(native) = native {
                    operation(native.raw_handle().unwrap());
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
                if state.lifecycle.get() != ComponentWindowLifecycle::Open {
                    continue;
                }
                apply_component_window_publication(state, services, window, publication, false)
                    .unwrap();
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
                        (state.lifecycle.get() == ComponentWindowLifecycle::Open)
                            .then(|| state.host.window_color_scheme_callback(generation))
                            .flatten()
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
                        (state.lifecycle.get() == ComponentWindowLifecycle::Open)
                            .then(|| state.host.window_size_callback(generation))
                            .flatten()
                    })
                };
                if let Some(callback) = callback {
                    callback.call(size);
                }
            }
            WindowWork::Placement {
                generation,
                placement,
                window,
            } => {
                let callback = {
                    let application = application.borrow();
                    application.windows.get(&window).and_then(|window| {
                        let state = window.state.borrow();
                        let state = state.as_ref()?;
                        (state.lifecycle.get() == ComponentWindowLifecycle::Open)
                            .then(|| state.host.window_placement_callback(generation))
                            .flatten()
                    })
                };
                if let Some(callback) = callback {
                    callback.call(placement);
                }
            }
            WindowWork::Open { policy, root } => {
                open_component_window(application, services, root, policy).unwrap();
            }
        }
    }
    if services.has_pending() {
        services.rearm();
    } else {
        exit_if_no_windows(application);
    }
    if let Some(changed) = &application.borrow().changed {
        changed.wake();
    }
}

fn close_component_window(
    application: &Rc<RefCell<LiveApplicationState>>,
    services: &Rc<LiveWindowServices>,
    id: u64,
) {
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
        proxy.queue_exit();
    }
}

fn exit_if_no_windows(application: &Rc<RefCell<LiveApplicationState>>) {
    let application = application.borrow();
    if application.exit_when_empty && application.windows.is_empty() {
        application.application.queue_exit();
    }
}

fn report_error(error: windows_core::Error) {
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
    let result = unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_APARTMENTTHREADED as u32) };
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
        _ => Err(HRESULT::from(windows_core::WIN32_ERROR(result as u32)).into()),
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
                HRESULT(0x8007007e_u32 as i32),
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
#[path = "../../tests/native/app.rs"]
mod tests;

#[cfg(feature = "test")]
#[path = "../../test_support/native/app.rs"]
mod test_support;
#[cfg(feature = "test")]
pub use test_support::*;
