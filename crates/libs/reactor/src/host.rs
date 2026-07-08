use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::*;
use bindings::*;

thread_local! {
    static ROOT_FRAMEWORK_ELEMENT: RefCell<Option<FrameworkElement>> = const { RefCell::new(None) };
    static ROOT_WINDOW: RefCell<Option<Window>> = const { RefCell::new(None) };
    /// Queued theme; applied once `ROOT_FRAMEWORK_ELEMENT` is available.
    static PENDING_THEME: Cell<Option<ElementTheme>> = const { Cell::new(None) };
    /// TitleBar height option requested before `ROOT_WINDOW` was set. Applied once
    /// the window becomes available in `post_render`.
    static PENDING_TALL: Cell<Option<bool>> = const { Cell::new(None) };
}

/// Requested application theme, matching `Microsoft.UI.Xaml.ElementTheme`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestedTheme {
    /// Use the system default (inherits from OS setting).
    Default,
    /// Force light theme.
    Light,
    /// Force dark theme.
    Dark,
}

/// Set the application theme. Queued if the root element isn't attached yet.
pub fn set_requested_theme(theme: RequestedTheme) {
    let element_theme = match theme {
        RequestedTheme::Light => ElementTheme::Light,
        RequestedTheme::Dark => ElementTheme::Dark,
        _ => ElementTheme::Default,
    };

    ROOT_FRAMEWORK_ELEMENT.with(|cell| {
        if let Some(ife) = cell.borrow().as_ref() {
            let _ = ife.SetRequestedTheme(element_theme);
            update_titlebar_theme();
        } else {
            PENDING_THEME.with(|p| p.set(Some(element_theme)));
        }
    });
}

fn update_titlebar_theme() {
    ROOT_FRAMEWORK_ELEMENT.with(|cell| {
        if let Some(ife) = cell.borrow().as_ref()
            && let Ok(theme) = ife.ActualTheme()
        {
            let titlebar_theme = match theme {
                ElementTheme::Dark => TitleBarTheme::Dark,
                ElementTheme::Light => TitleBarTheme::Light,
                _ => TitleBarTheme::UseDefaultAppMode,
            };

            let _ = ROOT_WINDOW.with(|wcell| -> Option<()> {
                let window = wcell.borrow();
                let window_2 = window.as_ref()?.cast::<IWindow2>().ok()?;
                let app_window = window_2.AppWindow().ok()?;
                let titlebar = app_window
                    .TitleBar()
                    .ok()?
                    .cast::<IAppWindowTitleBar3>()
                    .ok()?;
                titlebar.SetPreferredTheme(titlebar_theme).ok()
            });
        }
    });
}

pub fn set_titlebar_height(tall: bool) {
    let applied = ROOT_WINDOW.with(|wcell| -> Option<()> {
        let window = wcell.borrow();
        let window_2 = window.as_ref()?.cast::<IWindow2>().ok()?;
        let app_window = window_2.AppWindow().ok()?;
        let titlebar = app_window
            .TitleBar()
            .ok()?
            .cast::<IAppWindowTitleBar2>()
            .ok()?;
        let option = if tall {
            TitleBarHeightOption::Tall
        } else {
            TitleBarHeightOption::Standard
        };
        titlebar.SetPreferredHeightOption(option).ok()
    });
    if applied.is_none() {
        PENDING_TALL.with(|p| p.set(Some(tall)));
    }
}

/// Apply or remove the window backdrop material at runtime.
pub fn set_backdrop(backdrop: Option<Backdrop>) {
    ROOT_WINDOW.with(|cell| {
        if let Some(window) = cell.borrow().as_ref() {
            if let Some(b) = backdrop {
                let _ = b.apply_to(window);
            } else {
                if let Ok(w2) = window.cast::<IWindow2>() {
                    let _ = w2.SetSystemBackdrop(None);
                }
            }
        }
    });
}

/// Top-level window presenter (`AppWindowPresenterKind`).
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum PresenterKind {
    /// Platform default (overlapping window with a title bar).
    #[default]
    Default,
    /// Frameless, fills the active monitor.
    FullScreen,
    /// Floating "picture-in-picture" style overlay.
    CompactOverlay,
}

impl PresenterKind {
    fn to_native(self) -> Option<AppWindowPresenterKind> {
        match self {
            Self::Default => None,
            Self::FullScreen => Some(AppWindowPresenterKind::FullScreen),
            Self::CompactOverlay => Some(AppWindowPresenterKind::CompactOverlay),
        }
    }
}

/// Window backdrop material applied behind the app content.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Backdrop {
    Mica,
    MicaAlt,
    Acrylic,
}

impl Backdrop {
    /// Apply this backdrop material to an existing WinUI window.
    ///
    /// This is useful for manual window setup in [`App::run_custom`]
    /// or other custom hosts that do not go through [`ReactorHost`].
    pub fn apply_to(self, window: &impl Interface) -> Result<()> {
        let system_backdrop: SystemBackdrop = match self {
            Self::Mica => MicaBackdrop::new()?.cast()?,
            Self::MicaAlt => {
                let mica = MicaBackdrop::new()?;
                mica.SetKind(MicaKind::BaseAlt)?;
                mica.cast()?
            }
            Self::Acrylic => DesktopAcrylicBackdrop::new()?.cast()?,
        };
        window
            .cast::<IWindow2>()?
            .SetSystemBackdrop(&system_backdrop)
    }
}

/// WinUI-bound [`RenderHost`] hosting a single root [`Component`] inside
/// a `Microsoft.UI.Xaml.Window`.
pub struct ReactorHost {
    render_host: RenderHost<WinUIBackend, WinUIDispatcher>,
    window: Window,
    presenter: Cell<PresenterKind>,
    backdrop: Cell<Option<Backdrop>>,
    icon: RefCell<Option<String>>,
}

impl ReactorHost {
    pub fn new(title: impl AsRef<str>, root: Box<dyn Component>) -> Result<Self> {
        Self::new_with(title, root, |_| {})
    }

    fn new_with<F>(title: impl AsRef<str>, root: Box<dyn Component>, configure: F) -> Result<Self>
    where
        F: FnOnce(&mut Reconciler<WinUIBackend>),
    {
        Self::new_with_window_options(title, None, InnerConstraints::default(), root, configure)
    }

    pub fn new_with_window_options<F>(
        title: impl AsRef<str>,
        size: Option<WindowSize>,
        constraints: InnerConstraints,
        root: Box<dyn Component>,
        configure: F,
    ) -> Result<Self>
    where
        F: FnOnce(&mut Reconciler<WinUIBackend>),
    {
        let (window, resolved_dip_size, initial_dpi) = create_window(title, size, constraints)?;
        let dispatcher = WinUIDispatcher::for_current_thread()?;
        let marshaller = dispatcher.marshaller();
        let render_host = RenderHost::new(WinUIBackend::new(), root, dispatcher);
        render_host.set_marshaller(Some(marshaller));
        render_host.set_inner_size(resolved_dip_size);
        render_host.set_dpi(initial_dpi);
        render_host.with_reconciler_mut(configure);

        let attach_for_post_render = AttachState {
            window: window.clone(),
            render_host: render_host.clone_inner(),
        };
        let last_attached: Rc<Cell<Option<ControlId>>> = Rc::new(Cell::new(None));
        let last_attached_for_hook = Rc::clone(&last_attached);
        let subscribed = Rc::new(Cell::new(false));
        render_host.set_post_render(move |new_id| {
            if last_attached_for_hook.get() == new_id {
                return;
            }
            let state = &attach_for_post_render;
            match new_id {
                Some(rid) => {
                    if let Some(ui) = state.render_host.with_backend(|b| b.get_ui_element(rid)) {
                        let ui_element: UIElement = ui.cast().unwrap();
                        let _ = state.window.SetContent(&ui_element);
                        last_attached_for_hook.set(Some(rid));

                        if !subscribed.get() {
                            subscribed.set(true);
                            ROOT_WINDOW
                                .with(|cell| *cell.borrow_mut() = Some(state.window.clone()));
                            if let Ok(fe) = ui_element.cast::<FrameworkElement>() {
                                subscribe_actual_theme_changed(
                                    &fe,
                                    state.render_host.clone_inner(),
                                );
                                subscribe_size_and_dpi(
                                    &fe,
                                    state.render_host.clone_inner(),
                                    state.window.clone(),
                                    constraints,
                                );
                                ROOT_FRAMEWORK_ELEMENT
                                    .with(|cell| *cell.borrow_mut() = Some(fe.clone()));

                                // Apply any theme that was requested before the
                                // root element existed (e.g. from a first-mount
                                // use_effect).
                                if let Some(theme) = PENDING_THEME.with(|p| p.take()) {
                                    let _ = fe.SetRequestedTheme(theme);
                                    update_titlebar_theme();
                                }
                            }
                        }

                        // Wire TitleBar to window on every root change (mirrors C# mount behavior).
                        if let Some(tb) = state.render_host.with_backend(|b| b.find_titlebar()) {
                            let _ = state.window.SetExtendsContentIntoTitleBar(true);
                            if let Ok(tb_ui) = tb.cast::<UIElement>() {
                                let _ = state.window.SetTitleBar(&tb_ui);
                            }
                            // SetPreferredHeightOption is silently ignored unless
                            // ExtendsContentIntoTitleBar is already true.
                            if let Some(tall) = PENDING_TALL.with(|p| p.take()) {
                                set_titlebar_height(tall);
                            }
                        }
                    }
                }
                None => {
                    last_attached_for_hook.set(None);
                }
            }
        });

        render_host.kick();

        Ok(Self {
            render_host,
            window,
            presenter: Cell::new(PresenterKind::Default),
            backdrop: Cell::new(None),
            icon: RefCell::new(None),
        })
    }

    /// Set the window presenter (full-screen / compact overlay / default).
    /// Must be called before [`Self::activate`].
    pub fn set_presenter(&self, kind: PresenterKind) {
        self.presenter.set(kind);
    }

    /// Set the window backdrop material (Mica, Mica Alt, or Acrylic).
    /// Must be called before [`Self::activate`].
    pub fn set_backdrop(&self, backdrop: Backdrop) {
        self.backdrop.set(Some(backdrop));
    }

    /// Set the window icon from a path to an `.ico` file, used for the
    /// title-bar and taskbar. Must be called before [`Self::activate`].
    pub fn set_icon(&self, path: impl Into<String>) {
        *self.icon.borrow_mut() = Some(path.into());
    }

    pub fn activate(&self) -> Result<()> {
        let presenter = self.presenter.get();
        let backdrop = self.backdrop.get();
        let icon = self.icon.borrow().clone();
        let window = self.window.clone();
        let handler = DispatcherQueueHandler::new(move || {
            fault::catch("activate", || {
                let mut hwnd: HWND = HWND::default();
                if let Ok(native) = window.cast::<IWindowNative>() {
                    let _ = unsafe { native.WindowHandle(&mut hwnd) };
                }

                let app_window = window.cast::<IWindow2>().and_then(|w| w.AppWindow()).ok();
                if let Some(app_window) = &app_window {
                    if let Some(native_kind) = presenter.to_native()
                        && let Err(err) = app_window.SetPresenterByKind(native_kind)
                    {
                        fault::report("window presenter", format!("{err}"));
                    }
                    if let Some(icon) = &icon
                        && let Err(err) = app_window.SetIcon(icon)
                    {
                        fault::report("window icon", format!("{err}"));
                    }
                }
                if let Some(bd) = backdrop
                    && let Err(err) = bd.apply_to(&window)
                {
                    fault::report("backdrop", format!("{err}"));
                }
                let _ = window.Activate();

                // Clear the OS-supplied AppStarting cursor by posting a synthetic
                // WM_SETCURSOR; otherwise the spinner persists until the first
                // mouse move. PostMessageW (not SendMessageW) avoids flicker.
                if !hwnd.is_null() {
                    let lparam: LPARAM =
                        (((WM_MOUSEMOVE) << 16) | (HTCLIENT & 0xFFFF)) as i32 as LPARAM;
                    unsafe {
                        let _ = PostMessageW(hwnd, WM_SETCURSOR, hwnd as WPARAM, lparam);
                    }
                }
            });
        });
        let queue = DispatcherQueue::GetForCurrentThread()?;
        queue.TryEnqueueWithPriority(DispatcherQueuePriority::High, &handler)?;
        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn stats(&self) -> RenderStats {
        self.render_host.stats()
    }

    pub fn set_render_complete<F>(&self, f: F)
    where
        F: Fn(&RenderCompleteInfo) + 'static,
    {
        self.render_host.set_render_complete(f);
    }
}

fn get_default_display_size(hwnd: HWND, dpi: u32) -> WindowSize {
    unsafe {
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut monitor_info_ex = MONITORINFOEXW {
            Base: MONITORINFO {
                cbSize: size_of::<MONITORINFOEXW>() as u32,
                ..MONITORINFO::default()
            },
            ..MONITORINFOEXW::default()
        };
        if GetMonitorInfoW(monitor, &mut monitor_info_ex.Base).as_bool() {
            let work = monitor_info_ex.Base.rcWork;
            let work_width = work.right.saturating_sub(work.left);
            let work_height = work.bottom.saturating_sub(work.top);
            let scale = dpi as f64 / 96.0;
            WindowSize {
                width: work_width as f64 / scale / 2.0,
                height: work_height as f64 / scale / 2.0,
            }
        } else {
            WindowSize::default()
        }
    }
}

fn center_window_on_display(
    hwnd: HWND,
    client_width_px: i32,
    client_height_px: i32,
    nc_width_px: i32,
    nc_height_px: i32,
) {
    unsafe {
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut monitor_info_ex = MONITORINFOEXW {
            Base: MONITORINFO {
                cbSize: size_of::<MONITORINFOEXW>() as u32,
                ..MONITORINFO::default()
            },
            ..MONITORINFOEXW::default()
        };
        if !GetMonitorInfoW(monitor, &mut monitor_info_ex.Base).as_bool() {
            return;
        }
        let work = monitor_info_ex.Base.rcWork;
        let work_width = work.right.saturating_sub(work.left);
        let work_height = work.bottom.saturating_sub(work.top);

        let outer_width = client_width_px.saturating_add(nc_width_px);
        let outer_height = client_height_px.saturating_add(nc_height_px);
        let x = work.left + (work_width.saturating_sub(outer_width)) / 2;
        let y = work.top + (work_height.saturating_sub(outer_height)) / 2;
        let _ = SetWindowPos(
            hwnd,
            HWND::default(),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

fn subscribe_size_and_dpi(
    fe: &FrameworkElement,
    render_host: RenderHost<WinUIBackend, WinUIDispatcher>,
    window: Window,
    constraints: InnerConstraints,
) {
    let mut hwnd: HWND = HWND::default();
    if let Ok(native) = window.cast::<IWindowNative>() {
        let _ = unsafe { native.WindowHandle(&mut hwnd) };
    }

    let _ = fe
        .SizeChanged(move |_sender, args| {
            let size = args.unwrap().NewSize().unwrap();
            let new_dpi = unsafe { GetDpiForWindow(hwnd) };
            if new_dpi > 0 {
                render_host.set_dpi(new_dpi);
            }
            render_host.set_inner_size(WindowSize {
                width: size.width as f64,
                height: size.height as f64,
            });
            let _ = apply_constraints_for_window(&window, render_host.dpi(), &constraints);
        })
        .ok()
        .map(|r| r.into_token());
}

fn create_window(
    title: impl AsRef<str>,
    size: Option<WindowSize>,
    constraints: InnerConstraints,
) -> std::result::Result<(Window, WindowSize, u32), Error> {
    let window = Window::new()?;

    let mut hwnd = HWND::default();
    unsafe {
        window.cast::<IWindowNative>()?.WindowHandle(&mut hwnd)?;
    }
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    let dpi = if dpi == 0 { 96 } else { dpi };

    window.SetTitle(title.as_ref())?;

    let dip_size = match size {
        Some(s) => s,
        None => get_default_display_size(hwnd, dpi),
    };

    let dip_to_px = |dips: f64| (dips * dpi as f64 / 96.0).round() as i32;

    let window_2 = window.cast::<IWindow2>()?;
    let app_window = window_2.AppWindow()?;
    let app_window_2 = app_window.cast::<IAppWindow2>()?;
    app_window_2.ResizeClient(SizeInt32 {
        width: dip_to_px(dip_size.width),
        height: dip_to_px(dip_size.height),
    })?;

    app_window.SetPresenterByKind(AppWindowPresenterKind::Overlapped)?;
    set_requested_theme(RequestedTheme::Default);

    let outer_size = app_window.Size()?;
    let inner_size = app_window_2.ClientSize()?;
    let nc_width_px = outer_size.width.saturating_sub(inner_size.width);
    let nc_height_px = outer_size.height.saturating_sub(inner_size.height);

    let overlapped = app_window.Presenter()?.cast::<IOverlappedPresenter3>()?;
    if let Some(min_w) = constraints.min_width {
        overlapped.SetPreferredMinimumWidth(Some(dip_to_px(min_w).saturating_add(nc_width_px)))?;
    }
    if let Some(min_h) = constraints.min_height {
        overlapped
            .SetPreferredMinimumHeight(Some(dip_to_px(min_h).saturating_add(nc_height_px)))?;
    }
    if let Some(max_w) = constraints.max_width {
        overlapped.SetPreferredMaximumWidth(Some(dip_to_px(max_w).saturating_add(nc_width_px)))?;
    }
    if let Some(max_h) = constraints.max_height {
        overlapped
            .SetPreferredMaximumHeight(Some(dip_to_px(max_h).saturating_add(nc_height_px)))?;
    }

    let actual_client_px = app_window_2.ClientSize()?;
    let actual_dip_size = WindowSize {
        width: actual_client_px.width as f64 * 96.0 / dpi as f64,
        height: actual_client_px.height as f64 * 96.0 / dpi as f64,
    };

    center_window_on_display(
        hwnd,
        actual_client_px.width,
        actual_client_px.height,
        nc_width_px,
        nc_height_px,
    );

    Ok((window, actual_dip_size, dpi))
}

/// Re-apply DIP `constraints` to the window's `OverlappedPresenter`,
/// re-measuring the non-client offset at current DPI.
fn apply_constraints_for_window(
    window: &Window,
    dpi: u32,
    constraints: &InnerConstraints,
) -> Result<()> {
    let dip_scale = dpi as f64 / 96.0;
    let dip_to_px = |dips: f64| (dips * dip_scale).round() as i32;

    let app_window = window.cast::<IWindow2>()?.AppWindow()?;
    let app_window_2 = app_window.cast::<IAppWindow2>()?;

    let outer_size = app_window.Size()?;
    let inner_size = app_window_2.ClientSize()?;
    let nc_width_px = outer_size.width.saturating_sub(inner_size.width);
    let nc_height_px = outer_size.height.saturating_sub(inner_size.height);

    let presenter = app_window.Presenter()?.cast::<IOverlappedPresenter3>()?;

    if let Some(min_w) = constraints.min_width {
        presenter.SetPreferredMinimumWidth(Some(dip_to_px(min_w).saturating_add(nc_width_px)))?;
    }
    if let Some(min_h) = constraints.min_height {
        presenter.SetPreferredMinimumHeight(Some(dip_to_px(min_h).saturating_add(nc_height_px)))?;
    }
    if let Some(max_w) = constraints.max_width {
        presenter.SetPreferredMaximumWidth(Some(dip_to_px(max_w).saturating_add(nc_width_px)))?;
    }
    if let Some(max_h) = constraints.max_height {
        presenter.SetPreferredMaximumHeight(Some(dip_to_px(max_h).saturating_add(nc_height_px)))?;
    }
    Ok(())
}

impl<B: Backend + 'static, D: Dispatcher + 'static> RenderHost<B, D> {
    pub fn with_backend<R>(&self, f: impl FnOnce(&B) -> R) -> R {
        self.with_reconciler(|r| f(&r.backend))
    }
}

fn subscribe_actual_theme_changed(
    fe: &FrameworkElement,
    render_host: RenderHost<WinUIBackend, WinUIDispatcher>,
) {
    update_color_scheme_from(fe);

    let _ = fe
        .ActualThemeChanged(move |sender, _| {
            if let Some(fe) = sender.as_ref() {
                update_color_scheme_from(fe);
                update_titlebar_theme();
            }
            render_host.with_reconciler_mut(|r| r.notify_theme_changed());
            render_host.request_render();
        })
        .ok()
        .map(|r| r.into_token());
}

fn update_color_scheme_from(fe: &FrameworkElement) {
    if let Ok(theme) = fe.ActualTheme() {
        let scheme = match theme {
            ElementTheme::Dark => ColorScheme::Dark,
            _ => ColorScheme::Light,
        };
        set_current_color_scheme(scheme);
    }
}

struct AttachState {
    window: Window,
    render_host: RenderHost<WinUIBackend, WinUIDispatcher>,
}
