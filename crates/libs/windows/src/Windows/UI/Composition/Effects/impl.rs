#[cfg(all(feature = "Graphics_Effects", feature = "implement_exclusive"))]
pub trait ISceneLightingEffect_Impl: Sized {
    fn AmbientAmount(&mut self) -> ::windows::core::Result<f32>;
    fn SetAmbientAmount(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn DiffuseAmount(&mut self) -> ::windows::core::Result<f32>;
    fn SetDiffuseAmount(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn NormalMapSource(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource>;
    fn SetNormalMapSource(&mut self, value: &::core::option::Option<super::super::super::Graphics::Effects::IGraphicsEffectSource>) -> ::windows::core::Result<()>;
    fn SpecularAmount(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpecularAmount(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn SpecularShine(&mut self) -> ::windows::core::Result<f32>;
    fn SetSpecularShine(&mut self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Graphics_Effects", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISceneLightingEffect {
    const NAME: &'static str = "Windows.UI.Composition.Effects.ISceneLightingEffect";
}
#[cfg(all(feature = "Graphics_Effects", feature = "implement_exclusive"))]
impl ISceneLightingEffect_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneLightingEffect_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneLightingEffect_Vtbl {
        unsafe extern "system" fn AmbientAmount<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AmbientAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAmbientAmount<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAmbientAmount(value).into()
        }
        unsafe extern "system" fn DiffuseAmount<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiffuseAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDiffuseAmount<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDiffuseAmount(value).into()
        }
        unsafe extern "system" fn NormalMapSource<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NormalMapSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNormalMapSource<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNormalMapSource(&*(&value as *const <super::super::super::Graphics::Effects::IGraphicsEffectSource as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Effects::IGraphicsEffectSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpecularAmount<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpecularAmount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpecularAmount<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpecularAmount(value).into()
        }
        unsafe extern "system" fn SpecularShine<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpecularShine() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpecularShine<Impl: ISceneLightingEffect_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpecularShine(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneLightingEffect, BASE_OFFSET>(),
            AmbientAmount: AmbientAmount::<Impl, IMPL_OFFSET>,
            SetAmbientAmount: SetAmbientAmount::<Impl, IMPL_OFFSET>,
            DiffuseAmount: DiffuseAmount::<Impl, IMPL_OFFSET>,
            SetDiffuseAmount: SetDiffuseAmount::<Impl, IMPL_OFFSET>,
            NormalMapSource: NormalMapSource::<Impl, IMPL_OFFSET>,
            SetNormalMapSource: SetNormalMapSource::<Impl, IMPL_OFFSET>,
            SpecularAmount: SpecularAmount::<Impl, IMPL_OFFSET>,
            SetSpecularAmount: SetSpecularAmount::<Impl, IMPL_OFFSET>,
            SpecularShine: SpecularShine::<Impl, IMPL_OFFSET>,
            SetSpecularShine: SetSpecularShine::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneLightingEffect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISceneLightingEffect2_Impl: Sized {
    fn ReflectanceModel(&mut self) -> ::windows::core::Result<SceneLightingEffectReflectanceModel>;
    fn SetReflectanceModel(&mut self, value: SceneLightingEffectReflectanceModel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISceneLightingEffect2 {
    const NAME: &'static str = "Windows.UI.Composition.Effects.ISceneLightingEffect2";
}
#[cfg(feature = "implement_exclusive")]
impl ISceneLightingEffect2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISceneLightingEffect2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISceneLightingEffect2_Vtbl {
        unsafe extern "system" fn ReflectanceModel<Impl: ISceneLightingEffect2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReflectanceModel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReflectanceModel<Impl: ISceneLightingEffect2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SceneLightingEffectReflectanceModel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReflectanceModel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISceneLightingEffect2, BASE_OFFSET>(),
            ReflectanceModel: ReflectanceModel::<Impl, IMPL_OFFSET>,
            SetReflectanceModel: SetReflectanceModel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISceneLightingEffect2 as ::windows::core::Interface>::IID
    }
}
