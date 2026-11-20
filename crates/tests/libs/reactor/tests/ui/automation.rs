use super::*;
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use windows_core::{HRESULT, Interface};

pub const ELEMENT_UNAVAILABLE: HRESULT = HRESULT(0x80040201u32 as i32);
const UIA_TIMEOUT: HRESULT = HRESULT(0x80131505u32 as i32);
pub const INVOKE_PATTERN: i32 = 10000;
pub(super) const VALUE_PATTERN: i32 = 10002;
pub(super) const RANGE_VALUE_PATTERN: i32 = 10003;
pub(super) const EXPAND_COLLAPSE_PATTERN: i32 = 10005;
pub(super) const TEXT_PATTERN: i32 = 10014;
pub(super) const SELECTION_PATTERN: i32 = 10001;
const PROCESS_ID: i32 = 30002;
pub(super) const LIST_ITEM: i32 = 50007;
const MENU_ITEM: i32 = 50011;

pub fn snapshot(element: &IUIAutomationElement) -> windows_core::Result<(i32, i32, Vec<u32>)> {
    let bounds = unsafe { element.CurrentBoundingRectangle()? };
    let width = bounds.right - bounds.left;
    let height = bounds.bottom - bounds.top;
    assert!(width > 0 && height > 0);
    unsafe {
        let dc = GetDC(std::ptr::null_mut());
        assert!(!dc.is_null(), "GetDC failed");
        let memory = CreateCompatibleDC(dc);
        assert!(!memory.is_null(), "CreateCompatibleDC failed");
        let bitmap = CreateCompatibleBitmap(dc, width, height);
        assert!(!bitmap.is_null(), "CreateCompatibleBitmap failed");
        let previous = SelectObject(memory, bitmap.cast());
        assert!(!previous.is_null());
        let copied = BitBlt(
            memory,
            0,
            0,
            width,
            height,
            dc,
            bounds.left,
            bounds.top,
            SRCCOPY,
        );
        assert!(!SelectObject(memory, previous).is_null());
        let mut info = BITMAPINFO::default();
        info.bmiHeader.biSize = size_of_val(&info.bmiHeader) as u32;
        info.bmiHeader.biWidth = width;
        info.bmiHeader.biHeight = -height;
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        let mut buffer = vec![0u32; (width * height) as usize];
        let lines = GetDIBits(
            dc,
            bitmap,
            0,
            height as u32,
            buffer.as_mut_ptr().cast(),
            &mut info,
            DIB_RGB_COLORS as u32,
        );
        assert!(DeleteObject(bitmap.cast()).as_bool());
        assert!(DeleteDC(memory).as_bool());
        assert_ne!(ReleaseDC(std::ptr::null_mut(), dc), 0);
        assert!(copied.as_bool(), "BitBlt failed");
        assert_eq!(lines, height, "GetDIBits failed");
        Ok((width, height, buffer))
    }
}

pub fn retired(ui: &Automation, item: &IUIAutomationInvokePattern, selection: &str) -> Result<()> {
    let result = unsafe { item.Invoke() };
    assert!(
        result.is_ok() || result == ELEMENT_UNAVAILABLE || result == HRESULT(0x80004005u32 as i32),
        "unexpected retired-provider result: {result:?}"
    );
    ui.name_stays("selection", selection, Duration::from_millis(250))
}

pub struct Com;

impl Com {
    pub fn new() -> Result<Self> {
        unsafe { CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED as u32).ok()? };
        Ok(Self)
    }
}

impl Drop for Com {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

pub struct Automation {
    pub client: IUIAutomation,
    process: u32,
    condition: IUIAutomationCondition,
    title: String,
    window: Cell<Option<usize>>,
    cancelled: Arc<AtomicBool>,
    pressed: Arc<Mutex<bool>>,
    stderr: std::path::PathBuf,
}

impl Automation {
    pub fn new(
        process: u32,
        fixture: &str,
        cancelled: Arc<AtomicBool>,
        pressed: Arc<Mutex<bool>>,
        stderr: std::path::PathBuf,
    ) -> Result<Self> {
        unsafe {
            let mut raw = std::ptr::null_mut();
            CoCreateInstance(
                &CUIAutomation,
                std::ptr::null_mut(),
                CLSCTX_INPROC_SERVER as u32,
                &IUIAutomation::IID,
                &mut raw,
            )
            .ok()?;
            assert!(
                !raw.is_null(),
                "UI Automation activation returned a null interface"
            );
            let client = IUIAutomation::from_raw(raw);
            let value = VARIANT {
                Anonymous: tagVARIANT_0 {
                    Anonymous: std::mem::ManuallyDrop::new(tagVARIANT_0_0 {
                        vt: VT_I4 as u16,
                        Anonymous: tagVARIANT_0_0_0 {
                            lVal: process as i32,
                        },
                        ..Default::default()
                    }),
                },
            };
            let condition = client.CreatePropertyCondition(PROCESS_ID, &value)?;
            Ok(Self {
                client,
                process,
                condition,
                title: format!("Reactor UI fixture: {fixture}"),
                window: Cell::new(None),
                cancelled,
                pressed,
                stderr,
            })
        }
    }

    fn ensure_active(&self) -> Result<()> {
        if self.cancelled.load(Ordering::Relaxed) {
            return Err("UI test cancelled".into());
        }
        Ok(())
    }

    fn roots(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            self.client
                .GetRootElement()?
                .FindAll(TreeScope_Children, &self.condition)
        }
    }

    fn find(
        &self,
        selector: &str,
        control_type: Option<i32>,
    ) -> windows_core::Result<Option<IUIAutomationElement>> {
        unsafe {
            let roots = self.roots()?;
            let all = self.client.CreateTrueCondition()?;
            let mut found = None;
            for root in 0..roots.Length()? {
                let root = roots.GetElement(root)?;
                let elements = root
                    .FindAll(TreeScope_Descendants, &all)
                    .inspect_err(|error| log_root(&root, error))?;
                for index in 0..elements.Length()? {
                    let element = elements.GetElement(index)?;
                    let name = if let Some(control_type) = control_type {
                        if element.CurrentControlType()? != control_type {
                            continue;
                        }
                        element.CurrentName()?
                    } else {
                        element.CurrentAutomationId()?
                    };
                    if name == selector && !element.CurrentIsOffscreen()?.as_bool() {
                        if let Some(previous) = &found {
                            assert!(
                                self.client.CompareElements(previous, &element)?.as_bool(),
                                "ambiguous UI selector: {selector}",
                            );
                        } else {
                            found = Some(element);
                        }
                    }
                }
            }
            Ok(found)
        }
    }

    pub(super) fn wait<T: std::fmt::Debug>(
        &self,
        description: &str,
        mut sample: impl FnMut() -> windows_core::Result<T>,
        ready: impl Fn(&T) -> bool,
    ) -> Result<T> {
        eprintln!("{description}");
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            self.ensure_active()?;
            let started = Instant::now();
            let observed = sample();
            self.ensure_active()?;
            match &observed {
                Ok(value) if ready(value) => return Ok(observed?),
                Ok(_) => {}
                Err(error) if transient(error) => {
                    if error.code() == UIA_TIMEOUT {
                        eprintln!(
                            "sample timed out after {} ms; repeating",
                            started.elapsed().as_millis()
                        );
                    }
                }
                Err(error) => {
                    eprintln!(
                        "sample failed after {} ms: {error}",
                        started.elapsed().as_millis()
                    );
                    return Err(format!("{description}: {error}").into());
                }
            }
            if Instant::now() >= deadline {
                if let Err(error) = self.dump() {
                    eprintln!("could not finish UI tree diagnostic: {error}");
                }
                return Err(format!("{description}: timed out; last observed {observed:?}").into());
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    pub fn element(&self, id: &str) -> Result<IUIAutomationElement> {
        Ok(self
            .wait(
                &format!("find automation ID {id:?}"),
                || self.find(id, None),
                Option::is_some,
            )?
            .unwrap())
    }

    /// Counts elements, including offscreen ones, whose automation ID starts with `prefix`.
    pub fn count(&self, prefix: &str) -> Result<usize> {
        unsafe {
            let roots = self.roots()?;
            let all = self.client.CreateTrueCondition()?;
            let mut count = 0;
            for root in 0..roots.Length()? {
                let elements = roots
                    .GetElement(root)?
                    .FindAll(TreeScope_Descendants, &all)?;
                for index in 0..elements.Length()? {
                    let id = elements.GetElement(index)?.CurrentAutomationId()?;
                    if String::from_utf16_lossy(&id).starts_with(prefix) {
                        count += 1;
                    }
                }
            }
            Ok(count)
        }
    }

    pub fn menu_item(&self, name: &str) -> Result<IUIAutomationElement> {
        self.named(MENU_ITEM, name)
    }

    /// Finds a visible element by control type and name in any fixture window, including popups.
    pub fn named(&self, control_type: i32, name: &str) -> Result<IUIAutomationElement> {
        Ok(self
            .wait(
                &format!("find control type {control_type} named {name:?}"),
                || self.find(name, Some(control_type)),
                Option::is_some,
            )?
            .unwrap())
    }

    pub fn list_item(&self, id: &str, name: &str) -> Result<IUIAutomationElement> {
        let list = self.element(id)?;
        Ok(self
            .wait(
                &format!("find list item {name:?} in {id:?}"),
                || unsafe {
                    let all = self.client.CreateTrueCondition()?;
                    let items = list.FindAll(TreeScope_Descendants, &all)?;
                    let mut found = None;
                    for index in 0..items.Length()? {
                        let item = items.GetElement(index)?;
                        if item.CurrentControlType()? != LIST_ITEM
                            || item.CurrentIsOffscreen()?.as_bool()
                        {
                            continue;
                        }
                        // DataItem text belongs to the container's child peer.
                        let content = item.FindAll(TreeScope_Descendants, &all)?;
                        let mut matches = false;
                        for index in 0..content.Length()? {
                            if content.GetElement(index)?.CurrentName()? == name {
                                matches = true;
                                break;
                            }
                        }
                        if matches {
                            if let Some(previous) = &found {
                                assert!(
                                    self.client.CompareElements(previous, &item)?.as_bool(),
                                    "ambiguous list item {name:?} in {id:?}"
                                );
                            } else {
                                found = Some(item);
                            }
                        }
                    }
                    Ok(found)
                },
                Option::is_some,
            )?
            .unwrap())
    }

    pub fn selected_list_item(
        &self,
        id: &str,
        expected: Option<&IUIAutomationElement>,
    ) -> Result<()> {
        let list = self.element(id)?;
        self.wait(
            &format!("list {id:?} selection becomes {expected:?}"),
            || unsafe {
                let items = list
                    .GetCurrentPatternAs::<IUIAutomationSelectionPattern>(SELECTION_PATTERN)?
                    .GetCurrentSelection()?;
                match expected {
                    Some(expected) => Ok(items.Length()? == 1
                        && self
                            .client
                            .CompareElements(&items.GetElement(0)?, expected)?
                            .as_bool()),
                    None => Ok(items.Length()? == 0),
                }
            },
            |matches| *matches,
        )?;
        Ok(())
    }

    /// Waits until the selection of `id` is one element named `name`, either directly or through
    /// a descendant such as a DataItem's text. Controls without a selection pattern, such as
    /// RadioButtons, are checked through the named descendant's selection item pattern.
    pub fn selected_name(&self, id: &str, name: &str) -> Result<()> {
        const SELECTION_ITEM_PATTERN: i32 = 10010;
        let control = self.element(id)?;
        self.wait(
            &format!("{id:?} selection becomes {name:?}"),
            || unsafe {
                let all = self.client.CreateTrueCondition()?;
                let named = |element: &IUIAutomationElement| -> windows_core::Result<bool> {
                    if element.CurrentName()? == name {
                        return Ok(true);
                    }
                    let content = element.FindAll(TreeScope_Descendants, &all)?;
                    for index in 0..content.Length()? {
                        if content.GetElement(index)?.CurrentName()? == name {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                };
                let Ok(selection) =
                    control.GetCurrentPatternAs::<IUIAutomationSelectionPattern>(SELECTION_PATTERN)
                else {
                    let items = control.FindAll(TreeScope_Descendants, &all)?;
                    let mut selected = Vec::new();
                    for index in 0..items.Length()? {
                        let item = items.GetElement(index)?;
                        if let Ok(pattern) = item
                            .GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(
                                SELECTION_ITEM_PATTERN,
                            )
                            && pattern.CurrentIsSelected()?.as_bool()
                        {
                            selected.push(item);
                        }
                    }
                    return Ok(selected.len() == 1 && named(&selected[0])?);
                };
                let items = selection.GetCurrentSelection()?;
                Ok(items.Length()? == 1 && named(&items.GetElement(0)?)?)
            },
            |matches| *matches,
        )?;
        Ok(())
    }

    pub fn menu_gone(&self, name: &str) -> Result<()> {
        self.wait(
            &format!("menu item {name:?} disappears"),
            || self.find(name, Some(MENU_ITEM)),
            Option::is_none,
        )?;
        Ok(())
    }

    pub fn gone(&self, id: &str) -> Result<()> {
        self.wait(
            &format!("automation ID {id:?} disappears"),
            || self.find(id, None),
            Option::is_none,
        )?;
        Ok(())
    }

    /// Waits until the element's bounds stop changing for `quiet`.
    pub fn settled(&self, id: &str, quiet: Duration) -> Result<()> {
        eprintln!("{id:?} bounds settle for {quiet:?}");
        let element = self.element(id)?;
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut last = unsafe { element.CurrentBoundingRectangle()? };
        let mut stable = Instant::now();
        loop {
            self.ensure_active()?;
            std::thread::sleep(Duration::from_millis(20));
            let bounds = unsafe { element.CurrentBoundingRectangle()? };
            if (bounds.left, bounds.top, bounds.right, bounds.bottom)
                != (last.left, last.top, last.right, last.bottom)
            {
                last = bounds;
                stable = Instant::now();
            } else if stable.elapsed() >= quiet {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(format!("{id:?} bounds did not settle").into());
            }
        }
    }

    pub fn invoke(&self, id: &str) -> Result<()> {
        eprintln!("invoke {id:?}");
        unsafe {
            self.element(id)?
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
                .Invoke()
                .ok()?;
        }
        Ok(())
    }

    pub fn removed_from_tree(&self, id: &str) -> Result<()> {
        let window = unsafe {
            self.client
                .ElementFromHandle(self.window_handle()? as *mut core::ffi::c_void)?
        };
        let all = unsafe { self.client.CreateTrueCondition()? };
        self.wait(
            &format!(
                "automation ID {id:?} is removed from the window tree, including offscreen elements"
            ),
            || unsafe {
                let elements = window.FindAll(TreeScope_Descendants, &all)?;
                for index in 0..elements.Length()? {
                    if elements.GetElement(index)?.CurrentAutomationId()? == id {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            |removed| *removed,
        )?;
        Ok(())
    }

    fn tray_window(&self) -> windows_core::Result<Option<HWND>> {
        let mut after = std::ptr::null_mut();
        let mut found = None;
        unsafe {
            loop {
                after = FindWindowExW(
                    std::ptr::null_mut(),
                    after,
                    windows_core::PCWSTR::null(),
                    windows_core::w!("windows-notifyicon"),
                );
                if after.is_null() {
                    return Ok(found);
                }
                let mut process = 0;
                GetWindowThreadProcessId(after, &mut process);
                if process == self.process {
                    assert!(found.is_none(), "multiple fixture notification windows");
                    found = Some(after);
                }
            }
        }
    }

    pub fn open_tray_menu(&self) -> Result<()> {
        self.tray_event(WM_CONTEXTMENU as u32)
    }

    pub fn activate_tray_icon(&self) -> Result<()> {
        self.tray_event(NIN_SELECT as u32)
    }

    pub fn remove_tray_registration(&self) -> Result<()> {
        let window = self
            .tray_window()?
            .ok_or("fixture icon window is missing")?;
        let mut data = NOTIFYICONDATAW {
            cbSize: size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: window,
            uID: 1,
            ..Default::default()
        };
        assert!(
            unsafe { Shell_NotifyIconW(NIM_DELETE as u32, &mut data) }.as_bool(),
            "failed to remove fixture's Shell registration"
        );
        Ok(())
    }

    pub fn recover_tray_registration(&self) -> Result<()> {
        let window = self
            .tray_window()?
            .ok_or("fixture icon window is missing")?;
        unsafe {
            let message = RegisterWindowMessageW(windows_core::w!("TaskbarCreated"));
            assert_ne!(message, 0);
            let mut result = 0;
            assert_ne!(
                SendMessageTimeoutW(
                    window,
                    message,
                    0,
                    0,
                    SMTO_ABORTIFHUNG as u32,
                    5000,
                    &mut result,
                ),
                0,
                "fixture did not queue Shell recovery"
            );
        }
        Ok(())
    }

    pub fn stderr(&self) -> Result<String> {
        Ok(std::fs::read_to_string(&self.stderr)?)
    }

    fn tray_event(&self, event: u32) -> Result<()> {
        eprintln!("send Shell notification {event:#x} to fixture icon");
        let hwnd = self
            .wait(
                "find fixture notification window",
                || self.tray_window(),
                Option::is_some,
            )?
            .unwrap();
        let bounds = unsafe {
            self.client
                .ElementFromHandle(self.window_handle()? as *mut core::ffi::c_void)?
                .CurrentBoundingRectangle()?
        };
        let x = bounds.left + 32;
        let y = bounds.top + 32;
        let point =
            usize::from(i16::try_from(x)? as u16) | (usize::from(i16::try_from(y)? as u16) << 16);
        self.ensure_active()?;
        unsafe {
            let mut process = 0;
            GetWindowThreadProcessId(hwnd, &mut process);
            assert_eq!(process, self.process, "notification window owner changed");
            PostMessageW(hwnd, WM_USER as u32 + 1, point, event as isize).ok()?;
        }
        Ok(())
    }

    pub fn tray_gone(&self) -> Result<()> {
        self.wait(
            "fixture notification window disappears",
            || self.tray_window(),
            Option::is_none,
        )?;
        Ok(())
    }

    pub fn name(&self, id: &str, expected: &str) -> Result<()> {
        self.wait(
            &format!("{id:?} name becomes {expected:?}"),
            || unsafe {
                self.find(id, None)?
                    .map(|element| element.CurrentName())
                    .transpose()
            },
            |name| name.as_ref().is_some_and(|name| name == expected),
        )?;
        Ok(())
    }

    pub fn name_stays(&self, id: &str, expected: &str, duration: Duration) -> Result<()> {
        eprintln!("{id:?} remains {expected:?} for {duration:?}");
        // Observe this peer, not unrelated peers retiring elsewhere in the window.
        let element = self.element(id)?;
        let deadline = Instant::now() + duration;
        loop {
            self.ensure_active()?;
            unsafe {
                assert_eq!(
                    element.CurrentAutomationId()?,
                    id,
                    "{id:?} lost its identity"
                );
                assert!(
                    !element.CurrentIsOffscreen()?.as_bool(),
                    "{id:?} disappeared"
                );
                assert_eq!(element.CurrentName()?, expected, "{id:?} changed");
            }
            if Instant::now() >= deadline {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    pub fn name_of(&self, element: &IUIAutomationElement, expected: &str) -> Result<()> {
        self.wait(
            &format!("retained element name becomes {expected:?}"),
            || unsafe { element.CurrentName() },
            |name| name == expected,
        )?;
        Ok(())
    }

    pub fn value(&self, id: &str, expected: &str) -> Result<()> {
        self.wait(
            &format!("{id:?} value becomes {expected:?}"),
            || unsafe {
                self.find(id, None)?
                    .map(|element| {
                        element
                            .GetCurrentPatternAs::<IUIAutomationValuePattern>(VALUE_PATTERN)?
                            .CurrentValue()
                    })
                    .transpose()
            },
            |value| value.as_ref().is_some_and(|value| value == expected),
        )?;
        Ok(())
    }

    pub fn text(&self, id: &str, expected: &str) -> Result<()> {
        self.wait(
            &format!("{id:?} text becomes {expected:?}"),
            || unsafe {
                self.find(id, None)?
                    .map(|element| {
                        element
                            .GetCurrentPatternAs::<IUIAutomationTextPattern>(TEXT_PATTERN)?
                            .DocumentRange()?
                            .GetText(-1)
                    })
                    .transpose()
            },
            |text| text.as_ref().is_some_and(|text| text == expected),
        )?;
        Ok(())
    }

    pub fn set_value(&self, id: &str, value: &str) -> Result<()> {
        eprintln!("set UIA value {id:?} to {value:?}");
        unsafe {
            self.element(id)?
                .GetCurrentPatternAs::<IUIAutomationValuePattern>(VALUE_PATTERN)?
                .SetValue(&value.into())
                .ok()?;
        }
        Ok(())
    }

    pub fn focus(&self, id: &str) -> Result<IUIAutomationElement> {
        eprintln!("focus {id:?}");
        let element = self.element(id)?;
        unsafe {
            let window = self.window_handle()? as HWND;
            if GetForegroundWindow() != window {
                assert!(
                    SetForegroundWindow(window).as_bool(),
                    "fixture could not acquire foreground; refusing input"
                );
            }
            element.SetFocus().ok()?;
        }
        self.focused(id)?;
        Ok(element)
    }

    pub fn focused(&self, id: &str) -> Result<()> {
        let element = self.element(id)?;
        let _ = self.wait(
            &format!("{id:?} has keyboard focus"),
            || unsafe {
                self.client
                    .CompareElements(&self.client.GetFocusedElement()?, &element)
            },
            |same| same.as_bool(),
        )?;
        Ok(())
    }

    fn pointer_ready(&self) -> Result<()> {
        self.ensure_active()?;
        unsafe {
            let mut owner = 0;
            GetWindowThreadProcessId(GetForegroundWindow(), &mut owner);
            if owner != self.process {
                return Err(format!(
                    "refusing pointer input: foreground PID {owner}, expected PID {}",
                    self.process
                )
                .into());
            }
            for key in [0x02, 0x04, 0x05, 0x06, 0x10, 0x11, 0x12, 0x5b, 0x5c] {
                if GetAsyncKeyState(key) & i16::MIN != 0 {
                    return Err("mouse button or modifier key is held".into());
                }
            }
        }
        Ok(())
    }

    pub fn pointer_move(&self, id: &str, x: f64, y: f64) -> Result<()> {
        self.pointer_move_to(&self.element(id)?, id, x, y)
    }

    /// Moves the pointer onto `id` until the fixture reports `expected` as the name of `observed`.
    /// WinUI drops pointer input for a short time after a window is first shown, with no signal
    /// for when input is accepted, so a move that the fixture does not observe is repeated. See
    /// microsoft/microsoft-ui-xaml#12753.
    pub fn pointer_enter(&self, id: &str, observed: &str, expected: &str) -> Result<()> {
        let element = self.element(id)?;
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut x = 0.5;
        loop {
            self.pointer_move_to(&element, id, x, 0.5)?;
            x = if x == 0.5 { 0.52 } else { 0.5 };
            let delivered = Instant::now() + Duration::from_millis(100);
            loop {
                let name = unsafe {
                    self.find(observed, None).and_then(|element| {
                        element.map(|element| element.CurrentName()).transpose()
                    })
                };
                match name {
                    Ok(Some(name)) if name == expected => return Ok(()),
                    Ok(_) => {}
                    Err(error) if transient(&error) => {}
                    Err(error) => return Err(error.into()),
                }
                if Instant::now() >= delivered {
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            if Instant::now() >= deadline {
                return Err(format!(
                    "pointer did not enter {id:?}: {observed:?} is not {expected:?}"
                )
                .into());
            }
            eprintln!("pointer move onto {id:?} was not delivered");
        }
    }

    pub fn pointer_move_to(
        &self,
        element: &IUIAutomationElement,
        id: &str,
        x: f64,
        y: f64,
    ) -> Result<()> {
        eprintln!("move pointer to {id:?}");
        let _coordinates = PhysicalCoordinates::new()?;
        let bounds = unsafe { element.CurrentBoundingRectangle()? };
        if bounds.right <= bounds.left || bounds.bottom <= bounds.top {
            return Err(format!("{id:?} has no pointer target area").into());
        }
        // Fractions outside 0..=1 target the surface around an accessible anchor.
        let x = f64::from(bounds.left) + f64::from(bounds.right - bounds.left) * x;
        let y = f64::from(bounds.top) + f64::from(bounds.bottom - bounds.top) * y;
        if !x.is_finite()
            || !y.is_finite()
            || x < f64::from(i32::MIN)
            || x > f64::from(i32::MAX)
            || y < f64::from(i32::MIN)
            || y > f64::from(i32::MAX)
        {
            return Err("invalid pointer position".into());
        }
        let point = POINT {
            x: x.round() as i32,
            y: y.round() as i32,
        };
        unsafe {
            let mut owner = 0;
            GetWindowThreadProcessId(WindowFromPoint(point), &mut owner);
            if owner != self.process {
                return Err(format!(
                    "pointer target {id:?} at ({}, {}) is covered by another process: \
                     owner PID {owner}, expected PID {}",
                    point.x, point.y, self.process
                )
                .into());
            }
            let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
            let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
            let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
            let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);
            if width < 2
                || height < 2
                || point.x < x
                || point.x >= x + width
                || point.y < y
                || point.y >= y + height
            {
                return Err("pointer target is outside the virtual desktop".into());
            }
            let pressed = self.pressed.lock().unwrap();
            self.pointer_ready()?;
            if !*pressed && GetAsyncKeyState(0x01) & i16::MIN != 0 {
                return Err("left mouse button is held by another input source".into());
            }
            let mut before = POINT::default();
            GetCursorPos(&mut before).ok()?;
            send_mouse(
                MOUSEEVENTF_MOVE
                    | MOUSEEVENTF_MOVE_NOCOALESCE
                    | MOUSEEVENTF_ABSOLUTE
                    | MOUSEEVENTF_VIRTUALDESK,
                ((f64::from(point.x - x) * 65535.0) / f64::from(width - 1)).round() as i32,
                ((f64::from(point.y - y) * 65535.0) / f64::from(height - 1)).round() as i32,
            )?;
            let mut after = POINT::default();
            GetCursorPos(&mut after).ok()?;
            eprintln!(
                "pointer target ({}, {}); cursor ({}, {}) -> ({}, {})",
                point.x, point.y, before.x, before.y, after.x, after.y
            );
            Ok(())
        }
    }

    fn pointer_over_process(&self) -> Result<()> {
        unsafe {
            let mut point = POINT::default();
            GetCursorPos(&mut point).ok()?;
            let mut owner = 0;
            GetWindowThreadProcessId(WindowFromPoint(point), &mut owner);
            if owner != self.process {
                return Err(format!(
                    "pointer at ({}, {}) is over another process; refusing button input: \
                     owner PID {owner}, expected PID {}",
                    point.x, point.y, self.process
                )
                .into());
            }
        }
        self.pointer_ready()
    }

    pub fn pointer_button(&self, down: bool) -> Result<()> {
        eprintln!("left mouse button {}", if down { "down" } else { "up" });
        let _coordinates = PhysicalCoordinates::new()?;
        let mut pressed = self.pressed.lock().unwrap();
        self.pointer_over_process()?;
        if *pressed == down {
            return Err("unbalanced pointer button action".into());
        }
        if down && unsafe { GetAsyncKeyState(0x01) } & i16::MIN != 0 {
            return Err("left mouse button is held by another input source".into());
        }
        send_mouse(
            if down {
                MOUSEEVENTF_LEFTDOWN
            } else {
                MOUSEEVENTF_LEFTUP
            },
            0,
            0,
        )?;
        *pressed = down;
        Ok(())
    }

    /// Clicks the left button `count` times without waiting for the UI thread in between.
    pub fn pointer_clicks(&self, count: usize) -> Result<()> {
        eprintln!("click left mouse button {count} times");
        let _coordinates = PhysicalCoordinates::new()?;
        let mut pressed = self.pressed.lock().unwrap();
        self.pointer_over_process()?;
        if *pressed || unsafe { GetAsyncKeyState(0x01) } & i16::MIN != 0 {
            return Err("left mouse button is already held".into());
        }
        for _ in 0..count {
            send_mouse(MOUSEEVENTF_LEFTDOWN, 0, 0)?;
            *pressed = true;
            send_mouse(MOUSEEVENTF_LEFTUP, 0, 0)?;
            *pressed = false;
        }
        Ok(())
    }

    pub fn type_text(&self, id: &str, text: &str) -> Result<()> {
        eprintln!("type Unicode input {text:?} into {id:?}");
        let element = self.focus(id)?;
        unsafe {
            for unit in text.encode_utf16() {
                self.ensure_active()?;
                assert!(
                    self.client
                        .CompareElements(&self.client.GetFocusedElement()?, &element)?
                        .as_bool(),
                    "keyboard focus changed; refusing input"
                );
                for key in [0x10, 0x11, 0x12, 0x5b, 0x5c] {
                    assert_eq!(GetAsyncKeyState(key) & i16::MIN, 0, "modifier key is held");
                }
                let mut inputs =
                    [KEYEVENTF_UNICODE, KEYEVENTF_UNICODE | KEYEVENTF_KEYUP].map(|flags| INPUT {
                        r#type: INPUT_KEYBOARD as u32,
                        Anonymous: INPUT_0 {
                            ki: KEYBDINPUT {
                                wScan: unit,
                                dwFlags: flags as u32,
                                ..Default::default()
                            },
                        },
                    });
                self.ensure_active()?;
                // UIA calls can block; check foreground after they return.
                let mut owner = 0;
                GetWindowThreadProcessId(GetForegroundWindow(), &mut owner);
                assert_eq!(owner, self.process, "refusing input to another process");
                assert_eq!(
                    SendInput(
                        inputs.len() as u32,
                        inputs.as_mut_ptr(),
                        size_of::<INPUT>() as i32
                    ),
                    inputs.len() as u32,
                    "keyboard input was not fully accepted"
                );
            }
        }
        Ok(())
    }

    pub fn press_keys(&self, id: &str, keys: &[i32]) -> Result<()> {
        self.press_keys_on(&self.focus(id)?, id, keys)
    }

    /// Presses keys into an element that has no automation ID, after focusing it.
    pub fn press_keys_on(
        &self,
        element: &IUIAutomationElement,
        id: &str,
        keys: &[i32],
    ) -> Result<()> {
        eprintln!("press virtual keys {keys:x?} into {id:?}");
        assert!(!keys.is_empty());
        let input = |key: i32, flags| INPUT {
            r#type: INPUT_KEYBOARD as u32,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: key.try_into().unwrap(),
                    dwFlags: flags,
                    ..Default::default()
                },
            },
        };
        let mut inputs = keys
            .iter()
            .map(|&key| input(key, 0))
            .chain(
                keys.iter()
                    .rev()
                    .map(|&key| input(key, KEYEVENTF_KEYUP as u32)),
            )
            .collect::<Vec<_>>();
        unsafe {
            assert!(
                self.client
                    .CompareElements(&self.client.GetFocusedElement()?, element)?
                    .as_bool(),
                "keyboard focus changed; refusing input"
            );
            for key in keys.iter().copied().chain([0x10, 0x11, 0x12, 0x5b, 0x5c]) {
                assert_eq!(GetAsyncKeyState(key) & i16::MIN, 0, "key is already held");
            }
            self.ensure_active()?;
            let mut owner = 0;
            GetWindowThreadProcessId(GetForegroundWindow(), &mut owner);
            assert_eq!(owner, self.process, "refusing input to another process");
            let inserted = SendInput(
                inputs.len() as u32,
                inputs.as_mut_ptr(),
                size_of::<INPUT>() as i32,
            ) as usize;
            if inserted != inputs.len() {
                // The accepted prefix contains key-downs followed by reversed key-ups.
                let held = inserted.min(inputs.len() - inserted);
                let mut releases = keys[..held]
                    .iter()
                    .rev()
                    .map(|&key| input(key, KEYEVENTF_KEYUP as u32))
                    .collect::<Vec<_>>();
                let released = if releases.is_empty() {
                    0
                } else {
                    SendInput(
                        releases.len() as u32,
                        releases.as_mut_ptr(),
                        size_of::<INPUT>() as i32,
                    )
                };
                return Err(format!(
                    "keyboard input accepted {inserted}/{} events; released {released}/{held} held keys",
                    inputs.len()
                )
                .into());
            }
        }
        Ok(())
    }

    pub fn window_handle(&self) -> Result<usize> {
        self.ensure_active()?;
        let window = if let Some(window) = self.window.get() {
            window
        } else {
            let window = self
                .wait(
                    &format!("find window {:?}", self.title),
                    || unsafe {
                        let roots = self.roots()?;
                        for index in 0..roots.Length()? {
                            let element = roots.GetElement(index)?;
                            if element.CurrentName()? == self.title {
                                return Ok(Some(element.CurrentNativeWindowHandle()? as usize));
                            }
                        }
                        Ok(None)
                    },
                    Option::is_some,
                )?
                .unwrap();
            self.window.set(Some(window));
            window
        };
        unsafe {
            let mut owner = 0;
            GetWindowThreadProcessId(window as HWND, &mut owner);
            assert_eq!(owner, self.process, "fixture window owner changed");
        }
        Ok(window)
    }

    pub fn window_title(&self, expected: &str) -> Result<()> {
        let window = self.window_handle()?;
        self.wait(
            &format!("window title becomes {expected:?}"),
            || unsafe {
                self.client
                    .ElementFromHandle(window as *mut core::ffi::c_void)?
                    .CurrentName()
            },
            |title| title == expected,
        )?;
        Ok(())
    }

    pub fn window_size(&self, width: f64, height: f64) -> Result<()> {
        let _physical = PhysicalCoordinates::new()?;
        let window = self.window_handle()? as HWND;
        self.wait(
            &format!("native client size and observer become {width} x {height} DIPs"),
            || unsafe {
                let mut rect = RECT::default();
                GetClientRect(window, &mut rect).ok()?;
                let dpi = GetDpiForWindow(window);
                if dpi == 0 {
                    return Err(windows_core::Error::new(E_UNEXPECTED, "window DPI is zero"));
                }
                let actual = (rect.right - rect.left, rect.bottom - rect.top);
                let pixels = |dips: f64| (dips * f64::from(dpi) / 96.0).round() as i32;
                let expected = (pixels(width), pixels(height));
                let observed = self
                    .find("size", None)?
                    .map(|element| element.CurrentName())
                    .transpose()?;
                let expected_observed = format!(
                    "Size {:.3} x {:.3}",
                    f64::from(actual.0) * 96.0 / f64::from(dpi),
                    f64::from(actual.1) * 96.0 / f64::from(dpi),
                );
                Ok((actual, expected, observed, expected_observed))
            },
            |(actual, expected, observed, expected_observed)| {
                actual == expected
                    && observed
                        .as_ref()
                        .is_some_and(|name| name == expected_observed.as_str())
            },
        )?;
        Ok(())
    }

    pub fn window_bounds(&self, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        let _physical = PhysicalCoordinates::new()?;
        let window = self.window_handle()? as HWND;
        self.wait(
            &format!("outer window bounds become {x} {y}; {width} x {height} physical pixels"),
            || unsafe {
                let mut rect = RECT::default();
                GetWindowRect(window, &mut rect).ok()?;
                Ok((
                    rect.left,
                    rect.top,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                ))
            },
            |actual| *actual == (x, y, width, height),
        )?;
        Ok(())
    }

    pub fn move_resize_window(&self, x: i32, y: i32, width: i32, height: i32) -> Result<()> {
        eprintln!("move/resize fixture window to {x} {y}; {width} x {height} physical pixels");
        let _physical = PhysicalCoordinates::new()?;
        let window = self.window_handle()? as HWND;
        unsafe {
            SetWindowPos(
                window,
                std::ptr::null_mut(),
                x,
                y,
                width,
                height,
                (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
            )
            .ok()?;
        }
        Ok(())
    }

    pub fn window_icon(&self, present: bool) -> Result<()> {
        let window = self.window_handle()? as HWND;
        self.wait(
            &format!("explicit window icons present: {present}"),
            || unsafe {
                let mut icons = [false; 2];
                for (index, kind) in [ICON_BIG, ICON_SMALL].into_iter().enumerate() {
                    let mut icon = 0;
                    if SendMessageTimeoutW(
                        window,
                        WM_GETICON as u32,
                        kind as usize,
                        0,
                        (SMTO_ABORTIFHUNG | SMTO_BLOCK) as u32,
                        1000,
                        &mut icon,
                    ) == 0
                    {
                        return Err(windows_core::Error::from_thread());
                    }
                    icons[index] = icon != 0;
                }
                Ok(icons)
            },
            |icons| *icons == [present; 2],
        )?;
        Ok(())
    }

    fn dump(&self) -> Result<()> {
        unsafe {
            let roots = self.roots()?;
            let all = self.client.CreateTrueCondition()?;
            for index in 0..roots.Length()? {
                let elements = roots.GetElement(index)?.FindAll(TreeScope_Subtree, &all)?;
                for index in 0..elements.Length()?.min(80) {
                    let element = elements.GetElement(index)?;
                    eprintln!(
                        "UI: type={} id={:?} name={:?} offscreen={}",
                        element.CurrentControlType()?,
                        element.CurrentAutomationId()?,
                        element.CurrentName()?,
                        element.CurrentIsOffscreen()?.as_bool(),
                    );
                }
            }
        }
        Ok(())
    }
}

// A search that overlaps a change to the UI tree can fail with ELEMENT_UNAVAILABLE. WinUI can
// instead fail it with UIA_E_TIMEOUT within about 100 ms, for example while a popup window closes
// or while a fixture starts; a WinForms popup only produced ELEMENT_UNAVAILABLE. Both mean the
// tree changed during the search, so the sample is repeated until the caller's deadline.
fn transient(error: &windows_core::Error) -> bool {
    error.code() == ELEMENT_UNAVAILABLE || error.code() == UIA_TIMEOUT
}

// Identifies the top-level window whose subtree failed so it can be matched with the window probe.
fn log_root(root: &IUIAutomationElement, error: &windows_core::Error) {
    if transient(error) {
        return;
    }
    let window = unsafe { root.CurrentNativeWindowHandle() };
    let name = unsafe { root.CurrentName() };
    eprintln!("root {window:?} {name:?} failed: {error}");
}
