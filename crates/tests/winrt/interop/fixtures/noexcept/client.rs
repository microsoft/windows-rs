use super::*;
use Noexcept::*;
use std::sync::Mutex;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    assert_eq!(widget.Number(), 0);
    widget.SetNumber(123);
    assert_eq!(widget.Number(), 123);

    assert!(widget.Text().is_empty());
    widget.SetText(h!("abc"));
    assert_eq!(widget.Text(), "abc");

    assert!(widget.Next().is_none());
    widget.SetNext(&widget);
    assert_eq!(widget.Next(), Some(widget.cast()?));

    // Clearing releases the widget's reference to itself.
    widget.Clear();
    assert_eq!(widget.Number(), 0);
    assert!(widget.Text().is_empty());
    assert!(widget.Next().is_none());

    widget.Check(&IValue::from(Value::default()))
}

#[implement(IValue)]
#[derive(Default)]
struct Value(Mutex<(i32, HSTRING, Option<IValue>)>);

impl IValue_Impl for Value_Impl {
    fn Number(&self) -> i32 {
        self.0.lock().unwrap().0
    }

    fn SetNumber(&self, value: i32) {
        self.0.lock().unwrap().0 = value;
    }

    fn Text(&self) -> HSTRING {
        self.0.lock().unwrap().1.clone()
    }

    fn SetText(&self, value: &HSTRING) {
        self.0.lock().unwrap().1 = value.clone();
    }

    fn Next(&self) -> Option<IValue> {
        self.0.lock().unwrap().2.clone()
    }

    fn SetNext(&self, value: Ref<IValue>) {
        self.0.lock().unwrap().2 = value.cloned();
    }

    fn Clear(&self) {
        *self.0.lock().unwrap() = Default::default();
    }
}
