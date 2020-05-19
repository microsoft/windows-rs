#![allow(overflowing_literals)]
use core::fmt;
use crate::*;

/// An alias for `std::result::Result<T, winrt::Error>`
#[must_use]
pub type Result<T> = std::result::Result<T, Error>;

impl<T> std::convert::From<Result<T>> for ErrorCode {
    fn from(result: Result<T>) -> Self {
        if let Err(error) = result {
            return error.code();
        }

        ErrorCode(0)
    }
}

/// A WinRT related error
#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
    info: Option<IRestrictedErrorInfo>,
}

impl Error {
    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

type HRESULT = i32;

/// The ErrorCode (a.k.a HRESULT) of an error
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ErrorCode(pub HRESULT);

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
            let mut restricted_error_info = IRestrictedErrorInfo::default();
            unsafe {
                if GetRestrictedErrorInfo(&mut restricted_error_info).is_ok() {
                    return Err(Error { code: self, info: Some(restricted_error_info) })
                }
            }
            Err(Error { code: self, info: None })
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
#[derive(Default, Clone)]
struct IRestrictedErrorInfo {
    ptr: ComPtr<IRestrictedErrorInfo>
}

impl IRestrictedErrorInfo {
    pub fn get_error_details(&self) -> std::result::Result<(BStr, ErrorCode, BStr, BStr), ErrorCode> {
        if self.ptr.is_null() {
            panic!("The `this` pointer was null when calling method");
        }

        let mut desc = BStr::default();
        let mut error_code = ErrorCode(0);
        let mut restricted_description = BStr::default();
        let mut capability_sid = BStr::default();

        let err = unsafe {
            ((*(*(self.ptr.as_raw()))).get_error_details)(
                self.ptr.as_raw(),
                &mut desc,
                &mut error_code,
                &mut restricted_description,
                &mut capability_sid,
            )
        };

        // Avoid using `ok` or `and_then`, as those will trigger a roundtrip
        // into `get_error_details()` again, causing an infinite recursion!
        if err.is_err() {
            return Err(err)
        }

        Ok((desc, error_code, restricted_description, capability_sid))
    }
}

unsafe impl ComInterface for IRestrictedErrorInfo {
    type VTable = abi_IRestrictedErrorInfo;
    fn iid() -> Guid {
        Guid::from_values(
            0x82ba7092,
            0x4c88,
            0x427d,
            [0xa7, 0xbc, 0x16, 0xdd, 0x93, 0xfe, 0xb6, 0x7e])
    }
}

impl fmt::Debug for IRestrictedErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dbg = f.debug_struct("IRestrictedErrorInfo");
        if let Ok((desc, error_code, restricted_desc, capability_sid)) = self.get_error_details() {
                dbg
                    .field("description", &desc)
                    .field("error_code", &error_code)
                    .field("restricted_description", &restricted_desc)
                    .field("capability_sid", &capability_sid)
                    .finish()
        } else {
            dbg.finish()
        }
    }
}

#[repr(C)]
struct abi_IRestrictedErrorInfo {
    __base: [usize; 3],
    get_error_details: extern "system" fn(RawComPtr<IRestrictedErrorInfo>, *mut BStr, *mut ErrorCode, *mut BStr, *mut BStr) -> ErrorCode,
    //get_reference: extern "system" fn(RawComPtr<IRestrictedErrorInfo>, ) -> ErrorCode,
}

#[link(name = "mincore")]
extern "system" {
    fn GetRestrictedErrorInfo(restricted_error_info: *mut IRestrictedErrorInfo) -> ErrorCode;
}