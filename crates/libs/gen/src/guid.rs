#![allow(clippy::many_single_char_names)]

use super::*;

pub fn gen_guid(g: &GUID) -> TokenStream {
    format!("0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}", g.0, g.1, g.2, g.3, g.4, g.5, g.6, g.7, g.8, g.9, g.10).into()
}

pub fn gen_type_guid(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.generics.is_empty() {
        match GUID::from_attributes(def.attributes()) {
            Some(guid) => {
                let guid = gen_guid(&guid);

                quote! {
                    ::windows::core::GUID::from_u128(#guid)
                }
            }
            None => {
                quote! {
                    ::windows::core::GUID::zeroed()
                }
            }
        }
    } else {
        let tokens = gen_type_name(def, gen);

        quote! {
            ::windows::core::GUID::from_signature(<#tokens as ::windows::core::RuntimeType>::SIGNATURE)
        }
    }
}

pub fn gen_guid_signature(def: &TypeDef, signature: &str) -> TokenStream {
    let signature = Literal::byte_string(signature.as_bytes());

    if def.generics.is_empty() {
        return quote! { ::windows::core::ConstBuffer::from_slice(#signature) };
    }

    let generics = def.generics.iter().enumerate().map(|(index, g)| {
        let g = gen_name(g, &Gen::absolute());
        let semi = if index != def.generics.len() - 1 {
            Some(quote! {
                .push_slice(b";")
            })
        } else {
            None
        };

        quote! {
            .push_other(<#g as ::windows::core::RuntimeType>::SIGNATURE)
            #semi
        }
    });

    quote! {
        {
            ::windows::core::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(#signature)
            .push_slice(b";")
            #(#generics)*
            .push_slice(b")")
        }
    }
}
