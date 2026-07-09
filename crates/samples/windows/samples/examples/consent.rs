fn main() -> windows::core::Result<()> {
    use windows::{Security::Credentials::UI::*, core::*};

    let operation = UserConsentVerifier::RequestVerificationAsync(h!("Hello from Rust"))?;
    let result: UserConsentVerificationResult = operation.join()?;

    println!("{result:?}");

    Ok(())
}
