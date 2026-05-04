pub mod Test {
    windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0x28337665_44f1_5895_b1c3_726675c39971);
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
        pub fn DoSomething(&self) -> windows_result::Result<()> {
            unsafe {
                windows_core::imp::call_in(self, |this__| {
                    (windows_core::Interface::vtable(self).DoSomething)(this__)
                })
            }
        }
        pub fn GetValue(&self) -> windows_result::Result<i32> {
            unsafe {
                windows_core::imp::call_in_out(self, |this__, result__| {
                    (windows_core::Interface::vtable(self).GetValue)(this__, result__)
                })
            }
        }
        pub fn GetThing(&self) -> windows_result::Result<ITest> {
            unsafe {
                windows_core::imp::call_in_out(self, |this__, result__| {
                    (windows_core::Interface::vtable(self).GetThing)(this__, result__)
                })
            }
        }
    }
    impl windows_core::RuntimeName for ITest {
        const NAME: &'static str = "Test.ITest";
    }
    pub trait ITest_Impl: windows_core::IUnknownImpl {
        fn DoSomething(&self) -> windows_result::Result<()>;
        fn GetValue(&self) -> windows_result::Result<i32>;
        fn GetThing(&self) -> windows_result::Result<ITest>;
    }
    impl ITest_Vtbl {
        pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
            unsafe extern "system" fn DoSomething<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    ITest_Impl::DoSomething(this).into()
                }
            }
            unsafe extern "system" fn GetValue<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut i32,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match ITest_Impl::GetValue(this) {
                        Ok(ok__) => {
                            result__.write(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            unsafe extern "system" fn GetThing<Identity: ITest_Impl, const OFFSET: isize>(
                this: *mut core::ffi::c_void,
                result__: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this: &Identity =
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                    match ITest_Impl::GetThing(this) {
                        Ok(ok__) => {
                            result__.write(core::mem::transmute_copy(&ok__));
                            core::mem::forget(ok__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into(),
                    }
                }
            }
            Self {
                base__: windows_core::IInspectable_Vtbl::new::<Identity, ITest, OFFSET>(),
                DoSomething: DoSomething::<Identity, OFFSET>,
                GetValue: GetValue::<Identity, OFFSET>,
                GetThing: GetThing::<Identity, OFFSET>,
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
        pub DoSomething:
            unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_result::HRESULT,
        pub GetValue:
            unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_result::HRESULT,
        pub GetThing: unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> windows_result::HRESULT,
    }
}
