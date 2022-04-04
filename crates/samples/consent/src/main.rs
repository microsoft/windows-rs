use windows::{core::*, Foundation::*, Security::Credentials::UI::*, Win32::Foundation::*, Win32::System::WinRT::*};

fn main() -> Result<()> {
    unsafe {
        let interop = factory::<UserConsentVerifier, IUserConsentVerifierInterop>(None)?;

        let window = HWND(0); // <== replace with your app's window handle

        let operation: IAsyncOperation<UserConsentVerificationResult> = interop.RequestVerificationForWindowAsync(window, "Hello from Rust")?;

        let result: UserConsentVerificationResult = operation.get()?;

        println!("{:?}", result);

        Ok(())
    }
}
