use crate::bindings::*;
use std::cell::{Cell, RefCell};
use std::mem::size_of;
use std::rc::{Rc, Weak};
use windows_core::{Error, HRESULT, PCWSTR, Result};
use windows_window::{Window, WindowBuilder};

const CALLBACK_MESSAGE: u32 = WM_USER as u32 + 1;
const ICON_ID: u32 = 1;
const E_FAIL: HRESULT = HRESULT(0x8000_4005_u32 as i32);
const E_INVALIDARG: HRESULT = HRESULT(0x8007_0057_u32 as i32);

/// A point in screen coordinates.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A rectangle in screen coordinates.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// A user interaction or availability change for a notification-area icon.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TrayIconEvent {
    /// The icon was selected with the mouse or keyboard.
    Activate { position: Point },
    /// The user requested the icon's context menu.
    ContextMenu { position: Point },
    /// The Windows Shell could not restore the icon after restarting.
    Unavailable,
}

type EventHandler = Box<dyn FnMut(TrayIconEvent)>;

struct OwnedIcon(*mut core::ffi::c_void);

impl OwnedIcon {
    fn load(path: &str) -> Result<Self> {
        let path = wide(path);
        let icon = unsafe {
            LoadImageW(
                core::ptr::null_mut(),
                PCWSTR(path.as_ptr()),
                IMAGE_ICON as u32,
                GetSystemMetrics(SM_CXSMICON),
                GetSystemMetrics(SM_CYSMICON),
                LR_LOADFROMFILE as u32,
            )
        };
        if icon.is_null() {
            Err(Error::from_thread())
        } else {
            Ok(Self(icon))
        }
    }
}

impl Drop for OwnedIcon {
    fn drop(&mut self) {
        unsafe {
            _ = DestroyIcon(self.0);
        }
    }
}

struct Registration {
    icon: OwnedIcon,
    tooltip: Option<[u16; 128]>,
}

impl Registration {
    fn add(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        let data = self.data(hwnd);
        if !unsafe { Shell_NotifyIconW(NIM_ADD as u32, &data) }.as_bool() {
            return Err(shell_error("failed to add notification-area icon"));
        }

        let version = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: ICON_ID,
            Anonymous: NOTIFYICONDATAW_0 {
                uVersion: NOTIFYICON_VERSION_4 as u32,
            },
            ..Default::default()
        };
        if !unsafe { Shell_NotifyIconW(NIM_SETVERSION as u32, &version) }.as_bool() {
            unsafe {
                _ = Shell_NotifyIconW(NIM_DELETE as u32, &data);
            }
            return Err(shell_error(
                "failed to enable notification-area icon version 4",
            ));
        }
        Ok(())
    }

    fn update(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        let data = self.data(hwnd);
        if unsafe { Shell_NotifyIconW(NIM_MODIFY as u32, &data) }.as_bool() {
            Ok(())
        } else {
            self.add(hwnd)
        }
    }

    fn recover(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        self.delete(hwnd);
        self.add(hwnd)
    }

    fn delete(&self, hwnd: *mut core::ffi::c_void) {
        let data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: ICON_ID,
            ..Default::default()
        };
        unsafe {
            _ = Shell_NotifyIconW(NIM_DELETE as u32, &data);
        }
    }

    fn data(&self, hwnd: *mut core::ffi::c_void) -> NOTIFYICONDATAW {
        let flags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: ICON_ID,
            uFlags: flags as u32,
            uCallbackMessage: CALLBACK_MESSAGE,
            hIcon: self.icon.0,
            szTip: self.tooltip.unwrap_or([0; 128]),
            ..Default::default()
        }
    }
}

struct Shared {
    active: Cell<bool>,
    handler: RefCell<Option<EventHandler>>,
    registration: RefCell<Registration>,
}

impl Shared {
    fn dispatch(&self, event: TrayIconEvent) {
        if let Some(handler) = self.handler.borrow_mut().as_mut() {
            handler(event);
        }
    }

    fn recover(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        let Ok(registration) = self.registration.try_borrow() else {
            return Ok(());
        };
        registration.recover(hwnd)
    }

    fn recover_if_missing(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        if icon_rect(hwnd).is_ok() {
            Ok(())
        } else {
            self.recover(hwnd)
        }
    }
}

/// A notification-area icon and its hidden callback window.
pub struct TrayIcon {
    window: Window,
    shared: Rc<Shared>,
}

impl TrayIcon {
    /// Begins configuring a notification-area icon loaded from an `.ico` file.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(path: impl Into<String>) -> TrayIconBuilder {
        TrayIconBuilder {
            handler: None,
            icon: path.into(),
            tooltip: None,
        }
    }

    /// Returns the hidden callback window's borrowed raw `HWND`.
    ///
    /// The handle remains owned by this value and must not be closed or destroyed.
    pub fn hwnd(&self) -> *mut core::ffi::c_void {
        self.window.hwnd()
    }

    /// Returns the icon's current bounding rectangle in screen coordinates.
    pub fn rect(&self) -> Result<Rect> {
        let value = icon_rect(self.window.hwnd())?;
        Ok(Rect {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        })
    }

    /// Replaces the icon using an `.ico` file.
    pub fn set_icon(&mut self, path: &str) -> Result<()> {
        let icon = OwnedIcon::load(path)?;
        let mut registration = self
            .shared
            .registration
            .try_borrow_mut()
            .map_err(|_| shell_error("notification-area icon is handling another update"))?;
        let previous = std::mem::replace(&mut registration.icon, icon);
        if let Err(error) = registration.update(self.window.hwnd()) {
            registration.icon = previous;
            _ = registration.update(self.window.hwnd());
            return Err(error);
        }
        Ok(())
    }

    /// Sets or clears the standard tooltip.
    pub fn set_tooltip(&mut self, tooltip: Option<&str>) -> Result<()> {
        let tooltip = tooltip.map(tooltip_text).transpose()?;
        let mut registration = self
            .shared
            .registration
            .try_borrow_mut()
            .map_err(|_| shell_error("notification-area icon is handling another update"))?;
        let previous = std::mem::replace(&mut registration.tooltip, tooltip);
        if let Err(error) = registration.update(self.window.hwnd()) {
            registration.tooltip = previous;
            _ = registration.update(self.window.hwnd());
            return Err(error);
        }
        Ok(())
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        self.shared.active.set(false);
        self.shared.registration.borrow().delete(self.window.hwnd());
    }
}

/// Configures a [`TrayIcon`].
pub struct TrayIconBuilder {
    handler: Option<EventHandler>,
    icon: String,
    tooltip: Option<String>,
}

impl TrayIconBuilder {
    /// Sets the standard tooltip shown for the icon.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Sets the icon's interaction handler.
    pub fn on_event<F>(mut self, handler: F) -> Self
    where
        F: FnMut(TrayIconEvent) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    /// Creates the hidden callback window and adds the icon to the notification area.
    pub fn build(self) -> Result<TrayIcon> {
        let taskbar_created = register_taskbar_created()?;
        let shared = Rc::new(Shared {
            active: Cell::new(true),
            handler: RefCell::new(self.handler),
            registration: RefCell::new(Registration {
                icon: OwnedIcon::load(&self.icon)?,
                tooltip: self.tooltip.as_deref().map(tooltip_text).transpose()?,
            }),
        });
        let callback = Rc::downgrade(&shared);
        let window = callback_window(callback, taskbar_created).create()?;
        shared.registration.borrow().add(window.hwnd())?;
        Ok(TrayIcon { window, shared })
    }
}

fn callback_window(shared: Weak<Shared>, taskbar_created: u32) -> WindowBuilder {
    Window::new("windows-trayicon")
        .style(0)
        .visible(false)
        .process_dpi_awareness(false)
        .quit_on_close(false)
        .on_message(move |hwnd, message, wparam, lparam| {
            let shared = shared.upgrade()?;
            if message == taskbar_created {
                if !shared.active.get() {
                    return Some(0);
                }
                let result = shared.recover(hwnd);
                if result.is_err() {
                    shared.dispatch(TrayIconEvent::Unavailable);
                }
                return Some(0);
            }
            if message == CALLBACK_MESSAGE {
                if let Some(event) = decode_event(wparam, lparam) {
                    shared.dispatch(event);
                    if shared.active.get() && shared.recover_if_missing(hwnd).is_err() {
                        shared.dispatch(TrayIconEvent::Unavailable);
                    }
                }
                return Some(0);
            }
            None
        })
}

fn icon_rect(hwnd: *mut core::ffi::c_void) -> Result<RECT> {
    let identifier = NOTIFYICONIDENTIFIER {
        cbSize: size_of::<NOTIFYICONIDENTIFIER>() as u32,
        hWnd: hwnd,
        uID: ICON_ID,
        ..Default::default()
    };
    let mut value = RECT::default();
    unsafe {
        Shell_NotifyIconGetRect(&identifier, &mut value).ok()?;
    }
    Ok(value)
}

fn decode_event(wparam: usize, lparam: isize) -> Option<TrayIconEvent> {
    let event = lparam as u16 as u32;
    let position = Point {
        x: wparam as u16 as i16 as i32,
        y: (wparam >> 16) as u16 as i16 as i32,
    };
    match event as i32 {
        NIN_SELECT | NIN_KEYSELECT => Some(TrayIconEvent::Activate { position }),
        WM_CONTEXTMENU => Some(TrayIconEvent::ContextMenu { position }),
        _ => None,
    }
}

fn register_taskbar_created() -> Result<u32> {
    let name = wide("TaskbarCreated");
    let message = unsafe { RegisterWindowMessageW(PCWSTR(name.as_ptr())) };
    if message == 0 {
        Err(Error::from_thread())
    } else {
        Ok(message)
    }
}

fn tooltip_text(value: &str) -> Result<[u16; 128]> {
    let value = value.encode_utf16().collect::<Vec<_>>();
    if value.len() >= 128 {
        return Err(Error::new(
            E_INVALIDARG,
            "notification-area tooltip exceeds 127 UTF-16 code units",
        ));
    }
    let mut result = [0; 128];
    result[..value.len()].copy_from_slice(&value);
    Ok(result)
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(Some(0)).collect()
}

fn shell_error(message: &'static str) -> Error {
    Error::new(E_FAIL, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_version_four_events_and_signed_coordinates() {
        let position = ((-20_i16 as u16 as usize) << 16) | (-10_i16 as u16 as usize);
        assert_eq!(
            decode_event(position, NIN_SELECT as isize),
            Some(TrayIconEvent::Activate {
                position: Point { x: -10, y: -20 }
            })
        );
        assert_eq!(
            decode_event(position, WM_CONTEXTMENU as isize),
            Some(TrayIconEvent::ContextMenu {
                position: Point { x: -10, y: -20 }
            })
        );
        assert_eq!(decode_event(position, 0), None);
    }

    #[test]
    fn validates_and_terminates_tooltips() {
        let value = tooltip_text("hello").unwrap();
        assert_eq!(
            &value[..6],
            &[
                b'h' as u16,
                b'e' as u16,
                b'l' as u16,
                b'l' as u16,
                b'o' as u16,
                0
            ]
        );
        assert!(tooltip_text(&"x".repeat(128)).is_err());
    }
}
