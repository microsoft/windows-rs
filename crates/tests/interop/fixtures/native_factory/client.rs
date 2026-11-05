use super::*;
use NativeFactory::*;

pub fn test() -> Result<()> {
    assert_eq!(Widget::new()?.HandleValue()?, 0);

    type Create =
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> HRESULT;

    // The harness deploys the DLL at run time, so it cannot be a static import of this test.
    let module = unsafe { GetModuleHandleW(w!("NativeFactory.dll")) };
    assert!(!module.is_null());
    let create: Create = unsafe {
        core::mem::transmute(GetProcAddress(module, s!("CreateWidgetFromHandle")).unwrap())
    };
    let handle = 0x1c8 as *mut core::ffi::c_void;
    let mut result = core::ptr::null_mut();
    unsafe { create(handle, &mut result).ok()? };
    let widget = unsafe { Widget::from_raw(result) };
    assert_eq!(widget.HandleValue()?, 0x1c8);
    let interop: IWidgetInterop = widget.cast()?;
    assert_eq!(unsafe { interop.Handle() }, handle);
    Ok(())
}
