#![doc = include_str!("../readme.md")]
#![allow(
    non_upper_case_globals,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms
)]

mod dependencies;
mod derive;
mod derive_writer;
mod filter;
mod guid;
mod io;
mod item_tree;
mod references;
mod signature;
mod tables;
mod tokens;
mod type_name;
mod types;
mod value;
mod winmd;
mod writer;

use dependencies::*;
use derive::*;
use derive_writer::*;
use filter::*;
use guid::*;
use io::*;
use item_tree::*;
use references::*;
use signature::*;
use std::cmp::Ordering;
use std::collections::*;
use std::fmt::Write;
use tables::*;
use tokens::*;
use type_name::*;
use types::*;
use value::*;
use winmd::*;
use writer::*;
mod method_names;
use method_names::*;

struct Config {
    pub includes: Dependencies, // TODO: can we get rid of Includes and just use it to create the ItemTree?
    pub references: References,
    pub output: String,
    pub flat: bool,
    // Editor: probably want to say no to this since it can be achieved in Rust and we should avoid duplicating facilities that Rust itself can provide.
    // pub skip_root: bool, TODO: need something like this for compat or have a style option like --reference
    // where you can say style=<full/flat/skip-root> for consistency

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

    pub derive: Derive,
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
    let mut references = Vec::new();
    let mut derive = Vec::new();

    let mut flat = false;
    // let mut minimal = false;
    let mut no_allow = false;
    let mut no_comment = false;
    let mut no_deps = false; // TODO: can we drop this in favor of --reference ?
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
                "--reference" => kind = ArgKind::Reference,
                "--derive" => kind = ArgKind::Derive,
                "--flat" => flat = true,
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
            ArgKind::Reference => {
                // TODO: need to use Reader to validate reference type path?
                references.push(ReferenceStage::parse(arg));
            }
            ArgKind::Derive => {
                derive.push(arg.as_str());
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

    if input.is_empty() {
        panic!("windows-bindgen: one `--in` is required");
    };

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
    let includes = Dependencies::filter(reader, &filter);

    let references = References::new(reader, references);

    let derive = Derive::new(reader, &derive);

    let config = Box::leak(Box::new(Config {
        includes,
        flat,
        //  minimal,
        references,
        derive,
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
    let items = ItemTree::new(reader, config);

    // dbg!(&config.tree);

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
    Reference,
    Derive,
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
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                {
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
    let mut use_default = false;

    for input in input {
        if *input == "default" {
            use_default = true;
        } else {
            expand_input(&mut paths, input);
        }
    }

    let mut input = vec![];

    if use_default {
        input = [
            std::include_bytes!("../default/Windows.winmd").to_vec(),
            std::include_bytes!("../default/Windows.Win32.winmd").to_vec(),
            std::include_bytes!("../default/Windows.Wdk.winmd").to_vec(),
        ]
        .into_iter()
        .map(|bytes| File::new(bytes).unwrap())
        .collect();
    }

    input.extend(paths.iter().map(|path| {
        let bytes = std::fs::read(path)
            .unwrap_or_else(|_| panic!("windows-bindgen: failed to read binary file `{path}`"));

        File::new(bytes)
            .unwrap_or_else(|| panic!("windows-bindgen: failed to read .winmd format `{path}`"))
    }));

    input
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
