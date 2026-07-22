use std::cell::RefCell;
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};

use super::*;

use super::app_shim::*;
use super::bindings::*;

/// A per-thread registry of open reactor windows keyed by a monotonic id.
///
/// Generic over the payload `T` so the key allocation and last-window
/// bookkeeping can be unit-tested without a live WinUI window; the app hosts it
/// as `WindowRegistry<ReactorHost>`.
struct WindowRegistry<T> {
    next_key: u64,
    entries: Vec<(u64, T)>,
}

impl<T> WindowRegistry<T> {
    const fn new() -> Self {
        Self {
            next_key: 0,
            entries: Vec::new(),
        }
    }

    /// Register `value`, returning its unique, never-reused key.
    fn insert(&mut self, value: T) -> u64 {
        let key = self.next_key;
        self.next_key += 1;
        self.entries.push((key, value));
        key
    }

    fn get(&self, key: u64) -> Option<&T> {
        self.entries
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, value)| value)
    }

    /// The first (primary) registered payload, if any.
    fn first(&self) -> Option<&T> {
        self.entries.first().map(|(_, value)| value)
    }

    /// Remove the entry with `key`. Returns the removed payload (if the key was
    /// present) and whether the registry is now empty. The caller is expected to
    /// drop the payload after releasing any registry borrow.
    fn remove(&mut self, key: u64) -> (Option<T>, bool) {
        let removed = self
            .entries
            .iter()
            .position(|(k, _)| *k == key)
            .map(|pos| self.entries.remove(pos).1);
        (removed, self.entries.is_empty())
    }
}

thread_local! {
    /// All reactor windows open on this UI thread. The registry owns each
    /// [`ReactorHost`]; a window is removed when it closes.
    static WINDOW_REGISTRY: RefCell<WindowRegistry<ReactorHost>> =
        const { RefCell::new(WindowRegistry::new()) };
    static APP_SLOT: RefCell<Option<Application>> = const { RefCell::new(None) };
}

/// Run `f` with the active (primary) [`ReactorHost`] for the current thread, if
/// any. The active host is the first window opened on the thread — theme and
/// backdrop free functions target it (app-global theme, v1).
pub fn with_active_host<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&ReactorHost) -> R,
{
    WINDOW_REGISTRY.with(|reg| reg.borrow().first().map(f))
}

/// A handle to an open reactor window. The window registry owns the host, so
/// dropping the handle does not close the window; call [`WindowHandle::close`]
/// to close it programmatically.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowHandle {
    key: u64,
}

impl WindowHandle {
    /// Close this window. The window's `Closed` event removes it from the
    /// registry; if it was the last open window the process exits (matching
    /// [`App::run`] semantics).
    pub fn close(&self) {
        let window = WINDOW_REGISTRY
            .with(|reg| reg.borrow().get(self.key).map(|host| host.window().clone()));
        if let Some(window) = window {
            let _ = window.Close();
        }
    }
}

/// Register `host` in the per-thread window registry and wire its `Closed`
/// event so it is removed on close (and the process exits when the last window
/// closes). Returns a [`WindowHandle`] for programmatic control.
pub(crate) fn register_host(host: ReactorHost) -> Result<WindowHandle> {
    let window = host.window().clone();
    let key = WINDOW_REGISTRY.with(|reg| reg.borrow_mut().insert(host));
    // The revoker is intentionally leaked (`into_token`): the registration must
    // outlive this call and is torn down when the window is destroyed. If wiring
    // fails, unwind the registration so a broken window can't linger and block
    // the last-window-closes exit.
    match window.Closed(move |_, _| on_window_closed(key)) {
        Ok(revoker) => {
            revoker.into_token();
            Ok(WindowHandle { key })
        }
        Err(err) => {
            let (removed, empty) = WINDOW_REGISTRY.with(|reg| reg.borrow_mut().remove(key));
            drop(removed);
            let _ = empty;
            Err(err)
        }
    }
}

/// Handle a window's `Closed` event: drop its host and exit the process once
/// the last window on the thread has closed. `Application.Exit()` fail-fasts on
/// live COM refs, so terminate directly.
fn on_window_closed(key: u64) {
    // Remove under the borrow, then drop the host outside it so teardown can't
    // re-enter a live registry borrow.
    let (removed, empty) = WINDOW_REGISTRY.with(|reg| reg.borrow_mut().remove(key));
    drop(removed);
    if empty {
        std::process::exit(0);
    }
}

/// Top-level reactor application; hosts a single root [`Component`].
pub struct App {
    title: Option<String>,
    inner_size: Option<WindowSize>,
    inner_constraints: InnerConstraints,
    presenter: PresenterKind,
    backdrop: Option<Backdrop>,
    icon: Option<String>,
    on_fault: Option<Box<dyn Fn(&Fault) + Send>>,
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
            presenter: PresenterKind::Default,
            backdrop: None,
            icon: None,
            on_fault: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inner_size(mut self, width: f64, height: f64) -> Self {
        self.inner_size = Some(WindowSize { width, height });
        self
    }

    pub fn inner_constraints(mut self, constraints: InnerConstraints) -> Self {
        self.inner_constraints = constraints;
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

    /// Set the window icon from a path to an `.ico` file. WinUI 3 does not adopt
    /// the executable's embedded icon for the window, so set it explicitly to
    /// control the title-bar and taskbar icon. [`App::run`] returns an error if
    /// the file does not exist.
    pub fn icon(mut self, path: impl Into<String>) -> Self {
        self.icon = Some(path.into());
        self
    }

    /// Set a handler invoked when a panic is caught at a reactor callback
    /// boundary — an event handler, a timer tick, or the render pass. Without a
    /// handler such panics are logged and execution continues (a panic that
    /// reaches WinUI's `extern "system"` delegate boundary would otherwise abort
    /// the process). The handler runs on the UI thread and does not replace
    /// [`error_boundary`](crate::error_boundary), which still recovers individual
    /// render subtrees first.
    pub fn on_fault<F>(mut self, f: F) -> Self
    where
        F: Fn(&Fault) + Send + 'static,
    {
        self.on_fault = Some(Box::new(f));
        self
    }

    /// Run with custom WinUI setup; the caller manages windows and content.
    pub fn run_custom<F>(self, setup: F) -> Result<()>
    where
        F: FnOnce(&Application) -> Result<()> + Send + 'static,
    {
        init_app_platform()?;
        let setup = Mutex::new(Some(setup));
        let on_fault = Mutex::new(self.on_fault);
        let result_slot: Arc<Mutex<Result<()>>> = Arc::new(Mutex::new(Ok(())));
        let result_slot_cb = Arc::clone(&result_slot);
        let start_result =
            Application::Start(&ApplicationInitializationCallback::new(move |_params| {
                let inner = || -> Result<()> {
                    let setup = setup.lock().unwrap().take().unwrap();
                    let on_fault = on_fault.lock().unwrap().take();

                    let on_launched: Box<dyn FnOnce() -> Result<()>> = Box::new(move || {
                        if let Some(on_fault) = on_fault {
                            fault::set_handler(on_fault);
                        }
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
        let size = self.inner_size;
        let constraints = self.inner_constraints;
        let presenter = self.presenter;
        let backdrop = self.backdrop;
        let icon = self.icon;
        let on_fault = Mutex::new(self.on_fault);
        if let Some(icon) = &icon
            && !std::path::Path::new(icon).is_file()
        {
            return Err(Error::new(
                E_FAIL,
                format!("windows_reactor::App::icon: icon file not found: {icon}"),
            ));
        }
        let factory = Mutex::new(Some(root_factory));
        let result_slot: Arc<Mutex<Result<()>>> = Arc::new(Mutex::new(Ok(())));
        let result_slot_cb = Arc::clone(&result_slot);
        let start_result =
            Application::Start(&ApplicationInitializationCallback::new(move |_params| {
                let inner = || -> Result<()> {
                    let factory = factory.lock().unwrap().take().unwrap();
                    let on_fault = on_fault.lock().unwrap().take();

                    let title = title.clone();
                    let icon = icon.clone();
                    let on_launched: Box<dyn FnOnce() -> Result<()>> = Box::new(move || {
                        if let Some(on_fault) = on_fault {
                            fault::set_handler(on_fault);
                        }
                        let app = APP_SLOT.with(|slot| slot.borrow().clone()).unwrap();
                        install_xaml_controls_resources(&app)?;

                        run_callback("OnLaunched", move || {
                            let root: Box<dyn Component> = Box::new(factory());
                            let host = ReactorHost::new_with_window_options(
                                &title,
                                size,
                                constraints,
                                root,
                                |_recon| {},
                            )?;
                            host.set_presenter(presenter);
                            if let Some(bd) = backdrop {
                                host.set_backdrop(bd);
                            }
                            if let Some(icon) = icon {
                                host.set_icon(icon);
                            }
                            host.activate()?;
                            register_host(host)?;
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

/// Builder for a secondary reactor window, opened at runtime on the current UI
/// thread (for example from an event handler). Mirrors [`App`]'s window options
/// but, unlike [`App::run`], hosts an additional window inside an already
/// running reactor application rather than bootstrapping the process.
///
/// The process exits when the **last** open window closes, so a secondary
/// window keeps the app alive until it (and every other window) is closed.
///
/// ```ignore
/// button("Open window").on_click(|| {
///     let _ = ReactorWindow::new()
///         .title("Details")
///         .inner_size(480.0, 320.0)
///         .render(details_page);
/// });
/// ```
pub struct ReactorWindow {
    title: Option<String>,
    inner_size: Option<WindowSize>,
    inner_constraints: InnerConstraints,
    presenter: PresenterKind,
    backdrop: Option<Backdrop>,
    icon: Option<String>,
}

impl Default for ReactorWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactorWindow {
    pub fn new() -> Self {
        Self {
            title: None,
            inner_size: None,
            inner_constraints: InnerConstraints::default(),
            presenter: PresenterKind::Default,
            backdrop: None,
            icon: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inner_size(mut self, width: f64, height: f64) -> Self {
        self.inner_size = Some(WindowSize { width, height });
        self
    }

    pub fn inner_constraints(mut self, constraints: InnerConstraints) -> Self {
        self.inner_constraints = constraints;
        self
    }

    /// Set the window presenter (defaults to overlapped window).
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

    /// Set the system backdrop for the window.
    pub fn backdrop(mut self, backdrop: Backdrop) -> Self {
        self.backdrop = Some(backdrop);
        self
    }

    /// Set the window icon from a path to an `.ico` file. [`ReactorWindow::open`]
    /// returns an error if the file does not exist.
    pub fn icon(mut self, path: impl Into<String>) -> Self {
        self.icon = Some(path.into());
        self
    }

    /// Open the window, building its root [`Component`] on the current UI thread.
    /// Returns a [`WindowHandle`] that can close the window programmatically.
    pub fn open<F, C>(self, root_factory: F) -> Result<WindowHandle>
    where
        F: FnOnce() -> C,
        C: Component + 'static,
    {
        if let Some(icon) = &self.icon
            && !std::path::Path::new(icon).is_file()
        {
            return Err(Error::new(
                E_FAIL,
                format!("windows_reactor::ReactorWindow::icon: icon file not found: {icon}"),
            ));
        }
        let root: Box<dyn Component> = Box::new(root_factory());
        let host = ReactorHost::new_with_window_options(
            self.title.unwrap_or_default(),
            self.inner_size,
            self.inner_constraints,
            root,
            |_recon| {},
        )?;
        host.set_presenter(self.presenter);
        if let Some(bd) = self.backdrop {
            host.set_backdrop(bd);
        }
        if let Some(icon) = self.icon {
            host.set_icon(icon);
        }
        host.activate()?;
        register_host(host)
    }

    /// Convenience form of [`ReactorWindow::open`] that accepts a render function
    /// directly, avoiding the empty-struct `Component` pattern (mirrors
    /// [`App::render`]).
    pub fn render<F>(self, f: F) -> Result<WindowHandle>
    where
        F: Fn(&mut RenderCx) -> Element + 'static,
    {
        self.open(move || RenderFn(f))
    }
}

/// Opener returned by [`RenderCx::use_open_window`]. A lightweight, copyable
/// handle that launches a secondary window with default options from within a
/// component's event handlers. Use [`ReactorWindow`] directly to configure the
/// title, size, backdrop, and so on.
#[derive(Clone, Copy)]
pub struct OpenWindow;

impl OpenWindow {
    /// Open a default-configured secondary window hosting `root`.
    pub fn open<F, C>(self, root_factory: F) -> Result<WindowHandle>
    where
        F: FnOnce() -> C,
        C: Component + 'static,
    {
        ReactorWindow::new().open(root_factory)
    }

    /// Open a default-configured secondary window from a render function.
    pub fn render<F>(self, f: F) -> Result<WindowHandle>
    where
        F: Fn(&mut RenderCx) -> Element + 'static,
    {
        ReactorWindow::new().render(f)
    }
}

impl RenderCx {
    /// Returns an [`OpenWindow`] opener for launching secondary windows from a
    /// component's event handlers. The opener is copyable and can be moved into
    /// multiple closures.
    pub fn use_open_window(&self) -> OpenWindow {
        OpenWindow
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

#[cfg(test)]
mod tests {
    use super::WindowRegistry;

    #[test]
    fn insert_assigns_unique_increasing_keys() {
        let mut reg = WindowRegistry::<&str>::new();
        let a = reg.insert("a");
        let b = reg.insert("b");
        let c = reg.insert("c");
        assert_eq!((a, b, c), (0, 1, 2));
    }

    #[test]
    fn get_resolves_by_key() {
        let mut reg = WindowRegistry::<i32>::new();
        let k0 = reg.insert(10);
        let k1 = reg.insert(20);
        assert_eq!(reg.get(k0), Some(&10));
        assert_eq!(reg.get(k1), Some(&20));
        assert_eq!(reg.get(999), None);
    }

    #[test]
    fn first_returns_the_primary_window() {
        let mut reg = WindowRegistry::<&str>::new();
        assert_eq!(reg.first(), None);
        reg.insert("primary");
        reg.insert("secondary");
        assert_eq!(reg.first(), Some(&"primary"));
    }

    #[test]
    fn remove_reports_emptiness_only_when_last_window_closes() {
        let mut reg = WindowRegistry::<i32>::new();
        let k0 = reg.insert(1);
        let k1 = reg.insert(2);

        // Closing a non-last window: value returned, registry not yet empty.
        assert_eq!(reg.remove(k0), (Some(1), false));
        // Closing the last window: value returned, registry now empty (exit).
        assert_eq!(reg.remove(k1), (Some(2), true));
    }

    #[test]
    fn removing_the_primary_promotes_the_next_window() {
        let mut reg = WindowRegistry::<&str>::new();
        let primary = reg.insert("primary");
        reg.insert("secondary");
        assert_eq!(reg.remove(primary), (Some("primary"), false));
        // The surviving window becomes the active/primary target.
        assert_eq!(reg.first(), Some(&"secondary"));
    }

    #[test]
    fn removing_a_missing_key_is_a_no_op_and_reports_current_emptiness() {
        let mut reg = WindowRegistry::<i32>::new();
        let k0 = reg.insert(1);
        // Unknown key: nothing removed, still non-empty.
        assert_eq!(reg.remove(42), (None, false));
        assert_eq!(reg.get(k0), Some(&1));
        // Remove the real one, then a stale double-remove on an empty registry.
        assert_eq!(reg.remove(k0), (Some(1), true));
        assert_eq!(reg.remove(k0), (None, true));
    }
}
