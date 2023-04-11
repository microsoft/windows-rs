use super::*;

/// Generates standalone Windows bindings.
pub fn standalone(names: &[&str]) -> String {
    let files = &File::with_default(&[]).unwrap();
    let reader = &Reader::new(files);
    let mut gen = &mut Gen::new(reader);
    gen.standalone = true;
    gen.sys = true;
    standalone_imp(gen, names)
}

/// Generates standalone Windows bindings for the Rust Standard Library.
pub fn standalone_std(names: &[&str]) -> String {
    let files = &File::with_default(&[]).unwrap();
    let reader = &Reader::new(files);
    let mut gen = &mut Gen::new(reader);
    gen.standalone = true;
    gen.sys = true;
    gen.std = true;
    standalone_imp(gen, names)
}

fn standalone_imp(gen: &Gen, names: &[&str]) -> String {
    let mut type_names = BTreeSet::new();
    let mut core_types = BTreeSet::new();
    let mut enums = BTreeSet::new();

    for name in names {
        let type_name = TypeName::parse(name);
        let mut found = false;

        if let Some(def) = gen.reader.get(type_name).next() {
            found = true;
            type_names.insert(type_name);
            let mut cfg = gen.reader.type_def_cfg(def, &[]);
            core_types.append(&mut cfg.core_types);
            for def in cfg.types.values().flatten() {
                type_names.insert(gen.reader.type_def_type_name(*def));
            }
            if gen.reader.type_def_kind(def) == TypeKind::Struct
                && gen.reader.type_def_fields(def).next().is_none()
                && gen.reader.type_def_guid(def).is_some()
            {
                core_types.insert(Type::GUID);
            }
        }

        if !found {
            if let Some(def) = gen
                .reader
                .get(TypeName::new(type_name.namespace, "Apis"))
                .next()
            {
                for method in gen.reader.type_def_methods(def) {
                    if found {
                        break;
                    }
                    let name = gen.reader.method_def_name(method);
                    if name == type_name.name {
                        found = true;
                        type_names.insert(type_name);
                        let mut cfg = gen
                            .reader
                            .signature_cfg(&gen.reader.method_def_signature(method, &[]));
                        core_types.append(&mut cfg.core_types);
                        for def in cfg.types.values().flatten() {
                            type_names.insert(gen.reader.type_def_type_name(*def));
                        }
                    }
                }
                for field in gen.reader.type_def_fields(def) {
                    if found {
                        break;
                    }
                    let name = gen.reader.field_name(field);
                    if name == type_name.name {
                        found = true;
                        type_names.insert(type_name);
                        let mut cfg = gen.reader.field_cfg(field);
                        core_types.append(&mut cfg.core_types);
                        for def in cfg.types.values().flatten() {
                            type_names.insert(gen.reader.type_def_type_name(*def));
                        }
                    }
                }
            }
        }

        if !found {
            for def in gen
                .reader
                .namespace_types(type_name.namespace, &Default::default())
            {
                if found {
                    break;
                }
                if gen.reader.type_def_kind(def) == TypeKind::Enum {
                    for field in gen.reader.type_def_fields(def) {
                        if found {
                            break;
                        }
                        let name = gen.reader.field_name(field);
                        if name == type_name.name {
                            found = true;
                            let enum_name = gen.reader.type_def_type_name(def);
                            type_names.insert(enum_name);
                            enums.insert((enum_name, type_name.name));
                        }
                    }
                }
            }
        }
    }

    let mut sorted = SortedTokens::default();

    for ty in core_types {
        match ty {
            Type::HRESULT => { sorted.insert("HRESULT", quote! { pub type HRESULT = i32; }); }
            Type::String => {
                sorted.insert("HSTRING", quote! { pub type HSTRING = *mut ::core::ffi::c_void; });
            }
            Type::IUnknown => {
                sorted.insert("IUnknown", quote! { pub type IUnknown = *mut ::core::ffi::c_void; });
            }
            Type::IInspectable => {
                sorted.insert("IInspectable", quote! { pub type IInspectable = *mut ::core::ffi::c_void; });
            }
            Type::PSTR => { sorted.insert("PSTR", quote! { pub type PSTR = *mut u8; }); }
            Type::PWSTR => { sorted.insert("PWSTR", quote! { pub type PWSTR = *mut u16; }); }
            Type::PCSTR => { sorted.insert("PCSTR", quote! { pub type PCSTR = *const u8; }); }
            Type::PCWSTR => { sorted.insert("PCWSTR", quote! { pub type PCWSTR = *const u16; }); }
            Type::BSTR => { sorted.insert("BSTR", quote! { pub type BSTR = *const u16; }); }
            Type::GUID => {sorted.insert("GUID", quote! {
                #[repr(C)]
                pub struct GUID {
                    pub data1: u32,
                    pub data2: u16,
                    pub data3: u16,
                    pub data4: [u8; 8],
                }
                impl GUID {
                    pub const fn from_u128(uuid: u128) -> Self {
                        Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
                    }
                }
                impl ::core::marker::Copy for GUID {}
                impl ::core::clone::Clone for GUID {
                    fn clone(&self) -> Self {
                        *self
                    }
                }
            }); }
            _ => {}
        }
    }

    for type_name in type_names {
        let mut found = false;

        for def in gen.reader.get(type_name) {
            found = true;
            let kind = gen.reader.type_def_kind(def);

            match kind {
                TypeKind::Class | TypeKind::Interface => unimplemented!(),
                TypeKind::Enum => { sorted.insert(gen.reader.type_def_name(def), enums::gen(gen, def)); }
                TypeKind::Struct => {
                    if gen.reader.type_def_fields(def).next().is_none() {
                        if let Some(guid) = gen.reader.type_def_guid(def) {
                            let ident = to_ident(type_name.name);
                            let value = gen.guid(&guid);
                            let guid = gen.type_name(&Type::GUID);
                            sorted.insert(type_name.name, quote! {
                                pub const #ident: #guid = #value;
                            });
                            continue;
                        }
                    }
                    sorted.insert(gen.reader.type_def_name(def), structs::gen(gen, def));
                }
                TypeKind::Delegate => { sorted.insert(gen.reader.type_def_name(def),delegates::gen(gen, def)); }
            }
        }

        if !found {
            if let Some(def) = gen
                .reader
                .get(TypeName::new(type_name.namespace, "Apis"))
                .next()
            {
                for method in gen.reader.type_def_methods(def) {
                    if found {
                        break;
                    }
                    let name = gen.reader.method_def_name(method);
                    if name == type_name.name {
                        found = true;
                        sorted.insert(&format!(".{}.{name}", gen.reader.method_def_module_name(method)), functions::gen(gen, method));
                    }
                }
                for field in gen.reader.type_def_fields(def) {
                    if found {
                        break;
                    }
                    let name = gen.reader.field_name(field);
                    if name == type_name.name {
                        found = true;
                        sorted.insert(name,constants::gen(gen, field));
                    }
                }
            }
        }
    }

    for (enum_type, field_name) in enums {
        if let Some(def) = gen.reader.get(enum_type).next() {
            for field in gen.reader.type_def_fields(def) {
                if gen.reader.field_name(field) == field_name {
                    let ident = to_ident(field_name);
                    let ty = to_ident(enum_type.name);
                    let constant = gen.reader.field_constant(field).unwrap();
                    let value = gen.value(&gen.reader.constant_value(constant));

                    sorted.insert(field_name,quote! {
                        pub const #ident: #ty = #value;
                    });

                    break;
                }
            }
        }
    }

    let mut tokens: TokenStream = format!(
        r#"// Bindings generated by `windows-bindgen` {}

"#,
        std::env!("CARGO_PKG_VERSION")
    )
    .into();

    tokens.combine(&allow());

    for value in sorted.0.values() {
        tokens.combine(value);
    }

    try_format(tokens.into_string())
}

fn try_format(tokens: String) -> String {
    use std::io::Write;

    let Ok(mut child) = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn() else {
        return tokens;
    };

    let Some(mut stdin) = child.stdin.take() else {
        return tokens;
    };

    if stdin.write_all(tokens.as_bytes()).is_err() {
        return tokens;
    }

    drop(stdin);

    let Ok(output) = child.wait_with_output() else {
        return tokens;
    };

    if !output.status.success() {
        return tokens;
    }

    if let Ok(result) = String::from_utf8(output.stdout) {
        result
    } else {
        tokens
    }
}

#[derive(Default)]
struct SortedTokens(BTreeMap<String, TokenStream>);

impl SortedTokens {
    fn insert(&mut self, key: &str, tokens: TokenStream) {
        self.0. entry(key.to_string()).or_default().combine(&tokens);
    }
}