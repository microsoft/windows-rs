use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Struct(pub tables::TypeDef);

impl Struct {
    pub fn type_signature(&self) -> String {
        let mut result = format!("struct({}.{}", self.0.namespace(), self.0.name());

        for field in self.0.fields() {
            result.push(';');
            result.push_str(&field.signature().kind.type_signature());
        }

        result.push(')');
        result
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0.fields().map(|f| f.definition()).flatten().collect()
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        vec![self.0]
    }

    pub fn is_blittable(&self) -> bool {
        self.0.fields().all(|f| f.is_blittable())
    }

    fn is_handle(&self) -> bool {
        self.0
            .has_attribute("Windows.Win32.Interop", "NativeTypedefAttribute")
    }

    pub fn gen_abi_name(&self, gen: Gen) -> TokenStream {
        if self.is_blittable() {
            self.0.gen_name(gen)
        } else {
            self.0.gen_abi_name(gen)
        }
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let name = self.0.gen_name(gen);

        let runtime_type = if self.0.is_winrt() {
            let signature = Literal::byte_string(&self.type_signature().as_bytes());

            quote! {
                unsafe impl ::windows::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
                }
            }
        } else {
            quote! {}
        };

        let is_handle = self.is_handle();
        let is_empty = self.0.fields().next().is_none();

        let copy = if is_handle {
            quote! {
                impl ::std::marker::Copy for #name {}
            }
        } else {
            quote! {}
        };

        let body = if is_handle {
            let fields = self.0.fields().map(|f| {
                let kind = f.signature().gen(gen);
                quote! {
                    pub #kind
                }
            });

            quote! {
                ( #(#fields),* );
            }
        } else {
            let fields = self.0.fields().map(|f| {
                let name = f.gen_name();
                let kind = f.signature().gen(gen);
                quote! {
                    pub #name: #kind
                }
            });

            quote! {
                { #(#fields),* }
            }
        };

        let abi = if self.is_blittable() {
            quote! {
                unsafe impl ::windows::Abi for #name {
                    type Abi = Self;
                }
            }
        } else {
            let abi_name = self.0.gen_abi_name(gen);
            let fields = self.0.fields().map(|f| f.signature().gen_abi(gen));

            quote! {
                #[repr(C)]
                #[doc(hidden)]
                pub struct #abi_name(#(#fields),*);
                unsafe impl ::windows::Abi for #name {
                    type Abi = #abi_name;
                }
            }
        };

        let constants = self.0.fields().filter_map(|f| {
            if f.flags().literal() {
                if let Some(constant) = f.constant() {
                    let name = to_ident(f.name());
                    let value = constant.value().gen();

                    return Some(quote! {
                        pub const #name: #value;
                    });
                }
            }

            None
        });

        let compare = if is_empty {
            quote! { true }
        } else {
            let fields = self.0.fields().enumerate().map(|(index, f)| {
                let name = f.gen_name();

                if let ElementType::Callback(_) = f.signature().kind {
                    quote! {
                        self.#name.map(|f| f as usize) == other.#name.map(|f| f as usize)
                    }
                } else if is_handle {
                    let index = Literal::u32_unsuffixed(index as u32);
    
                    quote! {
                        self.#index == other.#index
                    }
                } else {
                    quote! {
                        self.#name == other.#name
                    }
                }
            });

            quote! { #(#fields)&&* }
        };

        let defaults = if is_handle {
            let defaults = self.0.fields().map(|f| f.signature().gen_default());

            quote! {
                Self( #(#defaults),* )
            }
        } else {
            let defaults = self.0.fields().map(|f| {
                let name = f.gen_name();
                let value = f.signature().gen_default();
                quote! {
                    #name: #value
                }
            });

            quote! {
                Self{ #(#defaults),* }
            }
        };

        let debug_name = self.0.name();

        let debug_fields = self.0.fields().enumerate().filter_map(|(index, field)|{
            // TODO: there must be a simpler way to implement Debug just to exclude this type.
            if let ElementType::Callback(_) = field.signature().kind {
                return None;
            }

            let name = to_snake(field.name());

            if is_handle {
                let index = Literal::u32_unsuffixed(index as u32);

                Some(quote! {
                    .field(#name, &format_args!("{:?}", self.#index))
                })
            } else {
                let field = to_ident(&name);

                Some(quote! {
                    .field(#name, &format_args!("{:?}", self.#field))
                })
            }
        });

        quote! {
            #[repr(C)]
            #[allow(non_snake_case)]
            #[derive(::std::clone::Clone)]
            pub struct #name #body
            impl #name {
                #(#constants)*
            }
            impl ::std::default::Default for #name {
                fn default() -> Self {
                    #defaults
                }
            }
            impl ::std::fmt::Debug for #name {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct(#debug_name)
                        #(#debug_fields)*
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    #compare
                }
            }
            impl ::std::cmp::Eq for #name {}
            #abi
            #copy
            #runtime_type
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t = TypeReader::get_struct("Windows.Foundation", "Point");
        assert_eq!(t.type_signature(), "struct(Windows.Foundation.Point;f4;f4)");
    }

    #[test]
    fn test_fields() {
        let t = TypeReader::get_struct("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let f: Vec<tables::Field> = t.0.fields().collect();
        assert_eq!(f.len(), 7);

        assert_eq!(f[0].name(), "PresentCount");
        assert_eq!(f[1].name(), "PresentRefreshCount");
        assert_eq!(f[2].name(), "SyncRefreshCount");
        assert_eq!(f[3].name(), "SyncQPCTime");
        assert_eq!(f[4].name(), "SyncGPUTime");
        assert_eq!(f[5].name(), "CompositionMode");
        assert_eq!(f[6].name(), "ApprovedPresentDuration");

        assert_eq!(f[0].signature().kind, ElementType::U32);
        assert_eq!(f[1].signature().kind, ElementType::U32);
        assert_eq!(f[2].signature().kind, ElementType::U32);
        assert_eq!(f[3].signature().kind, ElementType::I64);
        assert_eq!(f[4].signature().kind, ElementType::I64);
        assert_eq!(
            f[5].signature().kind,
            ElementType::Enum(TypeReader::get_enum(
                "Windows.Win32.Dxgi",
                "DXGI_FRAME_PRESENTATION_MODE"
            ))
        );
        assert_eq!(f[6].signature().kind, ElementType::U32);
    }

    #[test]
    fn test_dependencies() {
        let t = TypeReader::get_struct("Windows.Foundation", "Point");
        assert_eq!(t.dependencies().len(), 0);

        let t = TypeReader::get_struct("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS");
        assert_eq!(t.dependencies().len(), 0);

        let t = TypeReader::get_struct("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let deps = t.dependencies();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name(), "DXGI_FRAME_PRESENTATION_MODE");
    }

    #[test]
    fn test_blittable() {
        assert_eq!(
            TypeReader::get_struct("Windows.Foundation", "Point").is_blittable(),
            true
        );
        assert_eq!(
            TypeReader::get_struct("Windows.UI.Xaml.Interop", "TypeName").is_blittable(),
            false
        );
    }
}
