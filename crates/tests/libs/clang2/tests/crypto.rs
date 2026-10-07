#![cfg(target_env = "msvc")]

#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/crypto.rs"));
}
use bindings::*;

type DeriveKey =
    unsafe fn(BCRYPT_ALG_HANDLE, Option<&[u8]>, Option<&[u8]>, u64, *mut u8, u32, u32) -> i32;
const _: DeriveKey = BCryptDeriveKeyPBKDF2;
const _: unsafe fn(BCRYPT_ALG_HANDLE, u32) -> i32 = BCryptCloseAlgorithmProvider;

#[test]
fn real_bcrypt_optional_buffers_produce_known_keys() {
    let mut algorithm = BCRYPT_ALG_HANDLE::default();
    let status = unsafe {
        BCryptOpenAlgorithmProvider(
            &raw mut algorithm,
            windows_core::w!("SHA256"),
            windows_core::PCWSTR::null(),
            BCRYPT_ALG_HANDLE_HMAC_FLAG.try_into().unwrap(),
        )
    };
    assert_eq!(status, 0);
    struct Algorithm(BCRYPT_ALG_HANDLE);
    impl Drop for Algorithm {
        fn drop(&mut self) {
            let status = unsafe { BCryptCloseAlgorithmProvider(self.0, 0) };
            assert_eq!(status, 0);
        }
    }
    let algorithm = Algorithm(algorithm);
    for (password, salt, expected) in [
        (
            Some(b"password".as_slice()),
            Some(b"salt".as_slice()),
            "120fb6cffcf8b32c43e7225256c4f837a86548c92ccc35480805987cb70be17b",
        ),
        (
            None,
            None,
            "f7ce0b653d2d72a4108cf5abe912ffdd777616dbbb27a70e8204f3ae2d0f6fad",
        ),
        (
            Some(b"".as_slice()),
            Some(b"".as_slice()),
            "f7ce0b653d2d72a4108cf5abe912ffdd777616dbbb27a70e8204f3ae2d0f6fad",
        ),
        (
            Some(b"pass\0word".as_slice()),
            Some(b"sa\0lt".as_slice()),
            "f9b77371b5a5a07700e7d9cf93dffb8f5e4cae1503c2c9fce223fa8b299061d8",
        ),
    ] {
        let mut output = [0xcc; 34];
        let status = unsafe {
            BCryptDeriveKeyPBKDF2(
                algorithm.0,
                password,
                salt,
                1,
                output[1..33].as_mut_ptr(),
                32,
                0,
            )
        };
        assert_eq!(status, 0);
        assert_eq!(output[0], 0xcc);
        assert_eq!(output[33], 0xcc);
        let hex: String = output[1..33]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        assert_eq!(hex, expected);
        let status = unsafe {
            BCryptDeriveKeyPBKDF2(
                algorithm.0,
                password,
                salt,
                0,
                output[1..33].as_mut_ptr(),
                32,
                0,
            )
        };
        assert!(status < 0);
        assert_eq!(output[0], 0xcc);
        assert_eq!(output[33], 0xcc);
    }
}
