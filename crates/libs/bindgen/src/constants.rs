#![allow(clippy::many_single_char_names)]

use super::*;

pub fn gen(def: &Field, gen: &Gen) -> TokenStream {
    let name = gen_ident(def.name());
    let ty = def.get_type(None).to_const();
    let cfg = def.cfg();
    let doc = gen.doc(&cfg);
    let features = gen.cfg(&cfg);

    if let Some(constant) = def.constant() {
        if ty == constant.value_type() {
            let value = gen_constant_type_value(&constant.value());
            quote! {
                #doc
                #features
                pub const #name: #value;
            }
        } else {
            let kind = gen_default_type(&ty, gen);
            let value = gen_constant_value(&constant.value());

            let value = if ty.underlying_type() == constant.value_type() {
                value
            } else {
                quote! { #value as _ }
            };

            if !gen.sys && ty.has_replacement() {
                quote! {
                    #doc
                    #features
                    pub const #name: #kind = #kind(#value);
                }
            } else {
                quote! {
                    #doc
                    #features
                    pub const #name: #kind = #value;
                }
            }
        }
    } else if let Some(guid) = GUID::from_attributes(def.attributes()) {
        let value = gen_guid(&guid, gen);
        let guid = gen_element_name(&Type::GUID, gen);
        quote! { pub const #name: #guid = #value; }
    } else if let Some((guid, id)) = get_property_key(def.attributes()) {
        let kind = gen_default_type(&ty, gen);
        let guid = gen_guid(&guid, gen);
        quote! {
            #doc
            #features
            pub const #name: #kind = #kind {
                fmtid: #guid,
                pid: #id,
            };
        }
    } else {
        quote! {}
    }
}

fn get_property_key(attributes: impl Iterator<Item = Attribute>) -> Option<(GUID, u32)> {
    attributes.into_iter().find(|attribute| attribute.name() == "PropertyKeyAttribute").map(|attribute| {
        let args = attribute.args();
        (GUID::from_args(&args), args[11].1.unwrap_u32())
    })
}
