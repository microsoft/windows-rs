use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub tables::MethodDef);

impl Function {
    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.0.name());
        quote! { #name }
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0.dependencies(&[])
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let name = self.gen_name();
        let signature = self.0.signature(&[]);

        let constraints = signature.gen_constraints(&signature.params, gen);
        let params = signature.gen_params(&signature.params, gen);

        let return_type = if let Some(t) = &signature.return_type {
            let tokens = t.gen(gen);
            quote! { -> #tokens }
        } else {
            quote! {}
        };

        let abi_params = signature.params.iter().map(|p| {
            let name = p.param.gen_name();
            let tokens = p.gen_abi_param(gen);
            quote! { #name: #tokens }
        });

        let abi_return_type = if let Some(t) = &signature.return_type {
            let tokens = t.gen_abi(gen);
            quote! { -> #tokens }
        } else {
            TokenStream::new()
        };

        let args = signature.params.iter().map(|p| p.gen_abi_arg());

        let mut link = self.0.impl_map().expect("Function").scope().name();

        // TODO: workaround for https://github.com/microsoft/windows-rs/issues/463
        if link.contains("-ms-win-") || link == "D3DCOMPILER_47" {
            link = "onecoreuap";
        }

        quote! {
            pub unsafe fn #name<#constraints>(#params) #return_type {
                #[link(name = #link)]
                extern "system" {
                    pub fn #name(#(#abi_params),*) #abi_return_type;
                }
                #name(#(#args),*)
            }
        }
    }
}
