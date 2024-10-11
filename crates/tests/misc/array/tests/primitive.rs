use windows::core::Array;

#[test]
fn array() {
    let a = Array::<i32>::new();
    assert!(a.is_empty());

    let mut a = Array::<i32>::with_len(3);
    assert_eq!(a[0], 0);
    assert_eq!(a[1], 0);
    assert_eq!(a[2], 0);

    a[0] = 1;
    a[1] = 2;
    a[2] = 3;

    assert_eq!(a[0], 1);
    assert_eq!(a[1], 2);
    assert_eq!(a[2], 3);

    let result = a.as_slice().iter().sum::<i32>();
    assert_eq!(result, 6);

    let a = Array::<i32>::from_slice(&[4, 5, 6]);

    assert_eq!(a.len(), 3);
    assert_eq!(a[0], 4);
    assert_eq!(a[1], 5);
    assert_eq!(a[2], 6);
}
