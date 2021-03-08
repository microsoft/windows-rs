use super::*;
macros::table!(InterfaceImpl);

impl InterfaceImpl {
    pub fn interface(&self) -> TypeDefOrRef {
        self.reader.decode(self.row, 1)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> + '_ {
        self.reader
            .equal_range(
                self.row.file_index,
                TableIndex::CustomAttribute,
                0,
                HasAttribute::InterfaceImpl(*self).encode(),
            )
            .map(move |row| Attribute {
                reader: self.reader,
                row,
            })
    }

    pub fn has_attribute(&self, namespace: &str, name: &str) -> bool {
        self.attributes()
            .any(|attribute| attribute.full_name() == (namespace, name))
    }

    pub fn is_default(&self) -> bool {
        self.has_attribute("Windows.Foundation.Metadata", "DefaultAttribute")
    }

    pub fn is_overridable(&self) -> bool {
        self.has_attribute("Windows.Foundation.Metadata", "OverridableAttribute")
    }

    pub fn generic_interface(&self, generics: &[ElementType]) -> Option<parser::GenericType> {
        Some(match self.interface() {
            TypeDefOrRef::TypeDef(def) => GenericType {
                def,
                generics: Vec::new(),
            },
            TypeDefOrRef::TypeRef(def) => {
                if def.full_name() == ("Windows.Win32.Com", "IUnknown") {
                    return None;
                }

                GenericType {
                    def: def.resolve(),
                    generics: Vec::new(),
                }
            }
            TypeDefOrRef::TypeSpec(def) => {
                let mut blob = def.blob();
                blob.read_unsigned();
                GenericType::from_blob(&mut blob, &generics)
            }
        })
    }
}

impl std::fmt::Debug for InterfaceImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interface = self.interface();
        write!(f, "{}.{}", interface.namespace(), interface.name())
    }
}