use crate::foundation::{IReference, PropertyValue};
use crate::*;

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
///
/// Objects implement the [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable) interface.
#[repr(transparent)]
#[derive(Default, Clone)]
pub struct Object {
    ptr: ComPtr<Object>,
}

impl Object {
    /// Returns the fully-qualified type name of the WinRT object.
    pub fn type_name(&self) -> Result<HString> {
        let this = self
            .get_abi()
            .expect("The `this` pointer was null when calling method");

        let mut string = HString::default();

        unsafe {
            (this.vtable().inspectable_type_name)(this, string.set_abi()).ok()?;
        }

        Ok(string)
    }
}

unsafe impl ComInterface for Object {
    type VTable = abi_IInspectable;

    const IID: Guid = {
        Guid::from_values(
            0xAF86_E2E0,
            0xB12D,
            0x4C6A,
            [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
        )
    };
}

unsafe impl RuntimeType for Object {
    const SIGNATURE: crate::ConstBuffer =
        crate::ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

unsafe impl AbiTransferable for Object {
    type Abi = RawComPtr<Object>;

    fn get_abi(&self) -> Self::Abi {
        self.ptr.get_abi()
    }

    fn set_abi(&mut self) -> *mut Self::Abi {
        self.ptr.set_abi()
    }
}

#[repr(C)]
pub struct abi_IInspectable {
    pub unknown: abi_IUnknown,

    // TODO: remove inspectable_ preamble as it's not longer needed
    pub inspectable_iids:
        unsafe extern "system" fn(NonNullRawComPtr<Object>, *mut u32, *mut *mut Guid) -> ErrorCode,
    pub inspectable_type_name: unsafe extern "system" fn(
        NonNullRawComPtr<Object>,
        *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub inspectable_trust_level:
        unsafe extern "system" fn(NonNullRawComPtr<Object>, *mut i32) -> ErrorCode,
}

macro_rules! primitive_boxed_type {
    ($(($t:ty, $m:ident)),+) => {
        $(impl std::convert::TryFrom<$t> for Object {
            type Error = Error;
            fn try_from(value: $t) -> Result<Self> {
                PropertyValue::$m(value)
            }
        }
        impl std::convert::TryFrom<Object> for $t {
            type Error = Error;
            fn try_from(value: Object) -> Result<Self> {
                <Object as ComInterface>::try_query::<IReference<$t>>(&value)?.value()
            }
        }
        impl std::convert::TryFrom<&Object> for $t {
            type Error = Error;
            fn try_from(value: &Object) -> Result<Self> {
                <Object as ComInterface>::try_query::<IReference<$t>>(value)?.value()
            }
        })*
    };
}

primitive_boxed_type! {
    (bool, create_boolean),
    (u8, create_uint8),
    (i16, create_int16),
    (u16, create_uint16),
    (i32, create_int32),
    (u32, create_uint32),
    (i64, create_int64),
    (u64, create_uint64),
    (f32, create_single),
    (f64, create_double)
}

impl std::convert::TryFrom<&str> for Object {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        PropertyValue::create_string(value)
    }
}
impl std::convert::TryFrom<HString> for Object {
    type Error = Error;
    fn try_from(value: HString) -> Result<Self> {
        PropertyValue::create_string(value)
    }
}
impl std::convert::TryFrom<&HString> for Object {
    type Error = Error;
    fn try_from(value: &HString) -> Result<Self> {
        PropertyValue::create_string(value)
    }
}
impl std::convert::TryFrom<Object> for HString {
    type Error = Error;
    fn try_from(value: Object) -> Result<Self> {
        <Object as ComInterface>::try_query::<IReference<HString>>(&value)?.value()
    }
}
impl std::convert::TryFrom<&Object> for HString {
    type Error = Error;
    fn try_from(value: &Object) -> Result<Self> {
        <Object as ComInterface>::try_query::<IReference<HString>>(value)?.value()
    }
}
