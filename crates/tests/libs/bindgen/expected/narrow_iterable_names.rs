windows_core::imp::define_winrt_interface!(IItems < T >, IItems_Vtbl < T, >, 0x767732f2_98a0_5eee_a518_5c855972b181, b"Test.IItems`1");
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
}
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
