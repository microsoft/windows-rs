use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
}
