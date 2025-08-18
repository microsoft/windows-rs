#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0x37b05fc1_6ee1_5798_b48d_602875fb73a2);
impl windows_core::RuntimeType for ITest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITest, windows_core::IUnknown, windows_core::IInspectable);
impl ITest {
    pub fn MethodString(&self, test: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).MethodString)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(test),
            )
            .ok()
        }
    }
    pub fn MethodInt32(&self, test: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).MethodInt32)(
                windows_core::Interface::as_raw(this),
                test,
            )
            .ok()
        }
    }
    pub fn MethodTest<P0>(&self, test: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITest>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).MethodTest)(
                windows_core::Interface::as_raw(this),
                test.param().abi(),
            )
            .ok()
        }
    }
    pub fn String(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).String)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetString(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetString)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn Int32(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Int32)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetInt32(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetInt32)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Test(&self) -> windows_core::Result<ITest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Test)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetTest<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ITest>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetTest)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn MethodStringN(&self, test: &windows_core::HSTRING) {
        let this = self;
        unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).MethodStringN)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(test),
            );
            debug_assert!(hresult__.0 == 0);
        }
    }
    pub fn MethodInt32N(&self, test: i32) {
        let this = self;
        unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).MethodInt32N)(
                windows_core::Interface::as_raw(this),
                test,
            );
            debug_assert!(hresult__.0 == 0);
        }
    }
    pub fn MethodTestN<P0>(&self, test: P0)
    where
        P0: windows_core::Param<ITest>,
    {
        let this = self;
        unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).MethodTestN)(
                windows_core::Interface::as_raw(this),
                test.param().abi(),
            );
            debug_assert!(hresult__.0 == 0);
        }
    }
    pub fn StringN(&self) -> windows_core::HSTRING {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).StringN)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            core::mem::transmute(result__)
        }
    }
    pub fn SetStringN(&self, value: &windows_core::HSTRING) {
        let this = self;
        unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).SetStringN)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
            );
            debug_assert!(hresult__.0 == 0);
        }
    }
    pub fn Int32N(&self) -> i32 {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).Int32N)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            result__
        }
    }
    pub fn SetInt32N(&self, value: i32) {
        let this = self;
        unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).SetInt32N)(
                windows_core::Interface::as_raw(this),
                value,
            );
            debug_assert!(hresult__.0 == 0);
        }
    }
    pub fn TestN(&self) -> Option<ITest> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            let hresult__ = (windows_core::Interface::vtable(this).TestN)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            );
            debug_assert!(hresult__.0 == 0);
            core::mem::transmute(result__)
        }
    }
    pub fn SetTestN<P0>(&self, value: P0)
    where
        P0: windows_core::Param<ITest>,
    {
        let this = self;
        unsafe {
            let hresult__ = (windows_core::Interface::vtable(this).SetTestN)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            );
            debug_assert!(hresult__.0 == 0);
        }
    }
}
impl windows_core::RuntimeName for ITest {
    const NAME: &'static str = "Test.ITest";
}
pub trait ITest_Impl: windows_core::IUnknownImpl {
    fn MethodString(&self, test: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn MethodInt32(&self, test: i32) -> windows_core::Result<()>;
    fn MethodTest(&self, test: windows_core::Ref<ITest>) -> windows_core::Result<()>;
    fn String(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetString(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Int32(&self) -> windows_core::Result<i32>;
    fn SetInt32(&self, value: i32) -> windows_core::Result<()>;
    fn Test(&self) -> windows_core::Result<ITest>;
    fn SetTest(&self, value: windows_core::Ref<ITest>) -> windows_core::Result<()>;
    fn MethodStringN(&self, test: &windows_core::HSTRING);
    fn MethodInt32N(&self, test: i32);
    fn MethodTestN(&self, test: windows_core::Ref<ITest>);
    fn StringN(&self) -> windows_core::HSTRING;
    fn SetStringN(&self, value: &windows_core::HSTRING);
    fn Int32N(&self) -> i32;
    fn SetInt32N(&self, value: i32);
    fn TestN(&self) -> Option<ITest>;
    fn SetTestN(&self, value: windows_core::Ref<ITest>);
}
impl ITest_Vtbl {
    pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MethodString<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            test: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::MethodString(this, core::mem::transmute(&test)).into()
            }
        }
        unsafe extern "system" fn MethodInt32<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            test: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::MethodInt32(this, test).into()
            }
        }
        unsafe extern "system" fn MethodTest<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            test: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::MethodTest(this, core::mem::transmute_copy(&test)).into()
            }
        }
        unsafe extern "system" fn String<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::String(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetString<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetString(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Int32<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::Int32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInt32<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetInt32(this, value).into()
            }
        }
        unsafe extern "system" fn Test<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::Test(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTest<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetTest(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn MethodStringN<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            test: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::MethodStringN(this, core::mem::transmute(&test));
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn MethodInt32N<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            test: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::MethodInt32N(this, test);
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn MethodTestN<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            test: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::MethodTestN(this, core::mem::transmute_copy(&test));
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn StringN<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = ITest_Impl::StringN(this);
                result__.write(core::mem::transmute_copy(&ok__));
                core::mem::forget(ok__);
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn SetStringN<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetStringN(this, core::mem::transmute(&value));
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn Int32N<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = ITest_Impl::Int32N(this);
                result__.write(core::mem::transmute_copy(&ok__));
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn SetInt32N<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetInt32N(this, value);
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn TestN<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                let ok__ = ITest_Impl::TestN(this);
                result__.write(core::mem::transmute_copy(&ok__));
                core::mem::forget(ok__);
                windows_core::HRESULT(0)
            }
        }
        unsafe extern "system" fn SetTestN<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetTestN(this, core::mem::transmute_copy(&value));
                windows_core::HRESULT(0)
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITest, OFFSET>(),
            MethodString: MethodString::<Identity, OFFSET>,
            MethodInt32: MethodInt32::<Identity, OFFSET>,
            MethodTest: MethodTest::<Identity, OFFSET>,
            String: String::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            Int32: Int32::<Identity, OFFSET>,
            SetInt32: SetInt32::<Identity, OFFSET>,
            Test: Test::<Identity, OFFSET>,
            SetTest: SetTest::<Identity, OFFSET>,
            MethodStringN: MethodStringN::<Identity, OFFSET>,
            MethodInt32N: MethodInt32N::<Identity, OFFSET>,
            MethodTestN: MethodTestN::<Identity, OFFSET>,
            StringN: StringN::<Identity, OFFSET>,
            SetStringN: SetStringN::<Identity, OFFSET>,
            Int32N: Int32N::<Identity, OFFSET>,
            SetInt32N: SetInt32N::<Identity, OFFSET>,
            TestN: TestN::<Identity, OFFSET>,
            SetTestN: SetTestN::<Identity, OFFSET>,
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
    pub MethodString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MethodInt32:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MethodTest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub String: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Int32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub Test: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MethodStringN: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub MethodInt32N:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub MethodTestN: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub StringN: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetStringN: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Int32N:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetInt32N: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub TestN: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetTestN: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
