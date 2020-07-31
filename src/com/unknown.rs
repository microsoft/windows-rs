use crate::*;

/// The [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown) interface
/// is the interface that all other COM and WinRT interfaces inherit from (directly or indirectly).
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IUnknown {
    ptr: ComPtr<IUnknown>,
}

unsafe impl ComInterface for IUnknown {
    type VTable = abi_IUnknown;

    fn iid() -> Guid {
        Guid::from_values(
            0x0000_0000,
            0x0000,
            0x0000,
            [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    }
}

unsafe impl AbiTransferable for IUnknown {
    type Abi = RawComPtr<Self>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
pub struct abi_IUnknown {
    pub unknown_query_interface:
        unsafe extern "system" fn(NonNullRawComPtr<IUnknown>, &Guid, *mut RawPtr) -> ErrorCode,
    pub unknown_add_ref: extern "system" fn(NonNullRawComPtr<IUnknown>) -> u32,
    pub unknown_release: extern "system" fn(NonNullRawComPtr<IUnknown>) -> u32,
}
