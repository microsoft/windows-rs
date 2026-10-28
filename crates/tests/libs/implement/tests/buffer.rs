#![cfg(windows)]
use windows::Storage::Streams::Buffer;
use windows::core::*;
use windows::{Win32::byte, Win32::*};

#[implement(IBufferByteAccess)]
struct TestBuffer(std::cell::UnsafeCell<Vec<u8>>);

impl IBufferByteAccess_Impl for TestBuffer_Impl {
    fn Buffer(&self, value: *mut *mut byte) -> Result<()> {
        unsafe { *value = (*self.0.get()).as_mut_ptr().cast() };
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    let object: IBufferByteAccess = TestBuffer(vec![0xAA, 0xBB, 0xCC].into()).into();

    let mut bytes = core::ptr::null_mut();
    unsafe { object.Buffer(&mut bytes).ok()? };
    assert!(!bytes.is_null());
    let bytes = unsafe { core::slice::from_raw_parts(bytes, 3) };
    assert_eq!(bytes, [0xAA, 0xBB, 0xCC]);

    let object = Buffer::Create(3)?;

    let mut bytes = core::ptr::null_mut();
    unsafe {
        object
            .cast::<IBufferByteAccess>()?
            .Buffer(&mut bytes)
            .ok()?;
    };
    assert!(!bytes.is_null());
    let bytes = unsafe { core::slice::from_raw_parts_mut(bytes, 3) };
    bytes.copy_from_slice(&[0xCC, 0xBB, 0xAA]);

    let mut bytes = core::ptr::null_mut();
    unsafe {
        object
            .cast::<IBufferByteAccess>()?
            .Buffer(&mut bytes)
            .ok()?;
    };
    assert!(!bytes.is_null());
    let bytes = unsafe { core::slice::from_raw_parts(bytes, 3) };

    assert_eq!(bytes, [0xCC, 0xBB, 0xAA]);
    Ok(())
}
