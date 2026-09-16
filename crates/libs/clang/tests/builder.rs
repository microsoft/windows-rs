#[test]
fn builder_uses_the_snapshot_pipeline() {
    helpers::ensure_libclang();

    let scratch =
        std::env::temp_dir().join(format!("windows-clang-builder-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    std::fs::write(
        scratch.join("dependency.h"),
        "typedef struct DEPENDENCY { int value; } DEPENDENCY;\n",
    )
    .unwrap();
    std::fs::write(
        scratch.join("api.h"),
        "#include \"dependency.h\"\n\
         typedef struct API { DEPENDENCY dependency; } API;\n\
         extern \"C\" API GetApi();\n\
         extern \"C\" API GetOtherApi();\n",
    )
    .unwrap();

    let include = format!("-I{}", scratch.display());
    let output = scratch.join("output").join("api.rdl");
    windows_clang::clang()
        .input_text("#include \"api.h\"\n")
        .args(["-x", "c++", include.as_str()])
        .filter("api.h")
        .symbols(["GetApi"])
        .reference_default()
        .namespace("Builder")
        .library("builder.dll")
        .output(&output)
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    assert!(rdl.contains("struct API"));
    assert!(rdl.contains("struct DEPENDENCY"));
    assert!(rdl.contains("extern \"C\" fn GetApi() -> API"));
    assert!(!rdl.contains("GetOtherApi"));
    assert!(rdl.contains("#[library(\"builder.dll\")]"));

    std::fs::remove_dir_all(scratch).unwrap();
}

#[test]
fn builder_excludes_items_supplied_by_references() {
    helpers::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang-builder-reference-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&scratch).unwrap();
    let reference = scratch.join("reference.winmd");
    windows_rdl::reader()
        .input_text(
            "#[win32]
            mod Reference {
                const EXISTING_VALUE: i32 = 1;
                #[library(\"reference.dll\")]
                extern \"C\" fn ExistingFunction() -> i32;
            }",
        )
        .output(&reference)
        .write()
        .unwrap();

    let output = scratch.join("api.rdl");
    windows_clang::clang()
        .input_text(
            "#define EXISTING_VALUE 1
             extern \"C\" int ExistingFunction();
             extern \"C\" int NewFunction();",
        )
        .args(["-x", "c++"])
        .reference(&reference)
        .namespace("Builder")
        .library("builder.dll")
        .output(&output)
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    assert!(!rdl.contains("EXISTING_VALUE"));
    assert!(!rdl.contains("ExistingFunction"));
    assert!(rdl.contains("NewFunction"));

    std::fs::remove_dir_all(scratch).unwrap();
}
