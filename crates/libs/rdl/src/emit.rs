//! RDL source-emission primitives shared by the winmd -> RDL writer and the
//! headers -> RDL scraper (`windows-clang`). Both turn metadata shapes into the
//! same RDL spelling, so the identifier, type, value, and GUID rendering lives
//! here in one place.

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use windows_metadata as metadata;

/// Renders a name as an RDL identifier, escaping Rust keywords (`r#...`) and the
/// special cases (`Self`/`self` -> `<name>_`, `_` -> `unused`).
pub fn write_ident(name: &str) -> TokenStream {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    let name = match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do"
        | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in"
        | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv"
        | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield"
        | "try" | "async" | "await" | "dyn" => format_ident!("r#{name}"),
        "Self" | "self" => format_ident!("{name}_"),
        "_" => format_ident!("unused"),
        _ => format_ident!("{}", windows_metadata::trim_tick(name)),
    };

    quote! { #name }
}

/// Renders a metadata [`Type`](metadata::Type) as its RDL type spelling, relative to
/// `namespace` (nearer namespaces are shortened with `super::`).
pub fn write_type(namespace: &str, item: &metadata::Type) -> TokenStream {
    use metadata::Type::*;
    match item {
        Bool => quote! { bool },
        Char => quote! { Char16 },
        I8 => quote! { i8 },
        U8 => quote! { u8 },
        I16 => quote! { i16 },
        U16 => quote! { u16 },
        I32 => quote! { i32 },
        U32 => quote! { u32 },
        I64 => quote! { i64 },
        U64 => quote! { u64 },
        F32 => quote! { f32 },
        F64 => quote! { f64 },
        ISize => quote! { isize },
        USize => quote! { usize },

        Void => quote! { void },
        String => quote! { String },
        Object => quote! { Object },
        ClassName(tn) if tn == ("System", "Type") => quote! { Type },
        ValueName(tn) if tn == ("System", "Guid") => quote! { GUID },
        ValueName(tn) if tn == ("Windows.Foundation", "HResult") => quote! { HRESULT },

        Array(ty) => {
            let ty = write_type(namespace, ty);
            quote! { [#ty] }
        }
        ArrayFixed(ty, len) => {
            let ty = write_type(namespace, ty);
            let len = Literal::usize_unsuffixed(*len);
            quote! { [#ty; #len] }
        }
        RefMut(ty) => {
            let ty = write_type(namespace, ty);
            quote! { &mut #ty }
        }
        RefConst(ty) => {
            let ty = write_type(namespace, ty);
            quote! { & #ty }
        }
        PtrMut(ty, pointers) => {
            let mut ty = write_type(namespace, ty);

            for _ in 0..*pointers {
                ty = quote! { *mut #ty };
            }

            ty
        }
        PtrConst(ty, pointers) => {
            let mut ty = write_type(namespace, ty);

            for _ in 0..*pointers {
                ty = quote! { *const #ty };
            }

            ty
        }
        ClassName(type_name) | ValueName(type_name) => {
            let name = write_ident(&type_name.name);

            let name = if type_name.generics.is_empty() {
                name
            } else {
                let generics = type_name
                    .generics
                    .iter()
                    .map(|ty| write_type(namespace, ty));
                quote! { #name <#(#generics),*> }
            };

            // The empty namespace test is for nested types.
            if namespace == type_name.namespace || type_name.namespace.is_empty() {
                name
            } else {
                let mut relative = namespace.split('.').peekable();
                let mut namespace = type_name.namespace.split('.').peekable();
                let shares_root = relative.peek() == namespace.peek();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }

                    namespace.next();
                }

                let mut tokens = TokenStream::new();

                if shares_root {
                    for _ in 0..relative.count() {
                        tokens = quote! { #tokens super:: };
                    }
                }

                for namespace in namespace {
                    let namespace = write_ident(namespace);
                    tokens = quote! { #tokens #namespace ::};
                }

                quote! { #tokens #name }
            }
        }
        Generic(name, _) => {
            let name = write_ident(name);
            quote! { #name }
        }
    }
}

/// Renders a metadata [`Value`](metadata::Value) as its RDL literal spelling.
pub fn write_value(namespace: &str, value: &metadata::Value) -> TokenStream {
    match value {
        metadata::Value::Bool(value) => quote! { #value },
        metadata::Value::U8(value) => {
            let literal = Literal::u8_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I8(value) => {
            let literal = Literal::i8_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U16(value) => {
            let literal = Literal::u16_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I16(value) => {
            let literal = Literal::i16_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U32(value) => {
            let literal = Literal::u32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I32(value) => {
            let literal = Literal::i32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::U64(value) => {
            let literal = Literal::u64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::I64(value) => {
            let literal = Literal::i64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::USize(value) => {
            let literal = Literal::u64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::ISize(value) => {
            let literal = Literal::i64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::F32(value) => {
            let literal = Literal::f32_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::F64(value) => {
            let literal = Literal::f64_unsuffixed(*value);
            quote! { #literal }
        }
        metadata::Value::Utf8(value) => quote! { #value },
        metadata::Value::Utf16(value) => quote! { #value },
        metadata::Value::TypeName(tn) => {
            write_type(namespace, &metadata::Type::ClassName(tn.clone()))
        }
        metadata::Value::EnumValue(_, inner) => write_value(namespace, inner),
    }
}

/// Formats GUID components as a UUID-style hex u128 literal, e.g.
/// `0x005023ca_72b1_11d3_9fc4_00c04f79a0a3`.
pub(crate) fn format_guid_u128(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> String {
    let d4_word = u16::from_be_bytes([d4[0], d4[1]]);
    let d4_node = u64::from_be_bytes([0, 0, d4[2], d4[3], d4[4], d4[5], d4[6], d4[7]]);
    format!("0x{d1:08x}_{d2:04x}_{d3:04x}_{d4_word:04x}_{d4_node:012x}")
}

/// Converts a UUID string (`xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx`) into the u128 hex literal
/// format used in RDL GUID attributes, e.g. `"0x00000000_0000_0000_c000_000000000046"`.
///
/// # Panics
///
/// Panics if `uuid` does not have exactly five hyphen-separated groups of 8-4-4-4-12 hex digits.
pub fn uuid_to_u128_literal(uuid: &str) -> String {
    let parts: Vec<&str> = uuid.split('-').collect();
    assert_eq!(
        parts.len(),
        5,
        "uuid_to_u128_literal: expected 5 hyphen-separated groups in `{uuid}`"
    );
    let d1 = u32::from_str_radix(parts[0], 16)
        .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d1 in `{uuid}`"));
    let d2 = u16::from_str_radix(parts[1], 16)
        .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d2 in `{uuid}`"));
    let d3 = u16::from_str_radix(parts[2], 16)
        .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d3 in `{uuid}`"));
    let d4_str = format!("{}{}", parts[3], parts[4]);
    let mut d4 = [0u8; 8];
    for i in 0..8 {
        d4[i] = u8::from_str_radix(&d4_str[i * 2..i * 2 + 2], 16)
            .unwrap_or_else(|_| panic!("uuid_to_u128_literal: invalid d4[{i}] in `{uuid}`"));
    }
    format_guid_u128(d1, d2, d3, d4)
}
