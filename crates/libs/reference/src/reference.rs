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
        Err(Error::empty())
    }
    fn GetInt16(&self) -> Result<i16> {
        Err(Error::empty())
    }
    fn GetUInt16(&self) -> Result<u16> {
        Err(Error::empty())
    }
    fn GetInt32(&self) -> Result<i32> {
        Err(Error::empty())
    }
    fn GetUInt32(&self) -> Result<u32> {
        Err(Error::empty())
    }
    fn GetInt64(&self) -> Result<i64> {
        Err(Error::empty())
    }
    fn GetUInt64(&self) -> Result<u64> {
        Err(Error::empty())
    }
    fn GetSingle(&self) -> Result<f32> {
        Err(Error::empty())
    }
    fn GetDouble(&self) -> Result<f64> {
        Err(Error::empty())
    }
    fn GetChar16(&self) -> Result<u16> {
        Err(Error::empty())
    }
    fn GetBoolean(&self) -> Result<bool> {
        Err(Error::empty())
    }
    fn GetString(&self) -> Result<HSTRING> {
        Err(Error::empty())
    }
    fn GetGuid(&self) -> Result<GUID> {
        Err(Error::empty())
    }
    fn GetDateTime(&self) -> Result<DateTime> {
        Err(Error::empty())
    }
    fn GetTimeSpan(&self) -> Result<TimeSpan> {
        Err(Error::empty())
    }
    fn GetPoint(&self) -> Result<Point> {
        Err(Error::empty())
    }
    fn GetSize(&self) -> Result<Size> {
        Err(Error::empty())
    }
    fn GetRect(&self) -> Result<Rect> {
        Err(Error::empty())
    }
    fn GetUInt8Array(&self, _value: &mut Array<u8>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetInt16Array(&self, _value: &mut Array<i16>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetUInt16Array(&self, _value: &mut Array<u16>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetInt32Array(&self, _value: &mut Array<i32>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetUInt32Array(&self, _value: &mut Array<u32>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetInt64Array(&self, _value: &mut Array<i64>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetUInt64Array(&self, _value: &mut Array<u64>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetSingleArray(&self, _value: &mut Array<f32>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetDoubleArray(&self, _value: &mut Array<f64>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetChar16Array(&self, _value: &mut Array<u16>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetBooleanArray(&self, _value: &mut Array<bool>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetStringArray(&self, _value: &mut Array<HSTRING>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetInspectableArray(&self, _value: &mut Array<IInspectable>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetGuidArray(&self, _value: &mut Array<GUID>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetDateTimeArray(&self, _value: &mut Array<DateTime>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetTimeSpanArray(&self, _value: &mut Array<TimeSpan>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetPointArray(&self, _value: &mut Array<Point>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetSizeArray(&self, _value: &mut Array<Size>) -> Result<()> {
        Err(Error::empty())
    }
    fn GetRectArray(&self, _value: &mut Array<Rect>) -> Result<()> {
        Err(Error::empty())
    }
}

/// Box a value into an `IReference<T>`.
///
/// `Type<T, Default = T>` selects the common cases where `T`'s storage representation is
/// the same as the value type — primitives, enums, `GUID`, `HSTRING`, and the various
/// `Windows.Foundation` value types (`DateTime`, `TimeSpan`, `Point`, `Size`, `Rect`).
impl<T> From<T> for IReference<T>
where
    T: RuntimeType + Type<T, Default = T>,
    T::Default: Clone,
{
    fn from(value: T) -> Self {
        ComObject::new(StockReference::<T> { value }).into_interface()
    }
}
