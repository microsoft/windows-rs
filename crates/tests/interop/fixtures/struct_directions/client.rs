use super::*;
use StructDirections::*;
use windows_reference::IReference;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    let value = Pair {
        X: 3,
        Y: 0x1_0000_0000,
    };
    let borrowed = Pair { X: -5, Y: 7 };
    let mut output = Pair::default();
    let returned = widget.ProcessPair(value, borrowed, &mut output)?;
    assert_eq!(
        output,
        Pair {
            X: -2,
            Y: 0x1_0000_0007
        }
    );
    assert_eq!(
        returned,
        Pair {
            X: 8,
            Y: 0xffff_fff9
        }
    );

    let (output, returned) = {
        let value = Label {
            Text: "left\0tail".into(),
            Position: value,
            Width: Some(IReference::from(17)),
        };
        let borrowed = Label {
            Text: "right".into(),
            Position: borrowed,
            Width: None,
        };
        let mut output = Label::default();
        let returned = widget.ProcessLabel(&value, borrowed, &mut output)?;
        (output, returned)
    };
    assert_eq!(output.Text, "left\0tailright");
    assert_eq!(
        output.Position,
        Pair {
            X: -2,
            Y: 0x1_0000_0007
        }
    );
    assert_eq!(output.Width.as_ref().unwrap().Value()?, 17);
    assert_eq!(returned.Text, "rightleft\0tail");
    assert_eq!(
        returned.Position,
        Pair {
            X: 8,
            Y: 0xffff_fff9
        }
    );
    assert!(returned.Width.is_none());

    widget.CheckPair(&PairHandler::new(|value, borrowed, output| {
        *output = Pair {
            X: value.X + borrowed.X,
            Y: value.Y + borrowed.Y,
        };
        Ok(Pair {
            X: value.X - borrowed.X,
            Y: value.Y - borrowed.Y,
        })
    }))?;
    widget.CheckLabel(&LabelHandler::new(|value, borrowed, output| {
        *output = Label {
            Text: HSTRING::from_wide(&[&value.Text[..], &borrowed.Text[..]].concat()),
            Position: Pair {
                X: value.Position.X + borrowed.Position.X,
                Y: value.Position.Y + borrowed.Position.Y,
            },
            Width: value.Width.clone(),
        };
        Ok(Label {
            Text: HSTRING::from_wide(&[&borrowed.Text[..], &value.Text[..]].concat()),
            Position: Pair {
                X: value.Position.X - borrowed.Position.X,
                Y: value.Position.Y - borrowed.Position.Y,
            },
            Width: borrowed.Width.clone(),
        })
    }))?;
    Ok(())
}
