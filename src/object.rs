#![allow(dead_code)]

use crate::*;

#[repr(C)]
pub struct Object {
    pub ptr: RawPtr,
}

impl TypeGuid for Object {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(0xAF86E2E0, 0xB12D, 0x4C6A, &[0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90]);
        &GUID
    }
}

impl RuntimeType for Object {
    type Abi = RawPtr;

    fn as_abi(&self) -> Self::Abi {
        self.ptr
    }

    fn as_abi_mut(&mut self) -> *mut Self::Abi {
        IUnknown::release(self.ptr);
        self.ptr = std::ptr::null_mut();
        &mut self.ptr
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

impl From<RawPtr> for Object {
    fn from(value: RawPtr) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
