use crate::*;

pub type RawPtr = *mut std::ffi::c_void;

#[repr(C)]
pub struct ComPtr(RawPtr);

impl ComPtr {
    pub fn query<I: TypeGuid + From<RawPtr>>(&self) -> I {
        let mut ptr = std::ptr::null_mut();
        if !self.0.is_null() {
            (self.deref::<IUnknown>().query)(self.0, I::type_guid(), &mut ptr);
        }
        ptr.into()
    }

    pub fn addref(&self) -> RawPtr {
        if !self.0.is_null() {
            (self.deref::<IUnknown>().addref)(self.0);
        }
        self.0
    }

    pub fn get(&self) -> RawPtr {
        self.0
    }

    pub fn set(&mut self) -> *mut RawPtr {
        if !self.0.is_null() {
            (self.deref::<IUnknown>().release)(self.0);
            self.0 = std::ptr::null_mut();
        }
        &mut self.0
    }

    pub fn deref<T>(&self) -> &T {
        unsafe { &(*(*(self.0 as *const *const T))) }
    }
}

impl Default for ComPtr {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

impl Clone for ComPtr {
    fn clone(&self) -> ComPtr {
        if !self.0.is_null() {
            (self.deref::<IUnknown>().addref)(self.0);
        }
        ComPtr(self.0)
    }
}

impl std::ops::Deref for ComPtr {
    type Target = RawPtr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(C)]
struct IUnknown {
    query: extern "system" fn(RawPtr, &Guid, *mut RawPtr) -> ErrorCode,
    addref: extern "system" fn(RawPtr) -> u32,
    release: extern "system" fn(RawPtr) -> u32,
}
