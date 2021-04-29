use bindings::{Windows::Foundation::*, Windows::Win32::WinRT::IMemoryBufferByteAccess};
use windows::*;

fn as_slice(buffer: &IMemoryBufferReference) -> Result<&mut [u8]> {
    let interop = buffer.cast::<IMemoryBufferByteAccess>()?;
    let mut data = std::ptr::null_mut();
    let mut len = 0;

    unsafe {
        interop.GetBuffer(&mut data, &mut len).ok()?;
        Ok(std::slice::from_raw_parts_mut(data, len as _))
    }
}

fn main() -> Result<()> {
    let buffer = MemoryBuffer::Create(11)?;
    let reference = buffer.CreateReference()?;
    assert_eq!(reference.Capacity()?, 11);

    // Write to buffer...
    {
        let slice = as_slice(&reference)?;
        slice.copy_from_slice(b"hello world");
    }

    // Read from buffer...
    {
        let slice = as_slice(&reference)?;
        assert_eq!(slice, b"hello world");
    }

    Ok(())
}
