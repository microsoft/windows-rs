use super::*;
use core::mem::size_of;
use core::num::NonZeroI32;
use static_assertions::const_assert_eq;

/// An error code value returned by most COM functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[must_use]
#[allow(non_camel_case_types)]
pub struct HRESULT(pub i32);

impl HRESULT {
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

    /// Asserts that `self` is a success code.
    ///
    /// This will invoke the [`panic!`] macro if `self` is a failure code and display
    /// the [`HRESULT`] value for diagnostics.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    /// Converts the [`HRESULT`] to [`Result<()>`][Result<_>].
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self.into())
        }
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn map<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(op())
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn and_then<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        self.ok()?;
        op()
    }

    /// The error message describing the error.
    pub fn message(self) -> String {
        #[cfg(windows)]
        {
            let mut message = crate::strings::HeapString::default();
            let mut code = self.0;
            let mut module = core::ptr::null_mut();

            let mut flags = FORMAT_MESSAGE_ALLOCATE_BUFFER
                | FORMAT_MESSAGE_FROM_SYSTEM
                | FORMAT_MESSAGE_IGNORE_INSERTS;

            unsafe {
                if self.0 & 0x1000_0000 == 0x1000_0000 {
                    code ^= 0x1000_0000;
                    flags |= FORMAT_MESSAGE_FROM_HMODULE;

                    module = LoadLibraryExA(
                        b"ntdll.dll\0".as_ptr(),
                        core::ptr::null_mut(),
                        LOAD_LIBRARY_SEARCH_DEFAULT_DIRS,
                    );
                }

                let size = FormatMessageW(
                    flags,
                    module as _,
                    code as _,
                    0,
                    &mut message.0 as *mut _ as *mut _,
                    0,
                    core::ptr::null(),
                );

                if !message.0.is_null() && size > 0 {
                    let message_slice = core::slice::from_raw_parts(message.0, size as usize);
                    String::from_utf16_lossy(crate::strings::wide_trim_end(message_slice))
                } else {
                    String::default()
                }
            }
        }

        #[cfg(not(windows))]
        {
            return format!("0x{:08x}", self.0 as u32);
        }
    }

    /// Maps a Win32 error code to an HRESULT value.
    pub const fn from_win32(error: u32) -> Self {
        Self(if error as i32 <= 0 {
            error
        } else {
            (error & 0x0000_FFFF) | (7 << 16) | 0x8000_0000
        } as i32)
    }

    /// Maps an NT error code to an HRESULT value.
    pub const fn from_nt(error: i32) -> Self {
        Self(if error >= 0 {
            error
        } else {
            error | 0x1000_0000
        })
    }
}

impl<T> From<Result<T>> for HRESULT {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.into();
        }
        HRESULT(0)
    }
}

impl core::fmt::Display for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#010X}", self.0))
    }
}

impl core::fmt::Debug for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("HRESULT({})", self))
    }
}

impl core::fmt::Display for NonZeroHRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <HRESULT as core::fmt::Display>::fmt(&HRESULT::from(*self), f)
    }
}

impl core::fmt::Debug for NonZeroHRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <HRESULT as core::fmt::Debug>::fmt(&HRESULT::from(*self), f)
    }
}

/// An error code value returned by most COM functions.
///
/// This type cannot represent `S_OK`. This type is intended for use with either
/// `Option` or `Result` so that the unused 0 bit pattern can be used as a "niche",
/// which allows `Option` and `Result` to use the bit pattern and conserve space.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[must_use]
#[allow(non_camel_case_types)]
pub struct NonZeroHRESULT(pub NonZeroI32);

const_assert_eq!(size_of::<NonZeroHRESULT>(), size_of::<HRESULT>());

// Check the Option niche optimization.
const_assert_eq!(size_of::<Option<NonZeroHRESULT>>(), size_of::<HRESULT>());

// Check the Result niche optimization.
const_assert_eq!(
    size_of::<core::result::Result<(), NonZeroHRESULT>>(),
    size_of::<HRESULT>()
);

impl NonZeroHRESULT {
    /// Returns [`true`] if `self` is a success code.
    ///
    /// Although `NonZeroHRESULT` cannot represent `S_OK`, it can represent other success values
    /// such as `S_FALSE`.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.0.get() >= 0
    }

    /// Returns [`true`] if `self` is a failure code.
    #[inline]
    pub const fn is_err(self) -> bool {
        self.0.get() < 0
    }

    /// Asserts that `self` is a success code.
    ///
    /// This will invoke the [`panic!`] macro if `self` is a failure code and display
    /// the [`HRESULT`] value for diagnostics.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0.get());
    }

    /// Converts the [`HRESULT`] to [`Result<()>`][Result<_>].
    #[inline]
    pub fn ok(self) -> Result<()> {
        // SAFETY: We use a static assertion to check that the size of these two types are identical.
        // Since there is only one possible niche, that niche must be used for the Ok(()) value.
        #[cfg(any(feature = "slim-error", not(windows)))]
        unsafe {
            core::mem::transmute(self)
        }

        // If we are not using slim Error, then we take the slow path.
        #[cfg(not(any(feature = "slim-error", not(windows))))]
        HRESULT::from(self).ok()
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn map<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(op())
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn and_then<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        self.ok()?;
        op()
    }

    /// The error message describing the error.
    pub fn message(self) -> String {
        HRESULT::from(self).message()
    }

    /// Converts `hr` to `NonZeroHRESULT`. If `hr` is `S_OK` (zero), then this returns `None`.
    pub const fn from_hresult_opt(hr: HRESULT) -> Option<Self> {
        // SAFETY: The niche optimization is guaranteed for Option<NonZeroI32>. This transmute
        // exploits the same niche optimization, only for an HRESULT that wraps an i32.
        unsafe { core::mem::transmute(hr) }
    }

    /// Converts `HRESULT` to `NonZeroHRESULT`.
    ///
    /// This is intended only for use in constant declarations. This will panic if `error` is 0.
    pub const fn from_hresult_unwrap(hr: HRESULT) -> Self {
        if let Some(nz) = NonZeroI32::new(hr.0) {
            Self(nz)
        } else {
            panic!("`hr` cannot be S_OK (zero)");
        }
    }

    /// The well-known `E_FAIL` constant.
    pub const E_FAIL: NonZeroHRESULT = Self::from_hresult_unwrap(HRESULT(0x80004005_u32 as i32));

    /// Converts `hr` to `NonZeroHRESULT`. If `hr` is `S_OK`, then this returns `E_FAIL`.
    pub const fn from_hresult_or_fail(hr: HRESULT) -> Self {
        if let Some(nz) = NonZeroI32::new(hr.0) {
            Self(nz)
        } else {
            Self::E_FAIL
        }
    }

    /// Maps a Win32 error code to a NonZeroHRESULT value. If `error` is 0, then this will
    /// return `E_FAIL`.
    pub const fn from_win32(error: u32) -> Self {
        Self::from_hresult_or_fail(HRESULT::from_win32(error))
    }

    /// Maps an NT error code to an HRESULT value.
    ///
    /// This is intended only for use in constant declarations. If `error` is 0, then this will
    /// return `E_FAIL`.
    pub const fn from_nt(error: i32) -> Self {
        Self::from_hresult_or_fail(HRESULT::from_nt(error))
    }

    /// Converts this `NonZeroHRESULT` to an `HRESULT`. This is usable in `const fn`.
    pub const fn to_hresult(self) -> HRESULT {
        HRESULT(self.0.get())
    }
}

impl From<NonZeroHRESULT> for HRESULT {
    fn from(hr: NonZeroHRESULT) -> Self {
        Self(hr.0.get())
    }
}
