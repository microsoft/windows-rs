#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
impl CapturedMetadataExposureCompensation {}
impl ::core::default::Default for CapturedMetadataExposureCompensation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CapturedMetadataExposureCompensation {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CapturedMetadataExposureCompensation").field("Flags", &self.Flags).field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for CapturedMetadataExposureCompensation {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CapturedMetadataExposureCompensation {}
unsafe impl ::windows::core::Abi for CapturedMetadataExposureCompensation {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
impl CapturedMetadataISOGains {}
impl ::core::default::Default for CapturedMetadataISOGains {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CapturedMetadataISOGains {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CapturedMetadataISOGains").field("AnalogGain", &self.AnalogGain).field("DigitalGain", &self.DigitalGain).finish()
    }
}
impl ::core::cmp::PartialEq for CapturedMetadataISOGains {
    fn eq(&self, other: &Self) -> bool {
        self.AnalogGain == other.AnalogGain && self.DigitalGain == other.DigitalGain
    }
}
impl ::core::cmp::Eq for CapturedMetadataISOGains {}
unsafe impl ::windows::core::Abi for CapturedMetadataISOGains {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
impl CapturedMetadataWhiteBalanceGains {}
impl ::core::default::Default for CapturedMetadataWhiteBalanceGains {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CapturedMetadataWhiteBalanceGains {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CapturedMetadataWhiteBalanceGains").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::core::cmp::PartialEq for CapturedMetadataWhiteBalanceGains {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for CapturedMetadataWhiteBalanceGains {}
unsafe impl ::windows::core::Abi for CapturedMetadataWhiteBalanceGains {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
impl FaceCharacterization {}
impl ::core::default::Default for FaceCharacterization {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FaceCharacterization {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FaceCharacterization").field("BlinkScoreLeft", &self.BlinkScoreLeft).field("BlinkScoreRight", &self.BlinkScoreRight).field("FacialExpression", &self.FacialExpression).field("FacialExpressionScore", &self.FacialExpressionScore).finish()
    }
}
impl ::core::cmp::PartialEq for FaceCharacterization {
    fn eq(&self, other: &Self) -> bool {
        self.BlinkScoreLeft == other.BlinkScoreLeft && self.BlinkScoreRight == other.BlinkScoreRight && self.FacialExpression == other.FacialExpression && self.FacialExpressionScore == other.FacialExpressionScore
    }
}
impl ::core::cmp::Eq for FaceCharacterization {}
unsafe impl ::windows::core::Abi for FaceCharacterization {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl FaceCharacterizationBlobHeader {}
impl ::core::default::Default for FaceCharacterizationBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FaceCharacterizationBlobHeader {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FaceCharacterizationBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::cmp::PartialEq for FaceCharacterizationBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for FaceCharacterizationBlobHeader {}
unsafe impl ::windows::core::Abi for FaceCharacterizationBlobHeader {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FaceRectInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FaceRectInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FaceRectInfo").field("Region", &self.Region).field("confidenceLevel", &self.confidenceLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FaceRectInfo {
    fn eq(&self, other: &Self) -> bool {
        self.Region == other.Region && self.confidenceLevel == other.confidenceLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FaceRectInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl FaceRectInfoBlobHeader {}
impl ::core::default::Default for FaceRectInfoBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for FaceRectInfoBlobHeader {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("FaceRectInfoBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::cmp::PartialEq for FaceRectInfoBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for FaceRectInfoBlobHeader {}
unsafe impl ::windows::core::Abi for FaceRectInfoBlobHeader {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
impl HistogramBlobHeader {}
impl ::core::default::Default for HistogramBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HistogramBlobHeader {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HistogramBlobHeader").field("Size", &self.Size).field("Histograms", &self.Histograms).finish()
    }
}
impl ::core::cmp::PartialEq for HistogramBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Histograms == other.Histograms
    }
}
impl ::core::cmp::Eq for HistogramBlobHeader {}
unsafe impl ::windows::core::Abi for HistogramBlobHeader {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
impl HistogramDataHeader {}
impl ::core::default::Default for HistogramDataHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for HistogramDataHeader {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HistogramDataHeader").field("Size", &self.Size).field("ChannelMask", &self.ChannelMask).field("Linear", &self.Linear).finish()
    }
}
impl ::core::cmp::PartialEq for HistogramDataHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ChannelMask == other.ChannelMask && self.Linear == other.Linear
    }
}
impl ::core::cmp::Eq for HistogramDataHeader {}
unsafe impl ::windows::core::Abi for HistogramDataHeader {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HistogramGrid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HistogramGrid {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HistogramGrid").field("Width", &self.Width).field("Height", &self.Height).field("Region", &self.Region).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HistogramGrid {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Region == other.Region
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HistogramGrid {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::default::Default for HistogramHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HistogramHeader {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HistogramHeader").field("Size", &self.Size).field("Bins", &self.Bins).field("FourCC", &self.FourCC).field("ChannelMasks", &self.ChannelMasks).field("Grid", &self.Grid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HistogramHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Bins == other.Bins && self.FourCC == other.FourCC && self.ChannelMasks == other.ChannelMasks && self.Grid == other.Grid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HistogramHeader {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HistogramHeader {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMFDeviceTransform(pub ::windows::core::IUnknown);
impl IMFDeviceTransform {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn InitializeTransform<'a, Param0: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFAttributes>>(&self, pattributes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pattributes.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), ::core::mem::transmute(dwtypeindex), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes> {
        let mut result__: <super::MediaFoundation::IMFAttributes as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFAttributes>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamid), ::core::mem::transmute(dwtypeindex), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: <super::MediaFoundation::IMFMediaType as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes> {
        let mut result__: <super::MediaFoundation::IMFAttributes as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamid), &mut result__).from_abi::<super::MediaFoundation::IMFAttributes>(result__)
    }
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcinputstreams), ::core::mem::transmute(pcoutputstreams)).ok()
    }
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputidarraysize), ::core::mem::transmute(pdwinputstreamids), ::core::mem::transmute(dwoutputidarraysize), ::core::mem::transmute(pdwoutputstreamids)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessEvent<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFMediaEvent>>(&self, dwinputstreamid: u32, pevent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), pevent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessInput<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFSample>>(&self, dwinputstreamid: u32, psample: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), psample.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessMessage(&self, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(emessage), ::core::mem::transmute(ulparam)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(coutputbuffercount), ::core::mem::transmute(poutputsample), ::core::mem::transmute(pdwstatus)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetInputStreamState<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFMediaType>>(&self, dwstreamid: u32, pmediatype: Param1, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), pmediatype.into_param().abi(), ::core::mem::transmute(value), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputStreamState(&self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState> {
        let mut result__: <super::MediaFoundation::DeviceStreamState as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), &mut result__).from_abi::<super::MediaFoundation::DeviceStreamState>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetOutputStreamState<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFMediaType>>(&self, dwstreamid: u32, pmediatype: Param1, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), pmediatype.into_param().abi(), ::core::mem::transmute(value), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputStreamState(&self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState> {
        let mut result__: <super::MediaFoundation::DeviceStreamState as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), &mut result__).from_abi::<super::MediaFoundation::DeviceStreamState>(result__)
    }
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputStreamPreferredState(&self, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::core::option::Option<super::MediaFoundation::IMFMediaType>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), ::core::mem::transmute(value), ::core::mem::transmute(ppmediatype)).ok()
    }
    pub unsafe fn FlushInputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn FlushOutputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMFDeviceTransform {
    type Vtable = IMFDeviceTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd818fbd8_fc46_42f2_87ac_1ea2d1f9bf32);
}
impl ::core::convert::From<IMFDeviceTransform> for ::windows::core::IUnknown {
    fn from(value: IMFDeviceTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMFDeviceTransform> for ::windows::core::IUnknown {
    fn from(value: &IMFDeviceTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMFDeviceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMFDeviceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwoutputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut ::core::mem::ManuallyDrop<super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER>, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMFDeviceTransformCallback(pub ::windows::core::IUnknown);
impl IMFDeviceTransformCallback {
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn OnBufferSent<'a, Param0: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFAttributes>>(&self, pcallbackattributes: Param0, pinid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallbackattributes.into_param().abi(), ::core::mem::transmute(pinid)).ok()
    }
}
unsafe impl ::windows::core::Interface for IMFDeviceTransformCallback {
    type Vtable = IMFDeviceTransformCallback_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5cb646_29ec_41fb_8179_8c4c6d750811);
}
impl ::core::convert::From<IMFDeviceTransformCallback> for ::windows::core::IUnknown {
    fn from(value: IMFDeviceTransformCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMFDeviceTransformCallback> for ::windows::core::IUnknown {
    fn from(value: &IMFDeviceTransformCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMFDeviceTransformCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMFDeviceTransformCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransformCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Media_MediaFoundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcallbackattributes: ::windows::core::RawPtr, pinid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))] usize,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MF_MEDIASOURCE_STATUS_INFO(pub i32);
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(0i32);
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(1i32);
impl ::core::convert::From<i32> for MF_MEDIASOURCE_STATUS_INFO {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MF_MEDIASOURCE_STATUS_INFO {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(pub i32);
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(0i32);
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(1i32);
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(2i32);
impl ::core::convert::From<i32> for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
impl MetadataTimeStamps {}
impl ::core::default::Default for MetadataTimeStamps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for MetadataTimeStamps {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("MetadataTimeStamps").field("Flags", &self.Flags).field("Device", &self.Device).field("Presentation", &self.Presentation).finish()
    }
}
impl ::core::cmp::PartialEq for MetadataTimeStamps {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Device == other.Device && self.Presentation == other.Presentation
    }
}
impl ::core::cmp::Eq for MetadataTimeStamps {}
unsafe impl ::windows::core::Abi for MetadataTimeStamps {
    type Abi = Self;
}
