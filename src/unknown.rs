use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct IUnknown {
    ptr: ComPtr<IUnknown>,
}

impl IUnknown {
    pub fn get(&self) -> RawPtr {
        self.ptr.get() as RawPtr
    }

    pub fn set(&mut self) -> *mut RawPtr {
        self.ptr.set() as *mut RawPtr
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

type IUnknownPtr = *const *const <IUnknown as ComInterface>::VTable;

#[repr(C)]
pub struct abi_IUnknown {
    pub(crate) query: extern "system" fn(IUnknownPtr, &Guid, *mut RawPtr) -> ErrorCode,
    pub(crate) addref: extern "system" fn(IUnknownPtr) -> u32,
    pub(crate) release: extern "system" fn(IUnknownPtr) -> u32,
}
