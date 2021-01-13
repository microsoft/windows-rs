use crate::*;
use squote::{quote, TokenStream};

// TODO: move winmd into gen crate to improve inlining and simplify
#[derive(Debug)]
pub struct Function {
    pub name: TypeName,
    pub method: winmd::MethodDef,
    pub signature: Signature,
}

impl Function {
    pub fn new(name: TypeName, method: &winmd::MethodDef) -> Self {
        let signature = Signature::new(method, &[], &name.namespace);
        Self {
            name,
            method: *method,
            signature,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.method.name();

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/91
        // Note that even with the fix, #[link] doesn't like this and warns about clashing
        // extern declarations. So we really need support for DLL imports asap.
        if name == "GetDeviceID" {
            return quote! {};
        }

        let name = format_ident(name);

        let params = self.signature.params.iter().map(|t| {
            let name = format_ident(t.param.unwrap().name());
            let tokens = t.gen_field();
            quote! { #name: #tokens }
        });

        let return_type = if let Some(t) = &self.signature.return_type {
            let tokens = t.gen_field();
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        // TODO: need to generate libs until Rust supports dynamic linking against DLLs.
        // This is actually the DLL name.
        let mut link = self.method.impl_map().unwrap().scope().name();
        if link == "ext-ms-win-core-iuri-l1-1-0" {
            link = "urlmon";
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
