use super::*;

/// An error or status code value returned by some operating system functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[must_use]
pub struct NTSTATUS(pub i32);

impl NTSTATUS {
    /// Returns [`true`] if `self` is a success code.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 >= 0
    }

    /// Returns [`true`] if `self` is a failure code.
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Maps an NT error code to an HRESULT value.
    #[inline]
    pub const fn to_hresult(self) -> HRESULT {
        HRESULT(if self.0 >= 0 {
            self.0
        } else {
            self.0 | 0x1000_0000
        })
    }

    /// Asserts that `self` is a success code.
    ///
    /// This will invoke the [`panic!`] macro if `self` is a failure code and display
    /// the [`NTSTATUS`] value for diagnostics.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "NTSTATUS 0x{:X}", self.0);
    }

    /// Converts the [`NTSTATUS`] to [`Result<()>`][Result<_>].
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.to_hresult().into())
        }
    }
}

impl From<NTSTATUS> for HRESULT {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult()
    }
}

impl From<NTSTATUS> for Error {
    fn from(value: NTSTATUS) -> Self {
        value.to_hresult().into()
    }
}

impl core::fmt::Display for NTSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#010X}", self.0))
    }
}

impl core::fmt::Debug for NTSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("NTSTATUS({self})"))
    }
}
