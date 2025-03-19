use super::*;

#[derive(Debug, PartialEq)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
}
