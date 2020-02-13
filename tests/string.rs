#[test]
fn strings() {
    let empty: winrt::String = winrt::String::new();
    assert!(empty.is_empty());
    assert!(empty.len() == 0);

    let hello: winrt::String = winrt::String::from("Hello");
    assert!(!hello.is_empty());
    assert!(hello.len() == 5);

    let rust: std::string::String = hello.to_string();
    assert!(rust == "Hello");
    assert!(rust.len() == 5);
}
