#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugHeatMaps_Impl: Sized {
    fn Hide(&mut self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
    fn ShowMemoryUsage(&mut self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
    fn ShowOverdraw(&mut self, subtree: &::core::option::Option<super::Visual>, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::Result<()>;
    fn ShowRedraw(&mut self, subtree: &::core::option::Option<super::Visual>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDebugHeatMaps {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.ICompositionDebugHeatMaps";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDebugHeatMaps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDebugHeatMaps_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDebugHeatMaps_Vtbl {
        unsafe extern "system" fn Hide<Impl: ICompositionDebugHeatMaps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowMemoryUsage<Impl: ICompositionDebugHeatMaps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowMemoryUsage(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowOverdraw<Impl: ICompositionDebugHeatMaps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr, contentkinds: CompositionDebugOverdrawContentKinds) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowOverdraw(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType), contentkinds).into()
        }
        unsafe extern "system" fn ShowRedraw<Impl: ICompositionDebugHeatMaps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subtree: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowRedraw(&*(&subtree as *const <super::Visual as ::windows::core::Abi>::Abi as *const <super::Visual as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDebugHeatMaps, BASE_OFFSET>(),
            Hide: Hide::<Impl, IMPL_OFFSET>,
            ShowMemoryUsage: ShowMemoryUsage::<Impl, IMPL_OFFSET>,
            ShowOverdraw: ShowOverdraw::<Impl, IMPL_OFFSET>,
            ShowRedraw: ShowRedraw::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDebugHeatMaps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugSettings_Impl: Sized {
    fn HeatMaps(&mut self) -> ::windows::core::Result<CompositionDebugHeatMaps>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDebugSettings {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.ICompositionDebugSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDebugSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDebugSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDebugSettings_Vtbl {
        unsafe extern "system" fn HeatMaps<Impl: ICompositionDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDebugSettings, BASE_OFFSET>(), HeatMaps: HeatMaps::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDebugSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICompositionDebugSettingsStatics_Impl: Sized {
    fn TryGetSettings(&mut self, compositor: &::core::option::Option<super::Compositor>) -> ::windows::core::Result<CompositionDebugSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICompositionDebugSettingsStatics {
    const NAME: &'static str = "Windows.UI.Composition.Diagnostics.ICompositionDebugSettingsStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICompositionDebugSettingsStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionDebugSettingsStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionDebugSettingsStatics_Vtbl {
        unsafe extern "system" fn TryGetSettings<Impl: ICompositionDebugSettingsStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICompositionDebugSettingsStatics, BASE_OFFSET>(),
            TryGetSettings: TryGetSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionDebugSettingsStatics as ::windows::core::Interface>::IID
    }
}
