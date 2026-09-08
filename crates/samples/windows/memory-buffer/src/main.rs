fn main() -> windows::core::Result<()> {
    use windows::{Foundation::*, Win32::IMemoryBufferByteAccess, core::*};

    #[expect(clippy::mut_from_ref)]
    /// # Safety
    /// The reference must remain alive and exclusive while the returned slice is used.
    unsafe fn as_mut_slice(buffer: &IMemoryBufferReference) -> Result<&mut [u8]> {
        let interop = buffer.cast::<IMemoryBufferByteAccess>()?;
        let mut data = std::ptr::null_mut();
        let mut len = 0;

        unsafe {
            interop.GetBuffer(&mut data, &mut len).ok()?;
            Ok(std::slice::from_raw_parts_mut(data, len as usize))
        }
    }

    let buffer = MemoryBuffer::Create(11)?;
    let reference = buffer.CreateReference()?;
    assert_eq!(reference.Capacity()?, 11);

    {
        let slice = unsafe { as_mut_slice(&reference)? };
        slice.copy_from_slice(b"hello world");
    }

    {
        let slice = unsafe { as_mut_slice(&reference)? };
        assert_eq!(slice, b"hello world");
    }

    Ok(())
}
