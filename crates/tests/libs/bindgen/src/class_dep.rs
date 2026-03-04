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
pub struct IVectorView<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IVectorView<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IVectorView<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVectorView<T> {
    type Vtable = IVectorView_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVectorView<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVectorView<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView<T> {
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
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IVectorView<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IVectorView<T> {
    type Item = T;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IVectorView<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IVectorView";
}
pub trait IVectorView_Impl<T>: IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn GetAt(&self, index: u32) -> windows_core::Result<T>;
    fn Size(&self) -> windows_core::Result<u32>;
    fn IndexOf(&self, value: windows_core::Ref<T>, index: &mut u32) -> windows_core::Result<bool>;
    fn GetMany(
        &self,
        startIndex: u32,
        items: &mut [<T as windows_core::Type<T>>::Default],
    ) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IVectorView_Vtbl<T> {
    pub const fn new<Identity: IVectorView_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<
            T: windows_core::RuntimeType + 'static,
            Identity: IVectorView_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: u32,
            result__: *mut windows_core::AbiType<T>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVectorView_Impl::GetAt(this, index) {
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
            Identity: IVectorView_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVectorView_Impl::Size(this) {
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
            Identity: IVectorView_Impl<T>,
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
                match IVectorView_Impl::IndexOf(
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
        unsafe extern "system" fn GetMany<
            T: windows_core::RuntimeType + 'static,
            Identity: IVectorView_Impl<T>,
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
                match IVectorView_Impl::GetMany(
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
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IVectorView<T>, OFFSET>(),
            GetAt: GetAt::<T, Identity, OFFSET>,
            Size: Size::<T, Identity, OFFSET>,
            IndexOf: IndexOf::<T, Identity, OFFSET>,
            GetMany: GetMany::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVectorView<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVectorView_Vtbl<T>
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
    pub IndexOf: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::AbiType<T>,
        *mut u32,
        *mut bool,
    ) -> windows_core::HRESULT,
    pub GetMany: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut T,
        *mut u32,
    ) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(
    IWwwFormUrlDecoderEntry,
    IWwwFormUrlDecoderEntry_Vtbl,
    0x125e7431_f678_4e8e_b670_20a9b06c512d
);
impl windows_core::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IWwwFormUrlDecoderEntry,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IWwwFormUrlDecoderEntry {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
pub trait IWwwFormUrlDecoderEntry_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Value(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWwwFormUrlDecoderEntry_Vtbl {
    pub const fn new<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<
            Identity: IWwwFormUrlDecoderEntry_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWwwFormUrlDecoderEntry_Impl::Name(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<
            Identity: IWwwFormUrlDecoderEntry_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWwwFormUrlDecoderEntry_Impl::Value(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWwwFormUrlDecoderEntry, OFFSET>(
            ),
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderEntry as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWwwFormUrlDecoderRuntimeClass,
    IWwwFormUrlDecoderRuntimeClass_Vtbl,
    0xd45a0451_f225_4542_9296_0e1df5d254df
);
impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFirstValueByName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(
    IWwwFormUrlDecoderRuntimeClassFactory,
    IWwwFormUrlDecoderRuntimeClassFactory_Vtbl,
    0x5b8c6b3d_24ae_41b5_a1bf_f0c3d544845b
);
impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateWwwFormUrlDecoder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WwwFormUrlDecoder(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    WwwFormUrlDecoder,
    windows_core::IUnknown,
    windows_core::IInspectable
);
windows_core::imp::required_hierarchy!(
    WwwFormUrlDecoder,
    IIterable<IWwwFormUrlDecoderEntry>,
    IVectorView<IWwwFormUrlDecoderEntry>
);
impl WwwFormUrlDecoder {
    pub fn First(&self) -> windows_core::Result<IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &windows_core::Interface::cast::<IIterable<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<IWwwFormUrlDecoderEntry> {
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
        P0: windows_core::Param<IWwwFormUrlDecoderEntry>,
    {
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
    pub fn GetMany(
        &self,
        startindex: u32,
        items: &mut [Option<IWwwFormUrlDecoderEntry>],
    ) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
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
    pub fn GetFirstValueByName(
        &self,
        name: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFirstValueByName)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(name),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateWwwFormUrlDecoder(
        query: &windows_core::HSTRING,
    ) -> windows_core::Result<WwwFormUrlDecoder> {
        Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWwwFormUrlDecoder)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(query),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWwwFormUrlDecoderRuntimeClassFactory<
        R,
        F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> windows_core::Result<R>,
    >(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<
            WwwFormUrlDecoder,
            IWwwFormUrlDecoderRuntimeClassFactory,
        > = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WwwFormUrlDecoder {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IWwwFormUrlDecoderRuntimeClass>();
}
unsafe impl windows_core::Interface for WwwFormUrlDecoder {
    type Vtable = <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID =
        <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
unsafe impl Send for WwwFormUrlDecoder {}
unsafe impl Sync for WwwFormUrlDecoder {}
impl IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
