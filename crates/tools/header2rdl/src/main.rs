fn main() {
    if let Err(e) = tool_header2rdl::run(std::env::args().skip(1)) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
