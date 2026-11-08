windows_core::imp::define_winrt_interface!(IIterable < T >, IIterable_Vtbl < T, >, 0xfaa585ea_6214_4217_afda_7f46de5869b3, b"Windows.Foundation.Collections.IIterable`1");
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).First)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IIterable_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>>;
}
impl<T: windows_core::RuntimeType + 'static> IIterable_Vtbl<T> {
    pub const fn new<Identity: IIterable_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn First<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterable_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterable_Impl::First(this) {
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
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            First: First::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterable<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
impl<T: windows_core::RuntimeType> IntoIterator for IIterable<T> {
    type Item = T;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First().unwrap())
    }
}
