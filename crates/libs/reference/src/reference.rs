use super::bindings;
use bindings::E_NOTIMPL;
use windows_core::*;

/// A boxed reference to a value of type `T`.
///
/// `IReference<T>` is the Rust projection of the WinRT `Windows.Foundation.IReference<T>`
/// interface, which Windows APIs use to pass optional or boxed value-type parameters.
///
/// This type intentionally exposes only [`Value`](Self::Value). The underlying
/// `IPropertyValue` accessors (e.g. `GetInt32`) are not exposed: callers should use
/// `Value` to retrieve the boxed value.
///
/// Use `IReference::<T>::from(value)` to box a value into an `IReference<T>`.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IReference<T>(IUnknown, core::marker::PhantomData<T>)
where
    T: RuntimeType + 'static;

impl<T: RuntimeType + 'static> imp::CanInto<IUnknown> for IReference<T> {}
impl<T: RuntimeType + 'static> imp::CanInto<IInspectable> for IReference<T> {}

unsafe impl<T: RuntimeType + 'static> Interface for IReference<T> {
    type Vtable = bindings::IReference_Vtbl<T>;
    const IID: GUID = GUID::from_signature(<Self as RuntimeType>::SIGNATURE);
}

impl<T: RuntimeType + 'static> RuntimeType for IReference<T> {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::new()
        .push_slice(b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}

impl<T: RuntimeType + 'static> RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}

impl<T: RuntimeType + 'static> IReference<T> {
    /// Retrieves the boxed value.
    pub fn Value(&self) -> Result<T> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (Interface::vtable(self).Value)(Interface::as_raw(self), &mut result__)
                .and_then(|| Type::from_abi(result__))
        }
    }
}

/// Box a value into an `IReference<T>`.
impl<T> From<T> for IReference<T>
where
    T: RuntimeType + Clone + 'static,
{
    fn from(value: T) -> Self {
        let inner: bindings::IReference<T> =
            ComObject::new(StockReference { value }).into_interface();
        // Safety: `IReference<T>` and `bindings::IReference<T>` are both `#[repr(transparent)]`
        // wrappers over `IUnknown` (plus `PhantomData<T>`) and share the same WinRT IID and
        // vtable layout. The two types are bit-for-bit identical.
        unsafe { core::mem::transmute(inner) }
    }
}

/// Box a string slice into an `IReference<HSTRING>`.
impl From<&str> for IReference<HSTRING> {
    fn from(value: &str) -> Self {
        HSTRING::from(value).into()
    }
}

/// Box a `String` into an `IReference<HSTRING>`.
impl From<String> for IReference<HSTRING> {
    fn from(value: String) -> Self {
        HSTRING::from(value).into()
    }
}

/// Box an `&HSTRING` into an `IReference<HSTRING>`.
impl From<&HSTRING> for IReference<HSTRING> {
    fn from(value: &HSTRING) -> Self {
        value.clone().into()
    }
}

// Internal stock implementation used to back `IReference::from`.
struct StockReference<T>
where
    T: RuntimeType + Clone + 'static,
{
    value: T,
}

implement_decl! {
    impl<T> StockReference as StockReference_Impl: [
        bindings::IReference<T>,
        bindings::IPropertyValue,
    ]
    where T: RuntimeType + Clone + 'static
}

impl<T> bindings::IReference_Impl<T> for StockReference_Impl<T>
where
    T: RuntimeType + Clone,
{
    fn Value(&self) -> Result<T> {
        Ok(self.value.clone())
    }
}

impl<T> bindings::IPropertyValue_Impl for StockReference_Impl<T>
where
    T: RuntimeType + Clone,
{
    fn Type(&self) -> Result<bindings::PropertyType> {
        Ok(bindings::PropertyType::OtherType)
    }
    fn IsNumericScalar(&self) -> Result<bool> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetUInt8(&self) -> Result<u8> {
        cast_value(&self.value)
    }
    fn GetInt16(&self) -> Result<i16> {
        cast_value(&self.value)
    }
    fn GetUInt16(&self) -> Result<u16> {
        cast_value(&self.value)
    }
    fn GetInt32(&self) -> Result<i32> {
        cast_value(&self.value)
    }
    fn GetUInt32(&self) -> Result<u32> {
        cast_value(&self.value)
    }
    fn GetInt64(&self) -> Result<i64> {
        cast_value(&self.value)
    }
    fn GetUInt64(&self) -> Result<u64> {
        cast_value(&self.value)
    }
    fn GetSingle(&self) -> Result<f32> {
        cast_value(&self.value)
    }
    fn GetDouble(&self) -> Result<f64> {
        cast_value(&self.value)
    }
    fn GetChar16(&self) -> Result<u16> {
        cast_value(&self.value)
    }
    fn GetBoolean(&self) -> Result<bool> {
        cast_value(&self.value)
    }
    fn GetString(&self) -> Result<HSTRING> {
        cast_value(&self.value)
    }
    fn GetGuid(&self) -> Result<GUID> {
        cast_value(&self.value)
    }
    fn GetDateTime(&self) -> Result<bindings::DateTime> {
        cast_value(&self.value)
    }
    fn GetTimeSpan(&self) -> Result<bindings::TimeSpan> {
        cast_value(&self.value)
    }
    fn GetPoint(&self) -> Result<bindings::Point> {
        cast_value(&self.value)
    }
    fn GetSize(&self) -> Result<bindings::Size> {
        cast_value(&self.value)
    }
    fn GetRect(&self) -> Result<bindings::Rect> {
        cast_value(&self.value)
    }
    fn GetUInt8Array(&self, _value: &mut Array<u8>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetInt16Array(&self, _value: &mut Array<i16>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetUInt16Array(&self, _value: &mut Array<u16>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetInt32Array(&self, _value: &mut Array<i32>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetUInt32Array(&self, _value: &mut Array<u32>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetInt64Array(&self, _value: &mut Array<i64>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetUInt64Array(&self, _value: &mut Array<u64>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetSingleArray(&self, _value: &mut Array<f32>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetDoubleArray(&self, _value: &mut Array<f64>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetChar16Array(&self, _value: &mut Array<u16>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetBooleanArray(&self, _value: &mut Array<bool>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetStringArray(&self, _value: &mut Array<HSTRING>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetInspectableArray(&self, _value: &mut Array<IInspectable>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetGuidArray(&self, _value: &mut Array<GUID>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetDateTimeArray(&self, _value: &mut Array<bindings::DateTime>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetTimeSpanArray(&self, _value: &mut Array<bindings::TimeSpan>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetPointArray(&self, _value: &mut Array<bindings::Point>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetSizeArray(&self, _value: &mut Array<bindings::Size>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
    fn GetRectArray(&self, _value: &mut Array<bindings::Rect>) -> Result<()> {
        Err(Error::from_hresult(E_NOTIMPL))
    }
}

fn cast_value<T: 'static, U: Clone + 'static>(value: &T) -> Result<U> {
    if core::any::TypeId::of::<T>() == core::any::TypeId::of::<U>() {
        // Safety: we verified `T` and `U` have the same `TypeId`, which guarantees
        // they are the same type. Reading `value` as `&U` is safe.
        Ok(unsafe { &*(value as *const T as *const U) }.clone())
    } else {
        Err(Error::from_hresult(E_NOTIMPL))
    }
}
