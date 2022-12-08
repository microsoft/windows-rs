#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionDebugHeatMaps(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionDebugHeatMaps {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49c90ac_2ff3_5805_718c_b725ee07650f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugHeatMaps_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowMemoryUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowOverdraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::HRESULT,
    pub ShowRedraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtree: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionDebugSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionDebugSettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2831987e_1d82_4d38_b7b7_efd11c7bc3d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettings_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HeatMaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompositionDebugSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICompositionDebugSettingsStatics {
    type Vtable = ICompositionDebugSettingsStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICompositionDebugSettingsStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ec1f1e_6af8_4af8_b814_c870fd5a9505);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDebugSettingsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TryGetSettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CompositionDebugHeatMaps(::windows::core::IUnknown);
impl CompositionDebugHeatMaps {
    pub fn Hide<P0>(&self, subtree: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Visual>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Hide)(::windows::core::Vtable::as_raw(this), subtree.into().abi()).ok() }
    }
    pub fn ShowMemoryUsage<P0>(&self, subtree: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Visual>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowMemoryUsage)(::windows::core::Vtable::as_raw(this), subtree.into().abi()).ok() }
    }
    pub fn ShowOverdraw<P0>(&self, subtree: P0, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Visual>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowOverdraw)(::windows::core::Vtable::as_raw(this), subtree.into().abi(), contentkinds).ok() }
    }
    pub fn ShowRedraw<P0>(&self, subtree: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Visual>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).ShowRedraw)(::windows::core::Vtable::as_raw(this), subtree.into().abi()).ok() }
    }
}
impl ::core::clone::Clone for CompositionDebugHeatMaps {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionDebugHeatMaps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionDebugHeatMaps {}
impl ::core::fmt::Debug for CompositionDebugHeatMaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionDebugHeatMaps").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionDebugHeatMaps {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps;{e49c90ac-2ff3-5805-718c-b725ee07650f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CompositionDebugHeatMaps {
    type Vtable = ICompositionDebugHeatMaps_Vtbl;
}
unsafe impl ::windows::core::Interface for CompositionDebugHeatMaps {
    const IID: ::windows::core::GUID = <ICompositionDebugHeatMaps as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugHeatMaps";
}
::windows::core::interface_hierarchy!(CompositionDebugHeatMaps, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CompositionDebugHeatMaps {}
unsafe impl ::core::marker::Sync for CompositionDebugHeatMaps {}
#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
pub struct CompositionDebugSettings(::windows::core::IUnknown);
impl CompositionDebugSettings {
    pub fn HeatMaps(&self) -> ::windows::core::Result<CompositionDebugHeatMaps> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeatMaps)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TryGetSettings(compositor: &super::Compositor) -> ::windows::core::Result<CompositionDebugSettings> {
        Self::ICompositionDebugSettingsStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TryGetSettings)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(compositor), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICompositionDebugSettingsStatics<R, F: FnOnce(&ICompositionDebugSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CompositionDebugSettings, ICompositionDebugSettingsStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CompositionDebugSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CompositionDebugSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompositionDebugSettings {}
impl ::core::fmt::Debug for CompositionDebugSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionDebugSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionDebugSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Diagnostics.CompositionDebugSettings;{2831987e-1d82-4d38-b7b7-efd11c7bc3d1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CompositionDebugSettings {
    type Vtable = ICompositionDebugSettings_Vtbl;
}
unsafe impl ::windows::core::Interface for CompositionDebugSettings {
    const IID: ::windows::core::GUID = <ICompositionDebugSettings as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.CompositionDebugSettings";
}
::windows::core::interface_hierarchy!(CompositionDebugSettings, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for CompositionDebugSettings {}
unsafe impl ::core::marker::Sync for CompositionDebugSettings {}
#[doc = "*Required features: `\"UI_Composition_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: Self = Self(0u32);
    pub const OffscreenRendered: Self = Self(1u32);
    pub const Colors: Self = Self(2u32);
    pub const Effects: Self = Self(4u32);
    pub const Shadows: Self = Self(8u32);
    pub const Lights: Self = Self(16u32);
    pub const Surfaces: Self = Self(32u32);
    pub const SwapChains: Self = Self(64u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for CompositionDebugOverdrawContentKinds {}
impl ::core::clone::Clone for CompositionDebugOverdrawContentKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompositionDebugOverdrawContentKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CompositionDebugOverdrawContentKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for CompositionDebugOverdrawContentKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionDebugOverdrawContentKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CompositionDebugOverdrawContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CompositionDebugOverdrawContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CompositionDebugOverdrawContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CompositionDebugOverdrawContentKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Diagnostics.CompositionDebugOverdrawContentKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
