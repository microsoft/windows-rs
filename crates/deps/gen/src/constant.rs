use super::*;

pub fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = to_ident(name);
    let signature = def.signature(None);

    let features = signature_features(&signature, gen);
    let cfg = gen.gen_cfg(&features);
    let doc = gen.gen_cfg_doc(&features);

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = gen_constant_type_value(&constant.value());

            quote! {
                #cfg
                #doc
                pub const #name: #value;
            }
        } else {
            let kind = gen_sig(&signature, gen);
            let value = gen_constant_value(&constant.value());

            quote! {
                #cfg
                #doc
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = GUID::from_attributes(def.attributes()) {
        let guid = gen_guid(&guid);
        quote! { pub const #name: ::windows::core::GUID = ::windows::core::GUID::from_u128(#guid); }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = gen_sig(&signature, gen);
        let fmtid = gen_guid(&pkey.fmtid);
        let pid = pkey.pid;
        quote! {
            #cfg
            #doc
            pub const #name: #kind = #kind {
                fmtid: ::windows::core::GUID::from_u128(#fmtid),
                pid: #pid,
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
