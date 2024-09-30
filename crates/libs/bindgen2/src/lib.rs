#![doc = include_str!("../readme.md")]
#![allow(
    non_upper_case_globals,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms
)]
// TODO: remove this once bindgen2 is up and running
#![allow(dead_code)]

mod filter;
mod io;
mod tree;
mod winmd;
mod writer;

use filter::*;
use std::cmp::Ordering;
use std::collections::*;
use tree::*;
use writer::*;
use winmd::*;

/// The Windows code generator.
pub fn bindgen<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args = expand_args(args);
    let mut kind = ArgKind::None;
    let mut output = None;
    let mut input = Vec::new();
    let mut include = Vec::new();
    let mut exclude = Vec::new();
    let mut flatten = false;
    let mut package = false;

    for arg in &args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => kind = ArgKind::Output,
                "--filter" => kind = ArgKind::Filter,
                "--flatten" => flatten = true,
                "--package" => package = true,
                _ => panic!("windows-bindgen: invalid option"),
            },
            ArgKind::Output => {
                if output.is_none() {
                    output = Some(arg.as_str());
                } else {
                    panic!("windows-bindgen: too many outputs");
                }
            }
            ArgKind::Input => input.push(arg.as_str()),
            ArgKind::Filter => {
                if let Some(rest) = arg.strip_prefix('!') {
                    exclude.push(rest);
                } else {
                    include.push(arg.as_str());
                }
            }
        }
    }

    if package && flatten {
        panic!("windows-bindgen: cannot combine `--package` and `--flatten` options");
    }

    let Some(output) = output.map(|output| output.to_string()) else {
        panic!("windows-bindgen: one `--out` is required");
    };

    // This isn't strictly necessary but avoids a common newbie pitfall where all metadata
    // would be generated when building a component for a specific API.
    if include.is_empty() {
        panic!("windows-bindgen: at least one `--filter` is required");
    }

    let reader = Reader::new(expand_input(&input));
    let filter = Filter::new(reader, &include, &exclude);
    let tree = Tree::new(reader, &filter, !package);

    dbg!(&tree);

    let writer = Writer {
        reader,
        output,
        flatten,
        package,
    };

    writer.write(&tree)
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
}

fn expand_args<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn expand_args<I, S>(result: &mut Vec<String>, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut expand = false;

        for arg in args.into_iter().map(|arg| arg.as_ref().to_string()) {
            if arg.starts_with('-') {
                expand = false;
            }
            if expand {
                for args in io::read_file_lines(&arg) {
                    if !args.starts_with("//") {
                        expand_args(result, [args]);
                    }
                }
            } else if arg == "--etc" {
                expand = true;
            } else {
                result.push(arg);
            }
        }
    }

    let mut result = vec![];
    expand_args(&mut result, args);
    result
}

fn expand_input(input: &[&str]) -> Vec<File> {
    fn expand_input(result: &mut Vec<String>, input: &str) {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .unwrap_or_else(|_| panic!("windows-bindgen: failed to read directory `{input}`"))
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file() {
                    result.push(path.to_string_lossy().to_string());
                }
            }

            if result.len() == prev_len {
                panic!("windows-bindgen: failed to find files in directory `{input}`");
            }
        } else {
            result.push(input.to_string());
        }
    }

    let mut paths = vec![];

    for input in input {
        expand_input(&mut paths, input);
    }

    if paths.is_empty() {
        [
            std::include_bytes!("../default/Windows.winmd").to_vec(),
            std::include_bytes!("../default/Windows.Win32.winmd").to_vec(),
            std::include_bytes!("../default/Windows.Wdk.winmd").to_vec(),
        ]
        .into_iter()
        .map(|bytes| File::new(bytes).unwrap())
        .collect()
    } else {
        paths
            .iter()
            .map(|path| {
                let bytes = std::fs::read(path).unwrap_or_else(|_| {
                    panic!("windows-bindgen: failed to read binary file `{path}`")
                });

                File::new(bytes).unwrap_or_else(|| {
                    panic!("windows-bindgen: failed to read .winmd format `{path}`")
                })
            })
            .collect()
    }
}
