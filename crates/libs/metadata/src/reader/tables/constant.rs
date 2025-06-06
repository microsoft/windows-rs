use super::*;

impl<'a> Constant<'a> {
    pub fn ty(&self) -> Type {
        match self.usize(0).try_into().unwrap() {
            ELEMENT_TYPE_U1 => Type::U8,
            ELEMENT_TYPE_I1 => Type::I8,
            ELEMENT_TYPE_U2 => Type::U16,
            ELEMENT_TYPE_I2 => Type::I16,
            ELEMENT_TYPE_U4 => Type::U32,
            ELEMENT_TYPE_I4 => Type::I32,
            ELEMENT_TYPE_U8 => Type::U64,
            ELEMENT_TYPE_I8 => Type::I64,
            ELEMENT_TYPE_R4 => Type::F32,
            ELEMENT_TYPE_R8 => Type::F64,
            ELEMENT_TYPE_STRING => Type::String,
            rest => panic!("{rest:?}"),
        }
    }

    pub fn parent(&self) -> HasConstant<'a> {
        self.decode(1)
    }

    pub fn value(&self) -> Value {
        let mut blob = self.blob(2);

        match self.ty() {
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
            Type::String => Value::Utf16(blob.read_utf16()),
            rest => panic!("{rest:?}"),
        }
    }
}
