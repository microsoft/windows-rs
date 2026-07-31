use super::*;

/// Generates bindings using command-line-style arguments.
///
/// ```rust,no_run
/// let args = [
///     "--out",
///     "src/bindings.rs",
///     "--filter",
///     "GetTickCount",
/// ];
///
/// windows_bindgen::bindgen(args);
/// ```
///
/// Supported arguments:
///
/// - `--in`: Metadata files or directories.
/// - `--out`: Generated Rust file.
/// - `--filter`: Included or excluded APIs.
/// - `--rustfmt`: Rust formatter override.
/// - `--derive`: Extra derived traits.
/// - `--flat`: Omits namespace modules.
/// - `--sys`: Generates raw bindings that depend only on `windows-link`.
/// - `--extern`: Uses extern declarations with `--sys`.
/// - `--minimal`: Omits class wrappers, inherited forwarders, and handle wrappers.
/// - `--implement`: Emits implementation traits for selected WinRT interfaces.
/// - `--dead-code`: Emits `pub(crate)` items for dead-code analysis.
/// - `--etc`: Reads arguments from response files.
/// # `--out`
///
/// Exactly one `--out` argument is required.
///
/// # `--filter`
///
/// Filters select APIs to include. Prefix a filter with `!` to exclude it.
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
/// Use a response file for longer filter lists:
///
/// ```text
/// --etc path/to/file.txt
/// ```
///
/// The in-repo crates use this convention; see the filter `.txt` files in
/// `crates/tools/bindings/src`.
///
/// ## Method-level filtering
///
/// Filters can target methods, properties, and events with `Namespace.Type::Member`.
/// `Property.Name` and `Event.Name` select accessor pairs.
///
/// A bare type projects the full type. `Type::{}` emits a name-only shell, while
/// `Type::Member` and `Type::{a, b}` select members.
///
/// ```text
/// --filter Windows.UI.Xaml.Controls.Button
///          Windows.UI.Xaml.Controls.TextBlock::put_Text
///          Windows.UI.Xaml.Controls.TextBlock::Property.FontSize
///          Windows.UI.Xaml.UIElement::Event.PointerPressed
/// ```
///
#[track_caller]
pub fn bindgen<I, S>(args: I)
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
                "--derive" => kind = ArgKind::Derive,
                "--flat" => {
                    builder.flat();
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
        }
    }

    builder.write();
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    Rustfmt,
    Derive,
    Implement,
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
