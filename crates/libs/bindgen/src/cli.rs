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
/// - `--rustfmt`: Comma-separated `rustfmt --config` options, such as `max_width=120`.
/// - `--derive`: Extra derived traits.
/// - `--modules`: Nests items in one module per namespace.
/// - `--package`: Generates a package with one file per namespace.
/// - `--sys`: Generates raw bindings that depend only on `windows-link`.
/// - `--extern`: Uses extern declarations with `--sys`.
/// - `--full`: Adds class wrappers, inherited forwarders, handle wrappers, and implementation traits
///   for interfaces not marked `ExclusiveTo`.
/// - `--implement`: Emits implementation traits for selected native and WinRT interfaces.
/// - `--compose`: Selects WinRT class composition targets.
/// - `--dead-code`: Emits `pub(crate)` items for dead-code analysis.
/// - `--etc`: Reads arguments from command files.
/// - `--filter-file`: Reads filters from text files.
///
/// # `--out`
///
/// Defaults to `bindings.rs` in the runtime `OUT_DIR` directory. An explicit path is required when
/// `OUT_DIR` is unset or empty, and always with `--package`. Bindgen does not emit Cargo
/// dependency-tracking directives.
///
/// # `--filter`
///
/// Filters select functions, constants, types, namespaces, or type members.
/// Signature dependencies are included automatically.
///
/// The bundled Win32 metadata uses `Windows.Win32`, not the subsystem namespaces used by the
/// `windows` and `windows-sys` package features. Bare names and qualified names both work:
///
/// ```text
/// --filter GetTickCount HWND
///
/// --filter Windows.Win32.GetTickCount
///
/// --filter Windows::Win32::{GetTickCount, GetTickCount64}
///
/// --filter Windows.Foundation
/// ```
///
/// Dots and `::` both separate path segments. Namespace filters include child namespaces;
/// prefix a type or namespace filter with `!` to exclude it. More specific rules take precedence;
/// exclusions win ties regardless of argument order.
///
/// Use a filter file for longer filter lists:
///
/// ```text
/// --filter-file path/to/filter.txt
/// ```
///
/// Filter and command files separate arguments with whitespace. Filter expressions keep `{...}`
/// groups together across lines and allow trailing commas:
///
/// ```text
/// ID2D1RenderTarget::{
///     BeginDraw,
///     EndDraw,
///     Clear, // Reset the surface
/// }
/// ```
///
/// Lines beginning with `//` after optional whitespace are comments. Inline comments start with
/// a separate `//` token, followed by whitespace or the end of the line. This leaves UNC paths
/// such as `--in //server/share/input.winmd` intact. In command files, grouping applies only to
/// `--filter` arguments; braces in paths are literal.
///
/// The in-repo crates use this convention; see the filter `.txt` files in
/// `crates/tools/bindings/src`.
///
/// ## Method-level filtering
///
/// Filters can target methods, properties, and events with `Namespace.Type::Member`.
/// Use the member's metadata name. A property or event name selects its accessor pair.
///
/// A bare type projects the full type. `Type::{}` emits a name-only shell, while
/// `Type::Member` and `Type::{a, b}` select members.
/// Explicit member or enum-variant selections narrow a whole-type selection regardless of rule
/// order. Excluded members and variants win over included ones.
///
/// ```text
/// --filter Windows.Storage.Streams.IDataReader::ReadByte
///
/// --filter Windows.Storage.Streams.IDataReader::{ReadByte, ReadUInt32}
///
/// --filter Windows.Storage.Streams.IDataReader::ByteOrder
///
/// --filter Windows.Storage.Streams.IDataReader::{}
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
    let mut output_required = false;
    let mut implement = None::<Vec<String>>;
    let mut compose = None::<Vec<String>>;

    for arg in args {
        if arg.starts_with('-') {
            assert!(!output_required, "`--out` requires a path");
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => {
                    kind = ArgKind::Output;
                    output_required = true;
                }
                "--filter" => kind = ArgKind::Filter,
                "--filter-file" => kind = ArgKind::FilterFile,
                "--rustfmt" => kind = ArgKind::Rustfmt,
                "--derive" => kind = ArgKind::Derive,
                "--modules" => {
                    builder.modules();
                }
                "--package" => {
                    builder.package();
                }
                "--sys" => {
                    builder.sys();
                }
                "--full" => {
                    builder.full();
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
                "--compose" => {
                    compose.get_or_insert_with(Vec::new);
                    kind = ArgKind::Compose;
                }
                _ => panic!("invalid option `{arg}`"),
            },
            ArgKind::Output => {
                assert!(!has_output, "exactly one `--out` is required");
                builder.output(arg);
                has_output = true;
                output_required = false;
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
            ArgKind::Compose => {
                compose.as_mut().unwrap().push(arg.clone());
            }
            ArgKind::Rustfmt => {
                builder.rustfmt(&arg);
            }
        }
    }

    assert!(!output_required, "`--out` requires a path");

    if let Some(implement) = implement {
        if implement.is_empty() {
            builder.implement_all();
        } else {
            builder.implements(implement);
        }
    }
    if let Some(compose) = compose {
        assert!(!compose.is_empty(), "`--compose` requires a class name");
        builder.composes(compose);
    }

    builder.write();
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
    Compose,
}

#[track_caller]
fn expand_args<I, S>(args: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    #[track_caller]
    fn expand<I, S>(result: &mut Vec<String>, filter_args: &mut bool, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut command_files = false;

        for arg in args.into_iter().map(|arg| arg.as_ref().to_string()) {
            if arg.starts_with('-') {
                command_files = false;
            }

            if command_files {
                let tokens = read_tokens(arg, *filter_args);
                expand(result, filter_args, tokens);
            } else if arg == "--etc" {
                command_files = true;
            } else {
                if arg.starts_with('-') {
                    *filter_args = arg == "--filter";
                }
                result.push(arg);
            }
        }
    }

    let mut result = Vec::new();
    expand(&mut result, &mut false, args);
    result
}

#[track_caller]
pub(super) fn read_tokens(input: impl AsRef<Path>, mut filter_args: bool) -> Vec<String> {
    let input = input.as_ref();
    let mut result = Vec::new();
    let mut current = String::new();
    let mut brace_depth = 0u32;
    let mut group_line = 0;

    for (index, line) in read_file_lines(input).iter().enumerate() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        let line_number = index + 1;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '/'
                && chars.peek() == Some(&'/')
                && (current.is_empty() || current.ends_with(char::is_whitespace))
                && chars.clone().nth(1).is_none_or(char::is_whitespace)
            {
                break;
            }
            if filter_args && ch == '{' {
                if brace_depth == 0 {
                    group_line = line_number;
                }
                brace_depth += 1;
                current.push(ch);
            } else if filter_args && ch == '}' {
                assert!(
                    brace_depth > 0,
                    "{}:{line_number}: unbalanced `}}`",
                    input.display()
                );
                brace_depth -= 1;
                current.push(ch);
            } else if ch.is_whitespace() && brace_depth == 0 {
                if !current.is_empty() {
                    if current.starts_with('-') {
                        filter_args = current == "--filter";
                    }
                    result.push(std::mem::take(&mut current));
                }
            } else {
                current.push(ch);
            }
        }
        if brace_depth > 0 {
            current.push('\n');
        } else if !current.is_empty() {
            if current.starts_with('-') {
                filter_args = current == "--filter";
            }
            result.push(std::mem::take(&mut current));
        }
    }

    assert!(
        brace_depth == 0,
        "{}:{group_line}: unbalanced `{{`",
        input.display()
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_file_paths_and_comments() {
        let path = std::env::temp_dir().join(format!(
            "windows-bindgen-command-tokens-{}.txt",
            std::process::id()
        ));
        std::fs::write(
            &path,
            r"--in C:\metadata\input.winmd relative//input.winmd \\server\share\input.winmd
--out generated/{bindings}.rs // output }
--filter Type::{Method} --in metadata/input{.winmd // literal brace
",
        )
        .unwrap();
        let tokens = read_tokens(&path, false);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(
            tokens,
            [
                "--in",
                r"C:\metadata\input.winmd",
                "relative//input.winmd",
                r"\\server\share\input.winmd",
                "--out",
                "generated/{bindings}.rs",
                "--filter",
                "Type::{Method}",
                "--in",
                "metadata/input{.winmd",
            ]
        );
    }

    #[test]
    fn command_file_unc_paths_are_not_comments() {
        let path = std::env::temp_dir().join(format!(
            "windows-bindgen-command-unc-{}.txt",
            std::process::id()
        ));
        std::fs::write(
            &path,
            "//Whole-line comment\n\
             --in //server/share/input.winmd // metadata\n\
             --out //server/share/output.rs // generated output\n\
             --filter-file //server/share/filters.txt //\n\
             --etc //server/share/commands.txt // nested commands\n\
             --filter Type::{} //\tfilter\n",
        )
        .unwrap();
        let tokens = read_tokens(&path, false);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(
            tokens,
            [
                "--in",
                "//server/share/input.winmd",
                "--out",
                "//server/share/output.rs",
                "--filter-file",
                "//server/share/filters.txt",
                "--etc",
                "//server/share/commands.txt",
                "--filter",
                "Type::{}",
            ]
        );
    }

    #[test]
    fn command_files_inherit_argument_kind() {
        let directory = std::env::temp_dir().join(format!(
            "windows-bindgen-command-context-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).unwrap();
        let selector = directory.join("selector.txt");
        let arguments = directory.join("arguments.txt");
        let outer = directory.join("outer.txt");
        std::fs::write(
            &outer,
            format!("--etc {} {}", selector.display(), arguments.display()),
        )
        .unwrap();

        for (option, value) in [
            ("--filter", "Test::{\n    CONSTANT,\n}"),
            ("--out", "generated/bindings}.rs"),
            ("--in", "metadata/input{.winmd"),
        ] {
            std::fs::write(&selector, option).unwrap();
            std::fs::write(&arguments, value).unwrap();
            for args in [
                vec![option, "--etc", arguments.to_str().unwrap()],
                vec![
                    "--etc",
                    selector.to_str().unwrap(),
                    arguments.to_str().unwrap(),
                ],
                vec!["--etc", outer.to_str().unwrap()],
            ] {
                assert_eq!(expand_args(args), [option, value]);
            }
        }

        for path in [selector, arguments, outer] {
            std::fs::remove_file(path).unwrap();
        }
        std::fs::remove_dir(directory).unwrap();
    }
}
