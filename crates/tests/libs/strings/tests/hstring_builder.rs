use windows_strings::*;

#[test]
fn hstring() {
    let s = HSTRING::from("hello");
    assert_eq!(s.len(), 5);
}

#[test]
fn hstring_builder() {
    // Dropping a builder is fine.
    _ = HStringBuilder::new(10);

    // A zero length builder is also fine.
    let b = HStringBuilder::new(0);
    let h: HSTRING = b.into();
    assert!(h.is_empty());

    // Trimming a zero length builder is also fine.
    let mut b = HStringBuilder::new(0);
    b.trim_end();
    let h: HSTRING = b.into();
    assert!(h.is_empty());

    // This depends on DerefMut.
    const HELLO: [u16; 5] = [0x48, 0x65, 0x6C, 0x6C, 0x6F];
    let mut b = HStringBuilder::new(5);
    b.copy_from_slice(&HELLO);
    let h: HSTRING = b.into();
    assert_eq!(&h, "Hello");

    // HSTRING can handle embedded nulls.
    const HELLO00: [u16; 7] = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x00];
    let mut b = HStringBuilder::new(7);
    b.copy_from_slice(&HELLO00);
    let h: HSTRING = b.into();
    assert_eq!(h.len(), 7);
    assert_eq!(*h, HELLO00);

    // But trim_end can avoid that.
    let mut b = HStringBuilder::new(7);
    b.copy_from_slice(&HELLO00);
    b.trim_end();
    let h: HSTRING = b.into();
    assert_eq!(h.len(), 5);
    assert_eq!(*h, HELLO);

    // HStringBuilder will initialize memory to zero.
    let b = HStringBuilder::new(5);
    assert_eq!(*b, [0, 0, 0, 0, 0]);
}

#[test]
fn debug() {
    const HELLO: [u16; 5] = [0x48, 0x65, 0x6C, 0x6C, 0x6F];
    let mut b = HStringBuilder::new(5);
    b.copy_from_slice(&HELLO);
    assert_eq!(format!("{b:?}"), "\"Hello\"");
}
