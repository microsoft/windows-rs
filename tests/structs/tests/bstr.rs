use test_structs::Windows::Win32::Debug::DebugPropertyInfo;
use windows::Abi;

#[test]
fn test() {
    let mut a = DebugPropertyInfo::default();
    assert_eq!(a.m_bstrName, "");
    a.m_bstrName = "Name".into();
    let b = a.clone();

    assert_eq!(a, b);
    assert_eq!(a.m_bstrName, "Name");
    assert_eq!(b.m_bstrName, "Name");
    assert_ne!(a.m_bstrName.abi(), b.m_bstrName.abi());
}
