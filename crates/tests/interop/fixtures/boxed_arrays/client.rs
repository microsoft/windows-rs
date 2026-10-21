use super::*;
use BoxedArrays::*;
use Windows::Foundation::{IReferenceArray, PropertyValue};

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    for count in [0, 3] {
        let numbers = [i32::MIN, 0, i32::MAX];
        let numbers = &numbers[..count];
        let boxed = widget.BoxNumbers(numbers)?;
        assert_eq!(
            boxed.cast::<IReferenceArray<i32>>()?.Value()?.as_slice(),
            numbers
        );
        let boxed = PropertyValue::CreateInt32Array(numbers)?;
        assert_eq!(widget.Numbers(&boxed)?.as_slice(), numbers);

        let booleans = [true, false, true];
        let booleans = &booleans[..count];
        let boxed = widget.BoxBooleans(booleans)?;
        assert_eq!(
            boxed.cast::<IReferenceArray<bool>>()?.Value()?.as_slice(),
            booleans
        );
        let boxed = PropertyValue::CreateBooleanArray(booleans)?;
        assert_eq!(widget.Booleans(&boxed)?.as_slice(), booleans);

        let strings = [HSTRING::from("first\0tail"), HSTRING::new(), "last".into()];
        let strings = &strings[..count];
        let boxed = widget.BoxStrings(strings)?;
        assert_eq!(
            boxed
                .cast::<IReferenceArray<HSTRING>>()?
                .Value()?
                .as_slice(),
            strings
        );
        let boxed = PropertyValue::CreateStringArray(strings)?;
        assert_eq!(widget.Strings(&boxed)?.as_slice(), strings);
    }

    let strings = PropertyValue::CreateStringArray(&[HSTRING::from("not a number")])?;
    assert_eq!(widget.Numbers(&strings).unwrap_err().code(), E_NOINTERFACE);
    Ok(())
}
