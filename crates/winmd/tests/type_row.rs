extern crate windows_winmd as winmd;

#[test]
fn win32() {
    let reader = winmd::TypeReader::get();

    if let winmd::Type::TypeDef(def) = reader.expect_type(("Windows.Foundation", "IStringable")) {
        assert!(def.name() == ("Windows.Foundation", "IStringable"));
    } else {
        panic!();
    }

    if let winmd::Type::MethodDef((def, method)) =
        reader.expect_type(("Windows.Win32.SystemServices", "CreateEventW"))
    {
        assert!(def.name() == ("Windows.Win32.SystemServices", "Apis"));
        assert!(method.name() == "CreateEventW");
    } else {
        panic!();
    }

    if let winmd::Type::Field((def, field)) =
        reader.expect_type(("Windows.Win32.SystemServices", "WM_KEYDOWN"))
    {
        assert!(def.name() == ("Windows.Win32.SystemServices", "Apis"));
        assert!(field.name() == "WM_KEYDOWN");
    } else {
        panic!();
    }
}
