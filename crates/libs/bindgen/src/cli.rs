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
/// - `--filter-file`: Reads filters from text files.
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
/// Use a filter file for longer filter lists:
///
/// ```text
/// --filter-file path/to/filter.txt
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
    let mut builder = Bindgen::new();
    let mut kind = ArgKind::None;
    let mut has_output = false;
    let mut implement = None::<Vec<String>>;

    for arg in args.into_iter().map(|arg| arg.as_ref().to_string()) {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => kind = ArgKind::Output,
                "--filter" => kind = ArgKind::Filter,
                "--filter-file" => kind = ArgKind::FilterFile,
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
                    implement.get_or_insert_with(Vec::new);
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
                if arg == "default" {
                    builder.input_default();
                } else {
                    builder.input(arg);
                }
            }
            ArgKind::Filter => {
                builder.filter(&arg);
            }
            ArgKind::FilterFile => {
                builder.filter_file(&arg);
            }
            ArgKind::Derive => {
                builder.derive(&arg);
            }
            ArgKind::Implement => {
                implement.as_mut().unwrap().push(arg.clone());
            }
            ArgKind::Rustfmt => {
                builder.rustfmt(&arg);
            }
        }
    }

    if let Some(implement) = implement {
        if implement.is_empty() {
            builder.implement_all();
        } else {
            builder.implements(implement);
        }
    }

    builder.write();
}

/// Generates bindings using commands from a text file.
///
/// Blank lines and lines beginning with `//` are ignored. Use `--filter-file` within the command
/// file to load filter-only files.
#[track_caller]
pub fn bindgen_file(input: impl AsRef<Path>) {
    bindgen(read_tokens(input));
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
    FilterFile,
    Rustfmt,
    Derive,
    Implement,
}

#[track_caller]
pub(super) fn read_tokens(input: impl AsRef<Path>) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_file_lines(input) {
        if line.trim_start().starts_with("//") {
            continue;
        }

        // Split on whitespace but keep `{...}` groups together so that
        // `Type::{a, b}` is not split across multiple filters.
        let mut current = String::new();
        let mut brace_depth = 0u32;

        for ch in line.chars() {
            if ch == '{' {
                brace_depth += 1;
                current.push(ch);
            } else if ch == '}' {
                brace_depth = brace_depth.saturating_sub(1);
                current.push(ch);
            } else if ch.is_whitespace() && brace_depth == 0 {
                if !current.is_empty() {
                    result.push(std::mem::take(&mut current));
                }
            } else {
                current.push(ch);
            }
        }
        if !current.is_empty() {
            result.push(current);
        }
    }

    result
}
