#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
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
    type Vtable = IIterable_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0xfaa585ea_6214_4217_afda_7f46de5869b3);
}
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IIterable<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> IIterable<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IIterable<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IIterable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
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
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IIterable<T>>
    for IVector<T>
{
    const QUERY: bool = true;
}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IVector<T> {
    type Vtable = IVector_Vtbl;
    const IID: windows_core::GUID =
        windows_core::GUID::from_u128(0x913337e9_11a1_4345_a3a2_4e7f956e222d);
}
impl<T: windows_core::RuntimeType + 'static> core::ops::Deref for IVector<T> {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
impl<T: windows_core::RuntimeType + 'static> IVector<T> {
    pub fn GetAt(&self) {}
    pub fn Size(&self) {}
    pub fn IndexOf(&self) {}
    pub fn SetAt(&self) {}
    pub fn InsertAt(&self) {}
    pub fn RemoveAt(&self) {}
    pub fn Append(&self) {}
    pub fn RemoveAtEnd(&self) {}
    pub fn Clear(&self) {}
    pub fn GetMany(&self) {}
    pub fn ReplaceAll(&self) {}
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IVector<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IVector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
