use super::*;

/// An error or status code value returned by some operating system functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[must_use]
pub struct RPC_STATUS(pub i32);

impl RPC_STATUS {
    /// Returns [`true`] if `self` is a success code.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0 == 0
    }

    /// Returns [`true`] if `self` is a failure code.
    #[inline]
    pub const fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Maps an RPC error code to an HRESULT value.
    #[inline]
    pub const fn to_hresult(self) -> HRESULT {
        WIN32_ERROR(self.0 as u32).to_hresult()
    }

    /// Converts the [`RPC_STATUS`] to [`Result<()>`][Result<_>].
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.to_hresult().into())
        }
    }
}

impl From<RPC_STATUS> for HRESULT {
    fn from(value: RPC_STATUS) -> Self {
        value.to_hresult()
    }
}

impl From<RPC_STATUS> for Error {
    fn from(value: RPC_STATUS) -> Self {
        value.to_hresult().into()
    }
}

impl core::fmt::Display for RPC_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#010X}", self.0))
    }
}

impl core::fmt::Debug for RPC_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("RPC_STATUS({self})"))
    }
}
