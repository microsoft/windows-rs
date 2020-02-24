use std::collections::BTreeSet;

use proc_macro2::TokenStream;

use super::Writer;
use crate::read::Reader;

pub struct RustWriter {
    reader: Reader,
    limits: BTreeSet<String>,
}

impl RustWriter {
    pub fn new() -> RustWriter {
        RustWriter {
            reader: Reader::from_os().unwrap(),
            limits: BTreeSet::new(),
        }
    }

    pub fn from_files<'a, P: IntoIterator<Item = &'a String>>(filenames: P) -> RustWriter {
        RustWriter {
            reader: Reader::from_files(filenames).unwrap(),
            limits: BTreeSet::new(),
        }
    }

    pub fn add_namespace(&mut self, namespace: &str) {
        let found = self
            .reader
            .namespaces()
            .keys()
            .find(|name| name.to_lowercase() == namespace)
            .unwrap_or_else(|| panic!("Namespace `{}` not found in winmd files", namespace));
        let mut namespace = found.as_str();
        self.limits.insert(namespace.to_string());

        while let Some(index) = namespace.rfind('.') {
            namespace = namespace.get(0..index).unwrap();

            if self.reader.namespaces().contains_key(namespace) {
                self.limits.insert(namespace.to_string());
            }
        }
    }

    pub fn write(&self) -> TokenStream {
        // TODO: ensure *all* windows.foundation.* namespaces are included
        Writer::write(&self.reader, &self.limits)
    }
}
