use windows_rdl::*;

#[test]
pub fn error_display() {
    let e = Error::new("message", "file_name.rdl", 2, 3);

    let s = format!("{e}");

    assert_eq!(s, "\nerror: message\n --> file_name.rdl:2:4");
}
