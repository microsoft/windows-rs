use super::*;
use bindings::*;

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
/// [`IInspectable`] represents the
/// [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IInspectable(pub IUnknown);

impl IInspectable {
    /// Returns the canonical type name for the underlying object.
    pub fn type_name(&self) -> Result<HSTRING> {
        unsafe {
            let mut abi = core::ptr::null_mut();
            (self.vtable().4)(core::mem::transmute_copy(self), &mut abi).ok()?;
            Ok(core::mem::transmute(abi))
        }
    }
}

#[repr(C)]
pub struct IInspectableVtbl(pub unsafe extern "system" fn(this: RawPtr, iid: &GUID, interface: *mut RawPtr) -> HRESULT, pub unsafe extern "system" fn(this: RawPtr) -> u32, pub unsafe extern "system" fn(this: RawPtr) -> u32, pub unsafe extern "system" fn(this: RawPtr, count: *mut u32, values: *mut *mut GUID) -> HRESULT, pub unsafe extern "system" fn(this: RawPtr, value: *mut RawPtr) -> HRESULT, pub unsafe extern "system" fn(this: RawPtr, value: *mut i32) -> HRESULT);

unsafe impl Interface for IInspectable {
    type Vtable = IInspectableVtbl;

    const IID: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
}

unsafe impl RuntimeType for IInspectable {
    const SIGNATURE: ConstBuffer = ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

impl core::fmt::Debug for IInspectable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents. This implementation
        // is used by all of the generated `Debug` implementations for WinRT
        // classes and interfaces.

        let name = self.cast::<IStringable>().and_then(|s| s.ToString()).or_else(|_| self.type_name()).unwrap_or_default();

        write!(f, "{:?} {}", self.0, name)
    }
}

macro_rules! primitive_boxed_type {
    ($(($t:ty, $m:ident)),+) => {
        $(impl core::convert::TryFrom<$t> for IInspectable {
            type Error = Error;
            fn try_from(value: $t) -> Result<Self> {
                PropertyValue::$m(value)
            }
        }
        impl core::convert::TryFrom<IInspectable> for $t {
            type Error = Error;
            fn try_from(value: IInspectable) -> Result<Self> {
                <IInspectable as Interface>::cast::<IReference<$t>>(&value)?.Value()
            }
        }
        impl core::convert::TryFrom<&IInspectable> for $t {
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

impl core::convert::TryFrom<&str> for IInspectable {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value: HSTRING = value.into();
        PropertyValue::CreateString(value)
    }
}
impl core::convert::TryFrom<HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: HSTRING) -> Result<Self> {
        PropertyValue::CreateString(value)
    }
}
impl core::convert::TryFrom<&HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: &HSTRING) -> Result<Self> {
        PropertyValue::CreateString(value)
    }
}
impl core::convert::TryFrom<IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: IInspectable) -> Result<Self> {
        <IInspectable as Interface>::cast::<IReference<HSTRING>>(&value)?.Value()
    }
}
impl core::convert::TryFrom<&IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: &IInspectable) -> Result<Self> {
        <IInspectable as Interface>::cast::<IReference<HSTRING>>(value)?.Value()
    }
}
