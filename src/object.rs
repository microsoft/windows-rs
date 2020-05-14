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
        let this = self.ptr.as_raw();
        if this.is_null() {
            panic!("The `this` pointer was null when calling method");
        }
        let mut string = HString::default();
        unsafe {
            ((*(*(this))).inspectable_type_name)(this, string.set_abi()).ok()?;
        }
        Ok(string)
    }
}

unsafe impl ComInterface for Object {
    type VTable = abi_IInspectable;

    fn iid() -> Guid {
        Guid::from_values(
            0xAF86_E2E0,
            0xB12D,
            0x4C6A,
            [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
        )
    }
}

unsafe impl RuntimeType for Object {
    type Abi = RawComPtr<Object>;

    fn signature() -> String {
        "cinterface(IInspectable)".to_owned()
    }

    fn abi(&self) -> Self::Abi {
        self.ptr.as_raw()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
pub struct abi_IInspectable {
    pub unknown_query_interface:
        extern "system" fn(RawComPtr<IUnknown>, &Guid, *mut RawPtr) -> ErrorCode,
    pub unknown_add_ref: extern "system" fn(RawComPtr<IUnknown>) -> u32,
    pub unknown_release: extern "system" fn(RawComPtr<IUnknown>) -> u32,

    pub inspectable_iids:
        extern "system" fn(RawComPtr<Object>, *mut u32, *mut *mut Guid) -> ErrorCode,
    pub inspectable_type_name:
        extern "system" fn(RawComPtr<Object>, *mut <HString as RuntimeType>::Abi) -> ErrorCode,
    pub inspectable_trust_level: extern "system" fn(RawComPtr<Object>, *mut i32) -> ErrorCode,
}
