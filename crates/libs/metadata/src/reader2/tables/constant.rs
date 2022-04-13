use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant(pub Row);

// impl Constant {
//     pub fn ty(&self, _scope: &Reader) -> Type {
//         todo!()
//     }
//     pub fn value<'a>(&self, scope: &'a Reader) -> Value {
//         let mut value = scope.blob(self.0, 2);

//         match self.ty(scope) {
//             Type::I8 => Value::I8(value.read_i8()),
//             Type::U8 => Value::U8(value.read_u8()),
//             Type::I16 => Value::I16(value.read_i16()),
//             Type::U16 => Value::U16(value.read_u16()),
//             Type::I32 => Value::I32(value.read_i32()),
//             Type::U32 => Value::U32(value.read_u32()),
//             Type::I64 => Value::I64(value.read_i64()),
//             Type::U64 => Value::U64(value.read_u64()),
//             Type::F32 => Value::F32(value.read_f32()),
//             Type::F64 => Value::F64(value.read_f64()),
//             _ => unimplemented!(),
//         }
//     }
// }
