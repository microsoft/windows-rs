#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IItems<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown>
    for IItems<T>
{
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable>
    for IItems<T>
{
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IItems<T> {
    type Vtable = IItems_Vtbl<T>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IItems<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({8dfef4bd-8c6e-5d6d-9614-0aca51e6943b}")
        .push_slice(b";")
        .push_other(T::SIGNATURE)
        .push_slice(b")");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"Test.IItems`1<")
        .push_other(T::NAME)
        .push_slice(b">");
}
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IItems<T>, OFFSET>(),
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
