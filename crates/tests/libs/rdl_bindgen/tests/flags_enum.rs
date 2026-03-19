use test_rdl_bindgen::flags_enum::Test::*;

#[test]
fn test() {
    windows_rdl::reader()
        .input("tests/flags_enum.rdl")
        .output("tests/flags_enum.winmd")
        .write()
        .unwrap();

    windows_rdl::writer()
        .input("tests/flags_enum.winmd")
        .output("tests/flags_enum.rdl")
        .filter("Test")
        .write()
        .unwrap();

    windows_bindgen::bindgen([
        "--in",
        "tests/flags_enum.winmd",
        "--out",
        "src/flags_enum.rs",
        "--filter",
        "Test",
        "--no-comment",
    ])
    .unwrap();

    // Validates that Win32 flags enums support bitwise operations and contains.
    let rw = Options::Read | Options::Write;
    assert_eq!(rw.0, 3);
    assert!(rw.contains(Options::Read));
    assert!(rw.contains(Options::Write));
    assert!(!rw.contains(Options::Execute));

    let read_only = rw & Options::Read;
    assert_eq!(read_only, Options::Read);

    let mut flags = Options::Read;
    flags |= Options::Write;
    assert!(flags.contains(Options::Read));
    assert!(flags.contains(Options::Write));

    flags &= Options::Read;
    assert!(flags.contains(Options::Read));
    assert!(!flags.contains(Options::Write));

    // Validates that flag enum constants can be used as match patterns.
    match Options::Read {
        Options::Read => {}
        _ => unreachable!(),
    }
}
