use super::*;

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
/// | `--sys` | Generates raw C-style bindings with no `windows-core` dependency (only `windows-link`). |
/// | `--extern` | Generates extern declarations rather than link macros for sys-style Rust bindings. Only valid with `--sys`. |
/// | `--minimal` | Generates minimal-mode bindings: drops per-class wrapper methods, inherited interface forwarders, sys-style typedef handles, and sys-style free function wrappers to reduce build time; also replaces each `add_*`/`remove_*` event accessor pair with a single auto-revoking method. Mutually exclusive with `--sys`. |
/// | `--implement` | Includes implementation traits for WinRT interfaces. With no following names, emits `_Impl` scaffolding for every WinRT interface in scope; with one or more type-name patterns, narrows emission to the listed types only. |
/// | `--link` | Overrides the default `windows-link` implementation for system calls. |
/// | `--etc` | Reads additional whitespace-separated arguments from one or more response files (lines beginning with `//` are ignored). |
///
///
/// # `--out`
///
/// Exactly one `--out` argument is required and instructs the `bindgen` function where to write the bindings.
///
/// # `--filter`
///
/// The `--filter` option controls which APIs are included or excluded in the generated bindings. You can
/// specify one or more include patterns and optionally exclude patterns prefixed with `!`.
///
/// ```text
/// --filter Windows.Win32.Storage.FileSystem.GetFullPathNameW
///
/// --filter Windows.Win32.Storage.FileSystem.GetFullPathNameW
///          !Windows.Win32.Storage.FileSystem.WIN32_FIND_DATAW
///
/// --filter Windows.Win32.Storage.FileSystem
///
/// --filter Windows.Win32.Storage.FileSystem
///          !Windows.Win32.Storage.FileSystem.WIN32_FIND_DATAW
/// ```
///
/// There is a filter convention that uses `--etc` response files that makes it easy to separate include
/// and exclude filters in a separate file:
///
/// ```text
/// --etc path/to/file.txt
/// ```
///
/// That said, some internal crates in the `windows-rs` repo do this as well; you can see examples in
/// the `crates/tools/bindings/src` directory where filter `.txt` files are read and processed.
///
/// ## Method-level filtering
///
/// In `--minimal` mode, filters can additionally target individual methods,
/// properties, and events on an interface, class, or struct.  The syntax
/// looks like `Namespace.Type::Member`.  You can also use `Property.Name`
/// or `Event.Name` sugar to target a getter/setter pair or an event pair.
///
/// ```text
/// --filter Windows.UI.Xaml.Controls.Button
///          Windows.UI.Xaml.Controls.TextBlock::put_Text
///          Windows.UI.Xaml.Controls.TextBlock::Property.FontSize
///          Windows.UI.Xaml.UIElement::Event.PointerPressed
/// ```
///
/// # `--in`
///
/// The `--in` option specifies the `.winmd` files to include for the generation.
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--in",
///     "my.winmd",
///     "--filter",
///     "MyApi",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
/// ```
///
/// # `--flat`
///
/// The `--flat` option avoids the default namespace-to-module conversion and instead generates
/// all the bindings in a single flat list in the `--out` output file. This avoids namespace module
/// nesting and makes the output more straightforward.
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
///     "--flat",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
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
/// ```
///
/// # `--derive`
///
/// The `--derive` option instructs the code generator to add the specified traits to the
/// default `derive` list for all generated types that support it.
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
///     "--derive",
///     "PartialEq",
///     "Eq",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
/// ```
///
/// # `--sys`
///
/// The `--sys` option generates raw or sys-style Rust bindings that do not include some of
/// the wrappers and other conveniences provided by the default bindings.
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
///     "Sleep",
///     "--sys",
///     "--flat",
/// ];
///
/// windows_bindgen::bindgen(args).unwrap();
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
/// # `--deps`
///
/// Controls which `windows-*` crates the generated bindings depend on.
///
/// - `core` — depend on `windows-core` (default).
/// - `specific` — depend on `windows-result`, `windows-strings`, and `windows-link` directly.
/// - `none` — no `windows-*` dependencies; shared types like `PCWSTR` and `GUID` are emitted
///   inline. Default for `--sys` bindings.
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
                "--deps" => kind = ArgKind::Deps,
                "--no-toml" => {
                    builder.no_toml();
                }
                "--package" => {
                    builder.package();
                }
                "--sys" => {
                    builder.sys();
                }
                "--minimal" => {
                    builder.minimal();
                }
                "--dead-code" => {
                    builder.dead_code();
                }
                "--extern" => {
                    builder.extern_fns();
                }
                "--implement" => {
                    builder.implement.get_or_insert_with(Vec::new);
                    kind = ArgKind::Implement;
                }
                "--link" => kind = ArgKind::Link,
                "--index" => {
                    builder.index();
                }
                _ => panic!("invalid option `{arg}`"),
            },
            ArgKind::Output => {
                assert!(!has_output, "exactly one `--out` is required");
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
            ArgKind::Implement => {
                builder
                    .implement
                    .get_or_insert_with(Vec::new)
                    .push(arg.clone());
            }
            ArgKind::Rustfmt => {
                builder.rustfmt(arg);
            }
            ArgKind::Link => {
                builder.link(arg);
            }
            ArgKind::Deps => {
                builder.deps(match arg.as_str() {
                    "core" => DepMode::Core,
                    "specific" => DepMode::Specific,
                    "none" => DepMode::None,
                    other => {
                        panic!("invalid `--deps` value `{other}`; expected `core`, `specific`, or `none`")
                    }
                });
            }
        }
    }

    builder.write()
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    Rustfmt,
    Reference,
    Derive,
    Implement,
    Link,
    Deps,
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
        // Split on whitespace but keep `{...}` groups together so that
        // `Type::{a, b}` is not split across multiple args.
        let mut args = Vec::new();
        let mut current = String::new();
        let mut brace_depth = 0u32;

        for ch in value.chars() {
            if ch == '{' {
                brace_depth += 1;
                current.push(ch);
            } else if ch == '}' {
                brace_depth = brace_depth.saturating_sub(1);
                current.push(ch);
            } else if ch.is_whitespace() && brace_depth == 0 {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            } else {
                current.push(ch);
            }
        }
        if !current.is_empty() {
            args.push(current);
        }

        expand_args(result, args);
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
                for args in read_file_lines(&arg) {
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
