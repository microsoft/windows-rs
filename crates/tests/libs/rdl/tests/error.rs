use windows_rdl::*;

// The two `arch_*_is_error` tests that previously lived in this file have
// been migrated to `crates/tests/fixtures/harness/data/error/arch_*` —
// `expected.err` there asserts the full error message rather than just the
// `contains("arch")` substring the legacy tests used. Only `error_display`
// remains because it's a unit test for the `Display` impl, not a fixture.

#[test]
pub fn error_display() {
    let e = Error::new("message", "file_name.rdl", 2, 3);

    let s = format!("{e}");

    assert_eq!(s, "\nerror: message\n --> file_name.rdl:2:4");
}
