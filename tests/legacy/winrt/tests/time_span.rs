use std::time::Duration;
use test_winrt::TestComponent::TestRunner;
use test_winrt::Windows::Foundation::{IPropertyValue, PropertyValue, TimeSpan};
use windows::Interface;

#[test]
fn conversion() -> windows::Result<()> {
    let a: TimeSpan = Duration::from_millis(1234).into();
    let b = TestRunner::CreateTimeSpan(1234)?;
    assert_eq!(a, b);

    let c: Duration = b.into();
    assert_eq!(c.as_millis(), 1234);

    Ok(())
}

#[test]
fn duration_param() -> windows::Result<()> {
    let object = PropertyValue::CreateTimeSpan(Duration::from_millis(1234))?;
    let pv: IPropertyValue = object.cast()?;
    assert!(pv.GetTimeSpan()? == Duration::from_millis(1234).into());

    Ok(())
}

#[test]
fn time_span_param() -> windows::Result<()> {
    let object = PropertyValue::CreateTimeSpan(TestRunner::CreateTimeSpan(1234)?)?;
    let pv: IPropertyValue = object.cast()?;
    assert!(pv.GetTimeSpan()? == Duration::from_millis(1234).into());

    Ok(())
}
