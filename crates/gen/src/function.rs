use crate::*;
use squote::{quote, TokenStream};

// TODO: move winmd into gen crate to improve inlining and simplify
#[derive(Debug)]
pub struct Function {
    pub name: TypeName,
    pub signature: Signature,
}

impl Function {
    pub fn new(name: TypeName, method: &winmd::MethodDef) -> Self {
        let signature = Signature::new(method, &[], &name.namespace);
        Self { name, signature }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.signature.method.name();
        let name = format_ident(name);

        let params = self.signature.params.iter().map(|t| {
            let name = format_ident(&t.name);
            let tokens = t.gen_field();
            quote! { #name: #tokens }
        });

        let return_type = if let Some(t) = &self.signature.return_type {
            let tokens = t.gen_field();
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        let mut link = self.signature.method.impl_map().unwrap().scope().name();

        // TODO: workaround for https://github.com/microsoft/windows-rs/issues/463
        if link.contains("-ms-win-") || link == "D3DCOMPILER_47" {
            link = "onecoreuap";
        }

        quote! {
            #[link(name = #link)]
            extern "system" {
                pub fn #name(#(#params),*) #return_type;
            }
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.signature.dependencies()
    }
}
