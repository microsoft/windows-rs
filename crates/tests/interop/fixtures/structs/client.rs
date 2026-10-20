use super::*;
use Structs::*;
use windows_collections::IIterable;
use windows_reference::IReference;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(
        widget.Offset(Point { X: 1, Y: 2 }, 3)?,
        Point { X: 4, Y: 5 }
    );

    let mixed = widget.Step(Mixed {
        Flag: true,
        Value: 0x1_0000_0005,
        Code: 254,
        Ratio: 1.5,
    })?;
    assert_eq!(
        mixed,
        Mixed {
            Flag: false,
            Value: 0x1_0000_0006,
            Code: 255,
            Ratio: 2.5,
        }
    );

    let label = widget.Label()?;
    assert_eq!(label.Text, "label");
    assert_eq!(label.Position, Point { X: 1, Y: 2 });
    assert!(label.Width.is_none());

    let label = Label {
        Text: "text".into(),
        Position: Point { X: 3, Y: 4 },
        Width: Some(IReference::from(5)),
    };
    widget.SetLabel(&label)?;
    assert_eq!(widget.Describe(&widget.Label()?)?, "text (3, 4) 5");
    assert_eq!(widget.Describe(&Label::default())?, " (0, 0) none");

    let points: Vec<Point> = widget.Points()?.into_iter().collect();
    assert_eq!(points, [Point { X: 1, Y: 2 }, Point { X: 3, Y: 4 }]);
    let points = IIterable::from(vec![Point { X: 1, Y: 2 }, Point { X: 3, Y: 4 }]);
    assert_eq!(widget.Sum(&points)?, Point { X: 4, Y: 6 });

    let labels = widget.Labels(2)?;
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[1].Text, "1");
    assert_eq!(labels[1].Position, Point { X: 1, Y: 1 });
    assert_eq!(labels[1].Width.as_ref().unwrap().Value()?, 1);
    assert!(widget.Labels(0)?.is_empty());

    let point = widget
        .BoxPoint(Point { X: 5, Y: 6 })?
        .cast::<IReference<Point>>()?;
    assert_eq!(point.Value()?, Point { X: 5, Y: 6 });
    let point: IInspectable = IReference::from(Point { X: 7, Y: 8 }).into();
    assert_eq!(widget.UnboxPoint(&point)?, Point { X: 7, Y: 8 });

    Ok(())
}
