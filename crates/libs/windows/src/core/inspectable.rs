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
    pub fn GetRuntimeClassName(&self) -> Result<HSTRING> {
        unsafe {
            let mut abi = core::ptr::null_mut();
            (self.vtable().GetRuntimeClassName)(core::mem::transmute_copy(self), &mut abi).ok()?;
            Ok(core::mem::transmute(abi))
        }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct IInspectableVtbl {
    pub base: IUnknownVtbl,
    pub GetIids: unsafe extern "system" fn(this: RawPtr, count: *mut u32, values: *mut *mut GUID) -> HRESULT,
    pub GetRuntimeClassName: unsafe extern "system" fn(this: RawPtr, value: *mut RawPtr) -> HRESULT,
    pub GetTrustLevel: unsafe extern "system" fn(this: RawPtr, value: *mut i32) -> HRESULT,
}

unsafe impl Interface for IInspectable {
    type Vtable = IInspectableVtbl;

    const IID: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
}

unsafe impl RuntimeType for IInspectable {
    const SIGNATURE: ConstBuffer = ConstBuffer::from_slice(b"cinterface(IInspectable)");
    type DefaultType = Option<Self>;
    fn from_default(from: &Self::DefaultType) -> Result<Self> {
        from.as_ref().cloned().ok_or(Error::OK)
    }
}

impl RuntimeName for IInspectable {
    const NAME: &'static str = "";
}

impl core::fmt::Debug for IInspectable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents. This implementation
        // is used by all of the generated `Debug` implementations for WinRT
        // classes and interfaces.

        let name = self.cast::<IStringable>().and_then(|s| s.ToString()).or_else(|_| self.GetRuntimeClassName()).unwrap_or_default();

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

#[cfg(feature = "implement")]
impl IInspectableVtbl {
    pub const fn new<Identity: IUnknownImpl, Name: RuntimeName, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIids(_: RawPtr, count: *mut u32, values: *mut *mut GUID) -> ::windows::core::HRESULT {
            // Note: even if we end up implementing this in future, it still doesn't need a this pointer
            // since the data to be returned is type- not instance-specific so can be shared for all
            // interfaces.
            *count = 0;
            *values = core::ptr::null_mut();
            HRESULT(0)
        }
        unsafe extern "system" fn GetRuntimeClassName<T: RuntimeName>(_: RawPtr, value: *mut RawPtr) -> HRESULT {
            let h: HSTRING = T::NAME.into(); // TODO: should be try_into
            *value = ::core::mem::transmute(h);
            HRESULT(0)
        }
        unsafe extern "system" fn GetTrustLevel(_: RawPtr, value: *mut i32) -> HRESULT {
            // Note: even if we end up implementing this in future, it still doesn't need a this pointer
            // since the data to be returned is type- not instance-specific so can be shared for all
            // interfaces.
            *value = 0;
            HRESULT(0)
        }
        Self { base: IUnknownVtbl::new::<Identity, OFFSET>(), GetIids, GetRuntimeClassName: GetRuntimeClassName::<Name>, GetTrustLevel }
    }
}
