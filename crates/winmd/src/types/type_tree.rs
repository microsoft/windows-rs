use super::{Type, TypeNamespaces};

use proc_macro2::TokenStream;

use std::iter::FromIterator;

/// A namespaced tree of types
#[derive(Default)]
pub struct TypeTree {
    types: Vec<Type>,
    namespaces: TypeNamespaces,
}

impl TypeTree {
    /// Insert a [`Type`] into [`TypeTree`]
    ///
    /// This recursively searchs the tree for an entry corresponding to the namespace
    pub fn insert(&mut self, namespace: String, t: Type) {
        let subtree = &mut self.namespaces.0;
        if let Some(pos) = namespace.find('.') {
            let (namespace, rest) = namespace.split_at(pos);
            subtree
                .entry(namespace.to_owned())
                .or_default()
                .insert(rest.to_owned(), t);
        } else {
            subtree.entry(namespace).or_default().types.push(t);
        }
    }

    /// Turn the tree into a token stream for code generation
    pub fn to_stream(&self) -> TokenStream {
        TokenStream::from_iter(
            self.types
                .iter()
                .map(|t| t.to_stream())
                .chain(std::iter::once(self.namespaces.to_stream())),
        )
    }
}
