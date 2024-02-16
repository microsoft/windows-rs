use super::*;

/// An error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq, Eq)]
pub struct Error {
    code: HRESULT,
    info: Option<ComPtr>,
}

impl Error {
    /// Creates an error object without any failure information.
    pub const fn empty() -> Self {
        Self {
            code: HRESULT(0),
            info: None,
        }
    }

    /// Creates a new error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new<T: AsRef<str>>(code: HRESULT, message: T) -> Self {
        let message: Vec<_> = message.as_ref().encode_utf16().collect();

        if message.is_empty() {
            Self::from_hresult(code)
        } else {
            unsafe {
                RoOriginateErrorW(code.0, message.len() as u32, message.as_ptr());
            }
            code.into()
        }
    }

    /// Creates a new error object with an error code, but without additional error information.
    pub fn from_hresult(code: HRESULT) -> Self {
        Self { code, info: None }
    }

    /// Creates a new `Error` from the Win32 error code returned by `GetLastError()`.
    pub fn from_win32() -> Self {
        Self {
            code: HRESULT::from_win32(unsafe { GetLastError() }),
            info: None,
        }
    }

    /// The error code describing the error.
    pub const fn code(&self) -> HRESULT {
        self.code
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        if let Some(info) = &self.info {
            let mut message = BasicString::default();

            // First attempt to retrieve the restricted error information.
            if let Some(info) = info.cast(&IID_IRestrictedErrorInfo) {
                let mut fallback = BasicString::default();
                let mut code = 0;

                unsafe {
                    com_call!(
                        IRestrictedErrorInfo_Vtbl,
                        info.GetErrorDetails(
                            &mut fallback as *mut _ as _,
                            &mut code,
                            &mut message as *mut _ as _,
                            &mut BasicString::default() as *mut _ as _
                        )
                    );
                }

                if message.is_empty() {
                    message = fallback
                };
            }

            // Next attempt to retrieve the regular error information.
            if message.is_empty() {
                unsafe {
                    com_call!(
                        IErrorInfo_Vtbl,
                        info.GetDescription(&mut message as *mut _ as _)
                    );
                }
            }

            return String::from_utf16_lossy(wide_trim_end(message.as_wide()));
        }

        // Otherwise fallback to a generic error code description.
        self.code.message()
    }

    /// The error object describing the error.
    pub fn as_ptr(&self) -> *mut std::ffi::c_void {
        self.info
            .as_ref()
            .map_or(std::ptr::null_mut(), |info| info.as_raw())
    }
}

impl std::error::Error for Error {}
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        if let Some(info) = error.info {
            unsafe {
                SetErrorInfo(0, info.as_raw());
            }
        }
        error.code
    }
}

impl From<HRESULT> for Error {
    fn from(code: HRESULT) -> Self {
        let mut info = None;
        unsafe { GetErrorInfo(0, &mut info as *mut _ as _) };
        Self { code, info }
    }
}

impl From<Error> for std::io::Error {
    fn from(from: Error) -> Self {
        Self::from_raw_os_error(from.code.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(from: std::io::Error) -> Self {
        match from.raw_os_error() {
            Some(status) => HRESULT::from_win32(status as u32).into(),
            None => HRESULT(E_UNEXPECTED).into(),
        }
    }
}

impl From<std::string::FromUtf16Error> for Error {
    fn from(_: std::string::FromUtf16Error) -> Self {
        Self {
            code: HRESULT::from_win32(ERROR_NO_UNICODE_TRANSLATION),
            info: None,
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self {
            code: HRESULT::from_win32(ERROR_NO_UNICODE_TRANSLATION),
            info: None,
        }
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self {
            code: HRESULT::from_win32(ERROR_INVALID_DATA),
            info: None,
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug
            .field("code", &self.code)
            .field("message", &self.message())
            .finish()
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
