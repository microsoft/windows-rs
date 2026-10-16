use super::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_core::{HRESULT, Interface};

pub const ELEMENT_UNAVAILABLE: HRESULT = HRESULT(0x80040201u32 as i32);
pub const INVOKE_PATTERN: i32 = 10000;
const VALUE_PATTERN: i32 = 10002;
const PROCESS_ID: i32 = 30002;
const MENU_ITEM: i32 = 50011;

pub struct Com;

impl Com {
    pub fn new() -> Result<Self> {
        unsafe { CoInitializeEx(std::ptr::null(), COINIT_MULTITHREADED as u32).ok()? };
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
    cancelled: Arc<AtomicBool>,
}

impl Automation {
    pub fn new(process: u32, fixture: &str, cancelled: Arc<AtomicBool>) -> Result<Self> {
        unsafe {
            let mut raw = std::ptr::null_mut();
            CoCreateInstance(
                &CUIAutomation,
                std::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
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
                Anonymous: VARIANT_0 {
                    Anonymous: std::mem::ManuallyDrop::new(VARIANT_0_0 {
                        vt: VT_I4 as u16,
                        Anonymous: VARIANT_0_0_0 {
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
                cancelled,
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
        menu: bool,
    ) -> windows_core::Result<Option<IUIAutomationElement>> {
        unsafe {
            let roots = self.roots()?;
            let all = self.client.CreateTrueCondition()?;
            let mut found = None;
            for root in 0..roots.Length()? {
                let elements = roots
                    .GetElement(root)?
                    .FindAll(TreeScope_Descendants, &all)?;
                for index in 0..elements.Length()? {
                    let element = elements.GetElement(index)?;
                    let name = if menu {
                        if element.CurrentControlType()? != MENU_ITEM {
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

    fn wait<T: std::fmt::Debug>(
        &self,
        description: &str,
        mut sample: impl FnMut() -> windows_core::Result<T>,
        ready: impl Fn(&T) -> bool,
    ) -> Result<T> {
        eprintln!("{description}");
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            self.ensure_active()?;
            let observed = sample();
            self.ensure_active()?;
            match &observed {
                Ok(value) if ready(value) => return Ok(observed?),
                Ok(_) => {}
                Err(error) if error.code() == ELEMENT_UNAVAILABLE => {}
                Err(error) => return Err(format!("{description}: {error}").into()),
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
                || self.find(id, false),
                Option::is_some,
            )?
            .unwrap())
    }

    pub fn menu_item(&self, name: &str) -> Result<IUIAutomationElement> {
        Ok(self
            .wait(
                &format!("find menu item {name:?}"),
                || self.find(name, true),
                Option::is_some,
            )?
            .unwrap())
    }

    pub fn menu_gone(&self, name: &str) -> Result<()> {
        self.wait(
            &format!("menu item {name:?} disappears"),
            || self.find(name, true),
            Option::is_none,
        )?;
        Ok(())
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

    pub fn name(&self, id: &str, expected: &str) -> Result<()> {
        self.wait(
            &format!("{id:?} name becomes {expected:?}"),
            || unsafe {
                self.find(id, false)?
                    .map(|element| element.CurrentName())
                    .transpose()
            },
            |name| name.as_ref().is_some_and(|name| name == expected),
        )?;
        Ok(())
    }

    pub fn name_stays(&self, id: &str, expected: &str, duration: Duration) -> Result<()> {
        eprintln!("{id:?} remains {expected:?} for {duration:?}");
        let deadline = Instant::now() + duration;
        loop {
            self.ensure_active()?;
            let element = self
                .find(id, false)?
                .ok_or_else(|| format!("{id:?} disappeared"))?;
            assert_eq!(
                unsafe { element.CurrentName()? },
                expected,
                "{id:?} changed"
            );
            if Instant::now() >= deadline {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    pub fn value(&self, id: &str, expected: &str) -> Result<()> {
        self.wait(
            &format!("{id:?} value becomes {expected:?}"),
            || unsafe {
                self.find(id, false)?
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

    pub fn type_text(&self, id: &str, text: &str) -> Result<()> {
        eprintln!("type Unicode input {text:?} into {id:?}");
        let element = self.element(id)?;
        unsafe {
            let window = self.window_handle()? as HWND;
            if GetForegroundWindow() != window {
                assert!(
                    SetForegroundWindow(window).as_bool(),
                    "fixture could not acquire foreground; refusing keyboard input"
                );
            }
            element.SetFocus().ok()?;
            let _ = self.wait(
                "input receives keyboard focus",
                || {
                    self.client
                        .CompareElements(&self.client.GetFocusedElement()?, &element)
                },
                |same| same.as_bool(),
            )?;
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

    pub fn window_handle(&self) -> Result<usize> {
        self.wait(
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
        )
        .map(Option::unwrap)
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
