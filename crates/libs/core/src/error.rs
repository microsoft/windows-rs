#![allow(missing_docs)]

use super::*;

/// An error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq, Eq)]
pub struct Error {
    pub(crate) code: HRESULT,
    pub(crate) info: Option<crate::imp::IErrorInfo>,
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl Error {
    /// An error object without any failure information.
    pub const OK: Self = Self { code: HRESULT(0), info: None };

    /// This creates a new error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: HRESULT, message: HSTRING) -> Self {
        unsafe {
            crate::imp::RoOriginateError(code.0, std::mem::transmute_copy(&message));
            Self { code, info: GetErrorInfo() }
        }
    }

    /// Creates a new `Error` from the Win32 error code returned by `GetLastError()`.
    pub fn from_win32() -> Self {
        unsafe { Self { code: HRESULT::from_win32(crate::imp::GetLastError()), info: None } }
    }

    /// The error code describing the error.
    pub const fn code(&self) -> HRESULT {
        self.code
    }

    /// The error object describing the error.
    pub fn info<T: Interface>(&self) -> Option<T> {
        self.info.as_ref().and_then(|info| info.cast::<T>().ok())
    }

    /// The error message describing the error.
    pub fn message(&self) -> HSTRING {
        if let Some(info) = &self.info {
            let mut message = BSTR::default();

            // First attempt to retrieve the restricted error information.
            if let Ok(info) = info.cast::<crate::imp::IRestrictedErrorInfo>() {
                let mut fallback = BSTR::default();
                let mut code = HRESULT(0);

                unsafe {
                    // The vfptr is called directly to avoid the default error propagation logic.
                    _ = (info.vtable().GetErrorDetails)(info.as_raw(), &mut fallback as *mut _ as _, &mut code, &mut message as *mut _ as _, &mut BSTR::default() as *mut _ as _);
                }

                if message.is_empty() {
                    message = fallback
                };
            }

            // Next attempt to retrieve the regular error information.
            if message.is_empty() {
                unsafe {
                    // The vfptr is called directly to avoid the default error propagation logic.
                    _ = (info.vtable().GetDescription)(info.as_raw(), &mut message as *mut _ as _);
                }
            }

            return HSTRING::from_wide(crate::imp::wide_trim_end(message.as_wide())).unwrap_or_default();
        }

        // Otherwise fallback to a generic error code description.
        self.code.message()
    }
}

impl From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        if error.info.is_some() {
            unsafe {
                crate::imp::SetErrorInfo(0, std::mem::transmute_copy(&error.info));
            }
        }

        error.code
    }
}

impl From<HRESULT> for Error {
    fn from(code: HRESULT) -> Self {
        let info = GetErrorInfo();

        // Call CapturePropagationContext here if a use case presents itself. Otherwise, we can avoid the overhead for error propagation.

        Self { code, info }
    }
}

impl From<Error> for std::io::Error {
    fn from(from: Error) -> Self {
        Self::from_raw_os_error(from.code.0)
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(_: std::string::FromUtf16Error) -> Self {
        Self { code: HRESULT::from_win32(crate::imp::ERROR_NO_UNICODE_TRANSLATION), info: None }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self { code: HRESULT::from_win32(crate::imp::ERROR_NO_UNICODE_TRANSLATION), info: None }
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self { code: crate::imp::E_INVALIDARG, info: None }
    }
}

// Unfortunately this is needed to make types line up. The Rust type system does
// not know the `Infallible` can never be constructed. This code needs to be here
// to satesify the type checker but it will never be run. Once `!` is stabilizied
// this can be removed.
impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug.field("code", &self.code).field("message", &self.message()).finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.message();
        if message.is_empty() {
            std::write!(fmt, "{}", self.code())
        } else {
            std::write!(fmt, "{} ({})", self.message(), self.code())
        }
    }
}

impl std::error::Error for Error {}

fn GetErrorInfo() -> Option<crate::imp::IErrorInfo> {
    let mut info = None;
    unsafe { crate::imp::GetErrorInfo(0, &mut info as *mut _ as _) };
    info
}
