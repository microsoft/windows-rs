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

// unsafe_query only exists to support generic interface queries that aren't yet supported by Rust because
// it lacks good const function support to work out the guids in a generic and thus type safe manner. Once
// const function support arrives, we should be able to remove this function and rely on ComInterface to
// calculate the guid for all types.
pub unsafe fn unsafe_query<From: ComInterface, Into: ComInterface>(
    from: &From,
    guid: &Guid,
) -> Into {
    let mut into = std::ptr::null_mut();
    let from: IUnknownPtr = std::mem::transmute_copy(from);
    if !from.is_null() {
        ((*(*(from as IUnknownPtr))).query)(from, guid, &mut into);
    }
    std::mem::transmute_copy(&into)
}

pub fn safe_query<From: ComInterface, Into: ComInterface>(from: &From) -> Into {
    unsafe { unsafe_query(from, &Into::GUID) }
}

#[repr(C)]
pub struct abi_IUnknown {
    pub(crate) query: extern "system" fn(IUnknownPtr, &Guid, *mut RawPtr) -> ErrorCode,
    pub(crate) addref: extern "system" fn(IUnknownPtr) -> u32,
    pub(crate) release: extern "system" fn(IUnknownPtr) -> u32,
}
