#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneLightingEffect(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISceneLightingEffect {
    type Vtable = ISceneLightingEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffectVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneLightingEffect2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISceneLightingEffect2 {
    type Vtable = ISceneLightingEffect2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e270e81_72f0_4c5c_95f8_8a6e0024f409);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Composition_Effects'*"]
#[repr(transparent)]
pub struct SceneLightingEffect(::windows::core::IUnknown);
impl SceneLightingEffect {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SceneLightingEffect, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Composition_Effects', 'Graphics_Effects'*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects', 'Graphics_Effects'*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn AmbientAmount(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SetAmbientAmount(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn DiffuseAmount(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SetDiffuseAmount(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Composition_Effects', 'Graphics_Effects'*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn NormalMapSource(&self) -> ::windows::core::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Effects::IGraphicsEffectSource>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects', 'Graphics_Effects'*"]
    #[cfg(feature = "Graphics_Effects")]
    pub fn SetNormalMapSource<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SpecularAmount(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SetSpecularAmount(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SpecularShine(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SetSpecularShine(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn ReflectanceModel(&self) -> ::windows::core::Result<SceneLightingEffectReflectanceModel> {
        let this = &::windows::core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__: SceneLightingEffectReflectanceModel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SceneLightingEffectReflectanceModel>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Composition_Effects'*"]
    pub fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for SceneLightingEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SceneLightingEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Effects.SceneLightingEffect;{91bb5e52-95d1-4f8b-9a5a-6408b24b8c6a})");
}
unsafe impl ::windows::core::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffectVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91bb5e52_95d1_4f8b_9a5a_6408b24b8c6a);
}
impl ::windows::core::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.SceneLightingEffect";
}
impl ::core::convert::From<SceneLightingEffect> for ::windows::core::IUnknown {
    fn from(value: SceneLightingEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows::core::IUnknown {
    fn from(value: &SceneLightingEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SceneLightingEffect> for ::windows::core::IInspectable {
    fn from(value: SceneLightingEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SceneLightingEffect> for ::windows::core::IInspectable {
    fn from(value: &SceneLightingEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffect {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffect> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffect> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffect> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffect> {
        ::core::convert::TryInto::<super::super::super::Graphics::Effects::IGraphicsEffect>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> for SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        ::core::convert::TryInto::<super::super::super::Graphics::Effects::IGraphicsEffectSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SceneLightingEffect {}
unsafe impl ::core::marker::Sync for SceneLightingEffect {}
#[doc = "*Required features: 'UI_Composition_Effects'*"]
#[repr(transparent)]
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
unsafe impl ::windows::core::Abi for SceneLightingEffectReflectanceModel {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SceneLightingEffectReflectanceModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneLightingEffectReflectanceModel {}
impl ::core::fmt::Debug for SceneLightingEffectReflectanceModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneLightingEffectReflectanceModel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)");
}
impl ::windows::core::DefaultType for SceneLightingEffectReflectanceModel {
    type DefaultType = Self;
}
