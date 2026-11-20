#[allow(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]
mod bindings;
use super::*;
use bindings::*;
use std::os::windows::ffi::*;
use windows_core::*;
use windows_window::*;

const CALLBACK_MESSAGE: u32 = WM_USER as u32 + 1;
const ICON_ID: u32 = 1;

/// A point in screen coordinates.
pub type Point = POINT;

/// Native work to enqueue for the application's guarded drain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotifyIconEvent {
    /// The icon was selected with the mouse or keyboard.
    Activate { position: Point },
    /// The user requested the icon's context menu.
    ContextMenu { position: Point },
    /// The Windows Shell restarted and the registration needs restoring.
    Recover,
}

type EventHandler = Box<dyn Fn(NotifyIconEvent)>;

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
                _ = DestroyIcon(self.handle.cast());
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
            hWnd: hwnd.cast(),
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
            hWnd: hwnd.cast(),
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
    ) -> bool
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let mut data = self.data(hwnd);
        data.hIcon = icon.handle.cast();
        if !notify(NIM_MODIFY as u32, &data) {
            return false;
        }
        self.icon = icon;
        true
    }

    fn set_tooltip_with<F>(
        &mut self,
        tooltip: Option<[u16; 128]>,
        hwnd: *mut core::ffi::c_void,
        notify: &mut F,
    ) -> bool
    where
        F: FnMut(u32, &NOTIFYICONDATAW) -> bool,
    {
        let mut data = self.data(hwnd);
        data.szTip = tooltip.unwrap_or([0; 128]);
        if !notify(NIM_MODIFY as u32, &data) {
            return false;
        }
        self.tooltip = tooltip;
        true
    }

    fn data(&self, hwnd: *mut core::ffi::c_void) -> NOTIFYICONDATAW {
        let flags = NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP;
        NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd.cast(),
            uID: ICON_ID,
            uFlags: flags as u32,
            uCallbackMessage: CALLBACK_MESSAGE,
            hIcon: self.icon.handle.cast(),
            szTip: self.tooltip.unwrap_or([0; 128]),
            ..Default::default()
        }
    }
}

/// A notification-area icon and its hidden callback window.
pub struct NotifyIcon {
    callback_window: Window,
    registration: Registration,
}

impl NotifyIcon {
    /// Begins configuring a notification-area icon loaded from an `.ico` file.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(path: impl Into<PathBuf>) -> NotifyIconBuilder {
        NotifyIconBuilder {
            handler: None,
            icon: path.into(),
            tooltip: None,
        }
    }

    /// Replaces the icon using an `.ico` file.
    ///
    /// Returns `false` if the Shell rejects the update, retaining the last applied image.
    pub fn set_icon(&mut self, path: impl AsRef<Path>) -> Result<bool> {
        let icon = OwnedIcon::load(path.as_ref())?;
        Ok(self.registration.replace_icon_with(
            icon,
            self.callback_window.hwnd(),
            &mut shell_notify,
        ))
    }

    /// Sets or clears the standard tooltip.
    ///
    /// Returns `false` if the Shell rejects the update, retaining the last applied tooltip.
    pub fn set_tooltip(&mut self, tooltip: Option<&str>) -> Result<bool> {
        let tooltip = tooltip.map(tooltip_text).transpose()?;
        Ok(self.registration.set_tooltip_with(
            tooltip,
            self.callback_window.hwnd(),
            &mut shell_notify,
        ))
    }

    pub fn recover(&self) -> Result<()> {
        self.registration.recover(self.callback_window.hwnd())
    }
}

impl Drop for NotifyIcon {
    fn drop(&mut self) {
        self.registration.delete(self.callback_window.hwnd());
    }
}

/// Configures a [`NotifyIcon`].
pub struct NotifyIconBuilder {
    handler: Option<EventHandler>,
    icon: PathBuf,
    tooltip: Option<String>,
}

impl NotifyIconBuilder {
    /// Sets the standard tooltip shown for the icon.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Sets the queue sink. It must not run application work or pump messages.
    pub fn on_event<F>(mut self, handler: F) -> Self
    where
        F: Fn(NotifyIconEvent) + 'static,
    {
        self.handler = Some(Box::new(handler));
        self
    }

    /// Creates the hidden callback window and adds the icon to the notification area.
    pub fn build(self) -> Result<NotifyIcon> {
        let taskbar_created = register_taskbar_created()?;
        let registration = Registration {
            icon: OwnedIcon::load(&self.icon)?,
            tooltip: self.tooltip.as_deref().map(tooltip_text).transpose()?,
        };
        let callback_window = callback_window(self.handler, taskbar_created).create()?;
        allow_message(callback_window.hwnd(), CALLBACK_MESSAGE)?;
        allow_message(callback_window.hwnd(), taskbar_created)?;
        registration.add(callback_window.hwnd())?;
        Ok(NotifyIcon {
            callback_window,
            registration,
        })
    }
}

fn callback_window(handler: Option<EventHandler>, taskbar_created: u32) -> WindowBuilder {
    Window::new("windows-notifyicon")
        .style(0)
        .visible(false)
        .quit_on_close(false)
        .on_message(move |_, message, wparam, lparam| {
            let event = if message == taskbar_created {
                Some(NotifyIconEvent::Recover)
            } else if message == CALLBACK_MESSAGE {
                decode_event(wparam, lparam)
            } else {
                return None;
            };
            if let Some(event) = event
                && let Some(handler) = &handler
            {
                handler(event);
            }
            Some(0)
        })
}

fn allow_message(hwnd: *mut core::ffi::c_void, message: u32) -> Result<()> {
    if unsafe {
        ChangeWindowMessageFilterEx(
            hwnd.cast(),
            message,
            MSGFLT_ALLOW as u32,
            core::ptr::null_mut(),
        )
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

fn decode_event(wparam: usize, lparam: isize) -> Option<NotifyIconEvent> {
    let event = lparam as u16 as u32;
    let position = Point {
        x: wparam as u16 as i16 as i32,
        y: (wparam >> 16) as u16 as i16 as i32,
    };
    match event as i32 {
        NIN_SELECT | NIN_KEYSELECT => Some(NotifyIconEvent::Activate { position }),
        WM_CONTEXTMENU => Some(NotifyIconEvent::ContextMenu { position }),
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
    unsafe { Shell_NotifyIconW(message, core::ptr::from_ref(data).cast_mut()) }.as_bool()
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

    #[test]
    fn builder_accepts_the_public_configuration() {
        let _builder =
            NotifyIcon::new("icon.ico")
                .tooltip("Example")
                .on_event(|event| match event {
                    NotifyIconEvent::Activate { position }
                    | NotifyIconEvent::ContextMenu { position } => {
                        let _ = position;
                    }
                    NotifyIconEvent::Recover => {}
                });
    }

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
            Some(NotifyIconEvent::Activate {
                position: Point { x: -10, y: -20 }
            })
        );
        assert_eq!(
            decode_event(position, WM_CONTEXTMENU as isize),
            Some(NotifyIconEvent::ContextMenu {
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

        assert!(registration.set_tooltip_with(
            Some(tooltip_text("updated").unwrap()),
            core::ptr::null_mut(),
            &mut notify,
        ));
        assert!(registration.set_tooltip_with(None, core::ptr::null_mut(), &mut notify));

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
    fn refused_tooltip_updates_preserve_applied_payload_until_retry_succeeds() {
        let mut registration = registration(1);
        let previous = Some(tooltip_text("previous").unwrap());
        let updated = tooltip_text("updated").unwrap();
        registration.tooltip = previous;
        let mut results = [false, true, false, true].into_iter();
        let mut payloads = Vec::new();
        let mut notify = |message, data: &NOTIFYICONDATAW| {
            assert_eq!(message, NIM_MODIFY as u32);
            assert_eq!(data.hIcon as usize, 1);
            payloads.push(data.szTip);
            results.next().unwrap()
        };

        assert!(!registration.set_tooltip_with(Some(updated), core::ptr::null_mut(), &mut notify));
        assert_eq!(registration.tooltip, previous);
        assert!(registration.set_tooltip_with(Some(updated), core::ptr::null_mut(), &mut notify));
        assert_eq!(registration.tooltip, Some(updated));
        assert!(!registration.set_tooltip_with(None, core::ptr::null_mut(), &mut notify));
        assert_eq!(registration.tooltip, Some(updated));
        assert!(registration.set_tooltip_with(None, core::ptr::null_mut(), &mut notify));
        assert_eq!(registration.tooltip, None);
        assert_eq!(payloads, [updated, updated, [0; 128], [0; 128]]);
    }

    #[test]
    fn refused_image_updates_preserve_applied_payload_until_retry_succeeds() {
        let mut registration = registration(1);
        let tooltip = tooltip_text("retained").unwrap();
        registration.tooltip = Some(tooltip);
        let mut results = [false, false, true].into_iter();
        let mut payloads = Vec::new();
        let mut notify = |message, data: &NOTIFYICONDATAW| {
            assert_ne!(data.uFlags & NIF_ICON as u32, 0);
            assert_eq!({ data.szTip }, tooltip);
            payloads.push((message, data.hIcon as usize));
            results.next().unwrap()
        };

        assert!(!registration.replace_icon_with(
            OwnedIcon::borrowed(2),
            core::ptr::null_mut(),
            &mut notify,
        ));
        assert_eq!(registration.icon.handle as usize, 1);
        assert!(!registration.replace_icon_with(
            OwnedIcon::borrowed(3),
            core::ptr::null_mut(),
            &mut notify,
        ));
        assert_eq!(registration.icon.handle as usize, 1);
        assert!(registration.replace_icon_with(
            OwnedIcon::borrowed(3),
            core::ptr::null_mut(),
            &mut notify,
        ));
        assert_eq!(registration.icon.handle as usize, 3);
        assert_eq!(registration.tooltip, Some(tooltip));
        assert_eq!(
            payloads,
            [
                (NIM_MODIFY as u32, 2),
                (NIM_MODIFY as u32, 3),
                (NIM_MODIFY as u32, 3)
            ]
        );
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
