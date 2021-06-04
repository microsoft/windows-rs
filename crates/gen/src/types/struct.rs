use super::*;

// TODO: need to split win32 and winrt structs as their signatures are different and win32 structs also include unions and they are
// radically different.

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Struct(pub tables::TypeDef);

impl Struct {
    pub fn gen(&self, gen: &Gen) -> TokenStream {
        self.gen_struct(self.0.name(), gen)
    }

    fn gen_struct(&self, struct_name: &str, gen: &Gen) -> TokenStream {
        if let Some(replacement) = self.gen_replacement() {
            return replacement;
        }

        let name = to_ident(struct_name);

        if let Some(guid) = Guid::from_attributes(self.0.attributes()) {
            let guid = guid.gen();

            return quote! {
                pub const #name: ::windows::Guid = ::windows::Guid::from_values(#guid);
            };
        }

        let fields: Vec<(tables::Field, Signature, Ident)> = self
            .0
            .fields()
            .filter_map(|f| {
                if f.flags().literal() {
                    None
                } else {
                    Some((f, f.signature(), to_ident(f.name())))
                }
            })
            .collect();

        if fields.is_empty() {
            return quote! {
                #[repr(C)]
                #[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug, ::std::cmp::PartialEq, ::std::cmp::Eq, ::std::marker::Copy)]
                pub struct #name(pub u8);
            };
        }

        let is_winrt = self.0.is_winrt();
        let is_handle = self.0.is_handle();
        let is_union = self.0.flags().explicit();
        let layout = self.0.class_layout();
        let is_packed = self.0.is_packed();

        let repr = if let Some(layout) = layout {
            let packing = Literal::u32_unsuffixed(layout.packing_size());
            quote! { #[repr(C, packed(#packing))] }
        } else if is_handle {
            quote! { #[repr(transparent)] }
        } else {
            quote! { #[repr(C)] }
        };

        // TODO: add test for Windows.Win32.Security.TRUSTEE_A
        let has_union = fields
            .iter()
            .any(|(_, signature, _)| signature.is_explicit());

        // TODO: workaround for getting windows-docs building
        let has_complex_array = fields
            .iter()
            .any(|(_, signature, _)| match &signature.kind {
                ElementType::Array((signature, _)) => {
                    !signature.is_blittable() || signature.kind.is_nullable()
                }
                _ => false,
            });

        let runtime_type = if is_winrt {
            let signature = Literal::byte_string(&self.0.type_signature().as_bytes());

            quote! {
                unsafe impl ::windows::RuntimeType for #name {
                    type DefaultType = Self;
                    const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(#signature);
                }
            }
        } else {
            quote! {}
        };

        let clone_or_copy = if self.0.is_blittable() {
            quote! {
                #[derive(::std::clone::Clone, ::std::marker::Copy)]
            }
        } else if is_union || has_union || is_packed {
            quote! {}
        } else {
            quote! {
                #[derive(::std::clone::Clone)]
            }
        };

        let body = if is_handle {
            let fields = fields.iter().map(|(_, signature, _)| {
                let kind = if is_winrt {
                    signature.gen_winrt(gen)
                } else {
                    signature.gen_win32(gen)
                };

                quote! {
                    pub #kind
                }
            });

            quote! {
                ( #(#fields),* );
            }
        } else {
            let fields = fields.iter().map(|(_, signature, name)| {
                let kind = if is_winrt {
                    signature.gen_winrt(gen)
                } else if is_union {
                    signature.gen_win32_abi(gen)
                } else {
                    signature.gen_win32(gen)
                };

                quote! {
                    pub #name: #kind
                }
            });

            quote! {
                { #(#fields),* }
            }
        };

        let struct_or_union = if is_union {
            quote! { union }
        } else {
            quote! { struct }
        };

        let abi = if self.0.is_blittable() {
            quote! {
                unsafe impl ::windows::Abi for #name {
                    type Abi = Self;
                }
            }
        } else {
            let abi_name = self.0.gen_abi_name(gen);

            let fields = if is_winrt {
                let fields = fields.iter().map(|(_, signature, name)| {
                    let kind = signature.gen_winrt_abi(gen);
                    quote! { pub #name: #kind }
                });
                quote! { #(#fields),* }
            } else {
                let fields = fields.iter().map(|(_, signature, name)| {
                    let kind = signature.gen_win32_abi(gen);
                    quote! { pub #name: #kind }
                });
                quote! { #(#fields),* }
            };

            quote! {
                #repr
                #[doc(hidden)]
                #[derive(::std::clone::Clone, ::std::marker::Copy)]
                pub #struct_or_union #abi_name{ #fields }
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

        let compare = if is_union | has_union | has_complex_array | is_packed {
            quote! {}
        } else {
            let compare = fields
                .iter()
                .enumerate()
                .map(|(index, (_, signature, name))| {
                    let is_callback = matches!(signature.kind, ElementType::Callback(_));

                    if is_callback && signature.pointers == 0 {
                        quote! {
                            self.#name.map(|f| f as usize) == other.#name.map(|f| f as usize)
                        }
                    } else if is_handle {
                        let index = Literal::usize_unsuffixed(index);

                        quote! {
                            self.#index == other.#index
                        }
                    } else {
                        quote! {
                            self.#name == other.#name
                        }
                    }
                });

            if layout.is_some() {
                quote! {
                    impl ::std::cmp::PartialEq for #name {
                        fn eq(&self, other: &Self) -> bool {
                            unsafe { #(#compare)&&* }
                        }
                    }
                    impl ::std::cmp::Eq for #name {}
                }
            } else {
                quote! {
                    impl ::std::cmp::PartialEq for #name {
                        fn eq(&self, other: &Self) -> bool {
                            #(#compare)&&*
                        }
                    }
                    impl ::std::cmp::Eq for #name {}
                }
            }
        };

        let default = if is_union || has_union || has_complex_array || is_packed {
            quote! {}
        } else {
            let defaults = fields.iter().map(|(_, signature, name)| {
                let value = if is_winrt {
                    signature.gen_winrt_default()
                } else {
                    signature.gen_win32_default()
                };

                if is_handle {
                    value
                } else {
                    quote! {
                        #name: #value
                    }
                }
            });

            let defaults = quote! { #(#defaults),* };

            if is_handle {
                quote! {
                    impl ::std::default::Default for #name {
                        fn default() -> Self {
                            Self(#defaults)
                        }
                    }
                    impl #name {
                        pub const NULL: Self = Self(#defaults);
                        pub fn is_null(&self) -> bool {
                            self.0 == #defaults
                        }
                    }
                }
            } else {
                quote! {
                    impl ::std::default::Default for #name {
                        fn default() -> Self {
                            Self{ #defaults }
                        }
                    }
                }
            }
        };

        let debug = if is_union || has_union || has_complex_array || is_packed {
            quote! {}
        } else {
            let debug_name = self.0.name();

            let debug_fields =
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(index, (_, signature, name))| {
                        // TODO: there must be a simpler way to implement Debug just to exclude this type.
                        match &signature.kind {
                            ElementType::Callback(_) => return None,
                            ElementType::Array((kind, _)) => {
                                if let ElementType::Callback(_) = kind.kind {
                                    return None;
                                }
                            }
                            _ => {}
                        }

                        let field = name.as_str();

                        if is_handle {
                            let index = Literal::usize_unsuffixed(index);

                            Some(quote! {
                                .field(#field, &format_args!("{:?}", self.#index))
                            })
                        } else {
                            Some(quote! {
                                .field(#field, &format_args!("{:?}", self.#name))
                            })
                        }
                    });

            if layout.is_some() {
                quote! {
                    impl ::std::fmt::Debug for #name {
                        fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            unsafe {
                                fmt.debug_struct(#debug_name)
                                    #(#debug_fields)*
                                    .finish()
                            }
                        }
                    }
                }
            } else {
                quote! {
                    impl ::std::fmt::Debug for #name {
                        fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            fmt.debug_struct(#debug_name)
                                #(#debug_fields)*
                                .finish()
                        }
                    }
                }
            }
        };

        let extensions = self.gen_extensions();
        let nested_types = gen_nested_types(struct_name, &self.0, gen);

        let convertible = if let Some(dependency) = self.0.is_convertible_to() {
            let dependency = dependency.gen_name(gen);

            quote! {
                impl<'a> ::windows::IntoParam<'a, #dependency> for #name {
                    fn into_param(self) -> ::windows::Param<'a, #dependency> {
                        ::windows::Param::Owned(#dependency(self.0))
                    }
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #repr
            #clone_or_copy
            pub #struct_or_union #name #body
            impl #name {
                #(#constants)*
            }
            #default
            #debug
            #compare
            #abi
            #runtime_type
            #extensions
            #nested_types
            #convertible
        }
    }

    fn gen_replacement(&self) -> Option<TokenStream> {
        match self.0.full_name() {
            ("Windows.Win32.System.SystemServices", "BOOL") => Some(gen_bool32()),
            ("Windows.Win32.System.SystemServices", "PWSTR") => Some(gen_pwstr()),
            ("Windows.Win32.System.SystemServices", "PSTR") => Some(gen_pstr()),
            ("Windows.Win32.System.OleAutomation", "BSTR") => Some(gen_bstr()),
            _ => None,
        }
    }

    fn gen_extensions(&self) -> TokenStream {
        match self.0.full_name() {
            ("Windows.Foundation", "TimeSpan") => gen_timespan(),
            ("Windows.Foundation.Numerics", "Vector2") => gen_vector2(),
            ("Windows.Foundation.Numerics", "Vector3") => gen_vector3(),
            ("Windows.Foundation.Numerics", "Vector4") => gen_vector4(),
            ("Windows.Foundation.Numerics", "Matrix3x2") => gen_matrix3x2(),
            ("Windows.Foundation.Numerics", "Matrix4x4") => gen_matrix4x4(),
            ("Windows.Win32.System.SystemServices", "HANDLE") => gen_handle(),
            _ => TokenStream::new(),
        }
    }
}

fn gen_nested_types<'a>(
    enclosing_name: &'a str,
    enclosing_type: &'a tables::TypeDef,
    gen: &Gen,
) -> TokenStream {
    if let Some(nested_types) = enclosing_type.nested_types() {
        nested_types
            .iter()
            .enumerate()
            .map(|(index, (_, nested_type))| {
                let nested_name = format!("{}_{}", enclosing_name, index);
                Struct(nested_type.clone()).gen_struct(&nested_name, gen)
            })
            .collect()
    } else {
        TokenStream::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let t = TypeReader::get_struct("Windows.Foundation", "Point");
        assert_eq!(
            t.0.type_signature(),
            "struct(Windows.Foundation.Point;f4;f4)"
        );
    }

    #[test]
    fn test_fields() {
        let t =
            TypeReader::get_struct("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
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
                "Windows.Win32.Graphics.Dxgi",
                "DXGI_FRAME_PRESENTATION_MODE"
            ))
        );
        assert_eq!(f[6].signature().kind, ElementType::U32);
    }

    #[test]
    fn test_dependencies() {
        let t = TypeReader::get_struct("Windows.Foundation", "Point");
        assert_eq!(t.0.dependencies().len(), 0);

        let t = TypeReader::get_struct("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS");
        assert_eq!(t.0.dependencies().len(), 0);

        let t =
            TypeReader::get_struct("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let deps = t.0.dependencies();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name(), "DXGI_FRAME_PRESENTATION_MODE");
    }

    #[test]
    fn test_blittable() {
        assert_eq!(
            TypeReader::get_struct("Windows.Foundation", "Point")
                .0
                .is_blittable(),
            true
        );
        assert_eq!(
            TypeReader::get_struct("Windows.UI.Xaml.Interop", "TypeName")
                .0
                .is_blittable(),
            false
        );
    }
}
