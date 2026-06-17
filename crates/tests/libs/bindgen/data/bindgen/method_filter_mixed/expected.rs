pub mod Test {
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x80018467_16db_5a56_8507_e6b78bdfa1c6);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFoo");
    }
    windows_core::imp::interface_hierarchy!(
        IFoo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IFoo {
        pub fn Keep(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Keep)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IFoo {
        const NAME: &'static str = "Test.IFoo";
    }
    #[repr(C)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Keep:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
