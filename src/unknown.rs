use crate::*;

/// The [IUnknown interface](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IUnknown {
    ptr: ComPtr<IUnknown>,
}

unsafe impl RuntimeType for IUnknown {
    type Abi = <ComPtr<IUnknown> as RuntimeType>::Abi;

    fn abi(&self) -> Self::Abi {
        self.ptr.abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

unsafe impl ComInterface for IUnknown {
    type VTable = abi_IUnknown;
    const GUID: Guid = Guid::from_values(
        0x00000000,
        0x0000,
        0x0000,
        [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
    );
}

#[repr(C)]
pub struct abi_IUnknown {
    pub(crate) query:
        extern "system" fn(<IUnknown as RuntimeType>::Abi, &Guid, *mut RawPtr) -> ErrorCode,
    pub(crate) addref: extern "system" fn(<IUnknown as RuntimeType>::Abi) -> u32,
    pub(crate) release: extern "system" fn(<IUnknown as RuntimeType>::Abi) -> u32,
}
