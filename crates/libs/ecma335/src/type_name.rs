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

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for namespace in self.namespace.split('.') {
            write!(f, "{namespace}::")?;
        }

        write!(f, "{}", self.name)
    }
}
