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

        let mut input = vec![];
        let mut output = String::new();
        let mut filter = vec![];
        let mut split = false;

        for arg in &args {
            if arg.starts_with('-') {
                kind = ArgKind::None;
            }

            match kind {
                ArgKind::None => match arg.as_str() {
                    "--in" => kind = ArgKind::Input,
                    "--out" => kind = ArgKind::Output,
                    "--filter" => kind = ArgKind::Filter,
                    "--split" => split = true,
                    _ => panic!("invalid option `{arg}`"),
                },
                ArgKind::Output => output = arg.to_string(),
                ArgKind::Input => input.push(arg.to_string()),
                ArgKind::Filter => filter.push(arg.to_string()),
            }
        }

        let path = std::path::Path::new(&output);

        let result = if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
        {
            windows_rdl::reader().inputs(&input).output(&output).write()
        } else {
            windows_rdl::writer()
                .inputs(&input)
                .output(&output)
                .filters(&filter)
                .split(split)
                .write()
        };

        if let Err(error) = result {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
}
