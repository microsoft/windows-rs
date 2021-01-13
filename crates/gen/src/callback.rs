use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Callback {
    pub name: TypeName,
    pub method: NativeMethod,
}

impl Callback {
    pub fn from_type_name(name: TypeName) -> Self {
        let method = name
            .def
            .methods()
            .find(|method| method.name() == "Invoke")
            .unwrap();

        let method = NativeMethod::new(&method, &name.namespace);
        Self { name, method }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.method.dependencies()
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();

        let params = self.method.params.iter().map(|param| {
            let name = format_ident(param.name);
            let tokens = param.t.gen_field();
            quote! { #name: #tokens }
        });

        let return_type = if let Some(t) = &self.method.return_type {
            let tokens = t.gen_field();
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        quote! {
            #[allow(non_camel_case_types)]
            pub type #name = extern "system" fn(#(#params),*) #return_type;
        }
    }
}
