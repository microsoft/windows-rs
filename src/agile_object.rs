use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IAgileObject {
    ptr: ComPtr<IAgileObject>,
}

unsafe impl ComInterface for IAgileObject {
    type VTable = abi_IAgileObject;

    fn iid() -> Guid {
        Guid::from_values(
            0x94EA2B94,
            0xE9CC,
            0x49E0,
            [0xC0, 0xFF, 0xEE, 0x64, 0xCA, 0x8F, 0x5B, 0x90],
        )
    }
}

#[repr(C)]
pub struct abi_IAgileObject {
    pub unknown_query_interface:
        extern "system" fn(RawComPtr<IUnknown>, &Guid, *mut RawPtr) -> ErrorCode,
    pub unknown_add_ref: extern "system" fn(RawComPtr<IUnknown>) -> u32,
    pub unknown_release: extern "system" fn(RawComPtr<IUnknown>) -> u32,
}
