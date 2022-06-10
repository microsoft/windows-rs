use super::*;
use bindings::*;

/// An error code value returned by most COM functions.
#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
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
    pub const fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(Error { code: self, info: None })
        }
    }

    /// Returns the [`Option`] as a [`Result`] if the option is a [`Some`] value, returning
    /// a suitable error if not.
    pub fn and_some<T: Interface>(self, some: Option<T>) -> Result<T> {
        if self.is_ok() {
            if let Some(result) = some {
                Ok(result)
            } else {
                Err(Error::OK)
            }
        } else {
            Err(Error::from(self))
        }
    }

    /// Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]
    /// converted to [`Result<T>`].
    #[inline]
    pub fn and_then<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(op())
    }

    /// If the [`Result`] is [`Ok`] converts the `T::Abi` into `T`.
    ///
    /// # Safety
    ///
    /// Safe to call if
    /// * `abi` is initialized if `self` is `Ok`
    /// * `abi` can be safely transmuted to `T`
    pub unsafe fn from_abi<T: Abi>(self, abi: core::mem::MaybeUninit<T::Abi>) -> Result<T> {
        if self.is_ok() {
            T::from_abi(abi.assume_init())
        } else {
            Err(Error::from(self))
        }
    }

    /// The error message describing the error.
    pub fn message(&self) -> HSTRING {
        let mut message = HeapString(core::ptr::null_mut());

        unsafe {
            let size = FormatMessageW(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, core::ptr::null(), self.0 as _, 0, PWSTR(core::mem::transmute(&mut message.0)), 0, core::ptr::null_mut());

            HSTRING::from_wide(wide_trim_end(core::slice::from_raw_parts(message.0 as *const u16, size as usize)))
        }
    }
}

unsafe impl Abi for HRESULT {
    type Abi = Self;
}

unsafe impl RuntimeType for HRESULT {
    const SIGNATURE: ConstBuffer = ConstBuffer::from_slice(b"struct(Windows.Foundation.HResult;i32)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> Result<Self> {
        Ok(*from)
    }
}

impl<T> core::convert::From<Result<T>> for HRESULT {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.into();
        }

        HRESULT(0)
    }
}

impl core::fmt::Debug for HRESULT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("HRESULT(0x{:08X})", self.0))
    }
}

struct HeapString(*mut u16);

impl Drop for HeapString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                heap_free(self.0 as _);
            }
        }
    }
}
