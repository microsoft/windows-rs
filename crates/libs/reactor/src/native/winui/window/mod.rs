use super::*;

pub(super) mod placement;

/// Advanced native window handle.
///
/// Clones share the native window but not the `Closed` registration. The registration belongs to
/// the handle passed to [`NativeWindow::set_closed`] and is revoked when that handle is dropped or
/// registers a replacement.
pub struct NativeWindow {
    state: Rc<NativeWindowState>,
    actual_theme_changed: Option<windows_core::EventRevoker>,
    closed: Option<windows_core::EventRevoker>,
    published_title: Option<String>,
    published_visuals: bool,
    size_changed: Option<windows_core::EventRevoker>,
    placement_changed: Option<windows_core::EventRevoker>,
    visuals: WindowVisuals,
}

pub(super) struct NativeWindowState {
    window: native::Window,
    root: ObjectId,
    root_element: native::FrameworkElement,
    surface: native::Border,
    background_style: native::Style,
    title_bar: Cell<Option<(ObjectId, WindowTitleBarHeight)>>,
    shown: Cell<bool>,
    closed: Cell<bool>,
    maximize_on_first_show: Cell<bool>,
    placement_callback: RefCell<Option<Rc<dyn Fn(WindowPlacement)>>>,
    last_placement: Cell<Option<WindowPlacement>>,
}

impl NativeWindowState {
    fn observe_placement(&self) -> windows_core::Result<()> {
        if !self.shown.get() || self.closed.get() {
            return Ok(());
        }
        let Some(callback) = self.placement_callback.borrow().clone() else {
            return Ok(());
        };
        let hwnd = unsafe {
            self.window
                .cast::<native::IWindowNative>()?
                .WindowHandle()?
        };
        if let Some(placement) = placement::read_placement(hwnd)?
            && self.last_placement.replace(Some(placement)) != Some(placement)
        {
            callback(placement);
        }
        Ok(())
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        if self.placement_changed.take().is_some() {
            self.state.placement_callback.borrow_mut().take();
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WindowTitleBarHeight {
    #[default]
    Standard,
    Tall,
    Collapsed,
}

impl Clone for NativeWindow {
    fn clone(&self) -> Self {
        Self {
            state: Rc::clone(&self.state),
            actual_theme_changed: None,
            closed: None,
            published_title: self.published_title.clone(),
            published_visuals: self.published_visuals,
            size_changed: None,
            placement_changed: None,
            visuals: self.visuals.clone(),
        }
    }
}

impl NativeWindow {
    pub fn raw_handle(&self) -> Result<*mut core::ffi::c_void, WinUiError> {
        let handle = unsafe {
            self.state
                .window
                .cast::<native::IWindowNative>()?
                .WindowHandle()?
        };
        Ok(handle.cast())
    }

    pub(crate) fn apply_publication(
        &mut self,
        title: Option<&str>,
        visuals: Option<&WindowVisuals>,
        initial: bool,
    ) -> Result<(), WinUiError> {
        if self.published_title.as_deref() != title {
            self.state.window.SetTitle(title.unwrap_or_default())?;
            self.published_title = title.map(str::to_owned);
        }

        if let Some(visuals) = visuals {
            if initial
                && let Some(position) = visuals
                    .initial_placement
                    .map(|placement| ScreenPoint::new(placement.x, placement.y))
                    .or(visuals.initial_position)
            {
                // Resolve the target DPI before applying DIP constraints and client sizing.
                placement::position_hidden(self.raw_handle()?.cast(), position)?;
            }
            self.apply_visuals(visuals, initial)?;
            if initial && let Some(placement) = visuals.initial_placement {
                placement::restore_hidden(self.raw_handle()?.cast(), placement)?;
                self.state.maximize_on_first_show.set(placement.maximized);
            }
            self.published_visuals = true;
        } else if self.published_visuals {
            self.apply_visuals(&WindowVisuals::default(), false)?;
            self.published_visuals = false;
        }

        Ok(())
    }

    pub(crate) fn set_size_observer(
        &mut self,
        observer: Option<Rc<dyn Fn(WindowSize)>>,
    ) -> Result<(), WinUiError> {
        self.size_changed = None;
        self.size_changed = observer
            .map(|observer| {
                let window = self
                    .state
                    .window
                    .cast::<native::IWindow2>()?
                    .AppWindow()?
                    .cast::<native::IAppWindow2>()?;
                let hwnd = self.raw_handle()?;
                self.state.root_element.SizeChanged(move |_, args| {
                    if args.as_ref().is_none() {
                        return;
                    }
                    // Root layout can exclude a border pixel from the native client area.
                    let size = window.ClientSize().unwrap();
                    let dpi = unsafe { native::GetDpiForWindow(hwnd.cast()) }.max(96);
                    let dips = 96.0 / f64::from(dpi);
                    observer(WindowSize {
                        width: f64::from(size.width) * dips,
                        height: f64::from(size.height) * dips,
                    });
                })
            })
            .transpose()?;
        Ok(())
    }

    pub(crate) fn set_color_scheme_observer(
        &mut self,
        observer: Option<Rc<dyn Fn(ColorScheme)>>,
    ) -> Result<(), WinUiError> {
        self.actual_theme_changed = None;
        self.actual_theme_changed = observer
            .map(|observer| {
                let initial = match self.state.root_element.ActualTheme()? {
                    native::ElementTheme::Dark => ColorScheme::Dark,
                    _ => ColorScheme::Light,
                };
                let last = Cell::new(initial);
                observer(initial);
                self.state
                    .root_element
                    .ActualThemeChanged(move |sender, _| {
                        let Some(sender) = sender.as_ref() else {
                            return;
                        };
                        let scheme = match sender.ActualTheme().unwrap() {
                            native::ElementTheme::Dark => ColorScheme::Dark,
                            _ => ColorScheme::Light,
                        };
                        if last.replace(scheme) != scheme {
                            observer(scheme);
                        }
                    })
            })
            .transpose()?;

        Ok(())
    }

    fn apply_visuals(&mut self, visuals: &WindowVisuals, initial: bool) -> Result<(), WinUiError> {
        let changes = window_visual_changes(&self.visuals, visuals);
        if !initial && !changes.any() {
            return Ok(());
        }
        let window_2 = self.state.window.cast::<native::IWindow2>()?;
        let app_window = window_2.AppWindow()?;
        let mut hwnd = None;
        let mut window_handle = || -> Result<*mut core::ffi::c_void, WinUiError> {
            if let Some(hwnd) = hwnd {
                return Ok(hwnd);
            }
            let value = self.raw_handle()?;
            hwnd = Some(value);
            Ok(value)
        };

        if changes.backdrop {
            match visuals.backdrop {
                WindowBackdrop::None => {
                    window_2.SetSystemBackdrop(None::<&native::SystemBackdrop>)?;
                }
                WindowBackdrop::Mica | WindowBackdrop::MicaAlt => {
                    let mica = native::MicaBackdrop::new()?;
                    mica.SetKind(match visuals.backdrop {
                        WindowBackdrop::Mica => native::MicaKind::Base,
                        WindowBackdrop::MicaAlt => native::MicaKind::BaseAlt,
                        _ => unreachable!(),
                    })?;
                    let backdrop: native::SystemBackdrop = mica.cast()?;
                    window_2.SetSystemBackdrop(&backdrop)?;
                }
                WindowBackdrop::Acrylic => {
                    let backdrop: native::SystemBackdrop =
                        native::DesktopAcrylicBackdrop::new()?.cast()?;
                    window_2.SetSystemBackdrop(&backdrop)?;
                }
            }
            self.state
                .surface
                .cast::<native::FrameworkElement>()?
                .SetStyle(
                    (visuals.backdrop == WindowBackdrop::None)
                        .then_some(&self.state.background_style),
                )?;
        }

        if changes.theme {
            app_window
                .TitleBar()?
                .cast::<native::IAppWindowTitleBar3>()?
                .SetPreferredTheme(match visuals.theme {
                    WindowTheme::System => native::TitleBarTheme::UseDefaultAppMode,
                    WindowTheme::Light => native::TitleBarTheme::Light,
                    WindowTheme::Dark => native::TitleBarTheme::Dark,
                })?;
            self.state
                .surface
                .cast::<native::FrameworkElement>()?
                .SetRequestedTheme(match visuals.theme {
                    WindowTheme::System => native::ElementTheme::Default,
                    WindowTheme::Light => native::ElementTheme::Light,
                    WindowTheme::Dark => native::ElementTheme::Dark,
                })?;
        }

        if changes.icon {
            if let Some(path) = &visuals.icon {
                app_window.SetIcon(path)?;
            } else {
                clear_window_icon(window_handle()?);
            }
        }

        if changes.constraints || initial {
            let presenter = app_window
                .Presenter()?
                .cast::<native::IOverlappedPresenter3>()?;
            let (min_width, min_height, max_width, max_height) =
                if let Some(constraints) = visuals.constraints {
                    let dpi = unsafe { native::GetDpiForWindow(window_handle()?.cast()) }.max(96);
                    let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
                    let client_window = app_window.cast::<native::IAppWindow2>()?;
                    let outer = app_window.Size()?;
                    let inner = client_window.ClientSize()?;
                    let non_client_width = outer.width.saturating_sub(inner.width);
                    let non_client_height = outer.height.saturating_sub(inner.height);
                    (
                        constraints
                            .min_width
                            .map(|value| pixels(value).saturating_add(non_client_width)),
                        constraints
                            .min_height
                            .map(|value| pixels(value).saturating_add(non_client_height)),
                        constraints
                            .max_width
                            .map(|value| pixels(value).saturating_add(non_client_width)),
                        constraints
                            .max_height
                            .map(|value| pixels(value).saturating_add(non_client_height)),
                    )
                } else {
                    (None, None, None, None)
                };
            presenter.SetPreferredMinimumWidth(min_width)?;
            presenter.SetPreferredMinimumHeight(min_height)?;
            presenter.SetPreferredMaximumWidth(max_width)?;
            presenter.SetPreferredMaximumHeight(max_height)?;
        }

        if (changes.client_size || initial)
            && !(initial && visuals.initial_placement.is_some())
            && let Some((width, height)) = visuals.client_size
        {
            let dpi = unsafe { native::GetDpiForWindow(window_handle()?.cast()) }.max(96);
            let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
            resize_window_client(
                &app_window,
                native::SizeInt32 {
                    width: pixels(width),
                    height: pixels(height),
                },
            )?;
        }

        self.visuals = visuals.clone();
        Ok(())
    }

    pub fn activate(&self) -> Result<(), WinUiError> {
        if self.state.maximize_on_first_show.replace(false) {
            placement::show_maximized(self.raw_handle()?.cast())?;
        }
        self.state.window.Activate()?;
        self.state.shown.set(true);
        self.state.observe_placement()?;
        Ok(())
    }

    pub(crate) fn restore_and_activate(&self) -> Result<(), WinUiError> {
        let hwnd = self.raw_handle()?.cast();
        unsafe {
            if native::IsIconic(hwnd).as_bool() {
                _ = native::ShowWindow(hwnd, native::SW_RESTORE);
            }
        }
        self.activate()
    }

    pub(crate) fn set_placement_observer(
        &mut self,
        observer: Option<Rc<dyn Fn(WindowPlacement)>>,
    ) -> Result<(), WinUiError> {
        self.placement_changed = None;
        self.state.last_placement.set(None);
        *self.state.placement_callback.borrow_mut() = observer;
        if self.state.placement_callback.borrow().is_some() {
            let state = Rc::downgrade(&self.state);
            self.placement_changed = Some(
                self.state
                    .window
                    .cast::<native::IWindow2>()?
                    .AppWindow()?
                    .Changed(move |_, _| {
                        if let Some(state) = state.upgrade() {
                            state.observe_placement().unwrap();
                        }
                    })?,
            );
            self.state.observe_placement()?;
        }
        Ok(())
    }

    pub fn close(&self) -> Result<(), WinUiError> {
        self.state.window.Close().map_err(Into::into)
    }

    pub fn set_closed(&mut self, callback: impl Fn() + 'static) -> Result<(), WinUiError> {
        let state = Rc::downgrade(&self.state);
        self.closed = Some(self.state.window.Closed(move |_, _| {
            if let Some(state) = state.upgrade() {
                state.closed.set(true);
            }
            callback();
        })?);
        Ok(())
    }
}

fn resize_window_client(
    window: &native::AppWindow,
    size: native::SizeInt32,
) -> windows_core::Result<()> {
    // ResizeClient includes the native caption even when XAML extends into the title bar.
    let outer = window.Size()?;
    let inner = window.cast::<native::IAppWindow2>()?.ClientSize()?;
    window.Resize(native::SizeInt32 {
        width: size
            .width
            .saturating_add(outer.width.saturating_sub(inner.width)),
        height: size
            .height
            .saturating_add(outer.height.saturating_sub(inner.height)),
    })
}

#[derive(Clone, Copy)]
pub(super) struct WindowVisualChanges {
    pub(super) backdrop: bool,
    pub(super) client_size: bool,
    pub(super) constraints: bool,
    pub(super) icon: bool,
    pub(super) theme: bool,
}

impl WindowVisualChanges {
    pub(super) fn any(self) -> bool {
        self.backdrop || self.client_size || self.constraints || self.icon || self.theme
    }
}

pub(super) fn window_visual_changes(
    previous: &WindowVisuals,
    next: &WindowVisuals,
) -> WindowVisualChanges {
    WindowVisualChanges {
        backdrop: previous.backdrop != next.backdrop,
        client_size: previous.client_size != next.client_size,
        constraints: previous.constraints != next.constraints,
        icon: previous.icon != next.icon,
        theme: previous.theme != next.theme,
    }
}

fn clear_window_icon(hwnd: *mut core::ffi::c_void) {
    unsafe {
        _ = native::SendMessageW(
            hwnd.cast(),
            native::WM_SETICON as u32,
            native::ICON_BIG as native::WPARAM,
            0,
        );
        _ = native::SendMessageW(
            hwnd.cast(),
            native::WM_SETICON as u32,
            native::ICON_SMALL as native::WPARAM,
            0,
        );
    }
}

impl WinUiAdapter {
    pub fn create_window_with_policy(
        &self,
        root_object: ObjectId,
        policy: &WindowPolicy,
    ) -> Result<NativeWindow, WinUiError> {
        if self
            .live_windows()
            .iter()
            .any(|window| window.root == root_object)
        {
            return Err(WinUiError::DuplicateWindowRoot(root_object));
        }
        let window = native::Window::new()?;
        let root = self.ui_element(root_object)?;
        // The opaque XAML island follows the app theme, not the requested window theme.
        let surface = native::Border::new()?;
        let background_style = native::XamlReader::Load(
            "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
             TargetType='Border'><Setter Property='Background' \
             Value='{ThemeResource SystemControlBackgroundAltHighBrush}'/></Style>",
        )?
        .cast::<native::Style>()?;
        surface
            .cast::<native::FrameworkElement>()?
            .SetStyle(&background_style)?;
        surface.SetChild(&root)?;
        window.SetContent(&surface)?;
        let state = Rc::new(NativeWindowState {
            window,
            root: root_object,
            root_element: root.cast()?,
            surface,
            background_style,
            title_bar: Cell::new(None),
            shown: Cell::new(false),
            closed: Cell::new(false),
            maximize_on_first_show: Cell::new(false),
            placement_callback: RefCell::new(None),
            last_placement: Cell::new(None),
        });
        if let Some((object, height)) = self.window_title_bar
            && self.owns(state.root, object)
        {
            self.apply_title_bar_to_window(&state, object, height)?;
        }
        self.apply_window_policy(&state.window, &state.surface.cast()?, policy)?;
        self.windows.borrow_mut().push(Rc::downgrade(&state));
        Ok(NativeWindow {
            state,
            actual_theme_changed: None,
            closed: None,
            published_title: None,
            published_visuals: false,
            size_changed: None,
            placement_changed: None,
            visuals: WindowVisuals {
                client_size: policy.client_size,
                constraints: policy
                    .minimum_client_size
                    .map(|(width, height)| WindowConstraints {
                        min_width: Some(width),
                        min_height: Some(height),
                        ..Default::default()
                    }),
                theme: policy.theme,
                ..Default::default()
            },
        })
    }

    fn owns(&self, root: ObjectId, mut object: ObjectId) -> bool {
        loop {
            if object == root {
                return true;
            }
            let Some((parent, _)) = self.owners.get(&object) else {
                return false;
            };
            object = *parent;
        }
    }

    fn live_windows(&self) -> Vec<Rc<NativeWindowState>> {
        let mut windows = self.windows.borrow_mut();
        let live = windows.iter().filter_map(Weak::upgrade).collect::<Vec<_>>();
        windows.retain(|window| window.strong_count() != 0);
        live
    }

    fn apply_title_bar_to_window(
        &self,
        state: &NativeWindowState,
        object: ObjectId,
        height: WindowTitleBarHeight,
    ) -> Result<(), WinUiError> {
        if self.kind(object)? != ObjectType::TitleBar {
            return Err(WinUiError::InvalidObject(object));
        }
        let element = self.ui_element(object)?;
        element.SetIsTabStop(false)?;
        state.window.SetExtendsContentIntoTitleBar(true)?;
        state.window.SetTitleBar(&element)?;
        state
            .window
            .cast::<native::IWindow2>()?
            .AppWindow()?
            .TitleBar()?
            .cast::<native::IAppWindowTitleBar2>()?
            .SetPreferredHeightOption(match height {
                WindowTitleBarHeight::Standard => native::TitleBarHeightOption::Standard,
                WindowTitleBarHeight::Tall => native::TitleBarHeightOption::Tall,
                WindowTitleBarHeight::Collapsed => native::TitleBarHeightOption::Collapsed,
            })?;
        state.title_bar.set(Some((object, height)));
        Ok(())
    }

    fn clear_title_bar_from_window(
        state: &NativeWindowState,
        object: ObjectId,
    ) -> Result<(), WinUiError> {
        if !state
            .title_bar
            .get()
            .is_some_and(|(current, _)| current == object)
        {
            return Ok(());
        }
        state
            .window
            .cast::<native::IWindow2>()?
            .AppWindow()?
            .TitleBar()?
            .cast::<native::IAppWindowTitleBar2>()?
            .SetPreferredHeightOption(native::TitleBarHeightOption::Standard)?;
        state.window.SetTitleBar(None::<&native::UIElement>)?;
        state.window.SetExtendsContentIntoTitleBar(false)?;
        state.title_bar.set(None);
        Ok(())
    }

    pub(super) fn set_window_title_bar(
        &mut self,
        object: ObjectId,
        height: WindowTitleBarHeight,
    ) -> Result<(), WinUiError> {
        if self.kind(object)? != ObjectType::TitleBar {
            return Err(WinUiError::InvalidObject(object));
        }
        self.window_title_bar = Some((object, height));
        Ok(())
    }

    pub(super) fn clear_window_title_bar(&mut self, object: ObjectId) -> Result<(), WinUiError> {
        for window in self.live_windows() {
            Self::clear_title_bar_from_window(&window, object)?;
        }
        if self
            .window_title_bar
            .is_some_and(|(current, _)| current == object)
        {
            self.window_title_bar = None;
        }
        Ok(())
    }

    pub(super) fn sync_window_title_bars(&self) -> Result<(), WinUiError> {
        for window in self.live_windows() {
            let desired = self
                .window_title_bar
                .filter(|(object, _)| self.owns(window.root, *object));
            if window.title_bar.get() == desired {
                continue;
            }
            if let Some((object, _)) = window.title_bar.get() {
                Self::clear_title_bar_from_window(&window, object)?;
            }
            if let Some((object, height)) = desired {
                self.apply_title_bar_to_window(&window, object, height)?;
            }
        }
        Ok(())
    }

    fn apply_window_policy(
        &self,
        window: &native::Window,
        root: &native::UIElement,
        policy: &WindowPolicy,
    ) -> Result<(), WinUiError> {
        if let Some(title) = &policy.title {
            window.SetTitle(title)?;
        }

        let window_2 = window.cast::<native::IWindow2>()?;
        let app_window = window_2.AppWindow()?;
        let title_bar = app_window.TitleBar()?;
        title_bar
            .cast::<native::IAppWindowTitleBar3>()?
            .SetPreferredTheme(match policy.theme {
                WindowTheme::System => native::TitleBarTheme::UseDefaultAppMode,
                WindowTheme::Light => native::TitleBarTheme::Light,
                WindowTheme::Dark => native::TitleBarTheme::Dark,
            })?;
        root.cast::<native::FrameworkElement>()?
            .SetRequestedTheme(match policy.theme {
                WindowTheme::System => native::ElementTheme::Default,
                WindowTheme::Light => native::ElementTheme::Light,
                WindowTheme::Dark => native::ElementTheme::Dark,
            })?;

        let needs_metrics = policy.client_size.is_some() || policy.minimum_client_size.is_some();
        if needs_metrics {
            let hwnd = unsafe { window.cast::<native::IWindowNative>()?.WindowHandle()? };
            let dpi = unsafe { native::GetDpiForWindow(hwnd) }.max(96);
            let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
            let client_window = app_window.cast::<native::IAppWindow2>()?;

            if let Some((width, height)) = policy.minimum_client_size {
                let outer = app_window.Size()?;
                let inner = client_window.ClientSize()?;
                let non_client_width = outer.width.saturating_sub(inner.width);
                let non_client_height = outer.height.saturating_sub(inner.height);
                let presenter = app_window
                    .Presenter()?
                    .cast::<native::IOverlappedPresenter3>()?;
                presenter.SetPreferredMinimumWidth(Some(
                    pixels(width).saturating_add(non_client_width),
                ))?;
                presenter.SetPreferredMinimumHeight(Some(
                    pixels(height).saturating_add(non_client_height),
                ))?;
            }

            if let Some((width, height)) = policy.client_size {
                resize_window_client(
                    &app_window,
                    native::SizeInt32 {
                        width: pixels(width),
                        height: pixels(height),
                    },
                )?;
            }
        }
        Ok(())
    }
}
