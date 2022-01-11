#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicCameraImpl: Sized {
    fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ViewportScaleFactor(&self) -> ::windows::core::Result<f64>;
    fn SetViewportScaleFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn SetNearPlaneDistance(&self, value: f64) -> ::windows::core::Result<()>;
    fn SetFarPlaneDistance(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCamera {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicCameraVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraVtbl {
        unsafe extern "system" fn RenderTargetSize<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTargetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportScaleFactor<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportScaleFactor<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewportScaleFactor(value).into()
        }
        unsafe extern "system" fn IsStereo<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNearPlaneDistance<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNearPlaneDistance(value).into()
        }
        unsafe extern "system" fn SetFarPlaneDistance<Impl: IHolographicCameraImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFarPlaneDistance(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicCamera>,
            ::windows::core::GetTrustLevel,
            RenderTargetSize::<Impl, IMPL_OFFSET>,
            ViewportScaleFactor::<Impl, IMPL_OFFSET>,
            SetViewportScaleFactor::<Impl, IMPL_OFFSET>,
            IsStereo::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            SetNearPlaneDistance::<Impl, IMPL_OFFSET>,
            SetFarPlaneDistance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCamera as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicCamera2Impl: Sized + IHolographicCameraImpl {
    fn LeftViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters>;
    fn RightViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters>;
    fn Display(&self) -> ::windows::core::Result<HolographicDisplay>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCamera2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicCamera2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCamera2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCamera2Vtbl {
        unsafe extern "system" fn LeftViewportParameters<Impl: IHolographicCamera2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeftViewportParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RightViewportParameters<Impl: IHolographicCamera2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightViewportParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: IHolographicCamera2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCamera2>, ::windows::core::GetTrustLevel, LeftViewportParameters::<Impl, IMPL_OFFSET>, RightViewportParameters::<Impl, IMPL_OFFSET>, Display::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCamera2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHolographicCamera3Impl: Sized + IHolographicCameraImpl + IHolographicCamera2Impl {
    fn IsPrimaryLayerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryLayerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxQuadLayerCount(&self) -> ::windows::core::Result<u32>;
    fn QuadLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCamera3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHolographicCamera3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCamera3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCamera3Vtbl {
        unsafe extern "system" fn IsPrimaryLayerEnabled<Impl: IHolographicCamera3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPrimaryLayerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPrimaryLayerEnabled<Impl: IHolographicCamera3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPrimaryLayerEnabled(value).into()
        }
        unsafe extern "system" fn MaxQuadLayerCount<Impl: IHolographicCamera3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxQuadLayerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuadLayers<Impl: IHolographicCamera3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuadLayers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCamera3>, ::windows::core::GetTrustLevel, IsPrimaryLayerEnabled::<Impl, IMPL_OFFSET>, SetIsPrimaryLayerEnabled::<Impl, IMPL_OFFSET>, MaxQuadLayerCount::<Impl, IMPL_OFFSET>, QuadLayers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCamera3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera4Impl: Sized {
    fn CanOverrideViewport(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCamera4 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera4";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCamera4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCamera4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCamera4Vtbl {
        unsafe extern "system" fn CanOverrideViewport<Impl: IHolographicCamera4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanOverrideViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCamera4>, ::windows::core::GetTrustLevel, CanOverrideViewport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCamera4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera5Impl: Sized {
    fn IsHardwareContentProtectionSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHardwareContentProtectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHardwareContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCamera5 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera5";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCamera5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCamera5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCamera5Vtbl {
        unsafe extern "system" fn IsHardwareContentProtectionSupported<Impl: IHolographicCamera5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHardwareContentProtectionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHardwareContentProtectionEnabled<Impl: IHolographicCamera5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHardwareContentProtectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHardwareContentProtectionEnabled<Impl: IHolographicCamera5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHardwareContentProtectionEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCamera5>, ::windows::core::GetTrustLevel, IsHardwareContentProtectionSupported::<Impl, IMPL_OFFSET>, IsHardwareContentProtectionEnabled::<Impl, IMPL_OFFSET>, SetIsHardwareContentProtectionEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCamera5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera6Impl: Sized {
    fn ViewConfiguration(&self) -> ::windows::core::Result<HolographicViewConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCamera6 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera6";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCamera6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCamera6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCamera6Vtbl {
        unsafe extern "system" fn ViewConfiguration<Impl: IHolographicCamera6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCamera6>, ::windows::core::GetTrustLevel, ViewConfiguration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCamera6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicCameraPoseImpl: Sized {
    fn HolographicCamera(&self) -> ::windows::core::Result<HolographicCamera>;
    fn Viewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TryGetViewTransform(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<HolographicStereoTransform>>;
    fn ProjectionTransform(&self) -> ::windows::core::Result<HolographicStereoTransform>;
    fn TryGetCullingFrustum(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>;
    fn TryGetVisibleFrustum(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>;
    fn NearPlaneDistance(&self) -> ::windows::core::Result<f64>;
    fn FarPlaneDistance(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCameraPose {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraPose";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicCameraPoseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraPoseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraPoseVtbl {
        unsafe extern "system" fn HolographicCamera<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HolographicCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Viewport<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Viewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetViewTransform<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetViewTransform(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionTransform<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicStereoTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetCullingFrustum<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetCullingFrustum(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetVisibleFrustum<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetVisibleFrustum(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearPlaneDistance<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NearPlaneDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FarPlaneDistance<Impl: IHolographicCameraPoseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FarPlaneDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicCameraPose>,
            ::windows::core::GetTrustLevel,
            HolographicCamera::<Impl, IMPL_OFFSET>,
            Viewport::<Impl, IMPL_OFFSET>,
            TryGetViewTransform::<Impl, IMPL_OFFSET>,
            ProjectionTransform::<Impl, IMPL_OFFSET>,
            TryGetCullingFrustum::<Impl, IMPL_OFFSET>,
            TryGetVisibleFrustum::<Impl, IMPL_OFFSET>,
            NearPlaneDistance::<Impl, IMPL_OFFSET>,
            FarPlaneDistance::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraPose as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicCameraPose2Impl: Sized {
    fn OverrideViewTransform(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, coordinatesystemtoviewtransform: &HolographicStereoTransform) -> ::windows::core::Result<()>;
    fn OverrideProjectionTransform(&self, projectiontransform: &HolographicStereoTransform) -> ::windows::core::Result<()>;
    fn OverrideViewport(&self, leftviewport: &super::super::Foundation::Rect, rightviewport: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCameraPose2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraPose2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicCameraPose2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraPose2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraPose2Vtbl {
        unsafe extern "system" fn OverrideViewTransform<Impl: IHolographicCameraPose2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideViewTransform(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&coordinatesystemtoviewtransform as *const <HolographicStereoTransform as ::windows::core::Abi>::Abi as *const <HolographicStereoTransform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideProjectionTransform<Impl: IHolographicCameraPose2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, projectiontransform: HolographicStereoTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideProjectionTransform(&*(&projectiontransform as *const <HolographicStereoTransform as ::windows::core::Abi>::Abi as *const <HolographicStereoTransform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideViewport<Impl: IHolographicCameraPose2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OverrideViewport(&*(&leftviewport as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&rightviewport as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCameraPose2>, ::windows::core::GetTrustLevel, OverrideViewTransform::<Impl, IMPL_OFFSET>, OverrideProjectionTransform::<Impl, IMPL_OFFSET>, OverrideViewport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraPose2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicCameraRenderingParametersImpl: Sized {
    fn SetFocusPoint(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetFocusPointWithNormal(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, normal: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetFocusPointWithNormalLinearVelocity(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, normal: &super::super::Foundation::Numerics::Vector3, linearvelocity: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DDevice>;
    fn Direct3D11BackBuffer(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicCameraRenderingParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraRenderingParametersVtbl {
        unsafe extern "system" fn SetFocusPoint<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusPoint(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFocusPointWithNormal<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetFocusPointWithNormal(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetFocusPointWithNormalLinearVelocity<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetFocusPointWithNormalLinearVelocity(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&linearvelocity as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Direct3D11Device<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3D11Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3D11BackBuffer<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direct3D11BackBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters>,
            ::windows::core::GetTrustLevel,
            SetFocusPoint::<Impl, IMPL_OFFSET>,
            SetFocusPointWithNormal::<Impl, IMPL_OFFSET>,
            SetFocusPointWithNormalLinearVelocity::<Impl, IMPL_OFFSET>,
            Direct3D11Device::<Impl, IMPL_OFFSET>,
            Direct3D11BackBuffer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicCameraRenderingParameters2Impl: Sized + IHolographicCameraRenderingParametersImpl {
    fn ReprojectionMode(&self) -> ::windows::core::Result<HolographicReprojectionMode>;
    fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> ::windows::core::Result<()>;
    fn CommitDirect3D11DepthBuffer(&self, value: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicCameraRenderingParameters2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParameters2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraRenderingParameters2Vtbl {
        unsafe extern "system" fn ReprojectionMode<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicReprojectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReprojectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReprojectionMode<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HolographicReprojectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReprojectionMode(value).into()
        }
        unsafe extern "system" fn CommitDirect3D11DepthBuffer<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitDirect3D11DepthBuffer(&*(&value as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters2>, ::windows::core::GetTrustLevel, ReprojectionMode::<Impl, IMPL_OFFSET>, SetReprojectionMode::<Impl, IMPL_OFFSET>, CommitDirect3D11DepthBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParameters2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicCameraRenderingParameters3Impl: Sized + IHolographicCameraRenderingParametersImpl + IHolographicCameraRenderingParameters2Impl {
    fn IsContentProtectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicCameraRenderingParameters3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParameters3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraRenderingParameters3Vtbl {
        unsafe extern "system" fn IsContentProtectionEnabled<Impl: IHolographicCameraRenderingParameters3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentProtectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsContentProtectionEnabled<Impl: IHolographicCameraRenderingParameters3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsContentProtectionEnabled(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters3>, ::windows::core::GetTrustLevel, IsContentProtectionEnabled::<Impl, IMPL_OFFSET>, SetIsContentProtectionEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParameters3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParameters4Impl: Sized {
    fn DepthReprojectionMethod(&self) -> ::windows::core::Result<HolographicDepthReprojectionMethod>;
    fn SetDepthReprojectionMethod(&self, value: HolographicDepthReprojectionMethod) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters4 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters4";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraRenderingParameters4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraRenderingParameters4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraRenderingParameters4Vtbl {
        unsafe extern "system" fn DepthReprojectionMethod<Impl: IHolographicCameraRenderingParameters4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicDepthReprojectionMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DepthReprojectionMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthReprojectionMethod<Impl: IHolographicCameraRenderingParameters4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HolographicDepthReprojectionMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDepthReprojectionMethod(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters4>, ::windows::core::GetTrustLevel, DepthReprojectionMethod::<Impl, IMPL_OFFSET>, SetDepthReprojectionMethod::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraRenderingParameters4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait IHolographicCameraViewportParametersImpl: Sized {
    fn HiddenAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>>;
    fn VisibleAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicCameraViewportParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraViewportParameters";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IHolographicCameraViewportParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicCameraViewportParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicCameraViewportParametersVtbl {
        unsafe extern "system" fn HiddenAreaMesh<Impl: IHolographicCameraViewportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HiddenAreaMesh() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibleAreaMesh<Impl: IHolographicCameraViewportParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibleAreaMesh() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicCameraViewportParameters>, ::windows::core::GetTrustLevel, HiddenAreaMesh::<Impl, IMPL_OFFSET>, VisibleAreaMesh::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicCameraViewportParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicDisplayImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxViewportSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn IsOpaque(&self) -> ::windows::core::Result<bool>;
    fn AdapterId(&self) -> ::windows::core::Result<HolographicAdapterId>;
    fn SpatialLocator(&self) -> ::windows::core::Result<super::super::Perception::Spatial::SpatialLocator>;
}
#[cfg(all(feature = "Foundation", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicDisplay {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicDisplay";
}
#[cfg(all(feature = "Foundation", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicDisplayVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicDisplayImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicDisplayVtbl {
        unsafe extern "system" fn DisplayName<Impl: IHolographicDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxViewportSize<Impl: IHolographicDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxViewportSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStereo<Impl: IHolographicDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpaque<Impl: IHolographicDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpaque() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdapterId<Impl: IHolographicDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialLocator<Impl: IHolographicDisplayImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpatialLocator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicDisplay>, ::windows::core::GetTrustLevel, DisplayName::<Impl, IMPL_OFFSET>, MaxViewportSize::<Impl, IMPL_OFFSET>, IsStereo::<Impl, IMPL_OFFSET>, IsOpaque::<Impl, IMPL_OFFSET>, AdapterId::<Impl, IMPL_OFFSET>, SpatialLocator::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicDisplay as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplay2Impl: Sized {
    fn RefreshRate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicDisplay2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicDisplay2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicDisplay2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicDisplay2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicDisplay2Vtbl {
        unsafe extern "system" fn RefreshRate<Impl: IHolographicDisplay2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicDisplay2>, ::windows::core::GetTrustLevel, RefreshRate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicDisplay2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplay3Impl: Sized {
    fn TryGetViewConfiguration(&self, kind: HolographicViewConfigurationKind) -> ::windows::core::Result<HolographicViewConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicDisplay3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicDisplay3";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicDisplay3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicDisplay3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicDisplay3Vtbl {
        unsafe extern "system" fn TryGetViewConfiguration<Impl: IHolographicDisplay3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: HolographicViewConfigurationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetViewConfiguration(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicDisplay3>, ::windows::core::GetTrustLevel, TryGetViewConfiguration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicDisplay3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplayStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<HolographicDisplay>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicDisplayStatics {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicDisplayStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicDisplayStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicDisplayStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicDisplayStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IHolographicDisplayStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicDisplayStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicDisplayStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHolographicFrameImpl: Sized {
    fn AddedCameras(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>>;
    fn RemovedCameras(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>>;
    fn GetRenderingParameters(&self, camerapose: &::core::option::Option<HolographicCameraPose>) -> ::windows::core::Result<HolographicCameraRenderingParameters>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn CurrentPrediction(&self) -> ::windows::core::Result<HolographicFramePrediction>;
    fn UpdateCurrentPrediction(&self) -> ::windows::core::Result<()>;
    fn PresentUsingCurrentPrediction(&self) -> ::windows::core::Result<HolographicFramePresentResult>;
    fn PresentUsingCurrentPredictionWithBehavior(&self, waitbehavior: HolographicFramePresentWaitBehavior) -> ::windows::core::Result<HolographicFramePresentResult>;
    fn WaitForFrameToFinish(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFrame {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrame";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHolographicFrameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFrameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFrameVtbl {
        unsafe extern "system" fn AddedCameras<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddedCameras() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovedCameras<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemovedCameras() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderingParameters<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, camerapose: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRenderingParameters(&*(&camerapose as *const <HolographicCameraPose as ::windows::core::Abi>::Abi as *const <HolographicCameraPose as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPrediction<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPrediction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateCurrentPrediction<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateCurrentPrediction().into()
        }
        unsafe extern "system" fn PresentUsingCurrentPrediction<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicFramePresentResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentUsingCurrentPrediction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentUsingCurrentPredictionWithBehavior<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, waitbehavior: HolographicFramePresentWaitBehavior, result__: *mut HolographicFramePresentResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentUsingCurrentPredictionWithBehavior(waitbehavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForFrameToFinish<Impl: IHolographicFrameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForFrameToFinish().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicFrame>,
            ::windows::core::GetTrustLevel,
            AddedCameras::<Impl, IMPL_OFFSET>,
            RemovedCameras::<Impl, IMPL_OFFSET>,
            GetRenderingParameters::<Impl, IMPL_OFFSET>,
            Duration::<Impl, IMPL_OFFSET>,
            CurrentPrediction::<Impl, IMPL_OFFSET>,
            UpdateCurrentPrediction::<Impl, IMPL_OFFSET>,
            PresentUsingCurrentPrediction::<Impl, IMPL_OFFSET>,
            PresentUsingCurrentPredictionWithBehavior::<Impl, IMPL_OFFSET>,
            WaitForFrameToFinish::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFrame as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHolographicFrame2Impl: Sized + IHolographicFrameImpl {
    fn GetQuadLayerUpdateParameters(&self, layer: &::core::option::Option<HolographicQuadLayer>) -> ::windows::core::Result<HolographicQuadLayerUpdateParameters>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFrame2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrame2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHolographicFrame2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFrame2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFrame2Vtbl {
        unsafe extern "system" fn GetQuadLayerUpdateParameters<Impl: IHolographicFrame2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuadLayerUpdateParameters(&*(&layer as *const <HolographicQuadLayer as ::windows::core::Abi>::Abi as *const <HolographicQuadLayer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicFrame2>, ::windows::core::GetTrustLevel, GetQuadLayerUpdateParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFrame2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrame3Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<HolographicFrameId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicFrame3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrame3";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicFrame3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFrame3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFrame3Vtbl {
        unsafe extern "system" fn Id<Impl: IHolographicFrame3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicFrame3>, ::windows::core::GetTrustLevel, Id::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFrame3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Perception", feature = "implement_exclusive"))]
pub trait IHolographicFramePredictionImpl: Sized {
    fn CameraPoses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Perception::PerceptionTimestamp>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Perception", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFramePrediction {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFramePrediction";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Perception", feature = "implement_exclusive"))]
impl IHolographicFramePredictionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFramePredictionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFramePredictionVtbl {
        unsafe extern "system" fn CameraPoses<Impl: IHolographicFramePredictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraPoses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IHolographicFramePredictionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicFramePrediction>, ::windows::core::GetTrustLevel, CameraPoses::<Impl, IMPL_OFFSET>, Timestamp::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFramePrediction as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicFramePresentationMonitorImpl: Sized + IClosableImpl {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFramePresentationMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFramePresentationMonitor";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicFramePresentationMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFramePresentationMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFramePresentationMonitorVtbl {
        unsafe extern "system" fn ReadReports<Impl: IHolographicFramePresentationMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicFramePresentationMonitor>, ::windows::core::GetTrustLevel, ReadReports::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFramePresentationMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicFramePresentationReportImpl: Sized {
    fn CompositorGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AppGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AppGpuOverrun(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MissedPresentationOpportunityCount(&self) -> ::windows::core::Result<u32>;
    fn PresentationCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFramePresentationReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFramePresentationReport";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicFramePresentationReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFramePresentationReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFramePresentationReportVtbl {
        unsafe extern "system" fn CompositorGpuDuration<Impl: IHolographicFramePresentationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositorGpuDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppGpuDuration<Impl: IHolographicFramePresentationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppGpuDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppGpuOverrun<Impl: IHolographicFramePresentationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppGpuOverrun() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MissedPresentationOpportunityCount<Impl: IHolographicFramePresentationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MissedPresentationOpportunityCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationCount<Impl: IHolographicFramePresentationReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicFramePresentationReport>,
            ::windows::core::GetTrustLevel,
            CompositorGpuDuration::<Impl, IMPL_OFFSET>,
            AppGpuDuration::<Impl, IMPL_OFFSET>,
            AppGpuOverrun::<Impl, IMPL_OFFSET>,
            MissedPresentationOpportunityCount::<Impl, IMPL_OFFSET>,
            PresentationCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFramePresentationReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicFrameRenderingReportImpl: Sized {
    fn FrameId(&self) -> ::windows::core::Result<HolographicFrameId>;
    fn MissedLatchCount(&self) -> ::windows::core::Result<u32>;
    fn SystemRelativeFrameReadyTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeActualGpuFinishTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeTargetLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFrameRenderingReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrameRenderingReport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicFrameRenderingReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFrameRenderingReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFrameRenderingReportVtbl {
        unsafe extern "system" fn FrameId<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MissedLatchCount<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MissedLatchCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeFrameReadyTime<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeFrameReadyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeActualGpuFinishTime<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeActualGpuFinishTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeTargetLatchTime<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTargetLatchTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicFrameRenderingReport>,
            ::windows::core::GetTrustLevel,
            FrameId::<Impl, IMPL_OFFSET>,
            MissedLatchCount::<Impl, IMPL_OFFSET>,
            SystemRelativeFrameReadyTime::<Impl, IMPL_OFFSET>,
            SystemRelativeActualGpuFinishTime::<Impl, IMPL_OFFSET>,
            SystemRelativeTargetLatchTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFrameRenderingReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHolographicFrameScanoutMonitorImpl: Sized + IClosableImpl {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFrameScanoutMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHolographicFrameScanoutMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFrameScanoutMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFrameScanoutMonitorVtbl {
        unsafe extern "system" fn ReadReports<Impl: IHolographicFrameScanoutMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicFrameScanoutMonitor>, ::windows::core::GetTrustLevel, ReadReports::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFrameScanoutMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicFrameScanoutReportImpl: Sized {
    fn RenderingReport(&self) -> ::windows::core::Result<HolographicFrameRenderingReport>;
    fn MissedScanoutCount(&self) -> ::windows::core::Result<u32>;
    fn SystemRelativeLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeScanoutStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativePhotonTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFrameScanoutReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrameScanoutReport";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicFrameScanoutReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicFrameScanoutReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicFrameScanoutReportVtbl {
        unsafe extern "system" fn RenderingReport<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderingReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MissedScanoutCount<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MissedScanoutCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeLatchTime<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeLatchTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeScanoutStartTime<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativeScanoutStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativePhotonTime<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemRelativePhotonTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicFrameScanoutReport>,
            ::windows::core::GetTrustLevel,
            RenderingReport::<Impl, IMPL_OFFSET>,
            MissedScanoutCount::<Impl, IMPL_OFFSET>,
            SystemRelativeLatchTime::<Impl, IMPL_OFFSET>,
            SystemRelativeScanoutStartTime::<Impl, IMPL_OFFSET>,
            SystemRelativePhotonTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicFrameScanoutReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IHolographicQuadLayerImpl: Sized {
    fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicQuadLayer {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayer";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IHolographicQuadLayerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicQuadLayerVtbl {
        unsafe extern "system" fn PixelFormat<Impl: IHolographicQuadLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IHolographicQuadLayerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayer>, ::windows::core::GetTrustLevel, PixelFormat::<Impl, IMPL_OFFSET>, Size::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IHolographicQuadLayerFactoryImpl: Sized {
    fn Create(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<HolographicQuadLayer>;
    fn CreateWithPixelFormat(&self, size: &super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<HolographicQuadLayer>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicQuadLayerFactory {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayerFactory";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IHolographicQuadLayerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicQuadLayerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHolographicQuadLayerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPixelFormat<Impl: IHolographicQuadLayerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPixelFormat(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), pixelformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>, CreateWithPixelFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
pub trait IHolographicQuadLayerUpdateParametersImpl: Sized {
    fn AcquireBufferToUpdateContent(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn UpdateViewport(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn UpdateContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateExtents(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn UpdateLocationWithStationaryMode(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn UpdateLocationWithDisplayRelativeMode(&self, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Graphics_DirectX_Direct3D11", feature = "Perception_Spatial", feature = "implement_exclusive"))]
impl IHolographicQuadLayerUpdateParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerUpdateParametersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicQuadLayerUpdateParametersVtbl {
        unsafe extern "system" fn AcquireBufferToUpdateContent<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireBufferToUpdateContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateViewport<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateViewport(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateContentProtectionEnabled<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateContentProtectionEnabled(value).into()
        }
        unsafe extern "system" fn UpdateExtents<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateExtents(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateLocationWithStationaryMode<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .UpdateLocationWithStationaryMode(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn UpdateLocationWithDisplayRelativeMode<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateLocationWithDisplayRelativeMode(&*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerUpdateParameters>,
            ::windows::core::GetTrustLevel,
            AcquireBufferToUpdateContent::<Impl, IMPL_OFFSET>,
            UpdateViewport::<Impl, IMPL_OFFSET>,
            UpdateContentProtectionEnabled::<Impl, IMPL_OFFSET>,
            UpdateExtents::<Impl, IMPL_OFFSET>,
            UpdateLocationWithStationaryMode::<Impl, IMPL_OFFSET>,
            UpdateLocationWithDisplayRelativeMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerUpdateParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IHolographicQuadLayerUpdateParameters2Impl: Sized {
    fn CanAcquireWithHardwareProtection(&self) -> ::windows::core::Result<bool>;
    fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParameters2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2";
}
#[cfg(all(feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IHolographicQuadLayerUpdateParameters2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicQuadLayerUpdateParameters2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicQuadLayerUpdateParameters2Vtbl {
        unsafe extern "system" fn CanAcquireWithHardwareProtection<Impl: IHolographicQuadLayerUpdateParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanAcquireWithHardwareProtection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireBufferToUpdateContentWithHardwareProtection<Impl: IHolographicQuadLayerUpdateParameters2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireBufferToUpdateContentWithHardwareProtection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerUpdateParameters2>, ::windows::core::GetTrustLevel, CanAcquireWithHardwareProtection::<Impl, IMPL_OFFSET>, AcquireBufferToUpdateContentWithHardwareProtection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicQuadLayerUpdateParameters2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
pub trait IHolographicSpaceImpl: Sized {
    fn PrimaryAdapterId(&self) -> ::windows::core::Result<HolographicAdapterId>;
    fn SetDirect3D11Device(&self, value: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CameraAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraAdded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraRemoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateNextFrame(&self) -> ::windows::core::Result<HolographicFrame>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicSpace {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpace";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11", feature = "implement_exclusive"))]
impl IHolographicSpaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceVtbl {
        unsafe extern "system" fn PrimaryAdapterId<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrimaryAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirect3D11Device<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirect3D11Device(&*(&value as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraAdded<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraAdded<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraAdded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraRemoved<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CameraRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraRemoved<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCameraRemoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateNextFrame<Impl: IHolographicSpaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNextFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicSpace>,
            ::windows::core::GetTrustLevel,
            PrimaryAdapterId::<Impl, IMPL_OFFSET>,
            SetDirect3D11Device::<Impl, IMPL_OFFSET>,
            CameraAdded::<Impl, IMPL_OFFSET>,
            RemoveCameraAdded::<Impl, IMPL_OFFSET>,
            CameraRemoved::<Impl, IMPL_OFFSET>,
            RemoveCameraRemoved::<Impl, IMPL_OFFSET>,
            CreateNextFrame::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpace as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicSpace2Impl: Sized {
    fn UserPresence(&self) -> ::windows::core::Result<HolographicSpaceUserPresence>;
    fn UserPresenceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserPresenceChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WaitForNextFrameReady(&self) -> ::windows::core::Result<()>;
    fn WaitForNextFrameReadyWithHeadStart(&self, requestedheadstartduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFramePresentationMonitor>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicSpace2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpace2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicSpace2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpace2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpace2Vtbl {
        unsafe extern "system" fn UserPresence<Impl: IHolographicSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicSpaceUserPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserPresence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPresenceChanged<Impl: IHolographicSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserPresenceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserPresenceChanged<Impl: IHolographicSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUserPresenceChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WaitForNextFrameReady<Impl: IHolographicSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForNextFrameReady().into()
        }
        unsafe extern "system" fn WaitForNextFrameReadyWithHeadStart<Impl: IHolographicSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForNextFrameReadyWithHeadStart(&*(&requestedheadstartduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateFramePresentationMonitor<Impl: IHolographicSpace2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFramePresentationMonitor(maxqueuedreports) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicSpace2>,
            ::windows::core::GetTrustLevel,
            UserPresence::<Impl, IMPL_OFFSET>,
            UserPresenceChanged::<Impl, IMPL_OFFSET>,
            RemoveUserPresenceChanged::<Impl, IMPL_OFFSET>,
            WaitForNextFrameReady::<Impl, IMPL_OFFSET>,
            WaitForNextFrameReadyWithHeadStart::<Impl, IMPL_OFFSET>,
            CreateFramePresentationMonitor::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpace2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpace3Impl: Sized {
    fn CreateFrameScanoutMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFrameScanoutMonitor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpace3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpace3";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpace3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpace3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpace3Vtbl {
        unsafe extern "system" fn CreateFrameScanoutMonitor<Impl: IHolographicSpace3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFrameScanoutMonitor(maxqueuedreports) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicSpace3>, ::windows::core::GetTrustLevel, CreateFrameScanoutMonitor::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpace3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicSpaceCameraAddedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<HolographicCamera>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicSpaceCameraAddedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicSpaceCameraAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceCameraAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceCameraAddedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IHolographicSpaceCameraAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IHolographicSpaceCameraAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicSpaceCameraAddedEventArgs>, ::windows::core::GetTrustLevel, Camera::<Impl, IMPL_OFFSET>, GetDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceCameraAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceCameraRemovedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<HolographicCamera>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpaceCameraRemovedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceCameraRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpaceCameraRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceCameraRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceCameraRemovedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IHolographicSpaceCameraRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicSpaceCameraRemovedEventArgs>, ::windows::core::GetTrustLevel, Camera::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceCameraRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IHolographicSpaceStaticsImpl: Sized {
    fn CreateForCoreWindow(&self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<HolographicSpace>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicSpaceStatics {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceStatics";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl IHolographicSpaceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceStaticsVtbl {
        unsafe extern "system" fn CreateForCoreWindow<Impl: IHolographicSpaceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForCoreWindow(&*(&window as *const <super::super::UI::Core::CoreWindow as ::windows::core::Abi>::Abi as *const <super::super::UI::Core::CoreWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicSpaceStatics>, ::windows::core::GetTrustLevel, CreateForCoreWindow::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicSpaceStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicSpaceStatics2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicSpaceStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceStatics2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IHolographicSpaceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Impl: IHolographicSpaceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailableChanged<Impl: IHolographicSpaceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailableChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsAvailableChanged<Impl: IHolographicSpaceStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicSpaceStatics2>, ::windows::core::GetTrustLevel, IsSupported::<Impl, IMPL_OFFSET>, IsAvailable::<Impl, IMPL_OFFSET>, IsAvailableChanged::<Impl, IMPL_OFFSET>, RemoveIsAvailableChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceStatics3Impl: Sized {
    fn IsConfigured(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpaceStatics3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpaceStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceStatics3Vtbl {
        unsafe extern "system" fn IsConfigured<Impl: IHolographicSpaceStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConfigured() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicSpaceStatics3>, ::windows::core::GetTrustLevel, IsConfigured::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
pub trait IHolographicViewConfigurationImpl: Sized {
    fn NativeRenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RequestRenderTargetSize(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat>;
    fn SetPixelFormat(&self, value: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn RefreshRate(&self) -> ::windows::core::Result<f64>;
    fn Kind(&self) -> ::windows::core::Result<HolographicViewConfigurationKind>;
    fn Display(&self) -> ::windows::core::Result<HolographicDisplay>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicViewConfiguration {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicViewConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Graphics_DirectX", feature = "implement_exclusive"))]
impl IHolographicViewConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicViewConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicViewConfigurationVtbl {
        unsafe extern "system" fn NativeRenderTargetSize<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NativeRenderTargetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTargetSize<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTargetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRenderTargetSize<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestRenderTargetSize(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPixelFormats<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelFormat<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPixelFormat(value).into()
        }
        unsafe extern "system" fn IsStereo<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshRate<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicViewConfigurationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IHolographicViewConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IHolographicViewConfiguration>,
            ::windows::core::GetTrustLevel,
            NativeRenderTargetSize::<Impl, IMPL_OFFSET>,
            RenderTargetSize::<Impl, IMPL_OFFSET>,
            RequestRenderTargetSize::<Impl, IMPL_OFFSET>,
            SupportedPixelFormats::<Impl, IMPL_OFFSET>,
            PixelFormat::<Impl, IMPL_OFFSET>,
            SetPixelFormat::<Impl, IMPL_OFFSET>,
            IsStereo::<Impl, IMPL_OFFSET>,
            RefreshRate::<Impl, IMPL_OFFSET>,
            Kind::<Impl, IMPL_OFFSET>,
            Display::<Impl, IMPL_OFFSET>,
            IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicViewConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IHolographicViewConfiguration2Impl: Sized {
    fn SupportedDepthReprojectionMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicViewConfiguration2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicViewConfiguration2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IHolographicViewConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicViewConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicViewConfiguration2Vtbl {
        unsafe extern "system" fn SupportedDepthReprojectionMethods<Impl: IHolographicViewConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedDepthReprojectionMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHolographicViewConfiguration2>, ::windows::core::GetTrustLevel, SupportedDepthReprojectionMethods::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicViewConfiguration2 as ::windows::core::Interface>::IID
    }
}
