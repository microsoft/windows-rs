#![allow(dead_code)]

use crate::*;

#[repr(C)]
#[derive(Default, Clone)]
pub struct Object {
    ptr: ComPtr,
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ((*(*(self.ptr.get() as *const *const IInspectable))).type_name)(
                self.ptr.get(),
                &mut ptr,
            )
            .ok_or(std::mem::transmute(ptr))
        }
    }
}

impl QueryType for Object {
    fn type_guid() -> &'static Guid {
        static GUID: Guid = Guid::from_values(
            0xAF86E2E0,
            0xB12D,
            0x4C6A,
            &[0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
        );
        &GUID
    }
}

impl RuntimeType for Object {
    type Abi = RawPtr;

    fn abi(&self) -> Self::Abi {
        self.ptr.get()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set()
    }
}

#[repr(C)]
struct IInspectable {
    __0: usize,
    __1: usize,
    __2: usize,
    __3: usize,
    type_name: extern "system" fn(RawPtr, *mut RawPtr) -> ErrorCode,
}
