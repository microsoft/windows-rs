use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ComInterface(pub GenericType);

impl ComInterface {
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0
            .def
            .methods()
            .map(|m| m.dependencies(&[]))
            .chain(self.0.interfaces().map(|i| i.0.dependencies()))
            .flatten()
            .collect()
    }

    pub fn definition(&self) -> Option<tables::TypeDef> {
        Some(self.0.def)
    }

    pub fn gen(&self, _: &Gen) -> TokenStream {
        quote! {}
    }
}
