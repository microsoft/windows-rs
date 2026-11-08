windows_core::imp::define_winrt_interface!(IPair < K, V >, IPair_Vtbl < K, V, >, 0x02b51929_c1c4_4a7e_8940_0312b5c18500);
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
    0x11111111_2222_3333_4444_555555555555
);
windows_core::imp::interface_hierarchy!(IValue, windows_core::IUnknown, windows_core::IInspectable);
#[repr(C)]
pub struct IValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_winrt_interface!(IVector < T >, IVector_Vtbl < T, >, 0x1b2776f1_623d_5a07_b5d8_9a225dfc1901);
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
