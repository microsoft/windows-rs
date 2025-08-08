use windows::{
    core::*, Security::Credentials::UI::*, Win32::Foundation::*, Win32::System::WinRT::*,
};

use windows_future::*;

fn main() -> Result<()> {
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
