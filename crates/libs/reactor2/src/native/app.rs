use super::app_shim::{create_application, install_xaml_controls_resources};
use super::bindings::*;
use super::bootstrap;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use windows_core::Interface;

thread_local! {
    static APP_CALLBACKS: RefCell<HashMap<u64, Rc<dyn Fn() -> windows_core::Result<()>>>> =
        RefCell::new(HashMap::new());
    static APP_FAULT: RefCell<Option<windows_core::Error>> = const { RefCell::new(None) };
    static APP_LIFETIME: RefCell<Option<AppLifetime>> = const { RefCell::new(None) };
    static APP_TIMERS: RefCell<HashMap<u64, DispatcherTimerState>> = RefCell::new(HashMap::new());
}

static NEXT_APP_CALLBACK: AtomicU64 = AtomicU64::new(1);
static NEXT_APP_TIMER: AtomicU64 = AtomicU64::new(1);

struct AppLifetime {
    _resource: Box<dyn Any>,
    _application: Application,
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

    pub fn component_services(&self) -> Arc<dyn crate::ComponentServices> {
        self.services.clone()
    }

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

#[derive(Clone)]
pub struct AppCallback {
    proxy: AppProxy,
    id: u64,
}

impl AppCallback {
    pub fn invoke(&self) -> windows_core::Result<()> {
        let id = self.id;
        self.proxy
            .dispatch(move |_| APP_CALLBACKS.with(|callbacks| callbacks.borrow()[&id]()))
    }
}

pub struct App;

impl App {
    pub fn run_with<T>(
        startup: impl FnOnce(&AppContext) -> windows_core::Result<T> + 'static,
    ) -> windows_core::Result<()>
    where
        T: 'static,
    {
        if !is_packaged_process()? {
            bootstrap::bootstrap()?;
        }
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
        APP_CALLBACKS.with(|callbacks| callbacks.borrow_mut().clear());
        APP_TIMERS.with(|timers| timers.borrow_mut().clear());
        APP_LIFETIME.with(|lifetime| drop(lifetime.borrow_mut().take()));
        let fault = APP_FAULT
            .with(|fault| fault.borrow_mut().take())
            .map_or(Ok(()), Err);
        start.and(callback_result).and(fault)
    }
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
