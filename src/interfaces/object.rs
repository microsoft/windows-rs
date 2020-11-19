use crate::foundation::{IReference, IStringable, PropertyValue};
use crate::*;

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
/// `Object` implements the
/// [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct Object(IUnknown);

impl Object {
    /// Returns the canonical type name for the underlying object.
    pub fn type_name(&self) -> Result<HString> {
        unsafe {
            let mut abi = std::ptr::null_mut();
            (self.vtable().4)(self.abi(), &mut abi).ok()?;
            Ok(std::mem::transmute(abi))
        }
    }
}

#[repr(C)]
pub struct Object_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> ErrorCode,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: RawPtr,
        count: *mut u32,
        values: *mut *mut Guid,
    ) -> ErrorCode,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut RawPtr) -> ErrorCode,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut i32) -> ErrorCode,
);

unsafe impl Interface for Object {
    type Vtable = Object_vtable;

    const IID: Guid = Guid::from_values(
        0xAF86_E2E0,
        0xB12D,
        0x4C6A,
        [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
    );
}

unsafe impl RuntimeType for Object {
    type DefaultType = Option<Self>;

    const SIGNATURE: crate::ConstBuffer =
        crate::ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents. This implementation
        // is used by all of the generated `Debug` implementations for WinRT
        // classes and interfaces.
        let name = self
            .cast::<IStringable>()
            .and_then(|s| s.to_string())
            .or_else(|_| self.type_name())
            .unwrap_or_default();

        write!(f, "{:?} {}", self.0, name)
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
                <Object as Interface>::cast::<IReference<$t>>(&value)?.value()
            }
        }
        impl std::convert::TryFrom<&Object> for $t {
            type Error = Error;
            fn try_from(value: &Object) -> Result<Self> {
                <Object as Interface>::cast::<IReference<$t>>(value)?.value()
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
        <Object as Interface>::cast::<IReference<HString>>(&value)?.value()
    }
}
impl std::convert::TryFrom<&Object> for HString {
    type Error = Error;
    fn try_from(value: &Object) -> Result<Self> {
        <Object as Interface>::cast::<IReference<HString>>(value)?.value()
    }
}
