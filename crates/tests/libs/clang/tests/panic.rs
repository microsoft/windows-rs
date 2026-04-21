#![cfg(target_pointer_width = "64")]

fn should_panic(input: &str) {
    windows_rdl::clang()
        .input_str(input)
        .output(".rdl")
        .namespace("Test")
        .write()
        .unwrap();
}

#[test]
#[should_panic(expected = "error: expected ';' after struct\n --> .h:3:2")]
fn semicolon_expected() {
    should_panic(
        r#"
struct Thing {
}
        "#,
    );
}
