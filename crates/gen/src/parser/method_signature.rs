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

    pub fn gen_constraint(&self, gen: &Gen) -> TokenStream {
        let params = self.params.iter().map(|p| p.gen_produce_type(gen));

        let return_type = if let Some(return_type) = &self.return_type {
            return_type.gen(gen)
        } else {
            quote! { () }
        };

        quote! { FnMut(#(#params),*) -> ::windows::Result<#return_type> + 'static }
    }

    // All WinRT ABI methods return an HRESULT while any return type is transformed into a trailing
    // out parameter. This is unlike Win32 methods that don't require this transformation.
    pub fn gen_winrt_abi(&self, gen: &Gen) -> TokenStream {
        let params = self
            .params
            .iter()
            .map(|p| {
                let name = p.param.gen_name();
                let abi = p.signature.gen_abi(gen);

                if p.param.is_input() {
                    // WinRT only uses const to mean that structs are passed by reference.
                    if p.signature.is_const {
                        quote! { #name: &#abi }
                    } else {
                        quote! { #name: #abi }
                    }
                } else {
                    quote! { #name: *mut #abi }
                }
            })
            .chain(self.return_type.iter().map(|p| {
                let abi = p.gen_abi(gen);
                quote! { result__: *mut #abi }
            }));

        quote! {
            (this: ::windows::RawPtr, #(#params),*) -> ::windows::ErrorCode
        }
    }

    pub fn gen_winrt_method(&self, gen: &MethodGen) -> TokenStream {
        let name = self.gen_name(gen);

        let params = if gen.kind == InterfaceKind::Composable {
            &self.params[..self.params.len() - 2]
        } else {
            &self.params
        };

        quote! {}
    }

    fn gen_name(&self, gen: &MethodGen) -> Ident {
        if gen.kind == InterfaceKind::Composable && self.params.len() == 2 {
            format_ident!("new")
        } else if gen.overload > 1 {
            format_ident!("{}{}", &gen.name, gen.overload)
        } else {
            to_ident(&gen.name)
        }
    }

    fn gen_constraints(&self, params: &[MethodParam], gen: &Gen) -> TokenStream {
        let mut tokens = Vec::new();

        for (index, param) in params.iter().enumerate() {
            if param.param.is_input()
                && !param.signature.is_array
                && param.signature.kind.is_convertible()
            {
                let name = squote::format_ident!("T{}__", index);
                let into = param.signature.kind.gen(gen);
                tokens.push(quote! { #name: ::std::convert::Into<::windows::Param<'a, #into>>, });
            }
        }

        if !tokens.is_empty() {
            tokens.insert(0, quote! { 'a, });
        }

        TokenStream::from_iter(tokens)
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

    pub fn gen_produce_type(&self, gen: &Gen) -> TokenStream {
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
