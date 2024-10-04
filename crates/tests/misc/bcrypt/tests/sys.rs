use windows_sys::{Win32::Foundation::*, Win32::Security::Cryptography::*};

#[test]
fn test() {
    unsafe {
        let mut rng = std::ptr::null_mut();
        assert_eq!(
            STATUS_SUCCESS,
            BCryptOpenAlgorithmProvider(&mut rng, BCRYPT_RNG_ALGORITHM, std::ptr::null(), 0)
        );

        let mut des = std::ptr::null_mut();
        assert_eq!(
            STATUS_SUCCESS,
            BCryptOpenAlgorithmProvider(&mut des, BCRYPT_3DES_ALGORITHM, std::ptr::null(), 0)
        );

        let mut object_len = [0; 4];
        let mut bytes_copied = 0;
        assert_eq!(
            STATUS_SUCCESS,
            BCryptGetProperty(
                des,
                BCRYPT_OBJECT_LENGTH,
                object_len.as_mut_ptr(),
                object_len.len() as u32,
                &mut bytes_copied,
                0
            )
        );
        let object_len = u32::from_le_bytes(object_len);

        let mut shared_secret = vec![0; object_len as usize];
        assert_eq!(
            STATUS_SUCCESS,
            BCryptGenRandom(
                rng,
                shared_secret.as_mut_ptr(),
                shared_secret.len() as u32,
                0
            )
        );

        let mut encrypt_key = std::ptr::null_mut();
        assert_eq!(
            STATUS_SUCCESS,
            BCryptGenerateSymmetricKey(
                des,
                &mut encrypt_key,
                std::ptr::null_mut(),
                0,
                shared_secret.as_ptr(),
                shared_secret.len() as u32,
                0
            )
        );

        let mut block_len = [0; 4];
        assert_eq!(
            STATUS_SUCCESS,
            BCryptGetProperty(
                des,
                BCRYPT_BLOCK_LENGTH,
                block_len.as_mut_ptr(),
                block_len.len() as u32,
                &mut bytes_copied,
                0
            )
        );
        let block_len = u32::from_le_bytes(block_len) as usize;

        let send_message = "I ❤️ Rust";
        let mut send_buffer = vec![0; block_len * send_message.len().div_ceil(block_len)];
        send_buffer[..send_message.len()].copy_from_slice(send_message.as_bytes());

        let mut encrypted_len = 0;
        assert_eq!(
            STATUS_SUCCESS,
            BCryptEncrypt(
                encrypt_key,
                send_buffer.as_ptr(),
                send_buffer.len() as u32,
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
                &mut encrypted_len,
                0
            )
        );

        let mut encrypted = vec![0; encrypted_len as usize];
        assert_eq!(
            STATUS_SUCCESS,
            BCryptEncrypt(
                encrypt_key,
                send_buffer.as_ptr(),
                send_buffer.len() as u32,
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
                encrypted.as_mut_ptr(),
                encrypted.len() as u32,
                &mut encrypted_len,
                0
            )
        );

        let mut decrypt_key = std::ptr::null_mut();
        assert_eq!(
            STATUS_SUCCESS,
            BCryptGenerateSymmetricKey(
                des,
                &mut decrypt_key,
                std::ptr::null_mut(),
                0,
                shared_secret.as_ptr(),
                shared_secret.len() as u32,
                0
            )
        );

        let mut decrypted_len = 0;
        assert_eq!(
            STATUS_SUCCESS,
            BCryptDecrypt(
                decrypt_key,
                encrypted.as_ptr(),
                encrypted.len() as u32,
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
                &mut decrypted_len,
                0
            )
        );

        let mut decrypted = vec![0; decrypted_len as usize];
        assert_eq!(
            STATUS_SUCCESS,
            BCryptDecrypt(
                decrypt_key,
                encrypted.as_ptr(),
                encrypted.len() as u32,
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
                decrypted.as_mut_ptr(),
                decrypted.len() as u32,
                &mut decrypted_len,
                0
            )
        );

        let receive_message =
            std::str::from_utf8(trim_null_end(&decrypted)).expect("Not a valid message");
        assert_eq!(send_message, receive_message);
    }
}

pub const fn trim_null_end(mut bytes: &[u8]) -> &[u8] {
    while let [rest @ .., last] = bytes {
        if *last == 0 {
            bytes = rest;
        } else {
            break;
        }
    }
    bytes
}
