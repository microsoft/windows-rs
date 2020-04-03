#![allow(dead_code)]
#![allow(overflowing_literals)]

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ErrorCode(pub i32);

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    // TODO: add `info: IErrorInfo`
}

#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

impl ErrorCode {
    pub fn is_ok(self) -> bool {
        self.0 >= 0
    }

    pub fn is_err(self) -> bool {
        self.0 < 0
    }

    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    pub fn ok(self) -> Result<()> {
        if self.0 < 0 {
            Err(Error { code: self })
        } else {
            Ok(())
        }
    }

    pub fn ok_or<T>(self, value: T) -> Result<T> {
        if self.0 < 0 {
            Err(Error { code: self })
        } else {
            Ok(value)
        }
    }

    pub(crate) const NOT_INITIALIZED: ErrorCode = ErrorCode(0x8004_01F0);
}
