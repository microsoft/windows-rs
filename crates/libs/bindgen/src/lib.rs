#![doc = include_str!("../readme.md")]
#![expect(
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    non_snake_case,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms
)]

mod config;
mod derive;
mod derive_writer;
mod filter;
mod guid;
mod index;
mod io;
mod libraries;
mod param;
mod references;
mod signature;
mod tables;
mod tokens;
mod type_map;
mod type_name;
mod type_tree;
mod types;
mod value;
mod warnings;
mod winmd;

use config::*;
use derive::*;
use derive_writer::*;
use filter::*;
use guid::*;
use io::*;
pub use libraries::*;
use param::*;
use references::*;
use signature::*;
use std::cmp::Ordering;
use std::collections::*;
use std::fmt::Write;
use tables::*;
use tokens::*;
use type_map::*;
use type_name::*;
use type_tree::*;
use types::*;
use value::*;
pub use warnings::*;
use winmd::*;
mod method_names;
use method_names::*;

/// The conventional way of calling the `bindgen` function is as follows:
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
/// ```
///
/// Here is a list of supported arguments.
///
/// | Argument | Description |
/// |----------|-------------|
/// | `--in` | .winmd files or directories to include. |
/// | `--out` | File name where the generated bindings will be saved. |
/// | `--filter` | APIs to include or exclude in the generated bindings. |
/// | `--rustfmt` | Overrides the default Rust formatting. |
/// | `--derive` | Extra traits for types to derive. |
/// | `--flat` | Avoids the default namespace-to-module conversion. |
/// | `--no-allow` | Avoids generating the default `allow` attribute. |
/// | `--no-comment` | Avoids generating the code generation comment. |
/// | `--no-deps` | Avoids dependencies on the various `windows-*` crates. |
/// | `--sys` | Generates raw or sys-style Rust bindings. |
/// | `--implement` | Includes implementation traits for WinRT interfaces. |
/// | `--link` | Overrides the default `windows-link` implementation for system calls. |
///
///
/// # `--out`
///
/// Exactly one `--out` argument is required and instructs the `bindgen` function where to write the bindings.
///
/// # `--filter`
///
/// At least one `--filter` is required and indicates what APIs to include in the generated bindings.
/// The following will, for example, also include the `Sleep` function:
///
/// ```rust
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
///     "Sleep",
/// ];
/// ```
///
/// The `--filter` argument can refer to the function or type name and nothing more. You can also refer
/// to the namespace that the API metadata uses to group functions and types:
///
/// ```rust
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "Windows.Foundation.Numerics",
///     "!Windows.Foundation.Numerics.Matrix3x2",
/// ];
/// ```
///
/// In this example, all types from the `Windows.Foundation.Numerics` namepace are included with the
/// exception of `Matrix3x2` which is excluded due to the `!` preamble.
///
/// # `--in`
///
/// `--in` can indicate a .winmd file or directory containing .winmd files. Alternatively, the special
/// "default" input can be used to include the particular .winmd files that ship with the `windows-bindgen`
/// crate. This may used to combine the default metadata with specific .winmd files.
///
/// ```rust
/// let args = [
///     "--in",
///     "default",
///     "Sample.winmd",
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "Sample",
/// ];
/// ```
///
/// # `--flat`
///
/// By default, the bindings include a mapping of namespaces to modules. Consider this example again:
///
/// ```rust
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
///     "Sleep",
/// ];
/// ```
///
/// The resulting bindings might look something like this:
///
/// ```rust
/// pub mod Windows {
///     pub mod Win32 {
///         pub mod System {
///             pub mod SystemInformation {
///                 #[inline]
///                 pub unsafe fn GetTickCount() -> u32 {
///                     windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
///                     unsafe { GetTickCount() }
///                 }
///             }
///             pub mod Threading {
///                 #[inline]
///                 pub unsafe fn Sleep(dwmilliseconds: u32) {
///                     windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
///                     unsafe { Sleep(dwmilliseconds) }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// That's because the default metadata defines `GetTickCount` in the `Windows.Win32.System.SystemInformation`
/// namespace while `Sleep` is defined in the `Windows.Win32.System.Threading` namespace. Fortunately, it's
/// easy to turn that off by using the `--flat` argument:
///
/// ```rust
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--flat",
///     "--filter",
///     "GetTickCount",
///     "Sleep",
/// ];
/// ```
///
/// The resulting bindings now look something like this:
///
/// ```rust
/// #[inline]
/// pub unsafe fn GetTickCount() -> u32 {
///     windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
///     unsafe { GetTickCount() }
/// }
/// #[inline]
/// pub unsafe fn Sleep(dwmilliseconds: u32) {
///     windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
///     unsafe { Sleep(dwmilliseconds) }
/// }
/// ```
///
/// # `--no-allow`
///
/// The bindings also include an allow attribute that covers various common warnings inherent in
/// generated bindings.
///
/// ```rust
/// #![allow(
///     non_snake_case,
///     non_upper_case_globals,
///     non_camel_case_types,
///     dead_code,
///     clippy::all
/// )]
/// ```
///
/// You can prevent this from being generated if you prefer to manage this yourself with the `--no-allow`
/// argument.
///
/// # `--sys`
///
/// The `--sys` argument instruct the `bindgen` function to generate raw, sometimes called sys-style Rust
/// bindings.
///
/// ```rust
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--flat",
///     "--sys",
///     "--filter",
///     "GetTickCount",
///     "Sleep",
/// ];
/// ```
///
/// The resulting bindings now look something like this:
///
/// ```rust
/// windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);
/// windows_link::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
/// ```
///
/// You'll notice that the bindings are simpler as there's no wrapper functions and other
/// conveniences. You just need to add a dependency on the tiny [windows-link](https://crates.io/crates/windows-link) crate and you're all set.
///
#[track_caller]
#[must_use]
pub fn bindgen<I, S>(args: I) -> Warnings
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
    let mut no_allow = false;
    let mut no_comment = false;
    let mut no_deps = false;
    let mut no_toml = false;
    let mut package = false;
    let mut implement = false;
    let mut specific_deps = false;
    let mut rustfmt = String::new();
    let mut output = String::new();
    let mut sys = false;
    let mut link = String::new();
    let mut index = false;

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
                "--no-toml" => no_toml = true,
                "--package" => package = true,
                "--sys" => sys = true,
                "--implement" => implement = true,
                "--specific-deps" => specific_deps = true,
                "--link" => kind = ArgKind::Link,
                "--index" => index = true,
                _ => panic!("invalid option `{arg}`"),
            },
            ArgKind::Output => {
                if output.is_empty() {
                    output = arg.to_string();
                } else {
                    panic!("exactly one `--out` is required");
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
                references.push(ReferenceStage::parse(arg));
            }
            ArgKind::Derive => {
                derive.push(arg.as_str());
            }
            ArgKind::Rustfmt => rustfmt = arg.to_string(),
            ArgKind::Link => link = arg.to_string(),
        }
    }

    if link.is_empty() {
        if sys || specific_deps {
            link = "windows_link".to_string();
        } else {
            link = "windows_core".to_string();
        }
    }

    if package && flat {
        panic!("cannot combine `--package` and `--flat`");
    }

    if input.is_empty() {
        input.push("default");
    };

    if output.is_empty() {
        panic!("exactly one `--out` is required");
    };

    if !sys && !no_deps {
        references.insert(
            0,
            ReferenceStage::parse("windows_collections,flat,Windows.Foundation.Collections"),
        );
        references.insert(
            0,
            ReferenceStage::parse("windows_numerics,flat,Windows.Foundation.Numerics"),
        );
        references.insert(
            0,
            ReferenceStage::parse("windows_future,flat,Windows.Foundation.Async*"),
        );
        references.insert(
            0,
            ReferenceStage::parse("windows_future,flat,Windows.Foundation.IAsync*"),
        );
    }

    // This isn't strictly necessary but avoids a common newbie pitfall where all metadata
    // would be generated when building a component for a specific API.
    if include.is_empty() {
        panic!("at least one `--filter` required");
    }

    let reader = Reader::new(expand_input(&input));
    let filter = Filter::new(&reader, &include, &exclude);
    let references = References::new(&reader, references);
    let types = TypeMap::filter(&reader, &filter, &references);
    let derive = Derive::new(&reader, &types, &derive);
    let warnings = WarningBuilder::default();

    let config = Config {
        types: &types,
        flat,
        references: &references,
        derive: &derive,
        no_allow,
        no_comment,
        no_deps,
        no_toml,
        package,
        rustfmt: &rustfmt,
        output: &output,
        sys,
        implement,
        specific_deps,
        link: &link,
        warnings: &warnings,
        namespace: "",
    };

    let tree = TypeTree::new(&types);

    config.write(tree);

    if index {
        index::write(&types, &format!("{output}/features.json"));
    }

    warnings.build()
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    Rustfmt,
    Reference,
    Derive,
    Link,
}

#[track_caller]
fn expand_args<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    // This function is needed to avoid a recursion limit in the Rust compiler.
    #[track_caller]
    fn from_string(result: &mut Vec<String>, value: &str) {
        expand_args(result, value.split_whitespace().map(|arg| arg.to_string()))
    }

    #[track_caller]
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

#[track_caller]
fn expand_input(input: &[&str]) -> Vec<File> {
    #[track_caller]
    fn expand_input(result: &mut Vec<String>, input: &str) {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .unwrap_or_else(|_| panic!("failed to read directory `{input}`"))
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
                panic!("failed to find .winmd files in directory `{input}`");
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

    for path in &paths {
        let Ok(bytes) = std::fs::read(path) else {
            panic!("failed to read binary file `{path}`");
        };

        let Some(file) = File::new(bytes) else {
            panic!("failed to read .winmd format `{path}`");
        };

        input.push(file);
    }

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
