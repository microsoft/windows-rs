use windows_rdl::*;

#[test]
pub fn parse() {
    // First compile a single-field struct to use as a reference type.
    let reference = std::env::temp_dir().join("windows_rdl_const_underlying_ref.winmd");

    reader()
        .input_str(
            r#"
#[win32]
mod Test {
    struct Flag {
        Value: i32,
    }
}
        "#,
        )
        .output(reference.to_str().unwrap())
        .write()
        .unwrap();

    // Now compile a constant whose type is the single-field struct defined above.
    reader()
        .input("tests/const-underlying.rdl")
        .input(reference.to_str().unwrap())
        .output("tests/const-underlying.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/const-underlying.winmd")
        .output("tests/const-underlying-out.rdl")
        .filter("Test")
        .write()
        .unwrap();
}
