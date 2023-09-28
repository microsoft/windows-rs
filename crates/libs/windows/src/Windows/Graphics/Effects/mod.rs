#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGraphicsEffect(::windows_core::IUnknown);
impl IGraphicsEffect {
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IGraphicsEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IGraphicsEffectSource> for IGraphicsEffect {}
impl ::windows_core::RuntimeType for IGraphicsEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{cb51c0ce-8fe6-4636-b202-861faa07d8f3}");
}
unsafe impl ::windows_core::Interface for IGraphicsEffect {
    type Vtable = IGraphicsEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGraphicsEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcb51c0ce_8fe6_4636_b202_861faa07d8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGraphicsEffectSource(::windows_core::IUnknown);
impl IGraphicsEffectSource {}
::windows_core::imp::interface_hierarchy!(IGraphicsEffectSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IGraphicsEffectSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2d8f9ddc-4339-4eb9-9216-f9deb75658a2}");
}
unsafe impl ::windows_core::Interface for IGraphicsEffectSource {
    type Vtable = IGraphicsEffectSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGraphicsEffectSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d8f9ddc_4339_4eb9_9216_f9deb75658a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
