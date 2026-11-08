windows_core::imp::define_winrt_interface!(IPair < K, V >, IPair_Vtbl < K, V, >, 0x02b51929_c1c4_4a7e_8940_0312b5c18500, b"Test.IPair`2");
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    windows_core::RuntimeName for IPair<K, V>
{
    const NAME: &'static str = "Test.IPair";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IPair_Impl<K, V>: windows_core::IUnknownImpl
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
}
impl<K: windows_core::RuntimeType + 'static, V: windows_core::RuntimeType + 'static>
    IPair_Vtbl<K, V>
{
    pub const fn new<Identity: IPair_Impl<K, V>, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            K: core::marker::PhantomData::<K>,
            V: core::marker::PhantomData::<V>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPair<K, V> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IPair_Vtbl<K, V>
where
    K: windows_core::RuntimeType + 'static,
    V: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    K: core::marker::PhantomData<K>,
    V: core::marker::PhantomData<V>,
}
windows_core::imp::define_winrt_interface!(
    IValue,
    IValue_Vtbl,
    0x11111111_2222_3333_4444_555555555555,
    b"Test.IValue"
);
windows_core::imp::interface_hierarchy!(IValue, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IValue {
    const NAME: &'static str = "Test.IValue";
}
pub trait IValue_Impl: windows_core::IUnknownImpl {}
impl IValue_Vtbl {
    pub const fn new<Identity: IValue_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValue as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_winrt_interface!(IVector < T >, IVector_Vtbl < T, >, 0x1b2776f1_623d_5a07_b5d8_9a225dfc1901, b"Test.IVector`1");
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Append)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
            )
            .ok()
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Test.IVector";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IVector_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn Append(&self, value: windows_core::Ref<T>) -> windows_core::Result<()>;
}
impl<T: windows_core::RuntimeType + 'static> IVector_Vtbl<T> {
    pub const fn new<Identity: IVector_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            result__: *mut windows_core::imp::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::GetAt(this, index) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Size<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        result__.write_bytes(0, 1);
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn Append<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::imp::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::Append(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            Append: Append::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVector<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IVector_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
