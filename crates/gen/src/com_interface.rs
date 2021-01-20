use crate::*;
use squote::{format_ident, quote, Ident, Literal, TokenStream};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ComInterface {
    pub name: TypeName,
    methods: Vec<Method>,
}

#[derive(Debug)]
struct Method {
    signature: Signature,
    overload: u32,
}

impl Method {
    fn gen_name(&self) -> Ident {
        if self.overload > 1 {
            format_ident!("{}{}", self.signature.method.name(), self.overload)
        } else {
            format_ident(self.signature.method.name())
        }
    }
}

impl ComInterface {
    pub fn from_type_name(name: TypeName) -> Self {
        let mut count = BTreeMap::new();

        let methods = name
            .def
            .methods()
            .map(|method| {
                let count = count.entry(method.name()).or_insert(0);
                *count += 1;

                Method {
                    signature: Signature::new(&method, &[], &name.namespace),
                    overload: *count,
                }
            })
            .collect();

        Self { name, methods }
    }

    pub fn gen(&self) -> TokenStream {
        let name = self.name.gen();
        let abi_name = self.name.gen_abi_definition();
        let guid = TypeGuid::from_type_def(&self.name.def);
        let guid = self.name.gen_guid(&guid);

        // TODO: here we're looking up the param name (from the file) repeatedly - cache name in Type

        let methods = self.methods.iter().enumerate().map(|(vtable_offset, method)| {
            let return_type = if let Some(t) = &method.signature.return_type {
                let tokens = t.gen_field();
                quote! { -> #tokens }
            } else {
                TokenStream::new()
            };

            let params = method.signature.params.iter().map(|t| {
                let name = format_ident(&t.name);
                let tokens = t.gen_field();
                quote! { #name: #tokens }
            });

            let args = method.signature.params.iter().map(|t| {
                let name = format_ident(&t.name);
                quote! { #name }
            });

            let name = method.gen_name();
            let vtable_offset = Literal::u32_unsuffixed((vtable_offset + 3) as u32);

            quote! {
                pub fn #name(&self, #(#params),*) #return_type {
                    unsafe {
                        (::windows::Interface::vtable(self).#vtable_offset)(::windows::Abi::abi(self), #(#args),*)
                    }
                }
            }
        });

        let abi_methods = self.methods.iter().map(|method| {
            let return_type = if let Some(t) = &method.signature.return_type {
                let tokens = t.gen_field();
                quote! { -> #tokens }
            } else {
                TokenStream::new()
            };

            let params = method.signature.params.iter().map(|t| {
                let name = format_ident(&t.name);
                let tokens = t.gen_field();
                quote! { #name: #tokens }
            });

            quote! {
                pub unsafe extern "system" fn (this: ::windows::RawPtr, #(#params),*) #return_type
            }
        });

        quote! {
            #[repr(transparent)]
            #[allow(non_camel_case_types)]
            pub struct #name(::windows::IUnknown);
            impl ::std::clone::Clone for #name {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for #name {}
            unsafe impl ::windows::Interface for #name {
                type Vtable = #abi_name;
                const IID: ::windows::Guid = #guid;
            }
            #[repr(C)]
            pub struct #abi_name(
                pub unsafe extern "system" fn(this: ::windows::RawPtr, iid: &::windows::Guid, interface: *mut ::windows::RawPtr) -> ::windows::ErrorCode,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                #(#abi_methods,)*
            );
            #[allow(non_snake_case)]
            impl #name {
                #(#methods)*
            }
        }
    }

    pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
        self.methods
            .iter()
            .map(|method| method.signature.dependencies())
            .flatten()
            .collect()
    }
}
