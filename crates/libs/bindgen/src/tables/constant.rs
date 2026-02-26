use super::*;

pub trait ConstantExt {
    fn constant_type(&self) -> Type;
    fn constant_value(&self) -> Value;
}

impl ConstantExt for Constant {
    fn constant_type(&self) -> Type {
        Type::from_element_type(self.usize(0)).unwrap()
    }

    fn constant_value(&self) -> Value {
        let mut blob = self.blob(2);

        match self.usize(0) as u8 {
            ELEMENT_TYPE_I1 => Value::I8(blob.read_i8()),
            ELEMENT_TYPE_U1 => Value::U8(blob.read_u8()),
            ELEMENT_TYPE_I2 => Value::I16(blob.read_i16()),
            ELEMENT_TYPE_U2 => Value::U16(blob.read_u16()),
            ELEMENT_TYPE_I4 => Value::I32(blob.read_i32()),
            ELEMENT_TYPE_U4 => Value::U32(blob.read_u32()),
            ELEMENT_TYPE_I8 => Value::I64(blob.read_i64()),
            ELEMENT_TYPE_U8 => Value::U64(blob.read_u64()),
            ELEMENT_TYPE_R4 => Value::F32(blob.read_f32()),
            ELEMENT_TYPE_R8 => Value::F64(blob.read_f64()),
            ELEMENT_TYPE_STRING => Value::String(blob.read_utf16()),
            rest => panic!("{rest:?}"),
        }
    }
}

