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
            Type::Item(item) => self.write_item_name(&item),
            Type::PtrMut(ty, pointers) => {
                let pointers = write_ptr_mut(*pointers);
                let ty = self.write_default_name(ty);
                quote! { #pointers #ty }
            }
            Type::PtrConst(ty, pointers) => {
                let pointers = write_ptr_const(*pointers);
                let ty = self.write_default_name(ty);
                quote! { #pointers #ty }
            }
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

    fn write_item_name(&self, item: &Item) -> TokenStream {
        match item {
            Item::Struct(item) => {
                let name = to_ident(item.def.name());
                let namespace = self.write_namespace(item.def.namespace());
                quote! { #namespace #name }
            }
            Item::CppStruct(item) => {
                let name = to_ident(item.def.name());
                let namespace = self.write_namespace(item.def.namespace());
                quote! { #namespace #name }
            }
            rest => panic!("windows-bindgen: {rest:?}"),
        }
    }

    fn write_namespace(&self, namespace: &str) -> TokenStream {
        if self.flat || namespace == self.namespace {
            return quote! {};
        }

        // TODO: should this be more general than just "Windows.*"?
        let is_external =
            namespace.starts_with("Windows.") && !self.namespace.starts_with("Windows");
        let mut relative = self.namespace.split('.').peekable();
        let mut namespace = namespace.split('.').peekable();

        while relative.peek() == namespace.peek() {
            if relative.next().is_none() {
                break;
            }

            namespace.next();
        }

        let mut tokens = TokenStream::new();

        if is_external {
            tokens.push_str("windows::");
            namespace.next();
        } else {
            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }
        }

        for namespace in namespace {
            tokens.push_str(namespace);
            tokens.push_str("::");
        }

        tokens
    }
}

fn write_ptr_mut(pointers: usize) -> TokenStream {
    "*mut ".repeat(pointers).into()
}

fn write_ptr_const(pointers: usize) -> TokenStream {
    "*const ".repeat(pointers).into()
}
