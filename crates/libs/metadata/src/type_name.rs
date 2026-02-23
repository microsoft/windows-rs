use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
}

impl TypeName {
    pub fn named(namespace: &str, name: &str) -> Self {
        TypeName {
            namespace: namespace.to_string(),
            name: name.to_string(),
            generics: vec![],
        }
    }
}

impl PartialEq<(&str, &str)> for &TypeName {
    fn eq(&self, other: &(&str, &str)) -> bool {
        self.namespace == other.0 && self.name == other.1
    }
}
