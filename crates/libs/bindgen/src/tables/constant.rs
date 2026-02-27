use super::*;

pub trait ConstantExt {
    fn constant_type(&self) -> Type;
    fn constant_value(&self) -> Value;
}

impl ConstantExt for Constant {
    fn constant_type(&self) -> Type {
        Type::from_element_type(self.ty().code() as usize)
            .unwrap_or_else(|| panic!("unexpected constant element type: {}", self.ty().code()))
    }

    fn constant_value(&self) -> Value {
        let mut blob = self.blob(2);

        match self.constant_type() {
            Type::I8 => Value::I8(blob.read_i8()),
            Type::U8 => Value::U8(blob.read_u8()),
            Type::I16 => Value::I16(blob.read_i16()),
            Type::U16 => Value::U16(blob.read_u16()),
            Type::I32 => Value::I32(blob.read_i32()),
            Type::U32 => Value::U32(blob.read_u32()),
            Type::I64 => Value::I64(blob.read_i64()),
            Type::U64 => Value::U64(blob.read_u64()),
            Type::F32 => Value::F32(blob.read_f32()),
            Type::F64 => Value::F64(blob.read_f64()),
            Type::String => Value::String(blob.read_utf16()),
            rest => panic!("{rest:?}"),
        }
    }
}
