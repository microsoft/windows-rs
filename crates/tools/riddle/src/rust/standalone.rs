use super::*;

// /// Generates standalone Windows bindings based on the `windows` crate's bindings.
// pub fn standalone_win(reader: &Reader, names: &[&str]) -> String {
//     let gen = &mut Gen::new(reader);
//     standalone_imp(gen, names)
// }

// /// Generates standalone Windows bindings based on the `windows-sys` crate's bindings.
// pub fn standalone_sys(reader: &Reader, names: &[&str]) -> String {
//     let gen = &mut Gen::new(reader);
//     gen.sys = true;
//     standalone_imp(gen, names)
// }

// /// Generates standalone Windows bindings for the Rust Standard Library.
// #[doc(hidden)]
// pub fn standalone_std(reader: &Reader, names: &[&str]) -> String {
//     let gen = &mut Gen::new(reader);
//     gen.std = true;
//     gen.sys = true;
//     standalone_imp(gen, names)
// }

pub fn standalone_imp<'a, I: Iterator<Item = &'a str>>(gen: &Gen, names: I) -> String {
    let mut types = BTreeSet::new();
    let mut functions = BTreeSet::new();
    let mut constants = BTreeSet::new();

    for name in names {
        let mut found = false;
        let type_name = TypeName::parse(name);

        for def in gen.reader.get(type_name) {
            found = true;
            gen.reader
                .type_collect_standalone(&Type::TypeDef(def, vec![]), &mut types);
        }

        for method in gen
            .reader
            .namespace_functions(type_name.namespace)
            .filter(|method| gen.reader.method_def_name(*method) == type_name.name)
        {
            found = true;
            functions.insert(method);
            let signature = gen.reader.method_def_signature(method, &[]);
            gen.reader
                .type_collect_standalone(&signature.return_type, &mut types);
            signature
                .params
                .iter()
                .for_each(|param| gen.reader.type_collect_standalone(&param.ty, &mut types));
        }

        if let Some(field) = gen
            .reader
            .namespace_constants(type_name.namespace)
            .find(|field| gen.reader.field_name(*field) == type_name.name)
        {
            found = true;
            constants.insert(field);
            gen.reader.type_collect_standalone(
                &gen.reader.field_type(field, None).to_const_type(),
                &mut types,
            );
        }

        if let Some(field) = gen
            .reader
            .namespace_types(type_name.namespace, &Default::default())
            .find_map(|def| {
                if gen.reader.type_def_kind(def) == TypeKind::Enum {
                    return gen
                        .reader
                        .type_def_fields(def)
                        .find(|field| gen.reader.field_name(*field) == type_name.name);
                }
                None
            })
        {
            found = true;
            constants.insert(field);
            gen.reader
                .type_collect_standalone(&gen.reader.field_type(field, None), &mut types);
        }

        assert!(found, "{} not found", type_name);
    }

    let mut sorted = SortedTokens::default();

    for ty in types {
        match ty {
            Type::HRESULT if gen.sys => {
                sorted.insert("HRESULT", quote! { pub type HRESULT = i32; })
            }
            Type::String if gen.sys => sorted.insert(
                "HSTRING",
                quote! { pub type HSTRING = *mut ::core::ffi::c_void; },
            ),
            Type::IUnknown if gen.sys => sorted.insert(
                "IUnknown",
                quote! { pub type IUnknown = *mut ::core::ffi::c_void; },
            ),
            Type::IInspectable if gen.sys => sorted.insert(
                "IInspectable",
                quote! { pub type IInspectable = *mut ::core::ffi::c_void; },
            ),
            Type::PSTR if gen.sys => sorted.insert("PSTR", quote! { pub type PSTR = *mut u8; }),
            Type::PWSTR if gen.sys => sorted.insert("PWSTR", quote! { pub type PWSTR = *mut u16; }),
            Type::PCSTR if gen.sys => {
                sorted.insert("PCSTR", quote! { pub type PCSTR = *const u8; })
            }
            Type::PCWSTR if gen.sys => {
                sorted.insert("PCWSTR", quote! { pub type PCWSTR = *const u16; })
            }
            Type::BSTR if gen.sys => sorted.insert("BSTR", quote! { pub type BSTR = *const u16; }),
            Type::GUID if gen.sys => {
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
                let kind = gen.reader.type_def_kind(def);
                match kind {
                    TypeKind::Class => {
                        let name = gen.reader.type_def_name(def);
                        if gen.sys {
                            let ident = to_ident(name);
                            sorted.insert(
                                name,
                                quote! { pub type #ident = *mut ::core::ffi::c_void; },
                            );
                        } else {
                            sorted.insert(name, classes::gen(gen, def));
                        }
                    }
                    TypeKind::Interface => {
                        let name = gen.reader.type_def_name(def);
                        if gen.sys {
                            let ident = to_ident(name);
                            sorted.insert(
                                name,
                                quote! { pub type #ident = *mut ::core::ffi::c_void; },
                            );
                        } else {
                            sorted.insert(name, interfaces::gen(gen, def));
                        }
                    }
                    TypeKind::Enum => {
                        sorted.insert(gen.reader.type_def_name(def), enums::gen(gen, def));
                    }
                    TypeKind::Struct => {
                        let name = gen.reader.type_def_name(def);
                        if gen.reader.type_def_fields(def).next().is_none() {
                            if let Some(guid) = gen.reader.type_def_guid(def) {
                                let ident = to_ident(name);
                                let value = gen.guid(&guid);
                                let guid = gen.type_name(&Type::GUID);
                                sorted.insert(
                                    name,
                                    quote! {
                                        pub const #ident: #guid = #value;
                                    },
                                );
                                continue;
                            }
                        }
                        sorted.insert(name, structs::gen(gen, def));
                    }
                    TypeKind::Delegate => {
                        sorted.insert(gen.reader.type_def_name(def), delegates::gen(gen, def));
                    }
                }
            }
            _ => {}
        }
    }

    for function in functions {
        sorted.insert(
            &format!(
                ".{}.{}",
                gen.reader.method_def_module_name(function),
                gen.reader.method_def_name(function)
            ),
            functions::gen(gen, function),
        );
    }

    for constant in constants {
        sorted.insert(
            gen.reader.field_name(constant),
            constants::gen(gen, constant),
        );
    }

    // TODO: move up a level so it applies to all code gen (once per file)
    let mut tokens: TokenStream = format!(
        r#"// Bindings generated by `riddle` {}

"#,
        std::env!("CARGO_PKG_VERSION")
    )
    .into();

    tokens.combine(&allow());
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
