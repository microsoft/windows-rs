#[doc = "*Required features: `\"Graphics_Effects\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffect(::windows::core::IUnknown);
impl IGraphicsEffect {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
}
::windows::imp::interface_hierarchy!(IGraphicsEffect, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IGraphicsEffectSource> for IGraphicsEffect {}
impl ::core::cmp::PartialEq for IGraphicsEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffect {}
impl ::core::fmt::Debug for IGraphicsEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffect").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IGraphicsEffect {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{cb51c0ce-8fe6-4636-b202-861faa07d8f3}");
}
unsafe impl ::windows::core::Interface for IGraphicsEffect {
    type Vtable = IGraphicsEffect_Vtbl;
}
impl ::core::clone::Clone for IGraphicsEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGraphicsEffect {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb51c0ce_8fe6_4636_b202_861faa07d8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffect_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Effects\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffectSource(::windows::core::IUnknown);
impl IGraphicsEffectSource {}
::windows::imp::interface_hierarchy!(IGraphicsEffectSource, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IGraphicsEffectSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffectSource {}
impl ::core::fmt::Debug for IGraphicsEffectSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffectSource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IGraphicsEffectSource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{2d8f9ddc-4339-4eb9-9216-f9deb75658a2}");
}
unsafe impl ::windows::core::Interface for IGraphicsEffectSource {
    type Vtable = IGraphicsEffectSource_Vtbl;
}
impl ::core::clone::Clone for IGraphicsEffectSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IGraphicsEffectSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d8f9ddc_4339_4eb9_9216_f9deb75658a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectSource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
