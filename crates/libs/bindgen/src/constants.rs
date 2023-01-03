use super::*;

pub fn gen(gen: &Gen, def: Field) -> TokenStream {
    let name = to_ident(gen.reader.field_name(def));
    let ty = gen.reader.field_type(def, None).to_const();
    let cfg = gen.reader.field_cfg(def);
    let doc = gen.cfg_doc(&cfg);
    let features = gen.cfg_features(&cfg);

    if let Some(constant) = gen.reader.field_constant(def) {
        let constant_type = gen.reader.constant_type(constant);

        if ty == constant_type {
            if ty == Type::String {
                let crate_name = gen.crate_name();
                if gen.reader.field_is_ansi(def) {
                    let value = gen.value(&gen.reader.constant_value(constant));
                    quote! {
                        #doc
                        #features
                        pub const #name: ::#crate_name::core::PCSTR = ::#crate_name::s!(#value);
                    }
                } else {
                    let value = gen.value(&gen.reader.constant_value(constant));
                    quote! {
                        #doc
                        #features
                        pub const #name: ::#crate_name::core::PCWSTR = ::#crate_name::w!(#value);
                    }
                }
            } else {
                let value = gen.typed_value(&gen.reader.constant_value(constant));
                quote! {
                    #doc
                    #features
                    pub const #name: #value;
                }
            }
        } else {
            let kind = gen.type_default_name(&ty);
            let value = gen.value(&gen.reader.constant_value(constant));

            let value = if gen.reader.type_underlying_type(&ty) == constant_type {
                value
            // TODO: workaround for https://github.com/microsoft/win32metadata/issues/1029
            } else if ty == Type::PCWSTR && value.0.starts_with('-') {
                quote! { #value as u16 as _ }
            } else {
                quote! { #value as _ }
            };

            if !gen.sys && gen.reader.type_has_replacement(&ty) {
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
    } else if let Some(guid) = gen.reader.field_guid(def) {
        let value = gen.guid(&guid);
        let guid = gen.type_name(&Type::GUID);
        quote! {
            #doc
            pub const #name: #guid = #value;
        }
    } else if let Some((guid, id)) = get_property_key(gen, def) {
        let kind = gen.type_default_name(&ty);
        let guid = gen.guid(&guid);
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

fn get_property_key(gen: &Gen, def: Field) -> Option<(GUID, u32)> {
    gen.reader.field_attributes(def).find(|attribute| gen.reader.attribute_name(*attribute) == "PropertyKeyAttribute").map(|attribute| {
        let args = gen.reader.attribute_args(attribute);
        let id = match args[11].1 {
            Value::U32(value) => value,
            _ => unimplemented!(),
        };
        (GUID::from_args(&args), id)
    })
}
