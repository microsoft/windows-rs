use super::*;

// pub struct ConstDef<'a> {
//     pub field: Field<'a>,
//     pub namespace: &'a str,
// }

// pub struct FnDef<'a> {
//     pub method: MethodDef<'a>,
//     pub namespace: &'a str,
// }

pub enum Item<'a> {
    Type(TypeDef<'a>),
    Fn(MethodDef<'a>),
    Const(Field<'a>),
}
