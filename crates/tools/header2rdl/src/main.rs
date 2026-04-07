fn main() {
    if let Err(e) = cli_run(std::env::args().skip(1)) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn cli_run(args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut c = tool_header2rdl::converter();
    let args: Vec<String> = args.collect();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--namespace" => {
                i += 1;
                c.namespace(args.get(i).ok_or("expected value for --namespace")?);
            }
            "--library" => {
                i += 1;
                c.library(args.get(i).ok_or("expected value for --library")?);
            }
            "--cpp" => {
                c.cpp(true);
            }
            "--include" => {
                i += 1;
                c.include(args.get(i).ok_or("expected value for --include")?);
            }
            "--define" => {
                i += 1;
                c.define(args.get(i).ok_or("expected value for --define")?);
            }
            "--arch" => {
                i += 1;
                c.arch(args.get(i).ok_or("expected value for --arch")?);
            }
            "--output" => {
                i += 1;
                c.output(args.get(i).ok_or("expected value for --output")?);
            }
            "--split" => {
                c.split(true);
            }
            arg if !arg.starts_with('-') => {
                c.input(arg);
            }
            arg => return Err(format!("unknown argument: {arg}")),
        }
        i += 1;
    }

    c.write()
}
