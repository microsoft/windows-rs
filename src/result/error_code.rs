use crate::*;

use bindings::{
    windows::win32::debug::{FormatMessageW, GetLastError},
    windows::win32::system_services::PWSTR,
    windows::win32::windows_programming::FORMAT_MESSAGE_OPTIONS,
};

/// A primitive error code value returned by most COM functions. An `ErrorCode` is sometimes called an `HRESULT`.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[must_use]
pub struct ErrorCode(pub u32);

impl ErrorCode {
    /// Returns `true` if `self` is a success code.
    #[inline]
    pub fn is_ok(self) -> bool {
        self.0 & 0x8000_0000 == 0
    }

    /// Returns `true` if `self` is a failure code.
    #[inline]
    pub fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Asserts that `self` is a success code.
    ///
    /// This will  invoke the `panic!` macro if `self` is a failure code and display
    /// the `HRESULT` value for diagnostics.
    #[inline]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    /// Converts the `ErrorCode` to `Result<()>`.
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.into())
        }
    }

    /// Returns the `Option` as a `Result` if the option is a `Some` value, returning
    /// a suitable error if not.
    pub fn and_some<T: Interface>(self, some: Option<T>) -> Result<T> {
        if self.is_ok() {
            if let Some(result) = some {
                Ok(result)
            } else {
                Err(Error::fast_error(ErrorCode::E_POINTER))
            }
        } else {
            Err(Error::from(self))
        }
    }

    /// Calls `op` if `self` is a success code, otherwise returns `ErrorCode`
    /// converted to `Result<T>`.
    #[inline]
    pub fn and_then<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(op())
    }

    /// If the `Result` is `Ok` converts the `T::Abi` into `T`.
    pub unsafe fn from_abi<T: Abi>(self, abi: T::Abi) -> Result<T> {
        if self.is_ok() {
            T::from_abi(abi)
        } else {
            Err(Error::from(self))
        }
    }

    /// Retrieves the error code stored on the calling thread.
    #[inline]
    pub fn from_thread() -> Self {
        Self::from_win32(unsafe { GetLastError() })
    }

    /// Creates a failure code with the provided win32 error code. This is equivalent to
    /// [HRESULT_FROM_WIN32](https://docs.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-hresult_from_win32).
    #[inline]
    pub fn from_win32(error: u32) -> Self {
        Self(if error as i32 <= 0 {
            error
        } else {
            (error & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
        })
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        let mut message = HeapString(std::ptr::null_mut());

        unsafe {
            let size = FormatMessageW(
                FORMAT_MESSAGE_OPTIONS::FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_OPTIONS::FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_OPTIONS::FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null(),
                self.0,
                0x0000_0400, // MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT)
                PWSTR(std::mem::transmute(&mut message.0)),
                0,
                std::ptr::null_mut(),
            );

            String::from_utf16_lossy(std::slice::from_raw_parts(
                message.0 as *const u16,
                size as usize,
            ))
            .trim_end()
            .to_owned()
        }
    }

    // This is a limited and closed set of common values used for flow control. In general, error codes are not actionable
    // beyond debugging and should be considered fatal. This list should therefore not be expanded.

    /// The operation succeeded.
    pub const S_OK: ErrorCode = ErrorCode(0);

    /// The COM runtime has not been loaded.
    pub const CO_E_NOTINITIALIZED: ErrorCode = ErrorCode(0x8004_01F0);

    /// The requested interface is not implemented.
    pub const E_NOINTERFACE: ErrorCode = ErrorCode(0x8000_4002);

    /// A null pointer was sent or received.
    pub const E_POINTER: ErrorCode = ErrorCode(0x8000_4003);
}

unsafe impl Abi for ErrorCode {
    type Abi = Self;
}

impl<T> std::convert::From<Result<T>> for ErrorCode {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.into();
        }

        ErrorCode(0)
    }
}

pub struct HeapString(*mut u16);

impl Drop for HeapString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                heap_free(self.0 as _);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        let code = ErrorCode::from_win32(0);
        assert_eq!(code.message(), "The operation completed successfully.");

        let code = ErrorCode::from_win32(997);
        assert_eq!(code.message(), "Overlapped I/O operation is in progress.");
    }
}
