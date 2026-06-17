#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVector<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVector<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"Windows.Foundation.Collections.IVector`1<")
        .push_other(T::NAME)
        .push_slice(b">");
}
impl<T: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_collections::IIterable<T>> for IVector<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
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
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<T>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetView)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IndexOf)(
                windows_core::Interface::as_raw(self),
                value.param().abi(),
                index,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetAt)(
                windows_core::Interface::as_raw(self),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<T>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertAt)(
                windows_core::Interface::as_raw(self),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAt)(
                windows_core::Interface::as_raw(self),
                index,
            )
            .ok()
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
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAtEnd)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMany)(
                windows_core::Interface::as_raw(self),
                startindex,
                items.len().try_into().unwrap(),
                core::mem::transmute_copy(&items),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ReplaceAll(
        &self,
        items: &[<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ReplaceAll)(
                windows_core::Interface::as_raw(self),
                items.len().try_into().unwrap(),
                core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IVector<T> {
    type Item = T;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = windows_collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IVector_Impl<T>: windows_collections::IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<T>>;
    fn IndexOf(&self, value: windows_core::Ref<T>, index: &mut u32) -> windows_core::Result<bool>;
    fn SetAt(&self, index: u32, value: windows_core::Ref<T>) -> windows_core::Result<()>;
    fn InsertAt(&self, index: u32, value: windows_core::Ref<T>) -> windows_core::Result<()>;
    fn RemoveAt(&self, index: u32) -> windows_core::Result<()>;
    fn Append(&self, value: windows_core::Ref<T>) -> windows_core::Result<()>;
    fn RemoveAtEnd(&self) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetMany(
        &self,
        startIndex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32>;
    fn ReplaceAll(
        &self,
        items: &[<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<()>;
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
            result__: *mut windows_core::AbiType<T>,
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
                    Err(err) => err.into(),
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
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetView<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::GetView(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IndexOf<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::AbiType<T>,
            index: *mut u32,
            result__: *mut bool,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::IndexOf(
                    this,
                    core::mem::transmute_copy(&value),
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            value: windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::SetAt(this, index, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InsertAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            value: windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::InsertAt(this, index, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::RemoveAt(this, index).into()
            }
        }
        unsafe extern "system" fn Append<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::Append(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RemoveAtEnd<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::RemoveAtEnd(this).into()
            }
        }
        unsafe extern "system" fn Clear<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn GetMany<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startindex: u32,
            items_array_size: u32,
            items: *mut windows_core::AbiType<T>,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVector_Impl::GetMany(
                    this,
                    startindex,
                    core::slice::from_raw_parts_mut(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                ) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceAll<
            T: windows_core::RuntimeType + 'static,
            Identity: IVector_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            items_array_size: u32,
            items: *const windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVector_Impl::ReplaceAll(
                    this,
                    core::slice::from_raw_parts(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVector<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            GetView: GetView::<T, Identity, OFFSET>,
            IndexOf: IndexOf::<T, Identity, OFFSET>,
            SetAt: SetAt::<T, Identity, OFFSET>,
            InsertAt: InsertAt::<T, Identity, OFFSET>,
            RemoveAt: RemoveAt::<T, Identity, OFFSET>,
            Append: Append::<T, Identity, OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<T, Identity, OFFSET>,
            Clear: Clear::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            ReplaceAll: ReplaceAll::<T, Identity, OFFSET>,
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
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IndexOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
        *mut u32,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub RemoveAtEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut windows_core::AbiType<T>,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub ReplaceAll: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
