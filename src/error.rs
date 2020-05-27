#![allow(overflowing_literals)]

use crate::runtime::*;
use crate::*;

/// The ErrorCode (a.k.a HRESULT) of an error
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ErrorCode(pub i32);

/// A WinRT related error
pub struct Error {
    code: ErrorCode,
    info: IRestrictedErrorInfo,
}

/// An alias for `std::result::Result<T, winrt::Error>`
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl<T> std::convert::From<Result<T>> for ErrorCode {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            if let Some(info) = error.info.as_raw() {
                unsafe {
                    SetRestrictedErrorInfo(info.as_ptr() as _);
                }
            }

            return error.code();
        }

        ErrorCode(0)
    }
}

impl std::convert::From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        let mut info = IErrorInfo::default();

        unsafe {
            GetErrorInfo(0, info.set_abi() as _);
        }

        let restricted = info.query::<IRestrictedErrorInfo>();

        if !restricted.is_null() {
            let capture = info.query::<ILanguageExceptionErrorInfo2>();

            if !capture.is_null() {
                capture.capture_propagation_context();
            }

            return Self {
                code,
                info: restricted,
            };
        }

        let mut message = String::new();

        if !info.is_null() {
            message = info.get_description();
        }

        Self::new(code, &message)
    }
}

impl Error {
    pub fn new(code: ErrorCode, message: &str) -> Self {
        let message: HString = message.into();

        unsafe {
            RoOriginateError(code, message.abi() as _);
        }

        let mut info = IErrorInfo::default();

        unsafe {
            GetErrorInfo(0, info.set_abi() as _);
        }

        let info = info.query::<IRestrictedErrorInfo>();
        Self { code, info }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub fn message(&self) -> String {
        if !self.info.is_null() {
            let (code, message) = self.info.get_error_details();

            if self.code == code {
                return message.trim_end().to_owned();
            }
        }

        const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
        const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
        const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;
        let mut message = HeapString::new();

        unsafe {
            let size = FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                std::ptr::null_mut(),
                self.code,
                0x00000400, // MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT)
                &mut message.ptr,
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

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("Error")
            .field("code", &format_args!("{:#010X}", self.code.0))
            .field("message", &self.message())
            .finish()
    }
}

impl ErrorCode {
    #[inline]
    pub fn is_ok(self) -> bool {
        self.0 >= 0
    }

    #[inline]
    pub fn is_err(self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn unwrap(self) {
        assert!(self.is_ok(), "HRESULT 0x{:X}", self.0);
    }

    #[inline]
    pub fn ok(self) -> Result<()> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(Error::from(self))
        }
    }

    #[inline]
    pub fn and_then<F, T>(self, value: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        self.ok()?;
        Ok(value())
    }

    pub(crate) const NOT_INITIALIZED: ErrorCode = ErrorCode(0x8004_01F0);
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

    pub fn set_abi(&mut self) -> *mut *mut u16 {
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
    pub fn set_abi(&mut self) -> *mut RawComPtr<Self> {
        self.ptr.set_abi()
    }

    pub fn get_description(&self) -> String {
        let mut description = BString::new();
        match self.ptr.as_raw() {
            Some(p) => {
                unsafe {
                    ((*(*p.as_ptr()).as_ptr()).get_description)(
                        self.ptr.as_raw(),
                        description.set_abi(),
                    );
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

#[repr(C)]
struct abi_IErrorInfo {
    __base: [usize; 5],
    get_description: extern "system" fn(RawComPtr<IErrorInfo>, *mut *mut u16) -> ErrorCode,
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
        let p = match self.ptr.as_raw() {
            Some(p) => p,
            None => return (code, String::new()),
        };

        unsafe {
            ((*(*p.as_ptr()).as_ptr()).get_error_details)(
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

#[repr(C)]
struct abi_IRestrictedErrorInfo {
    __base: [usize; 3],
    get_error_details: extern "system" fn(
        RawComPtr<IRestrictedErrorInfo>,
        *mut *mut u16,
        *mut ErrorCode,
        *mut *mut u16,
        *mut *mut u16,
    ) -> ErrorCode,
}

#[repr(transparent)]
#[derive(Default, Clone)]
struct ILanguageExceptionErrorInfo2 {
    ptr: ComPtr<ILanguageExceptionErrorInfo2>,
}

impl ILanguageExceptionErrorInfo2 {
    pub fn capture_propagation_context(&self) {
        if let Some(p) = self.ptr.as_raw() {
            unsafe {
                ((*(*p.as_ptr()).as_ptr()).capture_propagation_context)(
                    self.ptr.as_raw(),
                    std::ptr::null_mut(),
                );
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

#[repr(C)]
struct abi_ILanguageExceptionErrorInfo2 {
    __base: [usize; 5],
    capture_propagation_context:
        extern "system" fn(RawComPtr<ILanguageExceptionErrorInfo2>, RawPtr) -> ErrorCode,
}
