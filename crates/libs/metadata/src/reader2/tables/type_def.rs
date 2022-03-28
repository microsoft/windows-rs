use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeDef<'a>(Row<'a>, Vec<Type<'a>>);
