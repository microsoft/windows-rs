windows_core::imp::define_winrt_interface!(IIterable < T >, IIterable_Vtbl < T, >, 0xfaa585ea_6214_4217_afda_7f46de5869b3, b"Windows.Foundation.Collections.IIterable`1");
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterable<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterable";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
#[repr(C)]
pub struct IIterable_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_winrt_interface!(IIterator < T >, IIterator_Vtbl < T, >, 0x6a79e863_4300_459a_9966_cbb660963ee1, b"Windows.Foundation.Collections.IIterator`1");
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IIterator<T> {
    const NAME: &'static str = "Windows.Foundation.Collections.IIterator";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
#[repr(C)]
pub struct IIterator_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    T: core::marker::PhantomData<T>,
}
