use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ComInterface(pub GenericType);

impl ComInterface {
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0
            .def
            .methods()
            .map(|m| m.dependencies(&[]))
            .flatten()
            .chain(self.0.interfaces().map(|i| i.def))
            .collect()
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        vec![self.0.def]
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        quote! {}
    }
}
