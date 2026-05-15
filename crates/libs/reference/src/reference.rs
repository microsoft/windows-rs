use super::*;
use windows_core::*;

/// A stock implementation of `IReference<T>` for boxing a value of type `T`.
///
/// Most callers don't need to use this type directly. Use `IReference::<T>::from(value)`
/// to box a value into an `IReference<T>`, which is typically what WinRT APIs expect
/// when they need an optional or reference value.
struct StockReference<T>
where
    T: RuntimeType + 'static,
    T::Default: Clone,
{
    value: T::Default,
}

implement_decl! {
    impl<T> StockReference as StockReference_Impl: [
        IReference<T>,
        IPropertyValue,
    ]
    where T: RuntimeType + 'static, T::Default: Clone
}

impl<T> IReference_Impl<T> for StockReference_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone,
{
    fn Value(&self) -> Result<T> {
        T::from_default(&self.value)
    }
}

// `IReference<T>` derives from `IPropertyValue`, so we have to provide an implementation.
// The stock implementation does not attempt to perform value conversions between types
// the way the C++ implementation does — it simply reports `PropertyType::OtherType` and
// rejects all of the typed accessors with `E_NOTIMPL`. Callers should use `Value()` to
// retrieve the underlying value.
impl<T> IPropertyValue_Impl for StockReference_Impl<T>
where
    T: RuntimeType,
    T::Default: Clone,
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
    fn GetDateTime(&self) -> Result<DateTime> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetTimeSpan(&self) -> Result<TimeSpan> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetPoint(&self) -> Result<Point> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetSize(&self) -> Result<Size> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetRect(&self) -> Result<Rect> {
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
    fn GetDateTimeArray(&self, _value: &mut Array<DateTime>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetTimeSpanArray(&self, _value: &mut Array<TimeSpan>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetPointArray(&self, _value: &mut Array<Point>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetSizeArray(&self, _value: &mut Array<Size>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetRectArray(&self, _value: &mut Array<Rect>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
}

/// Box a value into an `IReference<T>`.
///
/// The `Type<T, Default = T>` constraint selects the common cases where `T`'s storage
/// representation is the same as the value type — primitives, enums, `GUID`, `HSTRING`,
/// and the `Windows.Foundation` value types (`DateTime`, `TimeSpan`, `Point`, `Size`,
/// `Rect`). The full bound is `T: RuntimeType + Type<T, Default = T>` so that we can both
/// box the value and project it back through `IReference::Value`.
impl<T> From<T> for IReference<T>
where
    T: RuntimeType + Type<T, Default = T>,
    T::Default: Clone,
{
    fn from(value: T) -> Self {
        ComObject::new(StockReference::<T> { value }).into_interface()
    }
}
