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
    assert_eq!(widget.Sum(&numbers)?, 10);
    assert_eq!(widget.Sum(&IIterable::from(vec![1, 2]))?, 3);

    let map = IMapView::from(std::collections::BTreeMap::from([(
        HSTRING::from("one"),
        1,
    )]));
    assert_eq!(widget.Lookup(&map, h!("one"))?, 1);

    let values: Vec<HSTRING> = widget.Values()?.into_iter().collect();
    assert_eq!(values, ["one", "two"]);

    let index = widget.Index(&IVectorView::from(values))?;
    assert_eq!(index.Size()?, 2);
    assert_eq!(index.Lookup(h!("two"))?, 1);

    let item: IInspectable = windows_reference::IReference::from("item").into();
    let values = widget.ReadOnly(&[Some(item.clone()), None])?;
    assert_eq!(values.GetAt(0)?, item);
    assert_eq!(values.Clear().unwrap_err().code(), HRESULT(0x80004001_u32 as _));
    let mut items = [None, None];
    assert_eq!(values.GetView()?.GetMany(0, &mut items)?, 2);
    assert_eq!(items, [Some(item), None]);

    Ok(())
}
