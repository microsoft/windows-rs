use super::*;

pub fn standalone_imp(writer: &Writer) -> String {
    let mut types = BTreeSet::new();
    let mut functions = BTreeSet::new();
    let mut constants = BTreeSet::new();

    for item in writer.reader.items() {
        item_collect_standalone(item.clone(), &mut types);

        match item {
            Item::Type(_) => {}
            Item::Fn(def, namespace) => _ = functions.insert((def, namespace)),
            Item::Const(def) => _ = constants.insert(def),
        }
    }

    let mut sorted = SortedTokens::default();

    for ty in types {
        match ty {
            Type::HRESULT if writer.sys => sorted.insert("HRESULT", quote! { pub type HRESULT = i32; }),
            Type::String if writer.sys => sorted.insert("HSTRING", quote! { pub type HSTRING = *mut ::core::ffi::c_void; }),
            Type::IUnknown if writer.sys => sorted.insert("IUnknown", quote! { pub type IUnknown = *mut ::core::ffi::c_void; }),
            Type::IInspectable if writer.sys => sorted.insert("IInspectable", quote! { pub type IInspectable = *mut ::core::ffi::c_void; }),
            Type::PSTR if writer.sys => sorted.insert("PSTR", quote! { pub type PSTR = *mut u8; }),
            Type::PWSTR if writer.sys => sorted.insert("PWSTR", quote! { pub type PWSTR = *mut u16; }),
            Type::PCSTR if writer.sys => sorted.insert("PCSTR", quote! { pub type PCSTR = *const u8; }),
            Type::PCWSTR if writer.sys => sorted.insert("PCWSTR", quote! { pub type PCWSTR = *const u16; }),
            Type::BSTR if writer.sys => sorted.insert("BSTR", quote! { pub type BSTR = *const u16; }),
            Type::GUID if writer.sys => {
                sorted.insert(
                    "GUID",
                    quote! {
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
                    },
                );
            }
            Type::TypeDef(def, _) => {
                let kind = def.kind();
                match kind {
                    TypeKind::Class => {
                        let name = def.name();
                        if writer.sys {
                            let ident = to_ident(name);
                            sorted.insert(name, quote! { pub type #ident = *mut ::core::ffi::c_void; });
                        } else {
                            sorted.insert(name, classes::writer(writer, def));
                        }
                    }
                    TypeKind::Interface => {
                        let name = def.name();
                        if writer.sys {
                            let ident = to_ident(name);
                            sorted.insert(name, quote! { pub type #ident = *mut ::core::ffi::c_void; });
                        } else {
                            sorted.insert(name, interfaces::writer(writer, def));
                        }
                    }
                    TypeKind::Enum => {
                        sorted.insert(def.name(), enums::writer(writer, def));
                    }
                    TypeKind::Struct => {
                        let name = def.name();
                        if def.fields().next().is_none() {
                            if let Some(guid) = type_def_guid(def) {
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
                        sorted.insert(def.name(), delegates::writer(writer, def));
                    }
                }
            }
            _ => {}
        }
    }

    for (function, namespace) in functions {
        sorted.insert(&format!(".{}.{}", function.module_name(), function.name()), functions::writer(writer, namespace, function));
    }

    for constant in constants {
        sorted.insert(constant.name(), constants::writer(writer, constant));
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

fn item_collect_standalone(item: Item, set: &mut BTreeSet<Type>) {
    match item {
        Item::Type(def) => type_collect_standalone(&Type::TypeDef(def, vec![]), set),
        Item::Const(def) => type_collect_standalone(&def.ty(None).to_const_type(), set),
        Item::Fn(def, namespace) => {
            let signature = method_def_signature(namespace, def, &[]);
            type_collect_standalone(&signature.return_type, set);
            signature.params.iter().for_each(|param| type_collect_standalone(&param.ty, set));
        }
    }
}
// TODO: remove or move to riddle
fn type_collect_standalone(ty: &Type, set: &mut BTreeSet<Type>) {
    let ty = ty.to_underlying_type();
    if !set.insert(ty.clone()) {
        return;
    }

    let Type::TypeDef(def, generics) = &ty else {
        return;
    };

    let def = *def;

    // Ensure that we collect all the typedefs of the same name. We need to
    // do this in the case where the user specifies a top level item that
    // references a typedef by name, but that name resolves to more than 1
    // Type based on target architecture (typically)
    //
    // Note this is a bit overeager as we can collect a typedef that is used
    // by one architecture but not by another
    let type_name = def.type_name();
    if !type_name.namespace.is_empty() {
        for row in def.reader().get_type_def(type_name.namespace, type_name.name) {
            if def != row {
                type_collect_standalone(&Type::TypeDef(row, Vec::new()), set);
            }
        }
    }

    for generic in generics {
        type_collect_standalone(generic, set);
    }
    for field in def.fields() {
        let ty = field.ty(Some(def));
        if let Type::TypeDef(def, _) = &ty {
            if def.namespace().is_empty() {
                continue;
            }
        }
        type_collect_standalone(&ty, set);
    }
    for method in def.methods() {
        // Skip delegate pseudo-constructors.
        if method.name() == ".ctor" {
            continue;
        }
        let signature = method_def_signature(def.namespace(), method, generics);
        type_collect_standalone(&signature.return_type, set);
        signature.params.iter().for_each(|param| type_collect_standalone(&param.ty, set));
    }
    for interface in type_interfaces(&ty) {
        type_collect_standalone(&interface.ty, set);
    }
    if def.kind() == TypeKind::Struct && def.fields().next().is_none() && type_def_guid(def).is_some() {
        set.insert(Type::GUID);
    }

    type_collect_standalone_nested(def, set);
}

fn type_collect_standalone_nested(td: TypeDef, set: &mut BTreeSet<Type>) {
    for nested in td.reader().nested_types(td) {
        type_collect_standalone_nested(nested, set);

        for field in nested.fields() {
            let ty = field.ty(Some(nested));
            if let Type::TypeDef(def, _) = &ty {
                // Skip the fields that actually refer to the anonymous nested
                // type, otherwise it will get added to the typeset and emitted
                if def.namespace().is_empty() {
                    continue;
                }

                type_collect_standalone(&ty, set);
            }
        }
    }
}
