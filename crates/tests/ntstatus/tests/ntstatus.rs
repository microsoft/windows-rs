use windows::{core::*, Win32::Foundation::*, Win32::Security::Cryptography::*};

#[test]
fn test() -> Result<()> {
    let status = NTSTATUS::default();
    assert_eq!(status.0, 0);
    assert_eq!(status.is_ok(), true);
    assert_eq!(status.is_err(), false);
    assert_eq!(status.ok().is_ok(), true);

    let status = STATUS_NOT_FOUND;
    assert_eq!(status.0, -1073741275);
    assert_eq!(status.is_ok(), false);
    assert_eq!(status.is_err(), true);
    assert_eq!(status.ok().is_ok(), false);

    let error = status.ok().unwrap_err();
    assert_eq!(error.code(), HRESULT(-805305819));

    unsafe {
        let mut provider = BCRYPT_ALG_HANDLE::default();
        BCryptOpenAlgorithmProvider(&mut provider, "RNG", None, Default::default())?;

        let mut random = GUID::zeroed();

        BCryptGenRandom(provider, &mut random as *mut _ as _, core::mem::size_of::<GUID>() as _, 0)?;

        assert_ne!(random, GUID::zeroed());
    }

    Ok(())
}

// A test version of BCryptVerifySignature to ensure that we can handle alternative status codes
// in a reasonable manner with the help of `Into`.

#[allow(non_snake_case)]
fn BCryptVerifySignature(status: NTSTATUS) -> Result<()> {
    status.ok()
}

fn is_valid(status: NTSTATUS) -> Result<bool> {
    match BCryptVerifySignature(status) {
        Err(e) => {
            if e.code() == STATUS_INVALID_SIGNATURE.into() {
                Ok(false)
            } else {
                Err(e)
            }
        }
        _ => Ok(true),
    }
}

#[test]
fn test_verify() -> Result<()> {
    assert_eq!(is_valid(STATUS_SUCCESS)?, true);
    assert_eq!(is_valid(STATUS_INVALID_SIGNATURE)?, false);
    assert_eq!(is_valid(STATUS_NOT_FOUND).is_err(), true);

    Ok(())
}
