use super::*;
#[allow(unused_imports)]
use core::mem::size_of;

// We define ErrorT and Error separately because it allows type inference to work correctly
// in some common situations.  If there were no separate type alias, then expressions like
// `let e = Error::from(...)` and `let e = Error::new(...)` would have a free (unspecified) type
// parameter, and so are ambiguous and fail to compile.
//
// Most code wants to use the default type parameter, so we give the type alias the `Error` name.
// For the code that wants to deal directly non-default error details, it can use `ErrorT<...>`.

/// An error object consists of both an error code and optional detailed error information for debugging.
///
/// # Extended error info and the `slim-errors` feature
///
/// The `Error` contains an `HRESULT` value that describes the error, as well as an optional
/// `IErrorInfo` object. The `IErrorInfo` object is a COM object that can provide detailed information
/// about an error, such as a text string, a `ProgID` of the originator, etc. If the error object
/// was originated in an WinRT component, then additional information such as a stack track may be
/// captured.
///
/// However, many systems based on COM do not use `IErrorInfo`. For these systems, the optional error
/// info within `Error` has no benefits, but has substantial costs because it increases the size of
/// the `Error` object, which also increases the size of `Result<T>`.
///
/// The `slim-errors` Cargo feature allows you to _statically_ disable error detail in `Error`.
/// This allows the `Error` and even the `Result<(), Error>` types to have a very small size; they
/// have the same size and alignment as `HRESULT` itself. Because of this, they can be returned
/// directly in a machine register, which gives better performance than writing the (relatively
/// large) `Error` object (with full error detail) to the stack on every function return.
#[derive(Clone)]
pub struct ErrorT<Detail = DefaultDetail> {
    /// The error code. Because this uses `NonZeroHRESULT`, this provides a "niche" to the Rust
    /// compiler. This allows the compiler to use more compact representations in some sitatutions.
    code: NonZeroHRESULT,

    /// Contains details about the error, such as error text.
    ///
    /// Note; We cannot use `Option<Detail>` here, because that would increase the size of `Error`
    /// even when `Detail` contains no information at all (a ZST).  The `Detail` type itself must
    /// be a ZST for `Error` to be a slim type.
    detail: Detail,
}

/// An error object consists of both an error code as well as detailed error information for debugging.
///
/// This is a type alias for the [`ErrorT`] type.
pub type Error = ErrorT<DefaultDetail>;

/// Specifies the default error detail for the `Error` type.
#[cfg(not(feature = "slim-errors"))]
pub type DefaultDetail = ErrorInfo;

/// Specifies the default error detail for the `Error` type.
#[cfg(feature = "slim-errors")]
pub type DefaultDetail = NoDetail;

/// On non-Windows platforms, `ErrorInfo` is mapped to `NoDetail` because non-Windows platforms
/// do not support `IErrorInfo`.
#[cfg(not(windows))]
pub type ErrorInfo = NoDetail;

/// This type implements `ErrorDetail`, by containing no error information at all. This type allows
/// for "slim" `Error` objects, which have the same size as `HRESULT`.
#[derive(Default, Clone)]
pub struct NoDetail;

/// Defines the behavior of error detail objects, which are stored within `Error`.
pub trait ErrorDetail: Sized + Default {
    /// If this implementation stores error detail in ambient (thread-local) storage, then this
    /// function transfers the error detail from ambient storage and returns it.
    fn from_ambient() -> Self;

    /// If this implementation stores error detail in ambient (thread-local) storage, then this
    /// function tranfers `self` into that ambient storage.
    ///
    /// If this implementation does not store error detail in ambient storage, then this function
    /// discards the error detail.
    fn into_ambient(self);

    /// If this implementation stores error detail in ambient (thread-local) storage, then this
    /// function creates a new error detail object, given the `HRESULT` and a message string,
    /// and stores the new error detail object in the ambient storage.
    fn originate_error(code: HRESULT, message: &str);

    /// Returns a message describing the error, if available.
    fn message(&self) -> Option<String>;
}

impl ErrorDetail for NoDetail {
    fn from_ambient() -> Self {
        Self
    }
    fn into_ambient(self) {}
    fn originate_error(_code: HRESULT, _message: &str) {}
    fn message(&self) -> Option<String> {
        None
    }
}

#[cfg(windows)]
pub use win_impl::ErrorInfo;

#[cfg(windows)]
mod win_impl {
    use super::*;
    use crate::bstr::BasicString;
    use crate::com::ComPtr;

    /// This type stores error detail, represented by a COM `IErrorInfo` object.
    ///
    /// # References
    ///
    /// * [`IErrorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-ierrorinfo)
    #[derive(Clone, Default)]
    pub struct ErrorInfo {
        pub(super) ptr: Option<ComPtr>,
    }

    unsafe impl Send for ErrorInfo {}
    unsafe impl Sync for ErrorInfo {}

    impl ErrorInfo {
        /// Gets a raw pointer to the `IErrorInfo` COM object stored in this `ErrorInfo`.
        pub fn as_ptr(&self) -> *mut core::ffi::c_void {
            self.ptr
                .as_ref()
                .map_or(core::ptr::null_mut(), |info| info.as_raw())
        }
    }

    impl ErrorDetail for ErrorInfo {
        fn from_ambient() -> Self {
            unsafe {
                let mut ptr = None;
                GetErrorInfo(0, &mut ptr as *mut _ as _);
                Self { ptr }
            }
        }

        fn into_ambient(self) {
            if let Some(ptr) = self.ptr {
                unsafe {
                    SetErrorInfo(0, ptr.as_raw());
                }
            }
        }

        fn originate_error(code: HRESULT, message: &str) {
            if message.is_empty() {
                return;
            }

            let mut wide_message: Vec<u16> = Vec::with_capacity(message.encode_utf16().count() + 1);
            wide_message.extend(message.encode_utf16());
            let wide_message_len = wide_message.len(); // does not count NUL
            wide_message.push(0);

            unsafe {
                RoOriginateErrorW(code.0, wide_message_len as u32, wide_message.as_ptr());
            }
        }

        fn message(&self) -> Option<String> {
            let info = self.ptr.as_ref()?;

            let mut message = BasicString::default();

            // First attempt to retrieve the restricted error information.
            if let Some(info) = info.cast(&IID_IRestrictedErrorInfo) {
                let mut fallback = BasicString::default();
                let mut code = 0;

                unsafe {
                    com_call!(
                        IRestrictedErrorInfo_Vtbl,
                        info.GetErrorDetails(
                            &mut fallback as *mut _ as _,
                            &mut code,
                            &mut message as *mut _ as _,
                            &mut BasicString::default() as *mut _ as _
                        )
                    );
                }

                if message.is_empty() {
                    message = fallback
                };
            }

            // Next attempt to retrieve the regular error information.
            if message.is_empty() {
                unsafe {
                    com_call!(
                        IErrorInfo_Vtbl,
                        info.GetDescription(&mut message as *mut _ as _)
                    );
                }
            }

            if message.is_empty() {
                return None;
            }

            Some(String::from_utf16_lossy(crate::strings::wide_trim_end(
                message.as_wide(),
            )))
        }
    }
}

impl<Detail: Default> ErrorT<Detail> {
    /// Creates an error object without any specific failure information.
    ///
    /// The `code()` for this error is `E_FAIL`.
    pub fn fail() -> Self {
        Self {
            code: NonZeroHRESULT::E_FAIL,
            detail: Detail::default(),
        }
    }

    /// Creates a new error object with an error code, but without additional error information.
    ///
    /// If `code` is `S_OK`, then this is remapped to `E_FAIL`. The `Error` object cannot
    /// store an `S_OK` value.
    pub fn from_hresult(code: HRESULT) -> Self {
        Self {
            code: NonZeroHRESULT::from_hresult_or_fail(code),
            detail: Detail::default(),
        }
    }

    /// Creates a new `Error` from the Win32 error code returned by `GetLastError()`.
    ///
    /// If `GetLastError()` returns `ERROR_SUCCESS` then this is remapped to `E_FAIL`. The `Error`
    /// object cannot store an `S_OK` value.
    pub fn from_win32() -> Self {
        #[cfg(windows)]
        {
            let hr = HRESULT::from_win32(unsafe { GetLastError() });
            Self::from_hresult(hr)
        }
        #[cfg(not(windows))]
        {
            unimplemented!()
        }
    }
}

impl<Detail: ErrorDetail> ErrorT<Detail> {
    /// Creates a new error object, capturing the stack and other information about the
    /// point of failure.
    ///
    /// If `code` is `S_OK`, then this is remapped to `E_FAIL`. The `Error` object cannot
    /// store an `S_OK` value.
    pub fn new<T: AsRef<str>>(code: HRESULT, message: T) -> Self {
        Detail::originate_error(code, message.as_ref());
        // This into() call will take the ambient thread-local error object, if any.
        Self::from(code)
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        if let Some(message) = self.detail.message() {
            return message;
        }

        // Otherwise fallback to a generic error code description.
        self.code.message()
    }
}

impl<Detail> ErrorT<Detail> {
    /// Gets access to the error detail stored in this `Error`.
    pub fn detail(&self) -> &Detail {
        &self.detail
    }

    /// The error code describing the error.
    pub const fn code(&self) -> HRESULT {
        self.code.to_hresult()
    }
}

#[cfg(feature = "std")]
impl<Detail: ErrorDetail> std::error::Error for ErrorT<Detail> {}

impl<Detail: ErrorDetail> From<ErrorT<Detail>> for HRESULT {
    fn from(error: ErrorT<Detail>) -> Self {
        error.detail.into_ambient();
        error.code.into()
    }
}

impl<Detail: ErrorDetail> From<HRESULT> for ErrorT<Detail> {
    fn from(code: HRESULT) -> Self {
        Self {
            code: NonZeroHRESULT::from_hresult_or_fail(code),
            detail: Detail::from_ambient(),
        }
    }
}

#[cfg(feature = "std")]
impl<Detail: ErrorDetail> From<ErrorT<Detail>> for std::io::Error {
    fn from(from: ErrorT<Detail>) -> Self {
        Self::from_raw_os_error(from.code.0.get())
    }
}

#[cfg(feature = "std")]
impl<Detail: ErrorDetail> From<std::io::Error> for ErrorT<Detail> {
    fn from(from: std::io::Error) -> Self {
        match from.raw_os_error() {
            Some(status) => HRESULT::from_win32(status as u32).into(),
            None => HRESULT(E_UNEXPECTED).into(),
        }
    }
}

impl<Detail: Default> From<alloc::string::FromUtf16Error> for ErrorT<Detail> {
    fn from(_: alloc::string::FromUtf16Error) -> Self {
        Self::from_hresult(HRESULT::from_win32(ERROR_NO_UNICODE_TRANSLATION))
    }
}

impl<Detail: Default> From<alloc::string::FromUtf8Error> for ErrorT<Detail> {
    fn from(_: alloc::string::FromUtf8Error) -> Self {
        Self::from_hresult(HRESULT::from_win32(ERROR_NO_UNICODE_TRANSLATION))
    }
}

impl<Detail: Default> From<core::num::TryFromIntError> for ErrorT<Detail> {
    fn from(_: core::num::TryFromIntError) -> Self {
        Self::from_hresult(HRESULT::from_win32(ERROR_INVALID_DATA))
    }
}

impl<Detail: ErrorDetail> core::fmt::Debug for ErrorT<Detail> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut debug = fmt.debug_struct("Error");
        debug
            .field("code", &self.code)
            .field("message", &self.message())
            .finish()
    }
}

impl<Detail: ErrorDetail> core::fmt::Display for ErrorT<Detail> {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let message = self.message();
        if message.is_empty() {
            core::write!(fmt, "{}", self.code())
        } else {
            core::write!(fmt, "{} ({})", self.message(), self.code())
        }
    }
}

impl<Detail> core::hash::Hash for ErrorT<Detail> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.code.hash(state);
        // We do not hash the detail.
    }
}

// Equality tests only the HRESULT, not the error info (if any).
impl<Detail> PartialEq for ErrorT<Detail> {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl<Detail> Eq for ErrorT<Detail> {}

impl<Detail> PartialOrd for ErrorT<Detail> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Detail> Ord for ErrorT<Detail> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.code.cmp(&other.code)
    }
}

// If we are using "slim" Error objects, then we can rely on Result<()>
// having a representation that is equivalent to HRESULT.
static_assertions::const_assert_eq!(
    size_of::<core::result::Result<(), ErrorT<NoDetail>>>(),
    size_of::<HRESULT>()
);

static_assertions::assert_impl_all!(Error: Send, Sync);
