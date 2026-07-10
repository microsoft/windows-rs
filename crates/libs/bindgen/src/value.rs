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
            Self::USize(value) => {
                let lit = Literal::u64_unsuffixed(*value);
                quote! { #lit }
            }
            Self::ISize(value) => {
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
}
