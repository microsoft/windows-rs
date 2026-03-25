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

// Test that `#[in]` and `#[out]` shorthands are accepted as aliases for
// `#[input]` and `#[output]` respectively. `in` is a Rust keyword and cannot
// be used directly in attribute paths by `syn`, so the reader normalizes them
// to their long forms before parsing.
#[test]
pub fn in_out_shorthands() {
    let winmd_long = std::env::temp_dir().join("windows_rdl_in_out_long.winmd");
    let winmd_short = std::env::temp_dir().join("windows_rdl_in_out_short.winmd");
    let rdl_long = std::env::temp_dir().join("windows_rdl_in_out_long.rdl");
    let rdl_short = std::env::temp_dir().join("windows_rdl_in_out_short.rdl");

    // Use the long `#[input]` / `#[output]` forms.
    reader()
        .input_str(
            r#"
#[winrt]
mod Test {
    interface ITest {
        fn Method(&self, #[input] in_ptr: *mut u32, #[output] out_val: u32);
    }
}
        "#,
        )
        .output(&winmd_long.to_string_lossy())
        .write()
        .unwrap();

    // Use the short `#[in]` / `#[out]` forms — should produce identical metadata.
    reader()
        .input_str(
            r#"
#[winrt]
mod Test {
    interface ITest {
        fn Method(&self, #[in] in_ptr: *mut u32, #[out] out_val: u32);
    }
}
        "#,
        )
        .output(&winmd_short.to_string_lossy())
        .write()
        .unwrap();

    // Both should round-trip to the same canonical RDL output.
    writer()
        .input(&winmd_long.to_string_lossy())
        .output(&rdl_long.to_string_lossy())
        .filter("Test")
        .write()
        .unwrap();

    writer()
        .input(&winmd_short.to_string_lossy())
        .output(&rdl_short.to_string_lossy())
        .filter("Test")
        .write()
        .unwrap();

    let long_contents = std::fs::read_to_string(&rdl_long).unwrap();
    let short_contents = std::fs::read_to_string(&rdl_short).unwrap();
    assert_eq!(
        long_contents, short_contents,
        "`#[in]`/`#[out]` should produce the same output as `#[input]`/`#[output]`"
    );
}
