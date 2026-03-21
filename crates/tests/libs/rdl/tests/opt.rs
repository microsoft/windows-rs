use windows_rdl::*;

#[test]
pub fn parse() {
    reader()
        .input("tests/opt.rdl")
        .output("tests/opt.winmd")
        .write()
        .unwrap();

    writer()
        .input("tests/opt.winmd")
        .output("tests/opt.rdl")
        .filter("Test")
        .write()
        .unwrap();
}

// Regression test: `#[opt]` must be preserved when followed by `#[out]`.
// The order of `#[opt]` and `#[out]` attributes should not matter.
#[test]
pub fn opt_before_out() {
    let winmd = std::env::temp_dir().join("windows_rdl_opt_before_out.winmd");
    let rdl = std::env::temp_dir().join("windows_rdl_opt_before_out.rdl");

    reader()
        .input_str(
            r#"
#[winrt]
mod Test {
    interface ITest {
        fn OutOptional(&self, #[opt] #[out] value: u32);
    }
}
        "#,
        )
        .output(&winmd.to_string_lossy())
        .write()
        .unwrap();

    writer()
        .input(&winmd.to_string_lossy())
        .output(&rdl.to_string_lossy())
        .filter("Test")
        .write()
        .unwrap();

    let contents = std::fs::read_to_string(&rdl).unwrap();
    assert!(
        contents.contains("#[opt]"),
        "Expected `#[opt]` in output but got:\n{contents}"
    );
}
