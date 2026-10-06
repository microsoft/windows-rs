use super::*;

struct PhysicalCoordinates(native::DPI_AWARENESS_CONTEXT);

impl PhysicalCoordinates {
    fn enter() -> windows_core::Result<Self> {
        let previous = unsafe {
            native::SetThreadDpiAwarenessContext(native::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)
        };
        if previous.is_null() {
            return Err(windows_core::Error::from_thread());
        }
        Ok(Self(previous))
    }
}

impl Drop for PhysicalCoordinates {
    fn drop(&mut self) {
        unsafe {
            _ = native::SetThreadDpiAwarenessContext(self.0);
        }
    }
}

fn invalid_bounds() -> windows_core::Error {
    windows_core::Error::new(
        HRESULT(0x80070057_u32 as i32),
        "window placement bounds overflow",
    )
}

pub(super) fn translate_rect(
    rect: native::RECT,
    x: i32,
    y: i32,
) -> windows_core::Result<native::RECT> {
    Ok(native::RECT {
        left: rect.left.checked_add(x).ok_or_else(invalid_bounds)?,
        top: rect.top.checked_add(y).ok_or_else(invalid_bounds)?,
        right: rect.right.checked_add(x).ok_or_else(invalid_bounds)?,
        bottom: rect.bottom.checked_add(y).ok_or_else(invalid_bounds)?,
    })
}

fn workspace_offset(
    hwnd: native::HWND,
    monitor: native::HMONITOR,
) -> windows_core::Result<(i32, i32)> {
    if unsafe { native::GetWindowLongW(hwnd, native::GWL_EXSTYLE) } & native::WS_EX_TOOLWINDOW != 0
    {
        return Ok((0, 0));
    }
    let mut info = native::MONITORINFO {
        cbSize: size_of::<native::MONITORINFO>() as u32,
        ..Default::default()
    };
    unsafe { native::GetMonitorInfoW(monitor, &mut info).ok()? };
    Ok((
        info.rcWork
            .left
            .checked_sub(info.rcMonitor.left)
            .ok_or_else(invalid_bounds)?,
        info.rcWork
            .top
            .checked_sub(info.rcMonitor.top)
            .ok_or_else(invalid_bounds)?,
    ))
}

fn native_placement(hwnd: native::HWND) -> windows_core::Result<native::WINDOWPLACEMENT> {
    let mut placement = native::WINDOWPLACEMENT {
        length: size_of::<native::WINDOWPLACEMENT>() as u32,
        ..Default::default()
    };
    unsafe { native::GetWindowPlacement(hwnd, &mut placement).ok()? };
    Ok(placement)
}

pub(super) fn read_placement(hwnd: native::HWND) -> windows_core::Result<Option<WindowPlacement>> {
    let _coordinates = PhysicalCoordinates::enter()?;
    let placement = native_placement(hwnd)?;
    if placement.showCmd == native::SW_SHOWMINIMIZED as u32 {
        return Ok(None);
    }
    let monitor =
        unsafe { native::MonitorFromWindow(hwnd, native::MONITOR_DEFAULTTONEAREST as u32) };
    let (x, y) = workspace_offset(hwnd, monitor)?;
    let rect = translate_rect(placement.rcNormalPosition, x, y)?;
    Ok(Some(WindowPlacement {
        x: rect.left,
        y: rect.top,
        width: rect
            .right
            .checked_sub(rect.left)
            .ok_or_else(invalid_bounds)?,
        height: rect
            .bottom
            .checked_sub(rect.top)
            .ok_or_else(invalid_bounds)?,
        maximized: placement.showCmd == native::SW_SHOWMAXIMIZED as u32,
    }))
}

pub(super) fn restore_hidden(
    hwnd: native::HWND,
    placement: WindowPlacement,
) -> windows_core::Result<()> {
    let _coordinates = PhysicalCoordinates::enter()?;
    let rect = native::RECT {
        left: placement.x,
        top: placement.y,
        right: placement
            .x
            .checked_add(placement.width)
            .ok_or_else(invalid_bounds)?,
        bottom: placement
            .y
            .checked_add(placement.height)
            .ok_or_else(invalid_bounds)?,
    };
    let monitor =
        unsafe { native::MonitorFromRect(&rect, native::MONITOR_DEFAULTTONEAREST as u32) };
    let (x, y) = workspace_offset(hwnd, monitor)?;
    let mut native = native_placement(hwnd)?;
    native.rcNormalPosition = translate_rect(
        rect,
        x.checked_neg().ok_or_else(invalid_bounds)?,
        y.checked_neg().ok_or_else(invalid_bounds)?,
    )?;
    native.showCmd = native::SW_HIDE as u32;
    unsafe { native::SetWindowPlacement(hwnd, &native).ok() }
}

pub(super) fn position_hidden(
    hwnd: native::HWND,
    position: ScreenPoint,
) -> windows_core::Result<()> {
    let _coordinates = PhysicalCoordinates::enter()?;
    unsafe {
        native::SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            position.x,
            position.y,
            0,
            0,
            (native::SWP_NOSIZE | native::SWP_NOACTIVATE | native::SWP_NOZORDER) as u32,
        )
        .ok()
    }
}

pub(super) fn show_maximized(hwnd: native::HWND) -> windows_core::Result<()> {
    let _coordinates = PhysicalCoordinates::enter()?;
    let mut placement = native_placement(hwnd)?;
    placement.showCmd = native::SW_SHOWMAXIMIZED as u32;
    unsafe { native::SetWindowPlacement(hwnd, &placement).ok() }
}
