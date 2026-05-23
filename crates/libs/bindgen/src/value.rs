use super::*;

pub use windows_metadata::Value;

pub trait ValueExt {
    fn write(&self) -> TokenStream;

    /// Returns true if this value is a floating-point literal close enough
    /// to a `core::{f32,f64}::consts` constant that `clippy::approx_constant`
    /// would flag it. Used by the constant emitters to attach a per-item
    /// `#[allow(clippy::approx_constant)]` on the affected `pub const`, so
    /// the lint can stay enabled for hand-written code in consuming crates.
    fn is_approx_constant(&self) -> bool;
}

impl ValueExt for Value {
    fn write(&self) -> TokenStream {
        match self {
            Self::Bool(value) => quote! { #value },
            Self::U8(value) => quote! { #value },
            Self::I8(value) => quote! { #value },
            Self::U16(value) => quote! { #value },
            Self::I16(value) => quote! { #value },
            Self::U32(value) => quote! { #value },
            Self::I32(value) => quote! { #value },
            Self::U64(value) => quote! { #value },
            Self::I64(value) => quote! { #value },
            Self::F32(value) => quote! { #value },
            Self::F64(value) => quote! { #value },
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

    fn is_approx_constant(&self) -> bool {
        let value = match self {
            Self::F32(v) => *v as f64,
            Self::F64(v) => *v,
            _ => return false,
        };

        if !value.is_finite() {
            return false;
        }

        let abs = value.abs();
        if abs == 0.0 {
            return false;
        }

        // Mirror clippy's `approx_constant` lint: it fires when a float literal
        // matches one of these constants to ~4 significant digits. We use a
        // relative tolerance of 1e-4 which catches the same cases without
        // producing false positives on unrelated near-by values.
        const CONSTANTS: &[f64] = &[
            core::f64::consts::E,
            core::f64::consts::FRAC_1_PI,
            core::f64::consts::FRAC_1_SQRT_2,
            core::f64::consts::FRAC_2_PI,
            core::f64::consts::FRAC_2_SQRT_PI,
            core::f64::consts::FRAC_PI_2,
            core::f64::consts::FRAC_PI_3,
            core::f64::consts::FRAC_PI_4,
            core::f64::consts::FRAC_PI_6,
            core::f64::consts::FRAC_PI_8,
            core::f64::consts::LN_10,
            core::f64::consts::LN_2,
            core::f64::consts::LOG10_2,
            core::f64::consts::LOG10_E,
            core::f64::consts::LOG2_10,
            core::f64::consts::LOG2_E,
            core::f64::consts::PI,
            core::f64::consts::SQRT_2,
            core::f64::consts::TAU,
        ];

        CONSTANTS
            .iter()
            .any(|c| (value - c).abs() / c.abs() <= 1e-4)
    }
}

impl Config<'_> {
    pub fn write_cpp_const_guid(&self, name: TokenStream, value: &GUID) -> TokenStream {
        let crate_name = self.write_core();
        let value = self.write_guid_u128(value);

        quote! {
            pub const #name: #crate_name GUID = #crate_name GUID::from_u128(#value);
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
                let crate_name = self.write_core();
                (quote! { #name: #crate_name GUID::from_u128(#value), }, rest)
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
