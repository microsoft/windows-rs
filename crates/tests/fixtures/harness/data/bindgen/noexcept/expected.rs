pub mod Test {
    windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0x98189505_75e7_5c31_b3ea_20572501df90);
    impl windows_core::RuntimeType for ITest {
        const SIGNATURE: windows_core::imp::ConstBuffer =
            windows_core::imp::ConstBuffer::for_interface::<Self>();
    }
    windows_core::imp::interface_hierarchy!(
        ITest,
        windows_core::IUnknown,
        windows_core::IInspectable
    );
    impl ITest {
        pub fn MethodInt32(&self, value: i32) {
            unsafe {
                let hresult__ = (windows_core::Interface::vtable(self).MethodInt32)(
                    windows_core::Interface::as_raw(self),
                    value,
                );
                assert!(hresult__.0 == 0);
            }
        }
        pub fn ReturnInt32(&self) -> i32 {
            unsafe {
                let mut result__ = core::mem::zeroed();
                let hresult__ = (windows_core::Interface::vtable(self).ReturnInt32)(
                    windows_core::Interface::as_raw(self),
                    &mut result__,
                );
                assert!(hresult__.0 == 0);
                result__
            }
        }
    }
    impl windows_core::RuntimeName for ITest {
        const NAME: &'static str = "Test.ITest";
    }
    pub trait ITest_Impl: windows_core::IUnknownImpl {
        fn MethodInt32(&self, value: i32);
        fn ReturnInt32(&self) -> i32;
    }
    impl ITest_Vtbl {
        pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn MethodInt32<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                value: i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    ITest_Impl::MethodInt32(this, value);
                    windows_core::HRESULT(0)
                }
            }
            unsafe extern "system" fn ReturnInt32<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    let ok__ = ITest_Impl::ReturnInt32(this);
                    result__.write(ok__);
                    windows_core::HRESULT(0)
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, ITest, OFFSET>(),
                MethodInt32: MethodInt32::<Identity, OFFSET>,
                ReturnInt32: ReturnInt32::<Identity, OFFSET>,
            }
        }
        pub fn matches(iid: &windows_core::GUID) -> bool {
            iid == &<ITest as windows_core::Interface>::IID
        }
    }
    #[repr(C)]
    #[doc(hidden)]
    pub struct ITest_Vtbl {
        pub base__: windows_core::IInspectable_Vtbl,
        pub MethodInt32:
            unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_result::HRESULT,
        pub ReturnInt32:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
    }
}
