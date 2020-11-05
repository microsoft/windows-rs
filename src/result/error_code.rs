use crate::*;

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
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

    #[inline]
    pub fn ok_ptr(self, ptr: RawPtr) -> Result<()> {
        if self.is_ok() {
            if ptr.is_null() {
                Err(ErrorCode::E_POINTER.into())
            } else {
                Ok(())
            }
        } else {
            Err(Error::from(self))
        }
    }

    pub fn and_some<T: ComInterface>(self, some: Option<T>) -> Result<T> {
        if self.is_ok() {
            if let Some(result) = some {
                Ok(result)
            } else {
                Err(ErrorCode::E_POINTER.into())
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

    pub fn into_result<T: Abi>(self, abi: T::Abi) -> Result<T> {
        if self.is_ok() {
            T::into_result(abi)
        } else {
            Err(Error::from(self))
        }
    }

    /// Creates a failure code from GetLastError()
    #[inline]
    pub(crate) fn last_win32_error() -> Self {
        Self::from_win32(unsafe { GetLastError() })
    }

    /// Creates a failure code with the provided win32 error code.
    #[inline]
    pub(crate) fn from_win32(error: u32) -> Self {
        // equivalent to MAKE_WIN32_HRESULT(error)
        Self(0x8007_0000 | error & 0xFFFF)
    }

    pub const S_OK: ErrorCode = ErrorCode(0);
    pub const CO_E_NOTINITIALIZED: ErrorCode = ErrorCode(0x8004_01F0);
    pub const E_NOINTERFACE: ErrorCode = ErrorCode(0x8000_4002);
    pub const E_POINTER: ErrorCode = ErrorCode(0x8000_4003);
}

impl<T> std::convert::From<Result<T>> for ErrorCode {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.into();
        }

        ErrorCode(0)
    }
}

impl std::convert::From<Error> for ErrorCode {
    fn from(error: Error) -> Self {
        unsafe {
            let _ = SetRestrictedErrorInfo(error.info().get_abi());
        }

        error.code()
    }
}

demand_load! {
    "combase.dll" {
        pub fn SetRestrictedErrorInfo(info: RawPtr) -> ErrorCode;
    }
}
