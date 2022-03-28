use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Type<'a> {
    TypeDef(TypeDef<'a>),
}
