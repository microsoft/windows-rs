use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use windows::{
    Win32::*,
    core::{BSTR, Error},
};

#[derive(Debug)]
pub enum AutomationError {
    InvalidUtf16(std::string::FromUtf16Error),
    Windows(Error),
    Timeout(String),
}

impl fmt::Display for AutomationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidUtf16(error) => error.fmt(formatter),
            Self::Windows(error) => error.fmt(formatter),
            Self::Timeout(description) => {
                write!(formatter, "timed out waiting for {description}")
            }
        }
    }
}

impl std::error::Error for AutomationError {}

impl From<Error> for AutomationError {
    fn from(error: Error) -> Self {
        Self::Windows(error)
    }
}

impl From<std::string::FromUtf16Error> for AutomationError {
    fn from(error: std::string::FromUtf16Error) -> Self {
        Self::InvalidUtf16(error)
    }
}

pub struct Automation {
    automation: IUIAutomation,
    _not_send: PhantomData<Rc<()>>,
}

impl Automation {
    pub fn new() -> Result<Self, AutomationError> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED as u32).ok()?;
            let automation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL as u32)?;
            Ok(Self {
                automation,
                _not_send: PhantomData,
            })
        }
    }

    pub fn wait_for_window(
        &self,
        process_id: u32,
        name: &str,
        timeout: Duration,
    ) -> Result<Element, AutomationError> {
        let root = unsafe { self.automation.GetRootElement()? };
        self.wait_for_element(
            &root,
            TreeScope_Children,
            timeout,
            &format!("window {name:?}"),
            |element| {
                Ok(unsafe { element.CurrentProcessId()? } == process_id as i32
                    && unsafe { element.CurrentName()? } == name)
            },
        )
    }

    pub fn wait_for_descendant_name(
        &self,
        root: &Element,
        name: &str,
        timeout: Duration,
    ) -> Result<Element, AutomationError> {
        self.wait_for_element(
            &root.0,
            TreeScope_Descendants,
            timeout,
            &format!("element {name:?}"),
            |element| Ok(unsafe { element.CurrentName()? } == name),
        )
    }

    fn wait_for_element(
        &self,
        root: &IUIAutomationElement,
        scope: TreeScope,
        timeout: Duration,
        description: &str,
        matches: impl Fn(&IUIAutomationElement) -> Result<bool, Error>,
    ) -> Result<Element, AutomationError> {
        let condition = unsafe { self.automation.CreateTrueCondition()? };
        let deadline = Instant::now() + timeout;

        loop {
            let elements = unsafe { root.FindAll(scope, &condition)? };
            let length = unsafe { elements.Length()? };
            for index in 0..length {
                let element = unsafe { elements.GetElement(index)? };
                if matches(&element)? {
                    return Ok(Element(element));
                }
            }
            if Instant::now() >= deadline {
                return Err(AutomationError::Timeout(description.to_string()));
            }
            thread::sleep(Duration::from_millis(50));
        }
    }
}

impl Drop for Automation {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

pub struct Element(IUIAutomationElement);

impl Element {
    pub fn name(&self) -> Result<String, AutomationError> {
        Ok(String::try_from(unsafe { self.0.CurrentName()? })?)
    }

    pub fn close_window(&self) -> Result<(), AutomationError> {
        let handle = unsafe { self.0.CurrentNativeWindowHandle()? };
        unsafe {
            PostMessageW(Some(HWND(handle.0)), WM_CLOSE as u32, WPARAM(0), LPARAM(0)).ok()?;
        }
        Ok(())
    }

    pub fn invoke(&self) -> Result<(), AutomationError> {
        let pattern: IUIAutomationInvokePattern =
            unsafe { self.0.GetCurrentPatternAs(PATTERNID(10_000))? };
        unsafe { pattern.Invoke().ok()? };
        Ok(())
    }

    pub fn set_value(&self, value: &str) -> Result<(), AutomationError> {
        let pattern: IUIAutomationValuePattern =
            unsafe { self.0.GetCurrentPatternAs(PATTERNID(10_002))? };
        unsafe { pattern.SetValue(&BSTR::from(value)).ok()? };
        Ok(())
    }

    pub fn select(&self) -> Result<(), AutomationError> {
        let pattern: IUIAutomationSelectionItemPattern =
            unsafe { self.0.GetCurrentPatternAs(PATTERNID(10_010))? };
        unsafe { pattern.Select().ok()? };
        Ok(())
    }
}
