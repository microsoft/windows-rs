use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct CppInterface {
    pub def: TypeDef,
}

impl CppInterface {
    pub fn expand(self, _filter: &NameTree) -> Self {
        // TODO: load methods, base interfaces
        self
    }

    pub fn write(&self, _writer: &Writer) -> TokenStream {
        quote! {}
    }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        if
        // !dependencies.config.sys &&
        dependencies.insert(self.def.namespace(), self.def.name()) {
            // TODO: add dependencies
        }
    }
}
