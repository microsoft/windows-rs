use super::*;

#[derive(Clone)]
pub struct Constant(pub Row);

impl Constant {
    pub fn value_type(&self) -> Type {
        let code = self.0.u32(0);
        Type::from_code(code).unwrap_or_else(|| unimplemented!()).to_const()
    }

    pub fn value(&self) -> ConstantValue {
        let mut value = self.0.blob(2);

        match self.value_type() {
            Type::I8 => ConstantValue::I8(value.read_i8()),
            Type::U8 => ConstantValue::U8(value.read_u8()),
            Type::I16 => ConstantValue::I16(value.read_i16()),
            Type::U16 => ConstantValue::U16(value.read_u16()),
            Type::I32 => ConstantValue::I32(value.read_i32()),
            Type::U32 => ConstantValue::U32(value.read_u32()),
            Type::I64 => ConstantValue::I64(value.read_i64()),
            Type::U64 => ConstantValue::U64(value.read_u64()),
            Type::F32 => ConstantValue::F32(value.read_f32()),
            Type::F64 => ConstantValue::F64(value.read_f64()),
            Type::String => ConstantValue::String(value.read_utf16()),
            _ => unimplemented!(),
        }
    }
}
