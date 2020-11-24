use tests::windows::devices::wi_fi_direct::{
    WiFiDirectConnectionParameters, WiFiDirectDevice, WiFiDirectDeviceSelectorType,
};

#[test]
fn wifi() -> winrt::Result<()> {
    // get_device_selector from IWiFiDirectDeviceStatics
    let a = WiFiDirectDevice::get_device_selector()?;
    assert!(!a.is_empty());

    // from_id_async from IWiFiDirectDeviceStatics
    assert!(
        WiFiDirectDevice::from_id_async(a)?.get()
            == Err(winrt::Error::fast_error(winrt::ErrorCode::E_POINTER))
    );

    // get_device_selector overload from IWiFiDirectDeviceStatics2 is renamed to get_device_selector2
    let c = WiFiDirectDevice::get_device_selector2(WiFiDirectDeviceSelectorType::DeviceInterface)?;
    assert!(!c.is_empty());

    // from_id_async overload from IWiFiDirectDeviceStatics2 is renamed to from_id_async2
    WiFiDirectDevice::from_id_async2(c, WiFiDirectConnectionParameters::new()?)?;
    Ok(())
}

#[test]
fn email() -> winrt::Result<()> {

}
