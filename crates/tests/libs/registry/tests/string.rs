use std::borrow::Cow;
use windows_registry::*;
use windows_strings::*;

#[test]
#[expect(clippy::unnecessary_to_owned)] // intentionally testing convertible types
fn string() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\string";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    key.set_string("string", "value")?;
    assert_eq!(key.get_type("string")?, Type::String);
    assert_eq!(key.get_string("string")?, "value");
    assert_eq!(key.get_hstring("string")?, "value");
    assert_eq!(key.get_multi_string("string")?, ["value".to_string()]);

    key.set_string("string_different_types", Cow::Owned("value".to_string()))?;
    assert_eq!(key.get_type("string_different_types")?, Type::String);
    assert_eq!(key.get_string("string_different_types")?, "value");
    assert_eq!(key.get_hstring("string_different_types")?, "value");
    assert_eq!(
        key.get_multi_string("string_different_types")?,
        ["value".to_string()],
    );

    key.set_hstring("hstring", h!("value"))?;
    assert_eq!(key.get_type("hstring")?, Type::String);
    assert_eq!(key.get_string("hstring")?, "value");
    assert_eq!(key.get_hstring("hstring")?, "value");
    assert_eq!(key.get_multi_string("hstring")?, ["value".to_string()]);

    key.set_expand_string("expand_string", "value")?;
    assert_eq!(key.get_type("expand_string")?, Type::ExpandString);
    assert_eq!(key.get_string("expand_string")?, "value");
    assert_eq!(key.get_hstring("expand_string")?, "value");
    assert_eq!(key.get_multi_string("expand_string")?, ["value"]);

    key.set_expand_string("expand_string_different_types".to_string(), "value")?;
    assert_eq!(
        key.get_type("expand_string_different_types")?,
        Type::ExpandString
    );
    assert_eq!(key.get_string("expand_string_different_types")?, "value");
    assert_eq!(key.get_hstring("expand_string_different_types")?, "value");
    assert_eq!(
        key.get_multi_string("expand_string_different_types")?,
        ["value"],
    );

    key.set_expand_hstring("expand_hstring", h!("value"))?;
    assert_eq!(key.get_type("expand_hstring")?, Type::ExpandString);
    assert_eq!(key.get_string("expand_hstring")?, "value");
    assert_eq!(key.get_hstring("expand_hstring")?, "value");
    assert_eq!(key.get_multi_string("expand_hstring")?, ["value"]);

    Ok(())
}
