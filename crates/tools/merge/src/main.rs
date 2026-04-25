enum ArgKind {
    None,
    Input,
    Output,
}

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

    let Some(output) = output else {
        panic!("exactly one `--out` is required");
    };

    windows_metadata::merge()
        .inputs(input)
        .output(&output)
        .merge()
        .unwrap_or_else(|e| panic!("{e}"));

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
