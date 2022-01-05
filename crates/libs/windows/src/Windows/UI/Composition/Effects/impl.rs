#[cfg(feature = "implement_exclusive")]
pub trait ISceneLightingEffectImpl: Sized {
    fn AmbientAmount(&self) -> ::windows::core::Result<f32>;
    fn SetAmbientAmount(&self, value: f32) -> ::windows::core::Result<()>;
    fn DiffuseAmount(&self) -> ::windows::core::Result<f32>;
    fn SetDiffuseAmount(&self, value: f32) -> ::windows::core::Result<()>;
    fn NormalMapSource(&self) -> ::windows::core::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn SetNormalMapSource(&self, value: &::core::option::Option<super::super::super::Graphics::Effects::IGraphicsEffectSource>) -> ::windows::core::Result<()>;
    fn SpecularAmount(&self) -> ::windows::core::Result<f32>;
    fn SetSpecularAmount(&self, value: f32) -> ::windows::core::Result<()>;
    fn SpecularShine(&self) -> ::windows::core::Result<f32>;
    fn SetSpecularShine(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneLightingEffect2Impl: Sized {
    fn ReflectanceModel(&self) -> ::windows::core::Result<SceneLightingEffectReflectanceModel>;
    fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows::core::Result<()>;
}
