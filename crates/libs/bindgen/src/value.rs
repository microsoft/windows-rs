use super::*;

pub use windows_metadata::Value;

pub trait ValueExt {
    fn write(&self) -> TokenStream;
}

impl ValueExt for Value {
    fn write(&self) -> TokenStream {
        match self {
            Self::Bool(value) => quote! { #value },
            Self::U8(value) => {
                let lit = Literal::u8_unsuffixed(*value);
                quote! { #lit }
            }
            Self::I8(value) => {
                let lit = Literal::i8_unsuffixed(*value);
                quote! { #lit }
            }
            Self::U16(value) => {
                let lit = Literal::u16_unsuffixed(*value);
                quote! { #lit }
            }
            Self::I16(value) => {
                let lit = Literal::i16_unsuffixed(*value);
                quote! { #lit }
            }
            Self::U32(value) => {
                let lit = Literal::u32_unsuffixed(*value);
                quote! { #lit }
            }
            Self::I32(value) => {
                let lit = Literal::i32_unsuffixed(*value);
                quote! { #lit }
            }
            Self::U64(value) => {
                let lit = Literal::u64_unsuffixed(*value);
                quote! { #lit }
            }
            Self::I64(value) => {
                let lit = Literal::i64_unsuffixed(*value);
                quote! { #lit }
            }
            Self::F32(value) => {
                let lit = Literal::f32_unsuffixed(*value);
                quote! { #lit }
            }
            Self::F64(value) => {
                let lit = Literal::f64_unsuffixed(*value);
                quote! { #lit }
            }
            Self::Utf16(value) => {
                let mut tokens = "\"".to_string();

                for u in value.chars() {
                    write!(tokens, "{}", u.escape_default()).unwrap();
                }

                tokens.push('\"');
                tokens.parse().unwrap()
            }
            rest => panic!("{rest:?}"),
        }
    }
}

impl Config<'_> {
    pub fn write_cpp_const_guid(&self, name: TokenStream, value: &GUID) -> TokenStream {
        let crate_name = self.write_core();
        let value = self.write_guid_value(value);

        quote! {
            pub const #name: #crate_name GUID = #value;
        }
    }

    pub fn write_guid_value(&self, value: &GUID) -> TokenStream {
        if self.bindgen.uses_inline_core_types() {
            let crate_name = self.write_core();
            let data1 = format!("0x{:08x?}", value.0)
                .parse::<TokenStream>()
                .unwrap();
            let data2 = format!("0x{:04x?}", value.1)
                .parse::<TokenStream>()
                .unwrap();
            let data3 = format!("0x{:04x?}", value.2)
                .parse::<TokenStream>()
                .unwrap();
            let data4_0 = Literal::u8_unsuffixed(value.3);
            let data4_1 = Literal::u8_unsuffixed(value.4);
            let data4_2 = Literal::u8_unsuffixed(value.5);
            let data4_3 = Literal::u8_unsuffixed(value.6);
            let data4_4 = Literal::u8_unsuffixed(value.7);
            let data4_5 = Literal::u8_unsuffixed(value.8);
            let data4_6 = Literal::u8_unsuffixed(value.9);
            let data4_7 = Literal::u8_unsuffixed(value.10);

            quote! {
                #crate_name GUID { data1: #data1, data2: #data2, data3: #data3, data4: [#data4_0, #data4_1, #data4_2, #data4_3, #data4_4, #data4_5, #data4_6, #data4_7] }
            }
        } else {
            let crate_name = self.write_core();
            let u128_value = self.write_guid_u128(value);
            quote! {
                #crate_name GUID::from_u128(#u128_value)
            }
        }
    }

    pub fn write_guid_u128(&self, value: &GUID) -> TokenStream {
        format!(
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
        .parse()
        .unwrap()
    }

    pub fn field_initializer<'a>(&self, field: Field, input: &'a str) -> (TokenStream, &'a str) {
        let name = to_ident(field.name());

        match field.field_type(None, self.reader) {
            Type::GUID => {
                let (literals, rest) = read_literal_array(input, 11);
                let value = self.write_guid_value(&GUID(
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
                let literals = literals
                    .iter()
                    .map(|literal| literal.parse::<TokenStream>().unwrap());
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
                let literal: TokenStream = literal.parse().unwrap();
                (quote! { #name: #literal, }, rest)
            }
        }
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
        panic!();
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
