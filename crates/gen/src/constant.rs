use super::*;

pub fn gen_constant(def: &Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = to_ident(name);
    let signature = def.signature();

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = gen_constant_value_with_type(&constant.value());

            quote! {
                pub const #name: #value;
            }
        } else {
            let kind = gen_win32_sig(&signature, gen);
            let value = gen_constant_value(&constant.value());

            quote! {
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = Guid::from_attributes(def.attributes()) {
        let guid = gen_guid(&guid);
        quote! { pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid); }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = gen_win32_sig(&signature, gen);
        let fmtid = gen_guid(&pkey.fmtid);
        let pid = pkey.pid;
        quote! {
            pub const #name: #kind = #kind {
                fmtid: ::windows::Guid::from_values(#fmtid),
                pid: #pid,
            };
        }
    } else {
        quote! {}
    }
}
