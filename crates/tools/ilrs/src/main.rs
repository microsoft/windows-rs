use std::env::*;

fn main() {
    let mut gen = metagen::Gen::new();
    let mut kind = ArgKind::None;

    for arg in args() {
        match kind {
            ArgKind::None => match arg.as_str() {
                "-s" => kind = ArgKind::Source,
                "-i" => kind = ArgKind::Input,
                "-r" => kind = ArgKind::Reference,
                "-o" => kind = ArgKind::Output,
                _ => return print_help(),
            },
            ArgKind::Source => gen.sources.push(arg),
            ArgKind::Input => gen.inputs.push(arg),
            ArgKind::Reference => gen.references.push(arg),
            ArgKind::Output => {
                if gen.output.is_empty() {
                    gen.output = arg;
                } else {
                    return print_help();
                }
            }
        }
    }

    if (gen.sources.is_empty() || gen.inputs.is_empty()) || gen.output.is_empty() {
        return print_help();
    }

    let _ = metagen::gen(&gen);
    // TODO: print error report
}

fn print_help() {
    println!("help");
}

enum ArgKind {
    None,
    Source,
    Input,
    Reference,
    Output,
}
