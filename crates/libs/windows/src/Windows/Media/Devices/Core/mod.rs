#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct CameraIntrinsics(::windows::core::IUnknown);
impl CameraIntrinsics {
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FocalLength(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FocalLength)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PrincipalPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrincipalPoint)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn RadialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RadialDistortion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TangentialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TangentialDistortion)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    pub fn ImageWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImageWidth)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn ImageHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImageHeight)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectOntoFrame(&self, coordinate: super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProjectOntoFrame)(::windows::core::Interface::as_raw(this), coordinate, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UnprojectAtUnitDepth(&self, pixelcoordinate: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnprojectAtUnitDepth)(::windows::core::Interface::as_raw(this), pixelcoordinate, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector2>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ProjectManyOntoFrame(&self, coordinates: &[super::super::super::Foundation::Numerics::Vector3], results: &mut [super::super::super::Foundation::Point]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ProjectManyOntoFrame)(::windows::core::Interface::as_raw(this), coordinates.len() as u32, coordinates.as_ptr(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UnprojectPixelsAtUnitDepth(&self, pixelcoordinates: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Numerics::Vector2]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnprojectPixelsAtUnitDepth)(::windows::core::Interface::as_raw(this), pixelcoordinates.len() as u32, pixelcoordinates.as_ptr(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn UndistortedProjectionTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UndistortedProjectionTransform)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DistortPoint(&self, input: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DistortPoint)(::windows::core::Interface::as_raw(this), input, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DistortPoints(&self, inputs: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Point]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DistortPoints)(::windows::core::Interface::as_raw(this), inputs.len() as u32, inputs.as_ptr(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UndistortPoint(&self, input: super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UndistortPoint)(::windows::core::Interface::as_raw(this), input, result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UndistortPoints(&self, inputs: &[super::super::super::Foundation::Point], results: &mut [super::super::super::Foundation::Point]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICameraIntrinsics2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).UndistortPoints)(::windows::core::Interface::as_raw(this), inputs.len() as u32, inputs.as_ptr(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create(focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32) -> ::windows::core::Result<CameraIntrinsics> {
        Self::ICameraIntrinsicsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), focallength, principalpoint, radialdistortion, tangentialdistortion, imagewidth, imageheight, result__.as_mut_ptr()).from_abi::<CameraIntrinsics>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICameraIntrinsicsFactory<R, F: FnOnce(&ICameraIntrinsicsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CameraIntrinsics, ICameraIntrinsicsFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CameraIntrinsics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CameraIntrinsics {}
impl ::core::fmt::Debug for CameraIntrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CameraIntrinsics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CameraIntrinsics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.CameraIntrinsics;{0aa6ed32-6589-49da-afde-594270ca0aac})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CameraIntrinsics {
    type Vtable = ICameraIntrinsics_Vtbl;
    const IID: ::windows::core::GUID = <ICameraIntrinsics as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CameraIntrinsics {
    const NAME: &'static str = "Windows.Media.Devices.Core.CameraIntrinsics";
}
impl ::core::convert::From<CameraIntrinsics> for ::windows::core::IUnknown {
    fn from(value: CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraIntrinsics> for ::windows::core::IUnknown {
    fn from(value: &CameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraIntrinsics> for &::windows::core::IUnknown {
    fn from(value: &CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<CameraIntrinsics> for ::windows::core::IInspectable {
    fn from(value: CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CameraIntrinsics> for ::windows::core::IInspectable {
    fn from(value: &CameraIntrinsics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&CameraIntrinsics> for &::windows::core::IInspectable {
    fn from(value: &CameraIntrinsics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for CameraIntrinsics {}
unsafe impl ::core::marker::Sync for CameraIntrinsics {}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct DepthCorrelatedCoordinateMapper(::windows::core::IUnknown);
impl DepthCorrelatedCoordinateMapper {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoint<'a, P0>(&self, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: P0) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnprojectPoint)(::windows::core::Interface::as_raw(this), sourcepoint, targetcoordinatesystem.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Numerics::Vector3>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub fn UnprojectPoints<'a, P0>(&self, sourcepoints: &[super::super::super::Foundation::Point], targetcoordinatesystem: P0, results: &mut [super::super::super::Foundation::Numerics::Vector3]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnprojectPoints)(::windows::core::Interface::as_raw(this), sourcepoints.len() as u32, sourcepoints.as_ptr(), targetcoordinatesystem.into().abi(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub fn MapPoint<'a, P0, P1>(&self, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: P0, targetcameraintrinsics: P1) -> ::windows::core::Result<super::super::super::Foundation::Point>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, CameraIntrinsics>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MapPoint)(::windows::core::Interface::as_raw(this), sourcepoint, targetcoordinatesystem.into().abi(), targetcameraintrinsics.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Perception_Spatial\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub fn MapPoints<'a, P0, P1>(&self, sourcepoints: &[super::super::super::Foundation::Point], targetcoordinatesystem: P0, targetcameraintrinsics: P1, results: &mut [super::super::super::Foundation::Point]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Perception::Spatial::SpatialCoordinateSystem>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, CameraIntrinsics>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).MapPoints)(::windows::core::Interface::as_raw(this), sourcepoints.len() as u32, sourcepoints.as_ptr(), targetcoordinatesystem.into().abi(), targetcameraintrinsics.into().abi(), results.len() as u32, results.as_mut_ptr()).ok() }
    }
}
impl ::core::clone::Clone for DepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DepthCorrelatedCoordinateMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DepthCorrelatedCoordinateMapper {}
impl ::core::fmt::Debug for DepthCorrelatedCoordinateMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DepthCorrelatedCoordinateMapper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DepthCorrelatedCoordinateMapper {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper;{f95d89fb-8af0-4cb0-926d-696866e5046a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows::core::GUID = <IDepthCorrelatedCoordinateMapper as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DepthCorrelatedCoordinateMapper {
    const NAME: &'static str = "Windows.Media.Devices.Core.DepthCorrelatedCoordinateMapper";
}
impl ::core::convert::From<DepthCorrelatedCoordinateMapper> for ::windows::core::IUnknown {
    fn from(value: DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for ::windows::core::IUnknown {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for &::windows::core::IUnknown {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DepthCorrelatedCoordinateMapper> for ::windows::core::IInspectable {
    fn from(value: DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for ::windows::core::IInspectable {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DepthCorrelatedCoordinateMapper> for &::windows::core::IInspectable {
    fn from(value: &DepthCorrelatedCoordinateMapper) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DepthCorrelatedCoordinateMapper> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: DepthCorrelatedCoordinateMapper) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DepthCorrelatedCoordinateMapper> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DepthCorrelatedCoordinateMapper) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&DepthCorrelatedCoordinateMapper> for ::windows::core::InParam<'a, super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DepthCorrelatedCoordinateMapper) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DepthCorrelatedCoordinateMapper {}
unsafe impl ::core::marker::Sync for DepthCorrelatedCoordinateMapper {}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameControlCapabilities(::windows::core::IUnknown);
impl FrameControlCapabilities {
    pub fn Exposure(&self) -> ::windows::core::Result<FrameExposureCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Exposure)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureCapabilities>(result__)
        }
    }
    pub fn ExposureCompensation(&self) -> ::windows::core::Result<FrameExposureCompensationCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExposureCompensation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureCompensationCapabilities>(result__)
        }
    }
    pub fn IsoSpeed(&self) -> ::windows::core::Result<FrameIsoSpeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsoSpeed)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameIsoSpeedCapabilities>(result__)
        }
    }
    pub fn Focus(&self) -> ::windows::core::Result<FrameFocusCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Focus)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFocusCapabilities>(result__)
        }
    }
    pub fn PhotoConfirmationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotoConfirmationSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Flash(&self) -> ::windows::core::Result<FrameFlashCapabilities> {
        let this = &::windows::core::Interface::cast::<IFrameControlCapabilities2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Flash)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFlashCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameControlCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameControlCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameControlCapabilities {}
impl ::core::fmt::Debug for FrameControlCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameControlCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameControlCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameControlCapabilities;{a8ffae60-4e9e-4377-a789-e24c4ae7e544})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameControlCapabilities {
    type Vtable = IFrameControlCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IFrameControlCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameControlCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameControlCapabilities";
}
impl ::core::convert::From<FrameControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for &::windows::core::IUnknown {
    fn from(value: &FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameControlCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameControlCapabilities> for &::windows::core::IInspectable {
    fn from(value: &FrameControlCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameController(::windows::core::IUnknown);
impl FrameController {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FrameController, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ExposureControl(&self) -> ::windows::core::Result<FrameExposureControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExposureControl)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureControl>(result__)
        }
    }
    pub fn ExposureCompensationControl(&self) -> ::windows::core::Result<FrameExposureCompensationControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExposureCompensationControl)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameExposureCompensationControl>(result__)
        }
    }
    pub fn IsoSpeedControl(&self) -> ::windows::core::Result<FrameIsoSpeedControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsoSpeedControl)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameIsoSpeedControl>(result__)
        }
    }
    pub fn FocusControl(&self) -> ::windows::core::Result<FrameFocusControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FocusControl)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFocusControl>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoConfirmationEnabled(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotoConfirmationEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPhotoConfirmationEnabled<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<bool>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPhotoConfirmationEnabled)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn FlashControl(&self) -> ::windows::core::Result<FrameFlashControl> {
        let this = &::windows::core::Interface::cast::<IFrameController2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FlashControl)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFlashControl>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameController {}
impl ::core::fmt::Debug for FrameController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameController;{c16459d9-baef-4052-9177-48aff2af7522})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameController {
    type Vtable = IFrameController_Vtbl;
    const IID: ::windows::core::GUID = <IFrameController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameController {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameController";
}
impl ::core::convert::From<FrameController> for ::windows::core::IUnknown {
    fn from(value: FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameController> for ::windows::core::IUnknown {
    fn from(value: &FrameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameController> for &::windows::core::IUnknown {
    fn from(value: &FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameController> for ::windows::core::IInspectable {
    fn from(value: FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameController> for ::windows::core::IInspectable {
    fn from(value: &FrameController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameController> for &::windows::core::IInspectable {
    fn from(value: &FrameController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for FrameController {}
unsafe impl ::core::marker::Sync for FrameController {}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameExposureCapabilities(::windows::core::IUnknown);
impl FrameExposureCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Supported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Min(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Min)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Max(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Max)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Step(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Step)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameExposureCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCapabilities {}
impl ::core::fmt::Debug for FrameExposureCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameExposureCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCapabilities;{bdbe9ce3-3985-4e72-97c2-0590d61307a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IFrameExposureCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameExposureCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCapabilities";
}
impl ::core::convert::From<FrameExposureCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for &::windows::core::IUnknown {
    fn from(value: &FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameExposureCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureCapabilities> for &::windows::core::IInspectable {
    fn from(value: &FrameExposureCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameExposureCompensationCapabilities(::windows::core::IUnknown);
impl FrameExposureCompensationCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Supported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Min)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Max)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Step)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameExposureCompensationCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCompensationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCompensationCapabilities {}
impl ::core::fmt::Debug for FrameExposureCompensationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureCompensationCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameExposureCompensationCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCompensationCapabilities;{b988a823-8065-41ee-b04f-722265954500})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IFrameExposureCompensationCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameExposureCompensationCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationCapabilities";
}
impl ::core::convert::From<FrameExposureCompensationCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for &::windows::core::IUnknown {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameExposureCompensationCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureCompensationCapabilities> for &::windows::core::IInspectable {
    fn from(value: &FrameExposureCompensationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameExposureCompensationControl(::windows::core::IUnknown);
impl FrameExposureCompensationControl {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<f32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<f32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameExposureCompensationControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureCompensationControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureCompensationControl {}
impl ::core::fmt::Debug for FrameExposureCompensationControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureCompensationControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameExposureCompensationControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureCompensationControl;{e95896c9-f7f9-48ca-8591-a26531cb1578})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControl_Vtbl;
    const IID: ::windows::core::GUID = <IFrameExposureCompensationControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameExposureCompensationControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureCompensationControl";
}
impl ::core::convert::From<FrameExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for &::windows::core::IUnknown {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureCompensationControl> for &::windows::core::IInspectable {
    fn from(value: &FrameExposureCompensationControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameExposureControl(::windows::core::IUnknown);
impl FrameExposureControl {
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Auto)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuto)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameExposureControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameExposureControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameExposureControl {}
impl ::core::fmt::Debug for FrameExposureControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameExposureControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameExposureControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameExposureControl;{b1605a61-ffaf-4752-b621-f5b6f117f432})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameExposureControl {
    type Vtable = IFrameExposureControl_Vtbl;
    const IID: ::windows::core::GUID = <IFrameExposureControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameExposureControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameExposureControl";
}
impl ::core::convert::From<FrameExposureControl> for ::windows::core::IUnknown {
    fn from(value: FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureControl> for ::windows::core::IUnknown {
    fn from(value: &FrameExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureControl> for &::windows::core::IUnknown {
    fn from(value: &FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameExposureControl> for ::windows::core::IInspectable {
    fn from(value: FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameExposureControl> for ::windows::core::IInspectable {
    fn from(value: &FrameExposureControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameExposureControl> for &::windows::core::IInspectable {
    fn from(value: &FrameExposureControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameFlashCapabilities(::windows::core::IUnknown);
impl FrameFlashCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Supported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RedEyeReductionSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn PowerSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PowerSupported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameFlashCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFlashCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashCapabilities {}
impl ::core::fmt::Debug for FrameFlashCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFlashCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameFlashCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFlashCapabilities;{bb9341a2-5ebe-4f62-8223-0e2b05bfbbd0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IFrameFlashCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameFlashCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashCapabilities";
}
impl ::core::convert::From<FrameFlashCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameFlashCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for &::windows::core::IUnknown {
    fn from(value: &FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameFlashCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameFlashCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFlashCapabilities> for &::windows::core::IInspectable {
    fn from(value: &FrameFlashCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameFlashControl(::windows::core::IUnknown);
impl FrameFlashControl {
    pub fn Mode(&self) -> ::windows::core::Result<FrameFlashMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameFlashMode>(result__)
        }
    }
    pub fn SetMode(&self, value: FrameFlashMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Auto)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuto)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RedEyeReduction(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RedEyeReduction)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRedEyeReduction)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PowerPercent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PowerPercent)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPowerPercent)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for FrameFlashControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFlashControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFlashControl {}
impl ::core::fmt::Debug for FrameFlashControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFlashControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameFlashControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFlashControl;{75d5f6c7-bd45-4fab-9375-45ac04b332c2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameFlashControl {
    type Vtable = IFrameFlashControl_Vtbl;
    const IID: ::windows::core::GUID = <IFrameFlashControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameFlashControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFlashControl";
}
impl ::core::convert::From<FrameFlashControl> for ::windows::core::IUnknown {
    fn from(value: FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashControl> for ::windows::core::IUnknown {
    fn from(value: &FrameFlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFlashControl> for &::windows::core::IUnknown {
    fn from(value: &FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameFlashControl> for ::windows::core::IInspectable {
    fn from(value: FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFlashControl> for ::windows::core::IInspectable {
    fn from(value: &FrameFlashControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFlashControl> for &::windows::core::IInspectable {
    fn from(value: &FrameFlashControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FrameFlashMode(pub i32);
impl FrameFlashMode {
    pub const Disable: Self = Self(0i32);
    pub const Enable: Self = Self(1i32);
    pub const Global: Self = Self(2i32);
}
impl ::core::marker::Copy for FrameFlashMode {}
impl ::core::clone::Clone for FrameFlashMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FrameFlashMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FrameFlashMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for FrameFlashMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFlashMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameFlashMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Devices.Core.FrameFlashMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameFocusCapabilities(::windows::core::IUnknown);
impl FrameFocusCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Supported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Min)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Max)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Step)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameFocusCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFocusCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFocusCapabilities {}
impl ::core::fmt::Debug for FrameFocusCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFocusCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameFocusCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFocusCapabilities;{7b25cd58-01c0-4065-9c40-c1a721425c1a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IFrameFocusCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameFocusCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusCapabilities";
}
impl ::core::convert::From<FrameFocusCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameFocusCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for &::windows::core::IUnknown {
    fn from(value: &FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameFocusCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameFocusCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFocusCapabilities> for &::windows::core::IInspectable {
    fn from(value: &FrameFocusCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameFocusControl(::windows::core::IUnknown);
impl FrameFocusControl {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameFocusControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameFocusControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameFocusControl {}
impl ::core::fmt::Debug for FrameFocusControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameFocusControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameFocusControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameFocusControl;{272df1d0-d912-4214-a67b-e38a8d48d8c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameFocusControl {
    type Vtable = IFrameFocusControl_Vtbl;
    const IID: ::windows::core::GUID = <IFrameFocusControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameFocusControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameFocusControl";
}
impl ::core::convert::From<FrameFocusControl> for ::windows::core::IUnknown {
    fn from(value: FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusControl> for ::windows::core::IUnknown {
    fn from(value: &FrameFocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFocusControl> for &::windows::core::IUnknown {
    fn from(value: &FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameFocusControl> for ::windows::core::IInspectable {
    fn from(value: FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameFocusControl> for ::windows::core::IInspectable {
    fn from(value: &FrameFocusControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameFocusControl> for &::windows::core::IInspectable {
    fn from(value: &FrameFocusControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameIsoSpeedCapabilities(::windows::core::IUnknown);
impl FrameIsoSpeedCapabilities {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Supported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn Min(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Min)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Max(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Max)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn Step(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Step)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for FrameIsoSpeedCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameIsoSpeedCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameIsoSpeedCapabilities {}
impl ::core::fmt::Debug for FrameIsoSpeedCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameIsoSpeedCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameIsoSpeedCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameIsoSpeedCapabilities;{16bdff61-6df6-4ac9-b92a-9f6ecd1ad2fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IFrameIsoSpeedCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameIsoSpeedCapabilities {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedCapabilities";
}
impl ::core::convert::From<FrameIsoSpeedCapabilities> for ::windows::core::IUnknown {
    fn from(value: FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for ::windows::core::IUnknown {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for &::windows::core::IUnknown {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameIsoSpeedCapabilities> for ::windows::core::IInspectable {
    fn from(value: FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for ::windows::core::IInspectable {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameIsoSpeedCapabilities> for &::windows::core::IInspectable {
    fn from(value: &FrameIsoSpeedCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct FrameIsoSpeedControl(::windows::core::IUnknown);
impl FrameIsoSpeedControl {
    pub fn Auto(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Auto)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetAuto(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAuto)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetValue<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::super::Foundation::IReference<u32>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
impl ::core::clone::Clone for FrameIsoSpeedControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FrameIsoSpeedControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FrameIsoSpeedControl {}
impl ::core::fmt::Debug for FrameIsoSpeedControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FrameIsoSpeedControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FrameIsoSpeedControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.FrameIsoSpeedControl;{1a03efed-786a-4c75-a557-7ab9a85f588c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControl_Vtbl;
    const IID: ::windows::core::GUID = <IFrameIsoSpeedControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FrameIsoSpeedControl {
    const NAME: &'static str = "Windows.Media.Devices.Core.FrameIsoSpeedControl";
}
impl ::core::convert::From<FrameIsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for ::windows::core::IUnknown {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for &::windows::core::IUnknown {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FrameIsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for ::windows::core::IInspectable {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FrameIsoSpeedControl> for &::windows::core::IInspectable {
    fn from(value: &FrameIsoSpeedControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraIntrinsics {
    type Vtable = ICameraIntrinsics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0aa6ed32_6589_49da_afde_594270ca0aac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub FocalLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FocalLength: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub PrincipalPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    PrincipalPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub RadialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    RadialDistortion: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TangentialDistortion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TangentialDistortion: usize,
    pub ImageWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ImageHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectOntoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinate: super::super::super::Foundation::Numerics::Vector3, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectAtUnitDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelcoordinate: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectAtUnitDepth: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ProjectManyOntoFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coordinates_array_size: u32, coordinates: *const super::super::super::Foundation::Numerics::Vector3, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ProjectManyOntoFrame: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub UnprojectPixelsAtUnitDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelCoordinates_array_size: u32, pixelcoordinates: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UnprojectPixelsAtUnitDepth: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraIntrinsics2 {
    type Vtable = ICameraIntrinsics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cdaa447_0798_4b4d_839f_c5ec414db27a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub UndistortedProjectionTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    UndistortedProjectionTransform: usize,
    #[cfg(feature = "Foundation")]
    pub DistortPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistortPoint: usize,
    #[cfg(feature = "Foundation")]
    pub DistortPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DistortPoints: usize,
    #[cfg(feature = "Foundation")]
    pub UndistortPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, input: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UndistortPoint: usize,
    #[cfg(feature = "Foundation")]
    pub UndistortPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputs_array_size: u32, inputs: *const super::super::super::Foundation::Point, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UndistortPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICameraIntrinsicsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICameraIntrinsicsFactory {
    type Vtable = ICameraIntrinsicsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0ddc486_2132_4a34_a659_9bfe2a055712);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraIntrinsicsFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, focallength: super::super::super::Foundation::Numerics::Vector2, principalpoint: super::super::super::Foundation::Numerics::Vector2, radialdistortion: super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDepthCorrelatedCoordinateMapper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDepthCorrelatedCoordinateMapper {
    type Vtable = IDepthCorrelatedCoordinateMapper_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf95d89fb_8af0_4cb0_926d_696866e5046a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDepthCorrelatedCoordinateMapper_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoint: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "Perception_Spatial"))]
    pub UnprojectPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "Perception_Spatial")))]
    UnprojectPoints: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub MapPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcepoint: super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, targetcameraintrinsics: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))]
    MapPoint: usize,
    #[cfg(all(feature = "Foundation", feature = "Perception_Spatial"))]
    pub MapPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcePoints_array_size: u32, sourcepoints: *const super::super::super::Foundation::Point, targetcoordinatesystem: *mut ::core::ffi::c_void, targetcameraintrinsics: *mut ::core::ffi::c_void, results_array_size: u32, results: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Perception_Spatial")))]
    MapPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameControlCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameControlCapabilities {
    type Vtable = IFrameControlCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8ffae60_4e9e_4377_a789_e24c4ae7e544);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControlCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Exposure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsoSpeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Focus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PhotoConfirmationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameControlCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameControlCapabilities2 {
    type Vtable = IFrameControlCapabilities2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xce9b0464_4730_440f_bd3e_efe8a8f230a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameControlCapabilities2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Flash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameController {
    type Vtable = IFrameController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc16459d9_baef_4052_9177_48aff2af7522);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ExposureControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExposureCompensationControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsoSpeedControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FocusControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PhotoConfirmationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoConfirmationEnabled: usize,
    #[cfg(feature = "Foundation")]
    pub SetPhotoConfirmationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPhotoConfirmationEnabled: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameController2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameController2 {
    type Vtable = IFrameController2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d3bc75_d87c_485b_8a09_5c358568b427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameController2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FlashControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureCapabilities {
    type Vtable = IFrameExposureCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbdbe9ce3_3985_4e72_97c2_0590d61307a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Min: usize,
    #[cfg(feature = "Foundation")]
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Max: usize,
    #[cfg(feature = "Foundation")]
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Step: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCompensationCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureCompensationCapabilities {
    type Vtable = IFrameExposureCompensationCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb988a823_8065_41ee_b04f_722265954500);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCompensationCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureCompensationControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureCompensationControl {
    type Vtable = IFrameExposureCompensationControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe95896c9_f7f9_48ca_8591_a26531cb1578);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureCompensationControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameExposureControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameExposureControl {
    type Vtable = IFrameExposureControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1605a61_ffaf_4752_b621_f5b6f117f432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameExposureControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFlashCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFlashCapabilities {
    type Vtable = IFrameFlashCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb9341a2_5ebe_4f62_8223_0e2b05bfbbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFlashCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RedEyeReductionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFlashControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFlashControl {
    type Vtable = IFrameFlashControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75d5f6c7_bd45_4fab_9375_45ac04b332c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFlashControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FrameFlashMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FrameFlashMode) -> ::windows::core::HRESULT,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetRedEyeReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPowerPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFocusCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFocusCapabilities {
    type Vtable = IFrameFocusCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b25cd58_01c0_4065_9c40_c1a721425c1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFocusCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameFocusControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameFocusControl {
    type Vtable = IFrameFocusControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x272df1d0_d912_4214_a67b_e38a8d48d8c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameFocusControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameIsoSpeedCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameIsoSpeedCapabilities {
    type Vtable = IFrameIsoSpeedCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16bdff61_6df6_4ac9_b92a_9f6ecd1ad2fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameIsoSpeedCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Min: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Max: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Step: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFrameIsoSpeedControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFrameIsoSpeedControl {
    type Vtable = IFrameIsoSpeedControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a03efed_786a_4c75_a557_7ab9a85f588c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameIsoSpeedControl_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Auto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAuto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Value: usize,
    #[cfg(feature = "Foundation")]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetValue: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceController(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IVariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceController_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7fbff880_ed8c_43fd_a7c3_b35809e4229a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceController_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MaxPhotosPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub PhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPhotosPerSecondLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetHighestConcurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, captureproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetHighestConcurrentFrameRate: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetCurrentFrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetCurrentFrameRate: usize,
    pub FrameCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFrameControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFrameControllers: usize,
}
#[doc = "*Required features: `\"Media_Devices_Core\"`*"]
#[repr(transparent)]
pub struct VariablePhotoSequenceController(::windows::core::IUnknown);
impl VariablePhotoSequenceController {
    pub fn Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Supported)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MaxPhotosPerSecond)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosPerSecondLimit)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPhotosPerSecondLimit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetHighestConcurrentFrameRate<'a, P0, E0>(&self, captureproperties: P0) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::MediaProperties::IMediaEncodingProperties>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetHighestConcurrentFrameRate)(::windows::core::Interface::as_raw(this), captureproperties.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentFrameRate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::MediaProperties::MediaRatio>(result__)
        }
    }
    pub fn FrameCapabilities(&self) -> ::windows::core::Result<FrameControlCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameCapabilities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<FrameControlCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFrameControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<FrameController>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesiredFrameControllers)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<FrameController>>(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoSequenceController {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for VariablePhotoSequenceController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoSequenceController {}
impl ::core::fmt::Debug for VariablePhotoSequenceController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoSequenceController").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for VariablePhotoSequenceController {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Devices.Core.VariablePhotoSequenceController;{7fbff880-ed8c-43fd-a7c3-b35809e4229a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for VariablePhotoSequenceController {
    type Vtable = IVariablePhotoSequenceController_Vtbl;
    const IID: ::windows::core::GUID = <IVariablePhotoSequenceController as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VariablePhotoSequenceController {
    const NAME: &'static str = "Windows.Media.Devices.Core.VariablePhotoSequenceController";
}
impl ::core::convert::From<VariablePhotoSequenceController> for ::windows::core::IUnknown {
    fn from(value: VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for ::windows::core::IUnknown {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for &::windows::core::IUnknown {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<VariablePhotoSequenceController> for ::windows::core::IInspectable {
    fn from(value: VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for ::windows::core::IInspectable {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&VariablePhotoSequenceController> for &::windows::core::IInspectable {
    fn from(value: &VariablePhotoSequenceController) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
