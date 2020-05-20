use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IRestrictedErrorInfo {
    ptr: ComPtr<IRestrictedErrorInfo>,
}

impl IRestrictedErrorInfo {}

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
pub struct abi_IRestrictedErrorInfo {
    __base: [usize; 3],
    get_error_details: extern "system" fn(
        RawComPtr<IRestrictedErrorInfo>,
        *mut RawPtr,
        *mut i32,
        *mut RawPtr,
        *mut RawPtr,
    ) -> ErrorCode,
}
