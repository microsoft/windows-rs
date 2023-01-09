#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
unsafe impl ::windows::core::Abi for MF_MEDIASOURCE_STATUS_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Media_Streaming\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
unsafe impl ::windows::core::Abi for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    type Abi = Self;
}
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
unsafe impl ::windows::core::Abi for CapturedMetadataExposureCompensation {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for CapturedMetadataISOGains {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for CapturedMetadataWhiteBalanceGains {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for FaceCharacterization {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for FaceCharacterizationBlobHeader {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for FaceRectInfo {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for FaceRectInfoBlobHeader {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for HistogramBlobHeader {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for HistogramDataHeader {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for HistogramGrid {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for HistogramHeader {
    type Abi = Self;
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
unsafe impl ::windows::core::Abi for MetadataTimeStamps {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
