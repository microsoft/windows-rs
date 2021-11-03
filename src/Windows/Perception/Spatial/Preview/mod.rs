#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISpatialGraphInteropFrameOfReferencePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2821135139, 29535, 22313, [169, 142, 230, 78, 209, 137, 171, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropFrameOfReferencePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGraphInteropPreviewStatics {
    type Vtable = ISpatialGraphInteropPreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3225576524, 8408, 20176, [174, 247, 104, 5, 184, 229, 63, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodeid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodeid: ::windows::runtime::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodeid: ::windows::runtime::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodeid: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISpatialGraphInteropPreviewStatics2 {
    type Vtable = ISpatialGraphInteropPreviewStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(613462367, 27837, 19230, [183, 101, 49, 228, 98, 163, 45, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc = "*Required features: `Perception_Spatial_Preview`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct SpatialGraphInteropFrameOfReferencePreview(::windows::runtime::IInspectable);
impl SpatialGraphInteropFrameOfReferencePreview {
    #[doc = "*Required features: `Perception_Spatial_Preview`*"]
    pub fn CoordinateSystem(&self) -> ::windows::runtime::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `Perception_Spatial_Preview`*"]
    pub fn NodeId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_Spatial_Preview`, `Foundation_Numerics`*"]
    pub fn CoordinateSystemToNodeTransform(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix4x4 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SpatialGraphInteropFrameOfReferencePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview;{a8271b23-735f-5729-a98e-e64ed189abc5})");
}
unsafe impl ::windows::runtime::Interface for SpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2821135139, 29535, 22313, [169, 142, 230, 78, 209, 137, 171, 197]);
}
impl ::windows::runtime::RuntimeName for SpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview";
}
unsafe impl ::std::marker::Send for SpatialGraphInteropFrameOfReferencePreview {}
unsafe impl ::std::marker::Sync for SpatialGraphInteropFrameOfReferencePreview {}
#[doc = "*Required features: `Perception_Spatial_Preview`*"]
pub struct SpatialGraphInteropPreview {}
impl SpatialGraphInteropPreview {
    #[doc = "*Required features: `Perception_Spatial_Preview`*"]
    pub fn CreateCoordinateSystemForNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(nodeid: Param0) -> ::windows::runtime::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), nodeid.into_param().abi(), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_Spatial_Preview`, `Foundation_Numerics`*"]
    pub fn CreateCoordinateSystemForNodeWithPosition<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(nodeid: Param0, relativeposition: Param1) -> ::windows::runtime::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), nodeid.into_param().abi(), relativeposition.into_param().abi(), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_Spatial_Preview`, `Foundation_Numerics`*"]
    pub fn CreateCoordinateSystemForNodeWithPositionAndOrientation<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(nodeid: Param0, relativeposition: Param1, relativeorientation: Param2) -> ::windows::runtime::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), nodeid.into_param().abi(), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `Perception_Spatial_Preview`*"]
    pub fn CreateLocatorForNode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(nodeid: Param0) -> ::windows::runtime::Result<super::SpatialLocator> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), nodeid.into_param().abi(), &mut result__).from_abi::<super::SpatialLocator>(result__)
        })
    }
    #[doc = "*Required features: `Perception_Spatial_Preview`*"]
    pub fn TryCreateFrameOfReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::SpatialCoordinateSystem>>(coordinatesystem: Param0) -> ::windows::runtime::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_Spatial_Preview`, `Foundation_Numerics`*"]
    pub fn TryCreateFrameOfReferenceWithPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(coordinatesystem: Param0, relativeposition: Param1) -> ::windows::runtime::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), relativeposition.into_param().abi(), &mut result__).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Perception_Spatial_Preview`, `Foundation_Numerics`*"]
    pub fn TryCreateFrameOfReferenceWithPositionAndOrientation<'a, Param0: ::windows::runtime::IntoParam<'a, super::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(coordinatesystem: Param0, relativeposition: Param1, relativeorientation: Param2) -> ::windows::runtime::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), coordinatesystem.into_param().abi(), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    pub fn ISpatialGraphInteropPreviewStatics<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISpatialGraphInteropPreviewStatics2<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SpatialGraphInteropPreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropPreview";
}
