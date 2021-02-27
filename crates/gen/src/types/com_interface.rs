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
                    let tokens = p.signature.gen_abi(gen);
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
                    (#(#params),*, #udt_return_type) #return_type
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
                // #(#methods)*
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
