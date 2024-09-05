use windows_registry::*;
use windows_strings::*;

#[test]
fn hstring() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\hstring";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_hstring("hstring", h!("simple"))?;
    assert_eq!(&key.get_hstring("hstring")?, h!("simple"));
    assert_eq!(
        unsafe { key.raw_get_info(w!("hstring"))? },
        (Type::String, 14)
    );

    // You can embed nulls.
    key.set_hstring("hstring", h!("hstring\0value\0"))?;

    // And get_hstring will only trim any trailing nulls.
    let value: HSTRING = key.get_hstring("hstring")?;
    assert_eq!(&value, h!("hstring\0value"));

    Ok(())
}
