#![allow(clippy::many_single_char_names)]

use super::*;

pub fn gen_guid(guid: &GUID) -> TokenStream {
    let a = Literal::u32_unsuffixed(guid.0);
    let b = Literal::u16_unsuffixed(guid.1);
    let c = Literal::u16_unsuffixed(guid.2);
    let d = Literal::u8_unsuffixed(guid.3);
    let e = Literal::u8_unsuffixed(guid.4);
    let f = Literal::u8_unsuffixed(guid.5);
    let g = Literal::u8_unsuffixed(guid.6);
    let h = Literal::u8_unsuffixed(guid.7);
    let i = Literal::u8_unsuffixed(guid.8);
    let j = Literal::u8_unsuffixed(guid.9);
    let k = Literal::u8_unsuffixed(guid.10);

    quote! {
        #a, #b, #c, [#d, #e, #f, #g, #h, #i, #j, #k],
    }
}

pub fn gen_type_guid(def: &TypeDef, gen: &Gen) -> TokenStream {
    if def.generics.is_empty() {
        match GUID::from_attributes(def.attributes()) {
            Some(guid) => {
                let guid = gen_guid(&guid);

                quote! {
                    ::windows::runtime::GUID::from_values(#guid)
                }
            }
            None => {
                quote! {
                    ::windows::runtime::GUID::zeroed()
                }
            }
        }
    } else {
        let tokens = gen_type_name(def, gen);

        quote! {
            ::windows::runtime::GUID::from_signature(<#tokens as ::windows::runtime::RuntimeType>::SIGNATURE)
        }
    }
}

pub fn gen_guid_signature(def: &TypeDef, signature: &str) -> TokenStream {
    let signature = Literal::byte_string(signature.as_bytes());

    if def.generics.is_empty() {
        return quote! { ::windows::runtime::ConstBuffer::from_slice(#signature) };
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
            .push_other(<#g as ::windows::runtime::RuntimeType>::SIGNATURE)
            #semi
        }
    });

    quote! {
        {
            ::windows::runtime::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(#signature)
            .push_slice(b";")
            #(#generics)*
            .push_slice(b")")
        }
    }
}
