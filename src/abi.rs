use crate::*;
use com::interfaces::IUnknown;

pub type RawPtr = *mut std::ffi::c_void;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct ComPtr {
    pub ptr: Option<com::ComPtr<dyn IUnknown>>,
}

pub unsafe fn query<I: QueryType>(ptr: RawPtr) -> RawPtr {
    if ptr.is_null() {
        return ptr;
    }
    let mut result = std::ptr::null_mut();
    let ptr = com::ComPtr::<dyn IUnknown>::new(ptr as *mut _);
    ptr.query_interface(I::type_guid() as *const Guid as *const _, &mut result);
    result
}

impl ComPtr {
    unsafe fn from_raw(ptr: RawPtr) -> Self {
        let ptr = if ptr.is_null() {
            None
        } else {
            Some(com::ComPtr::new(ptr as *mut _))
        };
        Self { ptr }
    }

    pub fn query<I: QueryType>(&self) -> Self {
        unsafe {
            let ptr = query::<I>(self.get());
            Self::from_raw(ptr)
        }
    }

    pub fn get(&self) -> RawPtr {
        self.ptr
            .as_ref()
            .map(|p| p.as_raw())
            .unwrap_or_else(std::ptr::null_mut) as RawPtr
    }

    pub fn set(&mut self) -> *mut RawPtr {
        if let Some(ref ptr) = self.ptr {
            unsafe { ptr.release() };
            self.ptr = None;
        }

        &mut self.ptr as *mut _ as *mut RawPtr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }
}

impl Drop for ComPtr {
    fn drop(&mut self) {
        if let Some(ref ptr) = self.ptr {
            unsafe { ptr.release() };
        }
    }
}
