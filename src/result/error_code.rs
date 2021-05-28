use crate::*;

use bindings::{
    Windows::Win32::Foundation::{E_POINTER, PWSTR},
    Windows::Win32::System::Diagnostics::Debug::*,
};

/// A primitive error code value returned by most COM functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[must_use]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub u32);

impl HRESULT {
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

    /// Converts the `HRESULT` to `Result<()>`.
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
                Err(Error::fast_error(E_POINTER))
            }
        } else {
            Err(Error::from(self))
        }
    }

    /// Calls `op` if `self` is a success code, otherwise returns `HRESULT`
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
        Self::from_win32(unsafe { GetLastError().0 })
    }

    /// Creates a failure code with the provided win32 error code.
    ///
    ///This is equivalent to [HRESULT_FROM_WIN32](https://docs.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-hresult_from_win32).
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
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
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
}

unsafe impl Abi for HRESULT {
    type Abi = Self;
}

impl<T> std::convert::From<Result<T>> for HRESULT {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.into();
        }

        HRESULT(0)
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
        let code = HRESULT::from_win32(0);
        assert_eq!(code.message(), "The operation completed successfully.");

        let code = HRESULT::from_win32(997);
        assert_eq!(code.message(), "Overlapped I/O operation is in progress.");
    }
}
