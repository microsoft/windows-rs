use tests::windows::application_model::email::EmailAttachment;
use tests::windows::devices::wi_fi_direct::{
    WiFiDirectConnectionParameters, WiFiDirectDevice, WiFiDirectDeviceSelectorType,
};
use tests::windows::storage::streams::{InMemoryRandomAccessStream, RandomAccessStreamReference};

// WiFiDirectDevice has a pair of static factory interfaces with overloads. This test
// ensures that both overloads are visible and callable.
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

// EmailAttachment has a pair of activation (constructor) factory interfaces with overloads. This
// test ensures that the overloads are visible and callable.
#[test]
fn email() -> winrt::Result<()> {
    let stream = InMemoryRandomAccessStream::new()?;
    let reference = RandomAccessStreamReference::create_from_stream(stream)?;

    // Default constructor via IActivationFactory
    let a = EmailAttachment::new()?;
    assert!(a.file_name()? == "");

    // create from IEmailAttachmentFactory
    let b = EmailAttachment::create("create.txt", &reference)?;
    assert!(b.file_name()? == "create.txt");

    // create from IEmailAttachmentFactory2 is renamed to create2
    let c = EmailAttachment::create2("create2.txt", &reference, "text")?;
    assert!(c.file_name()? == "create2.txt");

    Ok(())
}
