use crate::foundation::{IReference, PropertyValue};
use crate::*;

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
///
/// Objects implement the [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable) interface.

#[allow(non_camel_case_types)]
pub type Object_GetIids =
    extern "system" fn(this: RawPtr, count: *mut u32, values: *mut *mut Guid) -> ErrorCode;

#[allow(non_camel_case_types)]
pub type Object_GetRuntimeClassName =
    extern "system" fn(this: RawPtr, value: *mut RawPtr) -> ErrorCode;

#[allow(non_camel_case_types)]
pub type Object_GetTrustLevel = extern "system" fn(this: RawPtr, value: *mut i32) -> ErrorCode;

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct Object(IUnknown);

#[repr(C)]
pub struct Object_vtable(
    pub IUnknown_QueryInterface,
    pub IUnknown_AddRef,
    pub IUnknown_Release,
    pub Object_GetIids,
    pub Object_GetRuntimeClassName,
    pub Object_GetTrustLevel,
);

unsafe impl Interface for Object {
    type Vtable = Object_vtable;

    const IID: Guid = {
        Guid::from_values(
            0xAF86_E2E0,
            0xB12D,
            0x4C6A,
            [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
        )
    };
}

impl Object {
    pub fn type_name(&self) -> Result<HString> {
        unsafe {
            let mut abi = std::ptr::null_mut();
            (self.vtable().4)(self.get_abi(), &mut abi).ok()?;
            Ok(std::mem::transmute(abi))
        }
    }
}

unsafe impl RuntimeType for Object {
    const SIGNATURE: crate::ConstBuffer =
        crate::ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
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
                <Object as Interface>::cast_ok::<IReference<$t>>(&value)?.value()
            }
        }
        impl std::convert::TryFrom<&Object> for $t {
            type Error = Error;
            fn try_from(value: &Object) -> Result<Self> {
                <Object as Interface>::cast_ok::<IReference<$t>>(value)?.value()
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
        <Object as Interface>::cast_ok::<IReference<HString>>(&value)?.value()
    }
}
impl std::convert::TryFrom<&Object> for HString {
    type Error = Error;
    fn try_from(value: &Object) -> Result<Self> {
        <Object as Interface>::cast_ok::<IReference<HString>>(value)?.value()
    }
}
