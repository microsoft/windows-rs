use super::*;

#[derive(Clone)]
pub struct InterfaceImpl(pub Row);

impl InterfaceImpl {
    pub fn interface(&self) -> TypeDefOrRef {
        self.0.decode(1)
    }

    pub fn attributes(&self) -> impl Iterator<Item = Attribute> {
        self.0
            .file
            .equal_range(
                TableIndex::CustomAttribute,
                0,
                HasAttribute::InterfaceImpl(self.clone()).encode(),
            )
            .map(Attribute)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes().any(|attribute| attribute.name() == name)
    }

    pub fn is_default(&self) -> bool {
        self.has_attribute("DefaultAttribute")
    }

    pub fn is_overridable(&self) -> bool {
        self.has_attribute("OverridableAttribute")
    }

    pub fn generic_interface(&self, generics: &[ElementType]) -> Option<tables::TypeDef> {
        Some(match self.interface() {
            TypeDefOrRef::TypeDef(def) => def,
            TypeDefOrRef::TypeRef(def) => {
                if def.full_name() == ("Windows.Win32.System.Com", "IUnknown") {
                    return None;
                }

                def.resolve()
            }
            TypeDefOrRef::TypeSpec(def) => {
                let mut blob = def.blob();
                blob.read_unsigned();
                TypeDef::from_blob(&mut blob, &generics)
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
