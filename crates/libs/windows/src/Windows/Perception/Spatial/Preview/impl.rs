#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialGraphInteropFrameOfReferencePreviewImpl: Sized {
    fn CoordinateSystem(&mut self) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn NodeId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CoordinateSystemToNodeTransform(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.ISpatialGraphInteropFrameOfReferencePreview";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialGraphInteropFrameOfReferencePreviewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialGraphInteropFrameOfReferencePreviewVtbl {
        unsafe extern "system" fn CoordinateSystem<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeId<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NodeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoordinateSystemToNodeTransform<Impl: ISpatialGraphInteropFrameOfReferencePreviewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoordinateSystemToNodeTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialGraphInteropFrameOfReferencePreview, BASE_OFFSET>(),
            CoordinateSystem: CoordinateSystem::<Impl, IMPL_OFFSET>,
            NodeId: NodeId::<Impl, IMPL_OFFSET>,
            CoordinateSystemToNodeTransform: CoordinateSystemToNodeTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialGraphInteropFrameOfReferencePreview as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialGraphInteropPreviewStaticsImpl: Sized {
    fn CreateCoordinateSystemForNode(&mut self, nodeid: &::windows::core::GUID) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateCoordinateSystemForNodeWithPosition(&mut self, nodeid: &::windows::core::GUID, relativeposition: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateCoordinateSystemForNodeWithPositionAndOrientation(&mut self, nodeid: &::windows::core::GUID, relativeposition: &super::super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<super::SpatialCoordinateSystem>;
    fn CreateLocatorForNode(&mut self, nodeid: &::windows::core::GUID) -> ::windows::core::Result<super::SpatialLocator>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialGraphInteropPreviewStatics {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.ISpatialGraphInteropPreviewStatics";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialGraphInteropPreviewStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialGraphInteropPreviewStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialGraphInteropPreviewStaticsVtbl {
        unsafe extern "system" fn CreateCoordinateSystemForNode<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCoordinateSystemForNode(&*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoordinateSystemForNodeWithPosition<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCoordinateSystemForNodeWithPosition(&*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&relativeposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCoordinateSystemForNodeWithPositionAndOrientation<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateLocatorForNode<Impl: ISpatialGraphInteropPreviewStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateLocatorForNode(&*(&nodeid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialGraphInteropPreviewStatics, BASE_OFFSET>(),
            CreateCoordinateSystemForNode: CreateCoordinateSystemForNode::<Impl, IMPL_OFFSET>,
            CreateCoordinateSystemForNodeWithPosition: CreateCoordinateSystemForNodeWithPosition::<Impl, IMPL_OFFSET>,
            CreateCoordinateSystemForNodeWithPositionAndOrientation: CreateCoordinateSystemForNodeWithPositionAndOrientation::<Impl, IMPL_OFFSET>,
            CreateLocatorForNode: CreateLocatorForNode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialGraphInteropPreviewStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
pub trait ISpatialGraphInteropPreviewStatics2Impl: Sized {
    fn TryCreateFrameOfReference(&mut self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
    fn TryCreateFrameOfReferenceWithPosition(&mut self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>, relativeposition: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
    fn TryCreateFrameOfReferenceWithPositionAndOrientation(&mut self, coordinatesystem: &::core::option::Option<super::SpatialCoordinateSystem>, relativeposition: &super::super::super::Foundation::Numerics::Vector3, relativeorientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpatialGraphInteropPreviewStatics2 {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.ISpatialGraphInteropPreviewStatics2";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ISpatialGraphInteropPreviewStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialGraphInteropPreviewStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialGraphInteropPreviewStatics2Vtbl {
        unsafe extern "system" fn TryCreateFrameOfReference<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFrameOfReference(&*(&coordinatesystem as *const <super::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFrameOfReferenceWithPosition<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFrameOfReferenceWithPosition(&*(&coordinatesystem as *const <super::SpatialCoordinateSystem as ::windows::core::Abi>::Abi as *const <super::SpatialCoordinateSystem as ::windows::core::DefaultType>::DefaultType), &*(&relativeposition as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateFrameOfReferenceWithPositionAndOrientation<Impl: ISpatialGraphInteropPreviewStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialGraphInteropPreviewStatics2, BASE_OFFSET>(),
            TryCreateFrameOfReference: TryCreateFrameOfReference::<Impl, IMPL_OFFSET>,
            TryCreateFrameOfReferenceWithPosition: TryCreateFrameOfReferenceWithPosition::<Impl, IMPL_OFFSET>,
            TryCreateFrameOfReferenceWithPositionAndOrientation: TryCreateFrameOfReferenceWithPositionAndOrientation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialGraphInteropPreviewStatics2 as ::windows::core::Interface>::IID
    }
}
