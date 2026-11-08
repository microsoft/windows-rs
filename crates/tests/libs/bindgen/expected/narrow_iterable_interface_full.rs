windows_core::imp::define_winrt_interface!(IItems < T >, IItems_Vtbl < T, >, 0x8dfef4bd_8c6e_5d6d_9614_0aca51e6943b, b"Test.IItems`1");
impl<T: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_collections::IIterable<T>> for IItems<T>
{
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IItems<T> {
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>> {
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
        windows_collections::BufferedIterator::new(self.First().unwrap())
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
}
impl<T: windows_core::RuntimeType + 'static> IItems_Vtbl<T> {
    pub const fn new<Identity: IItems_Impl<T>, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, OFFSET>(),
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
    T: core::marker::PhantomData<T>,
}
