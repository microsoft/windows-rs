windows_core::imp::define_winrt_interface!(IIterator < T >, IIterator_Vtbl < T, >, 0x6a79e863_4300_459a_9966_cbb660963ee1, b"Windows.Foundation.Collections.IIterator`1");
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> windows_core::Result<T> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Current)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasCurrent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveNext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetMany(
        &self,
        items: &mut [<T as windows_core::imp::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        if core::mem::needs_drop::<<T as windows_core::imp::Type<T>>::Default>() {
            items.fill_with(|| unsafe { core::mem::zeroed() });
        }
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMany)(
                windows_core::Interface::as_raw(self),
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IIterator_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Current(&self) -> windows_core::Result<T>;
    fn HasCurrent(&self) -> windows_core::Result<bool>;
    fn MoveNext(&self) -> windows_core::Result<bool>;
    fn GetMany(
        &self,
        items: &mut [<T as windows_core::imp::Type<T>>::Default],
    ) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IIterator_Vtbl<T> {
    pub const fn new<Identity: IIterator_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Current<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::imp::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::Current(this) {
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
        unsafe extern "system" fn HasCurrent<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::HasCurrent(this) {
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
        unsafe extern "system" fn MoveNext<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::MoveNext(this) {
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
        unsafe extern "system" fn GetMany<
            T: windows_core::RuntimeType + 'static,
            Identity: IIterator_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            items_array_size: u32,
            items: *mut windows_core::imp::AbiType<T>,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::GetMany(
                    this,
                    windows_core::imp::array_slice_mut(
                        core::mem::transmute_copy(&items),
                        items_array_size,
                    ),
                ) {
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
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
            Current: Current::<T, Identity, OFFSET>,
            HasCurrent: HasCurrent::<T, Identity, OFFSET>,
            MoveNext: MoveNext::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterator<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::imp::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub HasCurrent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::imp::AbiType<T>,
        *mut u32,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
impl<T: windows_core::RuntimeType> Iterator for IIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.HasCurrent().unwrap_or(false) {
            self.Current().ok()
        } else {
            None
        };
        if result.is_some() {
            let _ = self.MoveNext();
        }
        result
    }
}
