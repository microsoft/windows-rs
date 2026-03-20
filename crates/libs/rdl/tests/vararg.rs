use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/vararg.rdl")
        .output("tests/vararg.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/vararg.winmd")
        .output("tests/vararg.rdl")
        .filter("Test")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: variadic parameters are not supported for callbacks")]
fn vararg_callback_not_supported() {
    reader()
        .input_str(
            r#"
#[win32]
mod Test {
    extern "C" fn Handler(a: i32, ...);
}
        "#,
        )
        .output(".")
        .write()
        .unwrap();
}
