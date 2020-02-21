#[test]
fn strings() {
    use winrt::HString;

    let empty = HString::new();
    assert!(empty.is_empty());
    assert!(empty.len() == 0);

    let mut hello = HString::from("Hello");
    assert!(!hello.is_empty());
    assert!(hello.len() == 5);

    let rust = hello.to_string();
    assert!(rust == "Hello");
    assert!(rust.len() == 5);

    let hello2 = hello.clone();
    hello.clear();
    assert!(hello.len() == 0);
    hello.clear();
    assert!(hello.len() == 0);
    assert!(!hello2.is_empty());
    assert!(hello2.len() == 5);

    assert!(HString::from("Hello") == HString::from("Hello"));
    assert!(HString::from("Hello") != HString::from("World"));

    assert!(HString::from("Hello") == "Hello");
    assert!(HString::from("Hello") != "Hello ");
    assert!(HString::from("Hello") != "Hell");
    assert!(HString::from("Hello") != "World");
}
