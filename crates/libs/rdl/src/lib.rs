#![doc = include_str!("../readme.md")]
#![allow(clippy::large_enum_variant)]

mod error;
mod formatter;
mod reader;
mod writer;

use std::collections::{BTreeMap, HashSet};
use syn::spanned::Spanned;

pub use error::Error;
pub use reader::Reader;
pub use writer::Writer;

fn expand_files(inputs: &[String], extension: &str) -> Result<Vec<String>, Error> {
    fn expand_one(result: &mut Vec<String>, input: &str, extension: &str) -> Result<(), Error> {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .map_err(|_| Error::new("failed to read directory", input, 0, 0))?
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
                {
                    result.push(path.to_string_lossy().replace('\\', "/"));
                }
            }

            if result.len() == prev_len {
                return Err(Error::new(
                    &format!("failed to find .{extension} files in directory"),
                    input,
                    0,
                    0,
                ));
            }
        } else {
            result.push(input.to_string());
        }

        Ok(())
    }

    let mut result = vec![];

    for input in inputs {
        expand_one(&mut result, input, extension)?;
    }

    Ok(result)
}
