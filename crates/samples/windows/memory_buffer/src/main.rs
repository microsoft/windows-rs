use windows::{core::*, Foundation::*, Win32::System::WinRT::IMemoryBufferByteAccess};

// This example illustrates how to use IMemoryBufferByteAccess to access the underlying buffer
// owned by the MemoryBuffer/IMemoryBufferReference. Note that this is inherently unsafe as
// this function does not perform borrow/lifetime checking. The caller must ensure that the
// IMemoryBufferReference remains alive and that that buffer is not shared across threads.

#[expect(clippy::mut_from_ref)]
unsafe fn as_mut_slice(buffer: &IMemoryBufferReference) -> Result<&mut [u8]> {
    let interop = buffer.cast::<IMemoryBufferByteAccess>()?;
    let mut data = std::ptr::null_mut();
    let mut len = 0;

    unsafe {
        interop.GetBuffer(&mut data, &mut len)?;
        Ok(std::slice::from_raw_parts_mut(data, len as usize))
    }
}

fn main() -> Result<()> {
    let buffer = MemoryBuffer::Create(11)?;
    let reference = buffer.CreateReference()?;
    assert_eq!(reference.Capacity()?, 11);

    // Write to buffer...
    {
        let slice = unsafe { as_mut_slice(&reference)? };
        slice.copy_from_slice(b"hello world");
    }

    // Read from buffer...
    {
        let slice = unsafe { as_mut_slice(&reference)? };
        assert_eq!(slice, b"hello world");
    }

    Ok(())
}
