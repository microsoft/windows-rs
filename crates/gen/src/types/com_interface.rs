use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ComInterface(pub GenericType);

impl ComInterface {
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0
            .def
            .methods()
            .map(|m| m.dependencies(&[]))
            .flatten()
            .chain(self.0.interfaces().map(|i| i.def))
            .collect()
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        vec![self.0.def]
    }

    pub fn interfaces(&self) -> Vec<tables::TypeDef> {
        let mut result = Vec::new();
        let mut next = self.0.def;

        loop {
            let base = if let Some(next) = next
                .interface_impls()
                .next()
                .and_then(|i| i.generic_interface(&[]))
            {
                next.def
            } else {
                break;
            };

            next = base;
            result.push(base);
        }

        result
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let name = self.0.gen_name(gen);
        let abi_name = self.0.gen_abi_name(gen);
        let guid = self.0.gen_guid(gen);

        let bases = self.interfaces();

        let abi_signatures = bases
            .iter()
            .rev()
            .chain(std::iter::once(&self.0.def))
            .map(|def| def.methods())
            .flatten()
            .map(|method| {
                let signature = method.signature(&[]);

                let params = signature.params.iter().map(|p| {
                    let name = p.param.gen_name();
                    let tokens = p.gen_win32_abi_param(gen);
                    quote! { #name: #tokens }
                });

                let (udt_return_type, return_type) = if let Some(t) = &signature.return_type {
                    if t.is_struct() {
                        let mut t = t.clone();
                        t.pointers += 1;
                        let tokens = t.gen_abi(gen);
                        (quote! { result__: #tokens }, quote! {})
                    } else {
                        let tokens = t.gen_abi(gen);
                        (quote! {}, quote! { -> #tokens })
                    }
                } else {
                    (TokenStream::new(), TokenStream::new())
                };

                quote! {
                    (this: ::windows::RawPtr, #(#params,)* #udt_return_type) #return_type
                }
            });

        // TODO: dedupe this...
        let mut method_names = BTreeMap::<String, u32>::new();

        let methods = bases
            .iter()
            .rev()
            .chain(std::iter::once(&self.0.def))
            .map(|def| def.methods())
            .flatten()
            .enumerate()
            .map(|(vtable_offset, method)| {
                let signature = method.signature(&[]);

                let constraints = signature.gen_win32_constraints(&signature.params, gen);
                let params = signature.gen_win32_params(&signature.params, gen);

                let (udt_return_type, udt_return_local, return_type) = if let Some(t) = &signature.return_type {
                    if t.is_struct() {
                        let tokens = t.kind.gen_abi(gen);
                        (quote! { &mut result__ }, quote! { let mut result__: #tokens = ::std::default::Default::default(); }, quote! {})
                    } else {
                        let tokens = t.gen_abi(gen);
                        (quote! {}, quote!{}, quote! { -> #tokens })
                    }
                } else {
                    (TokenStream::new(), TokenStream::new(), TokenStream::new())
                };

                let args = signature.params.iter().map(|p| p.gen_win32_abi_arg());

                let name = method.name();
                let overload = method_names.entry(name.to_string()).or_insert(0);
                *overload += 1;

                let name = if *overload > 1 {
                    format_ident!("{}{}", name, overload)
                } else {
                    to_ident(name)
                };

                let vtable_offset = Literal::u32_unsuffixed(vtable_offset as u32 + 3);

                quote! {
                    pub unsafe fn #name<#constraints>(&self, #params) #return_type {
                        #udt_return_local
                        (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args,)* #udt_return_type)
                    }
                }
            });

        quote! {
            #[repr(transparent)]
            #[derive(::std::cmp::PartialEq, ::std::cmp::Eq, ::std::clone::Clone, ::std::fmt::Debug)]
            pub struct #name(::windows::IUnknown);
            impl #name {

            }
            unsafe impl ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #[allow(non_snake_case)]
            impl #name {
                #(#methods)*
            }
            // #conversions
            #[repr(C)]
            #[doc(hidden)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                #(pub unsafe extern "system" fn #abi_signatures,)*
            );

        }
    }
}
