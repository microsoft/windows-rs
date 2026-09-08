fn main() -> windows::core::Result<()> {
    use windows::{Security::Credentials::UI::*, Win32::*, core::*};
    use windows_future::*;

    unsafe {
        let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>()?;
        let operation: IAsyncOperation<UserConsentVerificationResult> =
            interop.RequestVerificationForWindowAsync(HWND::default(), h!("Hello from Rust"))?;

        let result: UserConsentVerificationResult = operation.join()?;
        println!("{result:?}");
        Ok(())
    }
}
