use crate::bindings::*;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use windows_core::{Error, PCWSTR, Result};
use windows_window::{Window, WindowBuilder};

const CALLBACK_MESSAGE: u32 = WM_USER as u32 + 1;
const DISPATCH_MESSAGE: u32 = WM_USER as u32 + 2;
const ICON_ID: u32 = 1;

/// A point in screen coordinates.
pub type Point = POINT;

/// A rectangle in screen coordinates.
pub type Rect = RECT;

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

struct OwnedIcon {
    handle: *mut core::ffi::c_void,
    owned: bool,
}

impl OwnedIcon {
    fn load(path: &Path) -> Result<Self> {
        let path = wide_path(path)?;
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
            Ok(Self {
                handle: icon,
                owned: true,
            })
        }
    }

    #[cfg(test)]
    fn borrowed(handle: usize) -> Self {
        Self {
            handle: handle as _,
            owned: false,
        }
    }
}

impl Drop for OwnedIcon {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                _ = DestroyIcon(self.handle);
            }
        }
    }
}

struct Registration {
    icon: OwnedIcon,
    tooltip: Option<[u16; 128]>,
}

impl Registration {
    fn add(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        self.add_with(hwnd, &mut shell_notify)
    }

    fn add_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let data = self.data(hwnd);
        if !notify(NIM_ADD as u32, &data) {
            _ = notify(NIM_DELETE as u32, &data);
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
        if !notify(NIM_SETVERSION as u32, &version) {
            _ = notify(NIM_DELETE as u32, &data);
            return Err(shell_error(
                "failed to enable notification-area icon version 4",
            ));
        }
        Ok(())
    }

    fn update_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let data = self.data(hwnd);
        if notify(NIM_MODIFY as u32, &data) {
            Ok(())
        } else {
            Err(shell_error("failed to update notification-area icon"))
        }
    }

    fn recover(&self, hwnd: *mut core::ffi::c_void) -> Result<()> {
        let mut notify = shell_notify;
        self.recover_with(hwnd, &mut notify)
    }

    fn recover_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        self.delete_with(hwnd, notify);
        self.add_with(hwnd, notify)
    }

    fn delete(&self, hwnd: *mut core::ffi::c_void) {
        self.delete_with(hwnd, &mut shell_notify);
    }

    fn delete_with<F>(&self, hwnd: *mut core::ffi::c_void, notify: &mut F)
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: ICON_ID,
            ..Default::default()
        };
        _ = notify(NIM_DELETE as u32, &data);
    }

    fn replace_icon_with<F>(
        &mut self,
        icon: OwnedIcon,
        hwnd: *mut core::ffi::c_void,
        notify: &mut F,
    ) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let previous = std::mem::replace(&mut self.icon, icon);
        if let Err(error) = self.update_with(hwnd, notify) {
            self.icon = previous;
            return Err(error);
        }
        Ok(())
    }

    fn set_tooltip_with<F>(
        &mut self,
        tooltip: Option<[u16; 128]>,
        hwnd: *mut core::ffi::c_void,
        notify: &mut F,
    ) -> Result<()>
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let previous = std::mem::replace(&mut self.tooltip, tooltip);
        if let Err(error) = self.update_with(hwnd, notify) {
            self.tooltip = previous;
            return Err(error);
        }
        Ok(())
    }

    fn data(&self, hwnd: *mut core::ffi::c_void) -> NOTIFYICONDATAW {
        let flags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: ICON_ID,
            uFlags: flags as u32,
            uCallbackMessage: CALLBACK_MESSAGE,
            hIcon: self.icon.handle,
            szTip: self.tooltip.unwrap_or([0; 128]),
            ..Default::default()
        }
    }
}

struct Shared {
    active: Cell<bool>,
    callback_hwnd: Cell<*mut core::ffi::c_void>,
    dispatching: Cell<bool>,
    handler: RefCell<Option<EventHandler>>,
    pending: RefCell<VecDeque<Pending>>,
    registration: RefCell<Registration>,
}

enum Pending {
    Event(TrayIconEvent),
    Recover,
}

struct DispatchGuard<'a>(&'a Cell<bool>);

impl Drop for DispatchGuard<'_> {
    fn drop(&mut self) {
        self.0.set(false);
    }
}

impl Shared {
    fn dispatch(&self, event: TrayIconEvent) {
        if let Some(handler) = self.handler.borrow_mut().as_mut() {
            handler(event);
        }
    }

    fn post(&self, hwnd: *mut core::ffi::c_void, pending: Pending) {
        self.pending.borrow_mut().push_back(pending);
        if !unsafe { PostMessageW(hwnd, DISPATCH_MESSAGE, 0, 0) }.as_bool() {
            self.pending.borrow_mut().pop_back();
            eprintln!("windows-trayicon could not queue Shell work");
        }
    }

    fn dispatch_pending(&self) {
        if self.dispatching.replace(true) {
            return;
        }
        let _guard = DispatchGuard(&self.dispatching);

        // Nested loops can enqueue more work through the callback window, so keep draining until
        // the queue is empty after the active handler returns.
        while self.active.get() {
            let Some(pending) = self.pending.borrow_mut().pop_front() else {
                break;
            };
            match pending {
                Pending::Event(event) => self.dispatch(event),
                Pending::Recover => {
                    if self.recover().is_err() {
                        self.dispatch(TrayIconEvent::Unavailable);
                    }
                }
            }
        }
    }

    fn recover(&self) -> Result<()> {
        let Ok(registration) = self.registration.try_borrow() else {
            return Err(shell_error(
                "notification-area icon recovery was requested during an update",
            ));
        };
        registration.recover(self.callback_hwnd.get())
    }
}

/// A notification-area icon and its hidden callback window.
pub struct TrayIcon {
    // Drop the Shell-facing window before the dispatch window so no callback can target a
    // destroyed dispatch HWND.
    callback_window: Window,
    _dispatch_window: Window,
    shared: Rc<Shared>,
}

impl TrayIcon {
    /// Begins configuring a notification-area icon loaded from an `.ico` file.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(path: impl Into<PathBuf>) -> TrayIconBuilder {
        TrayIconBuilder {
            handler: None,
            icon: path.into(),
            tooltip: None,
        }
    }

    /// Returns the hidden callback window's borrowed raw `HWND`.
    ///
    /// The handle remains owned by this value and must not be closed or destroyed.
    /// Thread message loops must not filter exclusively to this handle because event dispatch uses
    /// another private window.
    pub fn hwnd(&self) -> *mut core::ffi::c_void {
        self.callback_window.hwnd()
    }

    /// Returns the Shell's current icon anchor rectangle in screen coordinates.
    ///
    /// For an icon hidden in the overflow area, Windows may return the overflow button rectangle.
    pub fn rect(&self) -> Result<Rect> {
        let value = icon_rect(self.callback_window.hwnd())?;
        Ok(Rect {
            left: value.left,
            top: value.top,
            right: value.right,
            bottom: value.bottom,
        })
    }

    /// Replaces the icon using an `.ico` file.
    pub fn set_icon(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let icon = OwnedIcon::load(path.as_ref())?;
        let mut registration = self
            .shared
            .registration
            .try_borrow_mut()
            .map_err(|_| shell_error("notification-area icon is handling another update"))?;
        registration.replace_icon_with(icon, self.callback_window.hwnd(), &mut shell_notify)
    }

    /// Sets or clears the standard tooltip.
    pub fn set_tooltip(&mut self, tooltip: Option<&str>) -> Result<()> {
        let tooltip = tooltip.map(tooltip_text).transpose()?;
        let mut registration = self
            .shared
            .registration
            .try_borrow_mut()
            .map_err(|_| shell_error("notification-area icon is handling another update"))?;
        registration.set_tooltip_with(tooltip, self.callback_window.hwnd(), &mut shell_notify)
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        self.shared.active.set(false);
        if let Ok(registration) = self.shared.registration.try_borrow() {
            registration.delete(self.callback_window.hwnd());
        }
    }
}

/// Configures a [`TrayIcon`].
pub struct TrayIconBuilder {
    handler: Option<EventHandler>,
    icon: PathBuf,
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
            callback_hwnd: Cell::new(core::ptr::null_mut()),
            dispatching: Cell::new(false),
            handler: RefCell::new(self.handler),
            pending: RefCell::new(VecDeque::new()),
            registration: RefCell::new(Registration {
                icon: OwnedIcon::load(&self.icon)?,
                tooltip: self.tooltip.as_deref().map(tooltip_text).transpose()?,
            }),
        });
        let dispatch_window = dispatch_window(Rc::downgrade(&shared)).create()?;
        let callback_window = callback_window(
            Rc::downgrade(&shared),
            dispatch_window.hwnd(),
            taskbar_created,
        )
        .create()?;
        shared.callback_hwnd.set(callback_window.hwnd());
        allow_message(callback_window.hwnd(), CALLBACK_MESSAGE)?;
        allow_message(callback_window.hwnd(), taskbar_created)?;
        shared.registration.borrow().add(callback_window.hwnd())?;
        Ok(TrayIcon {
            callback_window,
            _dispatch_window: dispatch_window,
            shared,
        })
    }
}

fn callback_window(
    shared: Weak<Shared>,
    dispatch_hwnd: *mut core::ffi::c_void,
    taskbar_created: u32,
) -> WindowBuilder {
    Window::new("windows-trayicon")
        .style(0)
        .visible(false)
        .quit_on_close(false)
        .on_message(move |_, message, wparam, lparam| {
            let shared = shared.upgrade()?;
            if message == taskbar_created {
                if shared.active.get() {
                    shared.post(dispatch_hwnd, Pending::Recover);
                }
                return Some(0);
            }
            if message == CALLBACK_MESSAGE {
                if let Some(event) = decode_event(wparam, lparam) {
                    shared.post(dispatch_hwnd, Pending::Event(event));
                }
                return Some(0);
            }
            None
        })
}

fn dispatch_window(shared: Weak<Shared>) -> WindowBuilder {
    Window::new("windows-trayicon-dispatch")
        .style(0)
        .visible(false)
        .quit_on_close(false)
        .on_message(move |_, message, _, _| {
            if message == DISPATCH_MESSAGE {
                if let Some(shared) = shared.upgrade() {
                    shared.dispatch_pending();
                }
                return Some(0);
            }
            None
        })
}

fn allow_message(hwnd: *mut core::ffi::c_void, message: u32) -> Result<()> {
    if unsafe {
        ChangeWindowMessageFilterEx(hwnd, message, MSGFLT_ALLOW as u32, core::ptr::null_mut())
    }
    .as_bool()
    {
        Ok(())
    } else {
        let error = Error::from_thread();
        if error.code().is_ok() {
            Err(shell_error(
                "failed to allow a notification-area window message",
            ))
        } else {
            Err(error)
        }
    }
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

fn shell_notify(message: u32, data: &NOTIFYICONDATAW) -> bool {
    unsafe { Shell_NotifyIconW(message, data) }.as_bool()
}

fn wide_path(value: &Path) -> Result<Vec<u16>> {
    let mut value = value.as_os_str().encode_wide().collect::<Vec<_>>();
    if value.contains(&0) {
        return Err(Error::new(
            E_INVALIDARG,
            "icon path contains a null character",
        ));
    }
    value.push(0);
    Ok(value)
}

fn tooltip_text(value: &str) -> Result<[u16; 128]> {
    let value = value.encode_utf16().collect::<Vec<_>>();
    if value.contains(&0) {
        return Err(Error::new(
            E_INVALIDARG,
            "notification-area tooltip contains a null character",
        ));
    }
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

    fn registration(icon: usize) -> Registration {
        Registration {
            icon: OwnedIcon::borrowed(icon),
            tooltip: None,
        }
    }

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
        assert!(tooltip_text("before\0after").is_err());
        assert!(tooltip_text(&"x".repeat(128)).is_err());
    }

    #[test]
    fn preserves_windows_paths_that_are_not_utf8() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;

        let path = PathBuf::from(OsString::from_wide(&[b'a' as u16, 0xd800]));
        assert_eq!(wide_path(&path).unwrap(), [b'a' as u16, 0xd800, 0]);
    }

    #[test]
    fn updates_and_clears_the_tooltip_payload() {
        let mut registration = registration(1);
        let mut payloads = Vec::new();
        let mut notify = |message, data: &NOTIFYICONDATAW| {
            payloads.push((message, data.uFlags, data.szTip));
            true
        };

        registration
            .set_tooltip_with(
                Some(tooltip_text("updated").unwrap()),
                core::ptr::null_mut(),
                &mut notify,
            )
            .unwrap();
        registration
            .set_tooltip_with(None, core::ptr::null_mut(), &mut notify)
            .unwrap();

        assert_eq!(payloads[0].0, NIM_MODIFY as u32);
        assert_ne!(payloads[0].1 & NIF_TIP as u32, 0);
        assert_eq!(
            &payloads[0].2[..8],
            &[
                b'u' as u16,
                b'p' as u16,
                b'd' as u16,
                b'a' as u16,
                b't' as u16,
                b'e' as u16,
                b'd' as u16,
                0,
            ]
        );
        assert_eq!(payloads[1].0, NIM_MODIFY as u32);
        assert!(payloads[1].2.iter().all(|value| *value == 0));
    }

    #[test]
    fn failed_icon_replacement_restores_the_previous_icon() {
        let mut registration = registration(1);
        let mut payloads = Vec::new();
        let mut notify = |message, data: &NOTIFYICONDATAW| {
            payloads.push((message, data.hIcon as usize));
            false
        };

        assert!(
            registration
                .replace_icon_with(OwnedIcon::borrowed(2), core::ptr::null_mut(), &mut notify)
                .is_err()
        );
        assert_eq!(registration.icon.handle as usize, 1);
        assert_eq!(payloads, [(NIM_MODIFY as u32, 2)]);
    }

    #[test]
    fn failed_version_selection_deletes_the_added_icon() {
        let registration = registration(1);
        let mut results = [true, false, true].into_iter();
        let mut calls = Vec::new();
        let mut notify = |message, _: &NOTIFYICONDATAW| {
            calls.push(message);
            results.next().unwrap()
        };

        assert!(
            registration
                .add_with(core::ptr::null_mut(), &mut notify)
                .is_err()
        );
        assert_eq!(
            calls,
            [NIM_ADD as u32, NIM_SETVERSION as u32, NIM_DELETE as u32]
        );
    }

    #[test]
    fn failed_modify_does_not_attempt_registration() {
        let registration = registration(1);
        let mut calls = Vec::new();
        let mut notify = |message, _: &NOTIFYICONDATAW| {
            calls.push(message);
            false
        };

        assert!(
            registration
                .update_with(core::ptr::null_mut(), &mut notify)
                .is_err()
        );
        assert_eq!(calls, [NIM_MODIFY as u32]);
    }

    #[test]
    fn recovery_deletes_then_adds_and_sets_version() {
        let registration = registration(1);
        let mut calls = Vec::new();
        let mut notify = |message, _: &NOTIFYICONDATAW| {
            calls.push(message);
            true
        };

        registration
            .recover_with(core::ptr::null_mut(), &mut notify)
            .unwrap();
        assert_eq!(
            calls,
            [NIM_DELETE as u32, NIM_ADD as u32, NIM_SETVERSION as u32]
        );
    }
}
