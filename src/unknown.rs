use crate::*;

#[repr(transparent)]
pub struct IUnknown {
    ptr: RawPtr,
}

impl IUnknown {
    pub fn get(&self) -> RawPtr {
        self.ptr
    }

    pub fn set(&mut self) -> *mut RawPtr {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.ptr as *const *const abi_IUnknown))).release)(self.ptr);
                self.ptr = std::ptr::null_mut();
            }
        }
        &mut self.ptr
    }
}

impl Default for IUnknown {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
        }
    }
}

impl Clone for IUnknown {
    fn clone(&self) -> IUnknown {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.ptr as *const *const abi_IUnknown))).addref)(self.ptr);
            }
        }
        IUnknown { ptr: self.ptr }
    }
}

impl Drop for IUnknown {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*(self.ptr as *const *const abi_IUnknown))).release)(self.ptr);
            }
        }
    }
}

// unsafe_query only exists to support generic interface queries that aren't yet supported by Rust because
// it lacks good const function support to work out the guids in a generic and thus type safe manner. Once
// const function support arrives, we should be able to remove this function and rely on type_guid to
// calculate the guid for all types.
pub unsafe fn unsafe_query<From: TypeGuid, Into: TypeGuid>(from: &From, guid: &Guid) -> Into {
    let mut into = std::ptr::null_mut();
    let from: RawPtr = std::mem::transmute_copy(from);
    if !from.is_null() {
        ((*(*(from as *const *const abi_IUnknown))).query)(from, guid, &mut into);
    }
    std::mem::transmute_copy(&into)
}

pub fn safe_query<From: TypeGuid, Into: TypeGuid>(from: &From) -> Into {
    unsafe { unsafe_query(from, Into::type_guid()) }
}

#[repr(C)]
struct abi_IUnknown {
    query: extern "system" fn(RawPtr, &Guid, *mut RawPtr) -> ErrorCode,
    addref: extern "system" fn(RawPtr) -> u32,
    release: extern "system" fn(RawPtr) -> u32,
}
