#![allow(dead_code)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorCode(pub i32);

pub struct Error {
    code: ErrorCode,
    // IErrorInfo*
}

pub type Result<T> = std::result::Result<T, Error>;

impl ErrorCode {
    pub fn is_ok(&self) -> bool {
        self.0 >= 0
    }

    pub fn is_err(&self) -> bool {
        self.0 < 0
    }

    pub fn unwrap(&self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    fn ok_or(&self) -> Result<()> {
        if self.0 < 0 {
            Err(Error { code: *self })
        } else {
            Ok(())
        }
    }
}
