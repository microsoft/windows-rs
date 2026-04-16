#![doc = include_str!("../readme.md")]

mod clang;
mod error;
pub mod formatter;
mod reader;
mod writer;

use std::collections::{BTreeMap, HashSet};
use syn::spanned::Spanned;

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
