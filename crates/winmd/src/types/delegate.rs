use crate::tables::*;
use crate::types::*;
use crate::TypeReader;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct Delegate {
    pub name: TypeName,
    pub method: Method,
    // TODO: replace `method` with this:
    // pub params: Vec<Param>,
    // pub return_type: Option<Param>,
}

impl Delegate {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let method = def
            .methods(reader)
            .find(|method| method.name(reader) == "Invoke")
            .unwrap();
        let method = Method::from_method_def(reader, method, &name.generics);
        Self { name, method }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        self.method.dependencies()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = self.name.to_tokens(&self.name.namespace);
        let phantoms = self.name.phantoms();

        quote! {
            pub struct #name {
                #phantoms
            }
        }
    }
}
