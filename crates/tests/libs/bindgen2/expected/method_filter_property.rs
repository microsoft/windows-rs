windows_core::imp::define_interface!(
    Interface,
    Interface_Vtbl,
    0x11d0f78f_5ede_58a0_a9e8_f1d2800d7958
);
impl windows_core::RuntimeType for Interface {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Interface");
}
windows_core::imp::interface_hierarchy!(
    Interface,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl Interface {
    pub fn get_Value(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn put_Value(&self, value: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).put_Value)(
                windows_core::Interface::as_raw(self),
                value,
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for Interface {
    const NAME: &'static str = "Test.Interface";
}
#[repr(C)]
pub struct Interface_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_Value:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub put_Value: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
