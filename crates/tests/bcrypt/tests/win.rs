use windows::{core::*, Win32::Security::Cryptography::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let mut rng = Default::default();
        BCryptOpenAlgorithmProvider(&mut rng, BCRYPT_RNG_ALGORITHM, None, Default::default())?;

        let mut des = Default::default();
        BCryptOpenAlgorithmProvider(&mut des, BCRYPT_3DES_ALGORITHM, None, Default::default())?;

        let mut object_len = [0; 4];
        let mut bytes_copied = 0;
        BCryptGetProperty(
            des,
            BCRYPT_OBJECT_LENGTH,
            Some(&mut object_len),
            &mut bytes_copied,
            0,
        )?;
        let object_len = u32::from_le_bytes(object_len);

        let mut shared_secret = vec![0; object_len as usize];
        BCryptGenRandom(rng, &mut shared_secret, Default::default())?;

        let mut encrypt_key = Default::default();
        BCryptGenerateSymmetricKey(des, &mut encrypt_key, None, &shared_secret, 0)?;

        let mut block_len = [0; 4];
        BCryptGetProperty(
            des,
            BCRYPT_BLOCK_LENGTH,
            Some(&mut block_len),
            &mut bytes_copied,
            0,
        )?;
        let block_len = u32::from_le_bytes(block_len) as usize;

        let send_message = "I ❤️ Rust";
        let mut send_buffer =
            vec![0; block_len * ((send_message.len() + (block_len - 1)) / block_len)];
        send_buffer[..send_message.len()].copy_from_slice(send_message.as_bytes());

        let mut encrypted_len = 0;
        BCryptEncrypt(
            encrypt_key,
            Some(&send_buffer),
            None,
            None,
            None,
            &mut encrypted_len,
            Default::default(),
        )?;

        let mut encrypted = vec![0; encrypted_len as usize];
        BCryptEncrypt(
            encrypt_key,
            Some(&send_buffer),
            None,
            None,
            Some(&mut encrypted),
            &mut encrypted_len,
            Default::default(),
        )?;

        let mut decrypt_key = Default::default();
        BCryptGenerateSymmetricKey(des, &mut decrypt_key, None, &shared_secret, 0)?;

        let mut decrypted_len = 0;
        BCryptDecrypt(
            decrypt_key,
            Some(&encrypted),
            None,
            None,
            None,
            &mut decrypted_len,
            Default::default(),
        )?;

        let mut decrypted = vec![0; decrypted_len as usize];
        BCryptDecrypt(
            decrypt_key,
            Some(&encrypted),
            None,
            None,
            Some(&mut decrypted),
            &mut decrypted_len,
            Default::default(),
        )?;

        let receive_message =
            std::str::from_utf8(trim_null_end(&decrypted)).expect("Not a valid message");
        assert_eq!(send_message, receive_message);
    }
    Ok(())
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
