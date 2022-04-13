use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute(pub Row);

// impl Attribute {
//     pub fn name<'a>(&self, scope: &'a Reader) -> &'a str {
//         let AttributeType::MemberRef(member) = scope.decode(self.0, 1);
//         let MemberRefParent::TypeRef(type_ref) = member.parent(scope);
//         type_ref.name(scope)
//     }
//     pub fn args<'a>(&self, scope: &Reader) -> Vec<(String, Value)> {
//         let AttributeType::MemberRef(member) = scope.decode(self.0, 1);
//         let mut sig = member.signature(scope);
//         let mut values = scope.blob(self.0, 2);
//         let _prolog = values.read_u16();
//         let _this_and_gen_param_count = sig.read_usize();
//         let fixed_arg_count = sig.read_usize();
//         let _ret_type = sig.read_usize();
//         let mut args: Vec<(String, Value)> = Vec::with_capacity(fixed_arg_count as usize);

//         for _ in 0..fixed_arg_count {
//             let arg = match scope.type_from_blob(&mut sig, None, &[]).expect("Type not found") {
//                 Type::I8 => Value::I8(values.read_i8()),
//                 Type::U8 => Value::U8(values.read_u8()),
//                 Type::I16 => Value::I16(values.read_i16()),
//                 Type::U16 => Value::U16(values.read_u16()),
//                 Type::I32 => Value::I32(values.read_i32()),
//                 Type::U32 => Value::U32(values.read_u32()),
//                 Type::I64 => Value::I64(values.read_i64()),
//                 Type::U64 => Value::U64(values.read_u64()),
//                 Type::String => Value::String(values.read_str().to_string()),
//                 Type::TypeName => Value::TypeDef(scope.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
//                 Type::TypeDef((def, _)) => match def.underlying_type(scope) {
//                     Type::I8 => Value::I8(values.read_i8()),
//                     Type::U8 => Value::U8(values.read_u8()),
//                     Type::I16 => Value::I16(values.read_i16()),
//                     Type::U16 => Value::U16(values.read_u16()),
//                     Type::I32 => Value::I32(values.read_i32()),
//                     Type::U32 => Value::U32(values.read_u32()),
//                     Type::I64 => Value::I64(values.read_i64()),
//                     Type::U64 => Value::U64(values.read_u64()),
//                     _ => unimplemented!(),
//                 },
//                 _ => unimplemented!(),
//             };

//             args.push((String::new(), arg));
//         }
            
//         let named_arg_count = values.read_u16();
//         args.reserve(named_arg_count as usize);

//         for _ in 0..named_arg_count {
//             let _id = values.read_u8();
//             let arg_type = values.read_u8();
//             let name = values.read_str().to_string();
//             let arg = match arg_type {
//                 0x02 => Value::Bool(values.read_u8() != 0),
//                 0x06 => Value::I16(values.read_i16()),
//                 0x08 => Value::I32(values.read_i32()),
//                 0x09 => Value::U32(values.read_u32()),
//                 0x0E => Value::String(values.read_str().to_string()),
//                 0x50 => Value::TypeDef(scope.get(TypeName::parse(values.read_str())).next().expect("Type not found")),
//                 _ => unimplemented!(),
//             };
//             args.push((name, arg));
//         }
        
//         args
//     }
//     pub fn composable_type(&self, scope:&Reader) -> Option<TypeDef> {
//         let mut public = false;
//         let mut def = None;

//         // One of the arguments is a CompositionType enum and the Public variant
//         // has a value of 2 as a signed 32-bit integer.

//         for (_, arg) in self.args(scope) {
//             match arg {
//                 Value::I32(2) => public = true,
//                 Value::TypeDef(value) => def = Some(value),
//                 _ => {}
//             }
//         }

//         if public {
//             def
//         } else {
//             None
//         }
//     }
// }
