use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppInterface {
    pub def: TypeDef,
}

impl CppInterface {
    pub fn dependencies(&self, dependencies: &mut Dependencies, config: &Config) {
        if !config.sys && dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}
