use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use windows_rdl::{Diagnostic, Label, LabelStyle, Reader, Severity};

const HELP: &str = "\
Riddle checks and compiles RDL API descriptions.

Usage:
  riddle check [options] <input>...
  riddle build [options] <input>... --out <path>

Options:
  -r, --reference <path>  Add a .winmd reference file or directory
      --no-default        Do not use the default Windows metadata references
  -o, --out <path>        Output .winmd path for build
  -h, --help              Print help
  -V, --version           Print version

Use - as an input to read RDL from standard input.
";

enum Command {
    Check,
    Build,
}

struct Options {
    command: Command,
    inputs: Vec<PathBuf>,
    references: Vec<PathBuf>,
    output: Option<PathBuf>,
    reference_default: bool,
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

            let mut reader = Reader::new();
            configure(&mut reader, &options);
            let result = match options.command {
                Command::Check => reader.check(),
                Command::Build => reader.output(options.output.as_ref().unwrap()).write(),
            };

            match result {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprint!("{}", render(error.diagnostic(), options.stdin.as_deref()));
                    ExitCode::from(1)
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
        _ => return Err(format!("unknown command `{command}`")),
    };

    let mut inputs = vec![];
    let mut references = vec![];
    let mut output = None;
    let mut reference_default = true;
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
        Command::Check if output.is_some() => {
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
        Command::Check => {}
    }

    Ok(ParseResult::Run(Options {
        command,
        inputs,
        references,
        output,
        reference_default,
        stdin: None,
    }))
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

fn render(diagnostic: &Diagnostic, stdin: Option<&str>) -> String {
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
        render_label(&mut output, &label, stdin, &mut sources);
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
            if label.source == "<stdin>" {
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
