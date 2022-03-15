#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: Self = Self(0i32);
    pub const RgbFull: Self = Self(1i32);
    pub const BT2020: Self = Self(2i32);
    pub const BT709: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayColorSpace {}
impl ::core::clone::Clone for HdmiDisplayColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayColorSpace {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayColorSpace {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayColorSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayColorSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayColorSpace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayColorSpace;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
pub struct HdmiDisplayHdr2086Metadata {
    pub RedPrimaryX: u16,
    pub RedPrimaryY: u16,
    pub GreenPrimaryX: u16,
    pub GreenPrimaryY: u16,
    pub BluePrimaryX: u16,
    pub BluePrimaryY: u16,
    pub WhitePointX: u16,
    pub WhitePointY: u16,
    pub MaxMasteringLuminance: u16,
    pub MinMasteringLuminance: u16,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl ::core::marker::Copy for HdmiDisplayHdr2086Metadata {}
impl ::core::clone::Clone for HdmiDisplayHdr2086Metadata {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HdmiDisplayHdr2086Metadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HdmiDisplayHdr2086Metadata")
            .field("RedPrimaryX", &self.RedPrimaryX)
            .field("RedPrimaryY", &self.RedPrimaryY)
            .field("GreenPrimaryX", &self.GreenPrimaryX)
            .field("GreenPrimaryY", &self.GreenPrimaryY)
            .field("BluePrimaryX", &self.BluePrimaryX)
            .field("BluePrimaryY", &self.BluePrimaryY)
            .field("WhitePointX", &self.WhitePointX)
            .field("WhitePointY", &self.WhitePointY)
            .field("MaxMasteringLuminance", &self.MaxMasteringLuminance)
            .field("MinMasteringLuminance", &self.MinMasteringLuminance)
            .field("MaxContentLightLevel", &self.MaxContentLightLevel)
            .field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayHdr2086Metadata {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayHdr2086Metadata {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayHdr2086Metadata {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HdmiDisplayHdr2086Metadata>()) == 0 }
    }
}
impl ::core::cmp::Eq for HdmiDisplayHdr2086Metadata {}
impl ::core::default::Default for HdmiDisplayHdr2086Metadata {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: Self = Self(0i32);
    pub const EotfSdr: Self = Self(1i32);
    pub const Eotf2084: Self = Self(2i32);
    pub const DolbyVisionLowLatency: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayHdrOption {}
impl ::core::clone::Clone for HdmiDisplayHdrOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayHdrOption {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayHdrOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayHdrOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayHdrOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayHdrOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayHdrOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayInformation(::windows::core::IUnknown);
impl HdmiDisplayInformation {
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedDisplayModes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedDisplayModes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn GetCurrentDisplayMode(&self) -> ::windows::core::Result<HdmiDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentDisplayMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDefaultDisplayModeAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetDefaultDisplayModeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetCurrentDisplayModeAsync<'a, Param0: ::windows::core::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSetCurrentDisplayModeAsync)(::core::mem::transmute_copy(this), mode.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetCurrentDisplayModeWithHdrAsync<'a, Param0: ::windows::core::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0, hdroption: HdmiDisplayHdrOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSetCurrentDisplayModeWithHdrAsync)(::core::mem::transmute_copy(this), mode.into_param().abi(), hdroption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync<'a, Param0: ::windows::core::IntoParam<'a, HdmiDisplayMode>, Param2: ::windows::core::IntoParam<'a, HdmiDisplayHdr2086Metadata>>(&self, mode: Param0, hdroption: HdmiDisplayHdrOption, hdrmetadata: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(::core::mem::transmute_copy(this), mode.into_param().abi(), hdroption, hdrmetadata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisplayModesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::core::IInspectable>>>(&self, value: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayModesChanged)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDisplayModesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDisplayModesChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<HdmiDisplayInformation> {
        Self::IHdmiDisplayInformationStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayInformation>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHdmiDisplayInformationStatics<R, F: FnOnce(&IHdmiDisplayInformationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HdmiDisplayInformation, IHdmiDisplayInformationStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HdmiDisplayInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdmiDisplayInformation {}
impl ::core::fmt::Debug for HdmiDisplayInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayInformation;{130b3c0a-f565-476e-abd5-ea05aee74c69})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_Vtbl;
    const IID: ::windows::core::GUID = <IHdmiDisplayInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HdmiDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayInformation";
}
impl ::core::convert::From<HdmiDisplayInformation> for ::windows::core::IUnknown {
    fn from(value: HdmiDisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayInformation> for ::windows::core::IUnknown {
    fn from(value: &HdmiDisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HdmiDisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HdmiDisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HdmiDisplayInformation> for ::windows::core::IInspectable {
    fn from(value: HdmiDisplayInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayInformation> for ::windows::core::IInspectable {
    fn from(value: &HdmiDisplayInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HdmiDisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HdmiDisplayInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HdmiDisplayInformation {}
unsafe impl ::core::marker::Sync for HdmiDisplayInformation {}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
pub struct HdmiDisplayMode(::windows::core::IUnknown);
impl HdmiDisplayMode {
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn ResolutionWidthInRawPixels(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResolutionWidthInRawPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn ResolutionHeightInRawPixels(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResolutionHeightInRawPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn RefreshRate(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RefreshRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn StereoEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StereoEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn BitsPerPixel(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BitsPerPixel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), mode.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn ColorSpace(&self) -> ::windows::core::Result<HdmiDisplayColorSpace> {
        let this = self;
        unsafe {
            let mut result__: HdmiDisplayColorSpace = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorSpace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayColorSpace>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn PixelEncoding(&self) -> ::windows::core::Result<HdmiDisplayPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__: HdmiDisplayPixelEncoding = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PixelEncoding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayPixelEncoding>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn IsSdrLuminanceSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSdrLuminanceSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn IsSmpte2084Supported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSmpte2084Supported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn Is2086MetadataSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Is2086MetadataSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
    pub fn IsDolbyVisionLowLatencySupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHdmiDisplayMode2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDolbyVisionLowLatencySupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HdmiDisplayMode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HdmiDisplayMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HdmiDisplayMode {}
impl ::core::fmt::Debug for HdmiDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayMode;{0c06d5ad-1b90-4f51-9981-ef5a1c0ddf66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_Vtbl;
    const IID: ::windows::core::GUID = <IHdmiDisplayMode as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HdmiDisplayMode {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayMode";
}
impl ::core::convert::From<HdmiDisplayMode> for ::windows::core::IUnknown {
    fn from(value: HdmiDisplayMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayMode> for ::windows::core::IUnknown {
    fn from(value: &HdmiDisplayMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HdmiDisplayMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HdmiDisplayMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HdmiDisplayMode> for ::windows::core::IInspectable {
    fn from(value: HdmiDisplayMode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HdmiDisplayMode> for ::windows::core::IInspectable {
    fn from(value: &HdmiDisplayMode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HdmiDisplayMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HdmiDisplayMode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for HdmiDisplayMode {}
unsafe impl ::core::marker::Sync for HdmiDisplayMode {}
#[doc = "*Required features: `\"Graphics_Display_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: Self = Self(0i32);
    pub const Ycc444: Self = Self(1i32);
    pub const Ycc422: Self = Self(2i32);
    pub const Ycc420: Self = Self(3i32);
}
impl ::core::marker::Copy for HdmiDisplayPixelEncoding {}
impl ::core::clone::Clone for HdmiDisplayPixelEncoding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HdmiDisplayPixelEncoding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HdmiDisplayPixelEncoding {
    type Abi = Self;
}
impl ::core::fmt::Debug for HdmiDisplayPixelEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HdmiDisplayPixelEncoding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HdmiDisplayPixelEncoding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x130b3c0a_f565_476e_abd5_ea05aee74c69);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedDisplayModes: usize,
    pub GetCurrentDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDefaultDisplayModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDefaultDisplayModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeWithHdrAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, hdroption: HdmiDisplayHdrOption, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeWithHdrAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestSetCurrentDisplayModeWithHdrAndMetadataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisplayModesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisplayModesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDisplayModesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDisplayModesChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayInformationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHdmiDisplayInformationStatics {
    type Vtable = IHdmiDisplayInformationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ce6b260_f42a_4a15_914c_7b8e2a5a65df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformationStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayMode(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0c06d5ad_1b90_4f51_9981_ef5a1c0ddf66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ResolutionWidthInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ResolutionHeightInRawPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub RefreshRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub StereoEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub BitsPerPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayColorSpace) -> ::windows::core::HRESULT,
    pub PixelEncoding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HdmiDisplayPixelEncoding) -> ::windows::core::HRESULT,
    pub IsSdrLuminanceSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSmpte2084Supported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Is2086MetadataSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHdmiDisplayMode2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHdmiDisplayMode2 {
    type Vtable = IHdmiDisplayMode2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x07cd4e9f_4b3c_42b8_84e7_895368718af2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsDolbyVisionLowLatencySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
