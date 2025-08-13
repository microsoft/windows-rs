use windows::core::*;
use windows::Storage::Streams::Buffer;
use windows::Win32::System::WinRT::*;

#[implement(IBufferByteAccess)]
struct TestBuffer(std::cell::UnsafeCell<Vec<u8>>);

impl IBufferByteAccess_Impl for TestBuffer_Impl {
    fn Buffer(&self) -> Result<*mut u8> {
        unsafe { Ok((*self.0.get()).as_mut_ptr()) }
    }
}

#[test]
fn test() -> Result<()> {
    let object: IBufferByteAccess = TestBuffer(vec![0xAA, 0xBB, 0xCC].into()).into();

    let bytes: *const u8 = unsafe { object.Buffer()? };
    let bytes = unsafe { core::slice::from_raw_parts(bytes, 3) };
    assert_eq!(bytes, [0xAA, 0xBB, 0xCC]);

    let object = Buffer::Create(3)?;

    let bytes: *mut u8 = unsafe { object.cast::<IBufferByteAccess>()?.Buffer()? };
    let bytes = unsafe { core::slice::from_raw_parts_mut(bytes, 3) };
    bytes.copy_from_slice(&[0xCC, 0xBB, 0xAA]);

    let bytes: *const u8 = unsafe { object.cast::<IBufferByteAccess>()?.Buffer()? };
    let bytes = unsafe { core::slice::from_raw_parts(bytes, 3) };

    assert_eq!(bytes, [0xCC, 0xBB, 0xAA]);
    Ok(())
}
