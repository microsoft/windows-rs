#![allow(overflowing_literals)]

/// An alias for `std::result::Result<T, winrt::Error>`
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl<T> std::convert::From<Result<T>> for ErrorCode {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            // TODO: call SetErrorInfo
            return error.code();
        }

        ErrorCode(0)
    }
}

/// A WinRT related error
#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    // TODO: add `info: IErrorInfo`
}

impl Error {
    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

type HRESULT = i32;

/// The ErrorCode (a.k.a HRESULT) of an error
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ErrorCode(pub HRESULT);

impl ErrorCode {
    #[inline]
    pub fn is_ok(self) -> bool {
        self.0 >= 0
    }

    #[inline]
    pub fn is_err(self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(Error { code: self })
        }
    }

    #[inline]
    pub fn and_then<F, T>(self, value: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(value())
    }

    pub(crate) const NOT_INITIALIZED: ErrorCode = ErrorCode(0x8004_01F0);
}
