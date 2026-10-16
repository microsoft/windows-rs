use windows_clang2::*;

#[test]
#[ignore = "libclang PCH does not materialize its builtin GUID declaration"]
fn explicit_and_discovered_uuid() {
    helpers::ensure_libclang();
    let mut input = test_clang2::selected_fixture("uuid_macros", &["GUID"]);
    input.macros = vec!["IDENTITY_VALUE".into()];
    let explicit = projection::generate_partial(std::slice::from_ref(&input), "Fixture").unwrap();
    assert!(explicit.omissions.is_empty(), "{:?}", explicit.omissions);
    assert_eq!(explicit.output.constants, 1);
    input.macros.clear();
    input.macro_headers = vec!["uuid_macros.h".into()];
    let discovered = projection::generate_partial(&[input], "Fixture").unwrap();
    assert!(
        discovered.omissions.is_empty(),
        "{:?}",
        discovered.omissions
    );
    assert_eq!(discovered.output, explicit.output);
}
