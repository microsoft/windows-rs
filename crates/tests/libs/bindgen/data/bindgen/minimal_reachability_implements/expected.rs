pub mod Test {
    windows_core::imp::define_interface!(IBase, IBase_Vtbl, 0xdf7d3fd2_cd33_50c6_b037_b80b8a5e31a0);
    impl windows_core::RuntimeType for IBase {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IBase,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl IBase {
        pub fn BaseMethod(&self) -> windows_result::Result<i32> {
            unsafe {
                let mut result__ = core::mem::zeroed();
                (windows_core::Interface::vtable(self).BaseMethod)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                )
                .map(|| result__)
            }
        }
    }
    impl windows_core::RuntimeName for IBase {
        const NAME: &'static str = "Test.IBase";
    }
    pub trait IBase_Impl: windows_core::IUnknownImpl {
        fn BaseMethod(&self) -> windows_result::Result<i32>;
    }
    impl IBase_Vtbl {
        pub const fn new<Identity: IBase_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn BaseMethod<Identity: IBase_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IBase_Impl::BaseMethod(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IBase, OFFSET>(),
                BaseMethod: BaseMethod::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IBase as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IBase_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub BaseMethod:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
    windows_core::imp::define_interface!(IRoot, IRoot_Vtbl, 0x07f033ae_aecc_5845_8082_4a25947e32a1);
    impl windows_core::RuntimeType for IRoot {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        IRoot,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    windows_core::imp::required_hierarchy!(IRoot, IBase);
    impl IRoot {
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
    #[repr(C)]
    #[doc(hidden)]
    pub struct IRoot_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Direct:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
