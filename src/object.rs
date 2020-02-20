#![allow(dead_code)]

use crate::*;

#[repr(C)]
#[derive(Default, Clone)]
pub struct Object {
    ptr: ComPtr,
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        use abi::*;
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let abi = com::InterfacePtr::<dyn abi::IInspectable>::new(self.ptr.get() as *mut _);

            abi.get_runtime_class_name(&mut ptr)
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

pub mod abi {
    use com::interfaces::IUnknown;

    #[com::com_interface("AF86E2E0-B12D-4c6a-9C5A-D7AA65101E90")]
    pub trait IInspectable: IUnknown {
        unsafe fn get_iids(
            &self,
            iid_count: *mut u32,
            iids: *mut *mut crate::Guid,
        ) -> crate::ErrorCode;
        unsafe fn get_runtime_class_name(&self, class_name: *mut crate::RawPtr)
            -> crate::ErrorCode;
        unsafe fn get_trust_level(&self, trust_level: crate::RawPtr) -> crate::ErrorCode;
    }
}
