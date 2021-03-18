use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub Row);

impl Constant {
    pub fn value_type(&self) -> ElementType {
        let code = self.0.u32(0);
        ElementType::from_code(code).unwrap_or_else(|| panic!("Unexpected ElementType: {:x}", code))
    }

    pub fn value_blob(&self) -> Blob {
        self.0.blob(2)
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
            _ => unexpected!(),
        }
    }
}

impl std::fmt::Debug for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constant")
            .field("value", &self.value())
            .finish()
    }
}
