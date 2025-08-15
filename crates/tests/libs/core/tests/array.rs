use windows::core::Array;

#[test]
fn empty() {
    let empty = Array::<bool>::new();
    assert!(empty.is_empty());
    assert!(empty.is_empty());
}

#[test]
fn with_len() {
    let empty = Array::<u32>::with_len(3);
    assert!(!empty.is_empty());
    assert!(empty.len() == 3);
    assert!(empty[0] == 0);
    assert!(empty[1] == 0);
    assert!(empty[2] == 0);
}

#[test]
fn debug() {
    let array = Array::<i32>::from_slice(&[1, 2, 3]);
    assert_eq!(format!("{array:?}"), "[1, 2, 3]");
    assert_eq!(
        format!("\n{array:#?}\n"),
        r#"
[
    1,
    2,
    3,
]
"#
    );
}
