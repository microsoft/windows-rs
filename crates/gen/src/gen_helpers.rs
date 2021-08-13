// TODO: sort these out

use crate::*;

pub fn gen_field(def: &tables::Field, gen: &Gen) -> TokenStream {
    let name = def.name();
    let name = to_ident(name);
    let signature = def.signature();

    if let Some(constant) = def.constant() {
        if signature.kind == constant.value_type() {
            let value = constant.value().gen();

            quote! {
                pub const #name: #value;
            }
        } else {
            let kind = signature.gen_win32(gen);
            let value = constant.value().gen_value();

            quote! {
                pub const #name: #kind = #kind(#value as _);
            }
        }
    } else if let Some(guid) = Guid::from_attributes(def.attributes()) {
        let guid = guid.gen();
        quote! { pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid); }
    } else if let Some(pkey) = PropertyKey::from_attributes(def.attributes()) {
        let kind = signature.gen_win32(gen);
        let fmtid = pkey.fmtid.gen();
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
