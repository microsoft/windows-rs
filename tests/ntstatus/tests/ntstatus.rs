use test_ntstatus::{
    Windows::Win32::Foundation::{NTSTATUS, STATUS_NOT_FOUND},
    Windows::Win32::Security::Cryptography::Core::{BCryptGenRandom, BCryptOpenAlgorithmProvider},
};

use windows::{Guid, Result, HRESULT};

#[test]
fn test() -> Result<()> {
    let status = NTSTATUS::default();
    assert_eq!(status.0, 0);
    assert_eq!(status.is_ok(), true);
    assert_eq!(status.is_err(), false);
    assert_eq!(status.ok().is_ok(), true);

    let status = STATUS_NOT_FOUND;
    assert_eq!(status.0, 0xC000_0225);
    assert_eq!(status.is_ok(), false);
    assert_eq!(status.is_err(), true);
    assert_eq!(status.ok().is_ok(), false);

    let error = status.ok().unwrap_err();
    assert_eq!(error.code(), HRESULT(0xD000_0225));

    unsafe {
        let mut provider = std::ptr::null_mut();
        BCryptOpenAlgorithmProvider(&mut provider, "RNG", None, Default::default())?;

        let mut random = Guid::zeroed();

        BCryptGenRandom(
            provider,
            &mut random as *mut _ as _,
            std::mem::size_of::<Guid>() as _,
            0,
        )?;

        assert_ne!(random, Guid::zeroed());
    }

    Ok(())
}
