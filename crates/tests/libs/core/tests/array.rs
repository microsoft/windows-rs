use windows_core::Array;

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

// Exercises the alloc/dealloc round-trip across element sizes and
// alignments, using `clear()` to force an explicit free in addition
// to the implicit `Drop` at scope exit. On Windows this routes through
// `CoTaskMemAlloc`/`CoTaskMemFree`; on other targets it routes through
// the Rust global allocator.
#[test]
fn alloc_round_trip() {
    // u8: align 1, exercises the "smallest alignment" path.
    let mut a = Array::<u8>::with_len(7);
    assert_eq!(a.len(), 7);
    a.iter().for_each(|&v| assert_eq!(v, 0));
    a.clear();
    assert!(a.is_empty());

    // u64: align 8, exercises an alignment greater than `usize` on 32-bit hosts.
    let mut a = Array::<u64>::with_len(5);
    assert_eq!(a.len(), 5);
    a.iter().for_each(|&v| assert_eq!(v, 0));
    a.clear();
    assert!(a.is_empty());

    // Re-clear is a no-op.
    a.clear();
    assert!(a.is_empty());
}

#[test]
fn from_slice_round_trip() {
    let values: &[u32] = &[10, 20, 30, 40, 50];
    let array = Array::<u32>::from_slice(values);
    assert_eq!(array.len(), 5);
    assert_eq!(&array[..], values);
}

#[test]
fn deref_mut() {
    let mut array = Array::<u32>::with_len(3);
    array[0] = 7;
    array[2] = 9;
    assert_eq!(&array[..], &[7, 0, 9]);
}
