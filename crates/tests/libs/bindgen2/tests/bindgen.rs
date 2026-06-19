include!("../../../../../target/test_bindgen2/generated_tests.rs");

fn run(name: &str) {
    let input_path = format!("input/{name}.rdl");
    let expected_path = format!("expected/{name}.rs");
    let scratch = format!("../../../../target/test_bindgen2/{name}");
    std::fs::create_dir_all(&scratch).unwrap();

    // Extract args from `//!` comment lines at the top.
    let contents = std::fs::read_to_string(&input_path).unwrap();
    let mut args = Vec::new();

    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("//!") {
            for arg in rest.split_whitespace() {
                args.push(arg.to_string());
            }
        }
    }

    let winmd = format!("{scratch}/out.winmd");

    windows_rdl::reader()
        .input(&input_path)
        .output(&winmd)
        .write()
        .unwrap();

    let out_rs = format!("{scratch}/out.rs");
    let mut cli: Vec<String> = vec!["--in".into(), winmd];
    cli.extend(args);
    cli.push("--out".into());
    cli.push(out_rs.clone());

    windows_bindgen::bindgen(cli);

    let actual = std::fs::read_to_string(&out_rs).unwrap();
    std::fs::write(&expected_path, &actual).unwrap();
}
