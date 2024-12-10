use windows::{Win32::Foundation::*, Win32::NetworkManagement::IpHelper::*};

#[test]
fn test() {
    let status = BOOLEAN::default();
    assert_eq!(status.0, 0);

    let status = BOOLEAN(1);
    assert_eq!(status.0, 1);
    assert!(status.as_bool());
    assert!(status.ok().is_ok());

    unsafe {
        SetLastError(ERROR_ACCESS_DENIED);
    }
    let status = BOOLEAN(0);
    assert_eq!(status.0, 0);
    assert!(!status.as_bool());
    let result = status.ok();
    assert!(!result.is_ok());
    let error = result.unwrap_err();
    assert_eq!(error.code(), E_ACCESSDENIED);
}

#[test]
#[ignore]
fn no_run() {
    unsafe {
        _ = NotifyUnicastIpAddressChange(
            Default::default(),
            None,
            None,
            true,
            std::ptr::null_mut(),
        );
        _ = NotifyUnicastIpAddressChange(
            Default::default(),
            None,
            None,
            true,
            std::ptr::null_mut(),
        );
    }
}
