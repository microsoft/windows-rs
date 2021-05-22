use std::time::{SystemTime, UNIX_EPOCH};
// use test_winrt::TestComponent::TestRunner;
use test_winrt::Windows::Foundation::{DateTime, IPropertyValue, PropertyValue};
use windows::Interface;

#[test]
fn conversion() -> windows::Result<()> {
    // TODO: imitate time_span.rs
    // let a: DateTime = UNIX_EPOCH.into();
    // let b = TestRunner::CreateUnixEpoch()?;
    // assert_eq!(a, b);
    //
    // let c: SystemTime = b.into();
    // assert_eq!(c, UNIX_EPOCH);

    let date_time: DateTime = UNIX_EPOCH.into();
    let system_time: SystemTime = date_time.into();
    assert!(system_time == UNIX_EPOCH);

    Ok(())
}

#[test]
fn system_time_param() -> windows::Result<()> {
    let object = PropertyValue::CreateDateTime(UNIX_EPOCH)?;
    let pv: IPropertyValue = object.cast()?;
    assert!(pv.GetDateTime()? == UNIX_EPOCH.into());

    Ok(())
}

#[test]
fn date_time_param() -> windows::Result<()> {
    // TODO: imitate time_span.rs
    // let object = PropertyValue::CreateDateTime(TestRunner::CreateUnixEpoch()?)?;
    // let pv: IPropertyValue = object.cast()?;
    // assert!(pv.GetDateTime()? == UNIX_EPOCH.into());

    Ok(())
}
