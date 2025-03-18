use super::*;

#[derive(Debug)]
pub struct TypeName<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
    pub generics: Vec<Type<'a>>,
}

impl<'a> TypeName<'a> {
    pub fn new(namespace: &'a str, name: &'a str) -> Self {
        Self {
            namespace,
            name,
            generics: vec![],
        }
    }
}
