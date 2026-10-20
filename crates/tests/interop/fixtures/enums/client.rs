use super::*;
use Enums::*;
use windows_collections::IIterable;
use windows_reference::IReference;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(widget.Next(Color::Red)?, Color::Green);
    assert_eq!(widget.Next(Color::Blue)?, Color::Red);
    assert_eq!(widget.Next(Color::Unknown)?, Color::Unknown);
    assert_eq!(widget.Next(Color(7))?, Color::Unknown);

    assert_eq!(widget.Styles()?, Styles::Bold | Styles::Underline);
    widget.SetStyles(Styles::Italic)?;
    assert_eq!(widget.Styles()?, Styles::Italic);

    let colors: Vec<Color> = widget.Colors()?.into_iter().collect();
    assert_eq!(colors, [Color::Red, Color::Green, Color::Blue]);
    let styles = IIterable::from(vec![Styles::Bold, Styles::Underline]);
    assert_eq!(widget.Combine(&styles)?, Styles::Bold | Styles::Underline);

    let color = widget.BoxColor(Color::Blue)?.cast::<IReference<Color>>()?;
    assert_eq!(color.Value()?, Color::Blue);
    let styles: IInspectable = IReference::from(Styles::Italic | Styles::Underline).into();
    assert_eq!(
        widget.UnboxStyles(&styles)?,
        Styles::Italic | Styles::Underline
    );

    Ok(())
}
