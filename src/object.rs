use crate::*;

#[repr(transparent)]
#[derive(Default, Clone)]
pub struct Object {
    ptr: IUnknown,
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            ((*(*(self.ptr.get() as *const *const abi_IInspectable))).type_name)(
                self.ptr.get(),
                &mut ptr,
            )
            .and_then(|| std::mem::transmute(ptr))
        }
    }
}

unsafe impl TypeGuid for Object {
    const TYPE_GUID: Guid = Guid::from_values(
        0xAF86_E2E0,
        0xB12D,
        0x4C6A,
        [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
    );
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
struct abi_IInspectable {
    __base: [usize; 4],
    type_name: extern "system" fn(RawPtr, *mut RawPtr) -> ErrorCode,
}
