mod syntax;
use metadata::writer;
use std::io::Read;
use syn::*;
use syntax::*;

fn main() {
    if let Err(message) = run() {
        eprintln!("error: {message}");
        std::process::exit(1);
    }
}

type ToolResult = std::result::Result<(), String>;

fn run() -> ToolResult {
    let mut kind = ArgKind::None;
    let mut merge = Vec::<String>::new();
    let mut input = Vec::<String>::new();
    let mut reference = Vec::<String>::new();
    let mut output = String::new();
    let mut winrt = true;

    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }
        match kind {
            ArgKind::None => match arg.as_str() {
                "-merge" => kind = ArgKind::Merge,
                "-input" => kind = ArgKind::Input,
                "-ref" => kind = ArgKind::Reference,
                "-output" => kind = ArgKind::Output,
                "-win32" => {
                    winrt = false;
                    kind = ArgKind::None;
                }
                _ => print_help()?,
            },
            ArgKind::Merge => merge.push(arg),
            ArgKind::Input => input.push(arg),
            ArgKind::Reference => reference.push(arg),
            ArgKind::Output => {
                if output.is_empty() {
                    output = arg;
                } else {
                    print_help()?;
                }
            }
        }
    }

    if merge.len() + input.len() == 0 || output.is_empty() {
        print_help()?;
    }

    let mut items = Vec::new();

    // for merge in merge {
    //     // TODO: write the types in the winmd into `items`
    // }

    for filename in &input {
        let mut file = std::fs::File::open(filename).map_err(|_| format!("failed to open `{filename}`"))?;
        let mut source = String::new();
        file.read_to_string(&mut source).map_err(|_| format!("failed to read `{filename}`"))?;

        if let Err(error) = parse_str::<Module>(&source).and_then(|module| module.to_writer(&module.name.to_string(), &mut items)) {
            let start = error.span().start();
            let filename = std::fs::canonicalize(filename).map_err(|_| format!("failed to canonicalize `{filename}`"))?;
            return Err(format!("{error}\n  --> {}:{:?}:{:?} ", filename.to_string_lossy().trim_start_matches(r#"\\?\"#), start.line, start.column));
        }
    }

    let buffer = writer::write("test", winrt, &items, &[]);
    std::fs::write(&output, buffer).map_err(|_| format!("failed to write `{output}`"))
}

fn print_help() -> ToolResult {
    Err(r#"riddle.exe [options...]

options:
  -input  <path>  Path to source file with type definitions to parse
  -merge  <path>  Path to file or directory containing .winmd files to merge
  -ref    <path>  Path to file or directory containing .winmd files to reference
  -output <path>  Path to .winmd file to generate
  -win32          Kind of .winmd to generate; default is WinRT

examples:
  riddle -input first.rs second.rs -output out.winmd -ref local
  riddle -merge first.winmd second.winmd -output out.winmd
"#
    .to_string())
}

enum ArgKind {
    None,
    Merge,
    Input,
    Reference,
    Output,
}
