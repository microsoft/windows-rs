#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IIterable<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterable<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterable<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterable<T> {
    type Vtable = IIterable_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({faa585ea-6214-4217-afda-7f46de5869b3}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = self;
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
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
}
pub trait IIterable_Impl<T>: windows_core::IUnknownImpl
where
    T: windows_core::RuntimeType + 'static,
{
    fn First(&self) -> windows_core::Result<IIterator<T>>;
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
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIterable<T>, OFFSET>(),
            First: First::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIterable<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
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
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType> IntoIterator for &IIterable<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IIterator<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IIterator<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IIterator<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IIterator<T> {
    type Vtable = IIterator_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterator<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({6a79e863-4300-459a-9966-cbb660963ee1}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> IIterator<T> {
    pub fn Current(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasCurrent(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasCurrent)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn MoveNext(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MoveNext)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn GetMany(
        &self,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
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
        items: &mut [<T as windows_core::Type<T>>::Default],
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
            result__: *mut windows_core::AbiType<T>,
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
                    Err(err) => err.into(),
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
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
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
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
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
            items: *mut T,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IIterator_Impl::GetMany(
                    this,
                    core::slice::from_raw_parts_mut(
                        core::mem::transmute_copy(&items),
                        items_array_size as usize,
                    ),
                ) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IIterator<T>, OFFSET>(),
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
#[doc(hidden)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<T>,
    ) -> windows_core::HRESULT,
    pub HasCurrent:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MoveNext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut T,
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
            self.MoveNext().ok()?;
        }
        result
    }
}
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
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVector<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(
                windows_core::Interface::as_raw(this),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).SetAt)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).InsertAt)(
                windows_core::Interface::as_raw(this),
                index,
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAt)(
                windows_core::Interface::as_raw(this),
                index,
            )
            .ok()
        }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<T>,
    {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Append)(
                windows_core::Interface::as_raw(this),
                value.param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).RemoveAtEnd)(windows_core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(
                windows_core::Interface::as_raw(this),
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
        let this = self;
        unsafe {
            (windows_core::Interface::vtable(this).ReplaceAll)(
                windows_core::Interface::as_raw(this),
                items.len().try_into().unwrap(),
                core::mem::transmute(items.as_ptr()),
            )
            .ok()
        }
    }
    pub fn First(&self) -> windows_core::Result<IIterator<T>> {
        let this = &windows_core::Interface::cast::<IIterable<T>>(self)?;
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
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVector<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVector<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVector";
}
pub trait IVector_Impl<T>: IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
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
                        result__.write(core::mem::transmute_copy(&ok__));
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
                        result__.write(core::mem::transmute_copy(&ok__));
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
            items: *mut T,
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
                        result__.write(core::mem::transmute_copy(&ok__));
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
            items: *const T,
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
            GetView: 0,
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
#[doc(hidden)]
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
    GetView: usize,
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
        *mut T,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub ReplaceAll:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const T) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
