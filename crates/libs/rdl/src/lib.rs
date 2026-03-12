#![doc = include_str!("../readme.md")]
#![allow(dead_code)]

mod error;
mod formatter;
mod reader;
mod writer;

use std::collections::{BTreeMap, HashSet};
use syn::spanned::Spanned;

pub use error::Error;
pub use reader::Reader;
pub use writer::Writer;

fn expand_winmd(inputs: &[String]) -> Result<Vec<String>, Error> {
    fn expand_one(result: &mut Vec<String>, input: &str) -> Result<(), Error> {
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
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                {
                    result.push(path.to_string_lossy().replace('\\', "/"));
                }
            }

            if result.len() == prev_len {
                return Err(Error::new(
                    "failed to find .winmd files in directory",
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
        expand_one(&mut result, input)?;
    }

    Ok(result)
}
