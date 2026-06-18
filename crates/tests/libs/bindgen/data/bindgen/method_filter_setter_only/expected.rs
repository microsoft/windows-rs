pub mod Test {
    windows_core::imp::define_interface!(IBar, IBar_Vtbl, 0x43f14249_b5b1_55f1_995f_697768ac5488);
    impl windows_core::RuntimeType for IBar {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IBar");
    }
    windows_core::imp::interface_hierarchy!(
        IBar,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IBar {
        pub fn SetOther(&self, value: i32) -> windows_core::Result<()> {
            unsafe {
                (windows_core::Interface::vtable(self).SetOther)(
                    windows_core::Interface::as_raw(self),
                    value,
                )
                .ok()
            }
        }
    }
    impl windows_core::RuntimeName for IBar {
        const NAME: &'static str = "Test.IBar";
    }
    #[repr(C)]
    pub struct IBar_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Other: usize,
        pub SetOther:
            unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0xa2715897_a2fb_55df_90c3_93eb89fa1fc8);
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
        pub fn SetValue(&self, value: i32) -> windows_core::Result<()> {
            unsafe {
                (windows_core::Interface::vtable(self).SetValue)(
                    windows_core::Interface::as_raw(self),
                    value,
                )
                .ok()
            }
        }
    }
    impl windows_core::RuntimeName for IFoo {
        const NAME: &'static str = "Test.IFoo";
    }
    #[repr(C)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Value: usize,
        pub SetValue:
            unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    }
}
