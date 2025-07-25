use windows::Win32::Foundation::{NTSTATUS, STATUS_INVALID_ACL, STATUS_SUCCESS, S_OK};
use windows_result::*;

#[test]
fn test() {
    assert!(STATUS_SUCCESS.is_ok());
    assert!(!STATUS_INVALID_ACL.is_ok());

    assert!(!STATUS_SUCCESS.is_err());
    assert!(STATUS_INVALID_ACL.is_err());

    STATUS_SUCCESS.unwrap();
    let result: Result<(), NTSTATUS> = STATUS_SUCCESS.ok();
    assert!(result.is_ok());

    assert_eq!(STATUS_SUCCESS.to_hresult(), S_OK);

    // Tests convertibility.
    a().unwrap();
    b().unwrap();
}

#[test]
#[should_panic(expected = "NTSTATUS 0xC0000077")]
fn test_panic() {
    STATUS_INVALID_ACL.unwrap();
}

fn a() -> Result<(), Error> {
    fn a() -> Result<(), Error> {
        Ok(())
    }

    fn b() -> Result<(), HRESULT> {
        Ok(())
    }

    fn c() -> Result<(), NTSTATUS> {
        Ok(())
    }

    a()?;
    b()?;
    c()?;

    Ok(())
}

fn b() -> Result<(), HRESULT> {
    fn a() -> Result<(), Error> {
        Ok(())
    }

    fn b() -> Result<(), HRESULT> {
        Ok(())
    }

    fn c() -> Result<(), NTSTATUS> {
        Ok(())
    }

    a()?;
    b()?;
    c()?;

    Ok(())
}
