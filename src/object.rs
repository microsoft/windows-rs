#![allow(dead_code)]

use crate::*;
use com::interfaces::IUnknown;

#[repr(C)]
#[derive(Clone)]
pub struct Object {
    ptr: com::ComPtr<dyn abi::IInspectable>,
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        use abi::IInspectable;
        let mut class_name = std::ptr::null_mut();

        unsafe {
            self.ptr
                .get_runtime_class_name(&mut class_name)
                .ok_or(std::mem::transmute(class_name))
        }
    }
}

impl QueryType for Object {
    fn type_guid() -> &'static Guid {
        use com::ComInterface;
        static GUID: Guid = Guid(abi::IInspectable::IID);
        &GUID
    }
}

impl RuntimeType for Object {
    type Abi = RawPtr;

    fn abi(&self) -> Self::Abi {
        self.ptr.as_raw() as RawPtr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        unsafe { self.ptr.release() };
        &mut self.abi()
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
