use super::*;

pub(in crate::winui) fn title(runtime: &WinUiRuntime, id: NodeId) -> String {
    runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .title
        .clone()
}

pub(in crate::winui) fn owner(runtime: &WinUiRuntime, id: NodeId) -> Option<NodeId> {
    let hwnd = runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .hwnd;
    let owner = unsafe { bindings::GetWindowLongPtrW(hwnd, bindings::GWLP_HWNDPARENT) };
    runtime
        .windows
        .iter()
        .find_map(|(id, window)| (window.hwnd as isize == owner as isize).then_some(*id))
}

pub(in crate::winui) fn active(runtime: &WinUiRuntime) -> Option<NodeId> {
    let active = unsafe { bindings::GetActiveWindow() };
    runtime
        .windows
        .iter()
        .find_map(|(id, window)| (window.hwnd == active).then_some(*id))
}

pub(in crate::winui) fn active_handle() -> isize {
    unsafe { bindings::GetActiveWindow() as isize }
}

pub(in crate::winui) fn handle(runtime: &WinUiRuntime, id: NodeId) -> isize {
    runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .hwnd as isize
}

pub(in crate::winui) fn client_size(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<WindowSize> {
    let window = runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"));
    let size = window
        .app_window
        .cast::<bindings::IAppWindow2>()?
        .ClientSize()?;
    let dpi = unsafe { bindings::GetDpiForWindow(window.hwnd) }.max(96);
    Ok(WindowSize {
        width: size.width as f64 * 96.0 / dpi as f64,
        height: size.height as f64 * 96.0 / dpi as f64,
    })
}

pub(in crate::winui) fn presenter(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<WindowPresenter> {
    let kind = runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .app_window
        .Presenter()?
        .Kind()?;
    Ok(match kind {
        bindings::AppWindowPresenterKind::Default
        | bindings::AppWindowPresenterKind::Overlapped => WindowPresenter::Default,
        bindings::AppWindowPresenterKind::FullScreen => WindowPresenter::FullScreen,
        bindings::AppWindowPresenterKind::CompactOverlay => WindowPresenter::CompactOverlay,
        _ => panic!("unknown window presenter"),
    })
}

pub(in crate::winui) fn constraints(runtime: &WinUiRuntime, id: NodeId) -> WindowConstraints {
    runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .metrics
        .constraints
        .get()
}

pub(in crate::winui) fn activations(runtime: &WinUiRuntime) -> &[NodeId] {
    &runtime.window_activations
}

pub(in crate::winui) fn queue_close_request(runtime: &WinUiRuntime, id: NodeId) {
    assert!(runtime.windows.contains_key(&id), "window is unknown");
    runtime
        .events
        .borrow_mut()
        .push_back(NativeEvent::WindowCloseRequested { target: id });
}

pub(in crate::winui) fn backdrop(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<Option<WindowBackdrop>> {
    Ok(runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .backdrop)
}

pub(in crate::winui) fn icon(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<Option<WindowIcon>> {
    Ok(runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .icon
        .clone())
}

pub(in crate::winui) fn theme(runtime: &WinUiRuntime, id: NodeId) -> WindowsResult<WindowTheme> {
    let window = runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"));
    let theme = window
        .root
        .as_ref()
        .unwrap_or_else(|| panic!("window content is not assigned"))
        .cast::<bindings::IFrameworkElement>()?
        .RequestedTheme()?;
    Ok(match theme {
        bindings::ElementTheme::Default => WindowTheme::System,
        bindings::ElementTheme::Light => WindowTheme::Light,
        bindings::ElementTheme::Dark => WindowTheme::Dark,
        _ => panic!("unknown window theme"),
    })
}

pub(in crate::winui) fn title_bar(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<SystemTitleBar> {
    Ok(runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .title_bar)
}

pub(in crate::winui) fn overlapped(
    runtime: &WinUiRuntime,
    id: NodeId,
) -> WindowsResult<WindowOverlappedPolicy> {
    Ok(runtime
        .windows
        .get(&id)
        .unwrap_or_else(|| panic!("window is unknown"))
        .overlapped)
}
