use crate::*;

#[derive(Default, Debug)]
pub struct TypeLimits(pub BTreeSet<String>);

impl TypeLimits {
    pub fn insert(&mut self, reader: &Reader, namespace: &str) {
        let found = reader
            .types
            .keys()
            .find(|name| name.to_lowercase() == namespace)
            .unwrap_or_else(|| panic!("Namespace `{}` not found in winmd files", namespace));

        let mut namespace = found.as_str();
        self.0.insert(namespace.to_string());

        while let Some(pos) = namespace.rfind('.') {
            namespace = namespace.get(..pos).unwrap();

            if reader.types.contains_key(namespace) {
                self.0.insert(namespace.to_string());
            }
        }
    }
}
