use windows_ecma335::*;

enum ArgKind {
    None,
    Input,
    Output,
}

// TODO: maybe offer an in-memory merge option that returns a reader::File optimized for cached reads
// that this function can use but more importantly can be used by bindgen for simpler and faster lookups:
// - it stores all TypeDefs rather than TypeRefs - maybe although I'm not sure this is needed.
// - it sorts the TypeDef table so that TypeRefs can be resolved if they are from the same file.

fn main() {
    let time = std::time::Instant::now();
    let mut output = None;
    let mut input = vec![];
    let mut kind = ArgKind::None;

    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            kind = ArgKind::None;
        }

        match kind {
            ArgKind::None => match arg.as_str() {
                "--in" => kind = ArgKind::Input,
                "--out" => kind = ArgKind::Output,
                _ => panic!("invalid option `{arg}`"),
            },
            ArgKind::Output => {
                if output.is_none() {
                    output = Some(arg.to_string());
                } else {
                    panic!("exactly one `--out` is required");
                }
            }
            ArgKind::Input => input.push(arg.to_string()),
        }
    }

    let input = expand_input(input);

    let Some(output) = output else {
        panic!("exactly one `--out` is required");
    };

    let output = std::path::Path::new(&output);
    merge(output, &input);
    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

// TODO: move into merge library
fn expand_input(input: Vec<String>) -> Vec<std::path::PathBuf> {
    let mut result = vec![];

    for input in input {
        let path = std::path::Path::new(&input);

        if path.is_dir() {
            let prev_len = result.len();

            for path in path
                .read_dir()
                .unwrap_or_else(|_| panic!("failed to read directory `{input}`"))
                .flatten()
                .map(|entry| entry.path())
            {
                if path.is_file()
                    && path
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
                {
                    result.push(path.to_path_buf());
                }
            }

            if result.len() == prev_len {
                panic!("failed to find .winmd files in directory `{input}`");
            }
        } else {
            result.push(path.to_path_buf());
        }
    }

    result
}
