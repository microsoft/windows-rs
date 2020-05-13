use crate::*;

/// The [IUnknown interface](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IUnknown {
    ptr: ComPtr<IUnknown>,
}

impl IUnknown {
    pub fn set_abi(&mut self) -> *mut RawComPtr<Self> {
        self.ptr.set_abi()
    }
}

unsafe impl ComInterface for IUnknown {
    type VTable = abi_IUnknown;

    fn iid() -> Guid {
        Guid::from_values(
            0x00000000,
            0x0000,
            0x0000,
            [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    }
}

#[repr(C)]
pub struct abi_IUnknown {
    pub unknown_query_interface:
        extern "system" fn(RawComPtr<IUnknown>, &Guid, *mut RawPtr) -> ErrorCode,
    pub unknown_add_ref: extern "system" fn(RawComPtr<IUnknown>) -> u32,
    pub unknown_release: extern "system" fn(RawComPtr<IUnknown>) -> u32,
}
