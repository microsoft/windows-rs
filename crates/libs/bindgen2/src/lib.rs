#![doc = include_str!("../readme.md")]
#![allow(
    non_upper_case_globals,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms
)]

mod dependencies;
mod derive;
mod filter;
mod guid;
mod io;
mod item;
mod item_tree;
mod name_tree;
mod signature;
mod tables;
mod tokens;
mod r#type;
mod type_name;
mod value;
mod winmd;
mod writer;

use dependencies::*;
use derive::*;
use filter::*;
use guid::*;
use io::*;
use item::*;
use item_tree::*;
use name_tree::*;
use r#type::*;
use signature::*;
use std::cmp::Ordering;
use std::collections::*;
use std::fmt::Write;
use tables::*;
use tokens::*;
use type_name::*;
use value::*;
use winmd::*;
use writer::*;
mod method_names;
use method_names::*;

struct Config {
    pub tree: NameTree, // TODO: can we get rid of NameTree and just use it to create the ItemTree?
    pub output: String,
    pub flat: bool,
    // pub minimal: bool, // TODO: if minimal then don't include dependencies for method parameters.
    // and possibly types who's dependencies are filtered out?
    // and unscoped enum variants?
    pub no_allow: bool,
    pub no_comment: bool,
    pub no_deps: bool, // TODO: to avoid refering to windows/windows-sys/windows-core/windows-targets crates - the default is to refer to types in windows-core/windows/windows-sys/windows-targets etc?
    pub package: bool,
    pub rustfmt: String,
    pub sys: bool, // TODO: if sys and not package then include minimal "vtbl" definitions

    // TODO: options to include deprecated APIs - excluded by default?
    // options to include preview APIs - excluded by default?
    /// this provides implementation support for exclusive WinRT types only - other types can always
    /// be implemented
    pub implement: bool,
}

/// The Windows code generator.
#[track_caller]
pub fn bindgen<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args = expand_args(args);
    let mut kind = ArgKind::None;
    let mut input = Vec::new();
    let mut include = Vec::new();
    let mut exclude = Vec::new();

    let mut flat = false;
    // let mut minimal = false;
    let mut no_allow = false;
    let mut no_comment = false;
    let mut no_deps = false;
    let mut package = false;
    let mut implement = false;
    let mut rustfmt = String::new();
    let mut output = String::new();
    let mut sys = false;

    for arg in &args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => kind = ArgKind::Output,
                "--filter" => kind = ArgKind::Filter,
                "--rustfmt" => kind = ArgKind::Rustfmt,
                "--flat" => flat = true,
                // "--minimal" => minimal = true,
                "--no-allow" => no_allow = true,
                "--no-comment" => no_comment = true,
                "--no-deps" => no_deps = true,
                "--package" => package = true,
                "--sys" => sys = true,
                "--implement" => implement = true,
                _ => panic!("windows-bindgen: invalid option `{arg}`"),
            },
            ArgKind::Output => {
                if output.is_empty() {
                    output = arg.to_string();
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
            ArgKind::Rustfmt => rustfmt = arg.to_string(),
        }
    }

    if sys && !package {
        no_deps = true;
    }

    if package && flat {
        panic!("windows-bindgen: cannot combine `--package` and `--flat` options");
    }

    if output.is_empty() {
        panic!("windows-bindgen: one `--out` is required");
    };

    // This isn't strictly necessary but avoids a common newbie pitfall where all metadata
    // would be generated when building a component for a specific API.
    if include.is_empty() {
        panic!("windows-bindgen: at least one `--filter` is required");
    }

    let reader = Reader::new(expand_input(&input));
    let filter = Filter::new(reader, &include, &exclude);
    let tree = NameTree::new(reader, &filter);

    let config = Box::leak(Box::new(Config {
        tree,
        flat,
        //  minimal,
        no_allow,
        no_comment,
        no_deps,
        package,
        rustfmt,
        output,
        sys,
        implement,
    }));

    //dbg!(&filter);

    // TODO: maybe pass this "name" tree to the writer so that when it comes to generating methods it can figure out whether to include
    // it based on whether its parameters are included. It may be excluded by "--minimal" was specified.

    // TODO: the "name tree" wouldn't be needed after creating the "item tree" if the root/core named types were represented by Item(...)

    // dbg!(&tree);

    // TODO: this is where we need to populate the tree with methods based on whether or not they're included!!
    let items = ItemTree::new(reader, &config.tree);

    // dbg!(&items);

    let writer = Writer {
        config,
        namespace: "",
    };

    writer.write(items)
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    Rustfmt,
}

fn expand_args<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    // This function is needed to avoid a recursion limit in the Rust compiler.
    // TODO: maybe we need to avoid the recursion altogether?
    fn from_string(result: &mut Vec<String>, value: &str) {
        expand_args(result, value.split_whitespace().map(|arg| arg.to_string()))
    }

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
                        from_string(result, &args);
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

fn namespace_starts_with(namespace: &str, starts_with: &str) -> bool {
    namespace.starts_with(starts_with)
        && (namespace.len() == starts_with.len()
            || namespace.as_bytes().get(starts_with.len()) == Some(&b'.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with() {
        assert!(namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D11on12",
            "Windows.Win32.Graphics.Direct3D11on12"
        ));
        assert!(namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D11on12",
            "Windows.Win32.Graphics"
        ));
        assert!(!namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D11on12",
            "Windows.Win32.Graphics.Direct3D11"
        ));
        assert!(!namespace_starts_with(
            "Windows.Win32.Graphics.Direct3D",
            "Windows.Win32.Graphics.Direct3D11"
        ));
    }
}
