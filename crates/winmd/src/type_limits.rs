use crate::TypeReader;

use std::collections::BTreeSet;

/// The set of relevant namespaces
#[derive(Default, Debug)]
pub struct TypeLimits(pub BTreeSet<String>);

impl TypeLimits {
    /// Insert a namespace into the set of relevant namespaces
    pub fn insert(&mut self, reader: &TypeReader, namespace: &str) {
        let found = reader
            .types
            .keys()
            .find(|name| name.to_lowercase() == namespace)
            .unwrap_or_else(|| panic!("Namespace `{}` not found in winmd files", namespace));

        let mut namespace = found.as_str();
        self.0.insert(namespace.to_owned());

        while let Some(pos) = namespace.rfind('.') {
            namespace = &namespace[..pos];

            if reader.types.contains_key(namespace) {
                self.0.insert(namespace.to_owned());
            }
        }
    }
}
