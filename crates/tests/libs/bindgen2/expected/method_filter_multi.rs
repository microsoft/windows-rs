windows_core::imp::define_interface!(
    IInterface,
    IInterface_Vtbl,
    0x35e799f4_57d9_5b36_8a94_bcc88d70336d
);
impl windows_core::RuntimeType for IInterface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.IInterface");
}
windows_core::imp::interface_hierarchy!(
    IInterface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IInterface {
    pub fn First(&self, x: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).First)(
                windows_core::Interface::as_raw(self),
                x,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Second(&self, y: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Second)(
                windows_core::Interface::as_raw(self),
                y,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IInterface {
    const NAME: &'static str = "Test.IInterface";
}
#[repr(C)]
pub struct IInterface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub First:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
    pub Second:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32) -> windows_core::HRESULT,
}
