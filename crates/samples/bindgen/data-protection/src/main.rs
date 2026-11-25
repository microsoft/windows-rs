#[expect(nonstandard_style)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() -> windows_core::Result<()> {
    use bindings::*;

    let provider = DataProtectionProvider::CreateOverloadExplicit("LOCAL=user")?;
    let unprotected =
        CryptographicBuffer::ConvertStringToBinary("Hello world", BinaryStringEncoding::Utf8)?;

    let protected = provider.ProtectAsync(&unprotected)?.join()?;
    println!("protected {} bytes", protected.Length()?);
    let unprotected = provider.UnprotectAsync(&protected)?.join()?;

    let message =
        CryptographicBuffer::ConvertBinaryToString(BinaryStringEncoding::Utf8, &unprotected)?;
    println!("{message}");
    Ok(())
}
