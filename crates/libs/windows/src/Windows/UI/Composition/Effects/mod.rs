#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneLightingEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneLightingEffect {
    type Vtable = ISceneLightingEffect_Vtbl;
}
impl ::core::clone::Clone for ISceneLightingEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneLightingEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AmbientAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetAmbientAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub DiffuseAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetDiffuseAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Effects")]
    pub NormalMapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    NormalMapSource: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub SetNormalMapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    SetNormalMapSource: usize,
    pub SpecularAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpecularAmount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
    pub SpecularShine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT,
    pub SetSpecularShine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneLightingEffect2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneLightingEffect2 {
    type Vtable = ISceneLightingEffect2_Vtbl;
}
impl ::core::clone::Clone for ISceneLightingEffect2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneLightingEffect2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e270e81_72f0_4c5c_95f8_8a6e0024f409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ReflectanceModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows_core::HRESULT,
    pub SetReflectanceModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneLightingEffectReflectanceModel) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"UI_Composition_Effects\"`*"]
#[repr(transparent)]
pub struct SceneLightingEffect(::windows_core::IUnknown);
impl SceneLightingEffect {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneLightingEffect, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetName(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(name)).ok() }
    }
    pub fn AmbientAmount(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AmbientAmount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAmbientAmount(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAmbientAmount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DiffuseAmount(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DiffuseAmount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDiffuseAmount(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDiffuseAmount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn NormalMapSource(&self) -> ::windows_core::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NormalMapSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetNormalMapSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Graphics::Effects::IGraphicsEffectSource>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNormalMapSource)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn SpecularAmount(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpecularAmount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpecularAmount(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpecularAmount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SpecularShine(&self) -> ::windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpecularShine)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpecularShine(&self, value: f32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpecularShine)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReflectanceModel(&self) -> ::windows_core::Result<SceneLightingEffectReflectanceModel> {
        let this = &::windows_core::ComInterface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReflectanceModel)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ISceneLightingEffect2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetReflectanceModel)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneLightingEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneLightingEffect {}
impl ::core::fmt::Debug for SceneLightingEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneLightingEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneLightingEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Effects.SceneLightingEffect;{91bb5e52-95d1-4f8b-9a5a-6408b24b8c6a})");
}
impl ::core::clone::Clone for SceneLightingEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneLightingEffect {
    const IID: ::windows_core::GUID = <ISceneLightingEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.SceneLightingEffect";
}
::windows_core::imp::interface_hierarchy!(SceneLightingEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Graphics_Effects")]
impl ::windows_core::CanTryInto<super::super::super::Graphics::Effects::IGraphicsEffect> for SceneLightingEffect {}
#[cfg(feature = "Graphics_Effects")]
impl ::windows_core::CanTryInto<super::super::super::Graphics::Effects::IGraphicsEffectSource> for SceneLightingEffect {}
unsafe impl ::core::marker::Send for SceneLightingEffect {}
unsafe impl ::core::marker::Sync for SceneLightingEffect {}
#[doc = "*Required features: `\"UI_Composition_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: Self = Self(0i32);
    pub const PhysicallyBasedBlinnPhong: Self = Self(1i32);
}
impl ::core::marker::Copy for SceneLightingEffectReflectanceModel {}
impl ::core::clone::Clone for SceneLightingEffectReflectanceModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneLightingEffectReflectanceModel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SceneLightingEffectReflectanceModel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneLightingEffectReflectanceModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneLightingEffectReflectanceModel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)");
}
