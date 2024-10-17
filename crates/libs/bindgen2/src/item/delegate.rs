use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Delegate {
    pub def: TypeDef,
    pub generics: Vec<Type>,
}

impl Delegate {
    pub fn method(&self) -> MethodDef {
        self.def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap()
    }

    pub fn runtime_signature(&self) -> String {
        if self.generics.is_empty() {
            let guid = self.def.guid_attribute().unwrap();
            format!("delegate({{{guid}}})")
        } else {
            interface_signature(self.def, &self.generics)
        }
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies, _config: &Config) {
        if dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}
