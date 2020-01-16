#![allow(dead_code)]

use crate::*;

#[repr(C)]
pub struct Object {
    pub ptr: handle,
}

impl TypeGuid for Object {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(0xAF86E2E0, 0xB12D, 0x4C6A, &[0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90]);
        &GUID
    }
}

impl AsAbi for Object {
    type In = *const std::ffi::c_void;
    type Out = *mut handle;

    fn as_abi_in(&self) -> Self::In {
        self.ptr
    }

    fn as_abi_out(&mut self) -> Self::Out {
        debug_assert!(self.ptr.is_null());
        &mut self.ptr
    }

    fn detach_abi(&mut self) -> Self::In {
        let ptr = self.as_abi_in();
        self.ptr = std::ptr::null_mut();
        ptr
    }
}

impl From<handle> for Object {
    fn from(ptr: handle) -> Self {
        Self { ptr }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self { ptr: std::ptr::null_mut() }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        IUnknown::release(self.ptr);
    }
}
