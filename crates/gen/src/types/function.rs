use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub tables::MethodDef);

impl Function {
    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.0.name());
        quote! { #name }
    }

    pub fn dependencies(&self) -> Vec<ElementType> {
        self.0.dependencies(&[])
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        let name = self.gen_name();
        let signature = self.0.signature(&[]);

        let constraints = signature.gen_win32_constraints(&signature.params, gen);
        let params = signature.gen_win32_params(&signature.params, gen);

        let return_type = if let Some(t) = &signature.return_type {
            let tokens = t.gen_win32(gen);
            quote! { -> #tokens }
        } else {
            quote! {}
        };

        let abi_params = signature.params.iter().map(|p| {
            let name = p.param.gen_name();
            let tokens = p.gen_win32_abi_param(gen);
            quote! { #name: #tokens }
        });

        let abi_return_type = if let Some(t) = &signature.return_type {
            // TODO: This should be gen_win32_abi?
            let tokens = t.gen_win32(gen);
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        let args = signature.params.iter().map(|p| p.gen_win32_abi_arg());

        let mut link = self.0.impl_map().expect("Function").scope().name();

        // TODO: workaround for https://github.com/microsoft/windows-rs/issues/463
        if link.contains("-ms-win-") || link == "D3DCOMPILER_47" {
            link = "onecoreuap";
        }

        if cfg!(windows) {
            quote! {
                pub unsafe fn #name<#constraints>(#params) #return_type {
                    #[link(name = #link)]
                    extern "system" {
                        pub fn #name(#(#abi_params),*) #abi_return_type;
                    }
                    #name(#(#args),*)
                }
            }
        } else {
            quote! {
                pub unsafe fn #name<#constraints>(#params) #return_type {
                    panic!("Unsupported target OS");
                }
            }
        }
    }
}
