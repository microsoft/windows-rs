use crate::bstring::*;
use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IErrorInfo {
    ptr: ComPtr<IErrorInfo>,
}

impl IErrorInfo {
    pub fn get_description(&self) -> Result<String> {
        let mut description = BString::new();

        unsafe {
            ((*(*(self.ptr.as_raw()))).get_description)(self.ptr.as_raw(), description.set_abi())
                .and_then(|| description.into())
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
pub struct abi_IErrorInfo {
    __base: [usize; 5],
    get_description: extern "system" fn(RawComPtr<IErrorInfo>, *mut RawPtr) -> ErrorCode,
}
