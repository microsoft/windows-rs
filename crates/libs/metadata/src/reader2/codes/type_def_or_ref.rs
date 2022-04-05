// use super::*;

// #[derive(Clone)]
// pub enum TypeDefOrRef<'a> {
//     TypeDef(TypeDef),
//     TypeRef(TypeRef),
//     TypeSpec(TypeSpec),
// }

// impl<'a> Decode for TypeDefOrRef<'a> {
//     fn decode(file: &'static File, code: u32) -> Self {
//         let code = (code & ((1 << 2) - 1), (code >> 2) - 1);
//         match code.0 {
//             0 => Self::TypeDef(Row::new(code.1, TableIndex::TypeDef, file).into()),
//             1 => Self::TypeRef(TypeRef(Row::new(code.1, TableIndex::TypeRef, file))),
//             2 => Self::TypeSpec(TypeSpec(Row::new(code.1, TableIndex::TypeSpec, file))),
//             _ => unimplemented!(),
//         }
//     }
// }

// impl<'a> TypeDefOrRef<'a> {
//     pub fn encode(&self) -> u32 {
//         match self {
//             Self::TypeDef(value) => ((value.row.row + 1) << 2),
//             Self::TypeRef(value) => ((value.0.row + 1) << 2) | 1,
//             Self::TypeSpec(value) => ((value.0.row + 1) << 2) | 2,
//         }
//     }

//     pub fn type_name(&self) -> TypeName {
//         match self {
//             Self::TypeDef(value) => value.type_name(),
//             Self::TypeRef(value) => value.type_name(),
//             _ => unimplemented!(),
//         }
//     }

//     pub fn resolve(&self, enclosing: Option<&TypeDef>) -> TypeDef {
//         match self {
//             Self::TypeDef(value) => value.clone(),
//             Self::TypeRef(value) => TypeReader::get().expect_type_ref(enclosing, value),
//             _ => unimplemented!(),
//         }
//     }
// }
