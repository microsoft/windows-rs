use super::*;
use Primitives::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Primitives.Widget").then(activatable::<Widget, Primitives::Widget>)
}

#[implement(Primitives::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn NotBool(&self, value: bool) -> Result<bool> {
        Ok(!value)
    }

    fn NextChar16(&self, value: u16) -> Result<u16> {
        Ok(value.wrapping_add(1))
    }

    fn NextU8(&self, value: u8) -> Result<u8> {
        Ok(value.wrapping_add(1))
    }

    fn NextI16(&self, value: i16) -> Result<i16> {
        Ok(value.wrapping_add(1))
    }

    fn NextU16(&self, value: u16) -> Result<u16> {
        Ok(value.wrapping_add(1))
    }

    fn NextI32(&self, value: i32) -> Result<i32> {
        Ok(value.wrapping_add(1))
    }

    fn NextU32(&self, value: u32) -> Result<u32> {
        Ok(value.wrapping_add(1))
    }

    fn NextI64(&self, value: i64) -> Result<i64> {
        Ok(value.wrapping_add(1))
    }

    fn NextU64(&self, value: u64) -> Result<u64> {
        Ok(value.wrapping_add(1))
    }

    fn DoubleF32(&self, value: f32) -> Result<f32> {
        Ok(value * 2.0)
    }

    fn DoubleF64(&self, value: f64) -> Result<f64> {
        Ok(value * 2.0)
    }

    fn FormatGuid(&self, value: &GUID) -> Result<HSTRING> {
        Ok(format!("{value:?}").into())
    }

    fn ParseGuid(&self, value: &HSTRING) -> Result<GUID> {
        GUID::try_from(value.to_string_lossy().as_str()).map_err(|_| E_INVALIDARG.into())
    }
}
