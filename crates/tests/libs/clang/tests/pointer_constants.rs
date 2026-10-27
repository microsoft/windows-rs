use windows_clang::*;

#[test]
fn numeric_pointer_rdl() {
    helpers::ensure_libclang();
    let mut input = test_clang::fixture("pointer_constants");
    input.macros = vec!["INVALID".into()];
    let output = projection::generate(&[input], "Fixture").unwrap();
    let rdl = &output.partitions["pointer_constants"].rdl;
    let expected = std::fs::read_to_string(test_clang::input_dir().join("pointer_constants.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(*rdl, expected);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let path = root.join("target\\test-clang-pointer-constants.winmd");
    let roundtrip = path.with_extension("rdl");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(rdl)
        .output(&path)
        .write()
        .unwrap();
    windows_rdl::writer()
        .input(&path)
        .filter("Fixture")
        .output(&roundtrip)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&roundtrip).unwrap(), *rdl);
    std::fs::remove_file(path).unwrap();
    std::fs::remove_file(roundtrip).unwrap();
}

#[test]
fn pointer_rejections() {
    helpers::ensure_libclang();
    let input = test_clang::selected_fixture(
        "pointer_constants_invalid",
        &[
            "Relocated",
            "SideEffects",
            "Undefined",
            "Mutable",
            "CodeAddress",
            "ObjectAddress",
            "Safe",
        ],
    );
    assert!(projection::generate(std::slice::from_ref(&input), "Fixture").is_err());
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.constants, 1);
    for name in [
        "Relocated",
        "SideEffects",
        "Undefined",
        "Mutable",
        "CodeAddress",
        "ObjectAddress",
    ] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}: {:?}",
            partial.omissions
        );
    }
}
