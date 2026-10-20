use super::*;
use Arrays::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(widget.Sum(&[1, 2, 3])?, 6);
    assert_eq!(widget.Sum(&[])?, 0);
    assert_eq!(widget.Join(&[h!("a").clone(), h!("b").clone()])?, "a,b");

    let mut values = [0; 3];
    widget.Fill(&mut values)?;
    assert_eq!(values, [1, 2, 3]);

    assert_eq!(&widget.Range(3)?[..], [1, 2, 3]);
    assert!(widget.Range(0)?.is_empty());
    assert_eq!(&widget.Split(h!("a,b"))?[..], ["a", "b"]);

    let mut values = Array::new();
    widget.RangeOut(2, &mut values)?;
    assert_eq!(&values[..], [1, 2]);

    Ok(())
}
