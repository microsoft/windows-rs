use windows_registry::*;
use windows_result::*;
use windows_strings::*;

#[test]
fn raw() -> Result<()> {
    let test_key = "software\\windows-rs\\tests\\raw";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key)?;

    unsafe {
        key.raw_set_bytes(w!("raw"), Type::Other(1234), &[1, 2, 3])?;

        let (ty, len) = key.raw_get_info(w!("raw"))?;
        assert_eq!(ty, Type::Other(1234));
        assert_eq!(len, 3);

        let mut bytes = [0; 3];
        key.raw_get_bytes(w!("raw"), &mut bytes)?;
        assert_eq!(bytes, [1, 2, 3]);

        let mut larger = [0; 5];
        let (ty, slice) = key.raw_get_bytes(w!("raw"), &mut larger)?;
        assert_eq!(ty, Type::Other(1234));
        assert_eq!(slice, [1, 2, 3]);
        assert_eq!(larger, [1, 2, 3, 0, 0]);

        let mut bytes = [0; 2];
        let err = key.raw_get_bytes(w!("raw"), &mut bytes).unwrap_err();
        assert_eq!(err.code(), HRESULT(0x800700EAu32 as i32)); // HRESULT_FROM_WIN32(ERROR_INVALID_DATA)
        assert_eq!(err.message(), "More data is available.");
    }

    Ok(())
}
