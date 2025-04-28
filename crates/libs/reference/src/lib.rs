#![allow(non_snake_case, clippy::all)]
#![doc = include_str!("../readme.md")]
#![allow(missing_docs)]
#![no_std]

mod bindings;
use bindings::*;
use windows_core::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct Reference<T>(bindings::IReference<T>)
where
    T: RuntimeType + 'static;

impl<T: RuntimeType> From<T> for Reference<T> {
    fn from(value: T) -> Self {
        Self(Box(value).into())
    }
}

impl<T: RuntimeType> Reference<T> {
    pub fn value(&self) -> T {
        self.0.Value().unwrap()
    }
}

#[implement(bindings::IReference<T>, bindings::IPropertyValue)]
struct Box<T>(T)
where
    T: RuntimeType + 'static;

impl<T: RuntimeType> bindings::IReference_Impl<T> for Box_Impl<T> {
    fn Value(&self) -> Result<T> {
        Ok(self.0.clone())
    }
}

impl<T: RuntimeType> bindings::IPropertyValue_Impl for Box_Impl<T> {
    fn Type(&self) -> Result<PropertyType> {
        Err(Error::from(E_NOTIMPL))
    }
    fn IsNumericScalar(&self) -> Result<bool> {
        Err(Error::from(E_NOTIMPL))
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
    fn GetUInt8Array(&self, _: &mut Array<u8>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt16Array(&self, _: &mut Array<i16>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt16Array(&self, _: &mut Array<u16>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt32Array(&self, _: &mut Array<i32>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt32Array(&self, _: &mut Array<u32>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInt64Array(&self, _: &mut Array<i64>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetUInt64Array(&self, _: &mut Array<u64>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetSingleArray(&self, _: &mut Array<f32>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetDoubleArray(&self, _: &mut Array<f64>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetChar16Array(&self, _: &mut Array<u16>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetBooleanArray(&self, _: &mut Array<bool>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetStringArray(&self, _: &mut Array<HSTRING>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetInspectableArray(&self, _: &mut Array<IInspectable>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetGuidArray(&self, _: &mut Array<GUID>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetDateTimeArray(&self, _: &mut Array<DateTime>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetTimeSpanArray(&self, _: &mut Array<TimeSpan>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetPointArray(&self, _: &mut Array<Point>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetSizeArray(&self, _: &mut Array<Size>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
    fn GetRectArray(&self, _: &mut Array<Rect>) -> Result<()> {
        Err(Error::from(E_NOTIMPL))
    }
}
