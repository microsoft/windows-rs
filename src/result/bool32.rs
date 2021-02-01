use crate::{Abi, ErrorCode};

/// A 32-bit boolean error code value returned by some Win32 functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct BOOL(pub i32);

/// A BOOL representing true
pub const TRUE: BOOL = BOOL(1);
/// A BOOL representing false
pub const FALSE: BOOL = BOOL(0);

impl BOOL {
    /// Convert the `BOOL` into a `bool`
    #[inline]
    pub fn as_bool(self) -> bool {
        self.into()
    }

    /// Asserts that `self` is a success code.
    #[inline]
    pub fn unwrap(self) {
        self.ok().unwrap();
    }

    /// Expects that `self` is a success code.
    #[inline]
    pub fn expect(self, msg: &str) {
        self.ok().expect(msg);
    }

    /// Converts the `BOOL` to `Result<()>`.
    #[inline]
    pub fn ok(self) -> crate::Result<()> {
        if self == TRUE {
            Ok(())
        } else {
            Err(ErrorCode::from_thread().into())
        }
    }
}

unsafe impl Abi for BOOL {
    type Abi = Self;
}

impl From<BOOL> for bool {
    fn from(value: BOOL) -> Self {
        value == TRUE
    }
}

impl From<bool> for BOOL {
    fn from(b: bool) -> Self {
        if b {
            TRUE
        } else {
            FALSE
        }
    }
}

impl std::fmt::Debug for BOOL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = if *self == FALSE { "FALSE" } else { "TRUE" };
        f.write_str(msg)
    }
}

impl PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        let other: BOOL = (*other).into();
        self == &other
    }
}

impl std::ops::Neg for BOOL {
    type Output = Self;
    fn neg(self) -> Self::Output {
        if self == TRUE {
            FALSE
        } else {
            TRUE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bool_interop() {
        let win_bool = TRUE;
        let bool = true;
        assert_eq!(win_bool, bool);
    }
}
