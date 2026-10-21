use super::*;
use NontrivialArrays::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use windows_reference::IReference;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    for count in [0, 1, 3] {
        let mut fill = vec![HSTRING::new(); count];
        let mut output = Array::new();
        let returned = {
            let input = ["first\0tail".into(), HSTRING::new(), "last".into()];
            widget.Strings(&input[..count], &mut fill, &mut output)?
        };
        let expected = ["first\0tail".into(), HSTRING::new(), HSTRING::from("last")];
        assert_eq!(output.as_slice(), &expected[..count]);
        let reversed: Vec<_> = expected[..count].iter().rev().cloned().collect();
        assert_eq!(fill, reversed);
        assert_eq!(returned.as_slice(), reversed);

        let mut fill = vec![Entry::default(); count];
        let mut output = Array::new();
        let returned = {
            let input = entries();
            widget.Entries(&input[..count], &mut fill, &mut output)?
        };
        let expected = entries();
        for index in 0..count {
            check_entry(&output[index], &expected[index])?;
            check_entry(&fill[index], &expected[count - index - 1])?;
            check_entry(&returned[index], &expected[count - index - 1])?;
        }
        assert_eq!(output.len(), count);
        assert_eq!(returned.len(), count);

        let mut fill = vec![None; count];
        let mut output = Array::new();
        let drops = Arc::new(AtomicUsize::new(0));
        let returned = {
            let input = [
                Some(widget.CreateValue(7)?),
                None,
                Some(IValue::from(Value(42, drops.clone()))),
            ];
            widget.Values(&input[..count], &mut fill, &mut output)?
        };
        assert_eq!(drops.load(Ordering::Relaxed), usize::from(count != 3));
        let expected = [Some(7), None, Some(42)];
        let number = |value: &Option<IValue>| value.as_ref().map(IValue::Number).transpose();
        for index in 0..count {
            assert_eq!(number(&output[index])?, expected[index]);
            assert_eq!(number(&fill[index])?, expected[count - index - 1]);
            assert_eq!(number(&returned[index])?, expected[count - index - 1]);
        }
        assert_eq!(output.len(), count);
        assert_eq!(returned.len(), count);
        drop((fill, output, returned));
        assert_eq!(drops.load(Ordering::Relaxed), 1);
    }
    Ok(())
}

fn entries() -> [Entry; 3] {
    [
        Entry {
            Text: "first\0tail".into(),
            Code: 0x03a9,
            Enabled: true,
            Number: Some(IReference::from(-7)),
        },
        Entry::default(),
        Entry {
            Text: "last".into(),
            Code: 0xffff,
            Enabled: false,
            Number: Some(IReference::from(42)),
        },
    ]
}

fn check_entry(actual: &Entry, expected: &Entry) -> Result<()> {
    assert_eq!(actual.Text, expected.Text);
    assert_eq!(actual.Code, expected.Code);
    assert_eq!(actual.Enabled, expected.Enabled);
    assert_eq!(
        actual.Number.as_ref().map(IReference::Value).transpose()?,
        expected
            .Number
            .as_ref()
            .map(IReference::Value)
            .transpose()?
    );
    Ok(())
}

#[implement(IValue)]
struct Value(i32, Arc<AtomicUsize>);

impl Drop for Value {
    fn drop(&mut self) {
        self.1.fetch_add(1, Ordering::Relaxed);
    }
}

impl IValue_Impl for Value_Impl {
    fn Number(&self) -> Result<i32> {
        Ok(self.0)
    }
}
