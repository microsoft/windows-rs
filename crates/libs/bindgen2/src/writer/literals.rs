use super::*;

impl Writer {
    pub fn write_value(&self, value: &Value) -> TokenStream {
        match value {
            Value::Bool(value) => quote! { #value },
            Value::U8(value) => quote! { #value },
            Value::I8(value) => quote! { #value },
            Value::U16(value) => quote! { #value },
            Value::I16(value) => quote! { #value },
            Value::U32(value) => quote! { #value },
            Value::I32(value) => quote! { #value },
            Value::U64(value) => quote! { #value },
            Value::I64(value) => quote! { #value },
            Value::F32(value) => quote! { #value },
            Value::F64(value) => quote! { #value },
            Value::String(value) => {
                let mut tokens = "\"".to_string();

                for u in value.chars() {
                    write!(tokens, "{}", u.escape_default()).unwrap();
                }

                tokens.push('\"');
                tokens.into()
            }
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }
}
