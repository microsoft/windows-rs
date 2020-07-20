use crate::*;

/// An `ErrorCode`, sometimes called an `HRESULT`, is the error code associated with a WinRT error.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ErrorCode(pub u32);

/// A WinRT error object consisting of both an error code as well as detailed error information for debugging.
pub struct Error {
    code: ErrorCode,
    info: IRestrictedErrorInfo,
}

/// An alias for `std::result::Result<T, winrt::Error>`.
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// This creates a new WinRT error object, capturing the stack and other information about the
    /// point of failure.
    pub fn new(code: ErrorCode, message: &str) -> Self {
        let message: HString = message.into();

        // RoOriginateError creates the error object and associates it with the thread.
        // Need to ignore the result, as that is the delay-load error, which would mean
        // that there's no WinRT to tell about the error.
        unsafe {
            let _ = RoOriginateError(code, message.get_abi() as _);
        }

        let mut info = IErrorInfo::default();

        // GetErrorInfo retrieves the error object from the thread.
        unsafe {
            GetErrorInfo(0, info.set_abi() as _);
        }

        // The error information is then associated with the returning error object and no longer
        // associated with the thread.
        let info = info.query::<IRestrictedErrorInfo>();
        Self { code, info }
    }

    /// The error code describing the error.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// The error message describing the error.
    pub fn message(&self) -> String {
        // First attempt to retrieve the restricted error information.
        if !self.info.is_null() {
            let (code, message) = self.info.get_error_details();

            if self.code == code {
                return message.trim_end().to_owned();
            }
        }

        const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x0000_0100;
        const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x0000_1000;
        const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x0000_0200;
        let mut message = HeapString::new();

        // If that fails simply ask for the generic formatted message for the error code.
        unsafe {
            let size = FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null_mut(),
                self.code,
                0x0000_0400, // MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT)
                message.set_abi(),
                0,
                std::ptr::null_mut(),
            );

            String::from_utf16_lossy(std::slice::from_raw_parts(
                message.ptr as *const u16,
                size as usize,
            ))
            .trim_end()
            .to_owned()
        }
    }
}

impl ErrorCode {
    /// Returns `true` if `self` is a success code.
    #[inline]
    pub fn is_ok(self) -> bool {
        self.0 & 0x8000_0000 == 0
    }

    /// Returns `true` if `self` is a failure code.
    #[inline]
    pub fn is_err(self) -> bool {
        !self.is_ok()
    }

    /// Asserts that `self` is a success code.
    ///
    /// This will  invoke the `panic!` macro if `self` is a failure code and display
    /// the `HRESULT` value for diagnostics.
    #[inline]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    /// Converts the `ErrorCode` to `Result<()>`.
    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(Error::from(self))
        }
    }

    /// Calls `op` if `self` is a success code, otherwise returns `ErrorCode`
    /// converted to `Result<T>`.
    #[inline]
    pub fn and_then<F, T>(self, op: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(op())
    }

    /// Creates a failure code from GetLastError()
    #[inline]
    pub(crate) fn last_win32_error() -> Self {
        Self::from_win32(unsafe { GetLastError() })
    }

    /// Creates a failure code with the provided win32 error code.
    #[inline]
    pub(crate) fn from_win32(error: u32) -> Self {
        // equivalent to MAKE_WIN32_HRESULT(error)
        Self(0x8007_0000 | error & 0xFFFF)
    }
}

impl<T> std::convert::From<Result<T>> for ErrorCode {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.into();
        }

        ErrorCode(0)
    }
}

impl std::convert::From<Error> for ErrorCode {
    fn from(error: Error) -> Self {
        if let Some(info) = error.info.get_abi() {
            // Set the error information on the thread if the result is `Err`
            // so that the caller can pick it up.
            unsafe {
                let _ = SetRestrictedErrorInfo(info.as_raw() as _);
            }
        }

        return error.code();
    }
}

impl std::convert::From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        let mut info = IErrorInfo::default();

        // GetErrorInfo retrieves the error object from the thread, if any.
        unsafe {
            GetErrorInfo(0, info.set_abi() as _);
        }

        // Hopefully it has richer information via IRestrictedErrorInfo.
        let restricted = info.query::<IRestrictedErrorInfo>();

        if !restricted.is_null() {
            let capture = info.query::<ILanguageExceptionErrorInfo2>();

            // If it does (and running on a recent version of Windows) then
            // capture_propagation_context adds a breadcrumb to the error info
            // to make debugging easier.
            if !capture.is_null() {
                capture.capture_propagation_context();
            }

            return Self {
                code,
                info: restricted,
            };
        }

        let mut message = String::new();

        // Otherwise, for older APIs we attempt to get the error message.
        if !info.is_null() {
            message = info.get_description();
        }

        Self::new(code, &message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Error")
            .field("code", &format_args!("{:#010X}", self.code.0))
            .field("message", &self.message())
            .finish()
    }
}

#[repr(transparent)]
struct BString {
    ptr: *mut u16,
}

impl BString {
    pub fn new() -> BString {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ptr.is_null() || self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { SysStringLen(self.ptr) as usize }
    }

    pub fn clear(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                SysFreeString(self.ptr);
            }

            self.ptr = std::ptr::null_mut();
        }
    }
}

unsafe impl AbiTransferable for BString {
    type Abi = *mut u16;

    fn get_abi(&self) -> Self::Abi {
        self.ptr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.clear();
        &mut self.ptr
    }
}

impl Drop for BString {
    fn drop(&mut self) {
        self.clear();
    }
}

impl From<BString> for String {
    fn from(from: BString) -> Self {
        if from.is_empty() {
            return String::new();
        }

        unsafe {
            String::from_utf16_lossy(std::slice::from_raw_parts(
                from.ptr as *const u16,
                from.len(),
            ))
        }
    }
}

#[repr(transparent)]
struct HeapString {
    ptr: *mut u16,
}

impl HeapString {
    pub fn new() -> HeapString {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}

unsafe impl AbiTransferable for HeapString {
    type Abi = *mut u16;

    fn get_abi(&self) -> Self::Abi {
        self.ptr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        &mut self.ptr
    }
}

impl Drop for HeapString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                HeapFree(GetProcessHeap(), 0, self.ptr as _);
            }
        }
    }
}

#[repr(transparent)]
#[derive(Default, Clone)]
struct IErrorInfo {
    ptr: ComPtr<IErrorInfo>,
}

impl IErrorInfo {
    pub fn get_description(&self) -> String {
        let mut description = BString::new();
        match self.get_abi() {
            Some(p) => {
                unsafe {
                    (p.vtable().get_description)(Some(p), description.set_abi());
                }

                description.into()
            }
            None => String::new(),
        }
    }
}

unsafe impl ComInterface for IErrorInfo {
    type VTable = abi_IErrorInfo;

    fn iid() -> Guid {
        Guid::from_values(
            0x1CF2_B120,
            0x547D,
            0x101B,
            [0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19],
        )
    }
}

unsafe impl AbiTransferable for IErrorInfo {
    type Abi = RawComPtr<Self>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
struct abi_IErrorInfo {
    base__: [usize; 5],
    get_description: unsafe extern "system" fn(
        RawComPtr<IErrorInfo>,
        *mut <BString as AbiTransferable>::Abi,
    ) -> ErrorCode,
}

#[repr(transparent)]
#[derive(Default, Clone)]
struct IRestrictedErrorInfo {
    ptr: ComPtr<IRestrictedErrorInfo>,
}

impl IRestrictedErrorInfo {
    pub fn get_error_details(&self) -> (ErrorCode, String) {
        let mut fallback = BString::new();
        let mut message = BString::new();
        let mut unused = BString::new();
        let mut code = ErrorCode(0);
        let p = match self.ptr.get_abi() {
            Some(p) => p,
            None => return (code, String::new()),
        };

        unsafe {
            (p.vtable().get_error_details)(
                Some(p),
                fallback.set_abi(),
                &mut code,
                message.set_abi(),
                unused.set_abi(),
            );
        }

        let message = if !message.is_empty() {
            message
        } else {
            fallback
        };

        (code, message.into())
    }
}

unsafe impl ComInterface for IRestrictedErrorInfo {
    type VTable = abi_IRestrictedErrorInfo;

    fn iid() -> Guid {
        Guid::from_values(
            0x82BA_7092,
            0x4C88,
            0x427D,
            [0xA7, 0xBC, 0x16, 0xDD, 0x93, 0xFE, 0xB6, 0x7E],
        )
    }
}

unsafe impl AbiTransferable for IRestrictedErrorInfo {
    type Abi = RawComPtr<Self>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
struct abi_IRestrictedErrorInfo {
    base__: [usize; 3],
    get_error_details: unsafe extern "system" fn(
        RawComPtr<IRestrictedErrorInfo>,
        *mut <BString as AbiTransferable>::Abi,
        *mut ErrorCode,
        *mut <BString as AbiTransferable>::Abi,
        *mut <BString as AbiTransferable>::Abi,
    ) -> ErrorCode,
}

#[repr(transparent)]
#[derive(Default, Clone)]
struct ILanguageExceptionErrorInfo2 {
    ptr: ComPtr<ILanguageExceptionErrorInfo2>,
}

impl ILanguageExceptionErrorInfo2 {
    pub fn capture_propagation_context(&self) {
        if let Some(p) = self.get_abi() {
            unsafe {
                (p.vtable().capture_propagation_context)(p, std::ptr::null_mut());
            }
        }
    }
}

unsafe impl ComInterface for ILanguageExceptionErrorInfo2 {
    type VTable = abi_ILanguageExceptionErrorInfo2;

    fn iid() -> Guid {
        Guid::from_values(
            0x5746_E5C4,
            0x5B97,
            0x424C,
            [0xB6, 0x20, 0x28, 0x22, 0x91, 0x57, 0x34, 0xDD],
        )
    }
}

unsafe impl AbiTransferable for ILanguageExceptionErrorInfo2 {
    type Abi = RawComPtr<Self>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
struct abi_ILanguageExceptionErrorInfo2 {
    base__: [usize; 5],
    capture_propagation_context: unsafe extern "system" fn(
        NonNullRawComPtr<ILanguageExceptionErrorInfo2>,
        RawPtr,
    ) -> ErrorCode,
}
