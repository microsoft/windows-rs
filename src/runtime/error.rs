use super::*;
use core::convert::TryInto;

use bindings::{
    Windows::Win32::Foundation::{GetLastError, BOOL, BSTR, S_OK},
    Windows::Win32::System::Ole::Automation::{GetErrorInfo, SetErrorInfo},
    Windows::Win32::System::WinRT::{ILanguageExceptionErrorInfo2, IRestrictedErrorInfo},
};

/// A WinRT error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq)]
pub struct Error {
    code: HRESULT,
    info: Option<IRestrictedErrorInfo>,
}

impl Error {
    /// An error object without any failure information.
    pub const OK: Self = Self { code: S_OK, info: None };

    /// This creates a new WinRT error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: HRESULT, message: &str) -> Self {
        let message: HSTRING = message.into();

        // RoOriginateError creates the error object and associates it with the thread.
        // Need to ignore the result, as that is the delay-load error, which would mean
        // that there's no WinRT to tell about the error.
        unsafe {
            let _ = RoOriginateError(code, core::mem::transmute_copy(&message));
        }

        let info = unsafe { GetErrorInfo(0).and_then(|e| e.cast()).ok() };

        // The error information is then associated with the returning error object and no longer
        // associated with the thread.
        Self { code, info }
    }

    #[doc(hidden)]
    pub const fn fast_error(code: HRESULT) -> Self {
        Self { code, info: None }
    }

    pub fn from_win32() -> Self {
        unsafe { Self::fast_error(GetLastError().into()) }
    }

    /// The error code describing the error.
    pub const fn code(&self) -> HRESULT {
        self.code
    }

    /// The error information describing the error.
    pub const fn info(&self) -> &Option<IRestrictedErrorInfo> {
        &self.info
    }

    /// The error message describing the error.
    pub fn message(&self) -> HSTRING {
        // First attempt to retrieve the restricted error information.
        if let Some(info) = &self.info {
            let mut fallback = BSTR::default();
            let mut message = BSTR::default();
            let mut unused = BSTR::default();
            let mut code = HRESULT(0);

            unsafe {
                let _ = info.GetErrorDetails(&mut fallback, &mut code, &mut message, &mut unused);
            }

            if self.code == code {
                let message = if !message.is_empty() { message } else { fallback };
                return HSTRING::from_wide(message.as_wide());
            }
        }

        self.code.message()
    }

    /// Returns the win32 error code if the underlying HRESULT's facility is win32
    pub fn win32_error(&self) -> Option<u32> {
        let hresult = self.code.0;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(hresult & 0xFFFF)
        } else {
            None
        }
    }
}

impl core::convert::From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        let code = error.code;
        let info = error.info.and_then(|info| info.cast().ok());

        unsafe {
            let _ = SetErrorInfo(0, info);
        }

        code
    }
}

#[cfg(not(feature = "no_std"))]
impl core::convert::From<Error> for std::io::Error {
    fn from(from: Error) -> Self {
        Self::from_raw_os_error((from.code.0 & 0xFFFF) as _)
    }
}

impl core::convert::From<HRESULT> for Error {
    fn from(code: HRESULT) -> Self {
        let info: Option<IRestrictedErrorInfo> = unsafe { GetErrorInfo(0).and_then(|e| e.cast()).ok() };

        if let Some(info) = info {
            // If it does (and therefore running on a recent version of Windows)
            // then capture_propagation_context adds a breadcrumb to the error
            // info to make debugging easier.
            if let Ok(capture) = info.cast::<ILanguageExceptionErrorInfo2>() {
                unsafe {
                    let _ = capture.CapturePropagationContext(None);
                }
            }

            return Self { code, info: Some(info) };
        }

        if let Ok(info) = unsafe { GetErrorInfo(0) } {
            let message = unsafe { info.GetDescription().unwrap_or_default() };
            let message: HSTRING = message.try_into().unwrap_or_default();
            Self::new(code, &message)
        } else {
            Self { code, info: None }
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug.field("code", &format_args!("{:#010X}", self.code.0)).field("message", &self.message());
        if let Some(win32) = self.win32_error() {
            debug.field("win32_error", &format_args!("{}", win32));
        }
        debug.finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(fmt, "{}", self.message())
    }
}

#[cfg(not(feature = "no_std"))]
impl std::error::Error for Error {}

demand_load! {
    "combase.dll" {
        fn RoOriginateError(code: HRESULT, message: core::mem::ManuallyDrop<HSTRING>) -> BOOL;
    }
}
