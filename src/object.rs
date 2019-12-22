#![allow(dead_code)]

use crate::*;

#[repr(C)]
pub struct Object {
    pub ptr: *mut std::ffi::c_void,
}

impl TypeInterface for Object {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(
            0xAF86E2E0, 0xB12D, 0x4C6A,
            &[0x9C,0x5A,0xD7,0xAA,0x65,0x10,0x1E,0x90],
        );
        &GUID
    }
}

impl AsAbi for Object {
    type In = *const std::ffi::c_void;
    type Out = *mut *mut std::ffi::c_void;

    fn as_abi_in(&self) -> Self::In {
        self.ptr
    }

    fn as_abi_out(&mut self) -> Self::Out {
        debug_assert!(self.ptr.is_null());
        &mut self.ptr
    }
}

impl From<*mut std::ffi::c_void> for Object {
    fn from(ptr: *mut std::ffi::c_void) -> Self {
        Self { ptr }
    }
}
