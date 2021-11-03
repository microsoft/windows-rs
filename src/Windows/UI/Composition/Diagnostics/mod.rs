#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `UI_Composition_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CompositionDebugHeatMaps(::windows::runtime::IInspectable);
impl CompositionDebugHeatMaps {
    #[doc = "*Required features: `UI_Composition_Diagnostics`*"]
    pub fn Hide<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), subtree.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Diagnostics`*"]
    pub fn ShowMemoryUsage<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), subtree.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Diagnostics`*"]
    pub fn ShowOverdraw<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(&self, subtree: Param0, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), subtree.into_param().abi(), contentkinds).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Diagnostics`*"]
    pub fn ShowRedraw<'a, Param0: ::windows::runtime::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), subtree.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionDebugHeatMaps {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps;{e49c90ac-2ff3-5805-718c-b725ee07650f})");
}
unsafe impl ::windows::runtime::Interface for CompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3835465900, 12275, 22533, [113, 140, 183, 37, 238, 7, 101, 15]);
}
impl ::windows::runtime::RuntimeName for CompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps";
}
impl ::std::convert::From<CompositionDebugHeatMaps> for ::windows::runtime::IUnknown {
    fn from(value: CompositionDebugHeatMaps) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompositionDebugHeatMaps> for ::windows::runtime::IUnknown {
    fn from(value: &CompositionDebugHeatMaps) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CompositionDebugHeatMaps> for ::windows::runtime::IInspectable {
    fn from(value: CompositionDebugHeatMaps) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositionDebugHeatMaps> for ::windows::runtime::IInspectable {
    fn from(value: &CompositionDebugHeatMaps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CompositionDebugHeatMaps {}
unsafe impl ::std::marker::Sync for CompositionDebugHeatMaps {}
#[doc = "*Required features: `UI_Composition_Diagnostics`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(0u32);
    pub const OffscreenRendered: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(1u32);
    pub const Colors: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(2u32);
    pub const Effects: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(4u32);
    pub const Shadows: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(8u32);
    pub const Lights: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(16u32);
    pub const Surfaces: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(32u32);
    pub const SwapChains: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(64u32);
    pub const All: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(4294967295u32);
}
impl ::std::convert::From<u32> for CompositionDebugOverdrawContentKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CompositionDebugOverdrawContentKinds {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CompositionDebugOverdrawContentKinds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Diagnostics.CompositionDebugOverdrawContentKinds;u4)");
}
impl ::windows::runtime::DefaultType for CompositionDebugOverdrawContentKinds {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CompositionDebugOverdrawContentKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CompositionDebugOverdrawContentKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Composition_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CompositionDebugSettings(::windows::runtime::IInspectable);
impl CompositionDebugSettings {
    #[doc = "*Required features: `UI_Composition_Diagnostics`*"]
    pub fn HeatMaps(&self) -> ::windows::runtime::Result<CompositionDebugHeatMaps> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CompositionDebugHeatMaps>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Diagnostics`*"]
    pub fn TryGetSettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::runtime::Result<CompositionDebugSettings> {
        Self::ICompositionDebugSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<CompositionDebugSettings>(result__)
        })
    }
    pub fn ICompositionDebugSettingsStatics<R, F: FnOnce(&ICompositionDebugSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CompositionDebugSettings, ICompositionDebugSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CompositionDebugSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugSettings;{2831987e-1d82-4d38-b7b7-efd11c7bc3d1})");
}
unsafe impl ::windows::runtime::Interface for CompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(674338942, 7554, 19768, [183, 183, 239, 209, 28, 123, 195, 209]);
}
impl ::windows::runtime::RuntimeName for CompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugSettings";
}
impl ::std::convert::From<CompositionDebugSettings> for ::windows::runtime::IUnknown {
    fn from(value: CompositionDebugSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CompositionDebugSettings> for ::windows::runtime::IUnknown {
    fn from(value: &CompositionDebugSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CompositionDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CompositionDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CompositionDebugSettings> for ::windows::runtime::IInspectable {
    fn from(value: CompositionDebugSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CompositionDebugSettings> for ::windows::runtime::IInspectable {
    fn from(value: &CompositionDebugSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CompositionDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CompositionDebugSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CompositionDebugSettings {}
unsafe impl ::std::marker::Sync for CompositionDebugSettings {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionDebugHeatMaps(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3835465900, 12275, 22533, [113, 140, 183, 37, 238, 7, 101, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugHeatMaps_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subtree: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subtree: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subtree: ::windows::runtime::RawPtr, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subtree: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionDebugSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(674338942, 7554, 19768, [183, 183, 239, 209, 28, 123, 195, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionDebugSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICompositionDebugSettingsStatics {
    type Vtable = ICompositionDebugSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1693196062, 27384, 19192, [184, 20, 200, 112, 253, 90, 149, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositor: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
