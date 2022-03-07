use super::*;
use bindings::*;

/// An error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq)]
pub struct Error {
    pub(crate) code: HRESULT,
    pub(crate) info: Option<IRestrictedErrorInfo>,
}

impl Error {
    /// An error object without any failure information.
    pub const OK: Self = Self { code: S_OK, info: None };

    /// This creates a new WinRT error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: HRESULT, message: HSTRING) -> Self {
        unsafe {
            let function = delay_load(b"combase.dll\0", b"RoOriginateError\0");

            if !function.is_null() {
                let function: RoOriginateError = core::mem::transmute(function);
                function(code, core::mem::transmute_copy(&message));
            }

            let info = GetErrorInfo(0).and_then(|e| e.cast()).ok();

            Self { code, info }
        }
    }

    pub fn from_win32() -> Self {
        unsafe { Self { code: GetLastError().into(), info: None } }
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
    #[cfg(feature = "Win32_Foundation")]
    pub fn win32_error(&self) -> Option<crate::Win32::Foundation::WIN32_ERROR> {
        let hresult = self.code.0 as u32;
        if ((hresult >> 16) & 0x7FF) == 7 {
            Some(crate::Win32::Foundation::WIN32_ERROR(hresult & 0xFFFF))
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
            Self::new(code, HSTRING::from_wide(message.as_wide()))
        } else {
            Self { code, info: None }
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug.field("code", &format_args!("{:#010X}", self.code.0)).field("message", &self.message()).finish()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(fmt, "{}", self.message())
    }
}

impl std::error::Error for Error {}

type RoOriginateError = extern "system" fn(code: HRESULT, message: core::mem::ManuallyDrop<HSTRING>) -> BOOL;
