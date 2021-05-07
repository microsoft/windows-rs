use crate::*;
use std::convert::TryInto;

use bindings::{
    Windows::Win32::System::OleAutomation::{GetErrorInfo, SetErrorInfo, BSTR},
    Windows::Win32::System::WinRT::{ILanguageExceptionErrorInfo2, IRestrictedErrorInfo},
};

/// A WinRT error object consists of both an error code as well as detailed error information for debugging.
#[derive(Clone, PartialEq)]
pub struct Error {
    code: HRESULT,
    info: Option<IRestrictedErrorInfo>,
}

impl Error {
    /// This creates a new WinRT error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: HRESULT, message: &str) -> Self {
        let message: HSTRING = message.into();

        // RoOriginateError creates the error object and associates it with the thread.
        // Need to ignore the result, as that is the delay-load error, which would mean
        // that there's no WinRT to tell about the error.
        unsafe {
            let _ = RoOriginateError(code, message.abi() as _);
        }

        let mut info = None;
        let info = unsafe {
            GetErrorInfo(0, &mut info)
                .and_some(info)
                .and_then(|e| e.cast())
                .ok()
        };

        // The error information is then associated with the returning error object and no longer
        // associated with the thread.
        Self { code, info }
    }

    // Use internally to create an Error object without error info. Typically used with QueryInterface
    // (E_NOINTERFACE) or the absense of an object to return (E_POINTER) to avoid the code gen overhead
    // for casts that should be cheap (few instructions). Think of it as a way to create recoverable errors
    // that don't need th overhead of debugging origination info.
    pub fn fast_error(code: HRESULT) -> Self {
        Self { code, info: None }
    }

    /// The error code describing the error.
    pub fn code(&self) -> HRESULT {
        self.code
    }

    /// The error information describing the error.
    pub fn info(&self) -> &Option<IRestrictedErrorInfo> {
        &self.info
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        // First attempt to retrieve the restricted error information.
        if let Some(info) = &self.info {
            let mut fallback = BSTR::default();
            let mut message = BSTR::default();
            let mut unused = BSTR::default();
            let mut code = HRESULT(0);

            unsafe {
                let _ = info.GetErrorDetails(&mut fallback, &mut code, &mut message, &mut unused);
            }

            let message = if !message.is_empty() {
                message
            } else {
                fallback
            };

            let message: String = message.try_into().unwrap_or_default();

            if self.code == code {
                return message.trim_end().to_owned();
            }
        }

        self.code.message()
    }
}

impl std::convert::From<Error> for HRESULT {
    fn from(error: Error) -> Self {
        let code = error.code;
        let info = error.info.and_then(|info| info.cast().ok());

        unsafe {
            let _ = SetErrorInfo(0, info);
        }

        code
    }
}

impl std::convert::From<HRESULT> for Error {
    fn from(code: HRESULT) -> Self {
        let mut info = None;
        let info: Option<IRestrictedErrorInfo> = unsafe {
            GetErrorInfo(0, &mut info)
                .and_some(info)
                .and_then(|e| e.cast())
                .ok()
        };

        if let Some(info) = info {
            // If it does (and therefore running on a recent version of Windows)
            // then capture_propagation_context adds a breadcrumb to the error
            // info to make debugging easier.
            if let Ok(capture) = info.cast::<ILanguageExceptionErrorInfo2>() {
                unsafe {
                    let _ = capture.CapturePropagationContext(None);
                }
            }

            return Self {
                code,
                info: Some(info),
            };
        }

        let mut result = None;
        unsafe {
            let _ = GetErrorInfo(0, &mut result);
        }

        if let Some(info) = result {
            let mut message = BSTR::default();
            unsafe {
                let _ = info.GetDescription(&mut message);
            }
            let message: String = message.try_into().unwrap_or_default();
            Self::new(code, &message)
        } else {
            Self::new(code, "")
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Error")
            .field("code", &format_args!("{:#010X}", self.code.0))
            .field("message", &self.message())
            .finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.message())
    }
}

impl std::error::Error for Error {}

demand_load! {
    "combase.dll" {
        fn RoOriginateError(code: HRESULT, message: RawPtr) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let code = Error::fast_error(HRESULT::from_win32(0));
        assert_eq!(code.message(), "The operation completed successfully.");

        let code = Error::fast_error(HRESULT::from_win32(997));
        assert_eq!(code.message(), "Overlapped I/O operation is in progress.");
    }
}
