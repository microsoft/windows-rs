use super::*;
use Collections::*;
use windows_collections::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    let numbers = widget.Numbers()?;
    assert_eq!(numbers.Size()?, 3);
    numbers.Append(4)?;
    assert_eq!(numbers.Size()?, 4);
    assert_eq!(numbers.GetAt(3)?, 4);

    let map = IMapView::from(std::collections::BTreeMap::from([(
        HSTRING::from("one"),
        1,
    )]));
    assert_eq!(widget.Lookup(&map, h!("one"))?, 1);

    let values: Vec<HSTRING> = widget.Values()?.into_iter().collect();
    assert_eq!(values, ["one", "two"]);

    Ok(())
}
