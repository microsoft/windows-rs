use windows::Win32::System::Diagnostics::Debug::DebugPropertyInfo;

#[test]
fn test() {
    let mut a = DebugPropertyInfo::default();
    assert_eq!(a.m_bstrName, "");
    a.m_bstrName = "Name".into();
    let b = a.clone();

    assert!(a == b);
    assert_eq!(a.m_bstrName, "Name");
    assert_eq!(b.m_bstrName, "Name");
}
