use windows::Win32::Security::Credentials::{
    CredEnumerateW, CredFree, CREDENTIALW, CRED_ENUMERATE_ALL_CREDENTIALS,
};

fn main() -> windows::core::Result<()> {
    let mut count = 0;
    let mut credentials_ptr = std::ptr::null_mut();
    unsafe {
        CredEnumerateW(
            None,
            Some(CRED_ENUMERATE_ALL_CREDENTIALS),
            &mut count,
            &mut credentials_ptr,
        )?;

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

        CredFree(std::mem::transmute::<*mut *mut _, *const _>(
            credentials_ptr,
        ));
    }

    Ok(())
}
