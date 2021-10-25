use test_winrt_conversion::*;
use windows::runtime::*;
use Component::Classes::Required;
use Component::Conversion::*;
use Component::Interfaces::IProperty;

#[test]
fn expect() -> Result<()> {
    assert!(Test::ExpectTimeSpan(std::time::Duration::from_millis(1234))? == 1234);

    assert!(Test::ExpectObject(Required::new()?)? == "Component.Classes.Required");

    let required: IProperty = Required::new()?.into();
    assert!(Test::ExpectObject(required)? == "Component.Classes.Required");

    Ok(())
}
