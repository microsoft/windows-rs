fn main() -> windows::core::Result<()> {
    use windows::wincred::{CRED_ENUMERATE_ALL_CREDENTIALS, CREDENTIALW, CredEnumerateW, CredFree};

    let mut count = 0;
    let mut credentials_ptr = std::ptr::null_mut();
    unsafe {
        CredEnumerateW(
            None,
            Some(CRED_ENUMERATE_ALL_CREDENTIALS),
            &mut count,
            &mut credentials_ptr,
        )
        .ok()?;

        let credentials =
            std::slice::from_raw_parts::<&CREDENTIALW>(credentials_ptr as _, count as usize);

        for credential in credentials {
            println!("/* CREDENTIALW */");
            println!("Type: {:?}", credential.Type);
            if !credential.TargetName.is_null() {
                println!("Target: {}", credential.TargetName.display());
            }
            if !credential.UserName.is_null() {
                println!("User name: {}", credential.UserName.display());
            }
            println!();
        }

        CredFree(credentials_ptr as *const _);
    }

    Ok(())
}
