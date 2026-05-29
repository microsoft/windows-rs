pub mod Test {
    windows_core::imp::define_interface!(
        IDerived,
        IDerived_Vtbl,
        0xd3fed0a4_8ba5_5d1a_ac8f_cf0cb08debba
    );
    impl windows_core::RuntimeType for IDerived {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IDerived");
    }
    windows_core::imp::interface_hierarchy!(
        IDerived,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(IDerived, IRequired);
    impl IDerived {
        pub fn Value(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).Value)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IDerived {
        const NAME: &'static str = "Test.IDerived";
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IDerived_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Value:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(
        IRequired,
        IRequired_Vtbl,
        0xfa61ccc9_5bec_5f57_bb33_4ad48dd7ffda
    );
    impl windows_core::RuntimeType for IRequired {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
        const NAME: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::from_slice(b"Test.IRequired");
    }
    windows_core::imp::interface_hierarchy!(
        IRequired,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl windows_core::RuntimeName for IRequired {
        const NAME: &'static str = "Test.IRequired";
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IRequired_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        Stub: usize,
    }
}
