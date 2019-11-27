#[test]
fn strings() {
    let empty: winrt::String = winrt::String::new();
    assert!(empty.is_empty());

    let hello: winrt::String = winrt::String::from("Hello");
    assert!(!hello.is_empty());

    let rust: std::string::String = hello.to_string();
    assert!(rust == "Hello");
}
