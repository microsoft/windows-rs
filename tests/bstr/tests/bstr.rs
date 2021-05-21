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

    let a = BSTR::new();
    assert_eq!(a.is_empty(), true);
    assert_eq!(a.len(), 0);
    assert_eq!(a.as_wide().len(), 0);

    let wide = &[0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let a = BSTR::from_wide(wide);
    assert_eq!(a.is_empty(), false);
    assert_eq!(a.len(), 5);
    assert_eq!(a.as_wide().len(), 5);
    assert_eq!(a.as_wide(), wide);
    assert_eq!(a, "hello");
}
