#![cfg(windows)]
use windows::{Win32::*, core::*};

#[test]
fn bstr_field() {
    // DebugPropertyInfo is a Win32 struct with non-blittable fields that turn into ManuallyDrop
    // fields since Win32 structs have no ownership semantics.

    let value: BSTR = "Name".into();

    let mut a = DebugPropertyInfo::default();
    assert_eq!(*a.m_bstrName, "");
    a.m_bstrName = unsafe { std::mem::transmute_copy(&value) };
    let b = a.clone();

    assert!(a == b);
    assert_eq!(*a.m_bstrName, value);
    assert_eq!(*b.m_bstrName, value);

    assert_eq!(*a.m_bstrName, "Name");
    assert_eq!(*b.m_bstrName, "Name");
}

#[test]
fn property_key() {
    assert!(
        DEVPKEY_Device_BiosDeviceName.fmtid
            == "540B947E-8B40-45BC-A8A2-6A0B894CBDA2".try_into().unwrap()
    );
    assert_eq!(DEVPKEY_Device_BiosDeviceName.pid, 10);
}
