use super::*;

pub fn standalone_imp(writer: &Writer) -> String {
    let mut types = BTreeSet::new();
    let mut functions = BTreeSet::new();
    let mut constants = BTreeSet::new();

    for item in writer.reader.items(writer.filter) {
        writer
            .reader
            .item_collect_standalone(item.clone(), &mut types);

        match item {
            Item::Type(_) => {}
            Item::Fn(def, namespace) => _ = functions.insert((def, namespace.clone())),
            Item::Const(def) => _ = constants.insert(def),
        }
    }

    let mut sorted = SortedTokens::default();

    for ty in types {
        match ty {
            Type::HRESULT if writer.sys => {
                sorted.insert("HRESULT", quote! { pub type HRESULT = i32; })
            }
            Type::String if writer.sys => sorted.insert(
                "HSTRING",
                quote! { pub type HSTRING = *mut ::core::ffi::c_void; },
            ),
            Type::IUnknown if writer.sys => sorted.insert(
                "IUnknown",
                quote! { pub type IUnknown = *mut ::core::ffi::c_void; },
            ),
            Type::IInspectable if writer.sys => sorted.insert(
                "IInspectable",
                quote! { pub type IInspectable = *mut ::core::ffi::c_void; },
            ),
            Type::PSTR if writer.sys => sorted.insert("PSTR", quote! { pub type PSTR = *mut u8; }),
            Type::PWSTR if writer.sys => {
                sorted.insert("PWSTR", quote! { pub type PWSTR = *mut u16; })
            }
            Type::PCSTR if writer.sys => {
                sorted.insert("PCSTR", quote! { pub type PCSTR = *const u8; })
            }
            Type::PCWSTR if writer.sys => {
                sorted.insert("PCWSTR", quote! { pub type PCWSTR = *const u16; })
            }
            Type::BSTR if writer.sys => {
                sorted.insert("BSTR", quote! { pub type BSTR = *const u16; })
            }
            Type::GUID if writer.sys => {
                sorted.insert("GUID", quote! {
                    #[repr(C)]
                    pub struct GUID {
                        pub data1: u32,
                        pub data2: u16,
                        pub data3: u16,
                        pub data4: [u8; 8],
                    }
                    impl ::core::marker::Copy for GUID {}
                    impl ::core::clone::Clone for GUID {
                        fn clone(&self) -> Self {
                            *self
                        }
                    }
                    impl GUID {
                        pub const fn from_u128(uuid: u128) -> Self {
                            Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
                        }
                    }
                });
            }
            Type::TypeDef(def, _) => {
                let kind = writer.reader.type_def_kind(def);
                match kind {
                    TypeKind::Class => {
                        let name = writer.reader.type_def_name(def);
                        if writer.sys {
                            let ident = to_ident(name);
                            sorted.insert(
                                name,
                                quote! { pub type #ident = *mut ::core::ffi::c_void; },
                            );
                        } else {
                            sorted.insert(name, classes::writer(writer, def));
                        }
                    }
                    TypeKind::Interface => {
                        let name = writer.reader.type_def_name(def);
                        if writer.sys {
                            let ident = to_ident(name);
                            sorted.insert(
                                name,
                                quote! { pub type #ident = *mut ::core::ffi::c_void; },
                            );
                        } else {
                            sorted.insert(name, interfaces::writer(writer, def));
                        }
                    }
                    TypeKind::Enum => {
                        sorted.insert(writer.reader.type_def_name(def), enums::writer(writer, def));
                    }
                    TypeKind::Struct => {
                        let name = writer.reader.type_def_name(def);
                        if writer.reader.type_def_fields(def).next().is_none() {
                            if let Some(guid) = writer.reader.type_def_guid(def) {
                                let ident = to_ident(name);
                                let value = writer.guid(&guid);
                                let guid = writer.type_name(&Type::GUID);
                                sorted.insert(
                                    name,
                                    quote! {
                                        pub const #ident: #guid = #value;
                                    },
                                );
                                continue;
                            }
                        }
                        sorted.insert(name, structs::writer(writer, def));
                    }
                    TypeKind::Delegate => {
                        sorted.insert(
                            writer.reader.type_def_name(def),
                            delegates::writer(writer, def),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    for (function, namespace) in functions {
        sorted.insert(
            &format!(
                ".{}.{}",
                writer.reader.method_def_module_name(function),
                writer.reader.method_def_name(function)
            ),
            functions::writer(writer, &namespace, function),
        );
    }

    for constant in constants {
        sorted.insert(
            writer.reader.field_name(constant),
            constants::writer(writer, constant),
        );
    }

    let mut tokens = TokenStream::new();
    sorted.0.values().for_each(|value| tokens.combine(value));
    tokens.into_string()
}

#[derive(Default)]
struct SortedTokens(BTreeMap<String, TokenStream>);

impl SortedTokens {
    fn insert(&mut self, key: &str, tokens: TokenStream) {
        self.0.entry(key.to_string()).or_default().combine(&tokens);
    }
}
