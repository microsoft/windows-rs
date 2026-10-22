#![allow(non_camel_case_types, non_snake_case)]
include!("../bindings.rs");
use std::cell::{Cell, UnsafeCell};
use windows_core::Interface;

const _: () = {
    let pointer = core::mem::size_of::<usize>();
    assert!(core::mem::size_of::<NamespaceConsumer>() == 4 * pointer);
    assert!(core::mem::size_of::<DispatcherQueueHandler_Vtbl>() == 4 * pointer);
    assert!(core::mem::offset_of!(IMemoryBufferByteAccess_Vtbl, GetBuffer) == 3 * pointer);
    assert!(core::mem::offset_of!(IBufferByteAccess_Vtbl, Buffer) == 3 * pointer);
    assert!(core::mem::offset_of!(IBuffer_Vtbl, Capacity) == 6 * pointer);
    assert!(core::mem::size_of::<IBuffer_Vtbl>() == 9 * pointer);
};

#[windows_core::implement(IBuffer, IBufferByteAccess, IMemoryBufferByteAccess)]
struct Buffer {
    data: UnsafeCell<[u8; 4]>,
    length: Cell<u32>,
}

impl IBuffer_Impl for Buffer_Impl {
    fn Capacity(&self) -> windows_core::Result<u32> {
        Ok(4)
    }

    fn Length(&self) -> windows_core::Result<u32> {
        Ok(self.length.get())
    }

    fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        if value > 4 {
            return Err(windows_core::HRESULT(0x80070057_u32 as i32).into());
        }
        self.length.set(value);
        Ok(())
    }
}

impl IBufferByteAccess_Impl for Buffer_Impl {
    fn Buffer(&self) -> windows_core::Result<*mut u8> {
        Ok(self.data.get().cast())
    }
}

impl IMemoryBufferByteAccess_Impl for Buffer_Impl {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> windows_core::Result<()> {
        unsafe {
            value.write(self.data.get().cast());
            capacity.write(4);
        }
        Ok(())
    }
}

fn main() {
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let invoked = calls.clone();
    let handler = DispatcherQueueHandler::new(move || {
        invoked.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    });
    handler.Invoke().unwrap();
    assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(
        IBuffer::IID,
        windows_core::GUID::from_u128(0x905a0fe0_bc53_11df_8c49_001e4fc686da)
    );
    assert_eq!(
        IBufferByteAccess::IID,
        windows_core::GUID::from_u128(0x905a0fef_bc53_11df_8c49_001e4fc686da)
    );
    assert_eq!(
        IMemoryBufferByteAccess::IID,
        windows_core::GUID::from_u128(0x5b0d3235_4dba_4d44_865e_8f1d0e4fd04d)
    );
    let buffer: IBuffer = Buffer {
        data: UnsafeCell::new([1, 2, 3, 4]),
        length: Cell::new(0),
    }
    .into();
    assert_eq!(buffer.Capacity().unwrap(), 4);
    buffer.SetLength(3).unwrap();
    assert_eq!(buffer.Length().unwrap(), 3);
    assert_eq!(
        buffer.SetLength(5).unwrap_err().code(),
        windows_core::HRESULT(0x80070057_u32 as i32)
    );
    assert_eq!(buffer.Length().unwrap(), 3);
    let access: IBufferByteAccess = buffer.cast().unwrap();
    let memory: IMemoryBufferByteAccess = buffer.cast().unwrap();
    unsafe {
        let bytes = access.Buffer().unwrap();
        let mut pointer = core::ptr::null_mut();
        let mut capacity = 0;
        memory.GetBuffer(&mut pointer, &mut capacity).ok().unwrap();
        assert_eq!(pointer, bytes);
        assert_eq!(capacity, 4);
        assert_eq!(core::slice::from_raw_parts(bytes, 4), &[1, 2, 3, 4]);
        pointer.add(2).write(9);
        assert_eq!(core::slice::from_raw_parts(bytes, 4), &[1, 2, 9, 4]);
    }
}
