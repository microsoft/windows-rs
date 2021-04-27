use test_weak::*;
use windows::*;

#[test]
fn test_implement() -> Result<()> {
    let strong: Windows::Foundation::IStringable = Stringable {}.into();

    let weak = strong.downgrade()?;
    assert_eq!(weak.upgrade().unwrap(), strong);

    assert_eq!(strong.ToString()?, "Stringable");
    drop(strong);

    assert_eq!(weak.upgrade(), None);
    Ok(())
}

#[implement(Windows::Foundation::IStringable)]
struct Stringable {}

#[allow(non_snake_case)]
impl Stringable {
    fn ToString(&self) -> Result<HString> {
        Ok("Stringable".into())
    }
}
