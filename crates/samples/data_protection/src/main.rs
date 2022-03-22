use windows::{core::*, Security::Cryptography::DataProtection::*, Security::Cryptography::*};

fn main() -> Result<()> {
    let provider = DataProtectionProvider::CreateOverloadExplicit("LOCAL=user")?;

    let buffer = CryptographicBuffer::ConvertStringToBinary("Hello world", BinaryStringEncoding::Utf8)?;

    let protected = provider.ProtectAsync(buffer)?.get()?;

    let unprotected = provider.UnprotectAsync(protected)?.get()?;

    let message = CryptographicBuffer::ConvertBinaryToString(BinaryStringEncoding::Utf8, unprotected)?;

    println!("{}", message);
    Ok(())
}
