use super::*;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct PointerInput {
    pub pressed: Arc<Mutex<bool>>,
}

impl PointerInput {
    pub fn release(pressed: &Mutex<bool>) -> Result<()> {
        let mut pressed = pressed.lock().unwrap();
        if *pressed {
            send_mouse(MOUSEEVENTF_LEFTUP, 0, 0)?;
            *pressed = false;
        }
        Ok(())
    }
}

impl Drop for PointerInput {
    fn drop(&mut self) {
        // The supervisor owns this guard, even if its UIA worker stops responding.
        if let Err(error) = Self::release(&self.pressed) {
            eprintln!("failed to release injected mouse button: {error}");
        }
    }
}

pub fn send_mouse(flags: i32, x: i32, y: i32) -> Result<()> {
    let mut input = INPUT {
        r#type: INPUT_MOUSE as u32,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: x,
                dy: y,
                dwFlags: flags as u32,
                ..Default::default()
            },
        },
    };
    if unsafe { SendInput(1, &mut input, size_of::<INPUT>() as i32) } != 1 {
        return Err("mouse input was not accepted".into());
    }
    Ok(())
}

pub struct PhysicalCoordinates(DPI_AWARENESS_CONTEXT);

/// Waits until the global cursor is the given system cursor. Shared system cursors keep one
/// handle, so the handle identifies the shape the target process set.
pub fn system_cursor(ui: &Automation, name: &str, id: windows_core::PCSTR) -> Result<()> {
    let expected = unsafe { LoadCursorW(std::ptr::null_mut(), windows_core::PCWSTR(id.0 as _)) };
    if expected.is_null() {
        return Err(windows_core::Error::from_thread().into());
    }
    ui.wait(
        &format!("cursor becomes {name} ({expected:?})"),
        || unsafe {
            let mut info = CURSORINFO {
                cbSize: size_of::<CURSORINFO>() as u32,
                ..Default::default()
            };
            GetCursorInfo(&mut info).ok()?;
            Ok((
                info.hCursor,
                info.flags,
                info.ptScreenPos.x,
                info.ptScreenPos.y,
            ))
        },
        |observed| observed.0 == expected,
    )?;
    Ok(())
}

impl PhysicalCoordinates {
    pub fn new() -> Result<Self> {
        let previous =
            unsafe { SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
        if previous.is_null() {
            return Err(windows_core::Error::from_thread().into());
        }
        Ok(Self(previous))
    }
}

impl Drop for PhysicalCoordinates {
    fn drop(&mut self) {
        if unsafe { SetThreadDpiAwarenessContext(self.0) }.is_null() {
            eprintln!(
                "failed to restore thread DPI context: {}",
                windows_core::Error::from_thread()
            );
        }
    }
}
