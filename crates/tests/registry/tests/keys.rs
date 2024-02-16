use windows_registry::*;
use windows_result::*;

#[test]
fn keys() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\keys";
    _ = CURRENT_USER.remove_tree(test_key);

    let key = CURRENT_USER.create(test_key)?;
    key.create("one")?;
    key.create("two")?;
    key.create("three")?;

    let names: Vec<String> = key.keys()?.collect();
    assert_eq!(names, ["one", "three", "two"]);

    let err = key.open("missing").unwrap_err();
    assert_eq!(err.code(), HRESULT(0x80070002u32 as i32)); // HRESULT_FROM_WIN32(ERROR_FILE_NOT_FOUND)
    assert_eq!(err.message(), "The system cannot find the file specified.");

    let key = key.open("one")?;
    let err = key.set_u32("name", 123).unwrap_err();
    assert_eq!(err.code(), HRESULT(0x80070005u32 as i32)); // E_ACCESSDENIED
    assert_eq!(err.message(), "Access is denied.");

    Ok(())
}
