use crate::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct IErrorInfo(IUnknown);

#[repr(C)]
pub struct IErrorInfo_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
    pub extern "system" fn(this: RawComPtr, guid: *mut Guid) -> ErrorCode, // GetGUID
    pub extern "system" fn(this: RawComPtr, source: *mut RawPtr) -> ErrorCode, // GetSource
    pub extern "system" fn(this: RawComPtr, description: *mut RawPtr) -> ErrorCode, // GetDescription
    pub extern "system" fn(this: RawComPtr, help: *mut RawPtr) -> ErrorCode,        // GetHelpFile
    pub extern "system" fn(this: RawComPtr, context: *mut u32) -> ErrorCode, // GetHelpContext
);

unsafe impl ComInterface for IErrorInfo {
    type Vtable = IErrorInfo_vtable;

    const IID: Guid = {
        Guid::from_values(
            0x1CF2_B120,
            0x547D,
            0x101B,
            [0x8E, 0x65, 0x08, 0x00, 0x2B, 0x2B, 0xD1, 0x19],
        )
    };
}

unsafe impl GetAbi for IErrorInfo {
    type Abi = RawComPtr;

    unsafe fn get_abi(&self) -> RawComPtr {
        self.0.get_abi()
    }
}

impl IErrorInfo {
    pub fn from_thread() -> Option<Self> {
        let mut result = None;
        unsafe {
            GetErrorInfo(0, &mut result);
        }
        result
    }

    pub fn description(&self) -> String {
        let mut value = BString::new();
        unsafe {
            (self.vtable().5)(self.get_abi(), value.set_abi());
        }
        value.into()
    }
}

#[link(name = "oleaut32")]
extern "system" {
    fn GetErrorInfo(reserved: u32, info: &mut Option<IErrorInfo>) -> ErrorCode;
}
