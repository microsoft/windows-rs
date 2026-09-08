fn main() -> windows::core::Result<()> {
    use windows::{Security::Cryptography::DataProtection::*, Security::Cryptography::*, core::*};

    let provider = DataProtectionProvider::CreateOverloadExplicit(h!("LOCAL=user"))?;
    let unprotected =
        CryptographicBuffer::ConvertStringToBinary(h!("Hello world"), BinaryStringEncoding::Utf8)?;

    let protected = provider.ProtectAsync(&unprotected)?.join()?;
    println!("protected {} bytes", protected.Length()?);
    let unprotected = provider.UnprotectAsync(&protected)?.join()?;

    let message =
        CryptographicBuffer::ConvertBinaryToString(BinaryStringEncoding::Utf8, &unprotected)?;
    println!("{message:?}");
    Ok(())
}
