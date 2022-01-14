#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IHolographicApplicationPreviewStatics_Impl: Sized {
    fn IsCurrentViewPresentedOnHolographicDisplay(&mut self) -> ::windows::core::Result<bool>;
    fn IsHolographicActivation(&mut self, activatedeventargs: &::core::option::Option<super::super::Activation::IActivatedEventArgs>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicApplicationPreviewStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.IHolographicApplicationPreviewStatics";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl IHolographicApplicationPreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicApplicationPreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicApplicationPreviewStatics_Vtbl {
        unsafe extern "system" fn IsCurrentViewPresentedOnHolographicDisplay<Impl: IHolographicApplicationPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsHolographicActivation<Impl: IHolographicApplicationPreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatedeventargs: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicApplicationPreviewStatics, BASE_OFFSET>(),
            IsCurrentViewPresentedOnHolographicDisplay: IsCurrentViewPresentedOnHolographicDisplay::<Impl, IMPL_OFFSET>,
            IsHolographicActivation: IsHolographicActivation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicApplicationPreviewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardPlacementOverridePreview_Impl: Sized {
    fn SetPlacementOverride(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::super::Foundation::Numerics::Vector3, normal: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetPlacementOverrideWithMaxSize(&mut self, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, topcenterposition: &super::super::super::Foundation::Numerics::Vector3, normal: &super::super::super::Foundation::Numerics::Vector3, maxsize: &super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn ResetPlacementOverride(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicKeyboardPlacementOverridePreview {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreview";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicKeyboardPlacementOverridePreview_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicKeyboardPlacementOverridePreview_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicKeyboardPlacementOverridePreview_Vtbl {
        unsafe extern "system" fn SetPlacementOverride<Impl: IHolographicKeyboardPlacementOverridePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPlacementOverride(
                    &*(&coordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&topcenterposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPlacementOverrideWithMaxSize<Impl: IHolographicKeyboardPlacementOverridePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, topcenterposition: super::super::super::Foundation::Numerics::Vector3, normal: super::super::super::Foundation::Numerics::Vector3, maxsize: super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ResetPlacementOverride<Impl: IHolographicKeyboardPlacementOverridePreview_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetPlacementOverride().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicKeyboardPlacementOverridePreview, BASE_OFFSET>(),
            SetPlacementOverride: SetPlacementOverride::<Impl, IMPL_OFFSET>,
            SetPlacementOverrideWithMaxSize: SetPlacementOverrideWithMaxSize::<Impl, IMPL_OFFSET>,
            ResetPlacementOverride: ResetPlacementOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicKeyboardPlacementOverridePreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicKeyboardPlacementOverridePreviewStatics_Impl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<HolographicKeyboardPlacementOverridePreview>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicKeyboardPlacementOverridePreviewStatics {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.Holographic.IHolographicKeyboardPlacementOverridePreviewStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicKeyboardPlacementOverridePreviewStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicKeyboardPlacementOverridePreviewStatics_Vtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IHolographicKeyboardPlacementOverridePreviewStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicKeyboardPlacementOverridePreviewStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicKeyboardPlacementOverridePreviewStatics as ::windows::core::Interface>::IID
    }
}
