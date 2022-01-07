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
impl ::windows::core::RuntimeName for ISceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.ISceneLightingEffect";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneLightingEffectVtbl {
    pub const fn new<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneLightingEffectVtbl {
        unsafe extern "system" fn AmbientAmount<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AmbientAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmbientAmount<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAmbientAmount(value).into()
        }
        unsafe extern "system" fn DiffuseAmount<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DiffuseAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiffuseAmount<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDiffuseAmount(value).into()
        }
        unsafe extern "system" fn NormalMapSource<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NormalMapSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalMapSource<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNormalMapSource(&*(&value as *const <super::super::super::Graphics::Effects::IGraphicsEffectSource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Effects::IGraphicsEffectSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpecularAmount<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpecularAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpecularAmount<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpecularAmount(value).into()
        }
        unsafe extern "system" fn SpecularShine<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpecularShine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpecularShine<Impl: ISceneLightingEffectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpecularShine(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneLightingEffect>, base.5, AmbientAmount::<Impl, OFFSET>, SetAmbientAmount::<Impl, OFFSET>, DiffuseAmount::<Impl, OFFSET>, SetDiffuseAmount::<Impl, OFFSET>, NormalMapSource::<Impl, OFFSET>, SetNormalMapSource::<Impl, OFFSET>, SpecularAmount::<Impl, OFFSET>, SetSpecularAmount::<Impl, OFFSET>, SpecularShine::<Impl, OFFSET>, SetSpecularShine::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneLightingEffect2Impl: Sized {
    fn ReflectanceModel(&self) -> ::windows::core::Result<SceneLightingEffectReflectanceModel>;
    fn SetReflectanceModel(&self, value: SceneLightingEffectReflectanceModel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneLightingEffect2 {
    const NAME: &'static str = "Windows.UI.Composition.Effects.ISceneLightingEffect2";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneLightingEffect2Vtbl {
    pub const fn new<Impl: ISceneLightingEffect2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISceneLightingEffect2Vtbl {
        unsafe extern "system" fn ReflectanceModel<Impl: ISceneLightingEffect2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReflectanceModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReflectanceModel<Impl: ISceneLightingEffect2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReflectanceModel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISceneLightingEffect2>, base.5, ReflectanceModel::<Impl, OFFSET>, SetReflectanceModel::<Impl, OFFSET>)
    }
}
