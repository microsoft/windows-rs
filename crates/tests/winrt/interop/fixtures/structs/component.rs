use super::*;
use Structs::*;
use std::sync::Mutex;
use windows_collections::{IIterable, IVectorView};
use windows_reference::IReference;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Structs.Widget").then(activatable::<Widget, Structs::Widget>)
}

#[implement(Structs::Widget)]
struct Widget(Mutex<Label>);

impl Default for Widget {
    fn default() -> Self {
        Self(Mutex::new(Label {
            Text: "label".into(),
            Position: Point { X: 1, Y: 2 },
            Width: None,
        }))
    }
}

impl IWidget_Impl for Widget_Impl {
    fn Offset(&self, point: &Point, delta: i32) -> Result<Point> {
        Ok(Point {
            X: point.X + delta,
            Y: point.Y + delta,
        })
    }

    fn Step(&self, value: &Mixed) -> Result<Mixed> {
        Ok(Mixed {
            Flag: !value.Flag,
            Value: value.Value + 1,
            Code: value.Code + 1,
            Ratio: value.Ratio + 1.0,
        })
    }

    fn Label(&self) -> Result<Label> {
        Ok(self.0.lock().unwrap().clone())
    }

    fn SetLabel(&self, value: &Label) -> Result<()> {
        *self.0.lock().unwrap() = value.clone();
        Ok(())
    }

    fn Describe(&self, label: &Label) -> Result<HSTRING> {
        let width = match &label.Width {
            Some(width) => width.Value()?.to_string(),
            None => "none".to_string(),
        };

        Ok(format!(
            "{} ({}, {}) {width}",
            label.Text.to_string_lossy(),
            label.Position.X,
            label.Position.Y
        )
        .into())
    }

    fn Points(&self) -> Result<IVectorView<Point>> {
        Ok(IVectorView::from(vec![
            Point { X: 1, Y: 2 },
            Point { X: 3, Y: 4 },
        ]))
    }

    fn Sum(&self, points: Ref<IIterable<Point>>) -> Result<Point> {
        Ok(points
            .ok()?
            .into_iter()
            .fold(Point::default(), |sum, point| Point {
                X: sum.X + point.X,
                Y: sum.Y + point.Y,
            }))
    }

    fn Labels(&self, count: i32) -> Result<Array<Label>> {
        let labels: Vec<Label> = (0..count)
            .map(|index| Label {
                Text: index.to_string().into(),
                Position: Point { X: index, Y: index },
                Width: Some(IReference::from(index)),
            })
            .collect();

        Ok(Array::from_slice(&labels))
    }

    fn BoxPoint(&self, point: &Point) -> Result<IInspectable> {
        Ok(IReference::from(*point).into())
    }

    fn UnboxPoint(&self, value: Ref<IInspectable>) -> Result<Point> {
        value.ok()?.cast::<IReference<Point>>()?.Value()
    }
}
