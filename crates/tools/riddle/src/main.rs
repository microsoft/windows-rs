use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use windows_metadata as metadata;
use windows_rdl::{
    Diagnostic, DiagnosticReport, Error, Label, LabelStyle, Reader, Severity, formatter,
};

const HELP: &str = "\
Riddle checks and compiles RDL and validates Windows metadata.

Usage:
  riddle check [options] <input>...
  riddle build [options] <input>... --out <path>
  riddle validate [options] <input>...
  riddle fmt [--check] <input>...

Options:
  -r, --reference <path>  Add a .winmd reference file or directory
      --no-default        Do not use the default Windows metadata references
  -o, --out <path>        Output .winmd path for build
      --check             Check formatting without changing files
  -h, --help              Print help
  -V, --version           Print version

Use - with check, build, or fmt to read RDL from standard input.
";

#[derive(Clone, Copy)]
enum Command {
    Check,
    Build,
    Validate,
    Fmt,
}

struct Options {
    command: Command,
    inputs: Vec<PathBuf>,
    references: Vec<PathBuf>,
    output: Option<PathBuf>,
    reference_default: bool,
    format_check: bool,
    stdin: Option<String>,
}

enum ParseResult {
    Run(Options),
    Help,
    Version,
}

fn main() -> ExitCode {
    match parse(std::env::args().skip(1)) {
        Ok(ParseResult::Help) => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Ok(ParseResult::Version) => {
            println!("riddle {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Ok(ParseResult::Run(mut options)) => {
            if options.inputs.iter().any(|input| input == Path::new("-")) {
                let mut source = String::new();
                if let Err(error) = std::io::stdin().read_to_string(&mut source) {
                    eprintln!("error: failed to read standard input: {error}");
                    return ExitCode::from(1);
                }
                options.stdin = Some(source);
            }

            match options.command {
                Command::Fmt => match format_inputs(&options) {
                    Ok(changed) if options.format_check && changed => ExitCode::from(1),
                    Ok(_) => ExitCode::SUCCESS,
                    Err(error) => {
                        eprint!(
                            "{}",
                            render(error.diagnostic(), None, options.stdin.as_deref())
                        );
                        ExitCode::from(1)
                    }
                },
                Command::Validate => match validate_metadata(&options) {
                    Ok((_, errors)) if errors.is_empty() => ExitCode::SUCCESS,
                    Ok((paths, errors)) => {
                        eprint!("{}", render_metadata_validation(&paths, &errors));
                        ExitCode::from(1)
                    }
                    Err(error) => {
                        eprint!(
                            "{}",
                            render(error.diagnostic(), None, options.stdin.as_deref())
                        );
                        ExitCode::from(1)
                    }
                },
                Command::Check | Command::Build => {
                    let mut reader = Reader::new();
                    configure(&mut reader, &options);
                    match options.command {
                        Command::Check => {
                            let report = reader.check_all();
                            if report.is_success() {
                                ExitCode::SUCCESS
                            } else {
                                eprint!("{}", render_report(&report));
                                ExitCode::from(1)
                            }
                        }
                        Command::Build => {
                            match reader.output(options.output.as_ref().unwrap()).write() {
                                Ok(()) => ExitCode::SUCCESS,
                                Err(error) => {
                                    eprint!(
                                        "{}",
                                        render(error.diagnostic(), None, options.stdin.as_deref())
                                    );
                                    ExitCode::from(1)
                                }
                            }
                        }
                        Command::Validate | Command::Fmt => unreachable!(),
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("error: {error}\n\n{HELP}");
            ExitCode::from(2)
        }
    }
}

fn parse(args: impl Iterator<Item = String>) -> Result<ParseResult, String> {
    let mut args = args.peekable();
    let Some(command) = args.next() else {
        return Ok(ParseResult::Help);
    };
    if command == "-h" || command == "--help" {
        return Ok(ParseResult::Help);
    }
    if command == "-V" || command == "--version" {
        return Ok(ParseResult::Version);
    }

    let command = match command.as_str() {
        "check" => Command::Check,
        "build" => Command::Build,
        "validate" => Command::Validate,
        "fmt" => Command::Fmt,
        _ => return Err(format!("unknown command `{command}`")),
    };

    let mut inputs = vec![];
    let mut references = vec![];
    let mut output = None;
    let mut reference_default = true;
    let mut format_check = false;
    let mut positional = false;

    while let Some(arg) = args.next() {
        if positional {
            inputs.push(PathBuf::from(arg));
            continue;
        }
        match arg.as_str() {
            "--" => positional = true,
            "-h" | "--help" => return Ok(ParseResult::Help),
            "-V" | "--version" => return Ok(ParseResult::Version),
            "--no-default" => reference_default = false,
            "--check" => format_check = true,
            "-r" | "--reference" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("`{arg}` requires a path"))?;
                references.push(PathBuf::from(value));
            }
            "-o" | "--out" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("`{arg}` requires a path"))?;
                if output.replace(PathBuf::from(value)).is_some() {
                    return Err("output may only be specified once".to_string());
                }
            }
            _ if arg.starts_with('-') && arg != "-" => {
                return Err(format!("unknown option `{arg}`"));
            }
            _ => inputs.push(PathBuf::from(arg)),
        }
    }

    if inputs.is_empty() {
        return Err("at least one input is required".to_string());
    }
    if inputs
        .iter()
        .filter(|input| *input == Path::new("-"))
        .count()
        > 1
    {
        return Err("standard input may only be specified once".to_string());
    }

    match command {
        Command::Check | Command::Validate if output.is_some() => {
            return Err("`--out` is only valid with `riddle build`".to_string());
        }
        Command::Build => {
            let Some(path) = output.as_ref() else {
                return Err("`riddle build` requires `--out <path>`".to_string());
            };
            if path
                .extension()
                .is_none_or(|extension| !extension.eq_ignore_ascii_case("winmd"))
            {
                return Err("build output must have a .winmd extension".to_string());
            }
        }
        Command::Fmt => {
            if output.is_some() {
                return Err("`--out` is only valid with `riddle build`".to_string());
            }
            if !references.is_empty() || !reference_default {
                return Err("references are not valid with `riddle fmt`".to_string());
            }
        }
        Command::Check => {}
        Command::Validate => {
            if inputs.iter().any(|input| input == Path::new("-")) {
                return Err("standard input is not supported by `riddle validate`".to_string());
            }
        }
    }
    if format_check && !matches!(command, Command::Fmt) {
        return Err("`--check` is only valid with `riddle fmt`".to_string());
    }

    Ok(ParseResult::Run(Options {
        command,
        inputs,
        references,
        output,
        reference_default,
        format_check,
        stdin: None,
    }))
}

fn validate_metadata(
    options: &Options,
) -> Result<(Vec<PathBuf>, Vec<metadata::validator::ValidationError>), Error> {
    let paths = windows_rdl::expand_input_files(&options.inputs, "winmd")?;
    let files = read_metadata_files(&paths)?;
    let index = metadata::reader::Index::new(files);

    let reference_paths = windows_rdl::expand_input_files(&options.references, "winmd")?;
    let mut references = read_metadata_files(&reference_paths)?;
    if options.reference_default {
        references.extend(
            [windows_default::WINRT, windows_default::WIN32]
                .into_iter()
                .map(|bytes| metadata::reader::File::new(bytes.to_vec()).unwrap()),
        );
    }

    let errors = if references.is_empty() {
        metadata::validator::validate(&index)
    } else {
        let references = metadata::reader::Index::new(references);
        metadata::validator::Validator::new(&index)
            .references(&references)
            .validate()
    };
    Ok((paths, errors))
}

fn read_metadata_files(paths: &[PathBuf]) -> Result<Vec<metadata::reader::File>, Error> {
    paths
        .iter()
        .map(|path| {
            metadata::reader::File::read(path)
                .ok_or_else(|| Error::new("invalid metadata", &path.to_string_lossy(), 0, 0))
        })
        .collect()
}

fn configure(reader: &mut Reader, options: &Options) {
    for input in &options.inputs {
        if input != Path::new("-") {
            reader.input(input);
        }
    }
    if let Some(source) = &options.stdin {
        reader.input_text_named("<stdin>", source);
    }
    reader.references(&options.references);
    if options.reference_default {
        reader.reference_default();
    }
}

fn render_metadata_validation(
    paths: &[PathBuf],
    errors: &[metadata::validator::ValidationError],
) -> String {
    let mut output = String::new();
    for error in errors {
        output.push_str(&format!("error: {}\n", error.message()));
        render_metadata_row(&mut output, " -->", paths, error.row());
        if let Some(related) = error.related() {
            render_metadata_row(&mut output, " :::", paths, related);
        }
    }
    if errors.len() > 1 {
        output.push_str(&format!(
            "error: aborting due to {} previous errors\n",
            errors.len()
        ));
    }
    output
}

fn render_metadata_row(
    output: &mut String,
    marker: &str,
    paths: &[PathBuf],
    row: metadata::reader::RowId,
) {
    let path = paths
        .get(row.file())
        .map_or_else(|| "<metadata>".into(), |path| path.to_string_lossy());
    output.push_str(&format!("{marker} {path}\n"));
    output.push_str(&format!(
        "  = note: metadata row {:?}[{}]\n",
        row.table(),
        row.row() + 1
    ));
}

fn format_inputs(options: &Options) -> Result<bool, Error> {
    let paths: Vec<_> = options
        .inputs
        .iter()
        .filter(|input| *input != Path::new("-"))
        .collect();
    let paths = windows_rdl::expand_input_files(&paths, "rdl")?;
    let mut outputs = vec![];

    for path in paths {
        let name = path.to_string_lossy().into_owned();
        let source = std::fs::read_to_string(&path)
            .map_err(|_| Error::new("failed to read input file", &name, 0, 0))?;
        let formatted = formatter::format_named(&name, &source)?;
        outputs.push((Some(path), name, source, formatted));
    }
    if let Some(source) = &options.stdin {
        let formatted = formatter::format_named("<stdin>", source)?;
        outputs.push((None, "<stdin>".to_string(), source.clone(), formatted));
    }

    let changed = outputs
        .iter()
        .any(|(_, _, source, formatted)| source != formatted);
    if options.format_check {
        for (_, name, source, formatted) in &outputs {
            if source != formatted {
                eprintln!("needs formatting: {name}");
            }
        }
    } else {
        for (path, _, source, formatted) in outputs {
            if let Some(path) = path {
                if source != formatted {
                    windows_rdl::write_to_file(path, formatted)?;
                }
            } else {
                print!("{formatted}");
            }
        }
    }
    Ok(changed)
}

fn render_report(report: &DiagnosticReport) -> String {
    let mut output = String::new();
    for diagnostic in report.diagnostics() {
        output.push_str(&render(diagnostic, Some(report), None));
    }
    let errors = report
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.severity == Severity::Error)
        .count();
    if errors > 1 {
        output.push_str(&format!(
            "error: aborting due to {errors} previous errors\n"
        ));
    }
    output
}

fn render(
    diagnostic: &Diagnostic,
    report: Option<&DiagnosticReport>,
    stdin: Option<&str>,
) -> String {
    let severity = match diagnostic.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    };
    let code = diagnostic
        .code
        .as_deref()
        .map_or_else(String::new, |code| format!("[{code}]"));
    let mut output = format!("{severity}{code}: {}\n", diagnostic.message);
    let labels = if diagnostic.labels.is_empty() && !diagnostic.file_name.is_empty() {
        vec![Label::primary(
            &diagnostic.file_name,
            diagnostic.line,
            diagnostic.column,
        )]
    } else {
        diagnostic.labels.clone()
    };
    let mut sources = HashMap::new();

    for label in labels {
        render_label(&mut output, &label, report, stdin, &mut sources);
    }
    for note in &diagnostic.notes {
        output.push_str(&format!("  = note: {note}\n"));
    }
    for help in &diagnostic.help {
        output.push_str(&format!("  = help: {help}\n"));
    }
    output
}

fn render_label(
    output: &mut String,
    label: &Label,
    report: Option<&DiagnosticReport>,
    stdin: Option<&str>,
    sources: &mut HashMap<String, Option<String>>,
) {
    let line = label.start.line;
    let column = label.start.column;
    if line == 0 {
        output.push_str(&format!(" --> {}\n", label.source));
        return;
    }

    output.push_str(&format!(" --> {}:{}:{}\n", label.source, line, column + 1));
    let source = sources
        .entry(label.source.clone())
        .or_insert_with(|| {
            if let Some(source) = report.and_then(|report| report.source(&label.source)) {
                Some(source.to_string())
            } else if label.source == "<stdin>" {
                stdin.map(str::to_string)
            } else {
                std::fs::read_to_string(&label.source).ok()
            }
        })
        .as_deref();
    let Some(source_line) = source.and_then(|source| source.lines().nth(line - 1)) else {
        return;
    };

    let width = line.to_string().len();
    output.push_str(&format!("{:width$} |\n", "", width = width));
    output.push_str(&format!("{line:>width$} | {source_line}\n"));
    let end_column = if label.end.line == line {
        label.end.column.max(column + 1)
    } else {
        column + 1
    };
    let marker = match label.style {
        LabelStyle::Primary => '^',
        LabelStyle::Secondary => '-',
    };
    let underline: String = std::iter::repeat_n(marker, end_column - column).collect();
    output.push_str(&format!(
        "{:width$} | {}{}",
        "",
        " ".repeat(column),
        underline,
        width = width
    ));
    if !label.message.is_empty() {
        output.push(' ');
        output.push_str(&label.message);
    }
    output.push('\n');
}
