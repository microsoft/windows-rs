#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(ITest, ITest_Vtbl, 0xcc6e2e5a_05a8_5720_8fc4_959d2e621196);
impl windows_core::RuntimeType for ITest {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(ITest, windows_core::IUnknown, windows_core::IInspectable);
impl ITest {
    pub fn TestIterable<P0>(&self, collection: P0, values: &[i32]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IIterable<i32>>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).TestIterable)(
                windows_core::Interface::as_raw(self),
                collection.param().abi(),
                values.len().try_into().unwrap(),
                values.as_ptr(),
            )
            .ok()
        }
    }
    pub fn GetIterable(
        &self,
        values: &[i32],
    ) -> windows_core::Result<windows_collections::IIterable<i32>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIterable)(
                windows_core::Interface::as_raw(self),
                values.len().try_into().unwrap(),
                values.as_ptr(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetMapView(
        &self,
        values: &[i32],
    ) -> windows_core::Result<
        windows_collections::IMapView<i32, windows_collections::IVectorView<i32>>,
    > {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMapView)(
                windows_core::Interface::as_raw(self),
                values.len().try_into().unwrap(),
                values.as_ptr(),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for ITest {
    const NAME: &'static str = "Test.ITest";
}
pub trait ITest_Impl: windows_core::IUnknownImpl {
    fn TestIterable(
        &self,
        collection: windows_core::Ref<windows_collections::IIterable<i32>>,
        values: &[i32],
    ) -> windows_core::Result<()>;
    fn GetIterable(
        &self,
        values: &[i32],
    ) -> windows_core::Result<windows_collections::IIterable<i32>>;
    fn GetMapView(
        &self,
        values: &[i32],
    ) -> windows_core::Result<
        windows_collections::IMapView<i32, windows_collections::IVectorView<i32>>,
    >;
}
impl ITest_Vtbl {
    pub const fn new<Identity: ITest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TestIterable<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            collection: *mut core::ffi::c_void,
            values_array_size: u32,
            values: *const i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITest_Impl::TestIterable(
                    this,
                    core::mem::transmute_copy(&collection),
                    core::slice::from_raw_parts(
                        core::mem::transmute_copy(&values),
                        values_array_size as usize,
                    ),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetIterable<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            values_array_size: u32,
            values: *const i32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::GetIterable(
                    this,
                    core::slice::from_raw_parts(
                        core::mem::transmute_copy(&values),
                        values_array_size as usize,
                    ),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMapView<Identity: ITest_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            values_array_size: u32,
            values: *const i32,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITest_Impl::GetMapView(
                    this,
                    core::slice::from_raw_parts(
                        core::mem::transmute_copy(&values),
                        values_array_size as usize,
                    ),
                ) {
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
            TestIterable: TestIterable::<Identity, OFFSET>,
            GetIterable: GetIterable::<Identity, OFFSET>,
            GetMapView: GetMapView::<Identity, OFFSET>,
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
    pub TestIterable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const i32,
    ) -> windows_core::HRESULT,
    pub GetIterable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetMapView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
