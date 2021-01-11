use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Function {
    pub name: TypeName,
    pub method: NativeMethod,
}

impl Function {
    pub fn new(name: TypeName, method: &winmd::MethodDef) -> Self {
        Self {
            method: NativeMethod::new(method, &name.namespace),
            name,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.method.def.name();

        // TODO: workaround for https://github.com/microsoft/win32metadata/issues/91
        // Note that even with the fix, #[link] doesn't like this and warns about clashing
        // extern declarations. So we really need support for DLL imports asap.
        if name == "GetDeviceID" {
            return quote! {};
        }

        let name = format_ident(name);

        let return_type = if let Some(t) = &self.method.return_type {
            let tokens = t.gen_field();
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        let params = self.method.params.iter().map(|(name, t)| {
            let name = format_ident(name);
            let tokens = t.gen_field();
            quote! { #name: #tokens }
        });

        // TODO: need to generate libs until Rust supports dynamic linking against DLLs.
        // This is actually the DLL name.
        let mut link = self.method.def.impl_map().unwrap().scope().name();
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
        self.method.dependencies()
    }
}
