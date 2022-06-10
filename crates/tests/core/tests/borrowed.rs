use windows::core::*;

#[test]
fn borrowed_works() {
    let num = Borrowed::new(Some(&1u8));
    assert_eq!(num.as_ref(), Some(&1u8));

    let hello = HSTRING::from("Hello");
    let h = Borrowed::new(Some(&hello));
    assert_eq!(h.as_ref(), Some(&hello));
}
