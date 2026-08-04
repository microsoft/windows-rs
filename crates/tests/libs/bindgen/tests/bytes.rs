#[test]
fn bindgen_accepts_metadata_bytes() {
    let temp = std::env::temp_dir();
    let winmd = temp.join("windows_bindgen_bytes.winmd");
    let output = temp.join("windows_bindgen_bytes.rs");
    let filters = temp.join("windows_bindgen_bytes.txt");

    windows_rdl::reader()
        .input_texts([r#"
#[win32]
mod Test {
    #[library("test.dll")]
    extern fn Function() -> u32;
}
"#])
        .output(&winmd)
        .write()
        .unwrap();

    let bytes = std::fs::read(winmd).unwrap();
    std::fs::write(&filters, "  // comment\nTest\n").unwrap();
    windows_bindgen::builder()
        .input_byte_sets([bytes])
        .output(&output)
        .filter_files([filters])
        .flat()
        .write();

    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .contains("fn Function")
    );
}

#[test]
fn bindgen_accepts_command_files() {
    let temp = std::env::temp_dir();
    let winmd = temp.join("windows_bindgen_commands.winmd");
    let output = temp.join("windows_bindgen_commands.rs");
    let commands = temp.join("windows_bindgen_commands.txt");
    let filters = temp.join("windows_bindgen_commands_filters.txt");

    windows_rdl::reader()
        .input_text(
            r#"
#[win32]
mod Test {
    #[library("test.dll")]
    extern fn Function() -> u32;
}
"#,
        )
        .output(&winmd)
        .write()
        .unwrap();

    std::fs::write(&filters, "Test\n").unwrap();
    std::fs::write(
        &commands,
        format!(
            "// commands\n--in {}\n--out {}\n--flat\n--filter-file {}\n",
            winmd.display(),
            output.display(),
            filters.display()
        ),
    )
    .unwrap();

    windows_bindgen::bindgen_file(commands);

    assert!(
        std::fs::read_to_string(output)
            .unwrap()
            .contains("fn Function")
    );
}
