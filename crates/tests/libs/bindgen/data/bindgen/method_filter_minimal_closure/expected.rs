pub mod Test {
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x29b2ee6f_e8bf_5d03_8e01_81e8ad109076);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFoo");
    }
    impl IFoo {
        pub fn Direct(&self) -> windows_core::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Direct)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
        pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Name)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| core::mem::transmute(result__))
            }
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
        pub Name: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT,
        SetName: usize,
    }
    windows_core::imp::define_interface!(IFoo2, IFoo2_Vtbl, 0xd5639aca_50ae_5b48_9f64_938ce24b8683);
    impl windows_core::RuntimeType for IFoo2 {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IFoo2");
    }
    windows_core::imp::interface_hierarchy!(
        IFoo2,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IFoo2 {
        const NAME: &'static str = "Test.IFoo2";
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo2_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Bar: usize,
    }
}
