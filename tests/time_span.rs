winrt::import!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
        nuget: KennyKerr.Windows.TestWinRT
    types
        test_component::*
        windows::foundation::*
);

use std::time::Duration;
use test_component::TestRunner;
use windows::foundation::*;
use winrt::TryInto;

#[test]
fn conversion() -> winrt::Result<()> {
    let a: winrt::TimeSpan = Duration::from_millis(1234).into();
    let b = TestRunner::create_time_span(1234)?;
    assert_eq!(a, b);

    let c: Duration = b.into();
    assert_eq!(c.as_millis(), 1234);

    Ok(())
}

#[test]
fn duration_param() -> winrt::Result<()> {
    let object = PropertyValue::create_time_span(Duration::from_millis(1234))?;
    let pv: IPropertyValue = object.try_into()?;
    assert!(pv.get_time_span()? == Duration::from_millis(1234).into());

    Ok(())
}

#[test]
fn time_span_param() -> winrt::Result<()> {
    let object = PropertyValue::create_time_span(TestRunner::create_time_span(1234)?)?;
    let pv: IPropertyValue = object.try_into()?;
    assert!(pv.get_time_span()? == Duration::from_millis(1234).into());

    Ok(())
}
