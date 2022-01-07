#[cfg(feature = "implement_exclusive")]
pub trait ICameraIntrinsicsImpl: Sized {
    fn FocalLength(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn PrincipalPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn RadialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn TangentialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn ImageWidth(&self) -> ::windows::core::Result<u32>;
    fn ImageHeight(&self) -> ::windows::core::Result<u32>;
    fn ProjectOntoFrame(&self, coordinate: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn UnprojectAtUnitDepth(&self, pixelcoordinate: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn ProjectManyOntoFrame(&self, coordinates: &[<super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UnprojectPixelsAtUnitDepth(&self, pixelcoordinates: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraIntrinsics {
    const NAME: &'static str = "Windows.Media.Devices.Core.ICameraIntrinsics";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraIntrinsicsVtbl {
    pub const fn new<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraIntrinsicsVtbl {
        unsafe extern "system" fn FocalLength<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocalLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrincipalPoint<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrincipalPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RadialDistortion<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RadialDistortion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TangentialDistortion<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TangentialDistortion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageWidth<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageHeight<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ImageHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectOntoFrame<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinate: super::super::super::Foundation::Numerics::Vector3, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectOntoFrame(&*(&coordinate as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprojectAtUnitDepth<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelcoordinate: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnprojectAtUnitDepth(&*(&pixelcoordinate as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectManyOntoFrame<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinates_array_size: u32, coordinates: *const super::super::super::Foundation::Numerics::Vector3, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProjectManyOntoFrame(::core::slice::from_raw_parts(::core::mem::transmute_copy(&coordinates), coordinates_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)).into()
        }
        unsafe extern "system" fn UnprojectPixelsAtUnitDepth<Impl: ICameraIntrinsicsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelCoordinates_array_size: u32, pixelcoordinates: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnprojectPixelsAtUnitDepth(::core::slice::from_raw_parts(::core::mem::transmute_copy(&pixelcoordinates), pixelCoordinates_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraIntrinsics>, base.5, FocalLength::<Impl, OFFSET>, PrincipalPoint::<Impl, OFFSET>, RadialDistortion::<Impl, OFFSET>, TangentialDistortion::<Impl, OFFSET>, ImageWidth::<Impl, OFFSET>, ImageHeight::<Impl, OFFSET>, ProjectOntoFrame::<Impl, OFFSET>, UnprojectAtUnitDepth::<Impl, OFFSET>, ProjectManyOntoFrame::<Impl, OFFSET>, UnprojectPixelsAtUnitDepth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraIntrinsics2Impl: Sized {
    fn UndistortedProjectionTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4>;
    fn DistortPoint(&self, input: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn DistortPoints(&self, inputs: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UndistortPoint(&self, input: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn UndistortPoints(&self, inputs: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraIntrinsics2 {
    const NAME: &'static str = "Windows.Media.Devices.Core.ICameraIntrinsics2";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraIntrinsics2Vtbl {
    pub const fn new<Impl: ICameraIntrinsics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraIntrinsics2Vtbl {
        unsafe extern "system" fn UndistortedProjectionTransform<Impl: ICameraIntrinsics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UndistortedProjectionTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistortPoint<Impl: ICameraIntrinsics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DistortPoint(&*(&input as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DistortPoints<Impl: ICameraIntrinsics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DistortPoints(::core::slice::from_raw_parts(::core::mem::transmute_copy(&inputs), inputs_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)).into()
        }
        unsafe extern "system" fn UndistortPoint<Impl: ICameraIntrinsics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UndistortPoint(&*(&input as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndistortPoints<Impl: ICameraIntrinsics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UndistortPoints(::core::slice::from_raw_parts(::core::mem::transmute_copy(&inputs), inputs_array_size as _), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraIntrinsics2>, base.5, UndistortedProjectionTransform::<Impl, OFFSET>, DistortPoint::<Impl, OFFSET>, DistortPoints::<Impl, OFFSET>, UndistortPoint::<Impl, OFFSET>, UndistortPoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraIntrinsicsFactoryImpl: Sized {
    fn Create(&self, focallength: &super::super::super::Foundation::Numerics::Vector2, principalpoint: &super::super::super::Foundation::Numerics::Vector2, radialdistortion: &super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: &super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32) -> ::windows::core::Result<CameraIntrinsics>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICameraIntrinsicsFactory {
    const NAME: &'static str = "Windows.Media.Devices.Core.ICameraIntrinsicsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICameraIntrinsicsFactoryVtbl {
    pub const fn new<Impl: ICameraIntrinsicsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICameraIntrinsicsFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICameraIntrinsicsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(
                &*(&focallength as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                &*(&principalpoint as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                &*(&radialdistortion as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                &*(&tangentialdistortion as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType),
                imagewidth,
                imageheight,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICameraIntrinsicsFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDepthCorrelatedCoordinateMapperImpl: Sized + IClosableImpl {
    fn UnprojectPoint(&self, sourcepoint: &super::super::super::Foundation::Point, targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn UnprojectPoints(&self, sourcepoints: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, results: &mut [<super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn MapPoint(&self, sourcepoint: &super::super::super::Foundation::Point, targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, targetcameraintrinsics: &::core::option::Option<CameraIntrinsics>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn MapPoints(&self, sourcepoints: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, targetcameraintrinsics: &::core::option::Option<CameraIntrinsics>, results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Media.Devices.Core.IDepthCorrelatedCoordinateMapper";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDepthCorrelatedCoordinateMapperVtbl {
    pub const fn new<Impl: IDepthCorrelatedCoordinateMapperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDepthCorrelatedCoordinateMapperVtbl {
        unsafe extern "system" fn UnprojectPoint<Impl: IDepthCorrelatedCoordinateMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnprojectPoint(&*(&sourcepoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&targetcoordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnprojectPoints<Impl: IDepthCorrelatedCoordinateMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .UnprojectPoints(::core::slice::from_raw_parts(::core::mem::transmute_copy(&sourcepoints), sourcePoints_array_size as _), &*(&targetcoordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _))
                .into()
        }
        unsafe extern "system" fn MapPoint<Impl: IDepthCorrelatedCoordinateMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, targetcameraintrinsics: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MapPoint(
                &*(&sourcepoint as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType),
                &*(&targetcoordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                &*(&targetcameraintrinsics as *const <CameraIntrinsics as ::windows::core::Abi>::Abi as *const <CameraIntrinsics as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapPoints<Impl: IDepthCorrelatedCoordinateMapperImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: ::windows::core::RawPtr, targetcameraintrinsics: ::windows::core::RawPtr, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .MapPoints(
                    ::core::slice::from_raw_parts(::core::mem::transmute_copy(&sourcepoints), sourcePoints_array_size as _),
                    &*(&targetcoordinatesystem as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::super::super::Perception::Spatial::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                    &*(&targetcameraintrinsics as *const <CameraIntrinsics as ::windows::core::Abi>::Abi as *const <CameraIntrinsics as ::windows::core::DefaultType>::DefaultType),
                    ::core::slice::from_raw_parts_mut(::core::mem::transmute_copy(&results), results_array_size as _),
                )
                .into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDepthCorrelatedCoordinateMapper>, base.5, UnprojectPoint::<Impl, OFFSET>, UnprojectPoints::<Impl, OFFSET>, MapPoint::<Impl, OFFSET>, MapPoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameControlCapabilitiesImpl: Sized {
    fn Exposure(&self) -> ::windows::core::Result<FrameExposureCapabilities>;
    fn ExposureCompensation(&self) -> ::windows::core::Result<FrameExposureCompensationCapabilities>;
    fn IsoSpeed(&self) -> ::windows::core::Result<FrameIsoSpeedCapabilities>;
    fn Focus(&self) -> ::windows::core::Result<FrameFocusCapabilities>;
    fn PhotoConfirmationSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameControlCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameControlCapabilitiesVtbl {
    pub const fn new<Impl: IFrameControlCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameControlCapabilitiesVtbl {
        unsafe extern "system" fn Exposure<Impl: IFrameControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exposure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureCompensation<Impl: IFrameControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposureCompensation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoSpeed<Impl: IFrameControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsoSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Focus<Impl: IFrameControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Focus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoConfirmationSupported<Impl: IFrameControlCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoConfirmationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameControlCapabilities>, base.5, Exposure::<Impl, OFFSET>, ExposureCompensation::<Impl, OFFSET>, IsoSpeed::<Impl, OFFSET>, Focus::<Impl, OFFSET>, PhotoConfirmationSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameControlCapabilities2Impl: Sized {
    fn Flash(&self) -> ::windows::core::Result<FrameFlashCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameControlCapabilities2 {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameControlCapabilities2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameControlCapabilities2Vtbl {
    pub const fn new<Impl: IFrameControlCapabilities2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameControlCapabilities2Vtbl {
        unsafe extern "system" fn Flash<Impl: IFrameControlCapabilities2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameControlCapabilities2>, base.5, Flash::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameControllerImpl: Sized {
    fn ExposureControl(&self) -> ::windows::core::Result<FrameExposureControl>;
    fn ExposureCompensationControl(&self) -> ::windows::core::Result<FrameExposureCompensationControl>;
    fn IsoSpeedControl(&self) -> ::windows::core::Result<FrameIsoSpeedControl>;
    fn FocusControl(&self) -> ::windows::core::Result<FrameFocusControl>;
    fn PhotoConfirmationEnabled(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>>;
    fn SetPhotoConfirmationEnabled(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameController {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameController";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameControllerVtbl {
    pub const fn new<Impl: IFrameControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameControllerVtbl {
        unsafe extern "system" fn ExposureControl<Impl: IFrameControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposureControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExposureCompensationControl<Impl: IFrameControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExposureCompensationControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsoSpeedControl<Impl: IFrameControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsoSpeedControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusControl<Impl: IFrameControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotoConfirmationEnabled<Impl: IFrameControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotoConfirmationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotoConfirmationEnabled<Impl: IFrameControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPhotoConfirmationEnabled(&*(&value as *const <super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameController>, base.5, ExposureControl::<Impl, OFFSET>, ExposureCompensationControl::<Impl, OFFSET>, IsoSpeedControl::<Impl, OFFSET>, FocusControl::<Impl, OFFSET>, PhotoConfirmationEnabled::<Impl, OFFSET>, SetPhotoConfirmationEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameController2Impl: Sized {
    fn FlashControl(&self) -> ::windows::core::Result<FrameFlashControl>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameController2 {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameController2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameController2Vtbl {
    pub const fn new<Impl: IFrameController2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameController2Vtbl {
        unsafe extern "system" fn FlashControl<Impl: IFrameController2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlashControl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameController2>, base.5, FlashControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Max(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Step(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameExposureCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameExposureCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameExposureCapabilitiesVtbl {
    pub const fn new<Impl: IFrameExposureCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameExposureCapabilitiesVtbl {
        unsafe extern "system" fn Supported<Impl: IFrameExposureCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IFrameExposureCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IFrameExposureCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IFrameExposureCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameExposureCapabilities>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureCompensationCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f32>;
    fn Max(&self) -> ::windows::core::Result<f32>;
    fn Step(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameExposureCompensationCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameExposureCompensationCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameExposureCompensationCapabilitiesVtbl {
    pub const fn new<Impl: IFrameExposureCompensationCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameExposureCompensationCapabilitiesVtbl {
        unsafe extern "system" fn Supported<Impl: IFrameExposureCompensationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IFrameExposureCompensationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IFrameExposureCompensationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IFrameExposureCompensationCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameExposureCompensationCapabilities>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureCompensationControlImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameExposureCompensationControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameExposureCompensationControlVtbl {
    pub const fn new<Impl: IFrameExposureCompensationControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameExposureCompensationControlVtbl {
        unsafe extern "system" fn Value<Impl: IFrameExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFrameExposureCompensationControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<f32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameExposureCompensationControl>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureControlImpl: Sized {
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameExposureControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameExposureControlVtbl {
    pub const fn new<Impl: IFrameExposureControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameExposureControlVtbl {
        unsafe extern "system" fn Auto<Impl: IFrameExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuto<Impl: IFrameExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuto(value).into()
        }
        unsafe extern "system" fn Value<Impl: IFrameExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFrameExposureControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameExposureControl>, base.5, Auto::<Impl, OFFSET>, SetAuto::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFlashCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameFlashCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameFlashCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameFlashCapabilitiesVtbl {
    pub const fn new<Impl: IFrameFlashCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameFlashCapabilitiesVtbl {
        unsafe extern "system" fn Supported<Impl: IFrameFlashCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RedEyeReductionSupported<Impl: IFrameFlashCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedEyeReductionSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowerSupported<Impl: IFrameFlashCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameFlashCapabilities>, base.5, Supported::<Impl, OFFSET>, RedEyeReductionSupported::<Impl, OFFSET>, PowerSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFlashControlImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<FrameFlashMode>;
    fn SetMode(&self, value: FrameFlashMode) -> ::windows::core::Result<()>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn RedEyeReduction(&self) -> ::windows::core::Result<bool>;
    fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameFlashControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameFlashControlVtbl {
    pub const fn new<Impl: IFrameFlashControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameFlashControlVtbl {
        unsafe extern "system" fn Mode<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FrameFlashMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FrameFlashMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Auto<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuto<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuto(value).into()
        }
        unsafe extern "system" fn RedEyeReduction<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RedEyeReduction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRedEyeReduction<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRedEyeReduction(value).into()
        }
        unsafe extern "system" fn PowerPercent<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PowerPercent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPowerPercent<Impl: IFrameFlashControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPowerPercent(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameFlashControl>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>, Auto::<Impl, OFFSET>, SetAuto::<Impl, OFFSET>, RedEyeReduction::<Impl, OFFSET>, SetRedEyeReduction::<Impl, OFFSET>, PowerPercent::<Impl, OFFSET>, SetPowerPercent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFocusCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameFocusCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameFocusCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameFocusCapabilitiesVtbl {
    pub const fn new<Impl: IFrameFocusCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameFocusCapabilitiesVtbl {
        unsafe extern "system" fn Supported<Impl: IFrameFocusCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IFrameFocusCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IFrameFocusCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IFrameFocusCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameFocusCapabilities>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFocusControlImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameFocusControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameFocusControlVtbl {
    pub const fn new<Impl: IFrameFocusControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameFocusControlVtbl {
        unsafe extern "system" fn Value<Impl: IFrameFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFrameFocusControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameFocusControl>, base.5, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameIsoSpeedCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameIsoSpeedCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameIsoSpeedCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameIsoSpeedCapabilitiesVtbl {
    pub const fn new<Impl: IFrameIsoSpeedCapabilitiesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameIsoSpeedCapabilitiesVtbl {
        unsafe extern "system" fn Supported<Impl: IFrameIsoSpeedCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Min<Impl: IFrameIsoSpeedCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Min() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Max<Impl: IFrameIsoSpeedCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Max() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Step<Impl: IFrameIsoSpeedCapabilitiesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Step() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameIsoSpeedCapabilities>, base.5, Supported::<Impl, OFFSET>, Min::<Impl, OFFSET>, Max::<Impl, OFFSET>, Step::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameIsoSpeedControlImpl: Sized {
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.IFrameIsoSpeedControl";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameIsoSpeedControlVtbl {
    pub const fn new<Impl: IFrameIsoSpeedControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameIsoSpeedControlVtbl {
        unsafe extern "system" fn Auto<Impl: IFrameIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuto<Impl: IFrameIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAuto(value).into()
        }
        unsafe extern "system" fn Value<Impl: IFrameIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IFrameIsoSpeedControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::IReference<u32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameIsoSpeedControl>, base.5, Auto::<Impl, OFFSET>, SetAuto::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVariablePhotoSequenceControllerImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32>;
    fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32>;
    fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetHighestConcurrentFrameRate(&self, captureproperties: &::core::option::Option<super::super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn FrameCapabilities(&self) -> ::windows::core::Result<FrameControlCapabilities>;
    fn DesiredFrameControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<FrameController>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVariablePhotoSequenceController {
    const NAME: &'static str = "Windows.Media.Devices.Core.IVariablePhotoSequenceController";
}
#[cfg(feature = "implement_exclusive")]
impl IVariablePhotoSequenceControllerVtbl {
    pub const fn new<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVariablePhotoSequenceControllerVtbl {
        unsafe extern "system" fn Supported<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Supported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPhotosPerSecond<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxPhotosPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhotosPerSecondLimit<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhotosPerSecondLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhotosPerSecondLimit<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPhotosPerSecondLimit(value).into()
        }
        unsafe extern "system" fn GetHighestConcurrentFrameRate<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, captureproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHighestConcurrentFrameRate(&*(&captureproperties as *const <super::super::MediaProperties::IMediaEncodingProperties as ::windows::core::Abi>::Abi as *const <super::super::MediaProperties::IMediaEncodingProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentFrameRate<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentFrameRate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FrameCapabilities<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FrameCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredFrameControllers<Impl: IVariablePhotoSequenceControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredFrameControllers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVariablePhotoSequenceController>, base.5, Supported::<Impl, OFFSET>, MaxPhotosPerSecond::<Impl, OFFSET>, PhotosPerSecondLimit::<Impl, OFFSET>, SetPhotosPerSecondLimit::<Impl, OFFSET>, GetHighestConcurrentFrameRate::<Impl, OFFSET>, GetCurrentFrameRate::<Impl, OFFSET>, FrameCapabilities::<Impl, OFFSET>, DesiredFrameControllers::<Impl, OFFSET>)
    }
}
