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
