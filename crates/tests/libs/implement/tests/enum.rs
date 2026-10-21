#![cfg(windows)]
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use windows::{Foundation::*, core::*};

#[implement(IStringable)]
#[derive(Default)]
enum Value {
    #[default]
    Empty,
    Number(i32),
    Text {
        value: String,
        _drop: DropCounter,
    },
}

impl IStringable_Impl for Value_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok(match &**self {
            Value::Empty => "empty".into(),
            Value::Number(value) => value.to_string().into(),
            Value::Text { value, .. } => value.into(),
        })
    }
}

struct DropCounter(Arc<AtomicUsize>);

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn variants_and_final_release() -> Result<()> {
    let empty: IStringable = Value::default().into();
    assert_eq!(empty.ToString()?, "empty");

    let number: IStringable = Value::Number(42).into();
    assert_eq!(number.ToString()?, "42");

    let drops = Arc::new(AtomicUsize::new(0));
    let text: IStringable = Value::Text {
        value: "owned".into(),
        _drop: DropCounter(drops.clone()),
    }
    .into();
    let identity: IUnknown = text.cast()?;
    let retained: IStringable = identity.cast()?;
    drop(text);
    drop(identity);
    assert_eq!(drops.load(Ordering::Relaxed), 0);
    assert_eq!(retained.ToString()?, "owned");
    drop(retained);
    assert_eq!(drops.load(Ordering::Relaxed), 1);
    Ok(())
}

#[implement(IStringable)]
pub enum GenericValue<'a, T, const N: usize>
where
    T: AsRef<str> + Send + Sync + 'a,
{
    Text(&'a T),
    Bytes([u8; N]),
}

impl<'a, T, const N: usize> IStringable_Impl for GenericValue_Impl<'a, T, N>
where
    T: AsRef<str> + Send + Sync + 'a,
{
    fn ToString(&self) -> Result<HSTRING> {
        Ok(match &**self {
            GenericValue::Text(value) => value.as_ref().into(),
            GenericValue::Bytes(value) => format!("{value:?}").into(),
        })
    }
}

#[test]
fn generics_and_borrowed_data() -> Result<()> {
    let text = String::from("borrowed");
    let borrowed: IStringable = GenericValue::<String, 3>::Text(&text).into();
    assert_eq!(borrowed.ToString()?, "borrowed");

    let bytes: IStringable = GenericValue::<String, 3>::Bytes([1, 2, 3]).into();
    assert_eq!(bytes.ToString()?, "[1, 2, 3]");
    Ok(())
}
