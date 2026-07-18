fn main() -> windows::core::Result<()> {
    use windows::{
        Security::Credentials::UI::*, core::*, Win32::*};

    use windows_future::*;

    unsafe {
        let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>()?;

        let window = HWND::default(); // <== replace with your app's window handle

        let operation: IAsyncOperation<UserConsentVerificationResult> =
            interop.RequestVerificationForWindowAsync(window, h!("Hello from Rust"))?;

        let result: UserConsentVerificationResult = operation.join()?;

        println!("{result:?}");

        Ok(())
    }
}
