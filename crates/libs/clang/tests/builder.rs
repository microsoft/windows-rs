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
        .symbol("GetApi")
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
fn builder_scans_only_the_requested_directory() {
    helpers::ensure_libclang();

    let scratch = std::env::temp_dir().join(format!(
        "windows-clang-builder-directory-{}",
        std::process::id()
    ));
    let nested = scratch.join("nested");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::write(
        scratch.join("api.h"),
        "typedef struct API { int value; } API;\n",
    )
    .unwrap();
    std::fs::write(nested.join("unrelated.h"), "this is not valid C").unwrap();

    let output = scratch.join("api.rdl");
    windows_clang::clang()
        .input(&scratch)
        .args(["-x", "c++"])
        .namespace("Builder")
        .output(&output)
        .write()
        .unwrap();

    let rdl = std::fs::read_to_string(output).unwrap();
    assert!(rdl.contains("struct API"));

    std::fs::remove_dir_all(scratch).unwrap();
}
