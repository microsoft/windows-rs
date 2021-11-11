#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_Holographic`*"]
pub struct HolographicAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl HolographicAdapterId {}
impl ::core::default::Default for HolographicAdapterId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HolographicAdapterId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HolographicAdapterId").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::core::cmp::PartialEq for HolographicAdapterId {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for HolographicAdapterId {}
unsafe impl ::windows::runtime::Abi for HolographicAdapterId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicAdapterId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicAdapterId;u4;i4)");
}
impl ::windows::runtime::DefaultType for HolographicAdapterId {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicCamera(pub ::windows::runtime::IInspectable);
impl HolographicCamera {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RenderTargetSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn ViewportScaleFactor(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetViewportScaleFactor(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsStereo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetNearPlaneDistance(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetFarPlaneDistance(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn LeftViewportParameters(&self) -> ::windows::runtime::Result<HolographicCameraViewportParameters> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicCameraViewportParameters>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn RightViewportParameters(&self) -> ::windows::runtime::Result<HolographicCameraViewportParameters> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicCameraViewportParameters>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Display(&self) -> ::windows::runtime::Result<HolographicDisplay> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicDisplay>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsPrimaryLayerEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetIsPrimaryLayerEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn MaxQuadLayerCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn QuadLayers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<HolographicQuadLayer>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn CanOverrideViewport(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsHardwareContentProtectionSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsHardwareContentProtectionEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetIsHardwareContentProtectionEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera5>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn ViewConfiguration(&self) -> ::windows::runtime::Result<HolographicViewConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCamera6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicViewConfiguration>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicCamera {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCamera;{e4e98445-9bed-4980-9ba0-e87680d1cb74})");
}
unsafe impl ::windows::runtime::Interface for HolographicCamera {
    type Vtable = IHolographicCamera_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4e98445_9bed_4980_9ba0_e87680d1cb74);
}
impl ::windows::runtime::RuntimeName for HolographicCamera {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCamera";
}
impl ::core::convert::From<HolographicCamera> for ::windows::runtime::IUnknown {
    fn from(value: HolographicCamera) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicCamera> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicCamera) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicCamera {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicCamera {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicCamera> for ::windows::runtime::IInspectable {
    fn from(value: HolographicCamera) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicCamera> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicCamera) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicCamera {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicCamera {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicCamera {}
unsafe impl ::core::marker::Sync for HolographicCamera {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicCameraPose(pub ::windows::runtime::IInspectable);
impl HolographicCameraPose {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn HolographicCamera(&self) -> ::windows::runtime::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicCamera>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Viewport(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetViewTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<HolographicStereoTransform>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<HolographicStereoTransform>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
    pub fn ProjectionTransform(&self) -> ::windows::runtime::Result<HolographicStereoTransform> {
        let this = self;
        unsafe {
            let mut result__: HolographicStereoTransform = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicStereoTransform>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetCullingFrustum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn TryGetVisibleFrustum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>>(&self, coordinatesystem: Param0) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn NearPlaneDistance(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn FarPlaneDistance(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn OverrideViewTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, HolographicStereoTransform>>(&self, coordinatesystem: Param0, coordinatesystemtoviewtransform: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), coordinatesystemtoviewtransform.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
    pub fn OverrideProjectionTransform<'a, Param0: ::windows::runtime::IntoParam<'a, HolographicStereoTransform>>(&self, projectiontransform: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), projectiontransform.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn OverrideViewport<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, leftviewport: Param0, rightviewport: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraPose2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), leftviewport.into_param().abi(), rightviewport.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicCameraPose {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraPose;{0d7d7e30-12de-45bd-912b-c7f6561599d1})");
}
unsafe impl ::windows::runtime::Interface for HolographicCameraPose {
    type Vtable = IHolographicCameraPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0d7d7e30_12de_45bd_912b_c7f6561599d1);
}
impl ::windows::runtime::RuntimeName for HolographicCameraPose {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraPose";
}
impl ::core::convert::From<HolographicCameraPose> for ::windows::runtime::IUnknown {
    fn from(value: HolographicCameraPose) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicCameraPose> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicCameraPose) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicCameraPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicCameraPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicCameraPose> for ::windows::runtime::IInspectable {
    fn from(value: HolographicCameraPose) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicCameraPose> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicCameraPose) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicCameraPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicCameraPose {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicCameraPose {}
unsafe impl ::core::marker::Sync for HolographicCameraPose {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicCameraRenderingParameters(pub ::windows::runtime::IInspectable);
impl HolographicCameraRenderingParameters {
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn SetFocusPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, position: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn SetFocusPointWithNormal<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(&self, coordinatesystem: Param0, position: Param1, normal: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), normal.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn SetFocusPointWithNormalLinearVelocity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>>(
        &self,
        coordinatesystem: Param0,
        position: Param1,
        normal: Param2,
        linearvelocity: Param3,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), normal.into_param().abi(), linearvelocity.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Direct3D11Device(&self) -> ::windows::runtime::Result<super::DirectX::Direct3D11::IDirect3DDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::Direct3D11::IDirect3DDevice>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX_Direct3D11`*"]
    pub fn Direct3D11BackBuffer(&self) -> ::windows::runtime::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn ReprojectionMode(&self) -> ::windows::runtime::Result<HolographicReprojectionMode> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe {
            let mut result__: HolographicReprojectionMode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicReprojectionMode>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX_Direct3D11`*"]
    pub fn CommitDirect3D11DepthBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsContentProtectionEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetIsContentProtectionEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn DepthReprojectionMethod(&self) -> ::windows::runtime::Result<HolographicDepthReprojectionMethod> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe {
            let mut result__: HolographicDepthReprojectionMethod = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicDepthReprojectionMethod>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetDepthReprojectionMethod(&self, value: HolographicDepthReprojectionMethod) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicCameraRenderingParameters4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicCameraRenderingParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraRenderingParameters;{8eac2ed1-5bf4-4e16-8236-ae0800c11d0d})");
}
unsafe impl ::windows::runtime::Interface for HolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8eac2ed1_5bf4_4e16_8236_ae0800c11d0d);
}
impl ::windows::runtime::RuntimeName for HolographicCameraRenderingParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraRenderingParameters";
}
impl ::core::convert::From<HolographicCameraRenderingParameters> for ::windows::runtime::IUnknown {
    fn from(value: HolographicCameraRenderingParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicCameraRenderingParameters> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicCameraRenderingParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicCameraRenderingParameters> for ::windows::runtime::IInspectable {
    fn from(value: HolographicCameraRenderingParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicCameraRenderingParameters> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicCameraRenderingParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicCameraRenderingParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicCameraRenderingParameters {}
unsafe impl ::core::marker::Sync for HolographicCameraRenderingParameters {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicCameraViewportParameters(pub ::windows::runtime::IInspectable);
impl HolographicCameraViewportParameters {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
    pub fn HiddenAreaMesh(&self) -> ::windows::runtime::Result<::windows::runtime::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<super::super::Foundation::Numerics::Vector2> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::runtime::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
    pub fn VisibleAreaMesh(&self) -> ::windows::runtime::Result<::windows::runtime::Array<super::super::Foundation::Numerics::Vector2>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<super::super::Foundation::Numerics::Vector2> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), ::windows::runtime::Array::<super::super::Foundation::Numerics::Vector2>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicCameraViewportParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicCameraViewportParameters;{80cdf3f7-842a-41e1-93ed-5692ab1fbb10})");
}
unsafe impl ::windows::runtime::Interface for HolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80cdf3f7_842a_41e1_93ed_5692ab1fbb10);
}
impl ::windows::runtime::RuntimeName for HolographicCameraViewportParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicCameraViewportParameters";
}
impl ::core::convert::From<HolographicCameraViewportParameters> for ::windows::runtime::IUnknown {
    fn from(value: HolographicCameraViewportParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicCameraViewportParameters> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicCameraViewportParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicCameraViewportParameters> for ::windows::runtime::IInspectable {
    fn from(value: HolographicCameraViewportParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicCameraViewportParameters> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicCameraViewportParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicCameraViewportParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicCameraViewportParameters {}
unsafe impl ::core::marker::Sync for HolographicCameraViewportParameters {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: HolographicDepthReprojectionMethod = HolographicDepthReprojectionMethod(0i32);
    pub const AutoPlanar: HolographicDepthReprojectionMethod = HolographicDepthReprojectionMethod(1i32);
}
impl ::core::convert::From<i32> for HolographicDepthReprojectionMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HolographicDepthReprojectionMethod {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicDepthReprojectionMethod {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicDepthReprojectionMethod;i4)");
}
impl ::windows::runtime::DefaultType for HolographicDepthReprojectionMethod {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicDisplay(pub ::windows::runtime::IInspectable);
impl HolographicDisplay {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn MaxViewportSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsStereo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsOpaque(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn AdapterId(&self) -> ::windows::runtime::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__: HolographicAdapterId = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicAdapterId>(result__)
        }
    }
    #[cfg(feature = "Perception_Spatial")]
    #[doc = "*Required features: `Graphics_Holographic`, `Perception_Spatial`*"]
    pub fn SpatialLocator(&self) -> ::windows::runtime::Result<super::super::Perception::Spatial::SpatialLocator> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Perception::Spatial::SpatialLocator>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<HolographicDisplay> {
        Self::IHolographicDisplayStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicDisplay>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn RefreshRate(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IHolographicDisplay2>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn TryGetViewConfiguration(&self, kind: HolographicViewConfigurationKind) -> ::windows::runtime::Result<HolographicViewConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IHolographicDisplay3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<HolographicViewConfiguration>(result__)
        }
    }
    pub fn IHolographicDisplayStatics<R, F: FnOnce(&IHolographicDisplayStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HolographicDisplay, IHolographicDisplayStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicDisplay {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicDisplay;{9acea414-1d9f-4090-a388-90c06f6eae9c})");
}
unsafe impl ::windows::runtime::Interface for HolographicDisplay {
    type Vtable = IHolographicDisplay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9acea414_1d9f_4090_a388_90c06f6eae9c);
}
impl ::windows::runtime::RuntimeName for HolographicDisplay {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicDisplay";
}
impl ::core::convert::From<HolographicDisplay> for ::windows::runtime::IUnknown {
    fn from(value: HolographicDisplay) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicDisplay> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicDisplay) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicDisplay> for ::windows::runtime::IInspectable {
    fn from(value: HolographicDisplay) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicDisplay> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicDisplay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicDisplay {}
unsafe impl ::core::marker::Sync for HolographicDisplay {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFrame(pub ::windows::runtime::IInspectable);
impl HolographicFrame {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn AddedCameras(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCamera>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn RemovedCameras(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCamera>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn GetRenderingParameters<'a, Param0: ::windows::runtime::IntoParam<'a, HolographicCameraPose>>(&self, camerapose: Param0) -> ::windows::runtime::Result<HolographicCameraRenderingParameters> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), camerapose.into_param().abi(), &mut result__).from_abi::<HolographicCameraRenderingParameters>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn CurrentPrediction(&self) -> ::windows::runtime::Result<HolographicFramePrediction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicFramePrediction>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn UpdateCurrentPrediction(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn PresentUsingCurrentPrediction(&self) -> ::windows::runtime::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__: HolographicFramePresentResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicFramePresentResult>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn PresentUsingCurrentPredictionWithBehavior(&self, waitbehavior: HolographicFramePresentWaitBehavior) -> ::windows::runtime::Result<HolographicFramePresentResult> {
        let this = self;
        unsafe {
            let mut result__: HolographicFramePresentResult = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), waitbehavior, &mut result__).from_abi::<HolographicFramePresentResult>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn WaitForFrameToFinish(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn GetQuadLayerUpdateParameters<'a, Param0: ::windows::runtime::IntoParam<'a, HolographicQuadLayer>>(&self, layer: Param0) -> ::windows::runtime::Result<HolographicQuadLayerUpdateParameters> {
        let this = &::windows::runtime::Interface::cast::<IHolographicFrame2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), layer.into_param().abi(), &mut result__).from_abi::<HolographicQuadLayerUpdateParameters>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<HolographicFrameId> {
        let this = &::windows::runtime::Interface::cast::<IHolographicFrame3>(self)?;
        unsafe {
            let mut result__: HolographicFrameId = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicFrameId>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrame;{c6988eb6-a8b9-3054-a6eb-d624b6536375})");
}
unsafe impl ::windows::runtime::Interface for HolographicFrame {
    type Vtable = IHolographicFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc6988eb6_a8b9_3054_a6eb_d624b6536375);
}
impl ::windows::runtime::RuntimeName for HolographicFrame {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrame";
}
impl ::core::convert::From<HolographicFrame> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFrame> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFrame> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFrame> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicFrame {}
unsafe impl ::core::marker::Sync for HolographicFrame {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_Holographic`*"]
pub struct HolographicFrameId {
    pub Value: u64,
}
impl HolographicFrameId {}
impl ::core::default::Default for HolographicFrameId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HolographicFrameId {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HolographicFrameId").field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for HolographicFrameId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for HolographicFrameId {}
unsafe impl ::windows::runtime::Abi for HolographicFrameId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFrameId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicFrameId;u8)");
}
impl ::windows::runtime::DefaultType for HolographicFrameId {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFramePrediction(pub ::windows::runtime::IInspectable);
impl HolographicFramePrediction {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn CameraPoses(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>>(result__)
        }
    }
    #[cfg(feature = "Perception")]
    #[doc = "*Required features: `Graphics_Holographic`, `Perception`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Perception::PerceptionTimestamp> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Perception::PerceptionTimestamp>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFramePrediction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePrediction;{520f4de1-5c0a-4e79-a81e-6abe02bb2739})");
}
unsafe impl ::windows::runtime::Interface for HolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x520f4de1_5c0a_4e79_a81e_6abe02bb2739);
}
impl ::windows::runtime::RuntimeName for HolographicFramePrediction {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePrediction";
}
impl ::core::convert::From<HolographicFramePrediction> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFramePrediction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFramePrediction> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFramePrediction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFramePrediction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFramePrediction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFramePrediction> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFramePrediction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFramePrediction> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFramePrediction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFramePrediction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFramePrediction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicFramePrediction {}
unsafe impl ::core::marker::Sync for HolographicFramePrediction {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: HolographicFramePresentResult = HolographicFramePresentResult(0i32);
    pub const DeviceRemoved: HolographicFramePresentResult = HolographicFramePresentResult(1i32);
}
impl ::core::convert::From<i32> for HolographicFramePresentResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HolographicFramePresentResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFramePresentResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentResult;i4)");
}
impl ::windows::runtime::DefaultType for HolographicFramePresentResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: HolographicFramePresentWaitBehavior = HolographicFramePresentWaitBehavior(0i32);
    pub const DoNotWaitForFrameToFinish: HolographicFramePresentWaitBehavior = HolographicFramePresentWaitBehavior(1i32);
}
impl ::core::convert::From<i32> for HolographicFramePresentWaitBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HolographicFramePresentWaitBehavior {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFramePresentWaitBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicFramePresentWaitBehavior;i4)");
}
impl ::windows::runtime::DefaultType for HolographicFramePresentWaitBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFramePresentationMonitor(pub ::windows::runtime::IInspectable);
impl HolographicFramePresentationMonitor {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn ReadReports(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFramePresentationMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePresentationMonitor;{ca87256c-6fae-428e-bb83-25dfee51136b})");
}
unsafe impl ::windows::runtime::Interface for HolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca87256c_6fae_428e_bb83_25dfee51136b);
}
impl ::windows::runtime::RuntimeName for HolographicFramePresentationMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationMonitor";
}
impl ::core::convert::From<HolographicFramePresentationMonitor> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFramePresentationMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFramePresentationMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFramePresentationMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFramePresentationMonitor> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFramePresentationMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFramePresentationMonitor> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFramePresentationMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicFramePresentationMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HolographicFramePresentationMonitor) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicFramePresentationMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HolographicFramePresentationMonitor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &HolographicFramePresentationMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for HolographicFramePresentationMonitor {}
unsafe impl ::core::marker::Sync for HolographicFramePresentationMonitor {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFramePresentationReport(pub ::windows::runtime::IInspectable);
impl HolographicFramePresentationReport {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn CompositorGpuDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn AppGpuDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn AppGpuOverrun(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn MissedPresentationOpportunityCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn PresentationCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFramePresentationReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFramePresentationReport;{80baf614-f2f4-4c8a-8de3-065c78f6d5de})");
}
unsafe impl ::windows::runtime::Interface for HolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80baf614_f2f4_4c8a_8de3_065c78f6d5de);
}
impl ::windows::runtime::RuntimeName for HolographicFramePresentationReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFramePresentationReport";
}
impl ::core::convert::From<HolographicFramePresentationReport> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFramePresentationReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFramePresentationReport> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFramePresentationReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFramePresentationReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFramePresentationReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFramePresentationReport> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFramePresentationReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFramePresentationReport> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFramePresentationReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFramePresentationReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFramePresentationReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicFramePresentationReport {}
unsafe impl ::core::marker::Sync for HolographicFramePresentationReport {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFrameRenderingReport(pub ::windows::runtime::IInspectable);
impl HolographicFrameRenderingReport {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn FrameId(&self) -> ::windows::runtime::Result<HolographicFrameId> {
        let this = self;
        unsafe {
            let mut result__: HolographicFrameId = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicFrameId>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn MissedLatchCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn SystemRelativeFrameReadyTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn SystemRelativeActualGpuFinishTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn SystemRelativeTargetLatchTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFrameRenderingReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameRenderingReport;{05f32de4-e384-51b3-b934-f0d3a0f78606})");
}
unsafe impl ::windows::runtime::Interface for HolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05f32de4_e384_51b3_b934_f0d3a0f78606);
}
impl ::windows::runtime::RuntimeName for HolographicFrameRenderingReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameRenderingReport";
}
impl ::core::convert::From<HolographicFrameRenderingReport> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFrameRenderingReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFrameRenderingReport> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFrameRenderingReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFrameRenderingReport> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFrameRenderingReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFrameRenderingReport> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFrameRenderingReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFrameRenderingReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicFrameRenderingReport {}
unsafe impl ::core::marker::Sync for HolographicFrameRenderingReport {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFrameScanoutMonitor(pub ::windows::runtime::IInspectable);
impl HolographicFrameScanoutMonitor {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn ReadReports(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFrameScanoutMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameScanoutMonitor;{7e83efa9-843c-5401-8095-9bc1b8b08638})");
}
unsafe impl ::windows::runtime::Interface for HolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7e83efa9_843c_5401_8095_9bc1b8b08638);
}
impl ::windows::runtime::RuntimeName for HolographicFrameScanoutMonitor {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutMonitor";
}
impl ::core::convert::From<HolographicFrameScanoutMonitor> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFrameScanoutMonitor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFrameScanoutMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFrameScanoutMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFrameScanoutMonitor> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFrameScanoutMonitor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFrameScanoutMonitor> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFrameScanoutMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicFrameScanoutMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HolographicFrameScanoutMonitor) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicFrameScanoutMonitor> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HolographicFrameScanoutMonitor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &HolographicFrameScanoutMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for HolographicFrameScanoutMonitor {}
unsafe impl ::core::marker::Sync for HolographicFrameScanoutMonitor {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicFrameScanoutReport(pub ::windows::runtime::IInspectable);
impl HolographicFrameScanoutReport {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn RenderingReport(&self) -> ::windows::runtime::Result<HolographicFrameRenderingReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicFrameRenderingReport>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn MissedScanoutCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn SystemRelativeLatchTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn SystemRelativeScanoutStartTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn SystemRelativePhotonTime(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicFrameScanoutReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicFrameScanoutReport;{0ebbe606-03a0-5ca0-b46e-bba068d7233f})");
}
unsafe impl ::windows::runtime::Interface for HolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0ebbe606_03a0_5ca0_b46e_bba068d7233f);
}
impl ::windows::runtime::RuntimeName for HolographicFrameScanoutReport {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicFrameScanoutReport";
}
impl ::core::convert::From<HolographicFrameScanoutReport> for ::windows::runtime::IUnknown {
    fn from(value: HolographicFrameScanoutReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicFrameScanoutReport> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicFrameScanoutReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicFrameScanoutReport> for ::windows::runtime::IInspectable {
    fn from(value: HolographicFrameScanoutReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicFrameScanoutReport> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicFrameScanoutReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicFrameScanoutReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicFrameScanoutReport {}
unsafe impl ::core::marker::Sync for HolographicFrameScanoutReport {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicQuadLayer(pub ::windows::runtime::IInspectable);
impl HolographicQuadLayer {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX`*"]
    pub fn PixelFormat(&self) -> ::windows::runtime::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(size: Param0) -> ::windows::runtime::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), size.into_param().abi(), &mut result__).from_abi::<HolographicQuadLayer>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`, `Graphics_DirectX`*"]
    pub fn CreateWithPixelFormat<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(size: Param0, pixelformat: super::DirectX::DirectXPixelFormat) -> ::windows::runtime::Result<HolographicQuadLayer> {
        Self::IHolographicQuadLayerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), size.into_param().abi(), pixelformat, &mut result__).from_abi::<HolographicQuadLayer>(result__)
        })
    }
    pub fn IHolographicQuadLayerFactory<R, F: FnOnce(&IHolographicQuadLayerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HolographicQuadLayer, IHolographicQuadLayerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicQuadLayer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicQuadLayer;{903460c9-c9d9-5d5c-41ac-a2d5ab0fd331})");
}
unsafe impl ::windows::runtime::Interface for HolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x903460c9_c9d9_5d5c_41ac_a2d5ab0fd331);
}
impl ::windows::runtime::RuntimeName for HolographicQuadLayer {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayer";
}
impl ::core::convert::From<HolographicQuadLayer> for ::windows::runtime::IUnknown {
    fn from(value: HolographicQuadLayer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicQuadLayer> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicQuadLayer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicQuadLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicQuadLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicQuadLayer> for ::windows::runtime::IInspectable {
    fn from(value: HolographicQuadLayer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicQuadLayer> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicQuadLayer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicQuadLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicQuadLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HolographicQuadLayer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HolographicQuadLayer) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HolographicQuadLayer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HolographicQuadLayer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for HolographicQuadLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &HolographicQuadLayer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for HolographicQuadLayer {}
unsafe impl ::core::marker::Sync for HolographicQuadLayer {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicQuadLayerUpdateParameters(pub ::windows::runtime::IInspectable);
impl HolographicQuadLayerUpdateParameters {
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX_Direct3D11`*"]
    pub fn AcquireBufferToUpdateContent(&self) -> ::windows::runtime::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn UpdateViewport<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn UpdateContentProtectionEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
    pub fn UpdateExtents<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`, `Perception_Spatial`*"]
    pub fn UpdateLocationWithStationaryMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Perception::Spatial::SpatialCoordinateSystem>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, coordinatesystem: Param0, position: Param1, orientation: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), coordinatesystem.into_param().abi(), position.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
    pub fn UpdateLocationWithDisplayRelativeMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Vector3>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Quaternion>>(&self, position: Param0, orientation: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position.into_param().abi(), orientation.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn CanAcquireWithHardwareProtection(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX_Direct3D11`*"]
    pub fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> ::windows::runtime::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows::runtime::Interface::cast::<IHolographicQuadLayerUpdateParameters2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::Direct3D11::IDirect3DSurface>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicQuadLayerUpdateParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters;{2b0ea3b0-798d-5bca-55c2-2c0c762ebb08})");
}
unsafe impl ::windows::runtime::Interface for HolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2b0ea3b0_798d_5bca_55c2_2c0c762ebb08);
}
impl ::windows::runtime::RuntimeName for HolographicQuadLayerUpdateParameters {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicQuadLayerUpdateParameters";
}
impl ::core::convert::From<HolographicQuadLayerUpdateParameters> for ::windows::runtime::IUnknown {
    fn from(value: HolographicQuadLayerUpdateParameters) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicQuadLayerUpdateParameters> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicQuadLayerUpdateParameters) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicQuadLayerUpdateParameters> for ::windows::runtime::IInspectable {
    fn from(value: HolographicQuadLayerUpdateParameters) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicQuadLayerUpdateParameters> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicQuadLayerUpdateParameters) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicQuadLayerUpdateParameters {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicQuadLayerUpdateParameters {}
unsafe impl ::core::marker::Sync for HolographicQuadLayerUpdateParameters {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: HolographicReprojectionMode = HolographicReprojectionMode(0i32);
    pub const OrientationOnly: HolographicReprojectionMode = HolographicReprojectionMode(1i32);
    pub const Disabled: HolographicReprojectionMode = HolographicReprojectionMode(2i32);
}
impl ::core::convert::From<i32> for HolographicReprojectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HolographicReprojectionMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicReprojectionMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicReprojectionMode;i4)");
}
impl ::windows::runtime::DefaultType for HolographicReprojectionMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicSpace(pub ::windows::runtime::IInspectable);
impl HolographicSpace {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn PrimaryAdapterId(&self) -> ::windows::runtime::Result<HolographicAdapterId> {
        let this = self;
        unsafe {
            let mut result__: HolographicAdapterId = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicAdapterId>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX_Direct3D11`*"]
    pub fn SetDirect3D11Device<'a, Param0: ::windows::runtime::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DDevice>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn CameraAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RemoveCameraAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn CameraRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RemoveCameraRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn CreateNextFrame(&self) -> ::windows::runtime::Result<HolographicFrame> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicFrame>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `Graphics_Holographic`, `UI_Core`*"]
    pub fn CreateForCoreWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Core::CoreWindow>>(window: Param0) -> ::windows::runtime::Result<HolographicSpace> {
        Self::IHolographicSpaceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), window.into_param().abi(), &mut result__).from_abi::<HolographicSpace>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsSupported() -> ::windows::runtime::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsAvailable() -> ::windows::runtime::Result<bool> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn IsAvailableChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<::windows::runtime::IInspectable>>>(handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IHolographicSpaceStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RemoveIsAvailableChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(token: Param0) -> ::windows::runtime::Result<()> {
        Self::IHolographicSpaceStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsConfigured() -> ::windows::runtime::Result<bool> {
        Self::IHolographicSpaceStatics3(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn UserPresence(&self) -> ::windows::runtime::Result<HolographicSpaceUserPresence> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__: HolographicSpaceUserPresence = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicSpaceUserPresence>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn UserPresenceChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RemoveUserPresenceChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn WaitForNextFrameReady(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn WaitForNextFrameReadyWithHeadStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, requestedheadstartduration: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), requestedheadstartduration.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> ::windows::runtime::Result<HolographicFramePresentationMonitor> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), maxqueuedreports, &mut result__).from_abi::<HolographicFramePresentationMonitor>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn CreateFrameScanoutMonitor(&self, maxqueuedreports: u32) -> ::windows::runtime::Result<HolographicFrameScanoutMonitor> {
        let this = &::windows::runtime::Interface::cast::<IHolographicSpace3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), maxqueuedreports, &mut result__).from_abi::<HolographicFrameScanoutMonitor>(result__)
        }
    }
    pub fn IHolographicSpaceStatics<R, F: FnOnce(&IHolographicSpaceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HolographicSpace, IHolographicSpaceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHolographicSpaceStatics2<R, F: FnOnce(&IHolographicSpaceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HolographicSpace, IHolographicSpaceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHolographicSpaceStatics3<R, F: FnOnce(&IHolographicSpaceStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HolographicSpace, IHolographicSpaceStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicSpace {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpace;{4380dba6-5e78-434f-807c-3433d1efe8b7})");
}
unsafe impl ::windows::runtime::Interface for HolographicSpace {
    type Vtable = IHolographicSpace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4380dba6_5e78_434f_807c_3433d1efe8b7);
}
impl ::windows::runtime::RuntimeName for HolographicSpace {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpace";
}
impl ::core::convert::From<HolographicSpace> for ::windows::runtime::IUnknown {
    fn from(value: HolographicSpace) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicSpace> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicSpace) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicSpace> for ::windows::runtime::IInspectable {
    fn from(value: HolographicSpace) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicSpace> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicSpace) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicSpace {}
unsafe impl ::core::marker::Sync for HolographicSpace {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicSpaceCameraAddedEventArgs(pub ::windows::runtime::IInspectable);
impl HolographicSpaceCameraAddedEventArgs {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Camera(&self) -> ::windows::runtime::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicCamera>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicSpaceCameraAddedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs;{58f1da35-bbb3-3c8f-993d-6c80e7feb99f})");
}
unsafe impl ::windows::runtime::Interface for HolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x58f1da35_bbb3_3c8f_993d_6c80e7feb99f);
}
impl ::windows::runtime::RuntimeName for HolographicSpaceCameraAddedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraAddedEventArgs";
}
impl ::core::convert::From<HolographicSpaceCameraAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HolographicSpaceCameraAddedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicSpaceCameraAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicSpaceCameraAddedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicSpaceCameraAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HolographicSpaceCameraAddedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicSpaceCameraAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicSpaceCameraAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicSpaceCameraAddedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicSpaceCameraAddedEventArgs {}
unsafe impl ::core::marker::Sync for HolographicSpaceCameraAddedEventArgs {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicSpaceCameraRemovedEventArgs(pub ::windows::runtime::IInspectable);
impl HolographicSpaceCameraRemovedEventArgs {
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Camera(&self) -> ::windows::runtime::Result<HolographicCamera> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicCamera>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicSpaceCameraRemovedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs;{805444a8-f2ae-322e-8da9-836a0a95a4c1})");
}
unsafe impl ::windows::runtime::Interface for HolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x805444a8_f2ae_322e_8da9_836a0a95a4c1);
}
impl ::windows::runtime::RuntimeName for HolographicSpaceCameraRemovedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicSpaceCameraRemovedEventArgs";
}
impl ::core::convert::From<HolographicSpaceCameraRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HolographicSpaceCameraRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicSpaceCameraRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicSpaceCameraRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicSpaceCameraRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HolographicSpaceCameraRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicSpaceCameraRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicSpaceCameraRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicSpaceCameraRemovedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicSpaceCameraRemovedEventArgs {}
unsafe impl ::core::marker::Sync for HolographicSpaceCameraRemovedEventArgs {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: HolographicSpaceUserPresence = HolographicSpaceUserPresence(0i32);
    pub const PresentPassive: HolographicSpaceUserPresence = HolographicSpaceUserPresence(1i32);
    pub const PresentActive: HolographicSpaceUserPresence = HolographicSpaceUserPresence(2i32);
}
impl ::core::convert::From<i32> for HolographicSpaceUserPresence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HolographicSpaceUserPresence {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicSpaceUserPresence {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicSpaceUserPresence;i4)");
}
impl ::windows::runtime::DefaultType for HolographicSpaceUserPresence {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
#[doc = "*Required features: `Graphics_Holographic`, `Foundation_Numerics`*"]
pub struct HolographicStereoTransform {
    pub Left: super::super::Foundation::Numerics::Matrix4x4,
    pub Right: super::super::Foundation::Numerics::Matrix4x4,
}
#[cfg(feature = "Foundation_Numerics")]
impl HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for HolographicStereoTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for HolographicStereoTransform {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HolographicStereoTransform").field("Left", &self.Left).field("Right", &self.Right).finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for HolographicStereoTransform {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Right == other.Right
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::Abi for HolographicStereoTransform {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::runtime::RuntimeType for HolographicStereoTransform {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Holographic.HolographicStereoTransform;struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4))");
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::runtime::DefaultType for HolographicStereoTransform {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HolographicViewConfiguration(pub ::windows::runtime::IInspectable);
impl HolographicViewConfiguration {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn NativeRenderTargetSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RenderTargetSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation`*"]
    pub fn RequestRenderTargetSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, size: Param0) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), size.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`, `Graphics_DirectX`*"]
    pub fn SupportedPixelFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX`*"]
    pub fn PixelFormat(&self) -> ::windows::runtime::Result<super::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::DirectX::DirectXPixelFormat = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DirectX::DirectXPixelFormat>(result__)
        }
    }
    #[cfg(feature = "Graphics_DirectX")]
    #[doc = "*Required features: `Graphics_Holographic`, `Graphics_DirectX`*"]
    pub fn SetPixelFormat(&self, value: super::DirectX::DirectXPixelFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsStereo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn RefreshRate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<HolographicViewConfigurationKind> {
        let this = self;
        unsafe {
            let mut result__: HolographicViewConfigurationKind = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicViewConfigurationKind>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn Display(&self) -> ::windows::runtime::Result<HolographicDisplay> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HolographicDisplay>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Holographic`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Holographic`, `Foundation_Collections`*"]
    pub fn SupportedDepthReprojectionMethods(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>> {
        let this = &::windows::runtime::Interface::cast::<IHolographicViewConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HolographicViewConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Holographic.HolographicViewConfiguration;{5c1de6e6-67e9-5004-b02c-67a3a122b576})");
}
unsafe impl ::windows::runtime::Interface for HolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c1de6e6_67e9_5004_b02c_67a3a122b576);
}
impl ::windows::runtime::RuntimeName for HolographicViewConfiguration {
    const NAME: &'static str = "Windows.Graphics.Holographic.HolographicViewConfiguration";
}
impl ::core::convert::From<HolographicViewConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: HolographicViewConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HolographicViewConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &HolographicViewConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HolographicViewConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a HolographicViewConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HolographicViewConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: HolographicViewConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HolographicViewConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &HolographicViewConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HolographicViewConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HolographicViewConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for HolographicViewConfiguration {}
unsafe impl ::core::marker::Sync for HolographicViewConfiguration {}
#[doc = "*Required features: `Graphics_Holographic`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: HolographicViewConfigurationKind = HolographicViewConfigurationKind(0i32);
    pub const PhotoVideoCamera: HolographicViewConfigurationKind = HolographicViewConfigurationKind(1i32);
}
impl ::core::convert::From<i32> for HolographicViewConfigurationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HolographicViewConfigurationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HolographicViewConfigurationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Holographic.HolographicViewConfigurationKind;i4)");
}
impl ::windows::runtime::DefaultType for HolographicViewConfigurationKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCamera(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCamera {
    type Vtable = IHolographicCamera_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe4e98445_9bed_4980_9ba0_e87680d1cb74);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCamera2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCamera2 {
    type Vtable = IHolographicCamera2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb55b9f1a_ba8c_4f84_ad79_2e7e1e2450f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCamera3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCamera3 {
    type Vtable = IHolographicCamera3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x45aa4fb3_7b59_524e_4a3f_4a6ad6650477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCamera4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCamera4 {
    type Vtable = IHolographicCamera4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9a2531d6_4723_4f39_a9a5_9d05181d9b44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCamera5(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCamera5 {
    type Vtable = IHolographicCamera5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x229706f2_628d_4ef5_9c08_a63fdd7787c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCamera6(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCamera6 {
    type Vtable = IHolographicCamera6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0209194f_632d_5154_ab52_0b5d15b12505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCamera6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraPose(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraPose {
    type Vtable = IHolographicCameraPose_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0d7d7e30_12de_45bd_912b_c7f6561599d1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraPose_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicStereoTransform) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraPose2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraPose2 {
    type Vtable = IHolographicCameraPose2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x232be073_5d2d_4560_814e_2697c4fce16b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraPose2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, coordinatesystemtoviewtransform: HolographicStereoTransform) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, projectiontransform: HolographicStereoTransform) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, leftviewport: super::super::Foundation::Rect, rightviewport: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraRenderingParameters {
    type Vtable = IHolographicCameraRenderingParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x8eac2ed1_5bf4_4e16_8236_ae0800c11d0d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, position: super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, position: super::super::Foundation::Numerics::Vector3, normal: super::super::Foundation::Numerics::Vector3, linearvelocity: super::super::Foundation::Numerics::Vector3) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraRenderingParameters2 {
    type Vtable = IHolographicCameraRenderingParameters2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x261270e3_b696_4634_94d6_be0681643599);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicReprojectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: HolographicReprojectionMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraRenderingParameters3 {
    type Vtable = IHolographicCameraRenderingParameters3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb1aa513f_136d_4b06_b9d4_e4b914cd0683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraRenderingParameters4 {
    type Vtable = IHolographicCameraRenderingParameters4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0878fa4c_e163_57dc_82b7_c406ab3e0537);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParameters4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicDepthReprojectionMethod) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: HolographicDepthReprojectionMethod) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicCameraViewportParameters(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicCameraViewportParameters {
    type Vtable = IHolographicCameraViewportParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80cdf3f7_842a_41e1_93ed_5692ab1fbb10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraViewportParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut super::super::Foundation::Numerics::Vector2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicDisplay(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicDisplay {
    type Vtable = IHolographicDisplay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9acea414_1d9f_4090_a388_90c06f6eae9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicAdapterId) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Perception_Spatial")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception_Spatial"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicDisplay2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicDisplay2 {
    type Vtable = IHolographicDisplay2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x75ac3f82_e755_436c_8d96_4d32d131473e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicDisplay3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicDisplay3 {
    type Vtable = IHolographicDisplay3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xfc4c6ac6_6480_5008_b29e_157d77c843f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplay3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: HolographicViewConfigurationKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicDisplayStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicDisplayStatics {
    type Vtable = IHolographicDisplayStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xcb374983_e7b0_4841_8355_3ae5b536e9a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicDisplayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFrame(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFrame {
    type Vtable = IHolographicFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc6988eb6_a8b9_3054_a6eb_d624b6536375);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, camerapose: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicFramePresentResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitbehavior: HolographicFramePresentWaitBehavior, result__: *mut HolographicFramePresentResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFrame2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFrame2 {
    type Vtable = IHolographicFrame2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x283f37bf_3bf2_5e91_6633_870574e6f217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, layer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFrame3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFrame3 {
    type Vtable = IHolographicFrame3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe5e964c9_8a27_55d3_9f98_94530d369052);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrame3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicFrameId) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFramePrediction(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFramePrediction {
    type Vtable = IHolographicFramePrediction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x520f4de1_5c0a_4e79_a81e_6abe02bb2739);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePrediction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Perception")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Perception"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFramePresentationMonitor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFramePresentationMonitor {
    type Vtable = IHolographicFramePresentationMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xca87256c_6fae_428e_bb83_25dfee51136b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePresentationMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFramePresentationReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFramePresentationReport {
    type Vtable = IHolographicFramePresentationReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x80baf614_f2f4_4c8a_8de3_065c78f6d5de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFramePresentationReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFrameRenderingReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFrameRenderingReport {
    type Vtable = IHolographicFrameRenderingReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x05f32de4_e384_51b3_b934_f0d3a0f78606);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameRenderingReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicFrameId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutMonitor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFrameScanoutMonitor {
    type Vtable = IHolographicFrameScanoutMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7e83efa9_843c_5401_8095_9bc1b8b08638);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutReport(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicFrameScanoutReport {
    type Vtable = IHolographicFrameScanoutReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0ebbe606_03a0_5ca0_b46e_bba068d7233f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicFrameScanoutReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicQuadLayer(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayer {
    type Vtable = IHolographicQuadLayer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x903460c9_c9d9_5d5c_41ac_a2d5ab0fd331);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicQuadLayerFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerFactory {
    type Vtable = IHolographicQuadLayerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa67538f3_5a14_5a10_489a_455065b37b76);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: super::super::Foundation::Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerUpdateParameters {
    type Vtable = IHolographicQuadLayerUpdateParameters_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2b0ea3b0_798d_5bca_55c2_2c0c762ebb08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Numerics::Vector2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, coordinatesystem: ::windows::runtime::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: super::super::Foundation::Numerics::Vector3, orientation: super::super::Foundation::Numerics::Quaternion) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerUpdateParameters2 {
    type Vtable = IHolographicQuadLayerUpdateParameters2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4f33d32d_82c1_46c1_8980_3cb70d98182b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParameters2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpace(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpace {
    type Vtable = IHolographicSpace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4380dba6_5e78_434f_807c_3433d1efe8b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicAdapterId) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpace2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpace2 {
    type Vtable = IHolographicSpace2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4f81a9a8_b7ff_4883_9827_7d677287ea70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicSpaceUserPresence) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedheadstartduration: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxqueuedreports: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpace3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpace3 {
    type Vtable = IHolographicSpace3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xdf1733d1_f224_587e_8d71_1e8fc8f07b1f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpace3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxqueuedreports: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraAddedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpaceCameraAddedEventArgs {
    type Vtable = IHolographicSpaceCameraAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x58f1da35_bbb3_3c8f_993d_6c80e7feb99f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraAddedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraRemovedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpaceCameraRemovedEventArgs {
    type Vtable = IHolographicSpaceCameraRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x805444a8_f2ae_322e_8da9_836a0a95a4c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceCameraRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpaceStatics {
    type Vtable = IHolographicSpaceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x364e6064_c8f2_3ba1_8391_66b8489e67fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpaceStatics2 {
    type Vtable = IHolographicSpaceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0e777088_75fc_48af_8758_0652f6f07c59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicSpaceStatics3 {
    type Vtable = IHolographicSpaceStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x3b00de3d_b1a3_4dfe_8e79_fec5909e6df8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicViewConfiguration {
    type Vtable = IHolographicViewConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x5c1de6e6_67e9_5004_b02c_67a3a122b576);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_DirectX"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_DirectX")))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::DirectX::DirectXPixelFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    #[cfg(feature = "Graphics_DirectX")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::DirectX::DirectXPixelFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HolographicViewConfigurationKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHolographicViewConfiguration2 {
    type Vtable = IHolographicViewConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe241756e_e0d0_5019_9af5_1b165bc2f54e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicViewConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
