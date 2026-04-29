#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0xf160c0aa_4a72_5933_8ec6_2d82c5f4358c);
impl windows_core::RuntimeType for ITest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITest, windows_core::IUnknown, windows_core::IInspectable);
impl ITest {
    pub fn Numerics(&self, n: windows_numerics::Vector2) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Numerics)(
                windows_core::Interface::as_raw(self),
                n,
            )
            .ok()
        }
    }
    pub fn Collections<P0>(&self, c: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IVector<i32>>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Collections)(
                windows_core::Interface::as_raw(self),
                c.param().abi(),
            )
            .ok()
        }
    }
    pub fn Async(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Async)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Windows<P0>(&self, s: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::IStringable>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Windows)(
                windows_core::Interface::as_raw(self),
                s.param().abi(),
            )
            .ok()
        }
    }
}
impl windows_core::RuntimeName for ITest {
    const NAME: &'static str = "Test.ITest";
}
pub trait ITest_Impl: windows_core::IUnknownImpl {
    fn Numerics(&self, n: &windows_numerics::Vector2) -> windows_core::Result<()>;
    fn Collections(
        &self,
        c: windows_core::Ref<windows_collections::IVector<i32>>,
    ) -> windows_core::Result<()>;
    fn Async(&self) -> windows_core::Result<windows_future::IAsyncAction>;
    fn Windows(
        &self,
        s: windows_core::Ref<windows::Foundation::IStringable>,
    ) -> windows_core::Result<()>;
}
impl ITest_Vtbl {
    pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Numerics<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            n: windows_numerics::Vector2,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::Numerics(this, core::mem::transmute(&n)).into()
            }
        }
        unsafe extern "system" fn Collections<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            c: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::Collections(this, core::mem::transmute_copy(&c)).into()
            }
        }
        unsafe extern "system" fn Async<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::Async(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Windows<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            s: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::Windows(this, core::mem::transmute_copy(&s)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITest, OFFSET>(),
            Numerics: Numerics::<Identity, OFFSET>,
            Collections: Collections::<Identity, OFFSET>,
            Async: Async::<Identity, OFFSET>,
            Windows: Windows::<Identity, OFFSET>,
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
    pub Numerics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
    ) -> windows_core::HRESULT,
    pub Collections: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Async: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Windows: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
