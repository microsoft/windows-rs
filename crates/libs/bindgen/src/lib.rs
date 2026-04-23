#![doc = include_str!("../readme.md")]
#![expect(
    non_upper_case_globals,
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

pub fn builder() -> Bindgen {
    Bindgen::new()
}

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
/// | `--specific-deps` | Uses specific crate dependencies rather than `windows-core`. |
/// | `--sys` | Generates raw or sys-style Rust bindings. |
/// | `--sys-fn-ptrs` | Additionally generates function pointers for sys-style Rust bindings. |
/// | `--sys-fn-extern` | Generates extern declarations rather than link macros for sys-style Rust bindings. |
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
/// # `--specific-deps`
///
/// By default, `windows-bindgen` uses `windows-core` uniformly for most dependencies to provide
/// consistency and convenience. However, if you want to avoid a dependency on the `windows-core`
/// crate entirely and instead target specific - and much smaller - crates directly, you can use
/// the `--specific-deps` option.
///
/// Consider the following example using the `WindowsStringHasEmbeddedNull` function:
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--flat",
///     "--filter",
///     "WindowsStringHasEmbeddedNull",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
/// ```
///
/// By default, the generated bindings will reference `windows_core` types and use the
/// `windows_core::link!` macro for convenience and consistent dependency management.
///
/// With the `--specific-deps` option:
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--flat",
///     "--specific-deps",
///     "--filter",
///     "WindowsStringHasEmbeddedNull",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
/// ```
///
/// The bindings will now directly reference specific crates such as `windows_strings::HSTRING`,
/// `windows_result::Result`, and `windows_link::link!` instead of `windows_core`. This allows
/// for fine-grained dependency management and can significantly reduce your dependency tree if
/// you don't need the full `windows-core` functionality.
///
/// This option is not the default because this level of control is not for everyone, but if you
/// need fine-grained dependency management and want to minimize your dependency tree, this option
/// provides that flexibility.
///
#[track_caller]
#[must_use]
pub fn bindgen<I, S>(args: I) -> Warnings
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args = expand_args(args);
    let mut builder = Bindgen::new();
    let mut kind = ArgKind::None;
    let mut has_output = false;

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
                "--flat" => {
                    builder.flat();
                }
                "--no-allow" => {
                    builder.no_allow();
                }
                "--no-comment" => {
                    builder.no_comment();
                }
                "--no-deps" => {
                    builder.no_deps();
                }
                "--no-toml" => {
                    builder.no_toml();
                }
                "--package" => {
                    builder.package();
                }
                "--sys" => {
                    builder.sys();
                }
                "--sys-fn-ptrs" => {
                    builder.sys_fn_ptrs();
                }
                "--sys-fn-extern" => {
                    builder.sys_fn_extern();
                }
                "--typedef" => {
                    builder.typedef();
                }
                "--implement" => {
                    builder.implement();
                }
                "--specific-deps" => {
                    builder.specific_deps();
                }
                "--link" => kind = ArgKind::Link,
                "--index" => {
                    builder.index();
                }
                _ => panic!("invalid option `{arg}`"),
            },
            ArgKind::Output => {
                if has_output {
                    panic!("exactly one `--out` is required");
                }
                builder.output(arg);
                has_output = true;
            }
            ArgKind::Input => {
                builder.input(arg);
            }
            ArgKind::Filter => {
                builder.filter(arg);
            }
            ArgKind::Reference => {
                builder.reference(arg);
            }
            ArgKind::Derive => {
                builder.derive(arg);
            }
            ArgKind::Rustfmt => {
                builder.rustfmt(arg);
            }
            ArgKind::Link => {
                builder.link(arg);
            }
        }
    }

    if builder.package && builder.flat {
        panic!("cannot combine `--package` and `--flat`");
    }

    if !has_output {
        panic!("exactly one `--out` is required");
    }

    builder.write()
}

/// Builder for generating Windows API bindings.
///
/// This provides a fluent builder API as an alternative to the command-line-like [`bindgen`] function.
///
/// # Example
///
/// ```rust,no_run
/// windows_bindgen::Bindgen::new()
///     .output("src/bindings.rs")
///     .filter("GetTickCount")
///     .write()
///     .unwrap();
/// ```
#[derive(Default)]
pub struct Bindgen {
    input: Vec<String>,
    filter: Vec<String>,
    output: String,
    references: Vec<String>,
    derive: Vec<String>,
    rustfmt: String,
    link: String,
    flat: bool,
    no_allow: bool,
    no_comment: bool,
    no_deps: bool,
    no_toml: bool,
    package: bool,
    implement: bool,
    specific_deps: bool,
    sys: bool,
    typedef: bool,
    sys_fn_ptrs: bool,
    sys_fn_extern: bool,
    index: bool,
}

impl Bindgen {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a `.winmd` file or directory containing `.winmd` files.
    /// Use `"default"` to include the metadata bundled with `windows-bindgen`.
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.input.push(input.to_string());
        self
    }

    /// Set the output file where generated bindings will be written.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
        self
    }

    /// Add a filter rule to include or exclude APIs.
    ///
    /// Filter rules may be a function or type name, a namespace prefix, or a fully-qualified name.
    /// Prefix with `!` to exclude rather than include.
    pub fn filter(&mut self, filter: &str) -> &mut Self {
        self.filter.push(filter.to_string());
        self
    }

    /// Add an extra trait for types to derive.
    pub fn derive(&mut self, derive: &str) -> &mut Self {
        self.derive.push(derive.to_string());
        self
    }

    /// Add a reference dependency.
    pub fn reference(&mut self, reference: &str) -> &mut Self {
        self.references.push(reference.to_string());
        self
    }

    /// Override the default Rust formatter path.
    pub fn rustfmt(&mut self, rustfmt: &str) -> &mut Self {
        self.rustfmt = rustfmt.to_string();
        self
    }

    /// Override the default `windows-link` implementation for system calls.
    pub fn link(&mut self, link: &str) -> &mut Self {
        self.link = link.to_string();
        self
    }

    /// Avoid the default namespace-to-module conversion.
    pub fn flat(&mut self) -> &mut Self {
        self.flat = true;
        self
    }

    /// Avoid generating the default `allow` attribute.
    pub fn no_allow(&mut self) -> &mut Self {
        self.no_allow = true;
        self
    }

    /// Avoid generating the code generation comment.
    pub fn no_comment(&mut self) -> &mut Self {
        self.no_comment = true;
        self
    }

    /// Avoid dependencies on the various `windows-*` crates.
    pub fn no_deps(&mut self) -> &mut Self {
        self.no_deps = true;
        self
    }

    /// Avoid generating the Cargo.toml features when using `package` mode.
    pub fn no_toml(&mut self) -> &mut Self {
        self.no_toml = true;
        self
    }

    /// Generate bindings as a package with one file per namespace.
    pub fn package(&mut self) -> &mut Self {
        self.package = true;
        self
    }

    /// Include implementation traits for WinRT interfaces.
    pub fn implement(&mut self) -> &mut Self {
        self.implement = true;
        self
    }

    /// Use specific crate dependencies rather than `windows-core`.
    pub fn specific_deps(&mut self) -> &mut Self {
        self.specific_deps = true;
        self
    }

    /// Generate raw or sys-style Rust bindings.
    pub fn sys(&mut self) -> &mut Self {
        self.sys = true;
        self
    }

    /// Generate raw or sys-style type aliases.
    pub fn typedef(&mut self) -> &mut Self {
        self.typedef = true;
        self
    }

    /// Additionally generate function pointers for sys-style Rust bindings.
    pub fn sys_fn_ptrs(&mut self) -> &mut Self {
        self.sys_fn_ptrs = true;
        self
    }

    /// Generate extern declarations rather than link macros for sys-style Rust bindings.
    pub fn sys_fn_extern(&mut self) -> &mut Self {
        self.sys_fn_extern = true;
        self
    }

    /// Generate a `features.json` index alongside the output file.
    pub fn index(&mut self) -> &mut Self {
        self.index = true;
        self
    }

    /// Generate the bindings.
    #[track_caller]
    #[must_use]
    pub fn write(&self) -> Warnings {
        let mut include: Vec<&str> = vec![];
        let mut exclude: Vec<&str> = vec![];

        for f in &self.filter {
            if let Some(rest) = f.strip_prefix('!') {
                exclude.push(rest);
            } else {
                include.push(f.as_str());
            }
        }

        let link = if self.link.is_empty() {
            if self.sys || self.specific_deps {
                "windows_link"
            } else {
                "windows_core"
            }
        } else {
            self.link.as_str()
        };

        let default_input = ["default"];
        let input: Vec<&str> = if self.input.is_empty() {
            default_input.to_vec()
        } else {
            self.input.iter().map(|s| s.as_str()).collect()
        };

        if self.output.is_empty() {
            panic!("output is required (call `.output()` or pass `--out`)");
        }

        if include.is_empty() {
            panic!("at least one `--filter` required");
        }

        if self.package && self.flat {
            panic!("cannot combine `--package` and `--flat`");
        }

        let reader = Reader::new(expand_input(&input));

        let mut references: Vec<ReferenceStage> = self
            .references
            .iter()
            .map(|s| ReferenceStage::parse(s))
            .collect();

        if !self.sys && !self.no_deps {
            if reader.contains_key("Windows.Foundation") {
                references.insert(
                    0,
                    ReferenceStage::parse(
                        "windows_collections,flat,Windows.Foundation.Collections",
                    ),
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

            if reader.contains_key("Windows.Win32.Foundation") {
                if self.specific_deps {
                    references.insert(
                        0,
                        ReferenceStage::parse(
                            "windows_result,flat,Windows.Win32.Foundation.WIN32_ERROR",
                        ),
                    );
                    references.insert(
                        0,
                        ReferenceStage::parse(
                            "windows_result,flat,Windows.Win32.Foundation.NTSTATUS",
                        ),
                    );
                    references.insert(
                        0,
                        ReferenceStage::parse(
                            "windows_result,flat,Windows.Win32.System.Rpc.RPC_STATUS",
                        ),
                    );
                } else {
                    references.insert(
                        0,
                        ReferenceStage::parse(
                            "windows_core,flat,Windows.Win32.Foundation.WIN32_ERROR",
                        ),
                    );
                    references.insert(
                        0,
                        ReferenceStage::parse(
                            "windows_core,flat,Windows.Win32.Foundation.NTSTATUS",
                        ),
                    );
                    references.insert(
                        0,
                        ReferenceStage::parse(
                            "windows_core,flat,Windows.Win32.System.Rpc.RPC_STATUS",
                        ),
                    );
                }
            }
        }

        let derive_str: Vec<&str> = self.derive.iter().map(|s| s.as_str()).collect();

        let filter = Filter::new(&reader, &include, &exclude);
        let references = References::new(&reader, references);
        let types = TypeMap::filter(&reader, &filter, &references);
        let derive = Derive::new(&reader, &types, &derive_str);
        let warnings = WarningBuilder::default();

        let config = Config {
            reader: &reader,
            types: &types,
            flat: self.flat,
            references: &references,
            derive: &derive,
            no_allow: self.no_allow,
            no_comment: self.no_comment,
            no_deps: self.no_deps,
            no_toml: self.no_toml,
            package: self.package,
            rustfmt: &self.rustfmt,
            output: &self.output,
            sys: self.sys,
            typedef: self.typedef,
            sys_fn_ptrs: self.sys_fn_ptrs,
            sys_fn_extern: self.sys_fn_extern,
            implement: self.implement,
            specific_deps: self.specific_deps,
            link,
            warnings: &warnings,
            namespace: "",
        };

        let tree = TypeTree::new(&types);
        config.write(tree);

        if self.index {
            index::write(&types, &format!("{}/features.json", self.output), &reader);
        }

        warnings.build()
    }
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
