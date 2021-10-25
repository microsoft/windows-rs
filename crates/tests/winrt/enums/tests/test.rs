use test_winrt_enums::*;
use Component::Enums::*;

#[test]
fn signed() {
    let value = Signed::One;
    assert!(value.0 == 1i32);

    let value = Signed::Two;
    assert!(value.0 == 2i32);

    let value = Signed::Three;
    assert!(value.0 == 3i32);

    let value: Signed = 2i32.into();
    assert!(value == Signed::Two);
}

#[test]
fn unsigned() {
    let value = Unsigned::One;
    assert!(value.0 == 0x001u32);

    let value = Unsigned::Two;
    assert!(value.0 == 0x010u32);

    let value = Unsigned::Three;
    assert!(value.0 == 0x100u32);

    let value: Unsigned = 0x010u32.into();
    assert!(value == Unsigned::Two);

    let value = Unsigned::One | Unsigned::Three;
    assert!(value.0 == 0x101u32);

    let value = Unsigned::One;
    assert_eq!(!value, Unsigned(0xFFFFFFFEu32))
}
