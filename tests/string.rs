#[test]
fn strings() {
    let empty = winrt::String::new();
    assert!(empty.is_empty());

    let hello = winrt::String::from("Hello");
    assert!(!hello.is_empty());

    let rust = std::string::String::from(&hello);
    assert!(rust == "Hello");
}
