use crate::*;
use std::convert::TryInto;

/// Provides detailed error information. `IErrorInfo` represents the
/// [IErrorInfo](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-ierrorinfo)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IErrorInfo(IUnknown);

#[repr(C)]
pub struct IErrorInfo_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr, guid: *mut Guid) -> ErrorCode, // GetGUID
    pub unsafe extern "system" fn(this: RawPtr, source: *mut RawPtr) -> ErrorCode, // GetSource
    pub unsafe extern "system" fn(this: RawPtr, description: *mut RawPtr) -> ErrorCode, // GetDescription
    pub unsafe extern "system" fn(this: RawPtr, help: *mut RawPtr) -> ErrorCode, // GetHelpFile
    pub unsafe extern "system" fn(this: RawPtr, context: *mut u32) -> ErrorCode, // GetHelpContext
);

impl IErrorInfo {
    /// Retrieves any error information stored on the calling thread. An error code indicates the
    /// absence of error information.
    pub fn from_thread() -> Result<Self> {
        let mut result = None;

        unsafe { GetErrorInfo(0, &mut result).and_some(result) }
    }

    /// Gets a description of the error.
    pub fn description(&self) -> String {
        let mut value = BString::new();

        unsafe {
            let _ = (self.vtable().5)(self.abi(), value.set_abi());
        }

        value.try_into().unwrap_or_default()
    }
}

unsafe impl Interface for IErrorInfo {
    type Vtable = IErrorInfo_vtable;

    const IID: Guid = Guid::from_values(
        0x1CF2_B120,
        0x547D,
        0x101B,
        [0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19],
    );
}

impl std::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[link(name = "oleaut32")]
extern "system" {
    fn GetErrorInfo(reserved: u32, info: &mut Option<IErrorInfo>) -> ErrorCode;
}
