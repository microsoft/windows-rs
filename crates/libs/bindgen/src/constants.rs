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
    } else if let Some(value) = initializer(gen, def) {
        let kind = gen.type_default_name(&ty);

        quote! {
            #doc
            #features
            pub const #name: #kind = #kind { #value };
        }
    } else {
        quote! {}
    }
}

fn initializer(gen: &Gen, def: Field) -> Option<TokenStream> {
    let Some(value) = constant(gen, def) else {
        return None;
    };

    let mut input = value.as_str();

    let Type::TypeDef((def, _)) = gen.reader.field_type(def, None) else {
        unimplemented!();
    };

    let mut result = quote! {};

    for field in gen.reader.type_def_fields(def) {
        let (value, rest) = field_initializer(gen, field, input);
        input = rest;
        result.combine(&value);
    }

    Some(result)
}

fn field_initializer<'a>(gen: &Gen, field: Field, input: &'a str) -> (TokenStream, &'a str) {
    let name = to_ident(gen.reader.field_name(field));

    match gen.reader.field_type(field, None) {
        Type::GUID => {
            let (literals, rest) = read_literal_array(input, 11);
            let value = gen.guid(&GUID::from_string_args(&literals));
            (quote! { #name: #value, }, rest)
        }
        Type::Win32Array((_, len)) => {
            let (literals, rest) = read_literal_array(input, len);
            let literals = literals.iter().map(|literal| TokenStream::from(*literal));
            (quote! { #name: [#(#literals,)*], }, rest)
        }
        _ => {
            let (literal, rest) = read_literal(input);
            let literal: TokenStream = literal.into();
            (quote! { #name: #literal, }, rest)
        }
    }
}

fn constant(gen: &Gen, def: Field) -> Option<String> {
    gen.reader
        .field_attributes(def)
        .find(|attribute| gen.reader.attribute_name(*attribute) == "ConstantAttribute")
        .map(|attribute| {
            let args = gen.reader.attribute_args(attribute);
            match &args[0].1 {
                Value::String(value) => value.clone(),
                _ => unimplemented!(),
            }
        })
}

fn read_literal(input: &str) -> (&str, &str) {
    let mut start = None;
    let mut end = 0;

    for (pos, c) in input.bytes().enumerate() {
        if start.is_none() {
            if c != b' ' && c != b',' {
                start = Some(pos);
            }
        } else if c == b' ' || c == b',' || c == b'}' {
            break;
        }
        end += 1;
    }

    let Some(start) = start else {
        unimplemented!();
    };

    (&input[start..end], &input[end..])
}

fn read_token(input: &str, token: u8) -> &str {
    for (pos, c) in input.bytes().enumerate() {
        if c == token {
            return &input[pos + 1..];
        } else if c != b' ' && c != b',' {
            break;
        }
    }

    panic!("`{}` expected", token.escape_ascii());
}

fn read_literal_array(input: &str, len: usize) -> (Vec<&str>, &str) {
    let mut input = read_token(input, b'{');
    let mut result = vec![];

    for _ in 0..len {
        let (literal, rest) = read_literal(input);
        result.push(literal);
        input = rest;
    }

    (result, read_token(input, b'}'))
}
