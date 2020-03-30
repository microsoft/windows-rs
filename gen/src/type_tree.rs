use crate::*;

#[derive(Default)]
pub struct TypeTree {
    types: Vec<Type>,
    namespaces: TypeNamespace,
}

impl TypeTree {
    pub fn insert(&mut self, namespace: String, t: Type) {
        if let Some(pos) = namespace.find('.') {
            self.namespaces
                .0
                .entry(namespace[..pos].to_string())
                .or_default()
                .insert(namespace[pos + 1..].to_string(), t);
        } else {
            self.namespaces
                .0
                .entry(namespace)
                .or_default()
                .types
                .push(t);
        }
    }

    pub fn into_stream(&self) -> TokenStream {
        TokenStream::from_iter(
            self.types
                .iter()
                .map(|t| t.into_stream())
                .chain(std::iter::once(self.namespaces.into_stream())),
        )
    }
}
