use super::*;

impl Writer {
    pub fn write_cpp_const(&self, item: &CppConst) -> TokenStream {
        let name = to_ident(item.field.name());

        // TODO: is this even needed?
        if let Some(guid) = item.field.guid_attribute() {
            return self.write_cpp_const_guid(name, &guid);
        }

        let field_ty = item.field.ty(None).to_const_type();

        let mut dependencies = Dependencies::new();

        if self.package {
            item.dependencies(&mut dependencies, self.minimal);
        }

        let cfg = self.write_cfg(item.field, item.def.namespace(), dependencies, false);

        if let Some(constant) = item.field.constant() {
            let constant_ty = constant.ty();

            if field_ty == constant_ty {
                if field_ty == Type::String {
                    let crate_name = self.write_crate();
                    let value = self.write_value(&constant.value());

                    // TODO: if self.no_deps then write these literals out as byte strings?
                    if is_ansi_encoding(item.field) {
                        quote! {
                            #cfg
                            pub const #name: #crate_name PCSTR = #crate_name s!(#value);
                        }
                    } else {
                        quote! {
                            #cfg
                            pub const #name: #crate_name PCWSTR = #crate_name w!(#value);
                        }
                    }
                } else {
                    // TODO: typed value
                    let ty = self.write_name(&field_ty);
                    let value = self.write_value(&constant.value());

                    quote! {
                        #cfg
                        pub const #name: #ty = #value;
                    }
                }
            } else {
                let underlying_ty = underlying_type(&field_ty);
                let ty = self.write_name(&field_ty);
                let mut value = self.write_value(&constant.value());

                if underlying_ty == constant_ty {
                    if is_signed_error(&field_ty) {
                        if let Value::I32(signed) = constant.value() {
                            value = format!("0x{:X}_u32 as _", signed).into();
                        }
                    }
                } else {
                    value = quote! { #value as _ };
                }

                quote! {
                    #cfg
                    pub const #name: #ty = #value;
                }
            }
        } else if let Some(attribute) = item.field.find_attribute("ConstantAttribute") {
            let args = attribute.args();
            let Some((_, Value::String(input))) = args.first() else {
                panic!()
            };

            let Type::Item(Item::CppStruct(item)) = field_ty else {
                panic!()
            };

            let mut input = input.as_str();
            let mut tokens = quote! {};

            for field in item.def.fields() {
                let (value, rest) = self.field_initializer(field, input);
                input = rest;
                tokens.combine(value);
            }

            let ty = self.write_name(&field_ty);

            quote! {
                #cfg
                pub const #name: #ty = #ty { #tokens };
            }
        } else {
            panic!()
        }
    }

    pub fn write_cpp_const_guid(&self, name: TokenStream, value: &GUID) -> TokenStream {
        let crate_name = self.write_crate();
        let value = self.write_guid_u128(value);

        quote! {
            pub const #name: #crate_name GUID = #value;
        }
    }

    fn write_guid_u128(&self, value: &GUID) -> TokenStream {
        let crate_name = self.write_crate();
        let number: TokenStream = format!(
            "0x{:08x?}_{:04x?}_{:04x?}_{:02x?}{:02x?}_{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}{:02x?}",
            value.0,
            value.1,
            value.2,
            value.3,
            value.4,
            value.5,
            value.6,
            value.7,
            value.8,
            value.9,
            value.10
        )
        .into();

        quote! {
            #crate_name GUID::from_u128(#number)
        }
    }

    // TODO: make method?
    fn field_initializer<'a>(&self, field: Field, input: &'a str) -> (TokenStream, &'a str) {
        let name = to_ident(field.name());

        match field.ty(None) {
            Type::GUID => {
                let (literals, rest) = read_literal_array(input, 11);
                let value = self.write_guid_u128(&GUID(
                    literals[0].parse().unwrap(),
                    literals[1].parse().unwrap(),
                    literals[2].parse().unwrap(),
                    literals[3].parse().unwrap(),
                    literals[4].parse().unwrap(),
                    literals[5].parse().unwrap(),
                    literals[6].parse().unwrap(),
                    literals[7].parse().unwrap(),
                    literals[8].parse().unwrap(),
                    literals[9].parse().unwrap(),
                    literals[10].parse().unwrap(),
                ));
                (quote! { #name: #value, }, rest)
            }
            Type::ArrayFixed(_, len) => {
                let (literals, rest) = read_literal_array(input, len);
                let literals = literals.iter().map(|literal| TokenStream::from(*literal));
                (quote! { #name: [#(#literals,)*], }, rest)
            }
            Type::PtrMut(_, _) => {
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
}

fn is_ansi_encoding(row: Field) -> bool {
    row.find_attribute("NativeEncodingAttribute").is_some_and(|attribute| matches!(attribute.args().first(), Some((_, Value::String(encoding))) if encoding == "ansi"))
}

fn is_signed_error(ty: &Type) -> bool {
    match ty {
        Type::HRESULT => true,
        Type::Item(item) => TypeName(item.namespace(), item.name()) == TypeName::NTSTATUS,
        _ => false,
    }
}

fn underlying_type(ty: &Type) -> Type {
    match ty {
        Type::Item(item) => item.underlying_type(),
        Type::HRESULT => Type::I32,
        _ => ty.clone(),
    }
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
