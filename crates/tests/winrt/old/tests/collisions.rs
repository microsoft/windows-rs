use windows::{
    core::HSTRING,
    ApplicationModel::Email::EmailAttachment,
    Devices::WiFiDirect::{
        WiFiDirectConnectionParameters, WiFiDirectDevice, WiFiDirectDeviceSelectorType,
    },
    Storage::Streams::{InMemoryRandomAccessStream, RandomAccessStreamReference},
};

// WiFiDirectDevice has a pair of static factory interfaces with overloads. This test
// ensures that both overloads are visible and callable.
#[test]
fn wifi() -> windows::core::Result<()> {
    // get_device_selector from IWiFiDirectDeviceStatics
    let a = WiFiDirectDevice::GetDeviceSelector()?;
    assert!(!a.is_empty());

    // from_id_async from IWiFiDirectDeviceStatics
    assert!(WiFiDirectDevice::FromIdAsync(&a)?.join() == Err(windows::core::Error::empty()));

    // get_device_selector overload from IWiFiDirectDeviceStatics2 is renamed to get_device_selector2
    let c = WiFiDirectDevice::GetDeviceSelector2(WiFiDirectDeviceSelectorType::DeviceInterface)?;
    assert!(!c.is_empty());

    // from_id_async overload from IWiFiDirectDeviceStatics2 is renamed to from_id_async2
    WiFiDirectDevice::FromIdAsync2(&c, &WiFiDirectConnectionParameters::new()?)?;
    Ok(())
}

// EmailAttachment has a pair of activation (constructor) factory interfaces with overloads. This
// test ensures that the overloads are visible and callable.
#[test]
fn email() -> windows::core::Result<()> {
    let stream = InMemoryRandomAccessStream::new()?;
    let reference = RandomAccessStreamReference::CreateFromStream(&stream)?;

    // Default constructor via IActivationFactory
    let a = EmailAttachment::new()?;
    assert!(a.FileName()?.is_empty());

    // create from IEmailAttachmentFactory
    let b = EmailAttachment::Create(&HSTRING::from("create.txt"), &reference)?;
    assert!(b.FileName()? == "create.txt");

    // create from IEmailAttachmentFactory2 is renamed to create2
    let c = EmailAttachment::Create2(
        &HSTRING::from("create2.txt"),
        &reference,
        &HSTRING::from("text"),
    )?;
    assert!(c.FileName()? == "create2.txt");

    Ok(())
}
