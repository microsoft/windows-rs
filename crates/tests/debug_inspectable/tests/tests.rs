use windows::{core::*, Foundation::Collections::*, Foundation::*};

#[test]
fn test() -> Result<()> {
    let stringable: IInspectable = Uri::CreateUri(h!("https://kennykerr.ca"))?.cast()?;
    let non_stringable: IInspectable = PropertySet::new()?.cast()?;

    assert_eq!(format!("{:?}", stringable), "\"https://kennykerr.ca/\"");
    assert_eq!(
        format!("{:?}", non_stringable),
        "\"Windows.Foundation.Collections.PropertySet\""
    );

    Ok(())
}
