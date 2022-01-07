#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugHeatMapsImpl: Sized {
    fn Hide(&self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
    fn ShowMemoryUsage(&self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
    fn ShowOverdraw(&self, subtree: &::core::option::Option<super::Visual>, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::Result<()>;
    fn ShowRedraw(&self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.ICompositionDebugHeatMaps";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDebugHeatMapsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDebugHeatMapsImpl, const OFFSET: isize>() -> ICompositionDebugHeatMapsVtbl {
        unsafe extern "system" fn Hide<Impl: ICompositionDebugHeatMapsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowMemoryUsage<Impl: ICompositionDebugHeatMapsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowMemoryUsage(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowOverdraw<Impl: ICompositionDebugHeatMapsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowOverdraw(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType), contentkinds).into()
        }
        unsafe extern "system" fn ShowRedraw<Impl: ICompositionDebugHeatMapsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowRedraw(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompositionDebugHeatMaps>, ::windows::core::GetTrustLevel, Hide::<Impl, OFFSET>, ShowMemoryUsage::<Impl, OFFSET>, ShowOverdraw::<Impl, OFFSET>, ShowRedraw::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugSettingsImpl: Sized {
    fn HeatMaps(&self) -> ::windows::core::Result<CompositionDebugHeatMaps>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.ICompositionDebugSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDebugSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDebugSettingsImpl, const OFFSET: isize>() -> ICompositionDebugSettingsVtbl {
        unsafe extern "system" fn HeatMaps<Impl: ICompositionDebugSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeatMaps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompositionDebugSettings>, ::windows::core::GetTrustLevel, HeatMaps::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugSettingsStaticsImpl: Sized {
    fn TryGetSettings(&self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<CompositionDebugSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDebugSettingsStatics {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.ICompositionDebugSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDebugSettingsStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDebugSettingsStaticsImpl, const OFFSET: isize>() -> ICompositionDebugSettingsStaticsVtbl {
        unsafe extern "system" fn TryGetSettings<Impl: ICompositionDebugSettingsStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetSettings(&*(&compositor as *const <super::Compositor as ::windows::core::Abi>::Abi as *const <super::Compositor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICompositionDebugSettingsStatics>, ::windows::core::GetTrustLevel, TryGetSettings::<Impl, OFFSET>)
    }
}
