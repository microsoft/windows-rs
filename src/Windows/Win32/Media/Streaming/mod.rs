#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
impl CapturedMetadataExposureCompensation {}
impl ::std::default::Default for CapturedMetadataExposureCompensation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CapturedMetadataExposureCompensation {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CapturedMetadataExposureCompensation").field("Flags", &self.Flags).field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for CapturedMetadataExposureCompensation {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for CapturedMetadataExposureCompensation {}
unsafe impl ::windows::runtime::Abi for CapturedMetadataExposureCompensation {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
impl CapturedMetadataISOGains {}
impl ::std::default::Default for CapturedMetadataISOGains {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CapturedMetadataISOGains {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CapturedMetadataISOGains").field("AnalogGain", &self.AnalogGain).field("DigitalGain", &self.DigitalGain).finish()
    }
}
impl ::std::cmp::PartialEq for CapturedMetadataISOGains {
    fn eq(&self, other: &Self) -> bool {
        self.AnalogGain == other.AnalogGain && self.DigitalGain == other.DigitalGain
    }
}
impl ::std::cmp::Eq for CapturedMetadataISOGains {}
unsafe impl ::windows::runtime::Abi for CapturedMetadataISOGains {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
impl CapturedMetadataWhiteBalanceGains {}
impl ::std::default::Default for CapturedMetadataWhiteBalanceGains {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CapturedMetadataWhiteBalanceGains {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CapturedMetadataWhiteBalanceGains").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::std::cmp::PartialEq for CapturedMetadataWhiteBalanceGains {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::std::cmp::Eq for CapturedMetadataWhiteBalanceGains {}
unsafe impl ::windows::runtime::Abi for CapturedMetadataWhiteBalanceGains {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
impl FaceCharacterization {}
impl ::std::default::Default for FaceCharacterization {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FaceCharacterization {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FaceCharacterization").field("BlinkScoreLeft", &self.BlinkScoreLeft).field("BlinkScoreRight", &self.BlinkScoreRight).field("FacialExpression", &self.FacialExpression).field("FacialExpressionScore", &self.FacialExpressionScore).finish()
    }
}
impl ::std::cmp::PartialEq for FaceCharacterization {
    fn eq(&self, other: &Self) -> bool {
        self.BlinkScoreLeft == other.BlinkScoreLeft && self.BlinkScoreRight == other.BlinkScoreRight && self.FacialExpression == other.FacialExpression && self.FacialExpressionScore == other.FacialExpressionScore
    }
}
impl ::std::cmp::Eq for FaceCharacterization {}
unsafe impl ::windows::runtime::Abi for FaceCharacterization {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl FaceCharacterizationBlobHeader {}
impl ::std::default::Default for FaceCharacterizationBlobHeader {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FaceCharacterizationBlobHeader {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FaceCharacterizationBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::std::cmp::PartialEq for FaceCharacterizationBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::std::cmp::Eq for FaceCharacterizationBlobHeader {}
unsafe impl ::windows::runtime::Abi for FaceCharacterizationBlobHeader {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Foundation`*"]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FaceRectInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FaceRectInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FaceRectInfo").field("Region", &self.Region).field("confidenceLevel", &self.confidenceLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FaceRectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Region == other.Region && self.confidenceLevel == other.confidenceLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FaceRectInfo {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl FaceRectInfoBlobHeader {}
impl ::std::default::Default for FaceRectInfoBlobHeader {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FaceRectInfoBlobHeader {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FaceRectInfoBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::std::cmp::PartialEq for FaceRectInfoBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::std::cmp::Eq for FaceRectInfoBlobHeader {}
unsafe impl ::windows::runtime::Abi for FaceRectInfoBlobHeader {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
impl HistogramBlobHeader {}
impl ::std::default::Default for HistogramBlobHeader {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HistogramBlobHeader {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HistogramBlobHeader").field("Size", &self.Size).field("Histograms", &self.Histograms).finish()
    }
}
impl ::std::cmp::PartialEq for HistogramBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Histograms == other.Histograms
    }
}
impl ::std::cmp::Eq for HistogramBlobHeader {}
unsafe impl ::windows::runtime::Abi for HistogramBlobHeader {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
impl HistogramDataHeader {}
impl ::std::default::Default for HistogramDataHeader {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HistogramDataHeader {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HistogramDataHeader").field("Size", &self.Size).field("ChannelMask", &self.ChannelMask).field("Linear", &self.Linear).finish()
    }
}
impl ::std::cmp::PartialEq for HistogramDataHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ChannelMask == other.ChannelMask && self.Linear == other.Linear
    }
}
impl ::std::cmp::Eq for HistogramDataHeader {}
unsafe impl ::windows::runtime::Abi for HistogramDataHeader {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Foundation`*"]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HistogramGrid {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HistogramGrid {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HistogramGrid").field("Width", &self.Width).field("Height", &self.Height).field("Region", &self.Region).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HistogramGrid {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Region == other.Region
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HistogramGrid {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Foundation`*"]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
#[cfg(feature = "Win32_Foundation")]
impl HistogramHeader {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HistogramHeader {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HistogramHeader {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HistogramHeader").field("Size", &self.Size).field("Bins", &self.Bins).field("FourCC", &self.FourCC).field("ChannelMasks", &self.ChannelMasks).field("Grid", &self.Grid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HistogramHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Bins == other.Bins && self.FourCC == other.FourCC && self.ChannelMasks == other.ChannelMasks && self.Grid == other.Grid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HistogramHeader {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HistogramHeader {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Streaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IMFDeviceTransform(::windows::runtime::IUnknown);
impl IMFDeviceTransform {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn InitializeTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaFoundation::IMFAttributes>>(&self, pattributes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> ::windows::runtime::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputstreamid), ::std::mem::transmute(dwtypeindex), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: u32) -> ::windows::runtime::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> ::windows::runtime::Result<super::MediaFoundation::IMFAttributes> {
        let mut result__: <super::MediaFoundation::IMFAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFAttributes>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> ::windows::runtime::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputstreamid), ::std::mem::transmute(dwtypeindex), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> ::windows::runtime::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> ::windows::runtime::Result<super::MediaFoundation::IMFAttributes> {
        let mut result__: <super::MediaFoundation::IMFAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwoutputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_Media_Streaming`*"]
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcinputstreams), ::std::mem::transmute(pcoutputstreams)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Streaming`*"]
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputidarraysize), ::std::mem::transmute(pdwinputstreamids), ::std::mem::transmute(dwoutputidarraysize), ::std::mem::transmute(pdwoutputstreamids)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn ProcessEvent<'a, Param1: ::windows::runtime::IntoParam<'a, super::MediaFoundation::IMFMediaEvent>>(&self, dwinputstreamid: u32, pevent: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputstreamid), pevent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn ProcessInput<'a, Param1: ::windows::runtime::IntoParam<'a, super::MediaFoundation::IMFSample>>(&self, dwinputstreamid: u32, psample: Param1, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwinputstreamid), psample.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn ProcessMessage(&self, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(emessage), ::std::mem::transmute(ulparam)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags), ::std::mem::transmute(coutputbuffercount), ::std::mem::transmute(poutputsample), ::std::mem::transmute(pdwstatus)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn SetInputStreamState<'a, Param1: ::windows::runtime::IntoParam<'a, super::MediaFoundation::IMFMediaType>>(&self, dwstreamid: u32, pmediatype: Param1, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamid), pmediatype.into_param().abi(), ::std::mem::transmute(value), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetInputStreamState(&self, dwstreamid: u32) -> ::windows::runtime::Result<super::MediaFoundation::DeviceStreamState> {
        let mut result__: <super::MediaFoundation::DeviceStreamState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamid), &mut result__).from_abi::<super::MediaFoundation::DeviceStreamState>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn SetOutputStreamState<'a, Param1: ::windows::runtime::IntoParam<'a, super::MediaFoundation::IMFMediaType>>(&self, dwstreamid: u32, pmediatype: Param1, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamid), pmediatype.into_param().abi(), ::std::mem::transmute(value), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetOutputStreamState(&self, dwstreamid: u32) -> ::windows::runtime::Result<super::MediaFoundation::DeviceStreamState> {
        let mut result__: <super::MediaFoundation::DeviceStreamState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamid), &mut result__).from_abi::<super::MediaFoundation::DeviceStreamState>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn GetInputStreamPreferredState(&self, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::std::option::Option<super::MediaFoundation::IMFMediaType>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamid), ::std::mem::transmute(value), ::std::mem::transmute(ppmediatype)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Streaming`*"]
    pub unsafe fn FlushInputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamindex), ::std::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Media_Streaming`*"]
    pub unsafe fn FlushOutputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwstreamindex), ::std::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMFDeviceTransform {
    type Vtable = IMFDeviceTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3625515992, 64582, 17138, [135, 172, 30, 162, 209, 249, 191, 50]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamid: u32, pmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamid: u32, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamid: u32, pmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputstreamid: u32, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamid: u32, pevent: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinputstreamid: u32, psample: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut ::std::mem::ManuallyDrop<super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER>, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamid: u32, pmediatype: ::windows::runtime::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamid: u32, pmediatype: ::windows::runtime::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamindex: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwstreamindex: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Media_Streaming`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IMFDeviceTransformCallback(::windows::runtime::IUnknown);
impl IMFDeviceTransformCallback {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    #[doc = "*Required features: `Win32_Media_Streaming`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn OnBufferSent<'a, Param0: ::windows::runtime::IntoParam<'a, super::MediaFoundation::IMFAttributes>>(&self, pcallbackattributes: Param0, pinid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallbackattributes.into_param().abi(), ::std::mem::transmute(pinid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMFDeviceTransformCallback {
    type Vtable = IMFDeviceTransformCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1834792518, 10732, 16891, [129, 121, 140, 76, 109, 117, 8, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransformCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallbackattributes: ::windows::runtime::RawPtr, pinid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
);
#[doc = "*Required features: `Win32_Media_Streaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MF_MEDIASOURCE_STATUS_INFO(pub i32);
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(0i32);
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(1i32);
impl ::std::convert::From<i32> for MF_MEDIASOURCE_STATUS_INFO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MF_MEDIASOURCE_STATUS_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Media_Streaming`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(pub i32);
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(0i32);
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(1i32);
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(2i32);
impl ::std::convert::From<i32> for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Media_Streaming`*"]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
impl MetadataTimeStamps {}
impl ::std::default::Default for MetadataTimeStamps {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MetadataTimeStamps {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MetadataTimeStamps").field("Flags", &self.Flags).field("Device", &self.Device).field("Presentation", &self.Presentation).finish()
    }
}
impl ::std::cmp::PartialEq for MetadataTimeStamps {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Device == other.Device && self.Presentation == other.Presentation
    }
}
impl ::std::cmp::Eq for MetadataTimeStamps {}
unsafe impl ::windows::runtime::Abi for MetadataTimeStamps {
    type Abi = Self;
}
