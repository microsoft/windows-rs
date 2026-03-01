use super::*;

pub use windows_metadata::Value;

pub trait ValueExt {
    fn write(&self) -> TokenStream;
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
                tokens.into()
            }
            rest => panic!("{rest:?}"),
        }
    }
}
