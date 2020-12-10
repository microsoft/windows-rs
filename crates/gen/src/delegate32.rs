use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Delegate32 {
    pub name: TypeName,
    pub method: Method,
}

impl Delegate32 {
    pub fn from_type_name(name: TypeName) -> Self {
        let method = name
            .def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap();

        let method = Method::from_method_def(&method, 0, &[], &name.namespace);
        Self { name, method }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.method.dependencies()
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();

        // TODO: here we're using gen_abi() because as an ABI function pointer it can't use the owning
        // types. Perhaps we should introduce a NonOwning<T> concept for this case.
        let params = self.method.params.iter().map(|param| param.gen_abi());

        let return_type = if let Some(return_type) = &self.method.return_type {
            let return_type = return_type.kind.gen_abi();

            quote! {
                -> #return_type
            }
        } else {
            TokenStream::new()
        };

        quote! {
            #[allow(non_camel_case_types)]
            pub type #name = extern "system" fn(#(#params),*) #return_type;
        }
    }
}
