use windows_rdl::*;

// Regression test: `#[optional]` must be preserved when followed by `#[output]`.
// The order of `#[optional]` and `#[output]` attributes should not matter.
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
        fn OutOptional(&self, #[optional] #[output] value: u32);
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
        contents.contains("#[optional]"),
        "Expected `#[optional]` in output but got:\n{contents}"
    );
}
