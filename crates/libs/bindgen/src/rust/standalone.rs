use super::*;
use metadata::AsRow;

pub fn standalone_imp(writer: &Writer) -> String {
    let mut types = std::collections::BTreeSet::new();
    let mut functions = std::collections::BTreeSet::new();
    let mut constants = std::collections::BTreeSet::new();

    for item in writer.reader.items() {
        item_collect_standalone(writer, item.clone(), &mut types);

        match item {
            metadata::Item::Type(_) => {}
            metadata::Item::Fn(def, namespace) => _ = functions.insert((def, namespace)),
            metadata::Item::Const(def) => _ = constants.insert(def),
        }
    }

    let mut sorted = SortedTokens::default();

    for ty in types {
        match ty {
            metadata::Type::HRESULT if writer.sys => sorted.insert("HRESULT", quote! { pub type HRESULT = i32; }),
            metadata::Type::IUnknown if writer.sys => sorted.insert(
                "IUnknown",
                if !writer.vtbl {
                    quote! {}
                } else {
                    quote! {
                        pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
                        #[repr(C)]
                        pub struct IUnknown_Vtbl {
                            pub QueryInterface: unsafe extern "system" fn(this: *mut core::ffi::c_void, iid: *const GUID, interface: *mut *mut core::ffi::c_void) -> HRESULT,
                            pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
                            pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
                        }
                    }
                },
            ),
            metadata::Type::IInspectable if writer.sys => sorted.insert(
                "IInspectable",
                if !writer.vtbl {
                    quote! {}
                } else {
                    quote! {
                        pub const IID_IInspectable: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
                        #[repr(C)]
                        pub struct IInspectable_Vtbl {
                            pub base: IUnknown_Vtbl,
                            pub GetIids: unsafe extern "system" fn(this: *mut std::ffi::c_void, count: *mut u32, values: *mut *mut GUID) -> HRESULT,
                            pub GetRuntimeClassName: unsafe extern "system" fn(this: *mut std::ffi::c_void, value: *mut *mut std::ffi::c_void) -> HRESULT,
                            pub GetTrustLevel: unsafe extern "system" fn(this: *mut std::ffi::c_void, value: *mut i32) -> HRESULT,
                        }
                    }
                },
            ),
            metadata::Type::PSTR if writer.sys => sorted.insert("PSTR", quote! { pub type PSTR = *mut u8; }),
            metadata::Type::PWSTR if writer.sys => sorted.insert("PWSTR", quote! { pub type PWSTR = *mut u16; }),
            metadata::Type::PCSTR if writer.sys => sorted.insert("PCSTR", quote! { pub type PCSTR = *const u8; }),
            metadata::Type::PCWSTR if writer.sys => sorted.insert("PCWSTR", quote! { pub type PCWSTR = *const u16; }),
            metadata::Type::BSTR if writer.sys => sorted.insert("BSTR", quote! { pub type BSTR = *const u16; }),
            metadata::Type::GUID if writer.sys => {
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
                        impl Copy for GUID {}
                        impl Clone for GUID {
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
            metadata::Type::TypeDef(def, _) => {
                sorted.insert(def.name(), writer.type_def(def));
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
struct SortedTokens(std::collections::BTreeMap<String, TokenStream>);

impl SortedTokens {
    fn insert(&mut self, key: &str, tokens: TokenStream) {
        self.0.entry(key.to_string()).or_default().combine(&tokens);
    }
}

fn item_collect_standalone(writer: &Writer, item: metadata::Item, set: &mut std::collections::BTreeSet<metadata::Type>) {
    match item {
        metadata::Item::Type(def) => type_collect_standalone(writer, &metadata::Type::TypeDef(def, vec![]), set),
        metadata::Item::Const(def) => type_collect_standalone(writer, &def.ty(None).to_const_type(), set),
        metadata::Item::Fn(def, namespace) => {
            let signature = metadata::method_def_signature(namespace, def, &[]);
            type_collect_standalone(writer, &signature.return_type, set);
            signature.params.iter().for_each(|param| type_collect_standalone(writer, &param.ty, set));
        }
    }
}
// TODO: remove or move to riddle
fn type_collect_standalone(writer: &Writer, ty: &metadata::Type, set: &mut std::collections::BTreeSet<metadata::Type>) {
    let ty = ty.to_underlying_type();
    if !set.insert(ty.clone()) {
        return;
    }

    if writer.vtbl {
        match ty {
            metadata::Type::IUnknown => {
                set.insert(metadata::Type::GUID);
                set.insert(metadata::Type::HRESULT);
            }
            metadata::Type::IInspectable => type_collect_standalone(writer, &metadata::Type::IUnknown, set),
            _ => {}
        }
    }

    let metadata::Type::TypeDef(def, generics) = ty.to_underlying_type() else {
        return;
    };

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
                type_collect_standalone(writer, &metadata::Type::TypeDef(row, Vec::new()), set);
            }
        }
    }

    for generic in &generics {
        type_collect_standalone(writer, generic, set);
    }

    for field in def.fields() {
        let ty = field.ty(Some(def));
        if let metadata::Type::TypeDef(def, _) = &ty {
            if def.namespace().is_empty() {
                continue;
            }
        }
        type_collect_standalone(writer, &ty, set);
    }

    for method in def.methods() {
        // Skip delegate pseudo-constructors.
        if method.name() == ".ctor" {
            continue;
        }
        let signature = metadata::method_def_signature(def.namespace(), method, &generics);
        type_collect_standalone(writer, &signature.return_type, set);
        signature.params.iter().for_each(|param| type_collect_standalone(writer, &param.ty, set));
    }

    for interface in metadata::type_interfaces(&ty) {
        type_collect_standalone(writer, &interface.ty, set);
    }

    match def.kind() {
        metadata::TypeKind::Struct => {
            if def.fields().next().is_none() && metadata::type_def_guid(def).is_some() {
                set.insert(metadata::Type::GUID);
            }
        }
        metadata::TypeKind::Interface => {
            if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
                type_collect_standalone(writer, &metadata::Type::IInspectable, set);
            }
        }
        _ => {}
    }

    type_collect_standalone_nested(writer, def, set);
}

fn type_collect_standalone_nested(writer: &Writer, td: metadata::TypeDef, set: &mut std::collections::BTreeSet<metadata::Type>) {
    for nested in td.reader().nested_types(td) {
        type_collect_standalone_nested(writer, nested, set);

        for field in nested.fields() {
            let ty = field.ty(Some(nested));
            if let metadata::Type::TypeDef(def, _) = ty.to_underlying_type() {
                // Skip the fields that actually refer to the anonymous nested
                // type, otherwise it will get added to the typeset and emitted
                if def.namespace().is_empty() {
                    continue;
                }
            }
            type_collect_standalone(writer, &ty, set);
        }
    }
}
