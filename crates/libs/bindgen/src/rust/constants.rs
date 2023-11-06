use super::*;

pub fn writer(writer: &Writer, def: Field) -> TokenStream {
    let name = to_ident(def.name());
    let ty = def.ty(None).to_const_type();
    let cfg = field_cfg(def);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);

    if let Some(constant) = def.constant() {
        let constant_type = constant.ty();

        if ty == constant_type {
            if ty == Type::String {
                let crate_name = writer.crate_name();
                if field_is_ansi(def) {
                    let value = writer.value(&constant.value());
                    quote! {
                        #doc
                        #features
                        pub const #name: #crate_name PCSTR = #crate_name s!(#value);
                    }
                } else {
                    let value = writer.value(&constant.value());
                    quote! {
                        #doc
                        #features
                        pub const #name: #crate_name PCWSTR = #crate_name w!(#value);
                    }
                }
            } else {
                let value = writer.typed_value(&constant.value());
                quote! {
                    #doc
                    #features
                    pub const #name: #value;
                }
            }
        } else {
            let kind = writer.type_default_name(&ty);
            let value = writer.value(&constant.value());
            let underlying_type = type_underlying_type(&ty);

            let value = if underlying_type == constant_type {
                value
            } else if writer.std && underlying_type == Type::ISize {
                quote! { ::core::ptr::invalid_mut(#value as _) }
            } else {
                quote! { #value as _ }
            };

            if !writer.sys && type_has_replacement(&ty) {
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
    } else if let Some(guid) = field_guid(def) {
        let value = writer.guid(&guid);
        let guid = writer.type_name(&Type::GUID);
        quote! {
            #doc
            pub const #name: #guid = #value;
        }
    } else if let Some(value) = initializer(writer, def) {
        let kind = writer.type_default_name(&ty);

        quote! {
            #doc
            #features
            pub const #name: #kind = #kind { #value };
        }
    } else {
        quote! {}
    }
}

fn initializer(writer: &Writer, def: Field) -> Option<TokenStream> {
    let Some(value) = constant(def) else {
        return None;
    };

    let mut input = value.as_str();

    let Type::TypeDef(def, _) = def.ty(None) else {
        unimplemented!();
    };

    let mut result = quote! {};

    for field in def.fields() {
        let (value, rest) = field_initializer(writer, field, input);
        input = rest;
        result.combine(&value);
    }

    Some(result)
}

fn field_initializer<'a>(writer: &Writer, field: Field, input: &'a str) -> (TokenStream, &'a str) {
    let name = to_ident(field.name());

    match field.ty(None) {
        Type::GUID => {
            let (literals, rest) = read_literal_array(input, 11);
            let value = writer.guid(&Guid::from_string_args(&literals));
            (quote! { #name: #value, }, rest)
        }
        Type::Win32Array(_, len) => {
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

fn constant(def: Field) -> Option<String> {
    def.find_attribute("ConstantAttribute").map(|attribute| {
        let args = attribute.args();
        match &args[0].1 {
            Value::String(value) => value.clone(),
            rest => unimplemented!("{rest:?}"),
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

fn field_guid(row: Field) -> Option<Guid> {
    row.find_attribute("GuidAttribute").map(|attribute| Guid::from_args(&attribute.args()))
}

fn field_is_ansi(row: Field) -> bool {
    row.find_attribute("NativeEncodingAttribute").is_some_and(|attribute| matches!(attribute.args().first(), Some((_, Value::String(encoding))) if encoding == "ansi"))
}

fn type_has_replacement(ty: &Type) -> bool {
    match ty {
        Type::HRESULT | Type::PCSTR | Type::PCWSTR => true,
        Type::TypeDef(row, _) => type_def_is_handle(*row) || row.kind() == TypeKind::Enum,
        _ => false,
    }
}

fn type_underlying_type(ty: &Type) -> Type {
    match ty {
        Type::TypeDef(row, _) => row.underlying_type(),
        Type::HRESULT => Type::I32,
        _ => ty.clone(),
    }
}
