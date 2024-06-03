use super::*;
use metadata::HasAttributes;

pub fn writer(writer: &Writer, def: metadata::Field) -> TokenStream {
    let name = to_ident(def.name());
    let ty = def.ty(None).to_const_type();
    let cfg = cfg::field_cfg(writer, def);
    let features = writer.cfg_features(&cfg);

    if let Some(constant) = def.constant() {
        let constant_type = constant.ty();

        if ty == constant_type {
            if ty == metadata::Type::String {
                let crate_name = writer.crate_name();
                if field_is_ansi(def) {
                    let value = writer.value(&constant.value());
                    quote! {
                        #features
                        pub const #name: #crate_name PCSTR = #crate_name s!(#value);
                    }
                } else {
                    let value = writer.value(&constant.value());
                    quote! {
                        #features
                        pub const #name: #crate_name PCWSTR = #crate_name w!(#value);
                    }
                }
            } else {
                let value = writer.typed_value(&constant.value());
                quote! {
                    #features
                    pub const #name: #value;
                }
            }
        } else {
            let kind = writer.type_default_name(&ty);
            let mut value = writer.value(&constant.value());
            let underlying_type = type_underlying_type(&ty);

            if underlying_type == constant_type {
                if is_signed_error(&ty) {
                    if let metadata::Value::I32(signed) = constant.value() {
                        value = format!("0x{:X}_u32 as _", signed).into();
                    }
                }
            } else {
                value = quote! { #value as _ };
            }

            if !writer.sys && type_has_replacement(&ty) {
                quote! {
                    #features
                    pub const #name: #kind = #kind(#value);
                }
            } else {
                quote! {
                    #features
                    pub const #name: #kind = #value;
                }
            }
        }
    } else if let Some(guid) = field_guid(def) {
        let value = writer.guid(&guid);
        let guid = writer.type_name(&metadata::Type::Name(metadata::TypeName::GUID));
        quote! {
            pub const #name: #guid = #value;
        }
    } else if let Some(value) = initializer(writer, def) {
        let kind = writer.type_default_name(&ty);

        quote! {
            #features
            pub const #name: #kind = #kind { #value };
        }
    } else {
        quote! {}
    }
}

fn is_signed_error(ty: &metadata::Type) -> bool {
    match ty {
        metadata::Type::Name(metadata::TypeName::HResult) => true,
        metadata::Type::TypeDef(def, _) => def.type_name() == metadata::TypeName::NTSTATUS,
        _ => false,
    }
}

fn initializer(writer: &Writer, def: metadata::Field) -> Option<TokenStream> {
    let value = constant(def)?;
    let mut input = value.as_str();

    let metadata::Type::TypeDef(def, _) = def.ty(None) else {
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

fn field_initializer<'a>(
    writer: &Writer,
    field: metadata::Field,
    input: &'a str,
) -> (TokenStream, &'a str) {
    let name = to_ident(field.name());

    match field.ty(None) {
        metadata::Type::Name(metadata::TypeName::GUID) => {
            let (literals, rest) = read_literal_array(input, 11);
            let value = writer.guid(&metadata::Guid::from_string_args(&literals));
            (quote! { #name: #value, }, rest)
        }
        metadata::Type::Win32Array(_, len) => {
            let (literals, rest) = read_literal_array(input, len);
            let literals = literals.iter().map(|literal| TokenStream::from(*literal));
            (quote! { #name: [#(#literals,)*], }, rest)
        }
        metadata::Type::MutPtr(_, _) => {
            // The Win32 metadata uses integer values for initializing pointers. This is a workaround
            // to allow most such cases to work.
            let (_, rest) = read_literal(input);
            (quote! { #name: core::ptr::null_mut(), }, rest)
        }
        _ => {
            let (literal, rest) = read_literal(input);
            let literal: TokenStream = literal.into();
            (quote! { #name: #literal, }, rest)
        }
    }
}

fn constant(def: metadata::Field) -> Option<String> {
    def.find_attribute("ConstantAttribute").map(|attribute| {
        let args = attribute.args();
        match &args[0].1 {
            metadata::Value::String(value) => value.to_string(),
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

fn field_guid(row: metadata::Field) -> Option<metadata::Guid> {
    row.find_attribute("GuidAttribute")
        .map(|attribute| metadata::Guid::from_args(&attribute.args()))
}

fn field_is_ansi(row: metadata::Field) -> bool {
    row.find_attribute("NativeEncodingAttribute").is_some_and(|attribute| matches!(attribute.args().first(), Some((_, metadata::Value::String(encoding))) if encoding == "ansi"))
}

fn type_has_replacement(ty: &metadata::Type) -> bool {
    match ty {
        metadata::Type::Name(metadata::TypeName::HResult)
        | metadata::Type::Const(metadata::TypeName::PSTR)
        | metadata::Type::Const(metadata::TypeName::PWSTR) => true,
        metadata::Type::TypeDef(row, _) => {
            metadata::type_def_is_handle(*row) || row.kind() == metadata::TypeKind::Enum
        }
        _ => false,
    }
}

fn type_underlying_type(ty: &metadata::Type) -> metadata::Type {
    match ty {
        metadata::Type::TypeDef(row, _) => row.underlying_type(),
        metadata::Type::Name(metadata::TypeName::HResult) => metadata::Type::I32,
        _ => ty.clone(),
    }
}
