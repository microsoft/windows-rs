#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0xaa1cc4e9_4780_5808_b172_2ef6449e2ba4);
impl windows_core::RuntimeType for ITest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITest, windows_core::IUnknown, windows_core::IInspectable);
impl ITest {
    pub fn Input<P0>(&self, input: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<ITest>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Input)(
                windows_core::Interface::as_raw(this),
                input.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Output(&self, value: i32, output: &mut Option<ITest>) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Output)(
                windows_core::Interface::as_raw(this),
                value,
                output as *mut _ as _,
            )
            .ok()
        }
    }
    pub fn Current(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetCurrent(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetCurrent)(
                windows_core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for ITest {
    const NAME: &'static str = "Test.ITest";
}
pub trait ITest_Impl: windows_core::IUnknownImpl {
    fn Input(&self, input: windows_core::Ref<ITest>) -> windows_core::Result<i32>;
    fn Output(&self, value: i32, output: windows_core::OutRef<ITest>) -> windows_core::Result<()>;
    fn Current(&self) -> windows_core::Result<i32>;
    fn SetCurrent(&self, value: i32) -> windows_core::Result<()>;
}
impl ITest_Vtbl {
    pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Input<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            input: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::Input(this, core::mem::transmute_copy(&input)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Output<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
            output: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::Output(this, value, core::mem::transmute_copy(&output)).into()
            }
        }
        unsafe extern "system" fn Current<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::Current(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrent<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::SetCurrent(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITest, OFFSET>(),
            Input: Input::<Identity, OFFSET>,
            Output: Output::<Identity, OFFSET>,
            Current: Current::<Identity, OFFSET>,
            SetCurrent: SetCurrent::<Identity, OFFSET>,
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
    pub Input: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub Output: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Current:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
