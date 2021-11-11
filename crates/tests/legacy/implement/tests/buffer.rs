use test_implement::*;
use windows::core::*;

use Windows::Storage::Streams::Buffer;
use Windows::Win32::System::WinRT::IBufferByteAccess;

#[implement(Windows::Win32::System::WinRT::IBufferByteAccess)]
struct TestBuffer(Vec<u8>);

#[allow(non_snake_case)]
impl TestBuffer {
    fn Buffer(&mut self) -> Result<*mut u8> {
        Ok(self.0.as_mut_ptr())
    }
}

#[test]
fn test() -> Result<()> {
    let object: IBufferByteAccess = TestBuffer(vec![0xAA, 0xBB, 0xCC]).into();

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
