use super::*;

impl Writer {
    pub fn write_core(&self) -> TokenStream {
        if self.config.no_deps {
            if self.config.flat {
                quote! {}
            } else {
                let mut tokens = TokenStream::new();

                for _ in 0..self.namespace.split('.').count() {
                    tokens.push_str("super::");
                }

                tokens
            }
        } else if self.config.sys {
                quote! { windows_sys::core:: }
        } else {
            quote! { windows_core:: }
        }
    }

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
            Type::BSTR => {
                let name = self.write_core();
                quote! { #name BSTR }
            }
            Type::IUnknown => {
                if self.config.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let name = self.write_core();
                    quote! { #name IUnknown }
                }
            }
            Type::GUID => {
                let name = self.write_core();
                quote! { #name GUID }
            }
            Type::HRESULT => {
                let name = self.write_core();
                quote! { #name HRESULT }
            }
            Type::String => {
                let name = self.write_core();
                quote! { #name HSTRING }
            }
            Type::Object => {
                if self.config.sys {
                    quote! { *mut core::ffi::c_void }
                } else {
                    let name = self.write_core();
                    quote! { #name IInspectable }
                }
            }
            Type::PSTR => {
                let name = self.write_core();
                quote! { #name PSTR }
            }
            Type::PCSTR => {
                let name = self.write_core();
                quote! { #name PCSTR }
            }
            Type::PWSTR => {
                let name = self.write_core();
                quote! { #name PWSTR }
            }
            Type::PCWSTR => {
                let name = self.write_core();
                quote! { #name PCWSTR }
            }
            Type::Item(item) => self.write_item_name(item),
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
            Type::ArrayFixed(ty, len) => {
                let name = self.write_default_name(ty);
                let len = Literal::usize_unsuffixed(*len);
                quote! { [#name; #len] }
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
            } else if ty.is_nullable() && !self.config.sys {
                quote! { Option<#tokens> }
            } else {
                tokens
            }
        }
    }

    fn write_item_name(&self, item: &Item) -> TokenStream {
        match item {
            Item::CppInterface(_) if self.config.sys => quote! { *mut core::ffi::c_void },
            _ => {
                let name = to_ident(item.name());
                let namespace = self.write_namespace(item.namespace());
                quote! { #namespace #name }
            }
        }

        // match item {
        //     Item::Struct(item) => {
        //         let name = to_ident(item.def.name());
        //         let namespace = self.write_namespace(item.def.namespace());
        //         quote! { #namespace #name }
        //     }
        //     Item::CppStruct(item) => {
        //         let name = to_ident(item.name());
        //         let namespace = self.write_namespace(item.def.namespace());
        //         quote! { #namespace #name }
        //     }
        //     rest => panic!("windows-bindgen: {rest:?}"),
        // }
    }

    fn write_namespace(&self, namespace: &str) -> TokenStream {
        if self.config.flat || namespace.is_empty() || namespace == self.namespace {
            return quote! {};
        }

        if !self.tree.includes_namespace(namespace) {
            todo!("deal with external references `{namespace}`");
        }

        let mut relative = self.namespace.split('.').peekable();
        let mut namespace = namespace.split('.').peekable();

        while relative.peek() == namespace.peek() {
            if relative.next().is_none() {
                break;
            }

            namespace.next();
        }

        let mut tokens = TokenStream::new();

        for _ in 0..relative.count() {
            tokens.push_str("super::");
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
