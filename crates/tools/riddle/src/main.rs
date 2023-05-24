use metadata::*;

enum ArgKind {
    None,
    Input,
    Output,
    Filter,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let time = std::time::Instant::now();
    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!(
            r#"Usage: riddle.exe [options...]

Options:
  -in     <path>       Path to files and directories containing .winmd and .idl files
  -out    <path>       Path to .winmd or .idl file to generate
  -filter <namespace>  Namespaces to include or !exclude in output
  -format              Format .idl files only
"#
        );
        return Ok(());
    }

    let mut kind = ArgKind::None;
    let mut output = None;
    let mut input = Vec::<&str>::new();
    let mut include = Vec::<&str>::new();
    let mut exclude = Vec::<&str>::new();
    let mut format = false;

    for arg in &args {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "-in" => kind = ArgKind::Input,
                "-out" => kind = ArgKind::Output,
                "-filter" => kind = ArgKind::Filter,
                "-format" => format = true,
                _ => return Err(Error::new(&format!("invalid option: `{arg}`"))),
            },
            ArgKind::Output => {
                if output.is_none() {
                    output = Some(arg.as_str());
                } else {
                    return Err(Error::new("too many outputs"));
                }
            }
            ArgKind::Input => input.push(arg.as_str()),
            ArgKind::Filter => {
                if let Some(rest) = arg.strip_prefix('!') {
                    exclude.push(rest);
                } else {
                    include.push(arg.as_str());
                }
            }
        }
    }

    if format {
        if output.is_some() || !include.is_empty() || !exclude.is_empty() {
            return Err(Error::new("-format cannot be combined with -output, -include, or -exclude"));
        }

        let input = filter_input(input, &["idl"])?;

        if input.is_empty() {
            return Err(Error::new("no .idl inputs"));
        }

        for path in &input {
            let source = read_to_string(path)?;
            write_to_file(path, writer::format_idl(&source).map_err(|error| error.with_path(path))?)?;
        }

        return Ok(());
    }

    let input = filter_input(input, &["winmd", "idl"])?;

    if input.is_empty() {
        return Err(Error::new("no inputs"));
    }

    let Some(output) = output else {
        return Err(Error::new("no output"));
    };

    let filter = reader::Filter::new(&include, &exclude);
    let module = writer::Module::read(&input, &filter)?;

    module.validate()?;
    module.write(output)?;

    println!("  Finished writing `{}` in {:.2}s", canonicalize(output)?, time.elapsed().as_secs_f32());
    Ok(())
}

fn filter_input(input: Vec<&str>, filter: &[&str]) -> Result<Vec<String>> {
    fn try_push(path: &str, filter: &[&str], results: &mut Vec<String>) -> Result<()> {
        let path = canonicalize(path)?;

        if filter.contains(&extension(&path).1) {
            results.push(path);
        }

        Ok(())
    }

    let mut results = vec![];

    for input in &input {
        let path = std::path::Path::new(input);

        if path.is_dir() {
            for entry in path.read_dir().map_err(|_| Error::new("failed to read directory").with_path(input))?.flatten() {
                let path = entry.path();

                if path.is_file() {
                    try_push(&path.to_string_lossy(), filter, &mut results)?;
                }
            }
        } else {
            try_push(&path.to_string_lossy(), filter, &mut results)?;
        }
    }
    Ok(results)
}
