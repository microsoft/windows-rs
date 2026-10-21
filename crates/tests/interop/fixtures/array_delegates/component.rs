use super::*;
use ArrayDelegates::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "ArrayDelegates.Widget").then(activatable::<Widget, ArrayDelegates::Widget>)
}

#[implement(ArrayDelegates::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn CheckStrings(&self, handler: Ref<StringsHandler>) -> Result<()> {
        let handler = handler.ok()?;
        for input in [
            vec![],
            vec![HSTRING::from("one")],
            vec!["first\0tail".into(), HSTRING::new(), "last".into()],
        ] {
            let mut fill = vec![HSTRING::new(); input.len()];
            let mut output = Array::new();
            let returned = handler.Invoke(&input, &mut fill, &mut output)?;
            check(output.as_slice() == input)?;
            let reversed: Vec<_> = input.iter().rev().cloned().collect();
            check(fill == reversed)?;
            check(returned.as_slice() == reversed)?;
        }
        Ok(())
    }

    fn CheckEntries(&self, handler: Ref<EntriesHandler>) -> Result<()> {
        let handler = handler.ok()?;
        for input in [
            vec![],
            vec![Entry {
                Text: "one".into(),
                Value: 1,
            }],
            vec![
                Entry {
                    Text: "first\0tail".into(),
                    Value: -7,
                },
                Entry {
                    Text: HSTRING::new(),
                    Value: 42,
                },
            ],
        ] {
            let mut fill = vec![Entry::default(); input.len()];
            let mut output = Array::new();
            let returned = handler.Invoke(&input, &mut fill, &mut output)?;
            check(output.as_slice() == input)?;
            let reversed: Vec<_> = input.iter().rev().cloned().collect();
            check(fill == reversed)?;
            check(returned.as_slice() == reversed)?;
        }
        Ok(())
    }
}
