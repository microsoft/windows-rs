#![cfg(all(target_arch = "x86_64", target_env = "msvc"))]

#[test]
fn test() {
    assert!(windows_clang::version().starts_with("clang version"));
}
