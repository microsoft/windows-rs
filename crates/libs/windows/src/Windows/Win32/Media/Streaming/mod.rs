#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
impl ::core::marker::Copy for CapturedMetadataExposureCompensation {}
impl ::core::clone::Clone for CapturedMetadataExposureCompensation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CapturedMetadataExposureCompensation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CapturedMetadataExposureCompensation").field("Flags", &self.Flags).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for CapturedMetadataExposureCompensation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CapturedMetadataExposureCompensation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CapturedMetadataExposureCompensation>()) == 0 }
    }
}
impl ::core::cmp::Eq for CapturedMetadataExposureCompensation {}
impl ::core::default::Default for CapturedMetadataExposureCompensation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
impl ::core::marker::Copy for CapturedMetadataISOGains {}
impl ::core::clone::Clone for CapturedMetadataISOGains {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CapturedMetadataISOGains {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CapturedMetadataISOGains").field("AnalogGain", &self.AnalogGain).field("DigitalGain", &self.DigitalGain).finish()
    }
}
unsafe impl ::windows::core::Abi for CapturedMetadataISOGains {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CapturedMetadataISOGains {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CapturedMetadataISOGains>()) == 0 }
    }
}
impl ::core::cmp::Eq for CapturedMetadataISOGains {}
impl ::core::default::Default for CapturedMetadataISOGains {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
impl ::core::marker::Copy for CapturedMetadataWhiteBalanceGains {}
impl ::core::clone::Clone for CapturedMetadataWhiteBalanceGains {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CapturedMetadataWhiteBalanceGains {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CapturedMetadataWhiteBalanceGains").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
unsafe impl ::windows::core::Abi for CapturedMetadataWhiteBalanceGains {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CapturedMetadataWhiteBalanceGains {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CapturedMetadataWhiteBalanceGains>()) == 0 }
    }
}
impl ::core::cmp::Eq for CapturedMetadataWhiteBalanceGains {}
impl ::core::default::Default for CapturedMetadataWhiteBalanceGains {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
impl ::core::marker::Copy for FaceCharacterization {}
impl ::core::clone::Clone for FaceCharacterization {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FaceCharacterization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceCharacterization").field("BlinkScoreLeft", &self.BlinkScoreLeft).field("BlinkScoreRight", &self.BlinkScoreRight).field("FacialExpression", &self.FacialExpression).field("FacialExpressionScore", &self.FacialExpressionScore).finish()
    }
}
unsafe impl ::windows::core::Abi for FaceCharacterization {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FaceCharacterization {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FaceCharacterization>()) == 0 }
    }
}
impl ::core::cmp::Eq for FaceCharacterization {}
impl ::core::default::Default for FaceCharacterization {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for FaceCharacterizationBlobHeader {}
impl ::core::clone::Clone for FaceCharacterizationBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FaceCharacterizationBlobHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceCharacterizationBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
unsafe impl ::windows::core::Abi for FaceCharacterizationBlobHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FaceCharacterizationBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FaceCharacterizationBlobHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for FaceCharacterizationBlobHeader {}
impl ::core::default::Default for FaceCharacterizationBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FaceRectInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FaceRectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceRectInfo").field("Region", &self.Region).field("confidenceLevel", &self.confidenceLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FaceRectInfo {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FaceRectInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FaceRectInfo>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FaceRectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FaceRectInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for FaceRectInfoBlobHeader {}
impl ::core::clone::Clone for FaceRectInfoBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FaceRectInfoBlobHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceRectInfoBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
unsafe impl ::windows::core::Abi for FaceRectInfoBlobHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FaceRectInfoBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FaceRectInfoBlobHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for FaceRectInfoBlobHeader {}
impl ::core::default::Default for FaceRectInfoBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
impl ::core::marker::Copy for HistogramBlobHeader {}
impl ::core::clone::Clone for HistogramBlobHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HistogramBlobHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramBlobHeader").field("Size", &self.Size).field("Histograms", &self.Histograms).finish()
    }
}
unsafe impl ::windows::core::Abi for HistogramBlobHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HistogramBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HistogramBlobHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for HistogramBlobHeader {}
impl ::core::default::Default for HistogramBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
impl ::core::marker::Copy for HistogramDataHeader {}
impl ::core::clone::Clone for HistogramDataHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HistogramDataHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramDataHeader").field("Size", &self.Size).field("ChannelMask", &self.ChannelMask).field("Linear", &self.Linear).finish()
    }
}
unsafe impl ::windows::core::Abi for HistogramDataHeader {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HistogramDataHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HistogramDataHeader>()) == 0 }
    }
}
impl ::core::cmp::Eq for HistogramDataHeader {}
impl ::core::default::Default for HistogramDataHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HistogramGrid {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HistogramGrid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramGrid").field("Width", &self.Width).field("Height", &self.Height).field("Region", &self.Region).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HistogramGrid {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HistogramGrid {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HistogramGrid>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HistogramGrid {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HistogramGrid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HistogramHeader {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HistogramHeader {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HistogramHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramHeader").field("Size", &self.Size).field("Bins", &self.Bins).field("FourCC", &self.FourCC).field("ChannelMasks", &self.ChannelMasks).field("Grid", &self.Grid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HistogramHeader {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HistogramHeader {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HistogramHeader>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HistogramHeader {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HistogramHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
#[repr(transparent)]
pub struct IMFDeviceTransform(::windows::core::IUnknown);
impl IMFDeviceTransform {
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn InitializeTransform<'a, Param0: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFAttributes>>(&self, pattributes: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeTransform)(::core::mem::transmute_copy(self), pattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputAvailableType(&self, dwinputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputAvailableType)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), ::core::mem::transmute(dwtypeindex), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputCurrentType(&self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputCurrentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputStreamAttributes(&self, dwinputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputStreamAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::IMFAttributes>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputAvailableType(&self, dwoutputstreamid: u32, dwtypeindex: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputAvailableType)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamid), ::core::mem::transmute(dwtypeindex), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputCurrentType(&self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFMediaType> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputCurrentType)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamid), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::IMFMediaType>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputStreamAttributes(&self, dwoutputstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::IMFAttributes> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputStreamAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputstreamid), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::IMFAttributes>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
    pub unsafe fn GetStreamCount(&self, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamCount)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcinputstreams), ::core::mem::transmute(pcoutputstreams)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
    pub unsafe fn GetStreamIDs(&self, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStreamIDs)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputidarraysize), ::core::mem::transmute(pdwinputstreamids), ::core::mem::transmute(dwoutputidarraysize), ::core::mem::transmute(pdwoutputstreamids)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessEvent<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFMediaEvent>>(&self, dwinputstreamid: u32, pevent: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), pevent.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessInput<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFSample>>(&self, dwinputstreamid: u32, psample: Param1, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinputstreamid), psample.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessMessage(&self, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessMessage)(::core::mem::transmute_copy(self), ::core::mem::transmute(emessage), ::core::mem::transmute(ulparam)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn ProcessOutput(&self, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ProcessOutput)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), ::core::mem::transmute(coutputbuffercount), ::core::mem::transmute(poutputsample), ::core::mem::transmute(pdwstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetInputStreamState<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFMediaType>>(&self, dwstreamid: u32, pmediatype: Param1, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInputStreamState)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), pmediatype.into_param().abi(), ::core::mem::transmute(value), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputStreamState(&self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState> {
        let mut result__: super::MediaFoundation::DeviceStreamState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetInputStreamState)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::DeviceStreamState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn SetOutputStreamState<'a, Param1: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFMediaType>>(&self, dwstreamid: u32, pmediatype: Param1, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOutputStreamState)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), pmediatype.into_param().abi(), ::core::mem::transmute(value), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetOutputStreamState(&self, dwstreamid: u32) -> ::windows::core::Result<super::MediaFoundation::DeviceStreamState> {
        let mut result__: super::MediaFoundation::DeviceStreamState = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputStreamState)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), ::core::mem::transmute(&mut result__)).from_abi::<super::MediaFoundation::DeviceStreamState>(result__)
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn GetInputStreamPreferredState(&self, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::core::option::Option<super::MediaFoundation::IMFMediaType>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputStreamPreferredState)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamid), ::core::mem::transmute(value), ::core::mem::transmute(ppmediatype)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
    pub unsafe fn FlushInputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushInputStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
    pub unsafe fn FlushOutputStream(&self, dwstreamindex: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlushOutputStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwstreamindex), ::core::mem::transmute(dwflags)).ok()
    }
}
impl ::core::convert::From<IMFDeviceTransform> for ::windows::core::IUnknown {
    fn from(value: IMFDeviceTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMFDeviceTransform> for ::windows::core::IUnknown {
    fn from(value: &IMFDeviceTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMFDeviceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMFDeviceTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMFDeviceTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMFDeviceTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDeviceTransform {}
impl ::core::fmt::Debug for IMFDeviceTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDeviceTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMFDeviceTransform {
    type Vtable = IMFDeviceTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd818fbd8_fc46_42f2_87ac_1ea2d1f9bf32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransform_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub InitializeTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    InitializeTransform: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetInputAvailableType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetInputAvailableType: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetInputCurrentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetInputCurrentType: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetInputStreamAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetInputStreamAttributes: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetOutputAvailableType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, dwtypeindex: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetOutputAvailableType: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetOutputCurrentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, pmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetOutputCurrentType: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetOutputStreamAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoutputstreamid: u32, ppattributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetOutputStreamAttributes: usize,
    pub GetStreamCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcinputstreams: *mut u32, pcoutputstreams: *mut u32) -> ::windows::core::HRESULT,
    pub GetStreamIDs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputidarraysize: u32, pdwinputstreamids: *mut u32, dwoutputidarraysize: u32, pdwoutputstreamids: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub ProcessEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, pevent: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    ProcessEvent: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub ProcessInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwinputstreamid: u32, psample: ::windows::core::RawPtr, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    ProcessInput: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub ProcessMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emessage: super::MediaFoundation::MFT_MESSAGE_TYPE, ulparam: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    ProcessMessage: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub ProcessOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32, coutputbuffercount: u32, poutputsample: *mut super::MediaFoundation::MFT_OUTPUT_DATA_BUFFER, pdwstatus: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    ProcessOutput: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub SetInputStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    SetInputStreamState: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetInputStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetInputStreamState: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub SetOutputStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamid: u32, pmediatype: ::windows::core::RawPtr, value: super::MediaFoundation::DeviceStreamState, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    SetOutputStreamState: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetOutputStreamState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetOutputStreamState: usize,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub GetInputStreamPreferredState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamid: u32, value: *mut super::MediaFoundation::DeviceStreamState, ppmediatype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    GetInputStreamPreferredState: usize,
    pub FlushInputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT,
    pub FlushOutputStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwstreamindex: u32, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
#[repr(transparent)]
pub struct IMFDeviceTransformCallback(::windows::core::IUnknown);
impl IMFDeviceTransformCallback {
    #[doc = "*Required features: `\"Win32_Media_Streaming\"`, `\"Win32_Media_MediaFoundation\"`*"]
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub unsafe fn OnBufferSent<'a, Param0: ::windows::core::IntoParam<'a, super::MediaFoundation::IMFAttributes>>(&self, pcallbackattributes: Param0, pinid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnBufferSent)(::core::mem::transmute_copy(self), pcallbackattributes.into_param().abi(), ::core::mem::transmute(pinid)).ok()
    }
}
impl ::core::convert::From<IMFDeviceTransformCallback> for ::windows::core::IUnknown {
    fn from(value: IMFDeviceTransformCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMFDeviceTransformCallback> for ::windows::core::IUnknown {
    fn from(value: &IMFDeviceTransformCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMFDeviceTransformCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMFDeviceTransformCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMFDeviceTransformCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMFDeviceTransformCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDeviceTransformCallback {}
impl ::core::fmt::Debug for IMFDeviceTransformCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDeviceTransformCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMFDeviceTransformCallback {
    type Vtable = IMFDeviceTransformCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d5cb646_29ec_41fb_8179_8c4c6d750811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDeviceTransformCallback_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Media_MediaFoundation")]
    pub OnBufferSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallbackattributes: ::windows::core::RawPtr, pinid: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Media_MediaFoundation"))]
    OnBufferSent: usize,
}
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MF_MEDIASOURCE_STATUS_INFO(pub i32);
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(0i32);
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(1i32);
impl ::core::marker::Copy for MF_MEDIASOURCE_STATUS_INFO {}
impl ::core::clone::Clone for MF_MEDIASOURCE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MF_MEDIASOURCE_STATUS_INFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MF_MEDIASOURCE_STATUS_INFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for MF_MEDIASOURCE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIASOURCE_STATUS_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(2i32);
impl ::core::marker::Copy for MF_TRANSFER_VIDEO_FRAME_FLAGS {}
impl ::core::clone::Clone for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TRANSFER_VIDEO_FRAME_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
impl ::core::marker::Copy for MetadataTimeStamps {}
impl ::core::clone::Clone for MetadataTimeStamps {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MetadataTimeStamps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MetadataTimeStamps").field("Flags", &self.Flags).field("Device", &self.Device).field("Presentation", &self.Presentation).finish()
    }
}
unsafe impl ::windows::core::Abi for MetadataTimeStamps {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MetadataTimeStamps {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MetadataTimeStamps>()) == 0 }
    }
}
impl ::core::cmp::Eq for MetadataTimeStamps {}
impl ::core::default::Default for MetadataTimeStamps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
