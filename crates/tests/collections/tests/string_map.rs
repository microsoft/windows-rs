use windows::{core::*, Foundation::Collections::*};

#[test]
fn test() -> Result<()> {
    let map = StringMap::new()?;
    map.Insert(h!("key"), h!("value"))?;
    assert!(map.HasKey(h!("key"))?);
    assert!(map.HasKey(&HSTRING::from("key"))?);

    Ok(())
}
