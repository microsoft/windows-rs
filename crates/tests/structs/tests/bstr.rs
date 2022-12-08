use windows::{core::*, Win32::System::Diagnostics::Debug::DebugPropertyInfo};

#[test]
fn test() {
    // DebugPropertyInfo is a Win32 struct with non-blittable fields that turn into ManuallyDrop
    // fields since Win32 structs have no ownership semantics.

    let value: BSTR = "Name".into();

    let mut a = DebugPropertyInfo::default();
    assert_eq!(a.m_bstrName.unwrap(), "");
    a.m_bstrName = ManuallyDrop::new(&value);
    let b = a.clone();

    assert!(a == b);
    assert_eq!(a.m_bstrName.unwrap(), &value);
    assert_eq!(b.m_bstrName.unwrap(), &value);
}
