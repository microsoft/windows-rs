use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct Object {
    ptr: ComPtr<Object>,
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ((*(*(self.ptr.get()))).type_name)(self.ptr.get(), &mut ptr)
                .and_then(|| std::mem::transmute(ptr))
        }
    }
}

unsafe impl ComInterface for Object {
    type VTable = abi_IInspectable;
    const GUID: Guid = Guid::from_values(
        0xAF86_E2E0,
        0xB12D,
        0x4C6A,
        [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
    );
}

unsafe impl RuntimeType for Object {
    type Abi = RawPtr;

    fn abi(&self) -> Self::Abi {
        self.ptr.get() as RawPtr
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set() as *mut RawPtr
    }
}

#[repr(C)]
pub struct abi_IInspectable {
    __base: [usize; 4],
    type_name: extern "system" fn(*const *const object::abi_IInspectable, *mut RawPtr) -> ErrorCode,
}
