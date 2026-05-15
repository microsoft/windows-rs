pub mod Test {
    windows_core::imp::define_interface!(IFoo, IFoo_Vtbl, 0x82179b8c_7f19_5e5c_b38c_05b90ed58bb3);
    impl windows_core::RuntimeType for IFoo {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
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
    pub trait IFoo_Impl: windows_core::IUnknownImpl {
        fn Keep(&self) -> windows_result::Result<i32>;
    }
    impl IFoo_Vtbl {
        pub const fn new<Identity: IFoo_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn Keep<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match IFoo_Impl::Keep(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            #[allow(unused_variables)]
            unsafe extern "system" fn Drop<Identity: IFoo_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                windows_core::HRESULT(0x80004001_u32 as i32)
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, IFoo, OFFSET>(),
                Keep: Keep::<Identity, OFFSET>,
                Drop: Drop::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<IFoo as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct IFoo_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub Keep:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
        Drop:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
