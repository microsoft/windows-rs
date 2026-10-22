use windows_clang::*;

#[test]
fn compiler_static_initializers() {
    helpers::ensure_libclang();
    let mut input = test_clang::selected_fixture("layout_constants", &["Record"]);
    input.macros = ["MEMBER_END", "MEMBER_OFFSET", "BUILTIN_OFFSET"]
        .map(String::from)
        .into();
    let expected = std::fs::read_to_string(test_clang::input_dir().join("layout_constants.rdl"))
        .unwrap()
        .replace("\r\n", "\n");
    let output = projection::generate(std::slice::from_ref(&input), "Fixture").unwrap();
    assert_eq!(output.partitions["layout_constants"].rdl, expected);
    let rejected = [
        "SIDE_EFFECT",
        "MUTABLE_READ",
        "DIV_ZERO",
        "SIGNED_OVERFLOW",
        "NULL_READ",
    ];
    input.macros.extend(rejected.map(String::from));
    let partial = projection::generate_partial(&[input], "Fixture").unwrap();
    assert_eq!(partial.output, output);
    assert_eq!(partial.omissions.len(), rejected.len());
    for name in rejected {
        assert!(partial.omissions.iter().any(|message| {
            message.contains(name) && message.contains("initializer is not a constant expression")
        }));
    }
}
