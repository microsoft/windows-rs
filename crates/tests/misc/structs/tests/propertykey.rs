use windows::Win32::Devices::Properties::DEVPKEY_Device_BiosDeviceName;

#[test]
fn test_debug_impl() {
    assert!(
        DEVPKEY_Device_BiosDeviceName.fmtid
            == "540B947E-8B40-45BC-A8A2-6A0B894CBDA2".try_into().unwrap()
    );
    assert_eq!(DEVPKEY_Device_BiosDeviceName.pid, 10);
}
