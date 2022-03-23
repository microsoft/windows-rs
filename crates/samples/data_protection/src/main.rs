use windows::{core::*, Security::Cryptography::DataProtection::*, Security::Cryptography::*, Storage::Streams::*, Win32::System::WinRT::*};

fn main() -> std::io::Result<()> {
    let provider = DataProtectionProvider::CreateOverloadExplicit("LOCAL=user")?;
    let unprotected = CryptographicBuffer::ConvertStringToBinary("Hello world", BinaryStringEncoding::Utf8)?;

    let protected = provider.ProtectAsync(unprotected)?.get()?;
    let protected_bytes = unsafe { as_mut_bytes(&protected)? };
    std::fs::write("secret.bin", protected_bytes)?;

    let protected_bytes = std::fs::read("secret.bin")?;
    let protected = CryptographicBuffer::CreateFromByteArray(&protected_bytes)?;
    let unprotected = provider.UnprotectAsync(protected)?.get()?;

    let message = CryptographicBuffer::ConvertBinaryToString(BinaryStringEncoding::Utf8, unprotected)?;
    println!("{}", message);
    Ok(())
}

unsafe fn as_mut_bytes(buffer: &IBuffer) -> Result<&mut [u8]> {
    let interop = buffer.cast::<IBufferByteAccess>()?;
    let data = interop.Buffer()?;
    Ok(std::slice::from_raw_parts_mut(data, buffer.Length()? as _))
}
