windows_core::imp::define_winrt_interface!(IItems < T >, IItems_Vtbl < T, >, 0x8dfef4bd_8c6e_5d6d_9614_0aca51e6943b, b"Test.IItems`1");
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IItems<T> {
    const NAME: &'static str = "Test.IItems";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
#[repr(C)]
pub struct IItems_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    T: core::marker::PhantomData<T>,
}
