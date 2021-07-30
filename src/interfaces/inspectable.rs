use super::*;
use bindings::Windows::Foundation::{IReference, IStringable, PropertyValue};

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
/// [`IInspectable`] represents the
/// [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IInspectable(IUnknown);

impl IInspectable {
    /// Returns the canonical type name for the underlying object.
    pub fn type_name(&self) -> Result<HSTRING> {
        unsafe {
            let mut abi = std::ptr::null_mut();
            (self.vtable().4)(self.abi(), &mut abi).ok()?;
            Ok(std::mem::transmute(abi))
        }
    }
}

#[repr(C)]
pub struct IInspectable_abi(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr, count: *mut u32, values: *mut *mut Guid) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, value: *mut i32) -> HRESULT,
);

unsafe impl Interface for IInspectable {
    type Vtable = IInspectable_abi;

    const IID: Guid = Guid::from_values(
        0xAF86_E2E0,
        0xB12D,
        0x4C6A,
        [0x9C, 0x5A, 0xD7, 0xAA, 0x65, 0x10, 0x1E, 0x90],
    );
}

unsafe impl RuntimeType for IInspectable {
    const SIGNATURE: crate::ConstBuffer =
        crate::ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

impl std::fmt::Debug for IInspectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents. This implementation
        // is used by all of the generated `Debug` implementations for WinRT
        // classes and interfaces.

        let name = self
            .cast::<IStringable>()
            .and_then(|s| s.ToString())
            .or_else(|_| self.type_name())
            .unwrap_or_default();

        write!(f, "{:?} {}", self.0, name)
    }
}

macro_rules! primitive_boxed_type {
    ($(($t:ty, $m:ident)),+) => {
        $(impl std::convert::TryFrom<$t> for IInspectable {
            type Error = Error;
            fn try_from(value: $t) -> Result<Self> {
                PropertyValue::$m(value)
            }
        }
        impl std::convert::TryFrom<IInspectable> for $t {
            type Error = Error;
            fn try_from(value: IInspectable) -> Result<Self> {
                <IInspectable as Interface>::cast::<IReference<$t>>(&value)?.Value()
            }
        }
        impl std::convert::TryFrom<&IInspectable> for $t {
            type Error = Error;
            fn try_from(value: &IInspectable) -> Result<Self> {
                <IInspectable as Interface>::cast::<IReference<$t>>(value)?.Value()
            }
        })*
    };
}

primitive_boxed_type! {
    (bool, CreateBoolean),
    (u8, CreateUInt8),
    (i16, CreateInt16),
    (u16, CreateUInt16),
    (i32, CreateInt32),
    (u32, CreateUInt32),
    (i64, CreateInt64),
    (u64, CreateUInt64),
    (f32, CreateSingle),
    (f64, CreateDouble)
}

impl std::convert::TryFrom<&str> for IInspectable {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        PropertyValue::CreateString(value)
    }
}
impl std::convert::TryFrom<HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: HSTRING) -> Result<Self> {
        PropertyValue::CreateString(value)
    }
}
impl std::convert::TryFrom<&HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: &HSTRING) -> Result<Self> {
        PropertyValue::CreateString(value)
    }
}
impl std::convert::TryFrom<IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: IInspectable) -> Result<Self> {
        <IInspectable as Interface>::cast::<IReference<HSTRING>>(&value)?.Value()
    }
}
impl std::convert::TryFrom<&IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: &IInspectable) -> Result<Self> {
        <IInspectable as Interface>::cast::<IReference<HSTRING>>(value)?.Value()
    }
}
