use super::*;

impl Writer {
    pub fn write_name(&self, ty: &Type) -> TokenStream {
        match ty {
            Type::Void => quote! { core::ffi::c_void },
            Type::Bool => quote! { bool },
            Type::Char => quote! { u16 },
            Type::I8 => quote! { i8 },
            Type::U8 => quote! { u8 },
            Type::I16 => quote! { i16 },
            Type::U16 => quote! { u16 },
            Type::I32 => quote! { i32 },
            Type::U32 => quote! { u32 },
            Type::I64 => quote! { i64 },
            Type::U64 => quote! { u64 },
            Type::F32 => quote! { f32 },
            Type::F64 => quote! { f64 },
            Type::ISize => quote! { isize },
            Type::USize => quote! { usize },
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    pub fn write_default_name(&self, ty: &Type) -> TokenStream {
        if let Type::Array(ty) = ty {
            self.write_default_name(ty)
        } else {
            let tokens = self.write_name(ty);

            if matches!(ty, Type::Param(_)) {
                quote! { <#tokens as windows_core::Type<#tokens>>::Default }
            } else if ty.is_nullable() && !self.sys {
                quote! { Option<#tokens> }
            } else {
                tokens
            }
        }
    }
}
