fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!(
            r#"Usage: riddle.exe [options...]

Options:
  --in  <path>          Path to files and directories containing .winmd and .rdl files
  --out <path>          Path to .winmd or .rdl file to generate
  --filter <namespace>  Namespaces to include or !exclude in output
  --split               Split output across multiple files
"#
        );
    } else {
        let mut kind = ArgKind::None;

        // TODO: use writer if output is .winmd or reader if output is .rdl
        let mut reader = windows_rdl::reader();

        for arg in &args {
            if arg.starts_with('-') {
                kind = ArgKind::None;
            }

            match kind {
                ArgKind::None => match arg.as_str() {
                    "--in" => kind = ArgKind::Input,
                    "--out" => kind = ArgKind::Output,
                    _ => panic!("invalid option `{arg}`"),
                },
                ArgKind::Output => _ = reader.output(arg.as_str()),
                ArgKind::Input => _ = reader.input(arg.as_str()),
            }
        }

        if let Err(error) = reader.write() {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

enum ArgKind {
    None,
    Input,
    Output,
}
