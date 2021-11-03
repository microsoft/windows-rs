#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Graphics_Display_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdmiDisplayColorSpace(pub i32);
impl HdmiDisplayColorSpace {
    pub const RgbLimited: HdmiDisplayColorSpace = HdmiDisplayColorSpace(0i32);
    pub const RgbFull: HdmiDisplayColorSpace = HdmiDisplayColorSpace(1i32);
    pub const BT2020: HdmiDisplayColorSpace = HdmiDisplayColorSpace(2i32);
    pub const BT709: HdmiDisplayColorSpace = HdmiDisplayColorSpace(3i32);
}
impl ::std::convert::From<i32> for HdmiDisplayColorSpace {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HdmiDisplayColorSpace {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdmiDisplayColorSpace {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayColorSpace;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_Display_Core`*"]
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
impl HdmiDisplayHdr2086Metadata {}
impl ::std::default::Default for HdmiDisplayHdr2086Metadata {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HdmiDisplayHdr2086Metadata {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HdmiDisplayHdr2086Metadata")
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
impl ::std::cmp::PartialEq for HdmiDisplayHdr2086Metadata {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimaryX == other.RedPrimaryX
            && self.RedPrimaryY == other.RedPrimaryY
            && self.GreenPrimaryX == other.GreenPrimaryX
            && self.GreenPrimaryY == other.GreenPrimaryY
            && self.BluePrimaryX == other.BluePrimaryX
            && self.BluePrimaryY == other.BluePrimaryY
            && self.WhitePointX == other.WhitePointX
            && self.WhitePointY == other.WhitePointY
            && self.MaxMasteringLuminance == other.MaxMasteringLuminance
            && self.MinMasteringLuminance == other.MinMasteringLuminance
            && self.MaxContentLightLevel == other.MaxContentLightLevel
            && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::std::cmp::Eq for HdmiDisplayHdr2086Metadata {}
unsafe impl ::windows::runtime::Abi for HdmiDisplayHdr2086Metadata {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdmiDisplayHdr2086Metadata {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Display.Core.HdmiDisplayHdr2086Metadata;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2;u2)");
}
#[doc = "*Required features: `Graphics_Display_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdmiDisplayHdrOption(pub i32);
impl HdmiDisplayHdrOption {
    pub const None: HdmiDisplayHdrOption = HdmiDisplayHdrOption(0i32);
    pub const EotfSdr: HdmiDisplayHdrOption = HdmiDisplayHdrOption(1i32);
    pub const Eotf2084: HdmiDisplayHdrOption = HdmiDisplayHdrOption(2i32);
    pub const DolbyVisionLowLatency: HdmiDisplayHdrOption = HdmiDisplayHdrOption(3i32);
}
impl ::std::convert::From<i32> for HdmiDisplayHdrOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HdmiDisplayHdrOption {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdmiDisplayHdrOption {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayHdrOption;i4)");
}
#[doc = "*Required features: `Graphics_Display_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HdmiDisplayInformation(::windows::runtime::IInspectable);
impl HdmiDisplayInformation {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation_Collections`*"]
    pub fn GetSupportedDisplayModes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn GetCurrentDisplayMode(&self) -> ::windows::runtime::Result<HdmiDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation`*"]
    pub fn SetDefaultDisplayModeAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation`*"]
    pub fn RequestSetCurrentDisplayModeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), mode.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation`*"]
    pub fn RequestSetCurrentDisplayModeWithHdrAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0, hdroption: HdmiDisplayHdrOption) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), mode.into_param().abi(), hdroption, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation`*"]
    pub fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HdmiDisplayMode>, Param2: ::windows::runtime::IntoParam<'a, HdmiDisplayHdr2086Metadata>>(&self, mode: Param0, hdroption: HdmiDisplayHdrOption, hdrmetadata: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), mode.into_param().abi(), hdroption, hdrmetadata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation`*"]
    pub fn DisplayModesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::runtime::IInspectable>>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Display_Core`, `Foundation`*"]
    pub fn RemoveDisplayModesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<HdmiDisplayInformation> {
        Self::IHdmiDisplayInformationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayInformation>(result__)
        })
    }
    pub fn IHdmiDisplayInformationStatics<R, F: FnOnce(&IHdmiDisplayInformationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HdmiDisplayInformation, IHdmiDisplayInformationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HdmiDisplayInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayInformation;{130b3c0a-f565-476e-abd5-ea05aee74c69})");
}
unsafe impl ::windows::runtime::Interface for HdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(319503370, 62821, 18286, [171, 213, 234, 5, 174, 231, 76, 105]);
}
impl ::windows::runtime::RuntimeName for HdmiDisplayInformation {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayInformation";
}
unsafe impl ::std::marker::Send for HdmiDisplayInformation {}
unsafe impl ::std::marker::Sync for HdmiDisplayInformation {}
#[doc = "*Required features: `Graphics_Display_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct HdmiDisplayMode(::windows::runtime::IInspectable);
impl HdmiDisplayMode {
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn ResolutionWidthInRawPixels(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn ResolutionHeightInRawPixels(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn RefreshRate(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn StereoEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn BitsPerPixel(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, HdmiDisplayMode>>(&self, mode: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), mode.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn ColorSpace(&self) -> ::windows::runtime::Result<HdmiDisplayColorSpace> {
        let this = self;
        unsafe {
            let mut result__: HdmiDisplayColorSpace = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayColorSpace>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn PixelEncoding(&self) -> ::windows::runtime::Result<HdmiDisplayPixelEncoding> {
        let this = self;
        unsafe {
            let mut result__: HdmiDisplayPixelEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HdmiDisplayPixelEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn IsSdrLuminanceSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn IsSmpte2084Supported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn Is2086MetadataSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Display_Core`*"]
    pub fn IsDolbyVisionLowLatencySupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHdmiDisplayMode2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HdmiDisplayMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Display.Core.HdmiDisplayMode;{0c06d5ad-1b90-4f51-9981-ef5a1c0ddf66})");
}
unsafe impl ::windows::runtime::Interface for HdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(201774509, 7056, 20305, [153, 129, 239, 90, 28, 13, 223, 102]);
}
impl ::windows::runtime::RuntimeName for HdmiDisplayMode {
    const NAME: &'static str = "Windows.Graphics.Display.Core.HdmiDisplayMode";
}
unsafe impl ::std::marker::Send for HdmiDisplayMode {}
unsafe impl ::std::marker::Sync for HdmiDisplayMode {}
#[doc = "*Required features: `Graphics_Display_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdmiDisplayPixelEncoding(pub i32);
impl HdmiDisplayPixelEncoding {
    pub const Rgb444: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(0i32);
    pub const Ycc444: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(1i32);
    pub const Ycc422: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(2i32);
    pub const Ycc420: HdmiDisplayPixelEncoding = HdmiDisplayPixelEncoding(3i32);
}
impl ::std::convert::From<i32> for HdmiDisplayPixelEncoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HdmiDisplayPixelEncoding {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdmiDisplayPixelEncoding {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Display.Core.HdmiDisplayPixelEncoding;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHdmiDisplayInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHdmiDisplayInformation {
    type Vtable = IHdmiDisplayInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(319503370, 62821, 18286, [171, 213, 234, 5, 174, 231, 76, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: ::windows::runtime::RawPtr, hdroption: HdmiDisplayHdrOption, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: ::windows::runtime::RawPtr, hdroption: HdmiDisplayHdrOption, hdrmetadata: HdmiDisplayHdr2086Metadata, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHdmiDisplayInformationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHdmiDisplayInformationStatics {
    type Vtable = IHdmiDisplayInformationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1827058272, 62506, 18965, [145, 76, 123, 142, 42, 90, 101, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayInformationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHdmiDisplayMode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHdmiDisplayMode {
    type Vtable = IHdmiDisplayMode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(201774509, 7056, 20305, [153, 129, 239, 90, 28, 13, 223, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HdmiDisplayColorSpace) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HdmiDisplayPixelEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHdmiDisplayMode2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHdmiDisplayMode2 {
    type Vtable = IHdmiDisplayMode2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(130895519, 19260, 17080, [132, 231, 137, 83, 104, 113, 138, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdmiDisplayMode2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
