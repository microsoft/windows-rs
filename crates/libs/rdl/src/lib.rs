#![doc = include_str!("../readme.md")]

mod clang;
mod error;
pub mod formatter;
mod reader;
mod writer;

use std::collections::{BTreeMap, HashMap, HashSet};
use syn::spanned::Spanned;
use windows_metadata as metadata;

pub use clang::Clang;
pub use error::Error;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
pub use reader::Reader;
pub use writer::Writer;

pub fn reader() -> Reader {
    Reader::new()
}

pub fn writer() -> Writer {
    Writer::new()
}

pub fn clang() -> Clang {
    Clang::new()
}

/// Returns the version string of the loaded libclang library,
/// e.g. `"clang version 18.1.0 (...)"`.
pub fn clang_version() -> Result<String, Error> {
    Clang::version()
}

fn expand_input_paths(
    inputs: &[String],
    ext1: &str,
    ext2: &str,
) -> Result<(Vec<String>, Vec<String>), Error> {
    let mut paths1 = vec![];
    let mut paths2 = vec![];

    for input in inputs {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_total = paths1.len() + paths2.len();

            for entry_path in path
                .read_dir()
                .map_err(|_| Error::new("failed to read directory", input, 0, 0))?
                .flatten()
                .map(|entry| entry.path())
            {
                if entry_path.is_file() {
                    if entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(ext1))
                    {
                        paths1.push(entry_path.to_string_lossy().replace('\\', "/"));
                    } else if entry_path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(ext2))
                    {
                        paths2.push(entry_path.to_string_lossy().replace('\\', "/"));
                    }
                }
            }

            if paths1.len() + paths2.len() == prev_total {
                return Err(Error::new(
                    &format!("failed to find .{ext1} or .{ext2} files in directory"),
                    input,
                    0,
                    0,
                ));
            }
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(ext1))
        {
            paths1.push(input.clone());
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(ext2))
        {
            paths2.push(input.clone());
        } else {
            return Err(Error::new(
                &format!("expected .{ext1} or .{ext2} file"),
                input,
                0,
                0,
            ));
        }
    }

    Ok((paths1, paths2))
}

fn write_to_file<C: AsRef<[u8]>>(path: &str, contents: C) -> Result<(), Error> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| writer_err!("failed to create directory `{path}`"))?;
    }

    std::fs::write(path, contents).map_err(|_| writer_err!("failed to write file `{path}`"))
}

fn write_ident(name: &str) -> TokenStream {
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

macro_rules! writer_err {
    ($($arg:tt)*) => {
        Error::new(&format!($($arg)*), "", 0, 0)
    };
}

use writer_err;

fn write_type(namespace: &str, item: &metadata::Type) -> TokenStream {
    use metadata::Type::*;
    match item {
        Bool => quote! { bool },
        Char => quote! { u16 },
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

fn write_value(namespace: &str, value: &metadata::Value) -> TokenStream {
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
fn format_guid_u128(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> String {
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
fn uuid_to_u128_literal(uuid: &str) -> String {
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
