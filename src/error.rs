#![allow(overflowing_literals)]

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

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
