use crate::*;
use squote::{quote, TokenStream};

#[derive(Debug)]
pub struct Function {
    pub name: TypeName,
    pub method: winmd::MethodDef,
    pub params: Vec<(&'static str, Type)>,
    pub return_type: Option<Type>,
}

impl Function {
    pub fn new(name: TypeName, method: &winmd::MethodDef) -> Self {
        let mut blob = method.sig();

        if blob.read_unsigned() & 0x10 != 0 {
            blob.read_unsigned();
        }

        let param_count = blob.read_unsigned();
        blob.read_modifiers();
        blob.read_expected(0x10);

        let mut params = Vec::with_capacity(param_count as usize);

        let return_type = if blob.read_expected(0x01) {
            None
        } else {
            Some(Type::from_blob(&mut blob, &[], name.namespace))
        };

        for param in method.params() {
            if return_type.is_none() || param.sequence() != 0 {
                blob.read_modifiers(); // const
                blob.read_expected(0x10); // ref
                params.push((
                    param.name(),
                    Type::from_blob(&mut blob, &[], name.namespace),
                ));
            }
        }

        Self {
            name,
            method: *method,
            params,
            return_type,
        }
    }

    pub fn gen(&self) -> TokenStream {
        let name = format_ident(self.method.name());

        let return_type = if let Some(t) = &self.return_type {
            let tokens = t.gen_field();
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        let params = self.params.iter().map(|(name, t)| {
            let name = format_ident(name);
            let tokens = t.gen_field();
            quote! { #name: #tokens }
        });

        // TODO: need to generate libs until Rust supports dynamic linking against DLLs.
        // This is actually the DLL name.
        let link = self.method.impl_map().unwrap().scope();

        quote! {
            #[link(name = #link)]
            extern "system" {
                pub fn #name(#(#params),*) #return_type;
            }
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.return_type
            .iter()
            .chain(self.params.iter().map(|(_, t)| t))
            .flat_map(|t| t.kind.dependencies())
            .collect()
    }
}
