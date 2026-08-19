use super::*;

struct WindowMetricsState {
    constraints: Cell<WindowConstraints>,
    dpi: Cell<u32>,
}

pub(super) struct NativeWindow {
    value: bindings::Window,
    app_window: bindings::AppWindow,
    hwnd: bindings::HWND,
    title: String,
    backdrop: Option<WindowBackdrop>,
    icon: Option<WindowIcon>,
    theme: WindowTheme,
    title_bar: SystemTitleBar,
    overlapped: WindowOverlappedPolicy,
    pub(super) root: Option<bindings::UIElement>,
    metrics: Rc<WindowMetricsState>,
    framework_closing: Rc<Cell<bool>>,
    _changed_revoker: windows_core::EventRevoker,
    _theme_changed_revoker: Option<windows_core::EventRevoker>,
    _closing_revoker: windows_core::EventRevoker,
    _closed_revoker: windows_core::EventRevoker,
}

fn window_handle(window: &bindings::Window) -> WindowsResult<bindings::HWND> {
    let mut hwnd = std::ptr::null_mut();
    unsafe {
        window
            .cast::<bindings::IWindowNative>()?
            .WindowHandle(&mut hwnd)
            .ok()?;
    }
    assert!(!hwnd.is_null(), "native window handle is null");
    Ok(hwnd)
}

fn window_dpi(hwnd: bindings::HWND) -> u32 {
    unsafe { bindings::GetDpiForWindow(hwnd) }.max(96)
}

fn apply_window_backdrop(
    window: &bindings::Window,
    backdrop: Option<WindowBackdrop>,
) -> WindowsResult<()> {
    let backdrop = match backdrop {
        Some(WindowBackdrop::Mica) => {
            Some(bindings::MicaBackdrop::new()?.cast::<bindings::SystemBackdrop>()?)
        }
        Some(WindowBackdrop::MicaAlt) => {
            let backdrop = bindings::MicaBackdrop::new()?;
            backdrop.SetKind(bindings::MicaKind::BaseAlt)?;
            Some(backdrop.cast::<bindings::SystemBackdrop>()?)
        }
        Some(WindowBackdrop::Acrylic) => {
            Some(bindings::DesktopAcrylicBackdrop::new()?.cast::<bindings::SystemBackdrop>()?)
        }
        None => None,
    };
    window
        .cast::<bindings::IWindow2>()?
        .SetSystemBackdrop(backdrop.as_ref())
}

fn apply_window_theme(root: &bindings::UIElement, theme: WindowTheme) -> WindowsResult<()> {
    root.cast::<bindings::IFrameworkElement>()?
        .SetRequestedTheme(match theme {
            WindowTheme::System => bindings::ElementTheme::Default,
            WindowTheme::Light => bindings::ElementTheme::Light,
            WindowTheme::Dark => bindings::ElementTheme::Dark,
        })
}

fn color_scheme(element: &bindings::IFrameworkElement) -> WindowsResult<ColorScheme> {
    Ok(match element.ActualTheme()? {
        bindings::ElementTheme::Dark => ColorScheme::Dark,
        bindings::ElementTheme::Light => ColorScheme::Light,
        _ => panic!("actual element theme must resolve to light or dark"),
    })
}

fn native_color(value: Color) -> bindings::Color {
    bindings::Color {
        a: value.a,
        r: value.r,
        g: value.g,
        b: value.b,
    }
}

fn apply_title_bar_theme(
    title_bar: &bindings::AppWindowTitleBar,
    theme: WindowTheme,
) -> WindowsResult<()> {
    title_bar
        .cast::<bindings::IAppWindowTitleBar3>()?
        .SetPreferredTheme(match theme {
            WindowTheme::System => bindings::TitleBarTheme::UseDefaultAppMode,
            WindowTheme::Light => bindings::TitleBarTheme::Light,
            WindowTheme::Dark => bindings::TitleBarTheme::Dark,
        })
}

fn apply_window_title_bar(
    app_window: &bindings::AppWindow,
    value: SystemTitleBar,
    theme: WindowTheme,
) -> WindowsResult<()> {
    assert!(
        value.is_default() || bindings::AppWindowTitleBar::IsCustomizationSupported()?,
        "window title-bar customization is not supported"
    );
    let title_bar = app_window.TitleBar()?;
    if value.is_default() {
        title_bar.ResetToDefault()?;
    } else {
        title_bar.SetExtendsContentIntoTitleBar(value.extend_content)?;
        title_bar
            .cast::<bindings::IAppWindowTitleBar2>()?
            .SetPreferredHeightOption(match value.buttons {
                SystemTitleBarButtonPolicy::Hidden => bindings::TitleBarHeightOption::Collapsed,
                SystemTitleBarButtonPolicy::System => match value.height {
                    TitleBarHeight::Standard => bindings::TitleBarHeightOption::Standard,
                    TitleBarHeight::Tall => bindings::TitleBarHeightOption::Tall,
                },
            })?;
        title_bar.SetIconShowOptions(match value.icon {
            SystemTitleBarIconPolicy::ShowIconAndSystemMenu => {
                bindings::IconShowOptions::ShowIconAndSystemMenu
            }
            SystemTitleBarIconPolicy::HideIconAndSystemMenu => {
                bindings::IconShowOptions::HideIconAndSystemMenu
            }
        })?;
        let colors = value.colors;
        title_bar.SetForegroundColor(colors.foreground.map(native_color))?;
        title_bar.SetBackgroundColor(colors.background.map(native_color))?;
        title_bar.SetInactiveForegroundColor(colors.inactive_foreground.map(native_color))?;
        title_bar.SetInactiveBackgroundColor(colors.inactive_background.map(native_color))?;
        title_bar.SetButtonForegroundColor(colors.button_foreground.map(native_color))?;
        title_bar.SetButtonBackgroundColor(colors.button_background.map(native_color))?;
        title_bar
            .SetButtonHoverForegroundColor(colors.button_hover_foreground.map(native_color))?;
        title_bar
            .SetButtonHoverBackgroundColor(colors.button_hover_background.map(native_color))?;
        title_bar
            .SetButtonPressedForegroundColor(colors.button_pressed_foreground.map(native_color))?;
        title_bar
            .SetButtonPressedBackgroundColor(colors.button_pressed_background.map(native_color))?;
        title_bar.SetButtonInactiveForegroundColor(
            colors.button_inactive_foreground.map(native_color),
        )?;
        title_bar.SetButtonInactiveBackgroundColor(
            colors.button_inactive_background.map(native_color),
        )?;
    }
    apply_title_bar_theme(&title_bar, theme)
}

fn apply_window_constraints(
    app_window: &bindings::AppWindow,
    dpi: u32,
    constraints: WindowConstraints,
) -> WindowsResult<()> {
    let app_window_2 = app_window.cast::<bindings::IAppWindow2>()?;
    let outer = app_window.Size()?;
    let client = app_window_2.ClientSize()?;
    let non_client_width = outer.width.saturating_sub(client.width);
    let non_client_height = outer.height.saturating_sub(client.height);
    let dip_to_px = |value: f64| (value * dpi as f64 / 96.0).round() as i32;
    let presenter = app_window
        .Presenter()?
        .cast::<bindings::IOverlappedPresenter3>()?;

    presenter.SetPreferredMinimumWidth(None)?;
    presenter.SetPreferredMinimumHeight(None)?;
    presenter.SetPreferredMaximumWidth(None)?;
    presenter.SetPreferredMaximumHeight(None)?;
    if let Some(value) = constraints.min_width {
        presenter
            .SetPreferredMinimumWidth(Some(dip_to_px(value).saturating_add(non_client_width)))?;
    }
    if let Some(value) = constraints.min_height {
        presenter
            .SetPreferredMinimumHeight(Some(dip_to_px(value).saturating_add(non_client_height)))?;
    }
    if let Some(value) = constraints.max_width {
        presenter
            .SetPreferredMaximumWidth(Some(dip_to_px(value).saturating_add(non_client_width)))?;
    }
    if let Some(value) = constraints.max_height {
        presenter
            .SetPreferredMaximumHeight(Some(dip_to_px(value).saturating_add(non_client_height)))?;
    }
    Ok(())
}

fn apply_overlapped_policy(
    app_window: &bindings::AppWindow,
    policy: WindowOverlappedPolicy,
) -> WindowsResult<()> {
    let presenter = app_window
        .Presenter()?
        .cast::<bindings::IOverlappedPresenter>()?;
    presenter.SetIsResizable(policy.resizable)?;
    presenter.SetIsMinimizable(policy.minimizable)?;
    presenter.SetIsMaximizable(policy.maximizable)
}

impl WinUiRuntime {
    pub(super) fn request_exit_if_empty(&self) {
        if self.windows.is_empty() {
            self.schedule_windows_empty();
        }
    }

    pub(super) fn create_window(&mut self, id: NodeId, create: &WindowCreate) -> WindowsResult<()> {
        assert!(!self.windows.contains_key(&id), "window already exists");
        self.exit_revision
            .set(self.exit_revision.get().wrapping_add(1));
        let value = bindings::Window::new()?;
        let hwnd = window_handle(&value)?;
        value.SetTitle(&create.title)?;
        let app_window = value.cast::<bindings::IWindow2>()?.AppWindow()?;

        let framework_closing = Rc::new(Cell::new(false));
        let metrics = Rc::new(WindowMetricsState {
            constraints: Cell::new(WindowConstraints::default()),
            dpi: Cell::new(window_dpi(hwnd)),
        });
        let changed_app_window = app_window.clone();
        let changed_metrics = Rc::clone(&metrics);
        let changed_events = Rc::clone(&self.events);
        let changed_waker = Rc::clone(&self.waker);
        let changed_revoker = app_window.Changed(move |_sender, args| {
            let args = args.as_ref().unwrap();
            if !args.DidPositionChange().unwrap() && !args.DidSizeChange().unwrap() {
                return;
            }
            let current_dpi = window_dpi(hwnd);
            if changed_metrics.dpi.replace(current_dpi) != current_dpi {
                let constraints = changed_metrics.constraints.get();
                if !constraints.is_empty() {
                    apply_window_constraints(&changed_app_window, current_dpi, constraints)
                        .unwrap();
                }
            }
            if args.DidSizeChange().unwrap() {
                let size = changed_app_window
                    .cast::<bindings::IAppWindow2>()
                    .unwrap()
                    .ClientSize()
                    .unwrap();
                let px_to_dip = |value: i32| value as f64 * 96.0 / current_dpi as f64;
                changed_events
                    .borrow_mut()
                    .push_back(NativeEvent::WindowSizeChanged {
                        target: id,
                        size: WindowSize {
                            width: px_to_dip(size.width),
                            height: px_to_dip(size.height),
                        },
                    });
                if let Some(wake) = changed_waker.borrow().as_ref() {
                    wake();
                }
            }
        })?;
        let closing_state = Rc::clone(&framework_closing);
        let closing_shutdown = Rc::clone(&self.shutting_down);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let closing_revoker = app_window.Closing(move |_sender, args| {
            if closing_state.get() || closing_shutdown.get() {
                return;
            }
            args.as_ref().unwrap().SetCancel(true).unwrap();
            events
                .borrow_mut()
                .push_back(NativeEvent::WindowCloseRequested { target: id });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;

        let closed_state = Rc::clone(&framework_closing);
        let closed_shutdown = Rc::clone(&self.shutting_down);
        let closed_revoker = value.Closed(move |_sender, _args| {
            assert!(
                closed_state.get() || closed_shutdown.get(),
                "native window closed outside Reactor ownership"
            );
        })?;

        self.windows.insert(
            id,
            NativeWindow {
                value,
                app_window,
                hwnd,
                title: create.title.clone(),
                backdrop: None,
                icon: None,
                theme: WindowTheme::System,
                title_bar: SystemTitleBar::default(),
                overlapped: WindowOverlappedPolicy::default(),
                root: None,
                metrics,
                framework_closing,
                _changed_revoker: changed_revoker,
                _theme_changed_revoker: None,
                _closing_revoker: closing_revoker,
                _closed_revoker: closed_revoker,
            },
        );
        Ok(())
    }

    pub(super) fn set_window_owner(&self, owner: NodeId, child: NodeId) -> WindowsResult<()> {
        let owner_hwnd = self
            .windows
            .get(&owner)
            .unwrap_or_else(|| panic!("owned window parent is unknown"))
            .hwnd;
        let child_hwnd = self
            .windows
            .get(&child)
            .unwrap_or_else(|| panic!("owned window is unknown"))
            .hwnd;
        unsafe {
            bindings::SetWindowLongPtrW(child_hwnd, bindings::GWLP_HWNDPARENT, owner_hwnd as _);
            assert_eq!(
                bindings::GetWindowLongPtrW(child_hwnd, bindings::GWLP_HWNDPARENT),
                owner_hwnd as _,
                "failed to assign owned window parent"
            );
        }
        Ok(())
    }

    pub(super) fn set_window_content(
        &mut self,
        window: NodeId,
        content: NodeId,
    ) -> WindowsResult<()> {
        let id = window;
        let root = self.node(content)?.handle.ui_element()?;
        let framework: bindings::IFrameworkElement = root.cast()?;
        let events = Rc::clone(&self.events);
        let theme_events = Rc::clone(&events);
        let waker = Rc::clone(&self.waker);
        let theme_framework = framework.clone();
        let theme_changed_revoker = framework.ActualThemeChanged(move |_sender, _args| {
            theme_events
                .borrow_mut()
                .push_back(NativeEvent::WindowColorSchemeChanged {
                    target: id,
                    scheme: color_scheme(&theme_framework).unwrap(),
                });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        let window = self
            .windows
            .get_mut(&window)
            .unwrap_or_else(|| panic!("window is unknown"));
        let activate = window.root.is_none();
        apply_window_theme(&root, window.theme)?;
        window.value.SetContent(&root)?;
        window.root = Some(root);
        window._theme_changed_revoker = Some(theme_changed_revoker);
        events
            .borrow_mut()
            .push_back(NativeEvent::WindowColorSchemeChanged {
                target: id,
                scheme: color_scheme(&framework)?,
            });
        if activate {
            window.value.Activate()?;
        }
        Ok(())
    }

    pub(super) fn update_window(&mut self, id: NodeId, update: &WindowUpdate) -> WindowsResult<()> {
        if let WindowUpdate::BindTitleBar(title_bar) = update {
            let title_bar = self.node(*title_bar)?.handle.ui_element()?;
            return self
                .windows
                .get(&id)
                .unwrap_or_else(|| panic!("window is unknown"))
                .value
                .SetTitleBar(&title_bar);
        }
        if matches!(update, WindowUpdate::UnbindTitleBar) {
            return self
                .windows
                .get(&id)
                .unwrap_or_else(|| panic!("window is unknown"))
                .value
                .SetTitleBar(None::<&bindings::UIElement>);
        }
        let window = self
            .windows
            .get_mut(&id)
            .unwrap_or_else(|| panic!("window is unknown"));
        match update {
            WindowUpdate::Title(title) => {
                window.value.SetTitle(title)?;
                window.title.clone_from(title);
                Ok(())
            }
            WindowUpdate::Backdrop(backdrop) => {
                apply_window_backdrop(&window.value, *backdrop)?;
                window.backdrop = *backdrop;
                Ok(())
            }
            WindowUpdate::Icon(icon) => {
                window.app_window.SetIcon(icon.path())?;
                window.icon = Some(icon.clone());
                Ok(())
            }
            WindowUpdate::Theme(theme) => {
                window.theme = *theme;
                if let Some(root) = &window.root {
                    apply_window_theme(root, *theme)?;
                }
                apply_title_bar_theme(&window.app_window.TitleBar()?, *theme)?;
                Ok(())
            }
            WindowUpdate::TitleBar(title_bar) => {
                apply_window_title_bar(&window.app_window, **title_bar, window.theme)?;
                window.title_bar = **title_bar;
                Ok(())
            }
            WindowUpdate::BindTitleBar(_) | WindowUpdate::UnbindTitleBar => unreachable!(),
            WindowUpdate::Overlapped(policy) => {
                apply_overlapped_policy(&window.app_window, *policy)?;
                window.overlapped = *policy;
                Ok(())
            }
            WindowUpdate::ClientSize(size) => {
                let dpi = window_dpi(window.hwnd);
                window.metrics.dpi.set(dpi);
                let dip_to_px = |value: f64| {
                    let value = (value * dpi as f64 / 96.0).round();
                    assert!(
                        value <= i32::MAX as f64,
                        "window client size exceeds the native pixel range"
                    );
                    value as i32
                };
                window
                    .app_window
                    .cast::<bindings::IAppWindow2>()?
                    .ResizeClient(bindings::SizeInt32 {
                        width: dip_to_px(size.width),
                        height: dip_to_px(size.height),
                    })
            }
            WindowUpdate::Constraints(constraints) => {
                let constraints = constraints.value();
                window.metrics.constraints.set(constraints);
                let dpi = window_dpi(window.hwnd);
                window.metrics.dpi.set(dpi);
                apply_window_constraints(&window.app_window, dpi, constraints)
            }
            WindowUpdate::Presenter(presenter) => {
                let presenter = match presenter {
                    WindowPresenter::Default => bindings::AppWindowPresenterKind::Default,
                    WindowPresenter::FullScreen => bindings::AppWindowPresenterKind::FullScreen,
                    WindowPresenter::CompactOverlay => {
                        bindings::AppWindowPresenterKind::CompactOverlay
                    }
                };
                window.app_window.SetPresenterByKind(presenter)
            }
        }
    }

    pub(super) fn activate_window(&mut self, id: NodeId) -> WindowsResult<()> {
        self.windows
            .get(&id)
            .unwrap_or_else(|| panic!("window is unknown"))
            .value
            .Activate()?;
        #[cfg(test)]
        self.window_activations.push(id);
        Ok(())
    }

    pub(super) fn close_window(&mut self, id: NodeId) -> WindowsResult<()> {
        let mut window = self
            .windows
            .remove(&id)
            .unwrap_or_else(|| panic!("window is unknown"));
        window.framework_closing.set(true);
        window.value.SetContent(None::<&bindings::UIElement>)?;
        window.root.take();
        window.value.Close()?;
        if self.windows.is_empty() {
            self.schedule_windows_empty();
        }
        Ok(())
    }

    fn schedule_windows_empty(&self) {
        let revision = self.exit_revision.get().wrapping_add(1);
        self.exit_revision.set(revision);
        let exit_revision = Rc::clone(&self.exit_revision);
        let on_windows_empty = Rc::clone(&self.on_windows_empty);
        let handler = bindings::DispatcherQueueHandler::new(move || {
            if exit_revision.get() == revision {
                on_windows_empty();
            }
        });
        assert!(
            self.dispatcher
                .TryEnqueueWithPriority(bindings::DispatcherQueuePriority::Normal, &handler)
                .unwrap(),
            "failed to enqueue application exit"
        );
    }

    pub(super) fn shutdown_windows(&mut self) {
        for (_, mut window) in std::mem::take(&mut self.windows) {
            window.framework_closing.set(true);
            _ = window.value.SetContent(None::<&bindings::UIElement>);
            window.root.take();
            _ = window.value.Close();
        }
    }
}

#[cfg(test)]
#[path = "../../testing/private/winui/window_access.rs"]
pub(super) mod tests;
