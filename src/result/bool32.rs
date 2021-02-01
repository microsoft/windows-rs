use crate::{Abi, ErrorCode};

/// A 32-bit boolean error code value returned by some Win32 functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct BOOL(pub i32);

/// A BOOL representing true.
pub const TRUE: BOOL = BOOL(1);

/// A BOOL representing false.
pub const FALSE: BOOL = BOOL(0);

impl BOOL {
    /// Convert the `BOOL` into a `bool`.
    #[inline]
    pub fn as_bool(self) -> bool {
        if self.0 == 0 {
            false
        } else {
            true
        }
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
        if self.as_bool() {
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
        value.as_bool()
    }
}

impl From<&BOOL> for bool {
    fn from(value: &BOOL) -> Self {
        value.as_bool()
    }
}

impl From<bool> for BOOL {
    fn from(value: bool) -> Self {
        if value {
            TRUE
        } else {
            FALSE
        }
    }
}

impl From<&bool> for BOOL {
    fn from(value: &bool) -> Self {
        (*value).into()
    }
}

impl std::fmt::Debug for BOOL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = if self.as_bool() { "TRUE" } else { "FALSE" };
        f.write_str(msg)
    }
}

impl PartialEq<BOOL> for BOOL {
    fn eq(&self, other: &BOOL) -> bool {
        self.as_bool() == other.as_bool()
    }
}

impl PartialEq<bool> for BOOL {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == *other
    }
}

impl PartialEq<BOOL> for bool {
    fn eq(&self, other: &BOOL) -> bool {
        *self == other.as_bool()
    }
}

impl std::ops::Not for BOOL {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self.as_bool() {
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
    fn comparison() {
        let win_bool: BOOL = TRUE;
        let rust_bool: bool = true;
        assert_eq!(rust_bool, win_bool);

        let win_bool: BOOL = FALSE;
        let rust_bool: bool = false;
        assert_eq!(win_bool, rust_bool);

        let win_bool = BOOL(123);
        let rust_bool: bool = true;
        assert_eq!(rust_bool, win_bool);

        let win_bool = BOOL(-123);
        let rust_bool: bool = true;
        assert_eq!(win_bool, rust_bool);

        let a: BOOL = TRUE;
        let b: BOOL = BOOL(123);
        assert_eq!(a, b);
    }

    #[test]
    fn win_bool_to_rust_bool() {
        let win_bool: BOOL = BOOL(123);
        let rust_bool: bool = win_bool.as_bool();
        assert!(rust_bool);

        let rust_bool: bool = win_bool.into();
        assert!(rust_bool);

        let win_bool_ref: &BOOL = &win_bool;
        let rust_bool: bool = win_bool_ref.as_bool();
        assert!(rust_bool);

        let rust_bool: bool = win_bool_ref.into();
        assert!(rust_bool);
    }

    #[test]
    fn rust_bool_to_win_bool() {
        let rust_bool: bool = true;
        let win_bool: BOOL = rust_bool.into();
        assert_eq!(win_bool, true);

        let rust_bool: &bool = &rust_bool;
        let win_bool: BOOL = rust_bool.into();
        assert_eq!(win_bool, true);

        let rust_bool: bool = false;
        let win_bool: BOOL = rust_bool.into();
        assert_eq!(win_bool, false);

        let rust_bool: &bool = &rust_bool;
        let win_bool: BOOL = rust_bool.into();
        assert_eq!(win_bool, false);
    }

    #[test]
    fn methods() {
        let win_bool: BOOL = BOOL(123);
        assert!(win_bool.as_bool());
        win_bool.unwrap();
        win_bool.expect("test");
        assert!(win_bool.ok().is_ok());

        let win_bool: BOOL = FALSE;
        assert!(!win_bool.as_bool());
        assert!(win_bool.ok().is_err());
    }

    #[test]
    fn format() {
        assert_eq!(format!("{:?}", TRUE), "TRUE");
        assert_eq!(format!("{:?}", FALSE), "FALSE");
        assert_eq!(format!("{:?}", BOOL(123)), "TRUE");
    }

    #[test]
    fn not() {
        let win_bool: BOOL = TRUE;
        let not: BOOL = !win_bool;
        assert!(not != win_bool);
        assert!(not == FALSE);

        let win_bool: BOOL = FALSE;
        let not: BOOL = !win_bool;
        assert!(not != win_bool);
        assert!(not == TRUE);
    }
}
