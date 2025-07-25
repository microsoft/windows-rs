use windows::{core::*, Win32::Foundation::*, Win32::Security::Cryptography::*};

#[test]
fn test() -> Result<(), HRESULT> {
    let status = NTSTATUS::default();
    assert_eq!(status.0, 0);
    assert!(status.is_ok());
    assert!(!status.is_err());
    assert!(status.ok().is_ok());

    let status = STATUS_NOT_FOUND;
    assert_eq!(status.0, -1073741275);
    assert!(!status.is_ok());
    assert!(status.is_err());
    assert!(!status.ok().is_ok());

    let error = status.ok().unwrap_err();
    assert_eq!(error, STATUS_NOT_FOUND);
    assert_eq!(error.to_hresult(), HRESULT(-805305819));

    unsafe {
        let mut provider = BCRYPT_ALG_HANDLE::default();
        BCryptOpenAlgorithmProvider(
            &mut provider,
            w!("RNG"),
            None,
            BCRYPT_OPEN_ALGORITHM_PROVIDER_FLAGS::default(),
        )
        .ok()?;

        let mut random = GUID::zeroed();
        let bytes = std::slice::from_raw_parts_mut(
            &mut random as *mut _ as *mut u8,
            std::mem::size_of::<GUID>(),
        );

        BCryptGenRandom(Some(provider), bytes, Default::default()).ok()?;

        assert_ne!(random, GUID::zeroed());
    }

    Ok(())
}

// A test version of BCryptVerifySignature to ensure that we can handle alternative status codes
// in a reasonable manner with the help of `Into`.

#[allow(non_snake_case)]
fn BCryptVerifySignature(status: NTSTATUS) -> Result<(), NTSTATUS> {
    status.ok()
}

fn is_valid(status: NTSTATUS) -> Result<bool, NTSTATUS> {
    match BCryptVerifySignature(status) {
        Err(e) => {
            if e == STATUS_INVALID_SIGNATURE {
                Ok(false)
            } else {
                Err(e)
            }
        }
        _ => Ok(true),
    }
}

#[test]
fn test_verify() -> Result<(), NTSTATUS> {
    assert!(is_valid(STATUS_SUCCESS)?);
    assert!(!(is_valid(STATUS_INVALID_SIGNATURE)?));
    assert!(is_valid(STATUS_NOT_FOUND).is_err());

    Ok(())
}
