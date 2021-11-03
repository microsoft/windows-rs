#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISceneLightingEffect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneLightingEffect {
    type Vtable = ISceneLightingEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2444975698, 38353, 20363, [154, 90, 100, 8, 178, 75, 140, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISceneLightingEffect2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISceneLightingEffect2 {
    type Vtable = ISceneLightingEffect2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2653359745, 29424, 19548, [149, 248, 138, 110, 0, 36, 244, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneLightingEffect2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SceneLightingEffectReflectanceModel) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Composition_Effects`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SceneLightingEffect(::windows::runtime::IInspectable);
impl SceneLightingEffect {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SceneLightingEffect, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Graphics_Effects")]
    #[doc = "*Required features: `UI_Composition_Effects`, `Graphics_Effects`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    #[doc = "*Required features: `UI_Composition_Effects`, `Graphics_Effects`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Graphics::Effects::IGraphicsEffect>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn AmbientAmount(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetAmbientAmount(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn DiffuseAmount(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetDiffuseAmount(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_Effects")]
    #[doc = "*Required features: `UI_Composition_Effects`, `Graphics_Effects`*"]
    pub fn NormalMapSource(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Effects::IGraphicsEffectSource>(result__)
        }
    }
    #[cfg(feature = "Graphics_Effects")]
    #[doc = "*Required features: `UI_Composition_Effects`, `Graphics_Effects`*"]
    pub fn SetNormalMapSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SpecularAmount(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetSpecularAmount(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SpecularShine(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetSpecularShine(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn ReflectanceModel(&self) -> ::windows::runtime::Result<SceneLightingEffectReflectanceModel> {
        let this = &::windows::runtime::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe {
            let mut result__: SceneLightingEffectReflectanceModel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SceneLightingEffectReflectanceModel>(result__)
        }
    }
    #[doc = "*Required features: `UI_Composition_Effects`*"]
    pub fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISceneLightingEffect2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SceneLightingEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Effects.SceneLightingEffect;{91bb5e52-95d1-4f8b-9a5a-6408b24b8c6a})");
}
unsafe impl ::windows::runtime::Interface for SceneLightingEffect {
    type Vtable = ISceneLightingEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2444975698, 38353, 20363, [154, 90, 100, 8, 178, 75, 140, 106]);
}
impl ::windows::runtime::RuntimeName for SceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.SceneLightingEffect";
}
#[cfg(feature = "Graphics_Effects")]
impl ::std::convert::TryFrom<SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::std::convert::TryFrom<&SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffect {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffect> for SceneLightingEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffect> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffect> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffect> {
        ::std::convert::TryInto::<super::super::super::Graphics::Effects::IGraphicsEffect>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::std::convert::TryFrom<SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::std::convert::TryFrom<&SceneLightingEffect> for super::super::super::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SceneLightingEffect) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> for SceneLightingEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> for &SceneLightingEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        ::std::convert::TryInto::<super::super::super::Graphics::Effects::IGraphicsEffectSource>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SceneLightingEffect {}
unsafe impl ::std::marker::Sync for SceneLightingEffect {}
#[doc = "*Required features: `UI_Composition_Effects`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: SceneLightingEffectReflectanceModel = SceneLightingEffectReflectanceModel(0i32);
    pub const PhysicallyBasedBlinnPhong: SceneLightingEffectReflectanceModel = SceneLightingEffectReflectanceModel(1i32);
}
impl ::std::convert::From<i32> for SceneLightingEffectReflectanceModel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SceneLightingEffectReflectanceModel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SceneLightingEffectReflectanceModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Composition.Effects.SceneLightingEffectReflectanceModel;i4)");
}
