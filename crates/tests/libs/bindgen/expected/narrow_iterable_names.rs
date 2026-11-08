windows_core::imp::define_winrt_interface!(IItems < T >, IItems_Vtbl < T, >, 0x767732f2_98a0_5eee_a518_5c855972b181, b"Test.IItems`1");
impl<T: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_collections::IIterable<T>> for IItems<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IItems<T> {
    pub fn First(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).First)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn First2(&self) -> windows_core::Result<windows_collections::IIterator<T>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<T>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for IItems<T> {
    type Item = T;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IItems<T> {
    type Item = T;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First2().unwrap())
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IItems<T> {
    const NAME: &'static str = "Test.IItems";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IItems_Impl<T>: windows_collections::IIterable_Impl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    fn First(&self) -> windows_core::Result<u32>;
}
impl<T: windows_core::RuntimeType + 'static> IItems_Vtbl<T> {
    pub const fn new<Identity: IItems_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn First<
            T: windows_core::RuntimeType + 'static,
            Identity: IItems_Impl<T>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IItems_Impl::First(this) {
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
            First: First::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IItems<T> as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IItems_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub First: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Items(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    Items,
    windows_core::IUnknown,
    windows_core::IInspectable,
    IItems<i32>
);
windows_core::imp::required_hierarchy!(Items, windows_collections::IIterable<i32>);
impl Items {
    pub fn First(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).First)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn First2(&self) -> windows_core::Result<windows_collections::IIterator<i32>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<i32>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for Items {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IItems<i32>>();
}
unsafe impl windows_core::Interface for Items {
    type Vtable = <IItems<i32> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IItems<i32> as windows_core::Interface>::IID;
    const INSPECTABLE: bool = true;
}
impl windows_core::RuntimeName for Items {
    const NAME: &'static str = "Test.Items";
}
impl IntoIterator for Items {
    type Item = i32;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &Items {
    type Item = i32;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First2().unwrap())
    }
}
