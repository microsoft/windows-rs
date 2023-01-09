impl ::core::default::Default for CapturedMetadataExposureCompensation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CapturedMetadataExposureCompensation {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for CapturedMetadataExposureCompensation {}
impl ::core::fmt::Debug for CapturedMetadataExposureCompensation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CapturedMetadataExposureCompensation").field("Flags", &self.Flags).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for CapturedMetadataISOGains {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CapturedMetadataISOGains {
    fn eq(&self, other: &Self) -> bool {
        self.AnalogGain == other.AnalogGain && self.DigitalGain == other.DigitalGain
    }
}
impl ::core::cmp::Eq for CapturedMetadataISOGains {}
impl ::core::fmt::Debug for CapturedMetadataISOGains {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CapturedMetadataISOGains").field("AnalogGain", &self.AnalogGain).field("DigitalGain", &self.DigitalGain).finish()
    }
}
impl ::core::default::Default for CapturedMetadataWhiteBalanceGains {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CapturedMetadataWhiteBalanceGains {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for CapturedMetadataWhiteBalanceGains {}
impl ::core::fmt::Debug for CapturedMetadataWhiteBalanceGains {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CapturedMetadataWhiteBalanceGains").field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::core::default::Default for FaceCharacterization {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FaceCharacterization {
    fn eq(&self, other: &Self) -> bool {
        self.BlinkScoreLeft == other.BlinkScoreLeft && self.BlinkScoreRight == other.BlinkScoreRight && self.FacialExpression == other.FacialExpression && self.FacialExpressionScore == other.FacialExpressionScore
    }
}
impl ::core::cmp::Eq for FaceCharacterization {}
impl ::core::fmt::Debug for FaceCharacterization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceCharacterization").field("BlinkScoreLeft", &self.BlinkScoreLeft).field("BlinkScoreRight", &self.BlinkScoreRight).field("FacialExpression", &self.FacialExpression).field("FacialExpressionScore", &self.FacialExpressionScore).finish()
    }
}
impl ::core::default::Default for FaceCharacterizationBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FaceCharacterizationBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for FaceCharacterizationBlobHeader {}
impl ::core::fmt::Debug for FaceCharacterizationBlobHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceCharacterizationBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FaceRectInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for FaceRectInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceRectInfo").field("Region", &self.Region).field("confidenceLevel", &self.confidenceLevel).finish()
    }
}
impl ::core::default::Default for FaceRectInfoBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FaceRectInfoBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for FaceRectInfoBlobHeader {}
impl ::core::fmt::Debug for FaceRectInfoBlobHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FaceRectInfoBlobHeader").field("Size", &self.Size).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for HistogramBlobHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HistogramBlobHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Histograms == other.Histograms
    }
}
impl ::core::cmp::Eq for HistogramBlobHeader {}
impl ::core::fmt::Debug for HistogramBlobHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramBlobHeader").field("Size", &self.Size).field("Histograms", &self.Histograms).finish()
    }
}
impl ::core::default::Default for HistogramDataHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HistogramDataHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ChannelMask == other.ChannelMask && self.Linear == other.Linear
    }
}
impl ::core::cmp::Eq for HistogramDataHeader {}
impl ::core::fmt::Debug for HistogramDataHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramDataHeader").field("Size", &self.Size).field("ChannelMask", &self.ChannelMask).field("Linear", &self.Linear).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HistogramGrid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for HistogramGrid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramGrid").field("Width", &self.Width).field("Height", &self.Height).field("Region", &self.Region).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HistogramHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::core::fmt::Debug for HistogramHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HistogramHeader").field("Size", &self.Size).field("Bins", &self.Bins).field("FourCC", &self.FourCC).field("ChannelMasks", &self.ChannelMasks).field("Grid", &self.Grid).finish()
    }
}
impl ::core::default::Default for MF_MEDIASOURCE_STATUS_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIASOURCE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIASOURCE_STATUS_INFO").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TRANSFER_VIDEO_FRAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TRANSFER_VIDEO_FRAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MetadataTimeStamps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MetadataTimeStamps {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Device == other.Device && self.Presentation == other.Presentation
    }
}
impl ::core::cmp::Eq for MetadataTimeStamps {}
impl ::core::fmt::Debug for MetadataTimeStamps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MetadataTimeStamps").field("Flags", &self.Flags).field("Device", &self.Device).field("Presentation", &self.Presentation).finish()
    }
}
