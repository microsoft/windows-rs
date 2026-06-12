use super::app_shim::*;
use super::bindings::*;
use super::winui::*;
use super::*;
use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};

const E_FAIL: HRESULT = HRESULT(0x80004005u32 as i32);

thread_local! {

    static HOST_SLOT: RefCell<Option<ReactorHost>> = const { RefCell::new(None) };

    static APP_SLOT: RefCell<Option<Application>> =
        const { RefCell::new(None) };
}

/// Run `f` with the [`ReactorHost`] for the current thread, if any.
pub fn with_active_host<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&ReactorHost) -> R,
{
    HOST_SLOT.with(|slot| slot.borrow().as_ref().map(f))
}

/// Top-level reactor application; hosts a single root [`Component`].
pub struct App {
    title: Option<String>,
    inner_size: Option<window::Size>,
    inner_constraints: InnerConstraints,
    eager_templated_realization: bool,
    presenter: PresenterKind,
    backdrop: Option<Backdrop>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            title: None,
            inner_size: None,
            inner_constraints: InnerConstraints::default(),
            eager_templated_realization: false,
            presenter: PresenterKind::Default,
            backdrop: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inner_size(mut self, width: f64, height: f64) -> Self {
        self.inner_size = Some(window::Size { width, height });
        self
    }

    pub fn inner_constraints(mut self, constraints: InnerConstraints) -> Self {
        self.inner_constraints = constraints;
        self
    }

    pub fn eager_templated_realization(mut self, on: bool) -> Self {
        self.eager_templated_realization = on;
        self
    }

    /// Set the top-level window presenter (defaults to overlapped window).
    pub fn presenter(mut self, kind: PresenterKind) -> Self {
        self.presenter = kind;
        self
    }

    /// Shortcut for `presenter(PresenterKind::FullScreen)`.
    pub fn fullscreen(self, on: bool) -> Self {
        self.presenter(if on {
            PresenterKind::FullScreen
        } else {
            PresenterKind::Default
        })
    }

    /// Set the system backdrop for the window created by [`App::run`].
    pub fn backdrop(mut self, backdrop: Backdrop) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    /// Run with custom WinUI setup; the caller manages windows and content.
    pub fn run_custom<F>(self, setup: F) -> Result<()>
    where
        F: FnOnce(&Application) -> Result<()> + Send + 'static,
    {
        init_app_platform()?;
        let setup = Mutex::new(Some(setup));
        let result_slot: Arc<Mutex<Result<()>>> = Arc::new(Mutex::new(Ok(())));
        let result_slot_cb = Arc::clone(&result_slot);
        let start_result =
            Application::Start(&ApplicationInitializationCallback::new(move |_params| {
                let inner = || -> Result<()> {
                    let setup = setup.lock().unwrap().take().unwrap();

                    let on_launched: Box<dyn FnOnce() -> Result<()>> = Box::new(move || {
                        let app = APP_SLOT.with(|slot| slot.borrow().clone()).unwrap();
                        install_xaml_controls_resources(&app)?;
                        run_callback("setup", || setup(&app))
                    });

                    let app = create_reactor_application(on_launched)?;
                    APP_SLOT.with(|slot| {
                        *slot.borrow_mut() = Some(app);
                    });
                    Ok(())
                };
                if let Err(err) = inner() {
                    *result_slot_cb.lock().unwrap() = Err(err);
                }
            }));

        let result =
            start_result.and_then(|_| std::mem::replace(&mut *result_slot.lock().unwrap(), Ok(())));
        report_app_start_result(result)
    }

    /// Run the app, building the root component on the UI thread.
    pub fn run<F, C>(self, root_factory: F) -> Result<()>
    where
        F: FnOnce() -> C + Send + 'static,
        C: Component + 'static,
    {
        init_app_platform()?;
        let title = self.title.unwrap_or_default();
        let eager = self.eager_templated_realization;
        let size = self.inner_size;
        let constraints = self.inner_constraints;
        let presenter = self.presenter;
        let backdrop = self.backdrop;
        let factory = Mutex::new(Some(root_factory));
        let result_slot: Arc<Mutex<Result<()>>> = Arc::new(Mutex::new(Ok(())));
        let result_slot_cb = Arc::clone(&result_slot);
        let start_result =
            Application::Start(&ApplicationInitializationCallback::new(move |_params| {
                let inner = || -> Result<()> {
                    let factory = factory.lock().unwrap().take().unwrap();

                    let title = title.clone();
                    let on_launched: Box<dyn FnOnce() -> Result<()>> = Box::new(move || {
                        let app = APP_SLOT.with(|slot| slot.borrow().clone()).unwrap();
                        install_xaml_controls_resources(&app)?;

                        run_callback("OnLaunched", move || {
                            let root: Box<dyn Component> = Box::new(factory());
                            let host = ReactorHost::new_with_window_options(
                                &title,
                                size,
                                constraints,
                                root,
                                |recon| {
                                    recon.eager_templated_realization = eager;
                                },
                            )?;
                            host.set_presenter(presenter);
                            if let Some(bd) = backdrop {
                                host.set_backdrop(bd);
                            }
                            host.activate()?;
                            // Exit the process on window close. Application.Exit()
                            // fail-fasts due to live COM refs, so terminate directly.
                            let _ = host
                                .window()
                                .add_Closed(|_, _| {
                                    std::process::exit(0);
                                })?
                                .into_token();
                            HOST_SLOT.with(|slot| {
                                *slot.borrow_mut() = Some(host);
                            });
                            Ok(())
                        })
                    });

                    let app = create_reactor_application(on_launched)?;
                    APP_SLOT.with(|slot| {
                        *slot.borrow_mut() = Some(app);
                    });
                    Ok(())
                };
                if let Err(err) = inner() {
                    *result_slot_cb.lock().unwrap() = Err(err);
                }
            }));

        let result =
            start_result.and_then(|_| std::mem::replace(&mut *result_slot.lock().unwrap(), Ok(())));
        report_app_start_result(result)
    }

    /// Convenience entry point that accepts a render function directly,
    /// avoiding the empty-struct `Component` pattern.
    ///
    /// The render function should return `Element`. Use `.into()` to convert
    /// widget builders into `Element`:
    ///
    /// ```ignore
    /// fn app(cx: &mut RenderCx) -> Element {
    ///     let (count, set_count) = cx.use_state(0);
    ///     button(format!("Clicks: {count}"))
    ///         .on_click(move || set_count.call(count + 1))
    ///         .into()
    /// }
    ///
    /// fn main() -> Result<()> {
    ///     App::new().render(app)
    /// }
    /// ```
    pub fn render<F>(self, f: F) -> Result<()>
    where
        F: Fn(&mut RenderCx) -> Element + Send + 'static,
    {
        self.run(move || RenderFn(f))
    }
}

/// Internal wrapper: adapts `Fn(&mut RenderCx) -> Element` into `Component<()>`
/// so it can be used with the existing host machinery.
struct RenderFn<F>(F);

impl<F> Component for RenderFn<F>
where
    F: Fn(&mut RenderCx) -> Element + 'static,
{
    fn render(&self, _props: &(), cx: &mut RenderCx) -> Element {
        (self.0)(cx)
    }
}

fn run_callback<F>(label: &'static str, f: F) -> Result<()>
where
    F: FnOnce() -> Result<()>,
{
    match std::panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => {
            diagnostics::emit(&format!(
                "windows_reactor: {label} callback returned error: {err:?}"
            ));
            Err(err)
        }
        Err(payload) => {
            let msg = diagnostics::format_panic_payload(&payload);
            diagnostics::emit(&format!(
                "windows_reactor: {label} callback panicked: {msg}"
            ));
            Err(Error::new(E_FAIL, format!("{label} panicked: {msg}")))
        }
    }
}

fn report_app_start_result(result: Result<()>) -> Result<()> {
    if let Err(err) = &result {
        diagnostics::emit(&format!(
            "windows_reactor: Application::Start failed: {err:?}"
        ));
    }
    result
}

fn init_app_platform() -> Result<()> {
    diagnostics::install();

    // SAFETY: FFI call into user32; returns HRESULT and has no aliasing requirements.
    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2).ok()? };

    // SAFETY: FFI call into ole32; null reserved arg is documented as required.
    let coinit_hr = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
    if coinit_hr == RPC_E_CHANGED_MODE {
        return Err(Error::new(
            RPC_E_CHANGED_MODE,
            "windows_reactor::App: the calling thread is already initialized for COM in an \
             apartment incompatible with STA (likely MTA). WinUI 3 requires STA. \
             Call App::run / App::run_custom from `main` (or another thread you fully \
             own) and do not call CoInitializeEx(COINIT_MULTITHREADED) before this call.",
        ));
    }
    coinit_hr.ok()
}
