mod composition;
mod generic_interfaces;
mod module_attributes;
mod nested_module;
mod nested_struct;
mod params;
mod r#struct;
mod win32_struct;
mod winrt_struct;

pub fn run_riddle(name: &str, dialect: &str, etc: &[&str]) -> Vec<windows_metadata::File> {
    let rdl = format!("tests/{name}.rdl");
    let winmd = format!("tests/{name}.winmd");
    let rs = format!("src/{name}.rs");

    let before = std::fs::read_to_string(&rdl)
        .unwrap_or_else(|e| panic!("Failed to read input: {rdl} : {e:?}"));

    // Convert .rdl to .winmd
    _ = std::fs::remove_file(&winmd);
    let command: Vec<String> = vec![
        "--in".to_owned(),
        rdl.to_owned(),
        "--out".to_owned(),
        winmd.to_owned(),
        "--filter".to_owned(),
        "Test".to_owned(),
    ];
    run_one_bindgen(&command);

    // Convert .winmd back to .rdl
    std::fs::remove_file(&rdl).unwrap_or_else(|e| panic!("Failed to delete output: {rdl} : {e:?}"));
    let mut command: Vec<String> = vec![
        "--in".to_owned(),
        winmd.to_owned(),
        "--out".to_owned(),
        rdl.to_owned(),
        "--filter".to_owned(),
        "Test".to_owned(),
        "--config".to_owned(),
    ];
    command.push(format!("type={dialect}"));
    run_one_bindgen(&command);

    // Check that .rdl is unchanged
    let after = std::fs::read_to_string(&rdl)
        .unwrap_or_else(|e| panic!("Failed to read output: {rdl} : {e:?}"));
    assert_eq!(before, after, "no equal {}", rdl);

    // Convert .rdl to .rs
    std::fs::remove_file(&rs).unwrap_or_else(|e| panic!("Failed to delete output: {rs} : {e:?}"));

    let mut command: Vec<String> = vec![
        "--in".to_owned(),
        rdl.to_owned(),
        "--out".to_owned(),
        rs.to_owned(),
        "--filter".to_owned(),
        "Test".to_owned(),
        "--config".to_owned(),
        "no-bindgen-comment".to_owned(),
    ];
    command.extend(etc.iter().map(|&s| s.to_owned()));
    run_one_bindgen(&command);

    // Return winmd file for validation
    let mut files = helpers::default_metadata();
    let winmd_bytes =
        std::fs::read(&winmd).unwrap_or_else(|e| panic!("Failed to read winmd: {winmd} : {e:?}"));
    files.push(
        windows_metadata::File::new(winmd_bytes)
            .unwrap_or_else(|| panic!("failed to parse winmd: {winmd}")),
    );
    files
}

fn run_one_bindgen(args: &[String]) -> String {
    match windows_bindgen::bindgen(args) {
        Ok(s) => s,
        Err(e) => {
            panic!(
                "Failed to run bindgen: {e:?}\nArgs: riddle.exe {args}",
                args = args.join(" ")
            );
        }
    }
}
