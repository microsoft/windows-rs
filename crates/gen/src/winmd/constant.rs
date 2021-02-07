use super::*;
macros::table!(Constant);

impl Constant {
    pub fn value_type(&self) -> ElementType {
        ElementType::from_code(self.reader.u32(self.row, 0))
    }

    pub fn value_blob(&self) -> Blob {
        self.reader.blob(self.row, 2)
    }

    pub fn value(&self) -> ConstantValue {
        let mut value = self.value_blob();

        match self.value_type() {
            ElementType::I8 => ConstantValue::I8(value.read_i8()),
            ElementType::U8 => ConstantValue::U8(value.read_u8()),
            ElementType::I16 => ConstantValue::I16(value.read_i16()),
            ElementType::U16 => ConstantValue::U16(value.read_u16()),
            ElementType::I32 => ConstantValue::I32(value.read_i32()),
            ElementType::U32 => ConstantValue::U32(value.read_u32()),
            ElementType::I64 => ConstantValue::I64(value.read_i64()),
            ElementType::U64 => ConstantValue::U64(value.read_u64()),
            ElementType::F32 => ConstantValue::F32(value.read_f32()),
            ElementType::F64 => ConstantValue::F64(value.read_f64()),
            ElementType::String => ConstantValue::String(value.read_utf16()),
            value_type => panic!("Unsupported constant: ({:?})", value_type),
        }
    }
}
