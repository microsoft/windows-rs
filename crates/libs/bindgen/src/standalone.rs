use super::*;

/// Generates standalone Windows bindings based on the `windows` crate's bindings.
pub fn standalone_win(names: &[&str]) -> String {
    let files = &File::with_default(&[]).unwrap();
    let reader = &Reader::new(files);
    let gen = &mut Gen::new(reader);

    let items = gather_items(gen, names.iter().map(|s| (*s, None)));
    standalone_imp(gen, items)
}

/// Generates standalone Windows bindings based on the `windows-sys` crate's bindings.
pub fn standalone_sys(names: &[&str]) -> String {
    let files = &File::with_default(&[]).unwrap();
    let reader = &Reader::new(files);
    let gen = &mut Gen::new(reader);
    gen.sys = true;

    let items = gather_items(gen, names.iter().map(|s| (*s, None)));
    standalone_imp(gen, items)
}

/// Generates standalone Windows bindings for the Rust Standard Library.
#[doc(hidden)]
pub fn standalone_std(names: &[&str]) -> String {
    let files = &File::with_default(&[]).unwrap();
    let reader = &Reader::new(files);
    let gen = &mut Gen::new(reader);
    gen.std = true;
    gen.sys = true;

    let items = gather_items(gen, names.iter().map(|s| (*s, None)));
    standalone_imp(gen, items)
}

/// All of the unique items gathered based on a root list of names
pub struct ItemSet {
    /// Set of structs, unions, interfaces, classes, enums, type aliases, and function pointers
    pub types: BTreeSet<Type>,
    /// Set of functions
    pub functions: BTreeSet<MethodDef>,
    /// Set of constants, including individual enum values
    pub constants: BTreeSet<Field>,
}

/// Disambiguates items during gathering.
///
/// In [some](https://docs.rs/windows-sys/0.48.0/windows_sys/Win32/UI/Input/KeyboardAndMouse/struct.VK_F.html)
/// [cases](https://docs.rs/windows-sys/0.48.0/windows_sys/Win32/UI/Input/KeyboardAndMouse/constant.VK_F.html)
/// items with the exact same name can be located within the same namespace,
/// which can cause the incorrect item to be emitted and the desired item to be
/// skipped.
#[derive(Copy, Clone, Debug)]
pub enum Disambiguate {
    /// The item is a constant or enum variant
    Constant,
    /// The item is a function
    Function,
    /// The item is a struct or union
    Record,
}

/// Takes a list of fully qualified type names and recursively gathers all of
/// the items needed to fully define them.
pub fn gather_items<'names>(
    gen: &mut Gen,
    names: impl Iterator<Item = (&'names str, Option<Disambiguate>)>,
) -> ItemSet {
    let mut types = BTreeSet::new();
    let mut functions = BTreeSet::new();
    let mut constants = BTreeSet::new();

    for (name, dis) in names {
        let type_name = TypeName::parse(name);

        // We can't simply use `if let` here to find the types and functions as there may be multiple definitions
        // to cover multi-arch support.
        let mut found = false;

        if matches!(dis, None | Some(Disambiguate::Record)) {
            for def in gen.reader.get(type_name) {
                gen.reader
                    .type_collect_standalone(&Type::TypeDef((def, vec![])), &mut types);
                found = true;
            }
        }

        if found {
            continue;
        }

        if matches!(dis, None | Some(Disambiguate::Function)) {
            for method in gen
                .reader
                .namespace_functions(type_name.namespace)
                .filter(|method| gen.reader.method_def_name(*method) == type_name.name)
            {
                functions.insert(method);
                let signature = gen.reader.method_def_signature(method, &[]);
                signature
                    .return_type
                    .iter()
                    .for_each(|ty| gen.reader.type_collect_standalone(ty, &mut types));
                signature
                    .params
                    .iter()
                    .for_each(|param| gen.reader.type_collect_standalone(&param.ty, &mut types));
                found = true;
            }
        }

        if found || !matches!(dis, None | Some(Disambiguate::Constant)) {
            continue;
        }

        if let Some(field) = gen
            .reader
            .namespace_constants(type_name.namespace)
            .find(|field| gen.reader.field_name(*field) == type_name.name)
        {
            constants.insert(field);
            gen.reader
                .type_collect_standalone(&gen.reader.field_type(field, None), &mut types);
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
            constants.insert(field);
            gen.reader
                .type_collect_standalone(&gen.reader.field_type(field, None), &mut types);
        }
    }

    ItemSet {
        types,
        functions,
        constants,
    }
}

/// Generates Rust code for every item in the provided set.
///
/// This is the actual implementation of the following helper functions
///
/// * [`standalone_sys`]
/// * [`standalone_win`]
pub fn standalone_imp(gen: &mut Gen, items: ItemSet) -> String {
    gen.namespace = "Windows.";
    gen.standalone = true;

    let mut sorted = SortedTokens::default();

    for ty in items.types {
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
            Type::TypeDef((def, _)) => {
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
                        if gen.reader.type_def_fields(def).next().is_none() {
                            if let Some(guid) = gen.reader.type_def_guid(def) {
                                let name = gen.reader.type_def_name(def);
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
                        sorted.insert(gen.reader.type_def_name(def), structs::gen(gen, def));
                    }
                    TypeKind::Delegate => {
                        sorted.insert(gen.reader.type_def_name(def), delegates::gen(gen, def));
                    }
                }
            }
            _ => {}
        }
    }

    for function in items.functions {
        sorted.insert(
            &format!(
                ".{}.{}",
                gen.reader.method_def_module_name(function),
                gen.reader.method_def_name(function)
            ),
            functions::gen(gen, function),
        );
    }

    for constant in items.constants {
        sorted.insert(
            &format!("{}", gen.reader.field_name(constant)),
            constants::gen(gen, constant),
        );
    }

    let mut tokens: TokenStream = format!(
        r#"// Bindings generated by `windows-bindgen` {}

"#,
        std::env!("CARGO_PKG_VERSION")
    )
    .into();

    tokens.combine(&allow());
    sorted.0.values().for_each(|value| tokens.combine(value));
    try_format(tokens.into_string())
}

#[derive(Default)]
struct SortedTokens(BTreeMap<String, TokenStream>);

impl SortedTokens {
    fn insert(&mut self, key: &str, tokens: TokenStream) {
        self.0.entry(key.to_string()).or_default().combine(&tokens);
    }
}
