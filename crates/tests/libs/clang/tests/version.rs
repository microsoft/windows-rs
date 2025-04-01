#[test]
fn test() {
    assert!(windows_clang::version().starts_with("clang version"));
}
