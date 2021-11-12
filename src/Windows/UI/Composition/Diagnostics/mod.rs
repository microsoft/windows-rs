#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositionDebugHeatMaps(pub ::windows::core::IInspectable);
impl CompositionDebugHeatMaps {
    pub fn Hide<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), subtree.into_param().abi()).ok() }
    }
    pub fn ShowMemoryUsage<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), subtree.into_param().abi()).ok() }
    }
    pub fn ShowOverdraw<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(&self, subtree: Param0, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), subtree.into_param().abi(), contentkinds).ok() }
    }
    pub fn ShowRedraw<'a, Param0: ::windows::core::IntoParam<'a, super::Visual>>(&self, subtree: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), subtree.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionDebugHeatMaps {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps;{e49c90ac-2ff3-5805-718c-b725ee07650f})");
}
unsafe impl ::windows::core::Interface for CompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49c90ac_2ff3_5805_718c_b725ee07650f);
}
impl ::windows::core::RuntimeName for CompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps";
}
impl ::core::convert::From<CompositionDebugHeatMaps> for ::windows::core::IUnknown {
    fn from(value: CompositionDebugHeatMaps) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionDebugHeatMaps> for ::windows::core::IUnknown {
    fn from(value: &CompositionDebugHeatMaps) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionDebugHeatMaps> for ::windows::core::IInspectable {
    fn from(value: CompositionDebugHeatMaps) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionDebugHeatMaps> for ::windows::core::IInspectable {
    fn from(value: &CompositionDebugHeatMaps) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositionDebugHeatMaps {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompositionDebugHeatMaps {}
unsafe impl ::core::marker::Sync for CompositionDebugHeatMaps {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for CompositionDebugOverdrawContentKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CompositionDebugOverdrawContentKinds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CompositionDebugOverdrawContentKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Diagnostics.CompositionDebugOverdrawContentKinds;u4)");
}
impl ::windows::core::DefaultType for CompositionDebugOverdrawContentKinds {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for CompositionDebugOverdrawContentKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for CompositionDebugOverdrawContentKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CompositionDebugSettings(pub ::windows::core::IInspectable);
impl CompositionDebugSettings {
    pub fn HeatMaps(&self) -> ::windows::core::Result<CompositionDebugHeatMaps> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CompositionDebugHeatMaps>(result__)
        }
    }
    pub fn TryGetSettings<'a, Param0: ::windows::core::IntoParam<'a, super::Compositor>>(compositor: Param0) -> ::windows::core::Result<CompositionDebugSettings> {
        Self::ICompositionDebugSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), compositor.into_param().abi(), &mut result__).from_abi::<CompositionDebugSettings>(result__)
        })
    }
    pub fn ICompositionDebugSettingsStatics<R, F: FnOnce(&ICompositionDebugSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CompositionDebugSettings, ICompositionDebugSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionDebugSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugSettings;{2831987e-1d82-4d38-b7b7-efd11c7bc3d1})");
}
unsafe impl ::windows::core::Interface for CompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2831987e_1d82_4d38_b7b7_efd11c7bc3d1);
}
impl ::windows::core::RuntimeName for CompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugSettings";
}
impl ::core::convert::From<CompositionDebugSettings> for ::windows::core::IUnknown {
    fn from(value: CompositionDebugSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CompositionDebugSettings> for ::windows::core::IUnknown {
    fn from(value: &CompositionDebugSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CompositionDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CompositionDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CompositionDebugSettings> for ::windows::core::IInspectable {
    fn from(value: CompositionDebugSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CompositionDebugSettings> for ::windows::core::IInspectable {
    fn from(value: &CompositionDebugSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CompositionDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CompositionDebugSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CompositionDebugSettings {}
unsafe impl ::core::marker::Sync for CompositionDebugSettings {}
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionDebugHeatMaps(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49c90ac_2ff3_5805_718c_b725ee07650f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugHeatMaps_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtree: ::windows::core::RawPtr, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionDebugSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2831987e_1d82_4d38_b7b7_efd11c7bc3d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICompositionDebugSettingsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICompositionDebugSettingsStatics {
    type Vtable = ICompositionDebugSettingsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ec1f1e_6af8_4af8_b814_c870fd5a9505);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
