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
}
