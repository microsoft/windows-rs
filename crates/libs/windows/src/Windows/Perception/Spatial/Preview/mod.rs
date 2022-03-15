#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropFrameOfReferencePreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8271b23_735f_5729_a98e_e64ed189abc5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropFrameOfReferencePreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CoordinateSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub NodeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CoordinateSystemToNodeTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CoordinateSystemToNodeTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialGraphInteropPreviewStatics {
    type Vtable = ISpatialGraphInteropPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc042644c_20d8_4ed0_aef7_6805b8e53f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateCoordinateSystemForNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoordinateSystemForNodeWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoordinateSystemForNodeWithPositionAndOrientation: usize,
    pub CreateLocatorForNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodeid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISpatialGraphInteropPreviewStatics2 {
    type Vtable = ISpatialGraphInteropPreviewStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2490b15f_6cbd_4b1e_b765_31e462a32df2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialGraphInteropPreviewStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TryCreateFrameOfReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TryCreateFrameOfReferenceWithPositionAndOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinatesystem: ::windows::core::RawPtr, relativeposition: super::super::super::Foundation::Numerics::Vector3, relativeorientation: super::super::super::Foundation::Numerics::Quaternion, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TryCreateFrameOfReferenceWithPositionAndOrientation: usize,
}
#[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
#[repr(transparent)]
pub struct SpatialGraphInteropFrameOfReferencePreview(::windows::core::IUnknown);
impl SpatialGraphInteropFrameOfReferencePreview {
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn CoordinateSystem(&self) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoordinateSystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn NodeId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NodeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CoordinateSystemToNodeTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix4x4 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CoordinateSystemToNodeTransform)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
}
impl ::core::clone::Clone for SpatialGraphInteropFrameOfReferencePreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SpatialGraphInteropFrameOfReferencePreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpatialGraphInteropFrameOfReferencePreview {}
impl ::core::fmt::Debug for SpatialGraphInteropFrameOfReferencePreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpatialGraphInteropFrameOfReferencePreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SpatialGraphInteropFrameOfReferencePreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview;{a8271b23-735f-5729-a98e-e64ed189abc5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SpatialGraphInteropFrameOfReferencePreview {
    type Vtable = ISpatialGraphInteropFrameOfReferencePreview_Vtbl;
    const IID: ::windows::core::GUID = <ISpatialGraphInteropFrameOfReferencePreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SpatialGraphInteropFrameOfReferencePreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropFrameOfReferencePreview";
}
impl ::core::convert::From<SpatialGraphInteropFrameOfReferencePreview> for ::windows::core::IUnknown {
    fn from(value: SpatialGraphInteropFrameOfReferencePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialGraphInteropFrameOfReferencePreview> for ::windows::core::IUnknown {
    fn from(value: &SpatialGraphInteropFrameOfReferencePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SpatialGraphInteropFrameOfReferencePreview> for ::windows::core::IInspectable {
    fn from(value: SpatialGraphInteropFrameOfReferencePreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SpatialGraphInteropFrameOfReferencePreview> for ::windows::core::IInspectable {
    fn from(value: &SpatialGraphInteropFrameOfReferencePreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SpatialGraphInteropFrameOfReferencePreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SpatialGraphInteropFrameOfReferencePreview {}
unsafe impl ::core::marker::Sync for SpatialGraphInteropFrameOfReferencePreview {}
#[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
pub struct SpatialGraphInteropPreview {}
impl SpatialGraphInteropPreview {
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn CreateCoordinateSystemForNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(nodeid: Param0) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCoordinateSystemForNode)(::core::mem::transmute_copy(this), nodeid.into_param().abi(), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPosition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(nodeid: Param0, relativeposition: Param1) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCoordinateSystemForNodeWithPosition)(::core::mem::transmute_copy(this), nodeid.into_param().abi(), relativeposition.into_param().abi(), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoordinateSystemForNodeWithPositionAndOrientation<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(nodeid: Param0, relativeposition: Param1, relativeorientation: Param2) -> ::windows::core::Result<super::SpatialCoordinateSystem> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCoordinateSystemForNodeWithPositionAndOrientation)(::core::mem::transmute_copy(this), nodeid.into_param().abi(), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<super::SpatialCoordinateSystem>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn CreateLocatorForNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(nodeid: Param0) -> ::windows::core::Result<super::SpatialLocator> {
        Self::ISpatialGraphInteropPreviewStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateLocatorForNode)(::core::mem::transmute_copy(this), nodeid.into_param().abi(), &mut result__).from_abi::<super::SpatialLocator>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`*"]
    pub fn TryCreateFrameOfReference<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialCoordinateSystem>>(coordinatesystem: Param0) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateFrameOfReference)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPosition<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>>(coordinatesystem: Param0, relativeposition: Param1) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateFrameOfReferenceWithPosition)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), relativeposition.into_param().abi(), &mut result__).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[doc = "*Required features: `\"Perception_Spatial_Preview\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TryCreateFrameOfReferenceWithPositionAndOrientation<'a, Param0: ::windows::core::IntoParam<'a, super::SpatialCoordinateSystem>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Vector3>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Quaternion>>(coordinatesystem: Param0, relativeposition: Param1, relativeorientation: Param2) -> ::windows::core::Result<SpatialGraphInteropFrameOfReferencePreview> {
        Self::ISpatialGraphInteropPreviewStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateFrameOfReferenceWithPositionAndOrientation)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), relativeposition.into_param().abi(), relativeorientation.into_param().abi(), &mut result__).from_abi::<SpatialGraphInteropFrameOfReferencePreview>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ISpatialGraphInteropPreviewStatics2<R, F: FnOnce(&ISpatialGraphInteropPreviewStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SpatialGraphInteropPreview, ISpatialGraphInteropPreviewStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for SpatialGraphInteropPreview {
    const NAME: &'static str = "Windows.Perception.Spatial.Preview.SpatialGraphInteropPreview";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
