use crate::*;

/// A WinRT Object
///
/// Objects implement the [IInspectable interface](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct Object {
    ptr: ComPtr<Object>,
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        let this = self.ptr.as_vtable();
        if this.is_null() {
            panic!("The `this` pointer was null when calling method");
        }
        let mut string = HString::default();
        unsafe {
            ((*(*(this))).type_name)(this, string.set_abi()).ok()?;
        }
        Ok(string)
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
    type Abi = ComInterfacePtr<Object>;

    fn abi(&self) -> Self::Abi {
        self.ptr.as_vtable()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
pub struct abi_IInspectable {
    __base: [usize; 4],
    type_name: extern "system" fn(
        *const *const object::abi_IInspectable,
        *mut <HString as RuntimeType>::Abi,
    ) -> ErrorCode,
}
