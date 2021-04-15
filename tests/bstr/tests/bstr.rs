use test_bstr::Windows::Win32::Automation::BSTR;

#[test]
fn test() {
    let b: BSTR = "hello".into();
    assert_eq!(b, "hello");
}
