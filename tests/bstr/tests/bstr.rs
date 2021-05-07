use test_bstr::Windows::Win32::System::OleAutomation::BSTR;
use windows::Abi;

#[test]
fn test() {
    let b: BSTR = "hello".into();
    assert_eq!(b, "hello");
}

#[test]
fn clone() {
    let a: BSTR = "hello".into();
    let b = a.clone();
    assert_eq!(a, "hello");
    assert_eq!(b, "hello");
    assert_ne!(a.abi(), b.abi());

    let a = BSTR::default();
    let b = a.clone();
    assert_eq!(a, "");
    assert_eq!(b, "");
    assert_eq!(a.abi(), b.abi());
}
