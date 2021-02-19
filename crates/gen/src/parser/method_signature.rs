use super::*;

#[derive(Debug)]
pub struct MethodSignature {
    pub params: Vec<MethodParam>,
    pub return_type: Option<Signature>,
}

#[derive(Debug)]
pub struct MethodParam {
    pub param: tables::Param,
    pub signature: Signature,
}

impl MethodSignature {
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.return_type
            .iter()
            .filter_map(|s| s.definition())
            .chain(self.params.iter().filter_map(|p| p.signature.definition()))
            .collect()
    }

    pub fn gen_constraint(&self, gen: Gen) -> TokenStream {
        let params = self.params.iter().map(|p| p.gen_produce_type(gen));

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.gen(gen)
        } else {
            quote! { () }
        };

        quote! { FnMut(#(#params),*) -> ::windows::Result<#return_type> + 'static }
    }
}

impl MethodParam {
    pub fn gen_abi_arg(&self) -> TokenStream {
        let name = self.param.gen_name();

        if self.signature.kind.is_blittable() {
            quote! { #name }
        } else {
            if self.param.is_input() {
                quote! { ::windows::Abi::abi(#name) }
            } else {
                quote! { ::windows::Abi::set_abi(#name) }
            }
        }
    }

    pub fn gen_produce_type(&self, gen: Gen) -> TokenStream {
        let tokens = self.signature.gen(gen);

        if self.param.is_input() {
            if self.signature.kind.is_primitive() {
                quote! { #tokens }
            } else {
                quote! { &#tokens }
            }
        } else {
            quote! { &mut #tokens }
        }
    }
}
