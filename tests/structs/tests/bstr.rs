use test_structs::Windows::Win32::System::Diagnostics::Debug::DebugPropertyInfo;
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
    assert_ne!(a.m_bstrName.abi().0, b.m_bstrName.abi().0);
}
