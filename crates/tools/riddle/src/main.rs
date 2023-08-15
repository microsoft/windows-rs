fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!(
            r#"Usage: riddle.exe [options...]

Options:
  --in  <path>          Path to files and directories containing .winmd and .rdl files
  --out <path>          Path to .winmd, .rdl, or .rs file to generate
  --filter <namespace>  Namespaces to include or !exclude in output
  --config <key=value>  Override a configuration value
  --format              Format .rdl files only
  --etc <path>          File containing command line options
"#
        );
    } else {
        match windows_bindgen::bindgen(args) {
            Ok(ok) => println!("{}", ok),
            Err(error) => {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }
}
