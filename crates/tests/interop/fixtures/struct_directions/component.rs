use super::*;
use StructDirections::*;
use windows_reference::IReference;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "StructDirections.Widget").then(activatable::<Widget, StructDirections::Widget>)
}

#[implement(StructDirections::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn ProcessPair(&self, value: &Pair, borrowed: &Pair, output: &mut Pair) -> Result<Pair> {
        *output = Pair {
            X: value.X + borrowed.X,
            Y: value.Y + borrowed.Y,
        };
        Ok(Pair {
            X: value.X - borrowed.X,
            Y: value.Y - borrowed.Y,
        })
    }

    fn ProcessLabel(&self, value: &Label, borrowed: &Label, output: &mut Label) -> Result<Label> {
        let position =
            self.ProcessPair(&value.Position, &borrowed.Position, &mut output.Position)?;
        output.Text = HSTRING::from_wide(&[&value.Text[..], &borrowed.Text[..]].concat());
        output.Width.clone_from(&value.Width);
        Ok(Label {
            Text: HSTRING::from_wide(&[&borrowed.Text[..], &value.Text[..]].concat()),
            Position: position,
            Width: borrowed.Width.clone(),
        })
    }

    fn CheckPair(&self, handler: Ref<PairHandler>) -> Result<()> {
        let value = Pair {
            X: 3,
            Y: 0x1_0000_0000,
        };
        let borrowed = Pair { X: -5, Y: 7 };
        let mut output = Pair::default();
        let returned = handler.ok()?.Invoke(value, borrowed, &mut output)?;
        check(
            output
                == Pair {
                    X: -2,
                    Y: 0x1_0000_0007,
                },
        )?;
        check(
            returned
                == Pair {
                    X: 8,
                    Y: 0xffff_fff9,
                },
        )
    }

    fn CheckLabel(&self, handler: Ref<LabelHandler>) -> Result<()> {
        let handler = handler.ok()?;
        let value = Label {
            Text: "left\0tail".into(),
            Position: Pair {
                X: 3,
                Y: 0x1_0000_0000,
            },
            Width: Some(IReference::from(17)),
        };
        let borrowed = Label {
            Text: "right".into(),
            Position: Pair { X: -5, Y: 7 },
            Width: None,
        };
        let mut output = Label::default();
        let returned = handler.Invoke(&value, borrowed, &mut output)?;
        check(output.Text == "left\0tailright")?;
        check(
            output.Position
                == Pair {
                    X: -2,
                    Y: 0x1_0000_0007,
                },
        )?;
        check(output.Width.as_ref().ok_or_else(Error::empty)?.Value()? == 17)?;
        check(returned.Text == "rightleft\0tail")?;
        check(
            returned.Position
                == Pair {
                    X: 8,
                    Y: 0xffff_fff9,
                },
        )?;
        check(returned.Width.is_none())
    }
}
