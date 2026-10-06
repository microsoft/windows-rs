fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    let Some(separator) = arguments.iter().position(|argument| argument == "--") else {
        return Err("usage: inspect ROOT HEADER [HEADER ...] -- CLANG_ARGUMENTS".into());
    };
    if separator < 2 {
        return Err("inspection requires a root name and at least one header".into());
    }
    helpers::ensure_libclang();
    let inputs = arguments[1..separator]
        .iter()
        .map(|path| {
            std::fs::read_to_string(path).map(|source| windows_clang2::Input::new(path, source))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let args: Vec<_> = arguments[separator + 1..]
        .iter()
        .map(String::as_str)
        .collect();
    let snapshot = windows_clang2::capture(inputs, &args, &[&arguments[0]])?;
    print!("{}", snapshot.dump());
    for diagnostic in snapshot.diagnostics() {
        eprintln!("{diagnostic}");
    }
    println!("{:#?}", snapshot.validate()?);
    Ok(())
}
