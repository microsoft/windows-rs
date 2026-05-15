use super::*;
use windows_core::*;

/// A stock implementation of `IReference<T>` for boxing a value of type `T`.
///
/// Most callers don't need to use this type directly. Use `IReference::<T>::from(value)`
/// to box a value into an `IReference<T>`, which is typically what WinRT APIs expect
/// when they need an optional or reference value.
struct StockReference<T>
where
    T: RuntimeType + Clone + 'static,
{
    value: T,
}

implement_decl! {
    impl<T> StockReference as StockReference_Impl: [
        IReference<T>,
        IPropertyValue,
    ]
    where T: RuntimeType + Clone + 'static
}

impl<T> IReference_Impl<T> for StockReference_Impl<T>
where
    T: RuntimeType + Clone,
{
    fn Value(&self) -> Result<T> {
        Ok(self.value.clone())
    }
}

// `IReference<T>` derives from `IPropertyValue`, so we have to provide an implementation.
// The stock implementation does not attempt to perform value conversions between types
// the way the C++ implementation does — it simply reports `PropertyType::OtherType` and
// rejects all of the typed accessors with `E_NOTIMPL`. Callers should use `Value()` to
// retrieve the underlying value.
impl<T> IPropertyValue_Impl for StockReference_Impl<T>
where
    T: RuntimeType + Clone,
{
    fn Type(&self) -> Result<PropertyType> {
        Ok(PropertyType::OtherType)
    }
    fn IsNumericScalar(&self) -> Result<bool> {
        Ok(false)
    }
    fn GetUInt8(&self) -> Result<u8> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt16(&self) -> Result<i16> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt16(&self) -> Result<u16> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt32(&self) -> Result<i32> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt32(&self) -> Result<u32> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt64(&self) -> Result<i64> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt64(&self) -> Result<u64> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetSingle(&self) -> Result<f32> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetDouble(&self) -> Result<f64> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetChar16(&self) -> Result<u16> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetBoolean(&self) -> Result<bool> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetString(&self) -> Result<HSTRING> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetGuid(&self) -> Result<GUID> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt8Array(&self, _value: &mut Array<u8>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt16Array(&self, _value: &mut Array<i16>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt16Array(&self, _value: &mut Array<u16>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt32Array(&self, _value: &mut Array<i32>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt32Array(&self, _value: &mut Array<u32>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt64Array(&self, _value: &mut Array<i64>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt64Array(&self, _value: &mut Array<u64>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetSingleArray(&self, _value: &mut Array<f32>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetDoubleArray(&self, _value: &mut Array<f64>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetChar16Array(&self, _value: &mut Array<u16>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetBooleanArray(&self, _value: &mut Array<bool>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetStringArray(&self, _value: &mut Array<HSTRING>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInspectableArray(&self, _value: &mut Array<IInspectable>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetGuidArray(&self, _value: &mut Array<GUID>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
}

/// Box a value into an `IReference<T>`.
///
/// This is intended for WinRT value types (primitives, enums, `GUID`, `HSTRING`, and the
/// `Windows.Foundation` value structs like `DateTime`, `TimeSpan`, `Point`, `Size`, and
/// `Rect`) — i.e. types where `T` is itself the storage representation.
impl<T> From<T> for IReference<T>
where
    T: RuntimeType + Clone + 'static,
{
    fn from(value: T) -> Self {
        ComObject::new(StockReference::<T> { value }).into_interface()
    }
}
