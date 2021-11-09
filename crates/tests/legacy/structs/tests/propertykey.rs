use test_structs::Windows::Win32::Devices::Properties::DEVPKEY_Device_BiosDeviceName;

#[test]
fn test_debug_impl() {
    assert_eq!("PROPERTYKEY { fmtid: 540B947E-8B40-45BC-A8A2-6A0B894CBDA2, pid: 10 }", format!("{:?}", DEVPKEY_Device_BiosDeviceName));
}
