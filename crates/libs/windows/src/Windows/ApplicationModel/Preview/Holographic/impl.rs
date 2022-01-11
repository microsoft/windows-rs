#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IHolographicApplicationPreviewStaticsImpl: Sized {
    fn IsCurrentViewPresentedOnHolographicDisplay(&self) -> ::windows::core::Result<bool>;
    fn IsHolographicActivation(&self, activatedeventargs: &::core::option::Option<super::super::Activation::IActivatedEventArgs>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicApplicationPreviewStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl IHolographicApplicationPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicApplicationPreviewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicApplicationPreviewStaticsVtbl {
        unsafe extern "system" fn IsCurrentViewPresentedOnHolographicDisplay<Impl: IHolographicApplicationPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentViewPresentedOnHolographicDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHolographicActivation<Impl: IHolographicApplicationPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatedeventargs: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHolographicActivation(&*(&activatedeventargs as *const <super::super::Activation::IActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::Activation::IActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicApplicationPreviewStatics>, ::windows::core::GetTrustLevel, IsCurrentViewPresentedOnHolographicDisplay::<Impl, IMPL_OFFSET>, IsHolographicActivation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicApplicationPreviewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardPlacementOverridePreviewImpl: Sized {
    fn SetPlacementOverride(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::super::Foundation::Numerics::Vector3, normal: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetPlacementOverrideWithMaxSize(&self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::super::Foundation::Numerics::Vector3, normal: &super::super::super::Foundation::Numerics::Vector3, maxsize: &super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ResetPlacementOverride(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicKeyboardPlacementOverridePreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicKeyboardPlacementOverridePreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicKeyboardPlacementOverridePreviewVtbl {
        unsafe extern "system" fn SetPlacementOverride<Impl: IHolographicKeyboardPlacementOverridePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPlacementOverride(
                    &*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&topcenterposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPlacementOverrideWithMaxSize<Impl: IHolographicKeyboardPlacementOverridePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPlacementOverrideWithMaxSize(
                    &*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&topcenterposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&maxsize as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn ResetPlacementOverride<Impl: IHolographicKeyboardPlacementOverridePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetPlacementOverride().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicKeyboardPlacementOverridePreview>, ::windows::core::GetTrustLevel, SetPlacementOverride::<Impl, IMPL_OFFSET>, SetPlacementOverrideWithMaxSize::<Impl, IMPL_OFFSET>, ResetPlacementOverride::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicKeyboardPlacementOverridePreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardPlacementOverridePreviewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<HolographicKeyboardPlacementOverridePreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicKeyboardPlacementOverridePreviewStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicKeyboardPlacementOverridePreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicKeyboardPlacementOverridePreviewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicKeyboardPlacementOverridePreviewStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IHolographicKeyboardPlacementOverridePreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicKeyboardPlacementOverridePreviewStatics>, ::windows::core::GetTrustLevel, GetForCurrentView::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicKeyboardPlacementOverridePreviewStatics as ::windows::core::Interface>::IID
    }
}
