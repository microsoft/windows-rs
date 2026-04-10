fn main() {
    if let Err(e) = cli_run(std::env::args().skip(1)) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn cli_run(args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut c = tool_header2rdl::converter();
    let args: Vec<String> = args.collect();
    let mut idx = 0;

    while idx < args.len() {
        match args[idx].as_str() {
            "--namespace" => {
                idx += 1;
                c.namespace(args.get(idx).ok_or("expected value for --namespace")?);
            }
            "--library" => {
                idx += 1;
                c.library(args.get(idx).ok_or("expected value for --library")?);
            }
            "--cpp" => {
                c.cpp(true);
            }
            "--include" => {
                idx += 1;
                c.include(args.get(idx).ok_or("expected value for --include")?);
            }
            "--system-include" => {
                idx += 1;
                c.system_include(args.get(idx).ok_or("expected value for --system-include")?);
            }
            "--define" => {
                idx += 1;
                let d = args.get(idx).ok_or("expected value for --define")?;
                if let Some(eq) = d.find('=') {
                    c.define(&d[..eq], Some(&d[eq + 1..]));
                } else {
                    c.define(d, None);
                }
            }
            "--arch" => {
                idx += 1;
                c.arch(args.get(idx).ok_or("expected value for --arch")?);
            }
            "--output" => {
                idx += 1;
                c.output(args.get(idx).ok_or("expected value for --output")?);
            }
            "--split" => {
                c.split(true);
            }
            arg if !arg.starts_with('-') => {
                c.file(arg);
            }
            arg => return Err(format!("unknown argument: {arg}")),
        }
        idx += 1;
    }

    c.write()
}
