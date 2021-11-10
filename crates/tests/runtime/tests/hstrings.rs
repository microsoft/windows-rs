use std::convert::TryFrom;
use windows::runtime::*;
type StringType = HSTRING;

#[test]
fn hstring_works() {
    let empty = StringType::new();
    assert!(empty.is_empty());
    assert!(empty.is_empty());

    let mut hello = StringType::from("Hello");
    assert!(!hello.is_empty());
    assert!(hello.len() == 5);

    let rust = hello.to_string();
    assert!(rust == "Hello");
    assert!(rust.len() == 5);

    let hello2 = hello.clone();
    hello.clear();
    assert!(hello.is_empty());
    hello.clear();
    assert!(hello.is_empty());
    assert!(!hello2.is_empty());
    assert!(hello2.len() == 5);

    assert!(StringType::from("Hello") == StringType::from("Hello"));
    assert!(StringType::from("Hello") != StringType::from("World"));

    assert!(StringType::from("Hello") == "Hello");
    assert!(StringType::from("Hello") != "Hello ");
    assert!(StringType::from("Hello") != "Hell");
    assert!(StringType::from("Hello") != "World");

    assert!(StringType::from("Hello").to_string() == String::from("Hello"));
}

#[test]
fn display_format() {
    let value = StringType::from("Hello world");
    assert!(format!("{}", value) == "Hello world");
}

#[test]
fn debug_format() {
    let value = StringType::from("Hello world");
    assert!(format!("{:?}", value) == "Hello world");
}

#[test]
fn from_empty_string() {
    let h = StringType::from("");
    assert!(format!("{}", h).is_empty());
}

#[test]
fn hstring_to_string() {
    let h = StringType::from("test");
    let s = String::try_from(h).unwrap();
    assert!(s == "test");
}

#[test]
fn hstring_to_string_err() {
    // 𝄞mu<invalid>ic
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let h = StringType::from_wide(wide_data);
    let err = String::try_from(h);
    assert!(err.is_err());
}

#[test]
fn hstring_to_string_lossy() {
    // 𝄞mu<invalid>ic
    let wide_data = &[0xD834, 0xDD1E, 0x006d, 0x0075, 0xD800, 0x0069, 0x0063];
    let h = StringType::from_wide(wide_data);
    let s = h.to_string_lossy();
    assert_eq!(s, "𝄞mu�ic");
}
