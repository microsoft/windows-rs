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
type CreateHash = unsafe fn(
    BCRYPT_ALG_HANDLE,
    *mut BCRYPT_HASH_HANDLE,
    Option<*mut u8>,
    u32,
    Option<&[u8]>,
    u32,
) -> i32;
const _: CreateHash = BCryptCreateHash;
const _: unsafe fn(BCRYPT_HASH_HANDLE, &[u8], u32) -> i32 = BCryptHashData;
const _: unsafe fn(BCRYPT_HASH_HANDLE, *mut u8, u32, u32) -> i32 = BCryptFinishHash;
const _: unsafe fn(BCRYPT_HASH_HANDLE) -> i32 = BCryptDestroyHash;

struct Algorithm(BCRYPT_ALG_HANDLE);

impl Algorithm {
    fn open(flags: u32) -> Self {
        let mut handle = BCRYPT_ALG_HANDLE::default();
        let status = unsafe {
            BCryptOpenAlgorithmProvider(&raw mut handle, windows_core::w!("SHA256"), None, flags)
        };
        assert_eq!(status, 0);
        assert!(!handle.is_null());
        Self(handle)
    }

    fn property(&self, name: windows_core::PCWSTR) -> u32 {
        let mut written = 0;
        assert_eq!(
            unsafe { BCryptGetProperty(self.0, name, None, 0, &raw mut written, 0) },
            0
        );
        assert_eq!(written, 4);
        let mut buffer = [0xcc; 10];
        assert_eq!(
            unsafe {
                BCryptGetProperty(
                    self.0,
                    name,
                    Some(buffer[1..9].as_mut_ptr()),
                    8,
                    &raw mut written,
                    0,
                )
            },
            0
        );
        assert_eq!(written, 4);
        assert_eq!(buffer[0], 0xcc);
        assert_eq!(buffer[9], 0xcc);
        u32::from_ne_bytes(buffer[1..5].try_into().unwrap())
    }
}

impl Drop for Algorithm {
    fn drop(&mut self) {
        assert_eq!(unsafe { BCryptCloseAlgorithmProvider(self.0, 0) }, 0);
    }
}

struct Hash<'a> {
    handle: BCRYPT_HASH_HANDLE,
    _algorithm: &'a Algorithm,
    storage: Option<Vec<u8>>,
}

impl<'a> Hash<'a> {
    fn new(algorithm: &'a Algorithm, storage_size: Option<u32>, secret: Option<&[u8]>) -> Self {
        let mut storage = storage_size.map(|size| vec![0xcc; size as usize + 2]);
        let mut handle = BCRYPT_HASH_HANDLE::default();
        let status = unsafe {
            BCryptCreateHash(
                algorithm.0,
                &raw mut handle,
                storage.as_mut().map(|buffer| buffer[1..].as_mut_ptr()),
                storage_size.unwrap_or(0),
                secret,
                0,
            )
        };
        assert_eq!(status, 0);
        assert!(!handle.is_null());
        Self {
            handle,
            _algorithm: algorithm,
            storage,
        }
    }
}

impl Drop for Hash<'_> {
    fn drop(&mut self) {
        // CNG retains the caller's object buffer until the hash is destroyed.
        assert_eq!(unsafe { BCryptDestroyHash(self.handle) }, 0);
        if let Some(storage) = &self.storage {
            assert_eq!(storage[0], 0xcc);
            assert_eq!(*storage.last().unwrap(), 0xcc);
        }
    }
}

#[test]
fn real_bcrypt_hash_lifecycle_preserves_buffers_and_ownership() {
    for (secret, data, expected) in [
        (
            None,
            b"".as_slice(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        ),
        (
            None,
            b"abc".as_slice(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ),
        (
            None,
            b"a\0bc\0d".as_slice(),
            "702f90638317c6b719064288228a8f775295013b1d77e1712ea3f3b653c45339",
        ),
        (
            Some(b"key\0bytes".as_slice()),
            b"a\0bc\0d".as_slice(),
            "da14e96230d317a38a773598683c884d1c5fe3d4f8e7048a0d4e98d8fe443437",
        ),
    ] {
        let algorithm = Algorithm::open(if secret.is_some() {
            BCRYPT_ALG_HANDLE_HMAC_FLAG.try_into().unwrap()
        } else {
            0
        });
        let object_size = algorithm.property(windows_core::w!("ObjectLength"));
        let digest_size = algorithm.property(windows_core::w!("HashDigestLength"));
        assert!(object_size > 0);
        assert_eq!(digest_size, 32);
        for storage in [None, Some(object_size)] {
            let hash = Hash::new(&algorithm, storage, secret);
            assert_eq!(unsafe { BCryptHashData(hash.handle, &[], 0) }, 0);
            for chunk in data.chunks(2) {
                assert_eq!(unsafe { BCryptHashData(hash.handle, chunk, 0) }, 0);
            }
            let mut digest = [0xcc; 34];
            assert_eq!(
                unsafe {
                    BCryptFinishHash(hash.handle, digest[1..33].as_mut_ptr(), digest_size, 0)
                },
                0
            );
            assert_eq!(digest[0], 0xcc);
            assert_eq!(digest[33], 0xcc);
            let hex: String = digest[1..33]
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect();
            assert_eq!(hex, expected);
        }
    }
}

#[test]
fn real_bcrypt_failures_preserve_status_and_output_bounds() {
    const BUFFER_TOO_SMALL: i32 = 0xc0000023u32 as i32;
    const INVALID_PARAMETER: i32 = 0xc000000du32 as i32;
    const NOT_SUPPORTED: i32 = 0xc00000bbu32 as i32;
    let algorithm = Algorithm::open(0);
    let mut result = 0;
    let mut output = [0xcc; 3];
    assert_eq!(
        unsafe {
            BCryptGetProperty(
                algorithm.0,
                windows_core::w!("ObjectLength"),
                Some(output[1..2].as_mut_ptr()),
                1,
                &raw mut result,
                0,
            )
        },
        BUFFER_TOO_SMALL
    );
    assert_eq!(output[0], 0xcc);
    assert_eq!(output[2], 0xcc);
    assert_eq!(
        unsafe {
            BCryptGetProperty(
                algorithm.0,
                windows_core::w!("Clang2MissingProperty"),
                None,
                0,
                &raw mut result,
                0,
            )
        },
        NOT_SUPPORTED
    );

    let mut handle = BCRYPT_HASH_HANDLE::default();
    assert_eq!(
        unsafe {
            BCryptCreateHash(
                algorithm.0,
                &raw mut handle,
                Some(output[1..2].as_mut_ptr()),
                1,
                None,
                0,
            )
        },
        BUFFER_TOO_SMALL
    );
    assert_eq!(output[0], 0xcc);
    assert_eq!(output[2], 0xcc);

    let hash = Hash::new(&algorithm, None, None);
    let mut digest = [0xcc; 33];
    assert_eq!(
        unsafe { BCryptFinishHash(hash.handle, digest[1..32].as_mut_ptr(), 31, 0) },
        INVALID_PARAMETER
    );
    assert_eq!(digest[0], 0xcc);
    assert_eq!(digest[32], 0xcc);
}

#[test]
fn real_bcrypt_optional_buffers_produce_known_keys() {
    let algorithm = Algorithm::open(BCRYPT_ALG_HANDLE_HMAC_FLAG.try_into().unwrap());
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
