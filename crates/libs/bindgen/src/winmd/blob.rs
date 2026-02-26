use super::*;

pub type Blob = windows_metadata::reader::Blob<'static>;

pub trait BlobExt {
    fn reader(&self) -> &'static Reader;
    fn read_usize(&mut self) -> usize;
    fn read_integer(&mut self, ty: &Type) -> Value;
}

impl BlobExt for Blob {
    fn reader(&self) -> &'static Reader {
        reader_from_index(self.index)
    }

    fn read_usize(&mut self) -> usize {
        self.read_compressed()
    }

    fn read_integer(&mut self, ty: &Type) -> Value {
        match *ty {
            Type::I8 => Value::I8(self.read_i8()),
            Type::U8 => Value::U8(self.read_u8()),
            Type::I16 => Value::I16(self.read_i16()),
            Type::U16 => Value::U16(self.read_u16()),
            Type::I32 => Value::I32(self.read_i32()),
            Type::U32 => Value::U32(self.read_u32()),
            Type::I64 => Value::I64(self.read_i64()),
            Type::U64 => Value::U64(self.read_u64()),
            _ => panic!(),
        }
    }
}

