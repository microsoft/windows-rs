use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Class {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Class {
    pub fn default_interface(&self, generics: &[Type]) -> Option<Type> {
        self.def
            .interface_impls()
            .find(|imp| imp.has_attribute("DefaultAttribute"))
            .map(|imp| imp.ty(generics))
    }

    pub fn runtime_signature(&self) -> String {
        format!(
            "rc({}.{};{})",
            self.def.namespace(),
            self.def.name(),
            self.default_interface(&self.generics)
                .unwrap()
                .runtime_signature()
        )
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}
