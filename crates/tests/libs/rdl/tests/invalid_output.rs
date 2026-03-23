#[test]
#[should_panic(expected = "error: invalid output\n --> .")]
fn invalid_output() {
    windows_rdl::reader().output(".").write().unwrap();
}
