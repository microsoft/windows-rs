#![allow(clippy::many_single_char_names)]

use super::*;

pub fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = gen_ident(name);
    let signature = def.signature(None);

    let cfg = gen.field_cfg(def);

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = gen_constant_type_value(&constant.value());
            quote! {
                pub const #name: #value;
            }
        } else {
            let kind = gen_sig(&signature, gen);
            let value = gen_constant_value(&constant.value());

            let value = if signature.kind.underlying_type() == constant.value_type() {
                value
            } else {
                quote! { #value as _ }
            };

            if gen.sys && (signature.kind == constant.value_type() || signature.kind.is_handle() || signature.kind == ElementType::HRESULT) {
                quote! {
                    #cfg
                    pub const #name: #kind = #value;
                }
            } else {
                quote! {
                    #cfg
                    pub const #name: #kind = #kind(#value);
                }
            }
        }
    } else if let Some(guid) = GUID::from_attributes(def.attributes()) {
        let value = gen_guid(&guid, gen);
        let guid = gen_element_name(&ElementType::GUID, gen);
        quote! { pub const #name: #guid = #value; }
    } else if let Some((guid, id)) = get_property_key(def.attributes()) {
        let kind = gen_sig(&signature, gen);
        let guid = gen_guid(&guid, gen);
        quote! {
            #cfg
            pub const #name: #kind = #kind {
                fmtid: #guid,
                pid: #id,
            };
        }
    } else {
        quote! {}
    }
}

pub fn gen_constant_type_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { bool = #value },
        ConstantValue::U8(value) => quote! { u8 = #value },
        ConstantValue::I8(value) => quote! { i8 = #value },
        ConstantValue::U16(value) => quote! { u16 = #value },
        ConstantValue::I16(value) => quote! { i16 = #value },
        ConstantValue::U32(value) => quote! { u32 = #value },
        ConstantValue::I32(value) => quote! { i32 = #value },
        ConstantValue::U64(value) => quote! { u64 = #value },
        ConstantValue::I64(value) => quote! { i64 = #value },
        ConstantValue::F32(value) => quote! { f32 = #value },
        ConstantValue::F64(value) => quote! { f64 = #value },
        ConstantValue::String(value) => quote! { &'static str = #value },
        _ => unimplemented!(),
    }
}

pub fn gen_guid(value: &GUID, gen: &Gen) -> TokenStream {
    let guid = gen_element_name(&ElementType::GUID, gen);

    if gen.sys {
        let a = Literal::u32_unsuffixed(value.0);
        let b = Literal::u16_unsuffixed(value.1);
        let c = Literal::u16_unsuffixed(value.2);
        let d = Literal::u8_unsuffixed(value.3);
        let e = Literal::u8_unsuffixed(value.4);
        let f = Literal::u8_unsuffixed(value.5);
        let g = Literal::u8_unsuffixed(value.6);
        let h = Literal::u8_unsuffixed(value.7);
        let i = Literal::u8_unsuffixed(value.8);
        let j = Literal::u8_unsuffixed(value.9);
        let k = Literal::u8_unsuffixed(value.10);

        // TODO: once code complete measure how much longer it takes if-any to use from_u128 to produce a more compact package

        quote! {
            #guid { data1:#a, data2:#b, data3:#c, data4:[#d, #e, #f, #g, #h, #i, #j, #k] }
        }
    } else {
        format!("{}::from_u128(0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?})", guid.into_string(), value.0, value.1, value.2, value.3, value.4, value.5, value.6, value.7, value.8, value.9, value.10).into()
    }
}

pub fn gen_constant_value(value: &ConstantValue) -> TokenStream {
    match value {
        ConstantValue::Bool(value) => quote! { #value },
        ConstantValue::U8(value) => quote! { #value },
        ConstantValue::I8(value) => quote! { #value },
        ConstantValue::U16(value) => quote! { #value },
        ConstantValue::I16(value) => quote! { #value },
        ConstantValue::U32(value) => quote! { #value },
        ConstantValue::I32(value) => quote! { #value },
        ConstantValue::U64(value) => quote! { #value },
        ConstantValue::I64(value) => quote! { #value },
        ConstantValue::F32(value) => quote! { #value },
        ConstantValue::F64(value) => quote! { #value },
        ConstantValue::String(value) => quote! { #value },
        _ => unimplemented!(),
    }
}

fn get_property_key(attributes: impl Iterator<Item = Attribute>) -> Option<(GUID, u32)> {
    attributes.into_iter().find(|attribute| attribute.name() == "PropertyKeyAttribute").map(|attribute| {
        let args = attribute.args();
        (GUID::from_args(&args), args[11].1.unwrap_u32())
    })
}
