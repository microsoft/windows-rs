#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraImpl: Sized {
    fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ViewportScaleFactor(&self) -> ::windows::core::Result<f64>;
    fn SetViewportScaleFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn SetNearPlaneDistance(&self, value: f64) -> ::windows::core::Result<()>;
    fn SetFarPlaneDistance(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCamera {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraVtbl {
    pub const fn new<Impl: IHolographicCameraImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraVtbl {
        unsafe extern "system" fn RenderTargetSize<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderTargetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportScaleFactor<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportScaleFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportScaleFactor<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewportScaleFactor(value).into()
        }
        unsafe extern "system" fn IsStereo<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNearPlaneDistance<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNearPlaneDistance(value).into()
        }
        unsafe extern "system" fn SetFarPlaneDistance<Impl: IHolographicCameraImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFarPlaneDistance(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCamera>, base.5, RenderTargetSize::<Impl, OFFSET>, ViewportScaleFactor::<Impl, OFFSET>, SetViewportScaleFactor::<Impl, OFFSET>, IsStereo::<Impl, OFFSET>, Id::<Impl, OFFSET>, SetNearPlaneDistance::<Impl, OFFSET>, SetFarPlaneDistance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera2Impl: Sized + IHolographicCameraImpl {
    fn LeftViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters>;
    fn RightViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters>;
    fn Display(&self) -> ::windows::core::Result<HolographicDisplay>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCamera2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCamera2Vtbl {
    pub const fn new<Impl: IHolographicCamera2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCamera2Vtbl {
        unsafe extern "system" fn LeftViewportParameters<Impl: IHolographicCamera2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeftViewportParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RightViewportParameters<Impl: IHolographicCamera2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RightViewportParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: IHolographicCamera2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCamera2>, base.5, LeftViewportParameters::<Impl, OFFSET>, RightViewportParameters::<Impl, OFFSET>, Display::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera3Impl: Sized + IHolographicCameraImpl + IHolographicCamera2Impl {
    fn IsPrimaryLayerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryLayerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxQuadLayerCount(&self) -> ::windows::core::Result<u32>;
    fn QuadLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCamera3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCamera3";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCamera3Vtbl {
    pub const fn new<Impl: IHolographicCamera3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCamera3Vtbl {
        unsafe extern "system" fn IsPrimaryLayerEnabled<Impl: IHolographicCamera3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPrimaryLayerEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPrimaryLayerEnabled<Impl: IHolographicCamera3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsPrimaryLayerEnabled(value).into()
        }
        unsafe extern "system" fn MaxQuadLayerCount<Impl: IHolographicCamera3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxQuadLayerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuadLayers<Impl: IHolographicCamera3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).QuadLayers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCamera3>, base.5, IsPrimaryLayerEnabled::<Impl, OFFSET>, SetIsPrimaryLayerEnabled::<Impl, OFFSET>, MaxQuadLayerCount::<Impl, OFFSET>, QuadLayers::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicCamera4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCamera4Vtbl {
        unsafe extern "system" fn CanOverrideViewport<Impl: IHolographicCamera4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanOverrideViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCamera4>, base.5, CanOverrideViewport::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicCamera5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCamera5Vtbl {
        unsafe extern "system" fn IsHardwareContentProtectionSupported<Impl: IHolographicCamera5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHardwareContentProtectionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHardwareContentProtectionEnabled<Impl: IHolographicCamera5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHardwareContentProtectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHardwareContentProtectionEnabled<Impl: IHolographicCamera5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsHardwareContentProtectionEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCamera5>, base.5, IsHardwareContentProtectionSupported::<Impl, OFFSET>, IsHardwareContentProtectionEnabled::<Impl, OFFSET>, SetIsHardwareContentProtectionEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicCamera6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCamera6Vtbl {
        unsafe extern "system" fn ViewConfiguration<Impl: IHolographicCamera6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCamera6>, base.5, ViewConfiguration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraPose {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraPose";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraPoseVtbl {
    pub const fn new<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraPoseVtbl {
        unsafe extern "system" fn HolographicCamera<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HolographicCamera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Viewport<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Viewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetViewTransform<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetViewTransform(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionTransform<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicStereoTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetCullingFrustum<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetCullingFrustum(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetVisibleFrustum<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetVisibleFrustum(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NearPlaneDistance<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NearPlaneDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FarPlaneDistance<Impl: IHolographicCameraPoseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FarPlaneDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraPose>, base.5, HolographicCamera::<Impl, OFFSET>, Viewport::<Impl, OFFSET>, TryGetViewTransform::<Impl, OFFSET>, ProjectionTransform::<Impl, OFFSET>, TryGetCullingFrustum::<Impl, OFFSET>, TryGetVisibleFrustum::<Impl, OFFSET>, NearPlaneDistance::<Impl, OFFSET>, FarPlaneDistance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraPose2Impl: Sized {
    fn OverrideViewTransform(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, coordinatesystemtoviewtransform: &HolographicStereoTransform) -> ::windows::core::Result<()>;
    fn OverrideProjectionTransform(&self, projectiontransform: &HolographicStereoTransform) -> ::windows::core::Result<()>;
    fn OverrideViewport(&self, leftviewport: &super::super::Foundation::Rect, rightviewport: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraPose2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraPose2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraPose2Vtbl {
    pub const fn new<Impl: IHolographicCameraPose2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraPose2Vtbl {
        unsafe extern "system" fn OverrideViewTransform<Impl: IHolographicCameraPose2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OverrideViewTransform(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&coordinatesystemtoviewtransform as *const <HolographicStereoTransform as ::windows::core::Abi>::Abi as *const <HolographicStereoTransform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideProjectionTransform<Impl: IHolographicCameraPose2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, projectiontransform: HolographicStereoTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OverrideProjectionTransform(&*(&projectiontransform as *const <HolographicStereoTransform as ::windows::core::Abi>::Abi as *const <HolographicStereoTransform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OverrideViewport<Impl: IHolographicCameraPose2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OverrideViewport(&*(&leftviewport as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&rightviewport as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraPose2>, base.5, OverrideViewTransform::<Impl, OFFSET>, OverrideProjectionTransform::<Impl, OFFSET>, OverrideViewport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParametersImpl: Sized {
    fn SetFocusPoint(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetFocusPointWithNormal(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, normal: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetFocusPointWithNormalLinearVelocity(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, normal: &super::super::Foundation::Numerics::Vector3, linearvelocity: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DDevice>;
    fn Direct3D11BackBuffer(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraRenderingParametersVtbl {
    pub const fn new<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraRenderingParametersVtbl {
        unsafe extern "system" fn SetFocusPoint<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusPoint(&*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetFocusPointWithNormal<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetFocusPointWithNormal(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetFocusPointWithNormalLinearVelocity<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .SetFocusPointWithNormalLinearVelocity(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&normal as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&linearvelocity as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Direct3D11Device<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direct3D11Device() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direct3D11BackBuffer<Impl: IHolographicCameraRenderingParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direct3D11BackBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters>, base.5, SetFocusPoint::<Impl, OFFSET>, SetFocusPointWithNormal::<Impl, OFFSET>, SetFocusPointWithNormalLinearVelocity::<Impl, OFFSET>, Direct3D11Device::<Impl, OFFSET>, Direct3D11BackBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParameters2Impl: Sized + IHolographicCameraRenderingParametersImpl {
    fn ReprojectionMode(&self) -> ::windows::core::Result<HolographicReprojectionMode>;
    fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> ::windows::core::Result<()>;
    fn CommitDirect3D11DepthBuffer(&self, value: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraRenderingParameters2Vtbl {
    pub const fn new<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraRenderingParameters2Vtbl {
        unsafe extern "system" fn ReprojectionMode<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicReprojectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReprojectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReprojectionMode<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HolographicReprojectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReprojectionMode(value).into()
        }
        unsafe extern "system" fn CommitDirect3D11DepthBuffer<Impl: IHolographicCameraRenderingParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CommitDirect3D11DepthBuffer(&*(&value as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DSurface as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters2>, base.5, ReprojectionMode::<Impl, OFFSET>, SetReprojectionMode::<Impl, OFFSET>, CommitDirect3D11DepthBuffer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParameters3Impl: Sized + IHolographicCameraRenderingParametersImpl + IHolographicCameraRenderingParameters2Impl {
    fn IsContentProtectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraRenderingParameters3 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraRenderingParameters3";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraRenderingParameters3Vtbl {
    pub const fn new<Impl: IHolographicCameraRenderingParameters3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraRenderingParameters3Vtbl {
        unsafe extern "system" fn IsContentProtectionEnabled<Impl: IHolographicCameraRenderingParameters3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContentProtectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsContentProtectionEnabled<Impl: IHolographicCameraRenderingParameters3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsContentProtectionEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters3>, base.5, IsContentProtectionEnabled::<Impl, OFFSET>, SetIsContentProtectionEnabled::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicCameraRenderingParameters4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraRenderingParameters4Vtbl {
        unsafe extern "system" fn DepthReprojectionMethod<Impl: IHolographicCameraRenderingParameters4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicDepthReprojectionMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DepthReprojectionMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDepthReprojectionMethod<Impl: IHolographicCameraRenderingParameters4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HolographicDepthReprojectionMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDepthReprojectionMethod(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraRenderingParameters4>, base.5, DepthReprojectionMethod::<Impl, OFFSET>, SetDepthReprojectionMethod::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraViewportParametersImpl: Sized {
    fn HiddenAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>>;
    fn VisibleAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicCameraViewportParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicCameraViewportParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicCameraViewportParametersVtbl {
    pub const fn new<Impl: IHolographicCameraViewportParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicCameraViewportParametersVtbl {
        unsafe extern "system" fn HiddenAreaMesh<Impl: IHolographicCameraViewportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn VisibleAreaMesh<Impl: IHolographicCameraViewportParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicCameraViewportParameters>, base.5, HiddenAreaMesh::<Impl, OFFSET>, VisibleAreaMesh::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplayImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxViewportSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn IsOpaque(&self) -> ::windows::core::Result<bool>;
    fn AdapterId(&self) -> ::windows::core::Result<HolographicAdapterId>;
    fn SpatialLocator(&self) -> ::windows::core::Result<super::super::Perception::Spatial::SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicDisplay {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicDisplay";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicDisplayVtbl {
    pub const fn new<Impl: IHolographicDisplayImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicDisplayVtbl {
        unsafe extern "system" fn DisplayName<Impl: IHolographicDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxViewportSize<Impl: IHolographicDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxViewportSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStereo<Impl: IHolographicDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpaque<Impl: IHolographicDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpaque() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdapterId<Impl: IHolographicDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialLocator<Impl: IHolographicDisplayImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpatialLocator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicDisplay>, base.5, DisplayName::<Impl, OFFSET>, MaxViewportSize::<Impl, OFFSET>, IsStereo::<Impl, OFFSET>, IsOpaque::<Impl, OFFSET>, AdapterId::<Impl, OFFSET>, SpatialLocator::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicDisplay2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicDisplay2Vtbl {
        unsafe extern "system" fn RefreshRate<Impl: IHolographicDisplay2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RefreshRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicDisplay2>, base.5, RefreshRate::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicDisplay3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicDisplay3Vtbl {
        unsafe extern "system" fn TryGetViewConfiguration<Impl: IHolographicDisplay3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: HolographicViewConfigurationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetViewConfiguration(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicDisplay3>, base.5, TryGetViewConfiguration::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicDisplayStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicDisplayStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IHolographicDisplayStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicDisplayStatics>, base.5, GetDefault::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicFrame {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrame";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicFrameVtbl {
    pub const fn new<Impl: IHolographicFrameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFrameVtbl {
        unsafe extern "system" fn AddedCameras<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddedCameras() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovedCameras<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemovedCameras() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRenderingParameters<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, camerapose: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRenderingParameters(&*(&camerapose as *const <HolographicCameraPose as ::windows::core::Abi>::Abi as *const <HolographicCameraPose as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Duration<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPrediction<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentPrediction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateCurrentPrediction<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateCurrentPrediction().into()
        }
        unsafe extern "system" fn PresentUsingCurrentPrediction<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicFramePresentResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PresentUsingCurrentPrediction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentUsingCurrentPredictionWithBehavior<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, waitbehavior: HolographicFramePresentWaitBehavior, result__: *mut HolographicFramePresentResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PresentUsingCurrentPredictionWithBehavior(waitbehavior) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForFrameToFinish<Impl: IHolographicFrameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WaitForFrameToFinish().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFrame>, base.5, AddedCameras::<Impl, OFFSET>, RemovedCameras::<Impl, OFFSET>, GetRenderingParameters::<Impl, OFFSET>, Duration::<Impl, OFFSET>, CurrentPrediction::<Impl, OFFSET>, UpdateCurrentPrediction::<Impl, OFFSET>, PresentUsingCurrentPrediction::<Impl, OFFSET>, PresentUsingCurrentPredictionWithBehavior::<Impl, OFFSET>, WaitForFrameToFinish::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrame2Impl: Sized + IHolographicFrameImpl {
    fn GetQuadLayerUpdateParameters(&self, layer: &::core::option::Option<HolographicQuadLayer>) -> ::windows::core::Result<HolographicQuadLayerUpdateParameters>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicFrame2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrame2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicFrame2Vtbl {
    pub const fn new<Impl: IHolographicFrame2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFrame2Vtbl {
        unsafe extern "system" fn GetQuadLayerUpdateParameters<Impl: IHolographicFrame2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, layer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuadLayerUpdateParameters(&*(&layer as *const <HolographicQuadLayer as ::windows::core::Abi>::Abi as *const <HolographicQuadLayer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFrame2>, base.5, GetQuadLayerUpdateParameters::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicFrame3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFrame3Vtbl {
        unsafe extern "system" fn Id<Impl: IHolographicFrame3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFrame3>, base.5, Id::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFramePredictionImpl: Sized {
    fn CameraPoses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Perception::PerceptionTimestamp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicFramePrediction {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFramePrediction";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicFramePredictionVtbl {
    pub const fn new<Impl: IHolographicFramePredictionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFramePredictionVtbl {
        unsafe extern "system" fn CameraPoses<Impl: IHolographicFramePredictionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraPoses() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timestamp<Impl: IHolographicFramePredictionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFramePrediction>, base.5, CameraPoses::<Impl, OFFSET>, Timestamp::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicFramePresentationMonitorImpl: Sized + IClosableImpl {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFramePresentationMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFramePresentationMonitor";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicFramePresentationMonitorVtbl {
    pub const fn new<Impl: IHolographicFramePresentationMonitorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFramePresentationMonitorVtbl {
        unsafe extern "system" fn ReadReports<Impl: IHolographicFramePresentationMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFramePresentationMonitor>, base.5, ReadReports::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicFramePresentationReportImpl: Sized {
    fn CompositorGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AppGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AppGpuOverrun(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MissedPresentationOpportunityCount(&self) -> ::windows::core::Result<u32>;
    fn PresentationCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFramePresentationReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFramePresentationReport";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IHolographicFramePresentationReportVtbl {
    pub const fn new<Impl: IHolographicFramePresentationReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFramePresentationReportVtbl {
        unsafe extern "system" fn CompositorGpuDuration<Impl: IHolographicFramePresentationReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositorGpuDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppGpuDuration<Impl: IHolographicFramePresentationReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppGpuDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppGpuOverrun<Impl: IHolographicFramePresentationReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppGpuOverrun() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MissedPresentationOpportunityCount<Impl: IHolographicFramePresentationReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MissedPresentationOpportunityCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PresentationCount<Impl: IHolographicFramePresentationReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PresentationCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFramePresentationReport>, base.5, CompositorGpuDuration::<Impl, OFFSET>, AppGpuDuration::<Impl, OFFSET>, AppGpuOverrun::<Impl, OFFSET>, MissedPresentationOpportunityCount::<Impl, OFFSET>, PresentationCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrameRenderingReportImpl: Sized {
    fn FrameId(&self) -> ::windows::core::Result<HolographicFrameId>;
    fn MissedLatchCount(&self) -> ::windows::core::Result<u32>;
    fn SystemRelativeFrameReadyTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeActualGpuFinishTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeTargetLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicFrameRenderingReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrameRenderingReport";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicFrameRenderingReportVtbl {
    pub const fn new<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFrameRenderingReportVtbl {
        unsafe extern "system" fn FrameId<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicFrameId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MissedLatchCount<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MissedLatchCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeFrameReadyTime<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeFrameReadyTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeActualGpuFinishTime<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeActualGpuFinishTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeTargetLatchTime<Impl: IHolographicFrameRenderingReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeTargetLatchTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFrameRenderingReport>, base.5, FrameId::<Impl, OFFSET>, MissedLatchCount::<Impl, OFFSET>, SystemRelativeFrameReadyTime::<Impl, OFFSET>, SystemRelativeActualGpuFinishTime::<Impl, OFFSET>, SystemRelativeTargetLatchTime::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicFrameScanoutMonitorImpl: Sized + IClosableImpl {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHolographicFrameScanoutMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrameScanoutMonitor";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHolographicFrameScanoutMonitorVtbl {
    pub const fn new<Impl: IHolographicFrameScanoutMonitorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFrameScanoutMonitorVtbl {
        unsafe extern "system" fn ReadReports<Impl: IHolographicFrameScanoutMonitorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadReports() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFrameScanoutMonitor>, base.5, ReadReports::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrameScanoutReportImpl: Sized {
    fn RenderingReport(&self) -> ::windows::core::Result<HolographicFrameRenderingReport>;
    fn MissedScanoutCount(&self) -> ::windows::core::Result<u32>;
    fn SystemRelativeLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeScanoutStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativePhotonTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicFrameScanoutReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicFrameScanoutReport";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicFrameScanoutReportVtbl {
    pub const fn new<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicFrameScanoutReportVtbl {
        unsafe extern "system" fn RenderingReport<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderingReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MissedScanoutCount<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MissedScanoutCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeLatchTime<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeLatchTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativeScanoutStartTime<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativeScanoutStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemRelativePhotonTime<Impl: IHolographicFrameScanoutReportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemRelativePhotonTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicFrameScanoutReport>, base.5, RenderingReport::<Impl, OFFSET>, MissedScanoutCount::<Impl, OFFSET>, SystemRelativeLatchTime::<Impl, OFFSET>, SystemRelativeScanoutStartTime::<Impl, OFFSET>, SystemRelativePhotonTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerImpl: Sized {
    fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicQuadLayer {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayer";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicQuadLayerVtbl {
    pub const fn new<Impl: IHolographicQuadLayerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicQuadLayerVtbl {
        unsafe extern "system" fn PixelFormat<Impl: IHolographicQuadLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IHolographicQuadLayerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayer>, base.5, PixelFormat::<Impl, OFFSET>, Size::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerFactoryImpl: Sized {
    fn Create(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<HolographicQuadLayer>;
    fn CreateWithPixelFormat(&self, size: &super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<HolographicQuadLayer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicQuadLayerFactory {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicQuadLayerFactoryVtbl {
    pub const fn new<Impl: IHolographicQuadLayerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicQuadLayerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IHolographicQuadLayerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPixelFormat<Impl: IHolographicQuadLayerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithPixelFormat(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), pixelformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerFactory>, base.5, Create::<Impl, OFFSET>, CreateWithPixelFormat::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerUpdateParametersImpl: Sized {
    fn AcquireBufferToUpdateContent(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn UpdateViewport(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn UpdateContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateExtents(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn UpdateLocationWithStationaryMode(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn UpdateLocationWithDisplayRelativeMode(&self, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicQuadLayerUpdateParametersVtbl {
    pub const fn new<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicQuadLayerUpdateParametersVtbl {
        unsafe extern "system" fn AcquireBufferToUpdateContent<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireBufferToUpdateContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateViewport<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateViewport(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateContentProtectionEnabled<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateContentProtectionEnabled(value).into()
        }
        unsafe extern "system" fn UpdateExtents<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateExtents(&*(&value as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateLocationWithStationaryMode<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .UpdateLocationWithStationaryMode(
                    &*(&coordinatesystem as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                    &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn UpdateLocationWithDisplayRelativeMode<Impl: IHolographicQuadLayerUpdateParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateLocationWithDisplayRelativeMode(&*(&position as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType), &*(&orientation as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerUpdateParameters>, base.5, AcquireBufferToUpdateContent::<Impl, OFFSET>, UpdateViewport::<Impl, OFFSET>, UpdateContentProtectionEnabled::<Impl, OFFSET>, UpdateExtents::<Impl, OFFSET>, UpdateLocationWithStationaryMode::<Impl, OFFSET>, UpdateLocationWithDisplayRelativeMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerUpdateParameters2Impl: Sized {
    fn CanAcquireWithHardwareProtection(&self) -> ::windows::core::Result<bool>;
    fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicQuadLayerUpdateParameters2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicQuadLayerUpdateParameters2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicQuadLayerUpdateParameters2Vtbl {
    pub const fn new<Impl: IHolographicQuadLayerUpdateParameters2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicQuadLayerUpdateParameters2Vtbl {
        unsafe extern "system" fn CanAcquireWithHardwareProtection<Impl: IHolographicQuadLayerUpdateParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanAcquireWithHardwareProtection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireBufferToUpdateContentWithHardwareProtection<Impl: IHolographicQuadLayerUpdateParameters2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireBufferToUpdateContentWithHardwareProtection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicQuadLayerUpdateParameters2>, base.5, CanAcquireWithHardwareProtection::<Impl, OFFSET>, AcquireBufferToUpdateContentWithHardwareProtection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceImpl: Sized {
    fn PrimaryAdapterId(&self) -> ::windows::core::Result<HolographicAdapterId>;
    fn SetDirect3D11Device(&self, value: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CameraAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraAdded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraRemoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateNextFrame(&self) -> ::windows::core::Result<HolographicFrame>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpace {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpace";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpaceVtbl {
    pub const fn new<Impl: IHolographicSpaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceVtbl {
        unsafe extern "system" fn PrimaryAdapterId<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicAdapterId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrimaryAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirect3D11Device<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDirect3D11Device(&*(&value as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::Abi>::Abi as *const <super::DirectX::Direct3D11::IDirect3DDevice as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraAdded<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraAdded<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraAdded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CameraRemoved<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CameraRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCameraRemoved<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCameraRemoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateNextFrame<Impl: IHolographicSpaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateNextFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpace>, base.5, PrimaryAdapterId::<Impl, OFFSET>, SetDirect3D11Device::<Impl, OFFSET>, CameraAdded::<Impl, OFFSET>, RemoveCameraAdded::<Impl, OFFSET>, CameraRemoved::<Impl, OFFSET>, RemoveCameraRemoved::<Impl, OFFSET>, CreateNextFrame::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpace2Impl: Sized {
    fn UserPresence(&self) -> ::windows::core::Result<HolographicSpaceUserPresence>;
    fn UserPresenceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserPresenceChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WaitForNextFrameReady(&self) -> ::windows::core::Result<()>;
    fn WaitForNextFrameReadyWithHeadStart(&self, requestedheadstartduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFramePresentationMonitor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpace2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpace2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpace2Vtbl {
    pub const fn new<Impl: IHolographicSpace2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpace2Vtbl {
        unsafe extern "system" fn UserPresence<Impl: IHolographicSpace2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicSpaceUserPresence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserPresence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserPresenceChanged<Impl: IHolographicSpace2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserPresenceChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUserPresenceChanged<Impl: IHolographicSpace2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUserPresenceChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WaitForNextFrameReady<Impl: IHolographicSpace2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WaitForNextFrameReady().into()
        }
        unsafe extern "system" fn WaitForNextFrameReadyWithHeadStart<Impl: IHolographicSpace2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WaitForNextFrameReadyWithHeadStart(&*(&requestedheadstartduration as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateFramePresentationMonitor<Impl: IHolographicSpace2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFramePresentationMonitor(maxqueuedreports) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpace2>, base.5, UserPresence::<Impl, OFFSET>, UserPresenceChanged::<Impl, OFFSET>, RemoveUserPresenceChanged::<Impl, OFFSET>, WaitForNextFrameReady::<Impl, OFFSET>, WaitForNextFrameReadyWithHeadStart::<Impl, OFFSET>, CreateFramePresentationMonitor::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicSpace3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpace3Vtbl {
        unsafe extern "system" fn CreateFrameScanoutMonitor<Impl: IHolographicSpace3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxqueuedreports: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateFrameScanoutMonitor(maxqueuedreports) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpace3>, base.5, CreateFrameScanoutMonitor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceCameraAddedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<HolographicCamera>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpaceCameraAddedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceCameraAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpaceCameraAddedEventArgsVtbl {
    pub const fn new<Impl: IHolographicSpaceCameraAddedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceCameraAddedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IHolographicSpaceCameraAddedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IHolographicSpaceCameraAddedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpaceCameraAddedEventArgs>, base.5, Camera::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicSpaceCameraRemovedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceCameraRemovedEventArgsVtbl {
        unsafe extern "system" fn Camera<Impl: IHolographicSpaceCameraRemovedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Camera() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpaceCameraRemovedEventArgs>, base.5, Camera::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceStaticsImpl: Sized {
    fn CreateForCoreWindow(&self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<HolographicSpace>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpaceStatics {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpaceStaticsVtbl {
    pub const fn new<Impl: IHolographicSpaceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceStaticsVtbl {
        unsafe extern "system" fn CreateForCoreWindow<Impl: IHolographicSpaceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForCoreWindow(&*(&window as *const <super::super::UI::Core::CoreWindow as ::windows::core::Abi>::Abi as *const <super::super::UI::Core::CoreWindow as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpaceStatics>, base.5, CreateForCoreWindow::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicSpaceStatics2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicSpaceStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicSpaceStatics2Vtbl {
    pub const fn new<Impl: IHolographicSpaceStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceStatics2Vtbl {
        unsafe extern "system" fn IsSupported<Impl: IHolographicSpaceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Impl: IHolographicSpaceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailableChanged<Impl: IHolographicSpaceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAvailableChanged(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsAvailableChanged<Impl: IHolographicSpaceStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsAvailableChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpaceStatics2>, base.5, IsSupported::<Impl, OFFSET>, IsAvailable::<Impl, OFFSET>, IsAvailableChanged::<Impl, OFFSET>, RemoveIsAvailableChanged::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHolographicSpaceStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceStatics3Vtbl {
        unsafe extern "system" fn IsConfigured<Impl: IHolographicSpaceStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConfigured() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpaceStatics3>, base.5, IsConfigured::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
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
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicViewConfiguration {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicViewConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicViewConfigurationVtbl {
    pub const fn new<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicViewConfigurationVtbl {
        unsafe extern "system" fn NativeRenderTargetSize<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NativeRenderTargetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTargetSize<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderTargetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestRenderTargetSize<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestRenderTargetSize(&*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPixelFormats<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedPixelFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PixelFormat<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PixelFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPixelFormat<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::DirectX::DirectXPixelFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPixelFormat(value).into()
        }
        unsafe extern "system" fn IsStereo<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsStereo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshRate<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RefreshRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HolographicViewConfigurationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Display<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Display() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IHolographicViewConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IHolographicViewConfiguration>,
            base.5,
            NativeRenderTargetSize::<Impl, OFFSET>,
            RenderTargetSize::<Impl, OFFSET>,
            RequestRenderTargetSize::<Impl, OFFSET>,
            SupportedPixelFormats::<Impl, OFFSET>,
            PixelFormat::<Impl, OFFSET>,
            SetPixelFormat::<Impl, OFFSET>,
            IsStereo::<Impl, OFFSET>,
            RefreshRate::<Impl, OFFSET>,
            Kind::<Impl, OFFSET>,
            Display::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            SetIsEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicViewConfiguration2Impl: Sized {
    fn SupportedDepthReprojectionMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHolographicViewConfiguration2 {
    const NAME: &'static str = "Windows.Graphics.Holographic.IHolographicViewConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IHolographicViewConfiguration2Vtbl {
    pub const fn new<Impl: IHolographicViewConfiguration2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicViewConfiguration2Vtbl {
        unsafe extern "system" fn SupportedDepthReprojectionMethods<Impl: IHolographicViewConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedDepthReprojectionMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicViewConfiguration2>, base.5, SupportedDepthReprojectionMethods::<Impl, OFFSET>)
    }
}
