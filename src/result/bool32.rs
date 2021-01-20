use crate::*;

/// A 32-bit boolean error code value returned by some Win32 functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct BOOL(pub i32);

impl BOOL {
    /// Returns `true` if `self` is a success code.
    #[inline]
    pub fn is_ok(self) -> bool {
        self.0 != 0
    }

    /// Returns `true` if `self` is a failure code.
    #[inline]
    pub fn is_err(self) -> bool {
        self.0 == 0
    }

    /// Asserts that `self` is a success code.
    #[inline]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }

    /// Converts the `BOOL` to `Result<()>`.
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(ErrorCode::from_thread().into())
        }
    }
}

unsafe impl Abi for BOOL {
    type Abi = Self;
}

impl From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            Self(1)
        } else {
            Self(0)
        }
    }
}

impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value.0 != 0
    }
}
