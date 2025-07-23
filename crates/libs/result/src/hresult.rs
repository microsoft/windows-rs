use super::*;

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
    pub fn ok(self) -> Result<(), Self> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn map<F, T>(self, op: F) -> Result<T, Self>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(op())
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn and_then<F, T>(self, op: F) -> Result<T, Self>
    where
        F: FnOnce() -> Result<T, Self>,
    {
        self.ok()?;
        op()
    }

    /// The error message describing the error.
    pub fn message(self) -> String {
        #[cfg(windows)]
        {
            let mut message = HeapString::default();
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
                    String::from_utf16_lossy(wide_trim_end(core::slice::from_raw_parts(
                        message.0,
                        size as usize,
                    )))
                } else {
                    String::default()
                }
            }
        }

        #[cfg(not(windows))]
        {
            return alloc::format!("0x{:08x}", self.0 as u32);
        }
    }

    /// Creates a new `HRESULT` from the Win32 error code returned by `GetLastError()`.
    pub fn from_thread() -> Self {
        let error = unsafe { GetLastError() };
        Self::from_win32(error)
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

impl From<Result<(), Error>> for HRESULT {
    fn from(result: Result<(), Error>) -> Self {
        match result {
            Ok(()) => Self(0),
            Err(error) => error.to_thread(),
        }
    }
}

impl From<Result<(), HRESULT>> for HRESULT {
    fn from(result: Result<(), HRESULT>) -> Self {
        match result {
            Ok(()) => Self(0),
            Err(error) => error,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HRESULT {}

#[cfg(feature = "std")]
impl From<HRESULT> for std::io::Error {
    fn from(from: HRESULT) -> Self {
        Self::from_raw_os_error(from.0)
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for HRESULT {
    fn from(from: std::io::Error) -> Self {
        match from.raw_os_error() {
            Some(status) => Self::from_win32(status as u32),
            None => Self(E_UNEXPECTED),
        }
    }
}

impl From<alloc::string::FromUtf16Error> for HRESULT {
    fn from(_: alloc::string::FromUtf16Error) -> Self {
        Self::from_win32(ERROR_NO_UNICODE_TRANSLATION)
    }
}

impl From<alloc::string::FromUtf8Error> for HRESULT {
    fn from(_: alloc::string::FromUtf8Error) -> Self {
        Self::from_win32(ERROR_NO_UNICODE_TRANSLATION)
    }
}

impl From<core::num::TryFromIntError> for HRESULT {
    fn from(_: core::num::TryFromIntError) -> Self {
        Self::from_win32(ERROR_INVALID_DATA)
    }
}

impl core::fmt::Display for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#010X}", self.0))
    }
}

impl core::fmt::Debug for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("HRESULT({self})"))
    }
}
