use test_clang2::{input_dir, selected_fixture};
use windows_clang2::*;

#[test]
fn array_expressions() {
    helpers::ensure_libclang();
    let input = selected_fixture(
        "array_expressions",
        &[
            "Arrays",
            "Computed",
            "UsingArray",
            "PointerArray",
            "ArrayCallback",
            "AliasCallback",
        ],
    );
    let output = projection::generate(&[input], "Fixture").unwrap();
    assert_eq!((output.records, output.aliases), (1, 5));
    if std::env::var_os("UPDATE_CLANG2_RDL").is_some() {
        std::fs::write(
            input_dir().join("array_expressions.rdl"),
            &output.partitions["array_expressions"].rdl,
        )
        .unwrap();
    }
    let expected = std::fs::read_to_string(input_dir().join("array_expressions.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    assert_eq!(output.partitions["array_expressions"].rdl, expected);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\..\\..\\..");
    let winmd = root.join("target\\test-clang2-array-expressions.winmd");
    windows_rdl::reader()
        .input(root.join("metadata\\metadata.rdl"))
        .input_text(&expected)
        .output(&winmd)
        .write()
        .unwrap();
    let rewritten = root.join("target\\test-clang2-array-expressions.rdl");
    windows_rdl::writer()
        .input(&winmd)
        .filter("Fixture")
        .output(&rewritten)
        .write()
        .unwrap();
    assert_eq!(std::fs::read_to_string(&rewritten).unwrap(), expected);
    for path in [winmd, rewritten] {
        std::fs::remove_file(path).unwrap();
    }
    let input = selected_fixture("array_initializers", &[]);
    assert!(projection::generate(std::slice::from_ref(&input), "Fixture").is_err());
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.records, 1, "{:?}", partial.output.partitions);
    assert_eq!(
        partial.output.partitions["array_initializers"].rdl,
        "#[win32]\nmod Fixture {\n    struct Safe {\n        values: [i32; 3],\n    }\n}\n"
    );
    for name in [
        "Initialized",
        "InitializedExpression",
        "InitializedArray",
        "InitializedBits",
        "Dependent",
        "PlatformInitializer",
    ] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
    let mut input = selected_fixture("array_parameters", &[]);
    input
        .arguments
        .extend(["-x".into(), "c".into(), "-std=c17".into()]);
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output.aliases, 1, "{partial:?}");
    assert_eq!(
        partial.output.partitions["array_parameters"].rdl,
        "#[win32]\nmod Fixture {\n    extern \"C\" fn GoodBound(values: *mut i32);\n}\n"
    );
    for name in ["StaticBound", "QualifiedBound", "VariableBound"] {
        assert!(
            partial
                .omissions
                .iter()
                .any(|message| message.contains(name)),
            "{name}"
        );
    }
}
