use crate::*;

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct IErrorInfo(IUnknown);

#[repr(C)]
pub struct IErrorInfo_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
    pub extern "system" fn(this: RawPtr, guid: *mut Guid) -> ErrorCode, // GetGUID
    pub extern "system" fn(this: RawPtr, source: *mut RawPtr) -> ErrorCode, // GetSource
    pub extern "system" fn(this: RawPtr, description: *mut RawPtr) -> ErrorCode, // GetDescription
    pub extern "system" fn(this: RawPtr, help: *mut RawPtr) -> ErrorCode, // GetHelpFile
    pub extern "system" fn(this: RawPtr, context: *mut u32) -> ErrorCode, // GetHelpContext
);

unsafe impl Interface for IErrorInfo {
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

impl std::fmt::Debug for IErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl IErrorInfo {
    pub fn from_thread() -> Result<Self> {
        let mut result = None;
        unsafe {
            GetErrorInfo(0, &mut result);
        }
        result.ok_or_else(|| Error::just_code(ErrorCode::E_NOINTERFACE))
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
