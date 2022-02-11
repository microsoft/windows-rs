use super::*;

#[derive(Clone)]
pub struct Constant(pub Row);

impl Constant {
    pub fn value_type(&self) -> Signature {
        let code = self.0.u32(0);
        Signature::from_code(code).unwrap_or_else(|| unimplemented!())
    }

    pub fn value(&self) -> ConstantValue {
        let mut value = self.0.blob(2);

        match self.value_type() {
            Signature::I8 => ConstantValue::I8(value.read_i8()),
            Signature::U8 => ConstantValue::U8(value.read_u8()),
            Signature::I16 => ConstantValue::I16(value.read_i16()),
            Signature::U16 => ConstantValue::U16(value.read_u16()),
            Signature::I32 => ConstantValue::I32(value.read_i32()),
            Signature::U32 => ConstantValue::U32(value.read_u32()),
            Signature::I64 => ConstantValue::I64(value.read_i64()),
            Signature::U64 => ConstantValue::U64(value.read_u64()),
            Signature::F32 => ConstantValue::F32(value.read_f32()),
            Signature::F64 => ConstantValue::F64(value.read_f64()),
            Signature::String => ConstantValue::String(value.read_utf16()),
            _ => unimplemented!(),
        }
    }
}
