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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_inclusion() {
        let reader = &TypeReader::from_os();

        {
            // Windows.Foundation's parent is empty so that's not included
            let mut limits = TypeLimits::default();
            limits.insert(reader, "windows.foundation");
            assert!(limits.0.len() == 1);
            assert!(limits.0.contains("Windows.Foundation"));
        }

        {
            // Windows.Foundation.Collections's parent is not empty so it gets included
            let mut limits = TypeLimits::default();
            limits.insert(reader, "windows.foundation.collections");
            assert!(limits.0.len() == 2);
            assert!(limits.0.contains("Windows.Foundation"));
            assert!(limits.0.contains("Windows.Foundation.Collections"));
        }

        {
            let mut limits = TypeLimits::default();
            limits.insert(reader, "windows.foundation.collections");
            limits.insert(reader, "windows.ui.xaml.controls");
            assert!(limits.0.len() == 5);
            assert!(limits.0.contains("Windows.Foundation"));
            assert!(limits.0.contains("Windows.Foundation.Collections"));
            assert!(limits.0.contains("Windows.UI"));
            assert!(limits.0.contains("Windows.UI.Xaml"));
            assert!(limits.0.contains("Windows.UI.Xaml.Controls"));
        }
    }
}
