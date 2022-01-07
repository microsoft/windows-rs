#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGraphInteropFrameOfReferencePreviewImpl: Sized {
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn NodeId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CoordinateSystemToNodeTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.ISpatialGraphInteropFrameOfReferencePreview";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialGraphInteropFrameOfReferencePreviewVtbl {
    pub const fn new<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialGraphInteropFrameOfReferencePreviewVtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeId<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NodeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystemToNodeTransform<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoordinateSystemToNodeTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialGraphInteropFrameOfReferencePreview>, base.5, CoordinateSystem::<Impl, OFFSET>, NodeId::<Impl, OFFSET>, CoordinateSystemToNodeTransform::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGraphInteropPreviewStaticsImpl: Sized {
    fn CreateCoordinateSystemForNode(&self, nodeid: &::windows::core::GUID) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateCoordinateSystemForNodeWithPosition(&self, nodeid: &::windows::core::GUID, relativeposition: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateCoordinateSystemForNodeWithPositionAndOrientation(&self, nodeid: &::windows::core::GUID, relativeposition: &super::super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateLocatorForNode(&self, nodeid: &::windows::core::GUID) -> ::windows::core::Result<super::SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialGraphInteropPreviewStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.ISpatialGraphInteropPreviewStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialGraphInteropPreviewStaticsVtbl {
    pub const fn new<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialGraphInteropPreviewStaticsVtbl {
        unsafe extern "system" fn CreateCoordinateSystemForNode<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCoordinateSystemForNode(&*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoordinateSystemForNodeWithPosition<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCoordinateSystemForNodeWithPosition(&*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&relativeposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoordinateSystemForNodeWithPositionAndOrientation<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCoordinateSystemForNodeWithPositionAndOrientation(
                &*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&relativeposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                &*(&relativeorientation as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocatorForNode<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateLocatorForNode(&*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialGraphInteropPreviewStatics>, base.5, CreateCoordinateSystemForNode::<Impl, OFFSET>, CreateCoordinateSystemForNodeWithPosition::<Impl, OFFSET>, CreateCoordinateSystemForNodeWithPositionAndOrientation::<Impl, OFFSET>, CreateLocatorForNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpatialGraphInteropPreviewStatics2Impl: Sized {
    fn TryCreateFrameOfReference(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
    fn TryCreateFrameOfReferenceWithPosition(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>, relativeposition: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
    fn TryCreateFrameOfReferenceWithPositionAndOrientation(&self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>, relativeposition: &super::super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpatialGraphInteropPreviewStatics2 {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.ISpatialGraphInteropPreviewStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpatialGraphInteropPreviewStatics2Vtbl {
    pub const fn new<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialGraphInteropPreviewStatics2Vtbl {
        unsafe extern "system" fn TryCreateFrameOfReference<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateFrameOfReference(&*(&coordinatesystem as *const <super::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFrameOfReferenceWithPosition<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateFrameOfReferenceWithPosition(&*(&coordinatesystem as *const <super::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&relativeposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFrameOfReferenceWithPositionAndOrientation<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCreateFrameOfReferenceWithPositionAndOrientation(
                &*(&coordinatesystem as *const <super::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType),
                &*(&relativeposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType),
                &*(&relativeorientation as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Quaternion as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialGraphInteropPreviewStatics2>, base.5, TryCreateFrameOfReference::<Impl, OFFSET>, TryCreateFrameOfReferenceWithPosition::<Impl, OFFSET>, TryCreateFrameOfReferenceWithPositionAndOrientation::<Impl, OFFSET>)
    }
}
