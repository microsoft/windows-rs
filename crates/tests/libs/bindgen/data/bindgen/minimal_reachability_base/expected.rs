pub mod Test {
    windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0x0b6cf23e_8852_59ac_bec4_00a8180e689c);
    impl windows_core::RuntimeType for IBase {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IBase,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IBase {
        const NAME: &'static str = "Test.IBase";
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IBase_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Inherited: usize,
    }
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0xf05f601f_33d2_54fc_adca_2df8c56fe621);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IFoo,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(IFoo, IBase);
    impl IFoo {
        pub fn Direct(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Direct)(
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
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
