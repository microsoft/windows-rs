impl ::core::default::Default for AEC_INPUT_STREAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AEC_INPUT_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AEC_INPUT_STREAM").field(&self.0).finish()
    }
}
impl ::core::default::Default for AEC_SYSTEM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AEC_SYSTEM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AEC_SYSTEM_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AEC_VAD_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AEC_VAD_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AEC_VAD_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AM_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
impl ::core::default::Default for ASF_FLAT_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ASF_FLAT_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ASF_INDEX_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ASF_INDEX_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier && self.cPerEntryBytes == other.cPerEntryBytes && self.szDescription == other.szDescription && self.dwInterval == other.dwInterval
    }
}
impl ::core::cmp::Eq for ASF_INDEX_DESCRIPTOR {}
impl ::core::fmt::Debug for ASF_INDEX_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASF_INDEX_DESCRIPTOR").field("Identifier", &self.Identifier).field("cPerEntryBytes", &self.cPerEntryBytes).field("szDescription", &self.szDescription).field("dwInterval", &self.dwInterval).finish()
    }
}
impl ::core::default::Default for ASF_INDEX_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ASF_INDEX_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.guidIndexType == other.guidIndexType && self.wStreamNumber == other.wStreamNumber
    }
}
impl ::core::cmp::Eq for ASF_INDEX_IDENTIFIER {}
impl ::core::fmt::Debug for ASF_INDEX_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASF_INDEX_IDENTIFIER").field("guidIndexType", &self.guidIndexType).field("wStreamNumber", &self.wStreamNumber).finish()
    }
}
impl ::core::default::Default for ASF_MUX_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ASF_MUX_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.cFramesWritten == other.cFramesWritten && self.cFramesDropped == other.cFramesDropped
    }
}
impl ::core::cmp::Eq for ASF_MUX_STATISTICS {}
impl ::core::fmt::Debug for ASF_MUX_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASF_MUX_STATISTICS").field("cFramesWritten", &self.cFramesWritten).field("cFramesDropped", &self.cFramesDropped).finish()
    }
}
impl ::core::default::Default for ASF_SELECTION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASF_SELECTION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASF_SELECTION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for ASF_STATUSFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ASF_STATUSFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ASF_STATUSFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for AecQualityMetrics_Struct {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AecQualityMetrics_Struct {
    fn eq(&self, other: &Self) -> bool {
        self.i64Timestamp == other.i64Timestamp
            && self.ConvergenceFlag == other.ConvergenceFlag
            && self.MicClippedFlag == other.MicClippedFlag
            && self.MicSilenceFlag == other.MicSilenceFlag
            && self.PstvFeadbackFlag == other.PstvFeadbackFlag
            && self.SpkClippedFlag == other.SpkClippedFlag
            && self.SpkMuteFlag == other.SpkMuteFlag
            && self.GlitchFlag == other.GlitchFlag
            && self.DoubleTalkFlag == other.DoubleTalkFlag
            && self.uGlitchCount == other.uGlitchCount
            && self.uMicClipCount == other.uMicClipCount
            && self.fDuration == other.fDuration
            && self.fTSVariance == other.fTSVariance
            && self.fTSDriftRate == other.fTSDriftRate
            && self.fVoiceLevel == other.fVoiceLevel
            && self.fNoiseLevel == other.fNoiseLevel
            && self.fERLE == other.fERLE
            && self.fAvgERLE == other.fAvgERLE
            && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for AecQualityMetrics_Struct {}
impl ::core::fmt::Debug for AecQualityMetrics_Struct {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AecQualityMetrics_Struct")
            .field("i64Timestamp", &self.i64Timestamp)
            .field("ConvergenceFlag", &self.ConvergenceFlag)
            .field("MicClippedFlag", &self.MicClippedFlag)
            .field("MicSilenceFlag", &self.MicSilenceFlag)
            .field("PstvFeadbackFlag", &self.PstvFeadbackFlag)
            .field("SpkClippedFlag", &self.SpkClippedFlag)
            .field("SpkMuteFlag", &self.SpkMuteFlag)
            .field("GlitchFlag", &self.GlitchFlag)
            .field("DoubleTalkFlag", &self.DoubleTalkFlag)
            .field("uGlitchCount", &self.uGlitchCount)
            .field("uMicClipCount", &self.uMicClipCount)
            .field("fDuration", &self.fDuration)
            .field("fTSVariance", &self.fTSVariance)
            .field("fTSDriftRate", &self.fTSDriftRate)
            .field("fVoiceLevel", &self.fVoiceLevel)
            .field("fNoiseLevel", &self.fNoiseLevel)
            .field("fERLE", &self.fERLE)
            .field("fAvgERLE", &self.fAvgERLE)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::default::Default for CodecAPIEventData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CodecAPIEventData {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.dataLength == other.dataLength && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for CodecAPIEventData {}
impl ::core::fmt::Debug for CodecAPIEventData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CodecAPIEventData").field("guid", &self.guid).field("dataLength", &self.dataLength).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for D3D12_BITSTREAM_ENCRYPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_BITSTREAM_ENCRYPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_BITSTREAM_ENCRYPTION_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    fn eq(&self, other: &Self) -> bool {
        self.IOCoherent == other.IOCoherent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_ARCHITECTURE").field("IOCoherent", &self.IOCoherent).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.VideoDecoderHeapDesc == other.VideoDecoderHeapDesc && self.MemoryPoolL0Size == other.MemoryPoolL0Size && self.MemoryPoolL1Size == other.MemoryPoolL1Size
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE").field("VideoDecoderHeapDesc", &self.VideoDecoderHeapDesc).field("MemoryPoolL0Size", &self.MemoryPoolL0Size).field("MemoryPoolL1Size", &self.MemoryPoolL1Size).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    fn eq(&self, other: &Self) -> bool {
        self.VideoDecoderHeapDesc == other.VideoDecoderHeapDesc && self.Protected == other.Protected && self.MemoryPoolL0Size == other.MemoryPoolL0Size && self.MemoryPoolL1Size == other.MemoryPoolL1Size
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODER_HEAP_SIZE1").field("VideoDecoderHeapDesc", &self.VideoDecoderHeapDesc).field("Protected", &self.Protected).field("MemoryPoolL0Size", &self.MemoryPoolL0Size).field("MemoryPoolL1Size", &self.MemoryPoolL1Size).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Configuration == other.Configuration && self.DecodeSample == other.DecodeSample && self.OutputFormat == other.OutputFormat && self.FrameRate == other.FrameRate && self.BitRate == other.BitRate && self.SupportFlags == other.SupportFlags && self.ScaleSupport == other.ScaleSupport
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_CONVERSION_SUPPORT").field("NodeIndex", &self.NodeIndex).field("Configuration", &self.Configuration).field("DecodeSample", &self.DecodeSample).field("OutputFormat", &self.OutputFormat).field("FrameRate", &self.FrameRate).field("BitRate", &self.BitRate).field("SupportFlags", &self.SupportFlags).field("ScaleSupport", &self.ScaleSupport).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Configuration == other.Configuration && self.FormatCount == other.FormatCount && self.pOutputFormats == other.pOutputFormats
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_FORMATS").field("NodeIndex", &self.NodeIndex).field("Configuration", &self.Configuration).field("FormatCount", &self.FormatCount).field("pOutputFormats", &self.pOutputFormats).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Configuration == other.Configuration && self.FormatCount == other.FormatCount
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_FORMAT_COUNT").field("NodeIndex", &self.NodeIndex).field("Configuration", &self.Configuration).field("FormatCount", &self.FormatCount).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.DecodeProfile == other.DecodeProfile && self.Width == other.Width && self.Height == other.Height && self.DecodeFormat == other.DecodeFormat && self.Components == other.Components && self.BinCount == other.BinCount && self.CounterBitDepth == other.CounterBitDepth
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_HISTOGRAM").field("NodeIndex", &self.NodeIndex).field("DecodeProfile", &self.DecodeProfile).field("Width", &self.Width).field("Height", &self.Height).field("DecodeFormat", &self.DecodeFormat).field("Components", &self.Components).field("BinCount", &self.BinCount).field("CounterBitDepth", &self.CounterBitDepth).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.ProfileCount == other.ProfileCount && self.pProfiles == other.pProfiles
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILES").field("NodeIndex", &self.NodeIndex).field("ProfileCount", &self.ProfileCount).field("pProfiles", &self.pProfiles).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.ProfileCount == other.ProfileCount
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_PROFILE_COUNT").field("NodeIndex", &self.NodeIndex).field("ProfileCount", &self.ProfileCount).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Configuration == other.Configuration && self.SupportFlags == other.SupportFlags
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_PROTECTED_RESOURCES").field("NodeIndex", &self.NodeIndex).field("Configuration", &self.Configuration).field("SupportFlags", &self.SupportFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Configuration == other.Configuration && self.Width == other.Width && self.Height == other.Height && self.DecodeFormat == other.DecodeFormat && self.FrameRate == other.FrameRate && self.BitRate == other.BitRate && self.SupportFlags == other.SupportFlags && self.ConfigurationFlags == other.ConfigurationFlags && self.DecodeTier == other.DecodeTier
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_DECODE_SUPPORT").field("NodeIndex", &self.NodeIndex).field("Configuration", &self.Configuration).field("Width", &self.Width).field("Height", &self.Height).field("DecodeFormat", &self.DecodeFormat).field("FrameRate", &self.FrameRate).field("BitRate", &self.BitRate).field("SupportFlags", &self.SupportFlags).field("ConfigurationFlags", &self.ConfigurationFlags).field("DecodeTier", &self.DecodeTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Codec == other.Codec && self.IsSupported == other.IsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC").field("NodeIndex", &self.NodeIndex).field("Codec", &self.Codec).field("IsSupported", &self.IsSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_HEAP_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_INPUT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Codec == other.Codec && self.ResolutionRatiosCount == other.ResolutionRatiosCount && self.IsSupported == other.IsSupported && self.MinResolutionSupported == other.MinResolutionSupported && self.MaxResolutionSupported == other.MaxResolutionSupported && self.ResolutionWidthMultipleRequirement == other.ResolutionWidthMultipleRequirement && self.ResolutionHeightMultipleRequirement == other.ResolutionHeightMultipleRequirement && self.pResolutionRatios == other.pResolutionRatios
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION")
            .field("NodeIndex", &self.NodeIndex)
            .field("Codec", &self.Codec)
            .field("ResolutionRatiosCount", &self.ResolutionRatiosCount)
            .field("IsSupported", &self.IsSupported)
            .field("MinResolutionSupported", &self.MinResolutionSupported)
            .field("MaxResolutionSupported", &self.MaxResolutionSupported)
            .field("ResolutionWidthMultipleRequirement", &self.ResolutionWidthMultipleRequirement)
            .field("ResolutionHeightMultipleRequirement", &self.ResolutionHeightMultipleRequirement)
            .field("pResolutionRatios", &self.pResolutionRatios)
            .finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Codec == other.Codec && self.ResolutionRatiosCount == other.ResolutionRatiosCount
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_ENCODER_OUTPUT_RESOLUTION_RATIOS_COUNT").field("NodeIndex", &self.NodeIndex).field("Codec", &self.Codec).field("ResolutionRatiosCount", &self.ResolutionRatiosCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_PROFILE_LEVEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Codec == other.Codec && self.RateControlMode == other.RateControlMode && self.IsSupported == other.IsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_ENCODER_RATE_CONTROL_MODE").field("NodeIndex", &self.NodeIndex).field("Codec", &self.Codec).field("RateControlMode", &self.RateControlMode).field("IsSupported", &self.IsSupported).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    fn eq(&self, other: &Self) -> bool {
        self.MaxSubregionsNumber == other.MaxSubregionsNumber && self.MaxIntraRefreshFrameDuration == other.MaxIntraRefreshFrameDuration && self.SubregionBlockPixelsSize == other.SubregionBlockPixelsSize && self.QPMapRegionPixelsSize == other.QPMapRegionPixelsSize
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOLUTION_SUPPORT_LIMITS").field("MaxSubregionsNumber", &self.MaxSubregionsNumber).field("MaxIntraRefreshFrameDuration", &self.MaxIntraRefreshFrameDuration).field("SubregionBlockPixelsSize", &self.SubregionBlockPixelsSize).field("QPMapRegionPixelsSize", &self.QPMapRegionPixelsSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_RESOURCE_REQUIREMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_ENCODER_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.CommandCount == other.CommandCount && self.pCommandInfos == other.pCommandInfos
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMANDS").field("NodeIndex", &self.NodeIndex).field("CommandCount", &self.CommandCount).field("pCommandInfos", &self.pCommandInfos).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.CommandCount == other.CommandCount
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_COUNT").field("NodeIndex", &self.NodeIndex).field("CommandCount", &self.CommandCount).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.CommandId == other.CommandId && self.Stage == other.Stage && self.ParameterCount == other.ParameterCount && self.pParameterInfos == other.pParameterInfos
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETERS").field("CommandId", &self.CommandId).field("Stage", &self.Stage).field("ParameterCount", &self.ParameterCount).field("pParameterInfos", &self.pParameterInfos).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.CommandId == other.CommandId && self.Stage == other.Stage && self.ParameterCount == other.ParameterCount && self.ParameterPacking == other.ParameterPacking
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_PARAMETER_COUNT").field("CommandId", &self.CommandId).field("Stage", &self.Stage).field("ParameterCount", &self.ParameterCount).field("ParameterPacking", &self.ParameterPacking).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.CommandId == other.CommandId && self.pCreationParameters == other.pCreationParameters && self.CreationParametersSizeInBytes == other.CreationParametersSizeInBytes && self.MemoryPoolL0Size == other.MemoryPoolL0Size && self.MemoryPoolL1Size == other.MemoryPoolL1Size
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SIZE").field("NodeIndex", &self.NodeIndex).field("CommandId", &self.CommandId).field("pCreationParameters", &self.pCreationParameters).field("CreationParametersSizeInBytes", &self.CreationParametersSizeInBytes).field("MemoryPoolL0Size", &self.MemoryPoolL0Size).field("MemoryPoolL1Size", &self.MemoryPoolL1Size).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.CommandId == other.CommandId && self.pInputData == other.pInputData && self.InputDataSizeInBytes == other.InputDataSizeInBytes && self.pOutputData == other.pOutputData && self.OutputDataSizeInBytes == other.OutputDataSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_EXTENSION_COMMAND_SUPPORT").field("NodeIndex", &self.NodeIndex).field("CommandId", &self.CommandId).field("pInputData", &self.pInputData).field("InputDataSizeInBytes", &self.InputDataSizeInBytes).field("pOutputData", &self.pOutputData).field("OutputDataSizeInBytes", &self.OutputDataSizeInBytes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.VideoDecodeSupport == other.VideoDecodeSupport && self.VideoProcessSupport == other.VideoProcessSupport && self.VideoEncodeSupport == other.VideoEncodeSupport
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_FEATURE_AREA_SUPPORT").field("NodeIndex", &self.NodeIndex).field("VideoDecodeSupport", &self.VideoDecodeSupport).field("VideoProcessSupport", &self.VideoProcessSupport).field("VideoEncodeSupport", &self.VideoEncodeSupport).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.InputFormat == other.InputFormat && self.BlockSizeFlags == other.BlockSizeFlags && self.PrecisionFlags == other.PrecisionFlags && self.SizeRange == other.SizeRange
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR").field("NodeIndex", &self.NodeIndex).field("InputFormat", &self.InputFormat).field("BlockSizeFlags", &self.BlockSizeFlags).field("PrecisionFlags", &self.PrecisionFlags).field("SizeRange", &self.SizeRange).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.SupportFlags == other.SupportFlags
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_PROTECTED_RESOURCES").field("NodeIndex", &self.NodeIndex).field("SupportFlags", &self.SupportFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.InputFormat == other.InputFormat && self.BlockSize == other.BlockSize && self.Precision == other.Precision && self.SizeRange == other.SizeRange && self.Protected == other.Protected && self.MotionVectorHeapMemoryPoolL0Size == other.MotionVectorHeapMemoryPoolL0Size && self.MotionVectorHeapMemoryPoolL1Size == other.MotionVectorHeapMemoryPoolL1Size && self.MotionEstimatorMemoryPoolL0Size == other.MotionEstimatorMemoryPoolL0Size && self.MotionEstimatorMemoryPoolL1Size == other.MotionEstimatorMemoryPoolL1Size
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_MOTION_ESTIMATOR_SIZE")
            .field("NodeIndex", &self.NodeIndex)
            .field("InputFormat", &self.InputFormat)
            .field("BlockSize", &self.BlockSize)
            .field("Precision", &self.Precision)
            .field("SizeRange", &self.SizeRange)
            .field("Protected", &self.Protected)
            .field("MotionVectorHeapMemoryPoolL0Size", &self.MotionVectorHeapMemoryPoolL0Size)
            .field("MotionVectorHeapMemoryPoolL1Size", &self.MotionVectorHeapMemoryPoolL1Size)
            .field("MotionEstimatorMemoryPoolL0Size", &self.MotionEstimatorMemoryPoolL0Size)
            .field("MotionEstimatorMemoryPoolL1Size", &self.MotionEstimatorMemoryPoolL1Size)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.pOutputStreamDesc == other.pOutputStreamDesc && self.NumInputStreamDescs == other.NumInputStreamDescs && self.pInputStreamDescs == other.pInputStreamDescs && self.MemoryPoolL0Size == other.MemoryPoolL0Size && self.MemoryPoolL1Size == other.MemoryPoolL1Size
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE").field("NodeMask", &self.NodeMask).field("pOutputStreamDesc", &self.pOutputStreamDesc).field("NumInputStreamDescs", &self.NumInputStreamDescs).field("pInputStreamDescs", &self.pInputStreamDescs).field("MemoryPoolL0Size", &self.MemoryPoolL0Size).field("MemoryPoolL1Size", &self.MemoryPoolL1Size).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.pOutputStreamDesc == other.pOutputStreamDesc && self.NumInputStreamDescs == other.NumInputStreamDescs && self.pInputStreamDescs == other.pInputStreamDescs && self.Protected == other.Protected && self.MemoryPoolL0Size == other.MemoryPoolL0Size && self.MemoryPoolL1Size == other.MemoryPoolL1Size
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_PROCESSOR_SIZE1").field("NodeMask", &self.NodeMask).field("pOutputStreamDesc", &self.pOutputStreamDesc).field("NumInputStreamDescs", &self.NumInputStreamDescs).field("pInputStreamDescs", &self.pInputStreamDescs).field("Protected", &self.Protected).field("MemoryPoolL0Size", &self.MemoryPoolL0Size).field("MemoryPoolL1Size", &self.MemoryPoolL1Size).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.MaxInputStreams == other.MaxInputStreams
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_PROCESS_MAX_INPUT_STREAMS").field("NodeIndex", &self.NodeIndex).field("MaxInputStreams", &self.MaxInputStreams).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.SupportFlags == other.SupportFlags
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_PROCESS_PROTECTED_RESOURCES").field("NodeIndex", &self.NodeIndex).field("SupportFlags", &self.SupportFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.DeinterlaceMode == other.DeinterlaceMode && self.Filters == other.Filters && self.FeatureSupport == other.FeatureSupport && self.InputFrameRate == other.InputFrameRate && self.OutputFrameRate == other.OutputFrameRate && self.EnableAutoProcessing == other.EnableAutoProcessing && self.PastFrames == other.PastFrames && self.FutureFrames == other.FutureFrames
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_PROCESS_REFERENCE_INFO").field("NodeIndex", &self.NodeIndex).field("DeinterlaceMode", &self.DeinterlaceMode).field("Filters", &self.Filters).field("FeatureSupport", &self.FeatureSupport).field("InputFrameRate", &self.InputFrameRate).field("OutputFrameRate", &self.OutputFrameRate).field("EnableAutoProcessing", &self.EnableAutoProcessing).field("PastFrames", &self.PastFrames).field("FutureFrames", &self.FutureFrames).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.InputSample == other.InputSample && self.InputFieldType == other.InputFieldType && self.InputStereoFormat == other.InputStereoFormat && self.InputFrameRate == other.InputFrameRate && self.OutputFormat == other.OutputFormat && self.OutputStereoFormat == other.OutputStereoFormat && self.OutputFrameRate == other.OutputFrameRate && self.SupportFlags == other.SupportFlags && self.ScaleSupport == other.ScaleSupport && self.FeatureSupport == other.FeatureSupport && self.DeinterlaceSupport == other.DeinterlaceSupport && self.AutoProcessingSupport == other.AutoProcessingSupport && self.FilterSupport == other.FilterSupport && self.FilterRangeSupport == other.FilterRangeSupport
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_VIDEO_PROCESS_SUPPORT")
            .field("NodeIndex", &self.NodeIndex)
            .field("InputSample", &self.InputSample)
            .field("InputFieldType", &self.InputFieldType)
            .field("InputStereoFormat", &self.InputStereoFormat)
            .field("InputFrameRate", &self.InputFrameRate)
            .field("OutputFormat", &self.OutputFormat)
            .field("OutputStereoFormat", &self.OutputStereoFormat)
            .field("OutputFrameRate", &self.OutputFrameRate)
            .field("SupportFlags", &self.SupportFlags)
            .field("ScaleSupport", &self.ScaleSupport)
            .field("FeatureSupport", &self.FeatureSupport)
            .field("DeinterlaceSupport", &self.DeinterlaceSupport)
            .field("AutoProcessingSupport", &self.AutoProcessingSupport)
            .field("FilterSupport", &self.FilterSupport)
            .field("FilterRangeSupport", &self.FilterRangeSupport)
            .finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_VIDEO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FEATURE_VIDEO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FEATURE_VIDEO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.NumMacroblocksAffected == other.NumMacroblocksAffected && self.FrameRate == other.FrameRate && self.BitRate == other.BitRate
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_QUERY_DATA_VIDEO_DECODE_STATISTICS").field("Status", &self.Status).field("NumMacroblocksAffected", &self.NumMacroblocksAffected).field("FrameRate", &self.FrameRate).field("BitRate", &self.BitRate).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pMotionVectorHeap == other.pMotionVectorHeap && self.PixelWidth == other.PixelWidth && self.PixelHeight == other.PixelHeight
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT").field("pMotionVectorHeap", &self.pMotionVectorHeap).field("PixelWidth", &self.PixelWidth).field("PixelHeight", &self.PixelHeight).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pMotionVectorTexture2D == other.pMotionVectorTexture2D && self.MotionVectorCoordinate == other.MotionVectorCoordinate
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT").field("pMotionVectorTexture2D", &self.pMotionVectorTexture2D).field("MotionVectorCoordinate", &self.MotionVectorCoordinate).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_COORDINATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RESOURCE_COORDINATE {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.SubresourceIndex == other.SubresourceIndex
    }
}
impl ::core::cmp::Eq for D3D12_RESOURCE_COORDINATE {}
impl ::core::fmt::Debug for D3D12_RESOURCE_COORDINATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_COORDINATE").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("SubresourceIndex", &self.SubresourceIndex).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.Configuration == other.Configuration
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_DECODER_DESC {}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODER_DESC").field("NodeMask", &self.NodeMask).field("Configuration", &self.Configuration).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_DECODER_HEAP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODER_HEAP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.Configuration == other.Configuration && self.DecodeWidth == other.DecodeWidth && self.DecodeHeight == other.DecodeHeight && self.Format == other.Format && self.FrameRate == other.FrameRate && self.BitRate == other.BitRate && self.MaxDecodePictureBufferCount == other.MaxDecodePictureBufferCount
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODER_HEAP_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODER_HEAP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODER_HEAP_DESC").field("NodeMask", &self.NodeMask).field("Configuration", &self.Configuration).field("DecodeWidth", &self.DecodeWidth).field("DecodeHeight", &self.DecodeHeight).field("Format", &self.Format).field("FrameRate", &self.FrameRate).field("BitRate", &self.BitRate).field("MaxDecodePictureBufferCount", &self.MaxDecodePictureBufferCount).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_ARGUMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_ARGUMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_ARGUMENT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.Offset == other.Offset && self.Size == other.Size
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_COMPRESSED_BITSTREAM").field("pBuffer", &self.pBuffer).field("Offset", &self.Offset).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.DecodeProfile == other.DecodeProfile && self.BitstreamEncryption == other.BitstreamEncryption && self.InterlaceType == other.InterlaceType
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_CONFIGURATION {}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_CONFIGURATION").field("DecodeProfile", &self.DecodeProfile).field("BitstreamEncryption", &self.BitstreamEncryption).field("InterlaceType", &self.InterlaceType).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_DECODE_CONFIGURATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.pReferenceTexture2D == other.pReferenceTexture2D && self.ReferenceSubresource == other.ReferenceSubresource && self.OutputColorSpace == other.OutputColorSpace && self.DecodeColorSpace == other.DecodeColorSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS").field("Enable", &self.Enable).field("pReferenceTexture2D", &self.pReferenceTexture2D).field("ReferenceSubresource", &self.ReferenceSubresource).field("OutputColorSpace", &self.OutputColorSpace).field("DecodeColorSpace", &self.DecodeColorSpace).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.pReferenceTexture2D == other.pReferenceTexture2D && self.ReferenceSubresource == other.ReferenceSubresource && self.OutputColorSpace == other.OutputColorSpace && self.DecodeColorSpace == other.DecodeColorSpace && self.OutputWidth == other.OutputWidth && self.OutputHeight == other.OutputHeight
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_CONVERSION_ARGUMENTS1").field("Enable", &self.Enable).field("pReferenceTexture2D", &self.pReferenceTexture2D).field("ReferenceSubresource", &self.ReferenceSubresource).field("OutputColorSpace", &self.OutputColorSpace).field("DecodeColorSpace", &self.DecodeColorSpace).field("OutputWidth", &self.OutputWidth).field("OutputHeight", &self.OutputHeight).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_DECODE_CONVERSION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Size == other.Size && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_FRAME_ARGUMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_FRAME_ARGUMENT").field("Type", &self.Type).field("Size", &self.Size).field("pData", &self.pData).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_DECODE_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.NumFrameArguments == other.NumFrameArguments && self.FrameArguments == other.FrameArguments && self.ReferenceFrames == other.ReferenceFrames && self.CompressedBitstream == other.CompressedBitstream && self.pHeap == other.pHeap
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS").field("NumFrameArguments", &self.NumFrameArguments).field("FrameArguments", &self.FrameArguments).field("ReferenceFrames", &self.ReferenceFrames).field("CompressedBitstream", &self.CompressedBitstream).field("pHeap", &self.pHeap).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.pBuffer == other.pBuffer
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_OUTPUT_HISTOGRAM").field("Offset", &self.Offset).field("pBuffer", &self.pBuffer).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.pOutputTexture2D == other.pOutputTexture2D && self.OutputSubresource == other.OutputSubresource && self.ConversionArguments == other.ConversionArguments
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS").field("pOutputTexture2D", &self.pOutputTexture2D).field("OutputSubresource", &self.OutputSubresource).field("ConversionArguments", &self.ConversionArguments).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    fn eq(&self, other: &Self) -> bool {
        self.pOutputTexture2D == other.pOutputTexture2D && self.OutputSubresource == other.OutputSubresource && self.ConversionArguments == other.ConversionArguments && self.Histograms == other.Histograms
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1").field("pOutputTexture2D", &self.pOutputTexture2D).field("OutputSubresource", &self.OutputSubresource).field("ConversionArguments", &self.ConversionArguments).field("Histograms", &self.Histograms).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    fn eq(&self, other: &Self) -> bool {
        self.NumTexture2Ds == other.NumTexture2Ds && self.ppTexture2Ds == other.ppTexture2Ds && self.pSubresources == other.pSubresources && self.ppHeaps == other.ppHeaps
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_REFERENCE_FRAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_DECODE_REFERENCE_FRAMES").field("NumTexture2Ds", &self.NumTexture2Ds).field("ppTexture2Ds", &self.ppTexture2Ds).field("pSubresources", &self.pSubresources).field("ppHeaps", &self.ppHeaps).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_DECODE_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_DECODE_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_DECODE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_DECODE_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {
    fn eq(&self, other: &Self) -> bool {
        self.ConfigurationFlags == other.ConfigurationFlags && self.DirectModeConfig == other.DirectModeConfig && self.DisableDeblockingFilterConfig == other.DisableDeblockingFilterConfig
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264").field("ConfigurationFlags", &self.ConfigurationFlags).field("DirectModeConfig", &self.DirectModeConfig).field("DisableDeblockingFilterConfig", &self.DisableDeblockingFilterConfig).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_DIRECT_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_H264_SLICES_DEBLOCKING_MODE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.ConfigurationFlags == other.ConfigurationFlags && self.MinLumaCodingUnitSize == other.MinLumaCodingUnitSize && self.MaxLumaCodingUnitSize == other.MaxLumaCodingUnitSize && self.MinLumaTransformUnitSize == other.MinLumaTransformUnitSize && self.MaxLumaTransformUnitSize == other.MaxLumaTransformUnitSize && self.max_transform_hierarchy_depth_inter == other.max_transform_hierarchy_depth_inter && self.max_transform_hierarchy_depth_intra == other.max_transform_hierarchy_depth_intra
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC")
            .field("ConfigurationFlags", &self.ConfigurationFlags)
            .field("MinLumaCodingUnitSize", &self.MinLumaCodingUnitSize)
            .field("MaxLumaCodingUnitSize", &self.MaxLumaCodingUnitSize)
            .field("MinLumaTransformUnitSize", &self.MinLumaTransformUnitSize)
            .field("MaxLumaTransformUnitSize", &self.MaxLumaTransformUnitSize)
            .field("max_transform_hierarchy_depth_inter", &self.max_transform_hierarchy_depth_inter)
            .field("max_transform_hierarchy_depth_intra", &self.max_transform_hierarchy_depth_intra)
            .finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_CUSIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_HEVC_TUSIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {
    fn eq(&self, other: &Self) -> bool {
        self.SupportFlags == other.SupportFlags && self.DisableDeblockingFilterSupportedModes == other.DisableDeblockingFilterSupportedModes
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264").field("SupportFlags", &self.SupportFlags).field("DisableDeblockingFilterSupportedModes", &self.DisableDeblockingFilterSupportedModes).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_H264_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.SupportFlags == other.SupportFlags && self.MinLumaCodingUnitSize == other.MinLumaCodingUnitSize && self.MaxLumaCodingUnitSize == other.MaxLumaCodingUnitSize && self.MinLumaTransformUnitSize == other.MinLumaTransformUnitSize && self.MaxLumaTransformUnitSize == other.MaxLumaTransformUnitSize && self.max_transform_hierarchy_depth_inter == other.max_transform_hierarchy_depth_inter && self.max_transform_hierarchy_depth_intra == other.max_transform_hierarchy_depth_intra
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC")
            .field("SupportFlags", &self.SupportFlags)
            .field("MinLumaCodingUnitSize", &self.MinLumaCodingUnitSize)
            .field("MaxLumaCodingUnitSize", &self.MaxLumaCodingUnitSize)
            .field("MinLumaTransformUnitSize", &self.MinLumaTransformUnitSize)
            .field("MaxLumaTransformUnitSize", &self.MaxLumaTransformUnitSize)
            .field("max_transform_hierarchy_depth_inter", &self.max_transform_hierarchy_depth_inter)
            .field("max_transform_hierarchy_depth_intra", &self.max_transform_hierarchy_depth_intra)
            .finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION_SUPPORT_HEVC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxL0ReferencesForP == other.MaxL0ReferencesForP && self.MaxL0ReferencesForB == other.MaxL0ReferencesForB && self.MaxL1ReferencesForB == other.MaxL1ReferencesForB && self.MaxLongTermReferences == other.MaxLongTermReferences && self.MaxDPBCapacity == other.MaxDPBCapacity
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_H264").field("MaxL0ReferencesForP", &self.MaxL0ReferencesForP).field("MaxL0ReferencesForB", &self.MaxL0ReferencesForB).field("MaxL1ReferencesForB", &self.MaxL1ReferencesForB).field("MaxLongTermReferences", &self.MaxLongTermReferences).field("MaxDPBCapacity", &self.MaxDPBCapacity).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.MaxL0ReferencesForP == other.MaxL0ReferencesForP && self.MaxL0ReferencesForB == other.MaxL0ReferencesForB && self.MaxL1ReferencesForB == other.MaxL1ReferencesForB && self.MaxLongTermReferences == other.MaxLongTermReferences && self.MaxDPBCapacity == other.MaxDPBCapacity
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_CODEC_PICTURE_CONTROL_SUPPORT_HEVC").field("MaxL0ReferencesForP", &self.MaxL0ReferencesForP).field("MaxL0ReferencesForB", &self.MaxL0ReferencesForB).field("MaxL1ReferencesForB", &self.MaxL1ReferencesForB).field("MaxLongTermReferences", &self.MaxLongTermReferences).field("MaxDPBCapacity", &self.MaxDPBCapacity).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.FrameStartOffset == other.FrameStartOffset
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_COMPRESSED_BITSTREAM").field("pBuffer", &self.pBuffer).field("FrameStartOffset", &self.FrameStartOffset).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.Bitstream == other.Bitstream && self.ReconstructedPicture == other.ReconstructedPicture && self.EncoderOutputMetadata == other.EncoderOutputMetadata
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS").field("Bitstream", &self.Bitstream).field("ReconstructedPicture", &self.ReconstructedPicture).field("EncoderOutputMetadata", &self.EncoderOutputMetadata).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_ENCODE_ERROR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.Offset == other.Offset
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_ENCODE_OPERATION_METADATA_BUFFER").field("pBuffer", &self.pBuffer).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_FRAME_SUBREGION_LAYOUT_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.bSize == other.bSize && self.bStartOffset == other.bStartOffset && self.bHeaderSize == other.bHeaderSize
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_FRAME_SUBREGION_METADATA").field("bSize", &self.bSize).field("bStartOffset", &self.bStartOffset).field("bHeaderSize", &self.bHeaderSize).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_FRAME_TYPE_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_FRAME_TYPE_H264").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_FRAME_TYPE_HEVC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_HEAP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_INTRA_REFRESH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_INTRA_REFRESH {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.IntraRefreshDuration == other.IntraRefreshDuration
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_INTRA_REFRESH {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_INTRA_REFRESH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_INTRA_REFRESH").field("Mode", &self.Mode).field("IntraRefreshDuration", &self.IntraRefreshDuration).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_INTRA_REFRESH_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_LEVELS_H264 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_LEVELS_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_LEVELS_H264").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_LEVELS_HEVC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_LEVELS_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_LEVELS_HEVC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_LEVEL_SETTING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Tier == other.Tier
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_LEVEL_TIER_CONSTRAINTS_HEVC").field("Level", &self.Level).field("Tier", &self.Tier).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.EncodeErrorFlags == other.EncodeErrorFlags && self.EncodeStats == other.EncodeStats && self.EncodedBitstreamWrittenBytesCount == other.EncodedBitstreamWrittenBytesCount && self.WrittenSubregionsCount == other.WrittenSubregionsCount
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_OUTPUT_METADATA {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_OUTPUT_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_OUTPUT_METADATA").field("EncodeErrorFlags", &self.EncodeErrorFlags).field("EncodeStats", &self.EncodeStats).field("EncodedBitstreamWrittenBytesCount", &self.EncodedBitstreamWrittenBytesCount).field("WrittenSubregionsCount", &self.WrittenSubregionsCount).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.AverageQP == other.AverageQP && self.IntraCodingUnitsCount == other.IntraCodingUnitsCount && self.InterCodingUnitsCount == other.InterCodingUnitsCount && self.SkipCodingUnitsCount == other.SkipCodingUnitsCount && self.AverageMotionEstimationXDirection == other.AverageMotionEstimationXDirection && self.AverageMotionEstimationYDirection == other.AverageMotionEstimationYDirection
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_OUTPUT_METADATA_STATISTICS").field("AverageQP", &self.AverageQP).field("IntraCodingUnitsCount", &self.IntraCodingUnitsCount).field("InterCodingUnitsCount", &self.InterCodingUnitsCount).field("SkipCodingUnitsCount", &self.SkipCodingUnitsCount).field("AverageMotionEstimationXDirection", &self.AverageMotionEstimationXDirection).field("AverageMotionEstimationYDirection", &self.AverageMotionEstimationYDirection).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.FrameType == other.FrameType
            && self.pic_parameter_set_id == other.pic_parameter_set_id
            && self.idr_pic_id == other.idr_pic_id
            && self.PictureOrderCountNumber == other.PictureOrderCountNumber
            && self.FrameDecodingOrderNumber == other.FrameDecodingOrderNumber
            && self.TemporalLayerIndex == other.TemporalLayerIndex
            && self.List0ReferenceFramesCount == other.List0ReferenceFramesCount
            && self.pList0ReferenceFrames == other.pList0ReferenceFrames
            && self.List1ReferenceFramesCount == other.List1ReferenceFramesCount
            && self.pList1ReferenceFrames == other.pList1ReferenceFrames
            && self.ReferenceFramesReconPictureDescriptorsCount == other.ReferenceFramesReconPictureDescriptorsCount
            && self.pReferenceFramesReconPictureDescriptors == other.pReferenceFramesReconPictureDescriptors
            && self.adaptive_ref_pic_marking_mode_flag == other.adaptive_ref_pic_marking_mode_flag
            && self.RefPicMarkingOperationsCommandsCount == other.RefPicMarkingOperationsCommandsCount
            && self.pRefPicMarkingOperationsCommands == other.pRefPicMarkingOperationsCommands
            && self.List0RefPicModificationsCount == other.List0RefPicModificationsCount
            && self.pList0RefPicModifications == other.pList0RefPicModifications
            && self.List1RefPicModificationsCount == other.List1RefPicModificationsCount
            && self.pList1RefPicModifications == other.pList1RefPicModifications
            && self.QPMapValuesCount == other.QPMapValuesCount
            && self.pRateControlQPMap == other.pRateControlQPMap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264")
            .field("Flags", &self.Flags)
            .field("FrameType", &self.FrameType)
            .field("pic_parameter_set_id", &self.pic_parameter_set_id)
            .field("idr_pic_id", &self.idr_pic_id)
            .field("PictureOrderCountNumber", &self.PictureOrderCountNumber)
            .field("FrameDecodingOrderNumber", &self.FrameDecodingOrderNumber)
            .field("TemporalLayerIndex", &self.TemporalLayerIndex)
            .field("List0ReferenceFramesCount", &self.List0ReferenceFramesCount)
            .field("pList0ReferenceFrames", &self.pList0ReferenceFrames)
            .field("List1ReferenceFramesCount", &self.List1ReferenceFramesCount)
            .field("pList1ReferenceFrames", &self.pList1ReferenceFrames)
            .field("ReferenceFramesReconPictureDescriptorsCount", &self.ReferenceFramesReconPictureDescriptorsCount)
            .field("pReferenceFramesReconPictureDescriptors", &self.pReferenceFramesReconPictureDescriptors)
            .field("adaptive_ref_pic_marking_mode_flag", &self.adaptive_ref_pic_marking_mode_flag)
            .field("RefPicMarkingOperationsCommandsCount", &self.RefPicMarkingOperationsCommandsCount)
            .field("pRefPicMarkingOperationsCommands", &self.pRefPicMarkingOperationsCommands)
            .field("List0RefPicModificationsCount", &self.List0RefPicModificationsCount)
            .field("pList0RefPicModifications", &self.pList0RefPicModifications)
            .field("List1RefPicModificationsCount", &self.List1RefPicModificationsCount)
            .field("pList1RefPicModifications", &self.pList1RefPicModifications)
            .field("QPMapValuesCount", &self.QPMapValuesCount)
            .field("pRateControlQPMap", &self.pRateControlQPMap)
            .finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.modification_of_pic_nums_idc == other.modification_of_pic_nums_idc && self.abs_diff_pic_num_minus1 == other.abs_diff_pic_num_minus1 && self.long_term_pic_num == other.long_term_pic_num
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_LIST_MODIFICATION_OPERATION").field("modification_of_pic_nums_idc", &self.modification_of_pic_nums_idc).field("abs_diff_pic_num_minus1", &self.abs_diff_pic_num_minus1).field("long_term_pic_num", &self.long_term_pic_num).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    fn eq(&self, other: &Self) -> bool {
        self.memory_management_control_operation == other.memory_management_control_operation && self.difference_of_pic_nums_minus1 == other.difference_of_pic_nums_minus1 && self.long_term_pic_num == other.long_term_pic_num && self.long_term_frame_idx == other.long_term_frame_idx && self.max_long_term_frame_idx_plus1 == other.max_long_term_frame_idx_plus1
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_H264_REFERENCE_PICTURE_MARKING_OPERATION").field("memory_management_control_operation", &self.memory_management_control_operation).field("difference_of_pic_nums_minus1", &self.difference_of_pic_nums_minus1).field("long_term_pic_num", &self.long_term_pic_num).field("long_term_frame_idx", &self.long_term_frame_idx).field("max_long_term_frame_idx_plus1", &self.max_long_term_frame_idx_plus1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
            && self.FrameType == other.FrameType
            && self.slice_pic_parameter_set_id == other.slice_pic_parameter_set_id
            && self.PictureOrderCountNumber == other.PictureOrderCountNumber
            && self.TemporalLayerIndex == other.TemporalLayerIndex
            && self.List0ReferenceFramesCount == other.List0ReferenceFramesCount
            && self.pList0ReferenceFrames == other.pList0ReferenceFrames
            && self.List1ReferenceFramesCount == other.List1ReferenceFramesCount
            && self.pList1ReferenceFrames == other.pList1ReferenceFrames
            && self.ReferenceFramesReconPictureDescriptorsCount == other.ReferenceFramesReconPictureDescriptorsCount
            && self.pReferenceFramesReconPictureDescriptors == other.pReferenceFramesReconPictureDescriptors
            && self.List0RefPicModificationsCount == other.List0RefPicModificationsCount
            && self.pList0RefPicModifications == other.pList0RefPicModifications
            && self.List1RefPicModificationsCount == other.List1RefPicModificationsCount
            && self.pList1RefPicModifications == other.pList1RefPicModifications
            && self.QPMapValuesCount == other.QPMapValuesCount
            && self.pRateControlQPMap == other.pRateControlQPMap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC")
            .field("Flags", &self.Flags)
            .field("FrameType", &self.FrameType)
            .field("slice_pic_parameter_set_id", &self.slice_pic_parameter_set_id)
            .field("PictureOrderCountNumber", &self.PictureOrderCountNumber)
            .field("TemporalLayerIndex", &self.TemporalLayerIndex)
            .field("List0ReferenceFramesCount", &self.List0ReferenceFramesCount)
            .field("pList0ReferenceFrames", &self.pList0ReferenceFrames)
            .field("List1ReferenceFramesCount", &self.List1ReferenceFramesCount)
            .field("pList1ReferenceFrames", &self.pList1ReferenceFrames)
            .field("ReferenceFramesReconPictureDescriptorsCount", &self.ReferenceFramesReconPictureDescriptorsCount)
            .field("pReferenceFramesReconPictureDescriptors", &self.pReferenceFramesReconPictureDescriptors)
            .field("List0RefPicModificationsCount", &self.List0RefPicModificationsCount)
            .field("pList0RefPicModifications", &self.pList0RefPicModifications)
            .field("List1RefPicModificationsCount", &self.List1RefPicModificationsCount)
            .field("pList1RefPicModifications", &self.pList1RefPicModifications)
            .field("QPMapValuesCount", &self.QPMapValuesCount)
            .field("pRateControlQPMap", &self.pRateControlQPMap)
            .finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_CODEC_DATA_HEVC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_CONTROL_SUBREGIONS_LAYOUT_DATA_SLICES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_DESC").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.WidthRatio == other.WidthRatio && self.HeightRatio == other.HeightRatio
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_PICTURE_RESOLUTION_RATIO_DESC").field("WidthRatio", &self.WidthRatio).field("HeightRatio", &self.HeightRatio).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PROFILE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PROFILE_H264 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PROFILE_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_PROFILE_H264").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_PROFILE_HEVC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_PROFILE_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_PROFILE_HEVC").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {
    fn eq(&self, other: &Self) -> bool {
        self.InitialQP == other.InitialQP && self.MinQP == other.MinQP && self.MaxQP == other.MaxQP && self.MaxFrameBitSize == other.MaxFrameBitSize && self.TargetBitRate == other.TargetBitRate && self.VBVCapacity == other.VBVCapacity && self.InitialVBVFullness == other.InitialVBVFullness
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_RATE_CONTROL_CBR").field("InitialQP", &self.InitialQP).field("MinQP", &self.MinQP).field("MaxQP", &self.MaxQP).field("MaxFrameBitSize", &self.MaxFrameBitSize).field("TargetBitRate", &self.TargetBitRate).field("VBVCapacity", &self.VBVCapacity).field("InitialVBVFullness", &self.InitialVBVFullness).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_CONFIGURATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    fn eq(&self, other: &Self) -> bool {
        self.ConstantQP_FullIntracodedFrame == other.ConstantQP_FullIntracodedFrame && self.ConstantQP_InterPredictedFrame_PrevRefOnly == other.ConstantQP_InterPredictedFrame_PrevRefOnly && self.ConstantQP_InterPredictedFrame_BiDirectionalRef == other.ConstantQP_InterPredictedFrame_BiDirectionalRef
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_RATE_CONTROL_CQP").field("ConstantQP_FullIntracodedFrame", &self.ConstantQP_FullIntracodedFrame).field("ConstantQP_InterPredictedFrame_PrevRefOnly", &self.ConstantQP_InterPredictedFrame_PrevRefOnly).field("ConstantQP_InterPredictedFrame_BiDirectionalRef", &self.ConstantQP_InterPredictedFrame_BiDirectionalRef).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_RATE_CONTROL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {
    fn eq(&self, other: &Self) -> bool {
        self.InitialQP == other.InitialQP && self.MinQP == other.MinQP && self.MaxQP == other.MaxQP && self.MaxFrameBitSize == other.MaxFrameBitSize && self.TargetAvgBitRate == other.TargetAvgBitRate && self.PeakBitRate == other.PeakBitRate && self.ConstantQualityTarget == other.ConstantQualityTarget
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_RATE_CONTROL_QVBR").field("InitialQP", &self.InitialQP).field("MinQP", &self.MinQP).field("MaxQP", &self.MaxQP).field("MaxFrameBitSize", &self.MaxFrameBitSize).field("TargetAvgBitRate", &self.TargetAvgBitRate).field("PeakBitRate", &self.PeakBitRate).field("ConstantQualityTarget", &self.ConstantQualityTarget).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {
    fn eq(&self, other: &Self) -> bool {
        self.InitialQP == other.InitialQP && self.MinQP == other.MinQP && self.MaxQP == other.MaxQP && self.MaxFrameBitSize == other.MaxFrameBitSize && self.TargetAvgBitRate == other.TargetAvgBitRate && self.PeakBitRate == other.PeakBitRate && self.VBVCapacity == other.VBVCapacity && self.InitialVBVFullness == other.InitialVBVFullness
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_RATE_CONTROL_VBR").field("InitialQP", &self.InitialQP).field("MinQP", &self.MinQP).field("MaxQP", &self.MaxQP).field("MaxFrameBitSize", &self.MaxFrameBitSize).field("TargetAvgBitRate", &self.TargetAvgBitRate).field("PeakBitRate", &self.PeakBitRate).field("VBVCapacity", &self.VBVCapacity).field("InitialVBVFullness", &self.InitialVBVFullness).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    fn eq(&self, other: &Self) -> bool {
        self.pReconstructedPicture == other.pReconstructedPicture && self.ReconstructedPictureSubresource == other.ReconstructedPictureSubresource
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_RECONSTRUCTED_PICTURE").field("pReconstructedPicture", &self.pReconstructedPicture).field("ReconstructedPictureSubresource", &self.ReconstructedPictureSubresource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    fn eq(&self, other: &Self) -> bool {
        self.ReconstructedPictureResourceIndex == other.ReconstructedPictureResourceIndex && self.IsLongTermReference == other.IsLongTermReference && self.LongTermPictureIdx == other.LongTermPictureIdx && self.PictureOrderCountNumber == other.PictureOrderCountNumber && self.FrameDecodingOrderNumber == other.FrameDecodingOrderNumber && self.TemporalLayerIndex == other.TemporalLayerIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_H264").field("ReconstructedPictureResourceIndex", &self.ReconstructedPictureResourceIndex).field("IsLongTermReference", &self.IsLongTermReference).field("LongTermPictureIdx", &self.LongTermPictureIdx).field("PictureOrderCountNumber", &self.PictureOrderCountNumber).field("FrameDecodingOrderNumber", &self.FrameDecodingOrderNumber).field("TemporalLayerIndex", &self.TemporalLayerIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.ReconstructedPictureResourceIndex == other.ReconstructedPictureResourceIndex && self.IsRefUsedByCurrentPic == other.IsRefUsedByCurrentPic && self.IsLongTermReference == other.IsLongTermReference && self.PictureOrderCountNumber == other.PictureOrderCountNumber && self.TemporalLayerIndex == other.TemporalLayerIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_REFERENCE_PICTURE_DESCRIPTOR_HEVC").field("ReconstructedPictureResourceIndex", &self.ReconstructedPictureResourceIndex).field("IsRefUsedByCurrentPic", &self.IsRefUsedByCurrentPic).field("IsLongTermReference", &self.IsLongTermReference).field("PictureOrderCountNumber", &self.PictureOrderCountNumber).field("TemporalLayerIndex", &self.TemporalLayerIndex).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.ResolvedLayoutMetadata == other.ResolvedLayoutMetadata
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS").field("ResolvedLayoutMetadata", &self.ResolvedLayoutMetadata).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_SEQUENCE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    fn eq(&self, other: &Self) -> bool {
        self.GOPLength == other.GOPLength && self.PPicturePeriod == other.PPicturePeriod && self.pic_order_cnt_type == other.pic_order_cnt_type && self.log2_max_frame_num_minus4 == other.log2_max_frame_num_minus4 && self.log2_max_pic_order_cnt_lsb_minus4 == other.log2_max_pic_order_cnt_lsb_minus4
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_H264").field("GOPLength", &self.GOPLength).field("PPicturePeriod", &self.PPicturePeriod).field("pic_order_cnt_type", &self.pic_order_cnt_type).field("log2_max_frame_num_minus4", &self.log2_max_frame_num_minus4).field("log2_max_pic_order_cnt_lsb_minus4", &self.log2_max_pic_order_cnt_lsb_minus4).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    fn eq(&self, other: &Self) -> bool {
        self.GOPLength == other.GOPLength && self.PPicturePeriod == other.PPicturePeriod && self.log2_max_pic_order_cnt_lsb_minus4 == other.log2_max_pic_order_cnt_lsb_minus4
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODER_SEQUENCE_GOP_STRUCTURE_HEVC").field("GOPLength", &self.GOPLength).field("PPicturePeriod", &self.PPicturePeriod).field("log2_max_pic_order_cnt_lsb_minus4", &self.log2_max_pic_order_cnt_lsb_minus4).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_TIER_HEVC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_TIER_HEVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_TIER_HEVC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_ENCODER_VALIDATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_ENCODER_VALIDATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    fn eq(&self, other: &Self) -> bool {
        self.NumTexture2Ds == other.NumTexture2Ds && self.ppTexture2Ds == other.ppTexture2Ds && self.pSubresources == other.pSubresources
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_ENCODE_REFERENCE_FRAMES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_ENCODE_REFERENCE_FRAMES").field("NumTexture2Ds", &self.NumTexture2Ds).field("ppTexture2Ds", &self.ppTexture2Ds).field("pSubresources", &self.pSubresources).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.CommandId == other.CommandId
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_EXTENSION_COMMAND_DESC {}
impl ::core::fmt::Debug for D3D12_VIDEO_EXTENSION_COMMAND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_EXTENSION_COMMAND_DESC").field("NodeMask", &self.NodeMask).field("CommandId", &self.CommandId).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.CommandId == other.CommandId && self.Name == other.Name && self.CommandListSupportFlags == other.CommandListSupportFlags
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_EXTENSION_COMMAND_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_EXTENSION_COMMAND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_EXTENSION_COMMAND_INFO").field("CommandId", &self.CommandId).field("Name", &self.Name).field("CommandListSupportFlags", &self.CommandListSupportFlags).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {}
impl ::core::fmt::Debug for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_INFO").field("Name", &self.Name).field("Type", &self.Type).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_STAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_EXTENSION_COMMAND_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_FIELD_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.ColorSpace == other.ColorSpace
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_VIDEO_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_VIDEO_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_FORMAT").field("Format", &self.Format).field("ColorSpace", &self.ColorSpace).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_FRAME_CODED_INTERLACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_FRAME_STEREO_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_FRAME_STEREO_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_FRAME_STEREO_FORMAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.InputFormat == other.InputFormat && self.BlockSize == other.BlockSize && self.Precision == other.Precision && self.SizeRange == other.SizeRange
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_VIDEO_MOTION_ESTIMATOR_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_MOTION_ESTIMATOR_DESC").field("NodeMask", &self.NodeMask).field("InputFormat", &self.InputFormat).field("BlockSize", &self.BlockSize).field("Precision", &self.Precision).field("SizeRange", &self.SizeRange).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pInputTexture2D == other.pInputTexture2D && self.InputSubresourceIndex == other.InputSubresourceIndex && self.pReferenceTexture2D == other.pReferenceTexture2D && self.ReferenceSubresourceIndex == other.ReferenceSubresourceIndex && self.pHintMotionVectorHeap == other.pHintMotionVectorHeap
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_MOTION_ESTIMATOR_INPUT").field("pInputTexture2D", &self.pInputTexture2D).field("InputSubresourceIndex", &self.InputSubresourceIndex).field("pReferenceTexture2D", &self.pReferenceTexture2D).field("ReferenceSubresourceIndex", &self.ReferenceSubresourceIndex).field("pHintMotionVectorHeap", &self.pHintMotionVectorHeap).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pMotionVectorHeap == other.pMotionVectorHeap
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT").field("pMotionVectorHeap", &self.pMotionVectorHeap).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_MOTION_ESTIMATOR_SEARCH_BLOCK_SIZE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_MOTION_ESTIMATOR_VECTOR_PRECISION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.InputFormat == other.InputFormat && self.BlockSize == other.BlockSize && self.Precision == other.Precision && self.SizeRange == other.SizeRange
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC").field("NodeMask", &self.NodeMask).field("InputFormat", &self.InputFormat).field("BlockSize", &self.BlockSize).field("Precision", &self.Precision).field("SizeRange", &self.SizeRange).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Alpha == other.Alpha
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_ALPHA_BLENDING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_ALPHA_BLENDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_ALPHA_BLENDING").field("Enable", &self.Enable).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_ALPHA_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROCESS_AUTO_PROCESSING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROCESS_DEINTERLACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_FEATURE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROCESS_FEATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROCESS_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_FILTER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_FILTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Minimum == other.Minimum && self.Maximum == other.Maximum && self.Default == other.Default && self.Multiplier == other.Multiplier
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_FILTER_RANGE {}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_FILTER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_FILTER_RANGE").field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).field("Default", &self.Default).field("Multiplier", &self.Multiplier).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_INPUT_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_INPUT_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.pTexture2D == other.pTexture2D && self.Subresource == other.Subresource && self.ReferenceSet == other.ReferenceSet
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_INPUT_STREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_INPUT_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_INPUT_STREAM").field("pTexture2D", &self.pTexture2D).field("Subresource", &self.Subresource).field("ReferenceSet", &self.ReferenceSet).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.InputStream == other.InputStream && self.Transform == other.Transform && self.Flags == other.Flags && self.RateInfo == other.RateInfo && self.FilterLevels == other.FilterLevels && self.AlphaBlending == other.AlphaBlending
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS").field("InputStream", &self.InputStream).field("Transform", &self.Transform).field("Flags", &self.Flags).field("RateInfo", &self.RateInfo).field("FilterLevels", &self.FilterLevels).field("AlphaBlending", &self.AlphaBlending).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    fn eq(&self, other: &Self) -> bool {
        self.InputStream == other.InputStream && self.Transform == other.Transform && self.Flags == other.Flags && self.RateInfo == other.RateInfo && self.FilterLevels == other.FilterLevels && self.AlphaBlending == other.AlphaBlending && self.FieldType == other.FieldType
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1").field("InputStream", &self.InputStream).field("Transform", &self.Transform).field("Flags", &self.Flags).field("RateInfo", &self.RateInfo).field("FilterLevels", &self.FilterLevels).field("AlphaBlending", &self.AlphaBlending).field("FieldType", &self.FieldType).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format
            && self.ColorSpace == other.ColorSpace
            && self.SourceAspectRatio == other.SourceAspectRatio
            && self.DestinationAspectRatio == other.DestinationAspectRatio
            && self.FrameRate == other.FrameRate
            && self.SourceSizeRange == other.SourceSizeRange
            && self.DestinationSizeRange == other.DestinationSizeRange
            && self.EnableOrientation == other.EnableOrientation
            && self.FilterFlags == other.FilterFlags
            && self.StereoFormat == other.StereoFormat
            && self.FieldType == other.FieldType
            && self.DeinterlaceMode == other.DeinterlaceMode
            && self.EnableAlphaBlending == other.EnableAlphaBlending
            && self.LumaKey == other.LumaKey
            && self.NumPastFrames == other.NumPastFrames
            && self.NumFutureFrames == other.NumFutureFrames
            && self.EnableAutoProcessing == other.EnableAutoProcessing
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC")
            .field("Format", &self.Format)
            .field("ColorSpace", &self.ColorSpace)
            .field("SourceAspectRatio", &self.SourceAspectRatio)
            .field("DestinationAspectRatio", &self.DestinationAspectRatio)
            .field("FrameRate", &self.FrameRate)
            .field("SourceSizeRange", &self.SourceSizeRange)
            .field("DestinationSizeRange", &self.DestinationSizeRange)
            .field("EnableOrientation", &self.EnableOrientation)
            .field("FilterFlags", &self.FilterFlags)
            .field("StereoFormat", &self.StereoFormat)
            .field("FieldType", &self.FieldType)
            .field("DeinterlaceMode", &self.DeinterlaceMode)
            .field("EnableAlphaBlending", &self.EnableAlphaBlending)
            .field("LumaKey", &self.LumaKey)
            .field("NumPastFrames", &self.NumPastFrames)
            .field("NumFutureFrames", &self.NumFutureFrames)
            .field("EnableAutoProcessing", &self.EnableAutoProcessing)
            .finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROCESS_INPUT_STREAM_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    fn eq(&self, other: &Self) -> bool {
        self.OutputIndex == other.OutputIndex && self.InputFrameOrField == other.InputFrameOrField
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_INPUT_STREAM_RATE").field("OutputIndex", &self.OutputIndex).field("InputFrameOrField", &self.InputFrameOrField).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_LUMA_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_LUMA_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Lower == other.Lower && self.Upper == other.Upper
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_LUMA_KEY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_LUMA_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_LUMA_KEY").field("Enable", &self.Enable).field("Lower", &self.Lower).field("Upper", &self.Upper).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_ORIENTATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.pTexture2D == other.pTexture2D && self.Subresource == other.Subresource
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_OUTPUT_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_OUTPUT_STREAM").field("pTexture2D", &self.pTexture2D).field("Subresource", &self.Subresource).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.OutputStream == other.OutputStream && self.TargetRectangle == other.TargetRectangle
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS").field("OutputStream", &self.OutputStream).field("TargetRectangle", &self.TargetRectangle).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.ColorSpace == other.ColorSpace && self.AlphaFillMode == other.AlphaFillMode && self.AlphaFillModeSourceStreamIndex == other.AlphaFillModeSourceStreamIndex && self.BackgroundColor == other.BackgroundColor && self.FrameRate == other.FrameRate && self.EnableStereo == other.EnableStereo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC").field("Format", &self.Format).field("ColorSpace", &self.ColorSpace).field("AlphaFillMode", &self.AlphaFillMode).field("AlphaFillModeSourceStreamIndex", &self.AlphaFillModeSourceStreamIndex).field("BackgroundColor", &self.BackgroundColor).field("FrameRate", &self.FrameRate).field("EnableStereo", &self.EnableStereo).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_REFERENCE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_REFERENCE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.NumPastFrames == other.NumPastFrames && self.ppPastFrames == other.ppPastFrames && self.pPastSubresources == other.pPastSubresources && self.NumFutureFrames == other.NumFutureFrames && self.ppFutureFrames == other.ppFutureFrames && self.pFutureSubresources == other.pFutureSubresources
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_REFERENCE_SET {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_REFERENCE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_REFERENCE_SET").field("NumPastFrames", &self.NumPastFrames).field("ppPastFrames", &self.ppPastFrames).field("pPastSubresources", &self.pPastSubresources).field("NumFutureFrames", &self.NumFutureFrames).field("ppFutureFrames", &self.ppFutureFrames).field("pFutureSubresources", &self.pFutureSubresources).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROCESS_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROCESS_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_VIDEO_PROCESS_TRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_PROCESS_TRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.SourceRectangle == other.SourceRectangle && self.DestinationRectangle == other.DestinationRectangle && self.Orientation == other.Orientation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_VIDEO_PROCESS_TRANSFORM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_VIDEO_PROCESS_TRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_PROCESS_TRANSFORM").field("SourceRectangle", &self.SourceRectangle).field("DestinationRectangle", &self.DestinationRectangle).field("Orientation", &self.Orientation).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_PROTECTED_RESOURCE_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_VIDEO_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_VIDEO_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_VIDEO_SAMPLE {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_VIDEO_SAMPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_SAMPLE").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_SCALE_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_SCALE_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.OutputSizeRange == other.OutputSizeRange && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_SCALE_SUPPORT {}
impl ::core::fmt::Debug for D3D12_VIDEO_SCALE_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_SCALE_SUPPORT").field("OutputSizeRange", &self.OutputSizeRange).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIDEO_SCALE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIDEO_SCALE_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIDEO_SIZE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIDEO_SIZE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.MaxWidth == other.MaxWidth && self.MaxHeight == other.MaxHeight && self.MinWidth == other.MinWidth && self.MinHeight == other.MinHeight
    }
}
impl ::core::cmp::Eq for D3D12_VIDEO_SIZE_RANGE {}
impl ::core::fmt::Debug for D3D12_VIDEO_SIZE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIDEO_SIZE_RANGE").field("MaxWidth", &self.MaxWidth).field("MaxHeight", &self.MaxHeight).field("MinWidth", &self.MinWidth).field("MinHeight", &self.MinHeight).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DCONTENTPROTECTIONCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DCONTENTPROTECTIONCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3DOVERLAYCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DOVERLAYCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Caps == other.Caps && self.MaxOverlayDisplayWidth == other.MaxOverlayDisplayWidth && self.MaxOverlayDisplayHeight == other.MaxOverlayDisplayHeight
    }
}
impl ::core::cmp::Eq for D3DOVERLAYCAPS {}
impl ::core::fmt::Debug for D3DOVERLAYCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DOVERLAYCAPS").field("Caps", &self.Caps).field("MaxOverlayDisplayWidth", &self.MaxOverlayDisplayWidth).field("MaxOverlayDisplayHeight", &self.MaxOverlayDisplayHeight).finish()
    }
}
impl ::core::default::Default for DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pFriendlyDeviceName == other.pFriendlyDeviceName && self.pUniqueDeviceName == other.pUniqueDeviceName && self.pManufacturerName == other.pManufacturerName && self.pModelName == other.pModelName && self.pIconURL == other.pIconURL
    }
}
impl ::core::cmp::Eq for DEVICE_INFO {}
impl ::core::fmt::Debug for DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_INFO").field("pFriendlyDeviceName", &self.pFriendlyDeviceName).field("pUniqueDeviceName", &self.pUniqueDeviceName).field("pManufacturerName", &self.pManufacturerName).field("pModelName", &self.pModelName).field("pIconURL", &self.pIconURL).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIRTYRECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIRTYRECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FrameNumber == other.FrameNumber && self.NumDirtyRects == other.NumDirtyRects && self.DirtyRects == other.DirtyRects
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIRTYRECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIRTYRECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRTYRECT_INFO").field("FrameNumber", &self.FrameNumber).field("NumDirtyRects", &self.NumDirtyRects).field("DirtyRects", &self.DirtyRects).finish()
    }
}
impl ::core::default::Default for DXVA2_AES_CTR_IV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_AES_CTR_IV {
    fn eq(&self, other: &Self) -> bool {
        self.IV == other.IV && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for DXVA2_AES_CTR_IV {}
impl ::core::fmt::Debug for DXVA2_AES_CTR_IV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_AES_CTR_IV").field("IV", &self.IV).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for DXVA2_AYUVSample16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_AYUVSample16 {
    fn eq(&self, other: &Self) -> bool {
        self.Cr == other.Cr && self.Cb == other.Cb && self.Y == other.Y && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DXVA2_AYUVSample16 {}
impl ::core::fmt::Debug for DXVA2_AYUVSample16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_AYUVSample16").field("Cr", &self.Cr).field("Cb", &self.Cb).field("Y", &self.Y).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for DXVA2_AYUVSample8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_AYUVSample8 {
    fn eq(&self, other: &Self) -> bool {
        self.Cr == other.Cr && self.Cb == other.Cb && self.Y == other.Y && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for DXVA2_AYUVSample8 {}
impl ::core::fmt::Debug for DXVA2_AYUVSample8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_AYUVSample8").field("Cr", &self.Cr).field("Cb", &self.Cb).field("Y", &self.Y).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for DXVA2_BufferfType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_BufferfType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_BufferfType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_ConfigPictureDecode {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_ConfigPictureDecode {
    fn eq(&self, other: &Self) -> bool {
        self.guidConfigBitstreamEncryption == other.guidConfigBitstreamEncryption
            && self.guidConfigMBcontrolEncryption == other.guidConfigMBcontrolEncryption
            && self.guidConfigResidDiffEncryption == other.guidConfigResidDiffEncryption
            && self.ConfigBitstreamRaw == other.ConfigBitstreamRaw
            && self.ConfigMBcontrolRasterOrder == other.ConfigMBcontrolRasterOrder
            && self.ConfigResidDiffHost == other.ConfigResidDiffHost
            && self.ConfigSpatialResid8 == other.ConfigSpatialResid8
            && self.ConfigResid8Subtraction == other.ConfigResid8Subtraction
            && self.ConfigSpatialHost8or9Clipping == other.ConfigSpatialHost8or9Clipping
            && self.ConfigSpatialResidInterleaved == other.ConfigSpatialResidInterleaved
            && self.ConfigIntraResidUnsigned == other.ConfigIntraResidUnsigned
            && self.ConfigResidDiffAccelerator == other.ConfigResidDiffAccelerator
            && self.ConfigHostInverseScan == other.ConfigHostInverseScan
            && self.ConfigSpecificIDCT == other.ConfigSpecificIDCT
            && self.Config4GroupedCoefs == other.Config4GroupedCoefs
            && self.ConfigMinRenderTargetBuffCount == other.ConfigMinRenderTargetBuffCount
            && self.ConfigDecoderSpecific == other.ConfigDecoderSpecific
    }
}
impl ::core::cmp::Eq for DXVA2_ConfigPictureDecode {}
impl ::core::fmt::Debug for DXVA2_ConfigPictureDecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_ConfigPictureDecode")
            .field("guidConfigBitstreamEncryption", &self.guidConfigBitstreamEncryption)
            .field("guidConfigMBcontrolEncryption", &self.guidConfigMBcontrolEncryption)
            .field("guidConfigResidDiffEncryption", &self.guidConfigResidDiffEncryption)
            .field("ConfigBitstreamRaw", &self.ConfigBitstreamRaw)
            .field("ConfigMBcontrolRasterOrder", &self.ConfigMBcontrolRasterOrder)
            .field("ConfigResidDiffHost", &self.ConfigResidDiffHost)
            .field("ConfigSpatialResid8", &self.ConfigSpatialResid8)
            .field("ConfigResid8Subtraction", &self.ConfigResid8Subtraction)
            .field("ConfigSpatialHost8or9Clipping", &self.ConfigSpatialHost8or9Clipping)
            .field("ConfigSpatialResidInterleaved", &self.ConfigSpatialResidInterleaved)
            .field("ConfigIntraResidUnsigned", &self.ConfigIntraResidUnsigned)
            .field("ConfigResidDiffAccelerator", &self.ConfigResidDiffAccelerator)
            .field("ConfigHostInverseScan", &self.ConfigHostInverseScan)
            .field("ConfigSpecificIDCT", &self.ConfigSpecificIDCT)
            .field("Config4GroupedCoefs", &self.Config4GroupedCoefs)
            .field("ConfigMinRenderTargetBuffCount", &self.ConfigMinRenderTargetBuffCount)
            .field("ConfigDecoderSpecific", &self.ConfigDecoderSpecific)
            .finish()
    }
}
impl ::core::default::Default for DXVA2_DecodeBufferDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_DecodeBufferDesc {
    fn eq(&self, other: &Self) -> bool {
        self.CompressedBufferType == other.CompressedBufferType && self.BufferIndex == other.BufferIndex && self.DataOffset == other.DataOffset && self.DataSize == other.DataSize && self.FirstMBaddress == other.FirstMBaddress && self.NumMBsInBuffer == other.NumMBsInBuffer && self.Width == other.Width && self.Height == other.Height && self.Stride == other.Stride && self.ReservedBits == other.ReservedBits && self.pvPVPState == other.pvPVPState
    }
}
impl ::core::cmp::Eq for DXVA2_DecodeBufferDesc {}
impl ::core::fmt::Debug for DXVA2_DecodeBufferDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_DecodeBufferDesc").field("CompressedBufferType", &self.CompressedBufferType).field("BufferIndex", &self.BufferIndex).field("DataOffset", &self.DataOffset).field("DataSize", &self.DataSize).field("FirstMBaddress", &self.FirstMBaddress).field("NumMBsInBuffer", &self.NumMBsInBuffer).field("Width", &self.Width).field("Height", &self.Height).field("Stride", &self.Stride).field("ReservedBits", &self.ReservedBits).field("pvPVPState", &self.pvPVPState).finish()
    }
}
impl ::core::default::Default for DXVA2_DecodeExecuteParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_DecodeExecuteParams {
    fn eq(&self, other: &Self) -> bool {
        self.NumCompBuffers == other.NumCompBuffers && self.pCompressedBuffers == other.pCompressedBuffers && self.pExtensionData == other.pExtensionData
    }
}
impl ::core::cmp::Eq for DXVA2_DecodeExecuteParams {}
impl ::core::fmt::Debug for DXVA2_DecodeExecuteParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_DecodeExecuteParams").field("NumCompBuffers", &self.NumCompBuffers).field("pCompressedBuffers", &self.pCompressedBuffers).field("pExtensionData", &self.pExtensionData).finish()
    }
}
impl ::core::default::Default for DXVA2_DecodeExtensionData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_DecodeExtensionData {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.pPrivateInputData == other.pPrivateInputData && self.PrivateInputDataSize == other.PrivateInputDataSize && self.pPrivateOutputData == other.pPrivateOutputData && self.PrivateOutputDataSize == other.PrivateOutputDataSize
    }
}
impl ::core::cmp::Eq for DXVA2_DecodeExtensionData {}
impl ::core::fmt::Debug for DXVA2_DecodeExtensionData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_DecodeExtensionData").field("Function", &self.Function).field("pPrivateInputData", &self.pPrivateInputData).field("PrivateInputDataSize", &self.PrivateInputDataSize).field("pPrivateOutputData", &self.pPrivateOutputData).field("PrivateOutputDataSize", &self.PrivateOutputDataSize).finish()
    }
}
impl ::core::default::Default for DXVA2_DeinterlaceTech {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_DeinterlaceTech {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_DeinterlaceTech").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_DestData {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_DestData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_DestData").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_DetailFilterTech {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_DetailFilterTech {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_DetailFilterTech").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_ExtendedFormat {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_FilterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_FilterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_FilterType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_FilterValues {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_Fixed32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_Frequency {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA2_Frequency {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DXVA2_Frequency {}
impl ::core::fmt::Debug for DXVA2_Frequency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_Frequency").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for DXVA2_NoiseFilterTech {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_NoiseFilterTech {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_NoiseFilterTech").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_NominalRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_NominalRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_NominalRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_ProcAmp {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_ProcAmp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_ProcAmp").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_ProcAmpValues {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_SampleData {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_SampleData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_SampleData").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_SampleFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_SampleFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_SampleFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_SurfaceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_SurfaceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_SurfaceType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_VPDev {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VPDev {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VPDev").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_ValueRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_VideoChromaSubSampling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoChromaSubSampling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoChromaSubSampling").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA2_VideoDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_VideoLighting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoLighting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoLighting").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_VideoPrimaries {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoPrimaries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoPrimaries").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_VideoProcess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoProcess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoProcess").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA2_VideoProcessBltParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA2_VideoProcessorCaps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVA2_VideoProcessorCaps {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceCaps == other.DeviceCaps && self.InputPool == other.InputPool && self.NumForwardRefSamples == other.NumForwardRefSamples && self.NumBackwardRefSamples == other.NumBackwardRefSamples && self.Reserved == other.Reserved && self.DeinterlaceTechnology == other.DeinterlaceTechnology && self.ProcAmpControlCaps == other.ProcAmpControlCaps && self.VideoProcessorOperations == other.VideoProcessorOperations && self.NoiseFilterTechnology == other.NoiseFilterTechnology && self.DetailFilterTechnology == other.DetailFilterTechnology
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVA2_VideoProcessorCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVA2_VideoProcessorCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA2_VideoProcessorCaps")
            .field("DeviceCaps", &self.DeviceCaps)
            .field("InputPool", &self.InputPool)
            .field("NumForwardRefSamples", &self.NumForwardRefSamples)
            .field("NumBackwardRefSamples", &self.NumBackwardRefSamples)
            .field("Reserved", &self.Reserved)
            .field("DeinterlaceTechnology", &self.DeinterlaceTechnology)
            .field("ProcAmpControlCaps", &self.ProcAmpControlCaps)
            .field("VideoProcessorOperations", &self.VideoProcessorOperations)
            .field("NoiseFilterTechnology", &self.NoiseFilterTechnology)
            .field("DetailFilterTechnology", &self.DetailFilterTechnology)
            .finish()
    }
}
impl ::core::default::Default for DXVA2_VideoRenderTargetType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoRenderTargetType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoRenderTargetType").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::default::Default for DXVA2_VideoSample {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA2_VideoTransferFunction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoTransferFunction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoTransferFunction").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA2_VideoTransferMatrix {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA2_VideoTransferMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA2_VideoTransferMatrix").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVABufferInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVABufferInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pCompSurface == other.pCompSurface && self.DataOffset == other.DataOffset && self.DataSize == other.DataSize
    }
}
impl ::core::cmp::Eq for DXVABufferInfo {}
impl ::core::fmt::Debug for DXVABufferInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVABufferInfo").field("pCompSurface", &self.pCompSurface).field("DataOffset", &self.DataOffset).field("DataSize", &self.DataSize).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVACompBufferInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVACompBufferInfo {
    fn eq(&self, other: &Self) -> bool {
        self.NumCompBuffers == other.NumCompBuffers && self.WidthToCreate == other.WidthToCreate && self.HeightToCreate == other.HeightToCreate && self.BytesToAllocate == other.BytesToAllocate && self.Usage == other.Usage && self.Pool == other.Pool && self.Format == other.Format
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVACompBufferInfo {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVACompBufferInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVACompBufferInfo").field("NumCompBuffers", &self.NumCompBuffers).field("WidthToCreate", &self.WidthToCreate).field("HeightToCreate", &self.HeightToCreate).field("BytesToAllocate", &self.BytesToAllocate).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Format", &self.Format).finish()
    }
}
impl ::core::default::Default for DXVAHDETW_CREATEVIDEOPROCESSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHDETW_CREATEVIDEOPROCESSOR {
    fn eq(&self, other: &Self) -> bool {
        self.pObject == other.pObject && self.pD3D9Ex == other.pD3D9Ex && self.VPGuid == other.VPGuid
    }
}
impl ::core::cmp::Eq for DXVAHDETW_CREATEVIDEOPROCESSOR {}
impl ::core::fmt::Debug for DXVAHDETW_CREATEVIDEOPROCESSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHDETW_CREATEVIDEOPROCESSOR").field("pObject", &self.pObject).field("pD3D9Ex", &self.pD3D9Ex).field("VPGuid", &self.VPGuid).finish()
    }
}
impl ::core::default::Default for DXVAHDETW_DESTROYVIDEOPROCESSOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHDETW_DESTROYVIDEOPROCESSOR {
    fn eq(&self, other: &Self) -> bool {
        self.pObject == other.pObject
    }
}
impl ::core::cmp::Eq for DXVAHDETW_DESTROYVIDEOPROCESSOR {}
impl ::core::fmt::Debug for DXVAHDETW_DESTROYVIDEOPROCESSOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHDETW_DESTROYVIDEOPROCESSOR").field("pObject", &self.pObject).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::default::Default for DXVAHDETW_VIDEOPROCESSBLTHD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::cmp::PartialEq for DXVAHDETW_VIDEOPROCESSBLTHD {
    fn eq(&self, other: &Self) -> bool {
        self.pObject == other.pObject && self.pOutputSurface == other.pOutputSurface && self.TargetRect == other.TargetRect && self.OutputFormat == other.OutputFormat && self.ColorSpace == other.ColorSpace && self.OutputFrame == other.OutputFrame && self.StreamCount == other.StreamCount && self.Enter == other.Enter
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::cmp::Eq for DXVAHDETW_VIDEOPROCESSBLTHD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::fmt::Debug for DXVAHDETW_VIDEOPROCESSBLTHD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHDETW_VIDEOPROCESSBLTHD").field("pObject", &self.pObject).field("pOutputSurface", &self.pOutputSurface).field("TargetRect", &self.TargetRect).field("OutputFormat", &self.OutputFormat).field("ColorSpace", &self.ColorSpace).field("OutputFrame", &self.OutputFrame).field("StreamCount", &self.StreamCount).field("Enter", &self.Enter).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::default::Default for DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::cmp::PartialEq for DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.pObject == other.pObject && self.pInputSurface == other.pInputSurface && self.SourceRect == other.SourceRect && self.DestinationRect == other.DestinationRect && self.InputFormat == other.InputFormat && self.FrameFormat == other.FrameFormat && self.ColorSpace == other.ColorSpace && self.StreamNumber == other.StreamNumber && self.OutputIndex == other.OutputIndex && self.InputFrameOrField == other.InputFrameOrField && self.PastFrames == other.PastFrames && self.FutureFrames == other.FutureFrames
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::cmp::Eq for DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::fmt::Debug for DXVAHDETW_VIDEOPROCESSBLTHD_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHDETW_VIDEOPROCESSBLTHD_STREAM")
            .field("pObject", &self.pObject)
            .field("pInputSurface", &self.pInputSurface)
            .field("SourceRect", &self.SourceRect)
            .field("DestinationRect", &self.DestinationRect)
            .field("InputFormat", &self.InputFormat)
            .field("FrameFormat", &self.FrameFormat)
            .field("ColorSpace", &self.ColorSpace)
            .field("StreamNumber", &self.StreamNumber)
            .field("OutputIndex", &self.OutputIndex)
            .field("InputFrameOrField", &self.InputFrameOrField)
            .field("PastFrames", &self.PastFrames)
            .field("FutureFrames", &self.FutureFrames)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHDETW_VIDEOPROCESSBLTSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHDETW_VIDEOPROCESSBLTSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.pObject == other.pObject && self.State == other.State && self.DataSize == other.DataSize && self.SetState == other.SetState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHDETW_VIDEOPROCESSBLTSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHDETW_VIDEOPROCESSBLTSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHDETW_VIDEOPROCESSBLTSTATE").field("pObject", &self.pObject).field("State", &self.State).field("DataSize", &self.DataSize).field("SetState", &self.SetState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHDETW_VIDEOPROCESSSTREAMSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHDETW_VIDEOPROCESSSTREAMSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.pObject == other.pObject && self.StreamNumber == other.StreamNumber && self.State == other.State && self.DataSize == other.DataSize && self.SetState == other.SetState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHDETW_VIDEOPROCESSSTREAMSTATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHDETW_VIDEOPROCESSSTREAMSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHDETW_VIDEOPROCESSSTREAMSTATE").field("pObject", &self.pObject).field("StreamNumber", &self.StreamNumber).field("State", &self.State).field("DataSize", &self.DataSize).field("SetState", &self.SetState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::default::Default for DXVAHDSW_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVAHD_ALPHA_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_ALPHA_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_ALPHA_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_BLT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_BLT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_BLT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_BLT_STATE_ALPHA_FILL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_BLT_STATE_ALPHA_FILL_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Mode == other.Mode && self.StreamNumber == other.StreamNumber
    }
}
impl ::core::cmp::Eq for DXVAHD_BLT_STATE_ALPHA_FILL_DATA {}
impl ::core::fmt::Debug for DXVAHD_BLT_STATE_ALPHA_FILL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_BLT_STATE_ALPHA_FILL_DATA").field("Mode", &self.Mode).field("StreamNumber", &self.StreamNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_BLT_STATE_BACKGROUND_COLOR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_BLT_STATE_CONSTRICTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_BLT_STATE_CONSTRICTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Size == other.Size
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_BLT_STATE_CONSTRICTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_BLT_STATE_CONSTRICTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_BLT_STATE_CONSTRICTION_DATA").field("Enable", &self.Enable).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for DXVAHD_BLT_STATE_OUTPUT_COLOR_SPACE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVAHD_BLT_STATE_PRIVATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_BLT_STATE_PRIVATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for DXVAHD_BLT_STATE_PRIVATE_DATA {}
impl ::core::fmt::Debug for DXVAHD_BLT_STATE_PRIVATE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_BLT_STATE_PRIVATE_DATA").field("Guid", &self.Guid).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_BLT_STATE_TARGET_RECT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_BLT_STATE_TARGET_RECT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.TargetRect == other.TargetRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_BLT_STATE_TARGET_RECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_BLT_STATE_TARGET_RECT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_BLT_STATE_TARGET_RECT_DATA").field("Enable", &self.Enable).field("TargetRect", &self.TargetRect).finish()
    }
}
impl ::core::default::Default for DXVAHD_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVAHD_COLOR_RGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_COLOR_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B && self.A == other.A
    }
}
impl ::core::cmp::Eq for DXVAHD_COLOR_RGBA {}
impl ::core::fmt::Debug for DXVAHD_COLOR_RGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_COLOR_RGBA").field("R", &self.R).field("G", &self.G).field("B", &self.B).field("A", &self.A).finish()
    }
}
impl ::core::default::Default for DXVAHD_COLOR_YCbCrA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_COLOR_YCbCrA {
    fn eq(&self, other: &Self) -> bool {
        self.Y == other.Y && self.Cb == other.Cb && self.Cr == other.Cr && self.A == other.A
    }
}
impl ::core::cmp::Eq for DXVAHD_COLOR_YCbCrA {}
impl ::core::fmt::Debug for DXVAHD_COLOR_YCbCrA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_COLOR_YCbCrA").field("Y", &self.Y).field("Cb", &self.Cb).field("Cr", &self.Cr).field("A", &self.A).finish()
    }
}
impl ::core::default::Default for DXVAHD_CONTENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_CONTENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputFrameFormat == other.InputFrameFormat && self.InputFrameRate == other.InputFrameRate && self.InputWidth == other.InputWidth && self.InputHeight == other.InputHeight && self.OutputFrameRate == other.OutputFrameRate && self.OutputWidth == other.OutputWidth && self.OutputHeight == other.OutputHeight
    }
}
impl ::core::cmp::Eq for DXVAHD_CONTENT_DESC {}
impl ::core::fmt::Debug for DXVAHD_CONTENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_CONTENT_DESC").field("InputFrameFormat", &self.InputFrameFormat).field("InputFrameRate", &self.InputFrameRate).field("InputWidth", &self.InputWidth).field("InputHeight", &self.InputHeight).field("OutputFrameRate", &self.OutputFrameRate).field("OutputWidth", &self.OutputWidth).field("OutputHeight", &self.OutputHeight).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_CUSTOM_RATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_CUSTOM_RATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.CustomRate == other.CustomRate && self.OutputFrames == other.OutputFrames && self.InputInterlaced == other.InputInterlaced && self.InputFramesOrFields == other.InputFramesOrFields
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_CUSTOM_RATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_CUSTOM_RATE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_CUSTOM_RATE_DATA").field("CustomRate", &self.CustomRate).field("OutputFrames", &self.OutputFrames).field("InputInterlaced", &self.InputInterlaced).field("InputFramesOrFields", &self.InputFramesOrFields).finish()
    }
}
impl ::core::default::Default for DXVAHD_DEVICE_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_DEVICE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_DEVICE_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_DEVICE_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_DEVICE_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_DEVICE_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_FEATURE_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_FEATURE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_FEATURE_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_FILTER_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_FILTER_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_FILTER_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_FILTER_RANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_FILTER_RANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Minimum == other.Minimum && self.Maximum == other.Maximum && self.Default == other.Default && self.Multiplier == other.Multiplier
    }
}
impl ::core::cmp::Eq for DXVAHD_FILTER_RANGE_DATA {}
impl ::core::fmt::Debug for DXVAHD_FILTER_RANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_FILTER_RANGE_DATA").field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).field("Default", &self.Default).field("Multiplier", &self.Multiplier).finish()
    }
}
impl ::core::default::Default for DXVAHD_FRAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_FRAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_FRAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_INPUT_FORMAT_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_INPUT_FORMAT_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_INPUT_FORMAT_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_ITELECINE_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_ITELECINE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_ITELECINE_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_OUTPUT_RATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_OUTPUT_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_OUTPUT_RATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_PROCESSOR_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_PROCESSOR_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_PROCESSOR_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_RATIONAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DXVAHD_RATIONAL {}
impl ::core::fmt::Debug for DXVAHD_RATIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_RATIONAL").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::default::Default for DXVAHD_STREAM_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.OutputIndex == other.OutputIndex && self.InputFrameOrField == other.InputFrameOrField && self.PastFrames == other.PastFrames && self.FutureFrames == other.FutureFrames && self.ppPastSurfaces == other.ppPastSurfaces && self.pInputSurface == other.pInputSurface && self.ppFutureSurfaces == other.ppFutureSurfaces
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::cmp::Eq for DXVAHD_STREAM_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
impl ::core::fmt::Debug for DXVAHD_STREAM_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_DATA").field("Enable", &self.Enable).field("OutputIndex", &self.OutputIndex).field("InputFrameOrField", &self.InputFrameOrField).field("PastFrames", &self.PastFrames).field("FutureFrames", &self.FutureFrames).field("ppPastSurfaces", &self.ppPastSurfaces).field("pInputSurface", &self.pInputSurface).field("ppFutureSurfaces", &self.ppFutureSurfaces).finish()
    }
}
impl ::core::default::Default for DXVAHD_STREAM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_STREAM_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_ALPHA_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_ALPHA_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Alpha == other.Alpha
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_ALPHA_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_ALPHA_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_ALPHA_DATA").field("Enable", &self.Enable).field("Alpha", &self.Alpha).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.SourceAspectRatio == other.SourceAspectRatio && self.DestinationAspectRatio == other.DestinationAspectRatio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_ASPECT_RATIO_DATA").field("Enable", &self.Enable).field("SourceAspectRatio", &self.SourceAspectRatio).field("DestinationAspectRatio", &self.DestinationAspectRatio).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_D3DFORMAT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_D3DFORMAT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_D3DFORMAT_DATA {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_D3DFORMAT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_D3DFORMAT_DATA").field("Format", &self.Format).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.DestinationRect == other.DestinationRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_DESTINATION_RECT_DATA").field("Enable", &self.Enable).field("DestinationRect", &self.DestinationRect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_FILTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_FILTER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Level == other.Level
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_FILTER_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_FILTER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_FILTER_DATA").field("Enable", &self.Enable).field("Level", &self.Level).finish()
    }
}
impl ::core::default::Default for DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FrameFormat == other.FrameFormat
    }
}
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {}
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_FRAME_FORMAT_DATA").field("FrameFormat", &self.FrameFormat).finish()
    }
}
impl ::core::default::Default for DXVAHD_STREAM_STATE_INPUT_COLOR_SPACE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_LUMA_KEY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_LUMA_KEY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Lower == other.Lower && self.Upper == other.Upper
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_LUMA_KEY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_LUMA_KEY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_LUMA_KEY_DATA").field("Enable", &self.Enable).field("Lower", &self.Lower).field("Upper", &self.Upper).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatFrame == other.RepeatFrame && self.OutputRate == other.OutputRate && self.CustomRate == other.CustomRate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_OUTPUT_RATE_DATA").field("RepeatFrame", &self.RepeatFrame).field("OutputRate", &self.OutputRate).field("CustomRate", &self.CustomRate).finish()
    }
}
impl ::core::default::Default for DXVAHD_STREAM_STATE_PALETTE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_PALETTE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.pEntries == other.pEntries
    }
}
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_PALETTE_DATA {}
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_PALETTE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_PALETTE_DATA").field("Count", &self.Count).field("pEntries", &self.pEntries).finish()
    }
}
impl ::core::default::Default for DXVAHD_STREAM_STATE_PRIVATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_PRIVATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.DataSize == other.DataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_PRIVATE_DATA {}
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_PRIVATE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_PRIVATE_DATA").field("Guid", &self.Guid).field("DataSize", &self.DataSize).field("pData", &self.pData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.ITelecineFlags == other.ITelecineFlags && self.Frames == other.Frames && self.InputField == other.InputField
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_PRIVATE_IVTC_DATA").field("Enable", &self.Enable).field("ITelecineFlags", &self.ITelecineFlags).field("Frames", &self.Frames).field("InputField", &self.InputField).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.SourceRect == other.SourceRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVAHD_STREAM_STATE_SOURCE_RECT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_STREAM_STATE_SOURCE_RECT_DATA").field("Enable", &self.Enable).field("SourceRect", &self.SourceRect).finish()
    }
}
impl ::core::default::Default for DXVAHD_SURFACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVAHD_SURFACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVAHD_SURFACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVAHD_VPCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVAHD_VPCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.VPGuid == other.VPGuid && self.PastFrames == other.PastFrames && self.FutureFrames == other.FutureFrames && self.ProcessorCaps == other.ProcessorCaps && self.ITelecineCaps == other.ITelecineCaps && self.CustomRateCount == other.CustomRateCount
    }
}
impl ::core::cmp::Eq for DXVAHD_VPCAPS {}
impl ::core::fmt::Debug for DXVAHD_VPCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_VPCAPS").field("VPGuid", &self.VPGuid).field("PastFrames", &self.PastFrames).field("FutureFrames", &self.FutureFrames).field("ProcessorCaps", &self.ProcessorCaps).field("ITelecineCaps", &self.ITelecineCaps).field("CustomRateCount", &self.CustomRateCount).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVAHD_VPDEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVAHD_VPDEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceType == other.DeviceType && self.DeviceCaps == other.DeviceCaps && self.FeatureCaps == other.FeatureCaps && self.FilterCaps == other.FilterCaps && self.InputFormatCaps == other.InputFormatCaps && self.InputPool == other.InputPool && self.OutputFormatCount == other.OutputFormatCount && self.InputFormatCount == other.InputFormatCount && self.VideoProcessorCount == other.VideoProcessorCount && self.MaxInputStreams == other.MaxInputStreams && self.MaxStreamStates == other.MaxStreamStates
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVAHD_VPDEVCAPS {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVAHD_VPDEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAHD_VPDEVCAPS")
            .field("DeviceType", &self.DeviceType)
            .field("DeviceCaps", &self.DeviceCaps)
            .field("FeatureCaps", &self.FeatureCaps)
            .field("FilterCaps", &self.FilterCaps)
            .field("InputFormatCaps", &self.InputFormatCaps)
            .field("InputPool", &self.InputPool)
            .field("OutputFormatCount", &self.OutputFormatCount)
            .field("InputFormatCount", &self.InputFormatCount)
            .field("VideoProcessorCount", &self.VideoProcessorCount)
            .field("MaxInputStreams", &self.MaxInputStreams)
            .field("MaxStreamStates", &self.MaxStreamStates)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVAUncompDataInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVAUncompDataInfo {
    fn eq(&self, other: &Self) -> bool {
        self.UncompWidth == other.UncompWidth && self.UncompHeight == other.UncompHeight && self.UncompFormat == other.UncompFormat
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVAUncompDataInfo {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVAUncompDataInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVAUncompDataInfo").field("UncompWidth", &self.UncompWidth).field("UncompHeight", &self.UncompHeight).field("UncompFormat", &self.UncompFormat).finish()
    }
}
impl ::core::default::Default for DXVA_AYUVsample2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_AYUVsample2 {
    fn eq(&self, other: &Self) -> bool {
        self.bCrValue == other.bCrValue && self.bCbValue == other.bCbValue && self.bY_Value == other.bY_Value && self.bSampleAlpha8 == other.bSampleAlpha8
    }
}
impl ::core::cmp::Eq for DXVA_AYUVsample2 {}
impl ::core::fmt::Debug for DXVA_AYUVsample2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_AYUVsample2").field("bCrValue", &self.bCrValue).field("bCbValue", &self.bCbValue).field("bY_Value", &self.bY_Value).field("bSampleAlpha8", &self.bSampleAlpha8).finish()
    }
}
impl ::core::default::Default for DXVA_BufferDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DXVA_COPPCommand {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_COPPCommand {
    fn eq(&self, other: &Self) -> bool {
        self.macKDI == other.macKDI && self.guidCommandID == other.guidCommandID && self.dwSequence == other.dwSequence && self.cbSizeData == other.cbSizeData && self.CommandData == other.CommandData
    }
}
impl ::core::cmp::Eq for DXVA_COPPCommand {}
impl ::core::fmt::Debug for DXVA_COPPCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_COPPCommand").field("macKDI", &self.macKDI).field("guidCommandID", &self.guidCommandID).field("dwSequence", &self.dwSequence).field("cbSizeData", &self.cbSizeData).field("CommandData", &self.CommandData).finish()
    }
}
impl ::core::default::Default for DXVA_COPPSignature {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_COPPSignature {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for DXVA_COPPSignature {}
impl ::core::fmt::Debug for DXVA_COPPSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_COPPSignature").field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for DXVA_COPPStatusInput {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_COPPStatusInput {
    fn eq(&self, other: &Self) -> bool {
        self.rApp == other.rApp && self.guidStatusRequestID == other.guidStatusRequestID && self.dwSequence == other.dwSequence && self.cbSizeData == other.cbSizeData && self.StatusData == other.StatusData
    }
}
impl ::core::cmp::Eq for DXVA_COPPStatusInput {}
impl ::core::fmt::Debug for DXVA_COPPStatusInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_COPPStatusInput").field("rApp", &self.rApp).field("guidStatusRequestID", &self.guidStatusRequestID).field("dwSequence", &self.dwSequence).field("cbSizeData", &self.cbSizeData).field("StatusData", &self.StatusData).finish()
    }
}
impl ::core::default::Default for DXVA_COPPStatusOutput {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_COPPStatusOutput {
    fn eq(&self, other: &Self) -> bool {
        self.macKDI == other.macKDI && self.cbSizeData == other.cbSizeData && self.COPPStatus == other.COPPStatus
    }
}
impl ::core::cmp::Eq for DXVA_COPPStatusOutput {}
impl ::core::fmt::Debug for DXVA_COPPStatusOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_COPPStatusOutput").field("macKDI", &self.macKDI).field("cbSizeData", &self.cbSizeData).field("COPPStatus", &self.COPPStatus).finish()
    }
}
impl ::core::default::Default for DXVA_ConfigPictureDecode {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_DeinterlaceBlt {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_DeinterlaceBlt {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Reserved == other.Reserved && self.rtTarget == other.rtTarget && self.DstRect == other.DstRect && self.SrcRect == other.SrcRect && self.NumSourceSurfaces == other.NumSourceSurfaces && self.Alpha == other.Alpha && self.Source == other.Source
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_DeinterlaceBlt {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_DeinterlaceBlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_DeinterlaceBlt").field("Size", &self.Size).field("Reserved", &self.Reserved).field("rtTarget", &self.rtTarget).field("DstRect", &self.DstRect).field("SrcRect", &self.SrcRect).field("NumSourceSurfaces", &self.NumSourceSurfaces).field("Alpha", &self.Alpha).field("Source", &self.Source).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_DeinterlaceBltEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_DeinterlaceBltEx {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.BackgroundColor == other.BackgroundColor && self.rcTarget == other.rcTarget && self.rtTarget == other.rtTarget && self.NumSourceSurfaces == other.NumSourceSurfaces && self.Alpha == other.Alpha && self.Source == other.Source && self.DestinationFormat == other.DestinationFormat && self.DestinationFlags == other.DestinationFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_DeinterlaceBltEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_DeinterlaceBltEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_DeinterlaceBltEx").field("Size", &self.Size).field("BackgroundColor", &self.BackgroundColor).field("rcTarget", &self.rcTarget).field("rtTarget", &self.rtTarget).field("NumSourceSurfaces", &self.NumSourceSurfaces).field("Alpha", &self.Alpha).field("Source", &self.Source).field("DestinationFormat", &self.DestinationFormat).field("DestinationFlags", &self.DestinationFlags).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_DeinterlaceBltEx32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_DeinterlaceBltEx32 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.BackgroundColor == other.BackgroundColor && self.rcTarget == other.rcTarget && self.rtTarget == other.rtTarget && self.NumSourceSurfaces == other.NumSourceSurfaces && self.Alpha == other.Alpha && self.Source == other.Source && self.DestinationFormat == other.DestinationFormat && self.DestinationFlags == other.DestinationFlags
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_DeinterlaceBltEx32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_DeinterlaceBltEx32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_DeinterlaceBltEx32").field("Size", &self.Size).field("BackgroundColor", &self.BackgroundColor).field("rcTarget", &self.rcTarget).field("rtTarget", &self.rtTarget).field("NumSourceSurfaces", &self.NumSourceSurfaces).field("Alpha", &self.Alpha).field("Source", &self.Source).field("DestinationFormat", &self.DestinationFormat).field("DestinationFlags", &self.DestinationFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA_DeinterlaceCaps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVA_DeinterlaceCaps {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.NumPreviousOutputFrames == other.NumPreviousOutputFrames && self.InputPool == other.InputPool && self.NumForwardRefSamples == other.NumForwardRefSamples && self.NumBackwardRefSamples == other.NumBackwardRefSamples && self.d3dOutputFormat == other.d3dOutputFormat && self.VideoProcessingCaps == other.VideoProcessingCaps && self.DeinterlaceTechnology == other.DeinterlaceTechnology
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVA_DeinterlaceCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVA_DeinterlaceCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_DeinterlaceCaps").field("Size", &self.Size).field("NumPreviousOutputFrames", &self.NumPreviousOutputFrames).field("InputPool", &self.InputPool).field("NumForwardRefSamples", &self.NumForwardRefSamples).field("NumBackwardRefSamples", &self.NumBackwardRefSamples).field("d3dOutputFormat", &self.d3dOutputFormat).field("VideoProcessingCaps", &self.VideoProcessingCaps).field("DeinterlaceTechnology", &self.DeinterlaceTechnology).finish()
    }
}
impl ::core::default::Default for DXVA_DeinterlaceQueryAvailableModes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_DeinterlaceQueryAvailableModes {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.NumGuids == other.NumGuids && self.Guids == other.Guids
    }
}
impl ::core::cmp::Eq for DXVA_DeinterlaceQueryAvailableModes {}
impl ::core::fmt::Debug for DXVA_DeinterlaceQueryAvailableModes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_DeinterlaceQueryAvailableModes").field("Size", &self.Size).field("NumGuids", &self.NumGuids).field("Guids", &self.Guids).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA_DeinterlaceQueryModeCaps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVA_DeinterlaceQueryModeCaps {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Guid == other.Guid && self.VideoDesc == other.VideoDesc
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVA_DeinterlaceQueryModeCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVA_DeinterlaceQueryModeCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_DeinterlaceQueryModeCaps").field("Size", &self.Size).field("Guid", &self.Guid).field("VideoDesc", &self.VideoDesc).finish()
    }
}
impl ::core::default::Default for DXVA_DeinterlaceTech {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_DeinterlaceTech {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_DeinterlaceTech").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_DestinationFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_DestinationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_DestinationFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_ExtendedFormat {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_ExtendedFormat {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DXVA_ExtendedFormat {}
impl ::core::fmt::Debug for DXVA_ExtendedFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_ExtendedFormat").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DXVA_Frequency {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_Frequency {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DXVA_Frequency {}
impl ::core::fmt::Debug for DXVA_Frequency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_Frequency").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for DXVA_NominalRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_NominalRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_NominalRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_PictureParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_ProcAmpControlBlt {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_ProcAmpControlBlt {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.DstRect == other.DstRect && self.SrcRect == other.SrcRect && self.Alpha == other.Alpha && self.Brightness == other.Brightness && self.Contrast == other.Contrast && self.Hue == other.Hue && self.Saturation == other.Saturation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_ProcAmpControlBlt {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_ProcAmpControlBlt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_ProcAmpControlBlt").field("Size", &self.Size).field("DstRect", &self.DstRect).field("SrcRect", &self.SrcRect).field("Alpha", &self.Alpha).field("Brightness", &self.Brightness).field("Contrast", &self.Contrast).field("Hue", &self.Hue).field("Saturation", &self.Saturation).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA_ProcAmpControlCaps {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVA_ProcAmpControlCaps {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.InputPool == other.InputPool && self.d3dOutputFormat == other.d3dOutputFormat && self.ProcAmpControlProps == other.ProcAmpControlProps && self.VideoProcessingCaps == other.VideoProcessingCaps
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVA_ProcAmpControlCaps {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVA_ProcAmpControlCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_ProcAmpControlCaps").field("Size", &self.Size).field("InputPool", &self.InputPool).field("d3dOutputFormat", &self.d3dOutputFormat).field("ProcAmpControlProps", &self.ProcAmpControlProps).field("VideoProcessingCaps", &self.VideoProcessingCaps).finish()
    }
}
impl ::core::default::Default for DXVA_ProcAmpControlProp {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_ProcAmpControlProp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_ProcAmpControlProp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA_ProcAmpControlQueryRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVA_ProcAmpControlQueryRange {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ProcAmpControlProp == other.ProcAmpControlProp && self.VideoDesc == other.VideoDesc
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVA_ProcAmpControlQueryRange {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVA_ProcAmpControlQueryRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_ProcAmpControlQueryRange").field("Size", &self.Size).field("ProcAmpControlProp", &self.ProcAmpControlProp).field("VideoDesc", &self.VideoDesc).finish()
    }
}
impl ::core::default::Default for DXVA_SampleFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_SampleFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_SampleFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_SampleFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_SampleFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_SampleFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_VideoChromaSubsampling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_VideoChromaSubsampling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_VideoChromaSubsampling").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for DXVA_VideoDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::PartialEq for DXVA_VideoDesc {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SampleWidth == other.SampleWidth && self.SampleHeight == other.SampleHeight && self.SampleFormat == other.SampleFormat && self.d3dFormat == other.d3dFormat && self.InputSampleFreq == other.InputSampleFreq && self.OutputFrameFreq == other.OutputFrameFreq
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::cmp::Eq for DXVA_VideoDesc {}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::fmt::Debug for DXVA_VideoDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_VideoDesc").field("Size", &self.Size).field("SampleWidth", &self.SampleWidth).field("SampleHeight", &self.SampleHeight).field("SampleFormat", &self.SampleFormat).field("d3dFormat", &self.d3dFormat).field("InputSampleFreq", &self.InputSampleFreq).field("OutputFrameFreq", &self.OutputFrameFreq).finish()
    }
}
impl ::core::default::Default for DXVA_VideoLighting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_VideoLighting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_VideoLighting").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_VideoPrimaries {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_VideoPrimaries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_VideoPrimaries").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_VideoProcessCaps {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_VideoProcessCaps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_VideoProcessCaps").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_VideoPropertyRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_VideoPropertyRange {
    fn eq(&self, other: &Self) -> bool {
        self.MinValue == other.MinValue && self.MaxValue == other.MaxValue && self.DefaultValue == other.DefaultValue && self.StepSize == other.StepSize
    }
}
impl ::core::cmp::Eq for DXVA_VideoPropertyRange {}
impl ::core::fmt::Debug for DXVA_VideoPropertyRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_VideoPropertyRange").field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).field("DefaultValue", &self.DefaultValue).field("StepSize", &self.StepSize).finish()
    }
}
impl ::core::default::Default for DXVA_VideoSample {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXVA_VideoSample {
    fn eq(&self, other: &Self) -> bool {
        self.rtStart == other.rtStart && self.rtEnd == other.rtEnd && self.SampleFormat == other.SampleFormat && self.lpDDSSrcSurface == other.lpDDSSrcSurface
    }
}
impl ::core::cmp::Eq for DXVA_VideoSample {}
impl ::core::fmt::Debug for DXVA_VideoSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_VideoSample").field("rtStart", &self.rtStart).field("rtEnd", &self.rtEnd).field("SampleFormat", &self.SampleFormat).field("lpDDSSrcSurface", &self.lpDDSSrcSurface).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_VideoSample2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_VideoSample2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Reserved == other.Reserved && self.rtStart == other.rtStart && self.rtEnd == other.rtEnd && self.SampleFormat == other.SampleFormat && self.SampleFlags == other.SampleFlags && self.lpDDSSrcSurface == other.lpDDSSrcSurface && self.rcSrc == other.rcSrc && self.rcDst == other.rcDst && self.Palette == other.Palette
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_VideoSample2 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_VideoSample2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_VideoSample2").field("Size", &self.Size).field("Reserved", &self.Reserved).field("rtStart", &self.rtStart).field("rtEnd", &self.rtEnd).field("SampleFormat", &self.SampleFormat).field("SampleFlags", &self.SampleFlags).field("lpDDSSrcSurface", &self.lpDDSSrcSurface).field("rcSrc", &self.rcSrc).field("rcDst", &self.rcDst).field("Palette", &self.Palette).finish()
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_VideoSample2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_VideoSample2 {
    fn eq(&self, other: &Self) -> bool {
        self.rtStart == other.rtStart && self.rtEnd == other.rtEnd && self.SampleFormat == other.SampleFormat && self.SampleFlags == other.SampleFlags && self.lpDDSSrcSurface == other.lpDDSSrcSurface && self.rcSrc == other.rcSrc && self.rcDst == other.rcDst && self.Palette == other.Palette
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_VideoSample2 {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_VideoSample2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_VideoSample2").field("rtStart", &self.rtStart).field("rtEnd", &self.rtEnd).field("SampleFormat", &self.SampleFormat).field("SampleFlags", &self.SampleFlags).field("lpDDSSrcSurface", &self.lpDDSSrcSurface).field("rcSrc", &self.rcSrc).field("rcDst", &self.rcDst).field("Palette", &self.Palette).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXVA_VideoSample32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXVA_VideoSample32 {
    fn eq(&self, other: &Self) -> bool {
        self.rtStart == other.rtStart && self.rtEnd == other.rtEnd && self.SampleFormat == other.SampleFormat && self.SampleFlags == other.SampleFlags && self.lpDDSSrcSurface == other.lpDDSSrcSurface && self.rcSrc == other.rcSrc && self.rcDst == other.rcDst && self.Palette == other.Palette
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXVA_VideoSample32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXVA_VideoSample32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXVA_VideoSample32").field("rtStart", &self.rtStart).field("rtEnd", &self.rtEnd).field("SampleFormat", &self.SampleFormat).field("SampleFlags", &self.SampleFlags).field("lpDDSSrcSurface", &self.lpDDSSrcSurface).field("rcSrc", &self.rcSrc).field("rcDst", &self.rcDst).field("Palette", &self.Palette).finish()
    }
}
impl ::core::default::Default for DXVA_VideoTransferFunction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_VideoTransferFunction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_VideoTransferFunction").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXVA_VideoTransferMatrix {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXVA_VideoTransferMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXVA_VideoTransferMatrix").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceStreamState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceStreamState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceStreamState").field(&self.0).finish()
    }
}
impl ::core::default::Default for DigitalWindowSetting {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DigitalWindowSetting {
    fn eq(&self, other: &Self) -> bool {
        self.OriginX == other.OriginX && self.OriginY == other.OriginY && self.WindowSize == other.WindowSize
    }
}
impl ::core::cmp::Eq for DigitalWindowSetting {}
impl ::core::fmt::Debug for DigitalWindowSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DigitalWindowSetting").field("OriginX", &self.OriginX).field("OriginY", &self.OriginY).field("WindowSize", &self.WindowSize).finish()
    }
}
impl ::core::default::Default for EAllocationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAllocationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAllocationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for EVRFilterConfigPrefs {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EVRFilterConfigPrefs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EVRFilterConfigPrefs").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_ACCESSMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_ACCESSMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_ACCESSMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_OPENMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_OPENMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_OPENMODE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAdvancedMediaCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdvancedMediaCapture {}
impl ::core::fmt::Debug for IAdvancedMediaCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdvancedMediaCapture").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAdvancedMediaCaptureInitializationSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdvancedMediaCaptureInitializationSettings {}
impl ::core::fmt::Debug for IAdvancedMediaCaptureInitializationSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdvancedMediaCaptureInitializationSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAdvancedMediaCaptureSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAdvancedMediaCaptureSettings {}
impl ::core::fmt::Debug for IAdvancedMediaCaptureSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAdvancedMediaCaptureSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAudioSourceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAudioSourceProvider {}
impl ::core::fmt::Debug for IAudioSourceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAudioSourceProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IClusterDetector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClusterDetector {}
impl ::core::fmt::Debug for IClusterDetector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClusterDetector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICodecAPI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICodecAPI {}
impl ::core::fmt::Debug for ICodecAPI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICodecAPI").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecodeCommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecodeCommandList {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecodeCommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecodeCommandList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecodeCommandList {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecodeCommandList1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecodeCommandList1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecodeCommandList1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecodeCommandList1").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecodeCommandList1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12CommandAllocator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn DecodeFrame<P0>(&self, pdecoder: P0, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecodeFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), poutputarguments, pinputarguments)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecodeCommandList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecodeCommandList2 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecodeCommandList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecodeCommandList2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecodeCommandList2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12CommandAllocator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn DecodeFrame<P0>(&self, pdecoder: P0, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecodeFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), poutputarguments, pinputarguments)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn DecodeFrame1<P0>(&self, pdecoder: P0, poutputarguments: *const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, pinputarguments: *const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecodeFrame1)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), poutputarguments, pinputarguments)
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecoder {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecoder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecoder {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecoder1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecoder1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecoder1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecoder1").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecoder1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_DECODER_DESC {
        let mut result__: D3D12_VIDEO_DECODER_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecoderHeap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecoderHeap {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecoderHeap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecoderHeap").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecoderHeap {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoDecoderHeap1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoDecoderHeap1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoDecoderHeap1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDecoderHeap1").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoDecoderHeap1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> D3D12_VIDEO_DECODER_HEAP_DESC {
        let mut result__: D3D12_VIDEO_DECODER_HEAP_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for ID3D12VideoDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12VideoDevice {}
impl ::core::fmt::Debug for ID3D12VideoDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12VideoDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12VideoDevice1 {}
impl ::core::fmt::Debug for ID3D12VideoDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDevice1").field(&self.0).finish()
    }
}
impl ID3D12VideoDevice1 {
    pub unsafe fn CheckFeatureSupport(&self, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), featurevideo, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateVideoDecoder<T>(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoDecoder)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoDecoderHeap<T>(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoDecoderHeap)(::windows::core::Vtable::as_raw(self), pvideodecoderheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoProcessor<T>(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, pinputstreamdescs: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoProcessor)(::windows::core::Vtable::as_raw(self), nodemask, poutputstreamdesc, pinputstreamdescs.len() as _, ::core::mem::transmute(pinputstreamdescs.as_ptr()), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D12VideoDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12VideoDevice2 {}
impl ::core::fmt::Debug for ID3D12VideoDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDevice2").field(&self.0).finish()
    }
}
impl ID3D12VideoDevice2 {
    pub unsafe fn CheckFeatureSupport(&self, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), featurevideo, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateVideoDecoder<T>(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoDecoder)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoDecoderHeap<T>(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoDecoderHeap)(::windows::core::Vtable::as_raw(self), pvideodecoderheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoProcessor<T>(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, pinputstreamdescs: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoProcessor)(::windows::core::Vtable::as_raw(self), nodemask, poutputstreamdesc, pinputstreamdescs.len() as _, ::core::mem::transmute(pinputstreamdescs.as_ptr()), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoMotionEstimator<P0, T>(&self, pdesc: *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoMotionEstimator)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoMotionVectorHeap<P0, T>(&self, pdesc: *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoMotionVectorHeap)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D12VideoDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12VideoDevice3 {}
impl ::core::fmt::Debug for ID3D12VideoDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoDevice3").field(&self.0).finish()
    }
}
impl ID3D12VideoDevice3 {
    pub unsafe fn CheckFeatureSupport(&self, featurevideo: D3D12_FEATURE_VIDEO, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), featurevideo, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateVideoDecoder<T>(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateVideoDecoder)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoDecoderHeap<T>(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateVideoDecoderHeap)(::windows::core::Vtable::as_raw(self), pvideodecoderheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoProcessor<T>(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, pinputstreamdescs: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateVideoProcessor)(::windows::core::Vtable::as_raw(self), nodemask, poutputstreamdesc, pinputstreamdescs.len() as _, ::core::mem::transmute(pinputstreamdescs.as_ptr()), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoMotionEstimator<P0, T>(&self, pdesc: *const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoMotionEstimator)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoMotionVectorHeap<P0, T>(&self, pdesc: *const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoMotionVectorHeap)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateVideoDecoder1<P0, T>(&self, pdesc: *const D3D12_VIDEO_DECODER_DESC, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoDecoder1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoDecoderHeap1<P0, T>(&self, pvideodecoderheapdesc: *const D3D12_VIDEO_DECODER_HEAP_DESC, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoDecoderHeap1)(::windows::core::Vtable::as_raw(self), pvideodecoderheapdesc, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateVideoProcessor1<P0, T>(&self, nodemask: u32, poutputstreamdesc: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC, pinputstreamdescs: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC], pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoProcessor1)(::windows::core::Vtable::as_raw(self), nodemask, poutputstreamdesc, pinputstreamdescs.len() as _, ::core::mem::transmute(pinputstreamdescs.as_ptr()), pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn CreateVideoExtensionCommand<P0, T>(&self, pdesc: *const D3D12_VIDEO_EXTENSION_COMMAND_DESC, pcreationparameters: *const ::core::ffi::c_void, creationparametersdatasizeinbytes: usize, pprotectedresourcesession: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoExtensionCommand)(::windows::core::Vtable::as_raw(self), pdesc, pcreationparameters, creationparametersdatasizeinbytes, pprotectedresourcesession.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ExecuteExtensionCommand<P0>(&self, pextensioncommand: P0, pexecutionparameters: *const ::core::ffi::c_void, executionparameterssizeinbytes: usize, poutputdata: *mut ::core::ffi::c_void, outputdatasizeinbytes: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoExtensionCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecuteExtensionCommand)(::windows::core::Vtable::as_raw(self), pextensioncommand.into().abi(), pexecutionparameters, executionparameterssizeinbytes, poutputdata, outputdatasizeinbytes).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoEncodeCommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoEncodeCommandList {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoEncodeCommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoEncodeCommandList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoEncodeCommandList {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoEncodeCommandList1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoEncodeCommandList1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoEncodeCommandList1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoEncodeCommandList1").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoEncodeCommandList1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12CommandAllocator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EstimateMotion<P0>(&self, pmotionestimator: P0, poutputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pinputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoMotionEstimator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EstimateMotion)(::windows::core::Vtable::as_raw(self), pmotionestimator.into().abi(), poutputarguments, pinputarguments)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveMotionVectorHeap(&self, poutputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pinputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT) {
        (::windows::core::Vtable::vtable(self).base__.ResolveMotionVectorHeap)(::windows::core::Vtable::as_raw(self), poutputarguments, pinputarguments)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pprotectedresourcesession.into().abi())
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoEncodeCommandList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoEncodeCommandList2 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoEncodeCommandList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoEncodeCommandList2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoEncodeCommandList2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12CommandAllocator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EstimateMotion<P0>(&self, pmotionestimator: P0, poutputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pinputarguments: *const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoMotionEstimator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EstimateMotion)(::windows::core::Vtable::as_raw(self), pmotionestimator.into().abi(), poutputarguments, pinputarguments)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveMotionVectorHeap(&self, poutputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pinputarguments: *const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT) {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveMotionVectorHeap)(::windows::core::Vtable::as_raw(self), poutputarguments, pinputarguments)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pprotectedresourcesession.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn InitializeExtensionCommand<P0>(&self, pextensioncommand: P0, pinitializationparameters: *const ::core::ffi::c_void, initializationparameterssizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoExtensionCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeExtensionCommand)(::windows::core::Vtable::as_raw(self), pextensioncommand.into().abi(), pinitializationparameters, initializationparameterssizeinbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ExecuteExtensionCommand<P0>(&self, pextensioncommand: P0, pexecutionparameters: *const ::core::ffi::c_void, executionparameterssizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoExtensionCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecuteExtensionCommand)(::windows::core::Vtable::as_raw(self), pextensioncommand.into().abi(), pexecutionparameters, executionparameterssizeinbytes)
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoEncoder {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoEncoder").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoEncoder {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoEncoderHeap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoEncoderHeap {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoEncoderHeap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoEncoderHeap").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoEncoderHeap {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoExtensionCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoExtensionCommand {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoExtensionCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoExtensionCommand").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoExtensionCommand {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoMotionEstimator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoMotionEstimator {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoMotionEstimator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoMotionEstimator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoMotionEstimator {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoMotionVectorHeap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoMotionVectorHeap {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoMotionVectorHeap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoMotionVectorHeap").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoMotionVectorHeap {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoProcessCommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoProcessCommandList {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoProcessCommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoProcessCommandList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoProcessCommandList {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoProcessCommandList1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoProcessCommandList1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoProcessCommandList1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoProcessCommandList1").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoProcessCommandList1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12CommandAllocator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn ProcessFrames<P0>(&self, pvideoprocessor: P0, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, pinputarguments: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ProcessFrames)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), poutputarguments, pinputarguments.len() as _, ::core::mem::transmute(pinputarguments.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoProcessCommandList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoProcessCommandList2 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoProcessCommandList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoProcessCommandList2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoProcessCommandList2 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetType(&self) -> super::super::Graphics::Direct3D12::D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn Reset<P0>(&self, pallocator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12CommandAllocator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi()).ok()
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[super::super::Graphics::Direct3D12::D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: super::super::Graphics::Direct3D12::D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: super::super::Graphics::Direct3D12::D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::Graphics::Direct3D12::ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn ProcessFrames<P0>(&self, pvideoprocessor: P0, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, pinputarguments: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ProcessFrames)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), poutputarguments, pinputarguments.len() as _, ::core::mem::transmute(pinputarguments.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const super::super::Graphics::Direct3D12::D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D12"))]
    pub unsafe fn ProcessFrames1<P0>(&self, pvideoprocessor: P0, poutputarguments: *const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, pinputarguments: &[D3D12_VIDEO_PROCESS_INPUT_STREAM_ARGUMENTS1])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ProcessFrames1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), poutputarguments, pinputarguments.len() as _, ::core::mem::transmute(pinputarguments.as_ptr()))
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoProcessor {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoProcessor").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoProcessor {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::PartialEq for ID3D12VideoProcessor1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::cmp::Eq for ID3D12VideoProcessor1 {}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ::core::fmt::Debug for ID3D12VideoProcessor1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VideoProcessor1").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D12")]
impl ID3D12VideoProcessor1 {
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D12\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetNodeMask(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNodeMask)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumInputStreamDescs(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNumInputStreamDescs)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetInputStreamDescs(&self, pinputstreamdescs: &mut [D3D12_VIDEO_PROCESS_INPUT_STREAM_DESC]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInputStreamDescs)(::windows::core::Vtable::as_raw(self), pinputstreamdescs.len() as _, ::core::mem::transmute(pinputstreamdescs.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetOutputStreamDesc(&self) -> D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC {
        let mut result__: D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputStreamDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for IDXVAHD_Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXVAHD_Device {}
impl ::core::fmt::Debug for IDXVAHD_Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXVAHD_Device").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXVAHD_VideoProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXVAHD_VideoProcessor {}
impl ::core::fmt::Debug for IDXVAHD_VideoProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXVAHD_VideoProcessor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3D9ExOverlayExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3D9ExOverlayExtension {}
impl ::core::fmt::Debug for IDirect3D9ExOverlayExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3D9ExOverlayExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DAuthenticatedChannel9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DAuthenticatedChannel9 {}
impl ::core::fmt::Debug for IDirect3DAuthenticatedChannel9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DAuthenticatedChannel9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DCryptoSession9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DCryptoSession9 {}
impl ::core::fmt::Debug for IDirect3DCryptoSession9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DCryptoSession9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9Video {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9Video {}
impl ::core::fmt::Debug for IDirect3DDevice9Video {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9Video").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DDeviceManager9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDeviceManager9 {}
impl ::core::fmt::Debug for IDirect3DDeviceManager9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDeviceManager9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectXVideoAccelerationService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectXVideoAccelerationService {}
impl ::core::fmt::Debug for IDirectXVideoAccelerationService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectXVideoAccelerationService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectXVideoDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectXVideoDecoder {}
impl ::core::fmt::Debug for IDirectXVideoDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectXVideoDecoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectXVideoDecoderService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectXVideoDecoderService {}
impl ::core::fmt::Debug for IDirectXVideoDecoderService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectXVideoDecoderService").field(&self.0).finish()
    }
}
impl IDirectXVideoDecoderService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, backbuffers: u32, format: super::super::Graphics::Direct3D9::D3DFORMAT, pool: super::super::Graphics::Direct3D9::D3DPOOL, usage: u32, dxvatype: DXVA2_VideoRenderTargetType, ppsurface: *mut ::core::option::Option<super::super::Graphics::Direct3D9::IDirect3DSurface9>, psharedhandle: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSurface)(::windows::core::Vtable::as_raw(self), width, height, backbuffers, format, pool, usage, dxvatype, ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectXVideoMemoryConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectXVideoMemoryConfiguration {}
impl ::core::fmt::Debug for IDirectXVideoMemoryConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectXVideoMemoryConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectXVideoProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectXVideoProcessor {}
impl ::core::fmt::Debug for IDirectXVideoProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectXVideoProcessor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectXVideoProcessorService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectXVideoProcessorService {}
impl ::core::fmt::Debug for IDirectXVideoProcessorService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectXVideoProcessorService").field(&self.0).finish()
    }
}
impl IDirectXVideoProcessorService {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct3D9\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9"))]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, backbuffers: u32, format: super::super::Graphics::Direct3D9::D3DFORMAT, pool: super::super::Graphics::Direct3D9::D3DPOOL, usage: u32, dxvatype: DXVA2_VideoRenderTargetType, ppsurface: *mut ::core::option::Option<super::super::Graphics::Direct3D9::IDirect3DSurface9>, psharedhandle: ::core::option::Option<*mut super::super::Foundation::HANDLE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSurface)(::windows::core::Vtable::as_raw(self), width, height, backbuffers, format, pool, usage, dxvatype, ::core::mem::transmute(ppsurface), ::core::mem::transmute(psharedhandle.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IEVRFilterConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEVRFilterConfig {}
impl ::core::fmt::Debug for IEVRFilterConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEVRFilterConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEVRFilterConfigEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEVRFilterConfigEx {}
impl ::core::fmt::Debug for IEVRFilterConfigEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEVRFilterConfigEx").field(&self.0).finish()
    }
}
impl IEVRFilterConfigEx {
    pub unsafe fn SetNumberOfStreams(&self, dwmaxstreams: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNumberOfStreams)(::windows::core::Vtable::as_raw(self), dwmaxstreams).ok()
    }
    pub unsafe fn GetNumberOfStreams(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNumberOfStreams)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IEVRTrustedVideoPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEVRTrustedVideoPlugin {}
impl ::core::fmt::Debug for IEVRTrustedVideoPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEVRTrustedVideoPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEVRVideoStreamControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEVRVideoStreamControl {}
impl ::core::fmt::Debug for IEVRVideoStreamControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEVRVideoStreamControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileClient {}
impl ::core::fmt::Debug for IFileClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFileIo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFileIo {}
impl ::core::fmt::Debug for IFileIo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFileIo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMF2DBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMF2DBuffer {}
impl ::core::fmt::Debug for IMF2DBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMF2DBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMF2DBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMF2DBuffer2 {}
impl ::core::fmt::Debug for IMF2DBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMF2DBuffer2").field(&self.0).finish()
    }
}
impl IMF2DBuffer2 {
    pub unsafe fn Lock2D(&self, ppbscanline0: *mut *mut u8, plpitch: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Lock2D)(::windows::core::Vtable::as_raw(self), ppbscanline0, plpitch).ok()
    }
    pub unsafe fn Unlock2D(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unlock2D)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetScanline0AndPitch(&self, pbscanline0: *mut *mut u8, plpitch: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetScanline0AndPitch)(::windows::core::Vtable::as_raw(self), pbscanline0, plpitch).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsContiguousFormat(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsContiguousFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContiguousLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContiguousLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContiguousCopyTo(&self, pbdestbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ContiguousCopyTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbdestbuffer.as_ptr()), pbdestbuffer.len() as _).ok()
    }
    pub unsafe fn ContiguousCopyFrom(&self, pbsrcbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ContiguousCopyFrom)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbsrcbuffer.as_ptr()), pbsrcbuffer.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for IMFASFContentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFContentInfo {}
impl ::core::fmt::Debug for IMFASFContentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFContentInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFASFIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFIndexer {}
impl ::core::fmt::Debug for IMFASFIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFIndexer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFASFMultiplexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFMultiplexer {}
impl ::core::fmt::Debug for IMFASFMultiplexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFMultiplexer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFASFMutualExclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFMutualExclusion {}
impl ::core::fmt::Debug for IMFASFMutualExclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFMutualExclusion").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFASFProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFProfile {}
impl ::core::fmt::Debug for IMFASFProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFProfile").field(&self.0).finish()
    }
}
impl IMFASFProfile {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFASFSplitter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFSplitter {}
impl ::core::fmt::Debug for IMFASFSplitter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFSplitter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFASFStreamConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFStreamConfig {}
impl ::core::fmt::Debug for IMFASFStreamConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFStreamConfig").field(&self.0).finish()
    }
}
impl IMFASFStreamConfig {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFASFStreamPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFStreamPrioritization {}
impl ::core::fmt::Debug for IMFASFStreamPrioritization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFStreamPrioritization").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFASFStreamSelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFASFStreamSelector {}
impl ::core::fmt::Debug for IMFASFStreamSelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFASFStreamSelector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFActivate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFActivate {}
impl ::core::fmt::Debug for IMFActivate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFActivate").field(&self.0).finish()
    }
}
impl IMFActivate {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFAsyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAsyncCallback {}
impl ::core::fmt::Debug for IMFAsyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAsyncCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFAsyncCallbackLogging {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAsyncCallbackLogging {}
impl ::core::fmt::Debug for IMFAsyncCallbackLogging {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAsyncCallbackLogging").field(&self.0).finish()
    }
}
impl IMFAsyncCallbackLogging {
    pub unsafe fn GetParameters(&self, pdwflags: *mut u32, pdwqueue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetParameters)(::windows::core::Vtable::as_raw(self), pdwflags, pdwqueue).ok()
    }
    pub unsafe fn Invoke<P0>(&self, pasyncresult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Invoke)(::windows::core::Vtable::as_raw(self), pasyncresult.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFAsyncResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAsyncResult {}
impl ::core::fmt::Debug for IMFAsyncResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAsyncResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAttributes {}
impl ::core::fmt::Debug for IMFAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAttributes").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFAudioMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAudioMediaType {}
impl ::core::fmt::Debug for IMFAudioMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAudioMediaType").field(&self.0).finish()
    }
}
impl IMFAudioMediaType {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
    pub unsafe fn GetMajorType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMajorType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCompressedFormat(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsCompressedFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsEqual<P0>(&self, pimediatype: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqual)(::windows::core::Vtable::as_raw(self), pimediatype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRepresentation(&self, guidrepresentation: ::windows::core::GUID, ppvrepresentation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRepresentation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidrepresentation), ppvrepresentation).ok()
    }
    pub unsafe fn FreeRepresentation(&self, guidrepresentation: ::windows::core::GUID, pvrepresentation: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreeRepresentation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidrepresentation), pvrepresentation).ok()
    }
}
impl ::core::cmp::PartialEq for IMFAudioPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAudioPolicy {}
impl ::core::fmt::Debug for IMFAudioPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAudioPolicy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFAudioStreamVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFAudioStreamVolume {}
impl ::core::fmt::Debug for IMFAudioStreamVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFAudioStreamVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFBufferListNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFBufferListNotify {}
impl ::core::fmt::Debug for IMFBufferListNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFBufferListNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFByteStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStream {}
impl ::core::fmt::Debug for IMFByteStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFByteStreamBuffering {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStreamBuffering {}
impl ::core::fmt::Debug for IMFByteStreamBuffering {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStreamBuffering").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFByteStreamCacheControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStreamCacheControl {}
impl ::core::fmt::Debug for IMFByteStreamCacheControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStreamCacheControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFByteStreamCacheControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStreamCacheControl2 {}
impl ::core::fmt::Debug for IMFByteStreamCacheControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStreamCacheControl2").field(&self.0).finish()
    }
}
impl IMFByteStreamCacheControl2 {
    pub unsafe fn StopBackgroundTransfer(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StopBackgroundTransfer)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFByteStreamHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStreamHandler {}
impl ::core::fmt::Debug for IMFByteStreamHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStreamHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFByteStreamProxyClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStreamProxyClassFactory {}
impl ::core::fmt::Debug for IMFByteStreamProxyClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStreamProxyClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFByteStreamTimeSeek {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFByteStreamTimeSeek {}
impl ::core::fmt::Debug for IMFByteStreamTimeSeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFByteStreamTimeSeek").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCameraOcclusionStateMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCameraOcclusionStateMonitor {}
impl ::core::fmt::Debug for IMFCameraOcclusionStateMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCameraOcclusionStateMonitor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCameraOcclusionStateReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCameraOcclusionStateReport {}
impl ::core::fmt::Debug for IMFCameraOcclusionStateReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCameraOcclusionStateReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCameraOcclusionStateReportCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCameraOcclusionStateReportCallback {}
impl ::core::fmt::Debug for IMFCameraOcclusionStateReportCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCameraOcclusionStateReportCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCameraSyncObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCameraSyncObject {}
impl ::core::fmt::Debug for IMFCameraSyncObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCameraSyncObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureEngine {}
impl ::core::fmt::Debug for IMFCaptureEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureEngineClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureEngineClassFactory {}
impl ::core::fmt::Debug for IMFCaptureEngineClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureEngineClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureEngineOnEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureEngineOnEventCallback {}
impl ::core::fmt::Debug for IMFCaptureEngineOnEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureEngineOnEventCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureEngineOnSampleCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureEngineOnSampleCallback {}
impl ::core::fmt::Debug for IMFCaptureEngineOnSampleCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureEngineOnSampleCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureEngineOnSampleCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureEngineOnSampleCallback2 {}
impl ::core::fmt::Debug for IMFCaptureEngineOnSampleCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureEngineOnSampleCallback2").field(&self.0).finish()
    }
}
impl IMFCaptureEngineOnSampleCallback2 {
    pub unsafe fn OnSample<P0>(&self, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFSample>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnSample)(::windows::core::Vtable::as_raw(self), psample.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFCapturePhotoConfirmation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCapturePhotoConfirmation {}
impl ::core::fmt::Debug for IMFCapturePhotoConfirmation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCapturePhotoConfirmation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCapturePhotoSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCapturePhotoSink {}
impl ::core::fmt::Debug for IMFCapturePhotoSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCapturePhotoSink").field(&self.0).finish()
    }
}
impl IMFCapturePhotoSink {
    pub unsafe fn GetOutputMediaType(&self, dwsinkstreamindex: u32, ppmediatype: ::core::option::Option<*mut ::core::option::Option<IMFMediaType>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetOutputMediaType)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, ::core::mem::transmute(ppmediatype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetService(&self, dwsinkstreamindex: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunknown: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, rguidservice, riid, ::core::mem::transmute(ppunknown.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddStream<P0, P1>(&self, dwsourcestreamindex: u32, pmediatype: P0, pattributes: P1, pdwsinkstreamindex: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), dwsourcestreamindex, pmediatype.into().abi(), pattributes.into().abi(), ::core::mem::transmute(pdwsinkstreamindex.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Prepare)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAllStreams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllStreams)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFCapturePreviewSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCapturePreviewSink {}
impl ::core::fmt::Debug for IMFCapturePreviewSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCapturePreviewSink").field(&self.0).finish()
    }
}
impl IMFCapturePreviewSink {
    pub unsafe fn GetOutputMediaType(&self, dwsinkstreamindex: u32, ppmediatype: ::core::option::Option<*mut ::core::option::Option<IMFMediaType>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetOutputMediaType)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, ::core::mem::transmute(ppmediatype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetService(&self, dwsinkstreamindex: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunknown: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, rguidservice, riid, ::core::mem::transmute(ppunknown.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddStream<P0, P1>(&self, dwsourcestreamindex: u32, pmediatype: P0, pattributes: P1, pdwsinkstreamindex: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), dwsourcestreamindex, pmediatype.into().abi(), pattributes.into().abi(), ::core::mem::transmute(pdwsinkstreamindex.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Prepare)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAllStreams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllStreams)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureRecordSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureRecordSink {}
impl ::core::fmt::Debug for IMFCaptureRecordSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureRecordSink").field(&self.0).finish()
    }
}
impl IMFCaptureRecordSink {
    pub unsafe fn GetOutputMediaType(&self, dwsinkstreamindex: u32, ppmediatype: ::core::option::Option<*mut ::core::option::Option<IMFMediaType>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetOutputMediaType)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, ::core::mem::transmute(ppmediatype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetService(&self, dwsinkstreamindex: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunknown: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, rguidservice, riid, ::core::mem::transmute(ppunknown.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddStream<P0, P1>(&self, dwsourcestreamindex: u32, pmediatype: P0, pattributes: P1, pdwsinkstreamindex: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), dwsourcestreamindex, pmediatype.into().abi(), pattributes.into().abi(), ::core::mem::transmute(pdwsinkstreamindex.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Prepare)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAllStreams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllStreams)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureSink {}
impl ::core::fmt::Debug for IMFCaptureSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureSink2 {}
impl ::core::fmt::Debug for IMFCaptureSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureSink2").field(&self.0).finish()
    }
}
impl IMFCaptureSink2 {
    pub unsafe fn GetOutputMediaType(&self, dwsinkstreamindex: u32, ppmediatype: ::core::option::Option<*mut ::core::option::Option<IMFMediaType>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetOutputMediaType)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, ::core::mem::transmute(ppmediatype.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetService(&self, dwsinkstreamindex: u32, rguidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppunknown: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetService)(::windows::core::Vtable::as_raw(self), dwsinkstreamindex, rguidservice, riid, ::core::mem::transmute(ppunknown.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddStream<P0, P1>(&self, dwsourcestreamindex: u32, pmediatype: P0, pattributes: P1, pdwsinkstreamindex: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), dwsourcestreamindex, pmediatype.into().abi(), pattributes.into().abi(), ::core::mem::transmute(pdwsinkstreamindex.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Prepare(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Prepare)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn RemoveAllStreams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllStreams)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFCaptureSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCaptureSource {}
impl ::core::fmt::Debug for IMFCaptureSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCaptureSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCdmSuspendNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCdmSuspendNotify {}
impl ::core::fmt::Debug for IMFCdmSuspendNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCdmSuspendNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFClock {}
impl ::core::fmt::Debug for IMFClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFClock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFClockConsumer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFClockConsumer {}
impl ::core::fmt::Debug for IMFClockConsumer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFClockConsumer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFClockStateSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFClockStateSink {}
impl ::core::fmt::Debug for IMFClockStateSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFClockStateSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFCollection {}
impl ::core::fmt::Debug for IMFCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentDecryptionModule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentDecryptionModule {}
impl ::core::fmt::Debug for IMFContentDecryptionModule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentDecryptionModule").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentDecryptionModuleAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentDecryptionModuleAccess {}
impl ::core::fmt::Debug for IMFContentDecryptionModuleAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentDecryptionModuleAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentDecryptionModuleFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentDecryptionModuleFactory {}
impl ::core::fmt::Debug for IMFContentDecryptionModuleFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentDecryptionModuleFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentDecryptionModuleSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentDecryptionModuleSession {}
impl ::core::fmt::Debug for IMFContentDecryptionModuleSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentDecryptionModuleSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentDecryptionModuleSessionCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentDecryptionModuleSessionCallbacks {}
impl ::core::fmt::Debug for IMFContentDecryptionModuleSessionCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentDecryptionModuleSessionCallbacks").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentDecryptorContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentDecryptorContext {}
impl ::core::fmt::Debug for IMFContentDecryptorContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentDecryptorContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentEnabler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentEnabler {}
impl ::core::fmt::Debug for IMFContentEnabler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentEnabler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentProtectionDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentProtectionDevice {}
impl ::core::fmt::Debug for IMFContentProtectionDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentProtectionDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFContentProtectionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFContentProtectionManager {}
impl ::core::fmt::Debug for IMFContentProtectionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFContentProtectionManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFD3D12SynchronizationObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFD3D12SynchronizationObject {}
impl ::core::fmt::Debug for IMFD3D12SynchronizationObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFD3D12SynchronizationObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFD3D12SynchronizationObjectCommands {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFD3D12SynchronizationObjectCommands {}
impl ::core::fmt::Debug for IMFD3D12SynchronizationObjectCommands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFD3D12SynchronizationObjectCommands").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFDLNASinkInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDLNASinkInit {}
impl ::core::fmt::Debug for IMFDLNASinkInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDLNASinkInit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFDRMNetHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDRMNetHelper {}
impl ::core::fmt::Debug for IMFDRMNetHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDRMNetHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFDXGIBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDXGIBuffer {}
impl ::core::fmt::Debug for IMFDXGIBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDXGIBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFDXGIDeviceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDXGIDeviceManager {}
impl ::core::fmt::Debug for IMFDXGIDeviceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDXGIDeviceManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFDXGIDeviceManagerSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDXGIDeviceManagerSource {}
impl ::core::fmt::Debug for IMFDXGIDeviceManagerSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDXGIDeviceManagerSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFDesiredSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFDesiredSample {}
impl ::core::fmt::Debug for IMFDesiredSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFDesiredSample").field(&self.0).finish()
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
impl ::core::cmp::PartialEq for IMFExtendedCameraControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedCameraControl {}
impl ::core::fmt::Debug for IMFExtendedCameraControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedCameraControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFExtendedCameraController {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedCameraController {}
impl ::core::fmt::Debug for IMFExtendedCameraController {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedCameraController").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFExtendedCameraIntrinsicModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedCameraIntrinsicModel {}
impl ::core::fmt::Debug for IMFExtendedCameraIntrinsicModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedCameraIntrinsicModel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFExtendedCameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedCameraIntrinsics {}
impl ::core::fmt::Debug for IMFExtendedCameraIntrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedCameraIntrinsics").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFExtendedCameraIntrinsicsDistortionModel6KT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedCameraIntrinsicsDistortionModel6KT {}
impl ::core::fmt::Debug for IMFExtendedCameraIntrinsicsDistortionModel6KT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedCameraIntrinsicsDistortionModel6KT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFExtendedCameraIntrinsicsDistortionModelArcTan {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedCameraIntrinsicsDistortionModelArcTan {}
impl ::core::fmt::Debug for IMFExtendedCameraIntrinsicsDistortionModelArcTan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedCameraIntrinsicsDistortionModelArcTan").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFExtendedDRMTypeSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFExtendedDRMTypeSupport {}
impl ::core::fmt::Debug for IMFExtendedDRMTypeSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFExtendedDRMTypeSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFFieldOfUseMFTUnlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFFieldOfUseMFTUnlock {}
impl ::core::fmt::Debug for IMFFieldOfUseMFTUnlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFFieldOfUseMFTUnlock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFFinalizableMediaSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFFinalizableMediaSink {}
impl ::core::fmt::Debug for IMFFinalizableMediaSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFFinalizableMediaSink").field(&self.0).finish()
    }
}
impl IMFFinalizableMediaSink {
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCharacteristics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddStreamSink<P0>(&self, dwstreamsinkidentifier: u32, pmediatype: P0) -> ::windows::core::Result<IMFStreamSink>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddStreamSink)(::windows::core::Vtable::as_raw(self), dwstreamsinkidentifier, pmediatype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveStreamSink(&self, dwstreamsinkidentifier: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveStreamSink)(::windows::core::Vtable::as_raw(self), dwstreamsinkidentifier).ok()
    }
    pub unsafe fn GetStreamSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamSinkCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamSinkByIndex(&self, dwindex: u32) -> ::windows::core::Result<IMFStreamSink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamSinkByIndex)(::windows::core::Vtable::as_raw(self), dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamSinkById(&self, dwstreamsinkidentifier: u32) -> ::windows::core::Result<IMFStreamSink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamSinkById)(::windows::core::Vtable::as_raw(self), dwstreamsinkidentifier, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPresentationClock<P0>(&self, ppresentationclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFPresentationClock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPresentationClock)(::windows::core::Vtable::as_raw(self), ppresentationclock.into().abi()).ok()
    }
    pub unsafe fn GetPresentationClock(&self) -> ::windows::core::Result<IMFPresentationClock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPresentationClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFGetService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFGetService {}
impl ::core::fmt::Debug for IMFGetService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFGetService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFHDCPStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFHDCPStatus {}
impl ::core::fmt::Debug for IMFHDCPStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFHDCPStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFHttpDownloadRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFHttpDownloadRequest {}
impl ::core::fmt::Debug for IMFHttpDownloadRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFHttpDownloadRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFHttpDownloadSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFHttpDownloadSession {}
impl ::core::fmt::Debug for IMFHttpDownloadSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFHttpDownloadSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFHttpDownloadSessionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFHttpDownloadSessionProvider {}
impl ::core::fmt::Debug for IMFHttpDownloadSessionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFHttpDownloadSessionProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFImageSharingEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFImageSharingEngine {}
impl ::core::fmt::Debug for IMFImageSharingEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFImageSharingEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFImageSharingEngineClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFImageSharingEngineClassFactory {}
impl ::core::fmt::Debug for IMFImageSharingEngineClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFImageSharingEngineClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFInputTrustAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFInputTrustAuthority {}
impl ::core::fmt::Debug for IMFInputTrustAuthority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFInputTrustAuthority").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFLocalMFTRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFLocalMFTRegistration {}
impl ::core::fmt::Debug for IMFLocalMFTRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFLocalMFTRegistration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaBuffer {}
impl ::core::fmt::Debug for IMFMediaBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngine {}
impl ::core::fmt::Debug for IMFMediaEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngine").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineAudioEndpointId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineAudioEndpointId {}
impl ::core::fmt::Debug for IMFMediaEngineAudioEndpointId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineAudioEndpointId").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineClassFactory {}
impl ::core::fmt::Debug for IMFMediaEngineClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineClassFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineClassFactory2 {}
impl ::core::fmt::Debug for IMFMediaEngineClassFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineClassFactory2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineClassFactory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineClassFactory3 {}
impl ::core::fmt::Debug for IMFMediaEngineClassFactory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineClassFactory3").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineClassFactory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineClassFactory4 {}
impl ::core::fmt::Debug for IMFMediaEngineClassFactory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineClassFactory4").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineClassFactoryEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineClassFactoryEx {}
impl ::core::fmt::Debug for IMFMediaEngineClassFactoryEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineClassFactoryEx").field(&self.0).finish()
    }
}
impl IMFMediaEngineClassFactoryEx {
    pub unsafe fn CreateInstance<P0>(&self, dwflags: u32, pattr: P0) -> ::windows::core::Result<IMFMediaEngine>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateInstance)(::windows::core::Vtable::as_raw(self), dwflags, pattr.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateTimeRange(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTimeRange)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateError(&self) -> ::windows::core::Result<IMFMediaError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineEME {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineEME {}
impl ::core::fmt::Debug for IMFMediaEngineEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineEME").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineEMENotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineEMENotify {}
impl ::core::fmt::Debug for IMFMediaEngineEMENotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineEMENotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineEx {}
impl ::core::fmt::Debug for IMFMediaEngineEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineEx").field(&self.0).finish()
    }
}
impl IMFMediaEngineEx {
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IMFMediaError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetErrorCode)(::windows::core::Vtable::as_raw(self), error).ok()
    }
    pub unsafe fn SetSourceElements<P0>(&self, psrcelements: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaEngineSrcElements>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSourceElements)(::windows::core::Vtable::as_raw(self), psrcelements.into().abi()).ok()
    }
    pub unsafe fn SetSource(&self, purl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSource)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(purl)).ok()
    }
    pub unsafe fn GetCurrentSource(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNetworkState(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.GetNetworkState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPreload(&self) -> MF_MEDIA_ENGINE_PRELOAD {
        (::windows::core::Vtable::vtable(self).base__.GetPreload)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPreload(&self, preload: MF_MEDIA_ENGINE_PRELOAD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPreload)(::windows::core::Vtable::as_raw(self), preload).ok()
    }
    pub unsafe fn GetBuffered(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBuffered)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CanPlayType(&self, r#type: &::windows::core::BSTR) -> ::windows::core::Result<MF_MEDIA_ENGINE_CANPLAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanPlayType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(r#type), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetReadyState(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.GetReadyState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSeeking(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSeeking)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentTime(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentTime)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetCurrentTime(&self, seektime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentTime)(::windows::core::Vtable::as_raw(self), seektime).ok()
    }
    pub unsafe fn GetStartTime(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetStartTime)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetDuration)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPaused(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsPaused)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDefaultPlaybackRate(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetDefaultPlaybackRate)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetDefaultPlaybackRate(&self, rate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultPlaybackRate)(::windows::core::Vtable::as_raw(self), rate).ok()
    }
    pub unsafe fn GetPlaybackRate(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetPlaybackRate)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPlaybackRate(&self, rate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPlaybackRate)(::windows::core::Vtable::as_raw(self), rate).ok()
    }
    pub unsafe fn GetPlayed(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPlayed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSeekable(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSeekable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnded(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsEnded)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoPlay(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetAutoPlay)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoPlay<P0>(&self, autoplay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAutoPlay)(::windows::core::Vtable::as_raw(self), autoplay.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoop(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetLoop)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoop<P0>(&self, r#loop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLoop)(::windows::core::Vtable::as_raw(self), r#loop.into()).ok()
    }
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuted(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetMuted)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuted<P0>(&self, muted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMuted)(::windows::core::Vtable::as_raw(self), muted.into()).ok()
    }
    pub unsafe fn GetVolume(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetVolume)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetVolume(&self, volume: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVolume)(::windows::core::Vtable::as_raw(self), volume).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVideo(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasVideo)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasAudio(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasAudio)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNativeVideoSize(&self, cx: ::core::option::Option<*mut u32>, cy: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNativeVideoSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(cx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(cy.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVideoAspectRatio(&self, cx: ::core::option::Option<*mut u32>, cy: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoAspectRatio)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(cx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(cy.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransferVideoFrame<P0>(&self, pdstsurf: P0, psrc: ::core::option::Option<*const MFVideoNormalizedRect>, pdst: *const super::super::Foundation::RECT, pborderclr: ::core::option::Option<*const MFARGB>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TransferVideoFrame)(::windows::core::Vtable::as_raw(self), pdstsurf.into().abi(), ::core::mem::transmute(psrc.unwrap_or(::std::ptr::null())), pdst, ::core::mem::transmute(pborderclr.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn OnVideoStreamTick(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnVideoStreamTick)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineExtension {}
impl ::core::fmt::Debug for IMFMediaEngineExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineNeedKeyNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineNeedKeyNotify {}
impl ::core::fmt::Debug for IMFMediaEngineNeedKeyNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineNeedKeyNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineNotify {}
impl ::core::fmt::Debug for IMFMediaEngineNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineOPMInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineOPMInfo {}
impl ::core::fmt::Debug for IMFMediaEngineOPMInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineOPMInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineProtectedContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineProtectedContent {}
impl ::core::fmt::Debug for IMFMediaEngineProtectedContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineProtectedContent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineSrcElements {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineSrcElements {}
impl ::core::fmt::Debug for IMFMediaEngineSrcElements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineSrcElements").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineSrcElementsEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineSrcElementsEx {}
impl ::core::fmt::Debug for IMFMediaEngineSrcElementsEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineSrcElementsEx").field(&self.0).finish()
    }
}
impl IMFMediaEngineSrcElementsEx {
    pub unsafe fn GetLength(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLength)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetURL(&self, index: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetURL)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self, index: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMedia(&self, index: u32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMedia)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddElement(&self, purl: &::windows::core::BSTR, ptype: &::windows::core::BSTR, pmedia: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddElement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(purl), ::core::mem::transmute_copy(ptype), ::core::mem::transmute_copy(pmedia)).ok()
    }
    pub unsafe fn RemoveAllElements(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllElements)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineSupportsSourceTransfer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineSupportsSourceTransfer {}
impl ::core::fmt::Debug for IMFMediaEngineSupportsSourceTransfer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineSupportsSourceTransfer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineTransferSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineTransferSource {}
impl ::core::fmt::Debug for IMFMediaEngineTransferSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineTransferSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEngineWebSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEngineWebSupport {}
impl ::core::fmt::Debug for IMFMediaEngineWebSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEngineWebSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaError {}
impl ::core::fmt::Debug for IMFMediaError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaError").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEvent {}
impl ::core::fmt::Debug for IMFMediaEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEvent").field(&self.0).finish()
    }
}
impl IMFMediaEvent {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEventGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEventGenerator {}
impl ::core::fmt::Debug for IMFMediaEventGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEventGenerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaEventQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaEventQueue {}
impl ::core::fmt::Debug for IMFMediaEventQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaEventQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeySession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeySession {}
impl ::core::fmt::Debug for IMFMediaKeySession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeySession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeySession2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeySession2 {}
impl ::core::fmt::Debug for IMFMediaKeySession2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeySession2").field(&self.0).finish()
    }
}
impl IMFMediaKeySession2 {
    pub unsafe fn GetError(&self, code: *mut u16, systemcode: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetError)(::windows::core::Vtable::as_raw(self), code, systemcode).ok()
    }
    pub unsafe fn KeySystem(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeySystem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SessionId(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SessionId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Update(&self, key: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Update)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(key.as_ptr()), key.len() as _).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeySessionNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeySessionNotify {}
impl ::core::fmt::Debug for IMFMediaKeySessionNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeySessionNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeySessionNotify2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeySessionNotify2 {}
impl ::core::fmt::Debug for IMFMediaKeySessionNotify2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeySessionNotify2").field(&self.0).finish()
    }
}
impl IMFMediaKeySessionNotify2 {
    pub unsafe fn KeyMessage(&self, destinationurl: &::windows::core::BSTR, message: &[u8]) {
        (::windows::core::Vtable::vtable(self).base__.KeyMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(destinationurl), ::core::mem::transmute(message.as_ptr()), message.len() as _)
    }
    pub unsafe fn KeyAdded(&self) {
        (::windows::core::Vtable::vtable(self).base__.KeyAdded)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn KeyError(&self, code: u16, systemcode: u32) {
        (::windows::core::Vtable::vtable(self).base__.KeyError)(::windows::core::Vtable::as_raw(self), code, systemcode)
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeySystemAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeySystemAccess {}
impl ::core::fmt::Debug for IMFMediaKeySystemAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeySystemAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeys {}
impl ::core::fmt::Debug for IMFMediaKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeys").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaKeys2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaKeys2 {}
impl ::core::fmt::Debug for IMFMediaKeys2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaKeys2").field(&self.0).finish()
    }
}
impl IMFMediaKeys2 {
    pub unsafe fn CreateSession<P0>(&self, mimetype: &::windows::core::BSTR, initdata: ::core::option::Option<&[u8]>, customdata: ::core::option::Option<&[u8]>, notify: P0) -> ::windows::core::Result<IMFMediaKeySession>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaKeySessionNotify>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSession)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(mimetype), ::core::mem::transmute(initdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), initdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(customdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), customdata.as_deref().map_or(0, |slice| slice.len() as _), notify.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeySystem(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeySystem)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSuspendNotify(&self) -> ::windows::core::Result<IMFCdmSuspendNotify> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSuspendNotify)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFMediaSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSession {}
impl ::core::fmt::Debug for IMFMediaSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSession").field(&self.0).finish()
    }
}
impl IMFMediaSession {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSharingEngine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSharingEngine {}
impl ::core::fmt::Debug for IMFMediaSharingEngine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSharingEngine").field(&self.0).finish()
    }
}
impl IMFMediaSharingEngine {
    pub unsafe fn GetError(&self) -> ::windows::core::Result<IMFMediaError> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetError)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetErrorCode)(::windows::core::Vtable::as_raw(self), error).ok()
    }
    pub unsafe fn SetSourceElements<P0>(&self, psrcelements: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaEngineSrcElements>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSourceElements)(::windows::core::Vtable::as_raw(self), psrcelements.into().abi()).ok()
    }
    pub unsafe fn SetSource(&self, purl: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSource)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(purl)).ok()
    }
    pub unsafe fn GetCurrentSource(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNetworkState(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.GetNetworkState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetPreload(&self) -> MF_MEDIA_ENGINE_PRELOAD {
        (::windows::core::Vtable::vtable(self).base__.GetPreload)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPreload(&self, preload: MF_MEDIA_ENGINE_PRELOAD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPreload)(::windows::core::Vtable::as_raw(self), preload).ok()
    }
    pub unsafe fn GetBuffered(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBuffered)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CanPlayType(&self, r#type: &::windows::core::BSTR) -> ::windows::core::Result<MF_MEDIA_ENGINE_CANPLAY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanPlayType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(r#type), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetReadyState(&self) -> u16 {
        (::windows::core::Vtable::vtable(self).base__.GetReadyState)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSeeking(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsSeeking)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCurrentTime(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentTime)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetCurrentTime(&self, seektime: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentTime)(::windows::core::Vtable::as_raw(self), seektime).ok()
    }
    pub unsafe fn GetStartTime(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetStartTime)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetDuration)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPaused(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsPaused)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDefaultPlaybackRate(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetDefaultPlaybackRate)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetDefaultPlaybackRate(&self, rate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDefaultPlaybackRate)(::windows::core::Vtable::as_raw(self), rate).ok()
    }
    pub unsafe fn GetPlaybackRate(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetPlaybackRate)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetPlaybackRate(&self, rate: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPlaybackRate)(::windows::core::Vtable::as_raw(self), rate).ok()
    }
    pub unsafe fn GetPlayed(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPlayed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSeekable(&self) -> ::windows::core::Result<IMFMediaTimeRange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSeekable)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnded(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsEnded)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoPlay(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetAutoPlay)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAutoPlay<P0>(&self, autoplay: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAutoPlay)(::windows::core::Vtable::as_raw(self), autoplay.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoop(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetLoop)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoop<P0>(&self, r#loop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLoop)(::windows::core::Vtable::as_raw(self), r#loop.into()).ok()
    }
    pub unsafe fn Play(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Play)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuted(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetMuted)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuted<P0>(&self, muted: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMuted)(::windows::core::Vtable::as_raw(self), muted.into()).ok()
    }
    pub unsafe fn GetVolume(&self) -> f64 {
        (::windows::core::Vtable::vtable(self).base__.GetVolume)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetVolume(&self, volume: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVolume)(::windows::core::Vtable::as_raw(self), volume).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVideo(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasVideo)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasAudio(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.HasAudio)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNativeVideoSize(&self, cx: ::core::option::Option<*mut u32>, cy: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNativeVideoSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(cx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(cy.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetVideoAspectRatio(&self, cx: ::core::option::Option<*mut u32>, cy: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoAspectRatio)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(cx.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(cy.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransferVideoFrame<P0>(&self, pdstsurf: P0, psrc: ::core::option::Option<*const MFVideoNormalizedRect>, pdst: *const super::super::Foundation::RECT, pborderclr: ::core::option::Option<*const MFARGB>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TransferVideoFrame)(::windows::core::Vtable::as_raw(self), pdstsurf.into().abi(), ::core::mem::transmute(psrc.unwrap_or(::std::ptr::null())), pdst, ::core::mem::transmute(pborderclr.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn OnVideoStreamTick(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OnVideoStreamTick)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFMediaSharingEngineClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSharingEngineClassFactory {}
impl ::core::fmt::Debug for IMFMediaSharingEngineClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSharingEngineClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSink {}
impl ::core::fmt::Debug for IMFMediaSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSinkPreroll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSinkPreroll {}
impl ::core::fmt::Debug for IMFMediaSinkPreroll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSinkPreroll").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSource {}
impl ::core::fmt::Debug for IMFMediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSource").field(&self.0).finish()
    }
}
impl IMFMediaSource {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSource2 {}
impl ::core::fmt::Debug for IMFMediaSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSource2").field(&self.0).finish()
    }
}
impl IMFMediaSource2 {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCharacteristics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePresentationDescriptor(&self) -> ::windows::core::Result<IMFPresentationDescriptor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePresentationDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Start<P0>(&self, ppresentationdescriptor: P0, pguidtimeformat: *const ::windows::core::GUID, pvarstartposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFPresentationDescriptor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Start)(::windows::core::Vtable::as_raw(self), ppresentationdescriptor.into().abi(), pguidtimeformat, pvarstartposition).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSourceAttributes(&self) -> ::windows::core::Result<IMFAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSourceAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamAttributes(&self, dwstreamidentifier: u32) -> ::windows::core::Result<IMFAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamAttributes)(::windows::core::Vtable::as_raw(self), dwstreamidentifier, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetD3DManager<P0>(&self, pmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetD3DManager)(::windows::core::Vtable::as_raw(self), pmanager.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSourceEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSourceEx {}
impl ::core::fmt::Debug for IMFMediaSourceEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSourceEx").field(&self.0).finish()
    }
}
impl IMFMediaSourceEx {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
    pub unsafe fn GetCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCharacteristics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePresentationDescriptor(&self) -> ::windows::core::Result<IMFPresentationDescriptor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePresentationDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Start<P0>(&self, ppresentationdescriptor: P0, pguidtimeformat: *const ::windows::core::GUID, pvarstartposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFPresentationDescriptor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self), ppresentationdescriptor.into().abi(), pguidtimeformat, pvarstartposition).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Pause(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Pause)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSourceExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSourceExtension {}
impl ::core::fmt::Debug for IMFMediaSourceExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSourceExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSourceExtensionLiveSeekableRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSourceExtensionLiveSeekableRange {}
impl ::core::fmt::Debug for IMFMediaSourceExtensionLiveSeekableRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSourceExtensionLiveSeekableRange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSourceExtensionNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSourceExtensionNotify {}
impl ::core::fmt::Debug for IMFMediaSourceExtensionNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSourceExtensionNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSourcePresentationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSourcePresentationProvider {}
impl ::core::fmt::Debug for IMFMediaSourcePresentationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSourcePresentationProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaSourceTopologyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaSourceTopologyProvider {}
impl ::core::fmt::Debug for IMFMediaSourceTopologyProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaSourceTopologyProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaStream {}
impl ::core::fmt::Debug for IMFMediaStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaStream").field(&self.0).finish()
    }
}
impl IMFMediaStream {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaStream2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaStream2 {}
impl ::core::fmt::Debug for IMFMediaStream2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaStream2").field(&self.0).finish()
    }
}
impl IMFMediaStream2 {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
    pub unsafe fn GetMediaSource(&self) -> ::windows::core::Result<IMFMediaSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMediaSource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamDescriptor(&self) -> ::windows::core::Result<IMFStreamDescriptor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RequestSample<P0>(&self, ptoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RequestSample)(::windows::core::Vtable::as_raw(self), ptoken.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaStreamSourceSampleRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaStreamSourceSampleRequest {}
impl ::core::fmt::Debug for IMFMediaStreamSourceSampleRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaStreamSourceSampleRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaTimeRange {}
impl ::core::fmt::Debug for IMFMediaTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaTimeRange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaType {}
impl ::core::fmt::Debug for IMFMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaType").field(&self.0).finish()
    }
}
impl IMFMediaType {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFMediaTypeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMediaTypeHandler {}
impl ::core::fmt::Debug for IMFMediaTypeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMediaTypeHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMetadata {}
impl ::core::fmt::Debug for IMFMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMetadata").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMetadataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMetadataProvider {}
impl ::core::fmt::Debug for IMFMetadataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMetadataProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMuxStreamAttributesManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMuxStreamAttributesManager {}
impl ::core::fmt::Debug for IMFMuxStreamAttributesManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMuxStreamAttributesManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMuxStreamMediaTypeManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMuxStreamMediaTypeManager {}
impl ::core::fmt::Debug for IMFMuxStreamMediaTypeManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMuxStreamMediaTypeManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFMuxStreamSampleManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFMuxStreamSampleManager {}
impl ::core::fmt::Debug for IMFMuxStreamSampleManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFMuxStreamSampleManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetCredential {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetCredential {}
impl ::core::fmt::Debug for IMFNetCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetCredential").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetCredentialCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetCredentialCache {}
impl ::core::fmt::Debug for IMFNetCredentialCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetCredentialCache").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetCredentialManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetCredentialManager {}
impl ::core::fmt::Debug for IMFNetCredentialManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetCredentialManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetCrossOriginSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetCrossOriginSupport {}
impl ::core::fmt::Debug for IMFNetCrossOriginSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetCrossOriginSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetProxyLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetProxyLocator {}
impl ::core::fmt::Debug for IMFNetProxyLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetProxyLocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetProxyLocatorFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetProxyLocatorFactory {}
impl ::core::fmt::Debug for IMFNetProxyLocatorFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetProxyLocatorFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetResourceFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetResourceFilter {}
impl ::core::fmt::Debug for IMFNetResourceFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetResourceFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFNetSchemeHandlerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFNetSchemeHandlerConfig {}
impl ::core::fmt::Debug for IMFNetSchemeHandlerConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFNetSchemeHandlerConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFObjectReferenceStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFObjectReferenceStream {}
impl ::core::fmt::Debug for IMFObjectReferenceStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFObjectReferenceStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFOutputPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFOutputPolicy {}
impl ::core::fmt::Debug for IMFOutputPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFOutputPolicy").field(&self.0).finish()
    }
}
impl IMFOutputPolicy {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFOutputSchema {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFOutputSchema {}
impl ::core::fmt::Debug for IMFOutputSchema {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFOutputSchema").field(&self.0).finish()
    }
}
impl IMFOutputSchema {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFOutputTrustAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFOutputTrustAuthority {}
impl ::core::fmt::Debug for IMFOutputTrustAuthority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFOutputTrustAuthority").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMPClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMPClient {}
impl ::core::fmt::Debug for IMFPMPClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMPClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMPClientApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMPClientApp {}
impl ::core::fmt::Debug for IMFPMPClientApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMPClientApp").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMPHost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMPHost {}
impl ::core::fmt::Debug for IMFPMPHost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMPHost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMPHostApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMPHostApp {}
impl ::core::fmt::Debug for IMFPMPHostApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMPHostApp").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMPServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMPServer {}
impl ::core::fmt::Debug for IMFPMPServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMPServer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMediaItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMediaItem {}
impl ::core::fmt::Debug for IMFPMediaItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMediaItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMediaPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMediaPlayer {}
impl ::core::fmt::Debug for IMFPMediaPlayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMediaPlayer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPMediaPlayerCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPMediaPlayerCallback {}
impl ::core::fmt::Debug for IMFPMediaPlayerCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPMediaPlayerCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPluginControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPluginControl {}
impl ::core::fmt::Debug for IMFPluginControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPluginControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFPluginControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPluginControl2 {}
impl ::core::fmt::Debug for IMFPluginControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPluginControl2").field(&self.0).finish()
    }
}
impl IMFPluginControl2 {
    pub unsafe fn GetPreferredClsid<P0>(&self, plugintype: u32, selector: P0) -> ::windows::core::Result<::windows::core::GUID>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPreferredClsid)(::windows::core::Vtable::as_raw(self), plugintype, selector.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPreferredClsidByIndex(&self, plugintype: u32, index: u32, selector: *mut ::windows::core::PWSTR, clsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPreferredClsidByIndex)(::windows::core::Vtable::as_raw(self), plugintype, index, selector, clsid).ok()
    }
    pub unsafe fn SetPreferredClsid<P0>(&self, plugintype: u32, selector: P0, clsid: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPreferredClsid)(::windows::core::Vtable::as_raw(self), plugintype, selector.into().abi(), ::core::mem::transmute(clsid.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn IsDisabled(&self, plugintype: u32, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsDisabled)(::windows::core::Vtable::as_raw(self), plugintype, clsid).ok()
    }
    pub unsafe fn GetDisabledByIndex(&self, plugintype: u32, index: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDisabledByIndex)(::windows::core::Vtable::as_raw(self), plugintype, index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisabled<P0>(&self, plugintype: u32, clsid: *const ::windows::core::GUID, disabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDisabled)(::windows::core::Vtable::as_raw(self), plugintype, clsid, disabled.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFPresentationClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPresentationClock {}
impl ::core::fmt::Debug for IMFPresentationClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPresentationClock").field(&self.0).finish()
    }
}
impl IMFPresentationClock {
    pub unsafe fn GetClockCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClockCharacteristics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCorrelatedTime(&self, dwreserved: u32, pllclocktime: *mut i64, phnssystemtime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCorrelatedTime)(::windows::core::Vtable::as_raw(self), dwreserved, pllclocktime, phnssystemtime).ok()
    }
    pub unsafe fn GetContinuityKey(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContinuityKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetState(&self, dwreserved: u32) -> ::windows::core::Result<MFCLOCK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetState)(::windows::core::Vtable::as_raw(self), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self, pclockproperties: *mut MFCLOCK_PROPERTIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProperties)(::windows::core::Vtable::as_raw(self), pclockproperties).ok()
    }
}
impl ::core::cmp::PartialEq for IMFPresentationDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPresentationDescriptor {}
impl ::core::fmt::Debug for IMFPresentationDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPresentationDescriptor").field(&self.0).finish()
    }
}
impl IMFPresentationDescriptor {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFPresentationTimeSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFPresentationTimeSource {}
impl ::core::fmt::Debug for IMFPresentationTimeSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFPresentationTimeSource").field(&self.0).finish()
    }
}
impl IMFPresentationTimeSource {
    pub unsafe fn GetClockCharacteristics(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClockCharacteristics)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCorrelatedTime(&self, dwreserved: u32, pllclocktime: *mut i64, phnssystemtime: *mut i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCorrelatedTime)(::windows::core::Vtable::as_raw(self), dwreserved, pllclocktime, phnssystemtime).ok()
    }
    pub unsafe fn GetContinuityKey(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContinuityKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetState(&self, dwreserved: u32) -> ::windows::core::Result<MFCLOCK_STATE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetState)(::windows::core::Vtable::as_raw(self), dwreserved, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self, pclockproperties: *mut MFCLOCK_PROPERTIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProperties)(::windows::core::Vtable::as_raw(self), pclockproperties).ok()
    }
}
impl ::core::cmp::PartialEq for IMFProtectedEnvironmentAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFProtectedEnvironmentAccess {}
impl ::core::fmt::Debug for IMFProtectedEnvironmentAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFProtectedEnvironmentAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFQualityAdvise {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFQualityAdvise {}
impl ::core::fmt::Debug for IMFQualityAdvise {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFQualityAdvise").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFQualityAdvise2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFQualityAdvise2 {}
impl ::core::fmt::Debug for IMFQualityAdvise2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFQualityAdvise2").field(&self.0).finish()
    }
}
impl IMFQualityAdvise2 {
    pub unsafe fn SetDropMode(&self, edropmode: MF_QUALITY_DROP_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDropMode)(::windows::core::Vtable::as_raw(self), edropmode).ok()
    }
    pub unsafe fn SetQualityLevel(&self, equalitylevel: MF_QUALITY_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetQualityLevel)(::windows::core::Vtable::as_raw(self), equalitylevel).ok()
    }
    pub unsafe fn GetDropMode(&self) -> ::windows::core::Result<MF_QUALITY_DROP_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDropMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetQualityLevel(&self) -> ::windows::core::Result<MF_QUALITY_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetQualityLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DropTime(&self, hnsamounttodrop: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DropTime)(::windows::core::Vtable::as_raw(self), hnsamounttodrop).ok()
    }
}
impl ::core::cmp::PartialEq for IMFQualityAdviseLimits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFQualityAdviseLimits {}
impl ::core::fmt::Debug for IMFQualityAdviseLimits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFQualityAdviseLimits").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFQualityManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFQualityManager {}
impl ::core::fmt::Debug for IMFQualityManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFQualityManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRateControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRateControl {}
impl ::core::fmt::Debug for IMFRateControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRateControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRateSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRateSupport {}
impl ::core::fmt::Debug for IMFRateSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRateSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFReadWriteClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFReadWriteClassFactory {}
impl ::core::fmt::Debug for IMFReadWriteClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFReadWriteClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRealTimeClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRealTimeClient {}
impl ::core::fmt::Debug for IMFRealTimeClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRealTimeClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRealTimeClientEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRealTimeClientEx {}
impl ::core::fmt::Debug for IMFRealTimeClientEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRealTimeClientEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRelativePanelReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRelativePanelReport {}
impl ::core::fmt::Debug for IMFRelativePanelReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRelativePanelReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRelativePanelWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRelativePanelWatcher {}
impl ::core::fmt::Debug for IMFRelativePanelWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRelativePanelWatcher").field(&self.0).finish()
    }
}
impl IMFRelativePanelWatcher {
    pub unsafe fn Shutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Shutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetShutdownStatus(&self) -> ::windows::core::Result<MFSHUTDOWN_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetShutdownStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFRemoteAsyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRemoteAsyncCallback {}
impl ::core::fmt::Debug for IMFRemoteAsyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRemoteAsyncCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRemoteDesktopPlugin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRemoteDesktopPlugin {}
impl ::core::fmt::Debug for IMFRemoteDesktopPlugin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRemoteDesktopPlugin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFRemoteProxy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFRemoteProxy {}
impl ::core::fmt::Debug for IMFRemoteProxy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFRemoteProxy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSAMIStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSAMIStyle {}
impl ::core::fmt::Debug for IMFSAMIStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSAMIStyle").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSSLCertificateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSSLCertificateManager {}
impl ::core::fmt::Debug for IMFSSLCertificateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSSLCertificateManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSample {}
impl ::core::fmt::Debug for IMFSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSample").field(&self.0).finish()
    }
}
impl IMFSample {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSampleAllocatorControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSampleAllocatorControl {}
impl ::core::fmt::Debug for IMFSampleAllocatorControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSampleAllocatorControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSampleGrabberSinkCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSampleGrabberSinkCallback {}
impl ::core::fmt::Debug for IMFSampleGrabberSinkCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSampleGrabberSinkCallback").field(&self.0).finish()
    }
}
impl IMFSampleGrabberSinkCallback {
    pub unsafe fn OnClockStart(&self, hnssystemtime: i64, llclockstartoffset: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockStart)(::windows::core::Vtable::as_raw(self), hnssystemtime, llclockstartoffset).ok()
    }
    pub unsafe fn OnClockStop(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockStop)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockPause(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockPause)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockRestart(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockRestart)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockSetRate(&self, hnssystemtime: i64, flrate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockSetRate)(::windows::core::Vtable::as_raw(self), hnssystemtime, flrate).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSampleGrabberSinkCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSampleGrabberSinkCallback2 {}
impl ::core::fmt::Debug for IMFSampleGrabberSinkCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSampleGrabberSinkCallback2").field(&self.0).finish()
    }
}
impl IMFSampleGrabberSinkCallback2 {
    pub unsafe fn OnClockStart(&self, hnssystemtime: i64, llclockstartoffset: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnClockStart)(::windows::core::Vtable::as_raw(self), hnssystemtime, llclockstartoffset).ok()
    }
    pub unsafe fn OnClockStop(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnClockStop)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockPause(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnClockPause)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockRestart(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnClockRestart)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockSetRate(&self, hnssystemtime: i64, flrate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnClockSetRate)(::windows::core::Vtable::as_raw(self), hnssystemtime, flrate).ok()
    }
    pub unsafe fn OnSetPresentationClock<P0>(&self, ppresentationclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFPresentationClock>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnSetPresentationClock)(::windows::core::Vtable::as_raw(self), ppresentationclock.into().abi()).ok()
    }
    pub unsafe fn OnProcessSample(&self, guidmajormediatype: *const ::windows::core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnProcessSample)(::windows::core::Vtable::as_raw(self), guidmajormediatype, dwsampleflags, llsampletime, llsampleduration, ::core::mem::transmute(psamplebuffer.as_ptr()), psamplebuffer.len() as _).ok()
    }
    pub unsafe fn OnShutdown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnShutdown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSampleOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSampleOutputStream {}
impl ::core::fmt::Debug for IMFSampleOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSampleOutputStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSampleProtection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSampleProtection {}
impl ::core::fmt::Debug for IMFSampleProtection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSampleProtection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSaveJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSaveJob {}
impl ::core::fmt::Debug for IMFSaveJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSaveJob").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSchemeHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSchemeHandler {}
impl ::core::fmt::Debug for IMFSchemeHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSchemeHandler").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSecureBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSecureBuffer {}
impl ::core::fmt::Debug for IMFSecureBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSecureBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSecureChannel {}
impl ::core::fmt::Debug for IMFSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSecureChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSeekInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSeekInfo {}
impl ::core::fmt::Debug for IMFSeekInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSeekInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorActivitiesReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorActivitiesReport {}
impl ::core::fmt::Debug for IMFSensorActivitiesReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorActivitiesReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorActivitiesReportCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorActivitiesReportCallback {}
impl ::core::fmt::Debug for IMFSensorActivitiesReportCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorActivitiesReportCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorActivityMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorActivityMonitor {}
impl ::core::fmt::Debug for IMFSensorActivityMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorActivityMonitor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorActivityReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorActivityReport {}
impl ::core::fmt::Debug for IMFSensorActivityReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorActivityReport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorDevice {}
impl ::core::fmt::Debug for IMFSensorDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorGroup {}
impl ::core::fmt::Debug for IMFSensorGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorProcessActivity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorProcessActivity {}
impl ::core::fmt::Debug for IMFSensorProcessActivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorProcessActivity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorProfile {}
impl ::core::fmt::Debug for IMFSensorProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorProfile").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorProfileCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorProfileCollection {}
impl ::core::fmt::Debug for IMFSensorProfileCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorProfileCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSensorStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorStream {}
impl ::core::fmt::Debug for IMFSensorStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorStream").field(&self.0).finish()
    }
}
impl IMFSensorStream {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSensorTransformFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSensorTransformFactory {}
impl ::core::fmt::Debug for IMFSensorTransformFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSensorTransformFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSequencerSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSequencerSource {}
impl ::core::fmt::Debug for IMFSequencerSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSequencerSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSharingEngineClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSharingEngineClassFactory {}
impl ::core::fmt::Debug for IMFSharingEngineClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSharingEngineClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFShutdown {}
impl ::core::fmt::Debug for IMFShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFShutdown").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSignedLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSignedLibrary {}
impl ::core::fmt::Debug for IMFSignedLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSignedLibrary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSimpleAudioVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSimpleAudioVolume {}
impl ::core::fmt::Debug for IMFSimpleAudioVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSimpleAudioVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSinkWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSinkWriter {}
impl ::core::fmt::Debug for IMFSinkWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSinkWriter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSinkWriterCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSinkWriterCallback {}
impl ::core::fmt::Debug for IMFSinkWriterCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSinkWriterCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSinkWriterCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSinkWriterCallback2 {}
impl ::core::fmt::Debug for IMFSinkWriterCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSinkWriterCallback2").field(&self.0).finish()
    }
}
impl IMFSinkWriterCallback2 {
    pub unsafe fn OnFinalize(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnFinalize)(::windows::core::Vtable::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn OnMarker(&self, dwstreamindex: u32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnMarker)(::windows::core::Vtable::as_raw(self), dwstreamindex, pvcontext).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSinkWriterEncoderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSinkWriterEncoderConfig {}
impl ::core::fmt::Debug for IMFSinkWriterEncoderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSinkWriterEncoderConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSinkWriterEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSinkWriterEx {}
impl ::core::fmt::Debug for IMFSinkWriterEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSinkWriterEx").field(&self.0).finish()
    }
}
impl IMFSinkWriterEx {
    pub unsafe fn AddStream<P0>(&self, ptargetmediatype: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), ptargetmediatype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetInputMediaType<P0, P1>(&self, dwstreamindex: u32, pinputmediatype: P0, pencodingparameters: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetInputMediaType)(::windows::core::Vtable::as_raw(self), dwstreamindex, pinputmediatype.into().abi(), pencodingparameters.into().abi()).ok()
    }
    pub unsafe fn BeginWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginWriting)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn WriteSample<P0>(&self, dwstreamindex: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFSample>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WriteSample)(::windows::core::Vtable::as_raw(self), dwstreamindex, psample.into().abi()).ok()
    }
    pub unsafe fn SendStreamTick(&self, dwstreamindex: u32, lltimestamp: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendStreamTick)(::windows::core::Vtable::as_raw(self), dwstreamindex, lltimestamp).ok()
    }
    pub unsafe fn PlaceMarker(&self, dwstreamindex: u32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PlaceMarker)(::windows::core::Vtable::as_raw(self), dwstreamindex, pvcontext).ok()
    }
    pub unsafe fn NotifyEndOfSegment(&self, dwstreamindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NotifyEndOfSegment)(::windows::core::Vtable::as_raw(self), dwstreamindex).ok()
    }
    pub unsafe fn Flush(&self, dwstreamindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self), dwstreamindex).ok()
    }
    pub unsafe fn Finalize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Finalize)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetServiceForStream(&self, dwstreamindex: u32, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetServiceForStream)(::windows::core::Vtable::as_raw(self), dwstreamindex, guidservice, riid, ppvobject).ok()
    }
    pub unsafe fn GetStatistics(&self, dwstreamindex: u32, pstats: *mut MF_SINK_WRITER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStatistics)(::windows::core::Vtable::as_raw(self), dwstreamindex, pstats).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSourceBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceBuffer {}
impl ::core::fmt::Debug for IMFSourceBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceBufferAppendMode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceBufferAppendMode {}
impl ::core::fmt::Debug for IMFSourceBufferAppendMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceBufferAppendMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceBufferList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceBufferList {}
impl ::core::fmt::Debug for IMFSourceBufferList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceBufferList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceBufferNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceBufferNotify {}
impl ::core::fmt::Debug for IMFSourceBufferNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceBufferNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceOpenMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceOpenMonitor {}
impl ::core::fmt::Debug for IMFSourceOpenMonitor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceOpenMonitor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceReader {}
impl ::core::fmt::Debug for IMFSourceReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceReaderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceReaderCallback {}
impl ::core::fmt::Debug for IMFSourceReaderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceReaderCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSourceReaderCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceReaderCallback2 {}
impl ::core::fmt::Debug for IMFSourceReaderCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceReaderCallback2").field(&self.0).finish()
    }
}
impl IMFSourceReaderCallback2 {
    pub unsafe fn OnReadSample<P0>(&self, hrstatus: ::windows::core::HRESULT, dwstreamindex: u32, dwstreamflags: u32, lltimestamp: i64, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFSample>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnReadSample)(::windows::core::Vtable::as_raw(self), hrstatus, dwstreamindex, dwstreamflags, lltimestamp, psample.into().abi()).ok()
    }
    pub unsafe fn OnFlush(&self, dwstreamindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnFlush)(::windows::core::Vtable::as_raw(self), dwstreamindex).ok()
    }
    pub unsafe fn OnEvent<P0>(&self, dwstreamindex: u32, pevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaEvent>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnEvent)(::windows::core::Vtable::as_raw(self), dwstreamindex, pevent.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFSourceReaderEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceReaderEx {}
impl ::core::fmt::Debug for IMFSourceReaderEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceReaderEx").field(&self.0).finish()
    }
}
impl IMFSourceReaderEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStreamSelection(&self, dwstreamindex: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamSelection)(::windows::core::Vtable::as_raw(self), dwstreamindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStreamSelection<P0>(&self, dwstreamindex: u32, fselected: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStreamSelection)(::windows::core::Vtable::as_raw(self), dwstreamindex, fselected.into()).ok()
    }
    pub unsafe fn GetNativeMediaType(&self, dwstreamindex: u32, dwmediatypeindex: u32) -> ::windows::core::Result<IMFMediaType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNativeMediaType)(::windows::core::Vtable::as_raw(self), dwstreamindex, dwmediatypeindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCurrentMediaType(&self, dwstreamindex: u32) -> ::windows::core::Result<IMFMediaType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentMediaType)(::windows::core::Vtable::as_raw(self), dwstreamindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentMediaType<P0>(&self, dwstreamindex: u32, pdwreserved: ::core::option::Option<*mut u32>, pmediatype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentMediaType)(::windows::core::Vtable::as_raw(self), dwstreamindex, ::core::mem::transmute(pdwreserved.unwrap_or(::std::ptr::null_mut())), pmediatype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetCurrentPosition(&self, guidtimeformat: *const ::windows::core::GUID, varposition: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentPosition)(::windows::core::Vtable::as_raw(self), guidtimeformat, varposition).ok()
    }
    pub unsafe fn ReadSample(&self, dwstreamindex: u32, dwcontrolflags: u32, pdwactualstreamindex: ::core::option::Option<*mut u32>, pdwstreamflags: ::core::option::Option<*mut u32>, plltimestamp: ::core::option::Option<*mut i64>, ppsample: ::core::option::Option<*mut ::core::option::Option<IMFSample>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReadSample)(::windows::core::Vtable::as_raw(self), dwstreamindex, dwcontrolflags, ::core::mem::transmute(pdwactualstreamindex.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwstreamflags.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(plltimestamp.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppsample.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Flush(&self, dwstreamindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self), dwstreamindex).ok()
    }
    pub unsafe fn GetServiceForStream(&self, dwstreamindex: u32, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetServiceForStream)(::windows::core::Vtable::as_raw(self), dwstreamindex, guidservice, riid, ppvobject).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetPresentationAttribute(&self, dwstreamindex: u32, guidattribute: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPresentationAttribute)(::windows::core::Vtable::as_raw(self), dwstreamindex, guidattribute, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFSourceResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSourceResolver {}
impl ::core::fmt::Debug for IMFSourceResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSourceResolver").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSpatialAudioObjectBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSpatialAudioObjectBuffer {}
impl ::core::fmt::Debug for IMFSpatialAudioObjectBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSpatialAudioObjectBuffer").field(&self.0).finish()
    }
}
impl IMFSpatialAudioObjectBuffer {
    pub unsafe fn Lock(&self, ppbbuffer: *mut *mut u8, pcbmaxlength: ::core::option::Option<*mut u32>, pcbcurrentlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Lock)(::windows::core::Vtable::as_raw(self), ppbbuffer, ::core::mem::transmute(pcbmaxlength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbcurrentlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unlock)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCurrentLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCurrentLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCurrentLength(&self, cbcurrentlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentLength)(::windows::core::Vtable::as_raw(self), cbcurrentlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFSpatialAudioSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSpatialAudioSample {}
impl ::core::fmt::Debug for IMFSpatialAudioSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSpatialAudioSample").field(&self.0).finish()
    }
}
impl IMFSpatialAudioSample {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
    pub unsafe fn GetSampleFlags(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSampleFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSampleFlags(&self, dwsampleflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSampleFlags)(::windows::core::Vtable::as_raw(self), dwsampleflags).ok()
    }
    pub unsafe fn GetSampleTime(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSampleTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSampleTime(&self, hnssampletime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSampleTime)(::windows::core::Vtable::as_raw(self), hnssampletime).ok()
    }
    pub unsafe fn GetSampleDuration(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSampleDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSampleDuration(&self, hnssampleduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSampleDuration)(::windows::core::Vtable::as_raw(self), hnssampleduration).ok()
    }
    pub unsafe fn GetBufferCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBufferCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferByIndex(&self, dwindex: u32) -> ::windows::core::Result<IMFMediaBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBufferByIndex)(::windows::core::Vtable::as_raw(self), dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ConvertToContiguousBuffer(&self) -> ::windows::core::Result<IMFMediaBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ConvertToContiguousBuffer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddBuffer<P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddBuffer)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi()).ok()
    }
    pub unsafe fn RemoveBufferByIndex(&self, dwindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveBufferByIndex)(::windows::core::Vtable::as_raw(self), dwindex).ok()
    }
    pub unsafe fn RemoveAllBuffers(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllBuffers)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetTotalLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTotalLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CopyToBuffer<P0>(&self, pbuffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyToBuffer)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFStreamDescriptor {}
impl ::core::fmt::Debug for IMFStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFStreamDescriptor").field(&self.0).finish()
    }
}
impl IMFStreamDescriptor {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFStreamSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFStreamSink {}
impl ::core::fmt::Debug for IMFStreamSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFStreamSink").field(&self.0).finish()
    }
}
impl IMFStreamSink {
    pub unsafe fn GetEvent(&self, dwflags: MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS) -> ::windows::core::Result<IMFMediaEvent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEvent)(::windows::core::Vtable::as_raw(self), dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginGetEvent<P0, P1>(&self, pcallback: P0, punkstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginGetEvent)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), punkstate.into().abi()).ok()
    }
    pub unsafe fn EndGetEvent<P0>(&self, presult: P0) -> ::windows::core::Result<IMFMediaEvent>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndGetEvent)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn QueueEvent(&self, met: u32, guidextendedtype: *const ::windows::core::GUID, hrstatus: ::windows::core::HRESULT, pvvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueueEvent)(::windows::core::Vtable::as_raw(self), met, guidextendedtype, hrstatus, pvvalue).ok()
    }
}
impl ::core::cmp::PartialEq for IMFStreamingSinkConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFStreamingSinkConfig {}
impl ::core::fmt::Debug for IMFStreamingSinkConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFStreamingSinkConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFSystemId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFSystemId {}
impl ::core::fmt::Debug for IMFSystemId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFSystemId").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimecodeTranslate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimecodeTranslate {}
impl ::core::fmt::Debug for IMFTimecodeTranslate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimecodeTranslate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedText {}
impl ::core::fmt::Debug for IMFTimedText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedText").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextBinary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextBinary {}
impl ::core::fmt::Debug for IMFTimedTextBinary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextBinary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextBouten {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextBouten {}
impl ::core::fmt::Debug for IMFTimedTextBouten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextBouten").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextCue {}
impl ::core::fmt::Debug for IMFTimedTextCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextCue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextCueList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextCueList {}
impl ::core::fmt::Debug for IMFTimedTextCueList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextCueList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextFormattedText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextFormattedText {}
impl ::core::fmt::Debug for IMFTimedTextFormattedText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextFormattedText").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextNotify {}
impl ::core::fmt::Debug for IMFTimedTextNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextRegion {}
impl ::core::fmt::Debug for IMFTimedTextRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextRegion").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextRuby {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextRuby {}
impl ::core::fmt::Debug for IMFTimedTextRuby {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextRuby").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextStyle {}
impl ::core::fmt::Debug for IMFTimedTextStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextStyle").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextStyle2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextStyle2 {}
impl ::core::fmt::Debug for IMFTimedTextStyle2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextStyle2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextTrack {}
impl ::core::fmt::Debug for IMFTimedTextTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextTrack").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimedTextTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimedTextTrackList {}
impl ::core::fmt::Debug for IMFTimedTextTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimedTextTrackList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTimer {}
impl ::core::fmt::Debug for IMFTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTimer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTopoLoader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTopoLoader {}
impl ::core::fmt::Debug for IMFTopoLoader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTopoLoader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTopology {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTopology {}
impl ::core::fmt::Debug for IMFTopology {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTopology").field(&self.0).finish()
    }
}
impl IMFTopology {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFTopologyNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTopologyNode {}
impl ::core::fmt::Debug for IMFTopologyNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTopologyNode").field(&self.0).finish()
    }
}
impl IMFTopologyNode {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFTopologyNodeAttributeEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTopologyNodeAttributeEditor {}
impl ::core::fmt::Debug for IMFTopologyNodeAttributeEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTopologyNodeAttributeEditor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTopologyServiceLookup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTopologyServiceLookup {}
impl ::core::fmt::Debug for IMFTopologyServiceLookup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTopologyServiceLookup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTopologyServiceLookupClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTopologyServiceLookupClient {}
impl ::core::fmt::Debug for IMFTopologyServiceLookupClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTopologyServiceLookupClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTrackedSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTrackedSample {}
impl ::core::fmt::Debug for IMFTrackedSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTrackedSample").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTranscodeProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTranscodeProfile {}
impl ::core::fmt::Debug for IMFTranscodeProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTranscodeProfile").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTranscodeSinkInfoProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTranscodeSinkInfoProvider {}
impl ::core::fmt::Debug for IMFTranscodeSinkInfoProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTranscodeSinkInfoProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTransform {}
impl ::core::fmt::Debug for IMFTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTransform").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTrustedInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTrustedInput {}
impl ::core::fmt::Debug for IMFTrustedInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTrustedInput").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFTrustedOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFTrustedOutput {}
impl ::core::fmt::Debug for IMFTrustedOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFTrustedOutput").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoCaptureSampleAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoCaptureSampleAllocator {}
impl ::core::fmt::Debug for IMFVideoCaptureSampleAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoCaptureSampleAllocator").field(&self.0).finish()
    }
}
impl IMFVideoCaptureSampleAllocator {
    pub unsafe fn SetDirectXManager<P0>(&self, pmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDirectXManager)(::windows::core::Vtable::as_raw(self), pmanager.into().abi()).ok()
    }
    pub unsafe fn UninitializeSampleAllocator(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UninitializeSampleAllocator)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InitializeSampleAllocator<P0>(&self, crequestedframes: u32, pmediatype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeSampleAllocator)(::windows::core::Vtable::as_raw(self), crequestedframes, pmediatype.into().abi()).ok()
    }
    pub unsafe fn AllocateSample(&self) -> ::windows::core::Result<IMFSample> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AllocateSample)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFVideoDeviceID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoDeviceID {}
impl ::core::fmt::Debug for IMFVideoDeviceID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoDeviceID").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoDisplayControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoDisplayControl {}
impl ::core::fmt::Debug for IMFVideoDisplayControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoDisplayControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoMediaType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoMediaType {}
impl ::core::fmt::Debug for IMFVideoMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoMediaType").field(&self.0).finish()
    }
}
impl IMFVideoMediaType {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
    pub unsafe fn GetMajorType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMajorType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCompressedFormat(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsCompressedFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn IsEqual<P0>(&self, pimediatype: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsEqual)(::windows::core::Vtable::as_raw(self), pimediatype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRepresentation(&self, guidrepresentation: ::windows::core::GUID, ppvrepresentation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRepresentation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidrepresentation), ppvrepresentation).ok()
    }
    pub unsafe fn FreeRepresentation(&self, guidrepresentation: ::windows::core::GUID, pvrepresentation: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreeRepresentation)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidrepresentation), pvrepresentation).ok()
    }
}
impl ::core::cmp::PartialEq for IMFVideoMixerBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoMixerBitmap {}
impl ::core::fmt::Debug for IMFVideoMixerBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoMixerBitmap").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoMixerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoMixerControl {}
impl ::core::fmt::Debug for IMFVideoMixerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoMixerControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoMixerControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoMixerControl2 {}
impl ::core::fmt::Debug for IMFVideoMixerControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoMixerControl2").field(&self.0).finish()
    }
}
impl IMFVideoMixerControl2 {
    pub unsafe fn SetStreamZOrder(&self, dwstreamid: u32, dwz: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStreamZOrder)(::windows::core::Vtable::as_raw(self), dwstreamid, dwz).ok()
    }
    pub unsafe fn GetStreamZOrder(&self, dwstreamid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamZOrder)(::windows::core::Vtable::as_raw(self), dwstreamid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamOutputRect(&self, dwstreamid: u32, pnrcoutput: *const MFVideoNormalizedRect) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStreamOutputRect)(::windows::core::Vtable::as_raw(self), dwstreamid, pnrcoutput).ok()
    }
    pub unsafe fn GetStreamOutputRect(&self, dwstreamid: u32) -> ::windows::core::Result<MFVideoNormalizedRect> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamOutputRect)(::windows::core::Vtable::as_raw(self), dwstreamid, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFVideoPositionMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoPositionMapper {}
impl ::core::fmt::Debug for IMFVideoPositionMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoPositionMapper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoPresenter {}
impl ::core::fmt::Debug for IMFVideoPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoPresenter").field(&self.0).finish()
    }
}
impl IMFVideoPresenter {
    pub unsafe fn OnClockStart(&self, hnssystemtime: i64, llclockstartoffset: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockStart)(::windows::core::Vtable::as_raw(self), hnssystemtime, llclockstartoffset).ok()
    }
    pub unsafe fn OnClockStop(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockStop)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockPause(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockPause)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockRestart(&self, hnssystemtime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockRestart)(::windows::core::Vtable::as_raw(self), hnssystemtime).ok()
    }
    pub unsafe fn OnClockSetRate(&self, hnssystemtime: i64, flrate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnClockSetRate)(::windows::core::Vtable::as_raw(self), hnssystemtime, flrate).ok()
    }
}
impl ::core::cmp::PartialEq for IMFVideoProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoProcessor {}
impl ::core::fmt::Debug for IMFVideoProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoProcessor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoProcessorControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoProcessorControl {}
impl ::core::fmt::Debug for IMFVideoProcessorControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoProcessorControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoProcessorControl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoProcessorControl2 {}
impl ::core::fmt::Debug for IMFVideoProcessorControl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoProcessorControl2").field(&self.0).finish()
    }
}
impl IMFVideoProcessorControl2 {
    pub unsafe fn SetBorderColor(&self, pbordercolor: ::core::option::Option<*const MFARGB>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBorderColor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbordercolor.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRectangle(&self, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSourceRectangle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDestinationRectangle(&self, pdstrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDestinationRectangle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdstrect.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetMirror(&self, emirror: MF_VIDEO_PROCESSOR_MIRROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMirror)(::windows::core::Vtable::as_raw(self), emirror).ok()
    }
    pub unsafe fn SetRotation(&self, erotation: MF_VIDEO_PROCESSOR_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRotation)(::windows::core::Vtable::as_raw(self), erotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConstrictionSize(&self, pconstrictionsize: ::core::option::Option<*const super::super::Foundation::SIZE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetConstrictionSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pconstrictionsize.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for IMFVideoProcessorControl3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoProcessorControl3 {}
impl ::core::fmt::Debug for IMFVideoProcessorControl3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoProcessorControl3").field(&self.0).finish()
    }
}
impl IMFVideoProcessorControl3 {
    pub unsafe fn SetBorderColor(&self, pbordercolor: ::core::option::Option<*const MFARGB>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBorderColor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbordercolor.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRectangle(&self, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSourceRectangle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDestinationRectangle(&self, pdstrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDestinationRectangle)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdstrect.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetMirror(&self, emirror: MF_VIDEO_PROCESSOR_MIRROR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMirror)(::windows::core::Vtable::as_raw(self), emirror).ok()
    }
    pub unsafe fn SetRotation(&self, erotation: MF_VIDEO_PROCESSOR_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRotation)(::windows::core::Vtable::as_raw(self), erotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetConstrictionSize(&self, pconstrictionsize: ::core::option::Option<*const super::super::Foundation::SIZE>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetConstrictionSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pconstrictionsize.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetRotationOverride(&self, uirotation: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRotationOverride)(::windows::core::Vtable::as_raw(self), uirotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableHardwareEffects<P0>(&self, fenabled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnableHardwareEffects)(::windows::core::Vtable::as_raw(self), fenabled.into()).ok()
    }
    pub unsafe fn GetSupportedHardwareEffects(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSupportedHardwareEffects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFVideoRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoRenderer {}
impl ::core::fmt::Debug for IMFVideoRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoRenderer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoRendererEffectControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoRendererEffectControl {}
impl ::core::fmt::Debug for IMFVideoRendererEffectControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoRendererEffectControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoSampleAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoSampleAllocator {}
impl ::core::fmt::Debug for IMFVideoSampleAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoSampleAllocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoSampleAllocatorCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoSampleAllocatorCallback {}
impl ::core::fmt::Debug for IMFVideoSampleAllocatorCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoSampleAllocatorCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoSampleAllocatorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoSampleAllocatorEx {}
impl ::core::fmt::Debug for IMFVideoSampleAllocatorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoSampleAllocatorEx").field(&self.0).finish()
    }
}
impl IMFVideoSampleAllocatorEx {
    pub unsafe fn SetDirectXManager<P0>(&self, pmanager: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDirectXManager)(::windows::core::Vtable::as_raw(self), pmanager.into().abi()).ok()
    }
    pub unsafe fn UninitializeSampleAllocator(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UninitializeSampleAllocator)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InitializeSampleAllocator<P0>(&self, crequestedframes: u32, pmediatype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFMediaType>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeSampleAllocator)(::windows::core::Vtable::as_raw(self), crequestedframes, pmediatype.into().abi()).ok()
    }
    pub unsafe fn AllocateSample(&self) -> ::windows::core::Result<IMFSample> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AllocateSample)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IMFVideoSampleAllocatorNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoSampleAllocatorNotify {}
impl ::core::fmt::Debug for IMFVideoSampleAllocatorNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoSampleAllocatorNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFVideoSampleAllocatorNotifyEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVideoSampleAllocatorNotifyEx {}
impl ::core::fmt::Debug for IMFVideoSampleAllocatorNotifyEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVideoSampleAllocatorNotifyEx").field(&self.0).finish()
    }
}
impl IMFVideoSampleAllocatorNotifyEx {
    pub unsafe fn NotifyRelease(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NotifyRelease)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IMFVirtualCamera {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFVirtualCamera {}
impl ::core::fmt::Debug for IMFVirtualCamera {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFVirtualCamera").field(&self.0).finish()
    }
}
impl IMFVirtualCamera {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItem(&self, guidkey: *const ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItem)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetItemType(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<MF_ATTRIBUTE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetItemType)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn CompareItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareItem)(::windows::core::Vtable::as_raw(self), guidkey, value, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, ptheirs: P0, matchtype: MF_ATTRIBUTES_MATCH_TYPE) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), ptheirs.into().abi(), matchtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT32(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDouble(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDouble)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGUID(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGUID)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringLength)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetString(&self, guidkey: *const ::windows::core::GUID, pwszvalue: &mut [u16], pcchlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetString)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pwszvalue.as_ptr()), pwszvalue.len() as _, ::core::mem::transmute(pcchlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const ::windows::core::GUID, ppwszvalue: *mut ::windows::core::PWSTR, pcchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedString)(::windows::core::Vtable::as_raw(self), guidkey, ppwszvalue, pcchlength).ok()
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBlobSize)(::windows::core::Vtable::as_raw(self), guidkey, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &mut [u8], pcbblobsize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _, ::core::mem::transmute(pcbblobsize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const ::windows::core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAllocatedBlob)(::windows::core::Vtable::as_raw(self), guidkey, ppbuf, pcbsize).ok()
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetItem(&self, guidkey: *const ::windows::core::GUID, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetItem)(::windows::core::Vtable::as_raw(self), guidkey, value).ok()
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteItem)(::windows::core::Vtable::as_raw(self), guidkey).ok()
    }
    pub unsafe fn DeleteAllItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteAllItems)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const ::windows::core::GUID, unvalue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT32)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const ::windows::core::GUID, unvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUINT64)(::windows::core::Vtable::as_raw(self), guidkey, unvalue).ok()
    }
    pub unsafe fn SetDouble(&self, guidkey: *const ::windows::core::GUID, fvalue: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDouble)(::windows::core::Vtable::as_raw(self), guidkey, fvalue).ok()
    }
    pub unsafe fn SetGUID(&self, guidkey: *const ::windows::core::GUID, guidvalue: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGUID)(::windows::core::Vtable::as_raw(self), guidkey, guidvalue).ok()
    }
    pub unsafe fn SetString<P0>(&self, guidkey: *const ::windows::core::GUID, wszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetString)(::windows::core::Vtable::as_raw(self), guidkey, wszvalue.into().abi()).ok()
    }
    pub unsafe fn SetBlob(&self, guidkey: *const ::windows::core::GUID, pbuf: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBlob)(::windows::core::Vtable::as_raw(self), guidkey, ::core::mem::transmute(pbuf.as_ptr()), pbuf.len() as _).ok()
    }
    pub unsafe fn SetUnknown<P0>(&self, guidkey: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUnknown)(::windows::core::Vtable::as_raw(self), guidkey, punknown.into().abi()).ok()
    }
    pub unsafe fn LockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn UnlockStore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockStore)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetItemByIndex(&self, unindex: u32, pguidkey: *mut ::windows::core::GUID, pvalue: ::core::option::Option<*mut super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetItemByIndex)(::windows::core::Vtable::as_raw(self), unindex, pguidkey, ::core::mem::transmute(pvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAttributes>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyAllItems)(::windows::core::Vtable::as_raw(self), pdest.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IMFWorkQueueServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFWorkQueueServices {}
impl ::core::fmt::Debug for IMFWorkQueueServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFWorkQueueServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMFWorkQueueServicesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMFWorkQueueServicesEx {}
impl ::core::fmt::Debug for IMFWorkQueueServicesEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMFWorkQueueServicesEx").field(&self.0).finish()
    }
}
impl IMFWorkQueueServicesEx {
    pub unsafe fn BeginRegisterTopologyWorkQueuesWithMMCSS<P0, P1>(&self, pcallback: P0, pstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginRegisterTopologyWorkQueuesWithMMCSS)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), pstate.into().abi()).ok()
    }
    pub unsafe fn EndRegisterTopologyWorkQueuesWithMMCSS<P0>(&self, presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndRegisterTopologyWorkQueuesWithMMCSS)(::windows::core::Vtable::as_raw(self), presult.into().abi()).ok()
    }
    pub unsafe fn BeginUnregisterTopologyWorkQueuesWithMMCSS<P0, P1>(&self, pcallback: P0, pstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginUnregisterTopologyWorkQueuesWithMMCSS)(::windows::core::Vtable::as_raw(self), pcallback.into().abi(), pstate.into().abi()).ok()
    }
    pub unsafe fn EndUnregisterTopologyWorkQueuesWithMMCSS<P0>(&self, presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndUnregisterTopologyWorkQueuesWithMMCSS)(::windows::core::Vtable::as_raw(self), presult.into().abi()).ok()
    }
    pub unsafe fn GetTopologyWorkQueueMMCSSClass(&self, dwtopologyworkqueueid: u32, pwszclass: ::windows::core::PWSTR, pcchclass: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTopologyWorkQueueMMCSSClass)(::windows::core::Vtable::as_raw(self), dwtopologyworkqueueid, ::core::mem::transmute(pwszclass), pcchclass).ok()
    }
    pub unsafe fn GetTopologyWorkQueueMMCSSTaskId(&self, dwtopologyworkqueueid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTopologyWorkQueueMMCSSTaskId)(::windows::core::Vtable::as_raw(self), dwtopologyworkqueueid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginRegisterPlatformWorkQueueWithMMCSS<P0, P1, P2>(&self, dwplatformworkqueue: u32, wszclass: P0, dwtaskid: u32, pcallback: P1, pstate: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginRegisterPlatformWorkQueueWithMMCSS)(::windows::core::Vtable::as_raw(self), dwplatformworkqueue, wszclass.into().abi(), dwtaskid, pcallback.into().abi(), pstate.into().abi()).ok()
    }
    pub unsafe fn EndRegisterPlatformWorkQueueWithMMCSS<P0>(&self, presult: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndRegisterPlatformWorkQueueWithMMCSS)(::windows::core::Vtable::as_raw(self), presult.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginUnregisterPlatformWorkQueueWithMMCSS<P0, P1>(&self, dwplatformworkqueue: u32, pcallback: P0, pstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncCallback>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginUnregisterPlatformWorkQueueWithMMCSS)(::windows::core::Vtable::as_raw(self), dwplatformworkqueue, pcallback.into().abi(), pstate.into().abi()).ok()
    }
    pub unsafe fn EndUnregisterPlatformWorkQueueWithMMCSS<P0>(&self, presult: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFAsyncResult>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndUnregisterPlatformWorkQueueWithMMCSS)(::windows::core::Vtable::as_raw(self), presult.into().abi()).ok()
    }
    pub unsafe fn GetPlaftormWorkQueueMMCSSClass(&self, dwplatformworkqueueid: u32, pwszclass: ::windows::core::PWSTR, pcchclass: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPlaftormWorkQueueMMCSSClass)(::windows::core::Vtable::as_raw(self), dwplatformworkqueueid, ::core::mem::transmute(pwszclass), pcchclass).ok()
    }
    pub unsafe fn GetPlatformWorkQueueMMCSSTaskId(&self, dwplatformworkqueueid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPlatformWorkQueueMMCSSTaskId)(::windows::core::Vtable::as_raw(self), dwplatformworkqueueid, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IOPMVideoOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOPMVideoOutput {}
impl ::core::fmt::Debug for IOPMVideoOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOPMVideoOutput").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPlayToControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayToControl {}
impl ::core::fmt::Debug for IPlayToControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayToControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPlayToControlWithCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayToControlWithCapabilities {}
impl ::core::fmt::Debug for IPlayToControlWithCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayToControlWithCapabilities").field(&self.0).finish()
    }
}
impl IPlayToControlWithCapabilities {
    pub unsafe fn Connect<P0>(&self, pfactory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IMFSharingEngineClassFactory>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Connect)(::windows::core::Vtable::as_raw(self), pfactory.into().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Disconnect)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IPlayToSourceClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPlayToSourceClassFactory {}
impl ::core::fmt::Debug for IPlayToSourceClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPlayToSourceClassFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IToc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToc {}
impl ::core::fmt::Debug for IToc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToc").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITocCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITocCollection {}
impl ::core::fmt::Debug for ITocCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITocCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITocEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITocEntry {}
impl ::core::fmt::Debug for ITocEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITocEntry").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITocEntryList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITocEntryList {}
impl ::core::fmt::Debug for ITocEntryList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITocEntryList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITocParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITocParser {}
impl ::core::fmt::Debug for ITocParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITocParser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IValidateBinding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValidateBinding {}
impl ::core::fmt::Debug for IValidateBinding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValidateBinding").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMCodecLeakyBucket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecLeakyBucket {}
impl ::core::fmt::Debug for IWMCodecLeakyBucket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecLeakyBucket").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMCodecOutputTimestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecOutputTimestamp {}
impl ::core::fmt::Debug for IWMCodecOutputTimestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecOutputTimestamp").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMCodecPrivateData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecPrivateData {}
impl ::core::fmt::Debug for IWMCodecPrivateData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecPrivateData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMCodecProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecProps {}
impl ::core::fmt::Debug for IWMCodecProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMCodecStrings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecStrings {}
impl ::core::fmt::Debug for IWMCodecStrings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecStrings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMColorConvProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMColorConvProps {}
impl ::core::fmt::Debug for IWMColorConvProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMColorConvProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMColorLegalizerProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMColorLegalizerProps {}
impl ::core::fmt::Debug for IWMColorLegalizerProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMColorLegalizerProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMFrameInterpProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMFrameInterpProps {}
impl ::core::fmt::Debug for IWMFrameInterpProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMFrameInterpProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMInterlaceProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMInterlaceProps {}
impl ::core::fmt::Debug for IWMInterlaceProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMInterlaceProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMResamplerProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMResamplerProps {}
impl ::core::fmt::Debug for IWMResamplerProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMResamplerProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMResizerProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMResizerProps {}
impl ::core::fmt::Debug for IWMResizerProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMResizerProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSampleExtensionSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSampleExtensionSupport {}
impl ::core::fmt::Debug for IWMSampleExtensionSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSampleExtensionSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMValidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMValidate {}
impl ::core::fmt::Debug for IWMValidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMValidate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMVideoDecoderHurryup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoDecoderHurryup {}
impl ::core::fmt::Debug for IWMVideoDecoderHurryup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoDecoderHurryup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMVideoDecoderReconBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoDecoderReconBuffer {}
impl ::core::fmt::Debug for IWMVideoDecoderReconBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoDecoderReconBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMVideoForceKeyFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoForceKeyFrame {}
impl ::core::fmt::Debug for IWMVideoForceKeyFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoForceKeyFrame").field(&self.0).finish()
    }
}
impl ::core::default::Default for KSMETHOD_OPMVIDEOOUTPUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KSMETHOD_OPMVIDEOOUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KSMETHOD_OPMVIDEOOUTPUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MACROBLOCK_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MACROBLOCK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.motionVectorX == other.motionVectorX && self.motionVectorY == other.motionVectorY && self.QPDelta == other.QPDelta
    }
}
impl ::core::cmp::Eq for MACROBLOCK_DATA {}
impl ::core::fmt::Debug for MACROBLOCK_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MACROBLOCK_DATA").field("flags", &self.flags).field("motionVectorX", &self.motionVectorX).field("motionVectorY", &self.motionVectorY).field("QPDelta", &self.QPDelta).finish()
    }
}
impl ::core::default::Default for MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF2DBuffer_LockFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF2DBuffer_LockFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF2DBuffer_LockFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF3DVideoOutputType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF3DVideoOutputType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF3DVideoOutputType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFARGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFARGB {
    fn eq(&self, other: &Self) -> bool {
        self.rgbBlue == other.rgbBlue && self.rgbGreen == other.rgbGreen && self.rgbRed == other.rgbRed && self.rgbAlpha == other.rgbAlpha
    }
}
impl ::core::cmp::Eq for MFARGB {}
impl ::core::fmt::Debug for MFARGB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFARGB").field("rgbBlue", &self.rgbBlue).field("rgbGreen", &self.rgbGreen).field("rgbRed", &self.rgbRed).field("rgbAlpha", &self.rgbAlpha).finish()
    }
}
impl ::core::default::Default for MFASF_INDEXER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFASF_INDEXER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFASF_INDEXER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFASF_MULTIPLEXERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFASF_MULTIPLEXERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFASF_MULTIPLEXERFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFASF_SPLITTERFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFASF_SPLITTERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFASF_SPLITTERFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFASF_STREAMSELECTOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFASF_STREAMSELECTOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFASF_STREAMSELECTOR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MFASYNCRESULT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MFASYNCRESULT {}
impl ::core::fmt::Debug for MFASYNCRESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFASYNCRESULT").field(&self.0).finish()
    }
}
impl MFASYNCRESULT {
    pub unsafe fn GetState(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStatus)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetStatus(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStatus)(::windows::core::Vtable::as_raw(self), hrstatus).ok()
    }
    pub unsafe fn GetObject(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetObject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStateNoAddRef(&self) -> ::core::option::Option<::windows::core::IUnknown> {
        (::windows::core::Vtable::vtable(self).base__.GetStateNoAddRef)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::default::Default for MFASYNC_WORKQUEUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFASYNC_WORKQUEUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFASYNC_WORKQUEUE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFAYUVSample {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFAYUVSample {
    fn eq(&self, other: &Self) -> bool {
        self.bCrValue == other.bCrValue && self.bCbValue == other.bCbValue && self.bYValue == other.bYValue && self.bSampleAlpha8 == other.bSampleAlpha8
    }
}
impl ::core::cmp::Eq for MFAYUVSample {}
impl ::core::fmt::Debug for MFAYUVSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFAYUVSample").field("bCrValue", &self.bCrValue).field("bCbValue", &self.bCbValue).field("bYValue", &self.bYValue).field("bSampleAlpha8", &self.bSampleAlpha8).finish()
    }
}
impl ::core::default::Default for MFAudioConstriction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFAudioConstriction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFAudioConstriction").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFAudioDecoderDegradationInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFAudioDecoderDegradationInfo {
    fn eq(&self, other: &Self) -> bool {
        self.eDegradationReason == other.eDegradationReason && self.eType == other.eType
    }
}
impl ::core::cmp::Eq for MFAudioDecoderDegradationInfo {}
impl ::core::fmt::Debug for MFAudioDecoderDegradationInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFAudioDecoderDegradationInfo").field("eDegradationReason", &self.eDegradationReason).field("eType", &self.eType).finish()
    }
}
impl ::core::default::Default for MFBYTESTREAM_BUFFERING_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFBYTESTREAM_BUFFERING_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbTotalFileSize == other.cbTotalFileSize && self.cbPlayableDataSize == other.cbPlayableDataSize && self.prgBuckets == other.prgBuckets && self.cBuckets == other.cBuckets && self.qwNetBufferingTime == other.qwNetBufferingTime && self.qwExtraBufferingTimeDuringSeek == other.qwExtraBufferingTimeDuringSeek && self.qwPlayDuration == other.qwPlayDuration && self.dRate == other.dRate
    }
}
impl ::core::cmp::Eq for MFBYTESTREAM_BUFFERING_PARAMS {}
impl ::core::fmt::Debug for MFBYTESTREAM_BUFFERING_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFBYTESTREAM_BUFFERING_PARAMS").field("cbTotalFileSize", &self.cbTotalFileSize).field("cbPlayableDataSize", &self.cbPlayableDataSize).field("prgBuckets", &self.prgBuckets).field("cBuckets", &self.cBuckets).field("qwNetBufferingTime", &self.qwNetBufferingTime).field("qwExtraBufferingTimeDuringSeek", &self.qwExtraBufferingTimeDuringSeek).field("qwPlayDuration", &self.qwPlayDuration).field("dRate", &self.dRate).finish()
    }
}
impl ::core::default::Default for MFBYTESTREAM_SEEK_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFBYTESTREAM_SEEK_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFBYTESTREAM_SEEK_ORIGIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFCLOCK_CHARACTERISTICS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFCLOCK_CHARACTERISTICS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFCLOCK_CHARACTERISTICS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFCLOCK_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCLOCK_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.qwCorrelationRate == other.qwCorrelationRate && self.guidClockId == other.guidClockId && self.dwClockFlags == other.dwClockFlags && self.qwClockFrequency == other.qwClockFrequency && self.dwClockTolerance == other.dwClockTolerance && self.dwClockJitter == other.dwClockJitter
    }
}
impl ::core::cmp::Eq for MFCLOCK_PROPERTIES {}
impl ::core::fmt::Debug for MFCLOCK_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCLOCK_PROPERTIES").field("qwCorrelationRate", &self.qwCorrelationRate).field("guidClockId", &self.guidClockId).field("dwClockFlags", &self.dwClockFlags).field("qwClockFrequency", &self.qwClockFrequency).field("dwClockTolerance", &self.dwClockTolerance).field("dwClockJitter", &self.dwClockJitter).finish()
    }
}
impl ::core::default::Default for MFCLOCK_RELATIONAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFCLOCK_RELATIONAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFCLOCK_RELATIONAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFCLOCK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFCLOCK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFCLOCK_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.HWProtectionFunctionID == other.HWProtectionFunctionID && self.PrivateDataByteCount == other.PrivateDataByteCount && self.HWProtectionDataByteCount == other.HWProtectionDataByteCount && self.Reserved == other.Reserved && self.InputData == other.InputData
    }
}
impl ::core::cmp::Eq for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {}
impl ::core::fmt::Debug for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCONTENTPROTECTIONDEVICE_INPUT_DATA").field("HWProtectionFunctionID", &self.HWProtectionFunctionID).field("PrivateDataByteCount", &self.PrivateDataByteCount).field("HWProtectionDataByteCount", &self.HWProtectionDataByteCount).field("Reserved", &self.Reserved).field("InputData", &self.InputData).finish()
    }
}
impl ::core::default::Default for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PrivateDataByteCount == other.PrivateDataByteCount && self.MaxHWProtectionDataByteCount == other.MaxHWProtectionDataByteCount && self.HWProtectionDataByteCount == other.HWProtectionDataByteCount && self.Status == other.Status && self.TransportTimeInHundredsOfNanoseconds == other.TransportTimeInHundredsOfNanoseconds && self.ExecutionTimeInHundredsOfNanoseconds == other.ExecutionTimeInHundredsOfNanoseconds && self.OutputData == other.OutputData
    }
}
impl ::core::cmp::Eq for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {}
impl ::core::fmt::Debug for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA")
            .field("PrivateDataByteCount", &self.PrivateDataByteCount)
            .field("MaxHWProtectionDataByteCount", &self.MaxHWProtectionDataByteCount)
            .field("HWProtectionDataByteCount", &self.HWProtectionDataByteCount)
            .field("Status", &self.Status)
            .field("TransportTimeInHundredsOfNanoseconds", &self.TransportTimeInHundredsOfNanoseconds)
            .field("ExecutionTimeInHundredsOfNanoseconds", &self.ExecutionTimeInHundredsOfNanoseconds)
            .field("OutputData", &self.OutputData)
            .finish()
    }
}
impl ::core::default::Default for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.TaskIndex == other.TaskIndex && self.ClassName == other.ClassName && self.BasePriority == other.BasePriority
    }
}
impl ::core::cmp::Eq for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {}
impl ::core::fmt::Debug for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA").field("TaskIndex", &self.TaskIndex).field("ClassName", &self.ClassName).field("BasePriority", &self.BasePriority).finish()
    }
}
impl ::core::default::Default for MFCameraExtrinsic_CalibratedTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraExtrinsic_CalibratedTransform {
    fn eq(&self, other: &Self) -> bool {
        self.CalibrationId == other.CalibrationId && self.Position == other.Position && self.Orientation == other.Orientation
    }
}
impl ::core::cmp::Eq for MFCameraExtrinsic_CalibratedTransform {}
impl ::core::fmt::Debug for MFCameraExtrinsic_CalibratedTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraExtrinsic_CalibratedTransform").field("CalibrationId", &self.CalibrationId).field("Position", &self.Position).field("Orientation", &self.Orientation).finish()
    }
}
impl ::core::default::Default for MFCameraExtrinsics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraExtrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.TransformCount == other.TransformCount && self.CalibratedTransforms == other.CalibratedTransforms
    }
}
impl ::core::cmp::Eq for MFCameraExtrinsics {}
impl ::core::fmt::Debug for MFCameraExtrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraExtrinsics").field("TransformCount", &self.TransformCount).field("CalibratedTransforms", &self.CalibratedTransforms).finish()
    }
}
impl ::core::default::Default for MFCameraIntrinsic_CameraModel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraIntrinsic_CameraModel {
    fn eq(&self, other: &Self) -> bool {
        self.FocalLength_x == other.FocalLength_x && self.FocalLength_y == other.FocalLength_y && self.PrincipalPoint_x == other.PrincipalPoint_x && self.PrincipalPoint_y == other.PrincipalPoint_y
    }
}
impl ::core::cmp::Eq for MFCameraIntrinsic_CameraModel {}
impl ::core::fmt::Debug for MFCameraIntrinsic_CameraModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraIntrinsic_CameraModel").field("FocalLength_x", &self.FocalLength_x).field("FocalLength_y", &self.FocalLength_y).field("PrincipalPoint_x", &self.PrincipalPoint_x).field("PrincipalPoint_y", &self.PrincipalPoint_y).finish()
    }
}
impl ::core::default::Default for MFCameraIntrinsic_DistortionModel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraIntrinsic_DistortionModel {
    fn eq(&self, other: &Self) -> bool {
        self.Radial_k1 == other.Radial_k1 && self.Radial_k2 == other.Radial_k2 && self.Radial_k3 == other.Radial_k3 && self.Tangential_p1 == other.Tangential_p1 && self.Tangential_p2 == other.Tangential_p2
    }
}
impl ::core::cmp::Eq for MFCameraIntrinsic_DistortionModel {}
impl ::core::fmt::Debug for MFCameraIntrinsic_DistortionModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraIntrinsic_DistortionModel").field("Radial_k1", &self.Radial_k1).field("Radial_k2", &self.Radial_k2).field("Radial_k3", &self.Radial_k3).field("Tangential_p1", &self.Tangential_p1).field("Tangential_p2", &self.Tangential_p2).finish()
    }
}
impl ::core::default::Default for MFCameraIntrinsic_DistortionModel6KT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraIntrinsic_DistortionModel6KT {
    fn eq(&self, other: &Self) -> bool {
        self.Radial_k1 == other.Radial_k1 && self.Radial_k2 == other.Radial_k2 && self.Radial_k3 == other.Radial_k3 && self.Radial_k4 == other.Radial_k4 && self.Radial_k5 == other.Radial_k5 && self.Radial_k6 == other.Radial_k6 && self.Tangential_p1 == other.Tangential_p1 && self.Tangential_p2 == other.Tangential_p2
    }
}
impl ::core::cmp::Eq for MFCameraIntrinsic_DistortionModel6KT {}
impl ::core::fmt::Debug for MFCameraIntrinsic_DistortionModel6KT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraIntrinsic_DistortionModel6KT").field("Radial_k1", &self.Radial_k1).field("Radial_k2", &self.Radial_k2).field("Radial_k3", &self.Radial_k3).field("Radial_k4", &self.Radial_k4).field("Radial_k5", &self.Radial_k5).field("Radial_k6", &self.Radial_k6).field("Tangential_p1", &self.Tangential_p1).field("Tangential_p2", &self.Tangential_p2).finish()
    }
}
impl ::core::default::Default for MFCameraIntrinsic_DistortionModelArcTan {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraIntrinsic_DistortionModelArcTan {
    fn eq(&self, other: &Self) -> bool {
        self.Radial_k0 == other.Radial_k0 && self.DistortionCenter_x == other.DistortionCenter_x && self.DistortionCenter_y == other.DistortionCenter_y && self.Tangential_x == other.Tangential_x && self.Tangential_y == other.Tangential_y
    }
}
impl ::core::cmp::Eq for MFCameraIntrinsic_DistortionModelArcTan {}
impl ::core::fmt::Debug for MFCameraIntrinsic_DistortionModelArcTan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraIntrinsic_DistortionModelArcTan").field("Radial_k0", &self.Radial_k0).field("DistortionCenter_x", &self.DistortionCenter_x).field("DistortionCenter_y", &self.DistortionCenter_y).field("Tangential_x", &self.Tangential_x).field("Tangential_y", &self.Tangential_y).finish()
    }
}
impl ::core::default::Default for MFCameraIntrinsic_DistortionModelType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFCameraIntrinsic_DistortionModelType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFCameraIntrinsic_DistortionModelType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFCameraIntrinsic_PinholeCameraModel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFCameraIntrinsic_PinholeCameraModel {
    fn eq(&self, other: &Self) -> bool {
        self.FocalLength == other.FocalLength && self.PrincipalPoint == other.PrincipalPoint
    }
}
impl ::core::cmp::Eq for MFCameraIntrinsic_PinholeCameraModel {}
impl ::core::fmt::Debug for MFCameraIntrinsic_PinholeCameraModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFCameraIntrinsic_PinholeCameraModel").field("FocalLength", &self.FocalLength).field("PrincipalPoint", &self.PrincipalPoint).finish()
    }
}
impl ::core::default::Default for MFCameraOcclusionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFCameraOcclusionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFCameraOcclusionState").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFDepthMeasurement {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFDepthMeasurement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFDepthMeasurement").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFExtendedCameraIntrinsic_IntrinsicModel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFExtendedCameraIntrinsic_IntrinsicModel {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.SplitFrameId == other.SplitFrameId && self.CameraModel == other.CameraModel
    }
}
impl ::core::cmp::Eq for MFExtendedCameraIntrinsic_IntrinsicModel {}
impl ::core::fmt::Debug for MFExtendedCameraIntrinsic_IntrinsicModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFExtendedCameraIntrinsic_IntrinsicModel").field("Width", &self.Width).field("Height", &self.Height).field("SplitFrameId", &self.SplitFrameId).field("CameraModel", &self.CameraModel).finish()
    }
}
impl ::core::default::Default for MFFOLDDOWN_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFFOLDDOWN_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cSrcChannels == other.cSrcChannels && self.cDstChannels == other.cDstChannels && self.dwChannelMask == other.dwChannelMask && self.Coeff == other.Coeff
    }
}
impl ::core::cmp::Eq for MFFOLDDOWN_MATRIX {}
impl ::core::fmt::Debug for MFFOLDDOWN_MATRIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFFOLDDOWN_MATRIX").field("cbSize", &self.cbSize).field("cSrcChannels", &self.cSrcChannels).field("cDstChannels", &self.cDstChannels).field("dwChannelMask", &self.dwChannelMask).field("Coeff", &self.Coeff).finish()
    }
}
impl ::core::default::Default for MFFrameSourceTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFFrameSourceTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFFrameSourceTypes").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.pbTicket == other.pbTicket && self.cbTicket == other.cbTicket
    }
}
impl ::core::cmp::Eq for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {}
impl ::core::fmt::Debug for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFINPUTTRUSTAUTHORITY_ACCESS_ACTION").field("Action", &self.Action).field("pbTicket", &self.pbTicket).field("cbTicket", &self.cbTicket).finish()
    }
}
impl ::core::default::Default for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwVer == other.dwVer && self.cbSignatureOffset == other.cbSignatureOffset && self.cbSignatureSize == other.cbSignatureSize && self.cbExtensionOffset == other.cbExtensionOffset && self.cbExtensionSize == other.cbExtensionSize && self.cActions == other.cActions && self.rgOutputActions == other.rgOutputActions
    }
}
impl ::core::cmp::Eq for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {}
impl ::core::fmt::Debug for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS").field("dwSize", &self.dwSize).field("dwVer", &self.dwVer).field("cbSignatureOffset", &self.cbSignatureOffset).field("cbSignatureSize", &self.cbSignatureSize).field("cbExtensionOffset", &self.cbExtensionOffset).field("cbExtensionSize", &self.cbExtensionSize).field("cActions", &self.cActions).field("rgOutputActions", &self.rgOutputActions).finish()
    }
}
impl ::core::default::Default for MFMEDIASOURCE_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFMEDIASOURCE_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFMEDIASOURCE_CHARACTERISTICS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MFMPEG2DLNASINKSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MFMPEG2DLNASINKSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cBytesWritten == other.cBytesWritten && self.fPAL == other.fPAL && self.fccVideo == other.fccVideo && self.dwVideoWidth == other.dwVideoWidth && self.dwVideoHeight == other.dwVideoHeight && self.cVideoFramesReceived == other.cVideoFramesReceived && self.cVideoFramesEncoded == other.cVideoFramesEncoded && self.cVideoFramesSkipped == other.cVideoFramesSkipped && self.cBlackVideoFramesEncoded == other.cBlackVideoFramesEncoded && self.cVideoFramesDuplicated == other.cVideoFramesDuplicated && self.cAudioSamplesPerSec == other.cAudioSamplesPerSec && self.cAudioChannels == other.cAudioChannels && self.cAudioBytesReceived == other.cAudioBytesReceived && self.cAudioFramesEncoded == other.cAudioFramesEncoded
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MFMPEG2DLNASINKSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MFMPEG2DLNASINKSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFMPEG2DLNASINKSTATS")
            .field("cBytesWritten", &self.cBytesWritten)
            .field("fPAL", &self.fPAL)
            .field("fccVideo", &self.fccVideo)
            .field("dwVideoWidth", &self.dwVideoWidth)
            .field("dwVideoHeight", &self.dwVideoHeight)
            .field("cVideoFramesReceived", &self.cVideoFramesReceived)
            .field("cVideoFramesEncoded", &self.cVideoFramesEncoded)
            .field("cVideoFramesSkipped", &self.cVideoFramesSkipped)
            .field("cBlackVideoFramesEncoded", &self.cBlackVideoFramesEncoded)
            .field("cVideoFramesDuplicated", &self.cVideoFramesDuplicated)
            .field("cAudioSamplesPerSec", &self.cAudioSamplesPerSec)
            .field("cAudioChannels", &self.cAudioChannels)
            .field("cAudioBytesReceived", &self.cAudioBytesReceived)
            .field("cAudioFramesEncoded", &self.cAudioFramesEncoded)
            .finish()
    }
}
impl ::core::default::Default for MFMediaKeyStatus {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFMediaKeyStatus {
    fn eq(&self, other: &Self) -> bool {
        self.pbKeyId == other.pbKeyId && self.cbKeyId == other.cbKeyId && self.eMediaKeyStatus == other.eMediaKeyStatus
    }
}
impl ::core::cmp::Eq for MFMediaKeyStatus {}
impl ::core::fmt::Debug for MFMediaKeyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFMediaKeyStatus").field("pbKeyId", &self.pbKeyId).field("cbKeyId", &self.cbKeyId).field("eMediaKeyStatus", &self.eMediaKeyStatus).finish()
    }
}
impl ::core::default::Default for MFNETSOURCE_CACHE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNETSOURCE_CACHE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNETSOURCE_CACHE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNETSOURCE_PROTOCOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNETSOURCE_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNETSOURCE_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNETSOURCE_STATISTICS_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNETSOURCE_STATISTICS_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNETSOURCE_STATISTICS_IDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNETSOURCE_TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNETSOURCE_TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNETSOURCE_TRANSPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNET_PROXYSETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNET_PROXYSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNET_PROXYSETTINGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNetAuthenticationFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNetAuthenticationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNetAuthenticationFlags").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MFNetCredentialManagerGetParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MFNetCredentialManagerGetParam {
    fn eq(&self, other: &Self) -> bool {
        self.hrOp == other.hrOp && self.fAllowLoggedOnUser == other.fAllowLoggedOnUser && self.fClearTextPackage == other.fClearTextPackage && self.pszUrl == other.pszUrl && self.pszSite == other.pszSite && self.pszRealm == other.pszRealm && self.pszPackage == other.pszPackage && self.nRetries == other.nRetries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MFNetCredentialManagerGetParam {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MFNetCredentialManagerGetParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFNetCredentialManagerGetParam").field("hrOp", &self.hrOp).field("fAllowLoggedOnUser", &self.fAllowLoggedOnUser).field("fClearTextPackage", &self.fClearTextPackage).field("pszUrl", &self.pszUrl).field("pszSite", &self.pszSite).field("pszRealm", &self.pszRealm).field("pszPackage", &self.pszPackage).field("nRetries", &self.nRetries).finish()
    }
}
impl ::core::default::Default for MFNetCredentialOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNetCredentialOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNetCredentialOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNetCredentialRequirements {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNetCredentialRequirements {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNetCredentialRequirements").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFNominalRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFNominalRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFNominalRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFOffset {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFOffset {
    fn eq(&self, other: &Self) -> bool {
        self.fract == other.fract && self.value == other.value
    }
}
impl ::core::cmp::Eq for MFOffset {}
impl ::core::fmt::Debug for MFOffset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFOffset").field("fract", &self.fract).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for MFPMPSESSION_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFPMPSESSION_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFPMPSESSION_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFPOLICYMANAGER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFPOLICYMANAGER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFPOLICYMANAGER_ACTION").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::default::Default for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::PartialEq for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.dwUserData == other.dwUserData && self.fProceedWithAuthentication == other.fProceedWithAuthentication && self.hrAuthenticationStatus == other.hrAuthenticationStatus && self.pwszURL == other.pwszURL && self.pwszSite == other.pwszSite && self.pwszRealm == other.pwszRealm && self.pwszPackage == other.pwszPackage && self.nRetries == other.nRetries && self.flags == other.flags && self.pCredential == other.pCredential
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::cmp::Eq for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::core::fmt::Debug for MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_ACQUIRE_USER_CREDENTIAL_EVENT")
            .field("header", &self.header)
            .field("dwUserData", &self.dwUserData)
            .field("fProceedWithAuthentication", &self.fProceedWithAuthentication)
            .field("hrAuthenticationStatus", &self.hrAuthenticationStatus)
            .field("pwszURL", &self.pwszURL)
            .field("pwszSite", &self.pwszSite)
            .field("pwszRealm", &self.pwszRealm)
            .field("pwszPackage", &self.pwszPackage)
            .field("nRetries", &self.nRetries)
            .field("flags", &self.flags)
            .field("pCredential", &self.pCredential)
            .finish()
    }
}
impl ::core::default::Default for MFP_CREATION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFP_CREATION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFP_CREATION_OPTIONS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_ERROR_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_ERROR_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_ERROR_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_ERROR_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_ERROR_EVENT").field("header", &self.header).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_EVENT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.eEventType == other.eEventType && self.hrEvent == other.hrEvent && self.pMediaPlayer == other.pMediaPlayer && self.eState == other.eState && self.pPropertyStore == other.pPropertyStore
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_EVENT_HEADER {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_EVENT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_EVENT_HEADER").field("eEventType", &self.eEventType).field("hrEvent", &self.hrEvent).field("pMediaPlayer", &self.pMediaPlayer).field("eState", &self.eState).field("pPropertyStore", &self.pPropertyStore).finish()
    }
}
impl ::core::default::Default for MFP_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFP_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFP_EVENT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_FRAME_STEP_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_FRAME_STEP_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_FRAME_STEP_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_FRAME_STEP_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_FRAME_STEP_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_MEDIAITEM_CLEARED_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_MEDIAITEM_CLEARED_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_MEDIAITEM_CLEARED_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_MEDIAITEM_CLEARED_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_MEDIAITEM_CLEARED_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_MEDIAITEM_CREATED_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_MEDIAITEM_CREATED_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem && self.dwUserData == other.dwUserData
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_MEDIAITEM_CREATED_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_MEDIAITEM_CREATED_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_MEDIAITEM_CREATED_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).field("dwUserData", &self.dwUserData).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_MEDIAITEM_SET_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_MEDIAITEM_SET_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_MEDIAITEM_SET_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_MEDIAITEM_SET_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_MEDIAITEM_SET_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
impl ::core::default::Default for MFP_MEDIAPLAYER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFP_MEDIAPLAYER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFP_MEDIAPLAYER_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_MF_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_MF_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.MFEventType == other.MFEventType && self.pMFMediaEvent == other.pMFMediaEvent && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_MF_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_MF_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_MF_EVENT").field("header", &self.header).field("MFEventType", &self.MFEventType).field("pMFMediaEvent", &self.pMFMediaEvent).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_PAUSE_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_PAUSE_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_PAUSE_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_PAUSE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_PAUSE_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_PLAYBACK_ENDED_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_PLAYBACK_ENDED_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_PLAYBACK_ENDED_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_PLAYBACK_ENDED_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_PLAYBACK_ENDED_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_PLAY_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_PLAY_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_PLAY_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_PLAY_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_PLAY_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_POSITION_SET_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_POSITION_SET_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_POSITION_SET_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_POSITION_SET_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_POSITION_SET_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_RATE_SET_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_RATE_SET_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem && self.flRate == other.flRate
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_RATE_SET_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_RATE_SET_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_RATE_SET_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).field("flRate", &self.flRate).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for MFP_STOP_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for MFP_STOP_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.pMediaItem == other.pMediaItem
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for MFP_STOP_EVENT {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for MFP_STOP_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFP_STOP_EVENT").field("header", &self.header).field("pMediaItem", &self.pMediaItem).finish()
    }
}
impl ::core::default::Default for MFPaletteEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MFPinholeCameraIntrinsic_IntrinsicModel {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFPinholeCameraIntrinsic_IntrinsicModel {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.CameraModel == other.CameraModel && self.DistortionModel == other.DistortionModel
    }
}
impl ::core::cmp::Eq for MFPinholeCameraIntrinsic_IntrinsicModel {}
impl ::core::fmt::Debug for MFPinholeCameraIntrinsic_IntrinsicModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFPinholeCameraIntrinsic_IntrinsicModel").field("Width", &self.Width).field("Height", &self.Height).field("CameraModel", &self.CameraModel).field("DistortionModel", &self.DistortionModel).finish()
    }
}
impl ::core::default::Default for MFPinholeCameraIntrinsics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFPinholeCameraIntrinsics {
    fn eq(&self, other: &Self) -> bool {
        self.IntrinsicModelCount == other.IntrinsicModelCount && self.IntrinsicModels == other.IntrinsicModels
    }
}
impl ::core::cmp::Eq for MFPinholeCameraIntrinsics {}
impl ::core::fmt::Debug for MFPinholeCameraIntrinsics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFPinholeCameraIntrinsics").field("IntrinsicModelCount", &self.IntrinsicModelCount).field("IntrinsicModels", &self.IntrinsicModels).finish()
    }
}
impl ::core::default::Default for MFRATE_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFRATE_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFRATE_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFRR_COMPONENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFRR_COMPONENTS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRRInfoVersion == other.dwRRInfoVersion && self.dwRRComponents == other.dwRRComponents && self.pRRComponents == other.pRRComponents
    }
}
impl ::core::cmp::Eq for MFRR_COMPONENTS {}
impl ::core::fmt::Debug for MFRR_COMPONENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFRR_COMPONENTS").field("dwRRInfoVersion", &self.dwRRInfoVersion).field("dwRRComponents", &self.dwRRComponents).field("pRRComponents", &self.pRRComponents).finish()
    }
}
impl ::core::default::Default for MFRR_COMPONENT_HASH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFRR_COMPONENT_HASH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulReason == other.ulReason && self.rgHeaderHash == other.rgHeaderHash && self.rgPublicKeyHash == other.rgPublicKeyHash && self.wszName == other.wszName
    }
}
impl ::core::cmp::Eq for MFRR_COMPONENT_HASH_INFO {}
impl ::core::fmt::Debug for MFRR_COMPONENT_HASH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFRR_COMPONENT_HASH_INFO").field("ulReason", &self.ulReason).field("rgHeaderHash", &self.rgHeaderHash).field("rgPublicKeyHash", &self.rgPublicKeyHash).field("wszName", &self.wszName).finish()
    }
}
impl ::core::default::Default for MFRatio {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFRatio {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for MFRatio {}
impl ::core::fmt::Debug for MFRatio {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFRatio").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for MFSESSION_GETFULLTOPOLOGY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSESSION_GETFULLTOPOLOGY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSESSION_GETFULLTOPOLOGY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSESSION_SETTOPOLOGY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSESSION_SETTOPOLOGY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSESSION_SETTOPOLOGY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSHUTDOWN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSHUTDOWN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSHUTDOWN_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSINK_WMDRMACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSINK_WMDRMACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSINK_WMDRMACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSTREAMSINK_MARKER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSTREAMSINK_MARKER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSTREAMSINK_MARKER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSampleAllocatorUsage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSampleAllocatorUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSampleAllocatorUsage").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSampleEncryptionProtectionScheme {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSampleEncryptionProtectionScheme {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSampleEncryptionProtectionScheme").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSensorDeviceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSensorDeviceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSensorDeviceMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSensorDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSensorDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSensorDeviceType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSensorStreamType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSensorStreamType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSensorStreamType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFSequencerTopologyFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFSequencerTopologyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFSequencerTopologyFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFStandardVideoFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFStandardVideoFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFStandardVideoFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFTIMER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFTIMER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFTIMER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFTOPOLOGY_DXVA_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFTOPOLOGY_DXVA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFTOPOLOGY_DXVA_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFTOPOLOGY_HARDWARE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFTOPOLOGY_HARDWARE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFTOPOLOGY_HARDWARE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFTOPONODE_ATTRIBUTE_UPDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MFT_AUDIO_DECODER_DEGRADATION_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFT_AUDIO_DECODER_DEGRADATION_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFT_AUDIO_DECODER_DEGRADATION_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFT_AUDIO_DECODER_DEGRADATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFT_AUDIO_DECODER_DEGRADATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFT_AUDIO_DECODER_DEGRADATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFT_DRAIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFT_DRAIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFT_DRAIN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFT_ENUM_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFT_ENUM_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFT_ENUM_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MFT_ENUM_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MFT_ENUM_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MFT_ENUM_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MFT_ENUM_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MFT_ENUM_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MFT_INPUT_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_INPUT_STREAM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.hnsMaxLatency == other.hnsMaxLatency && self.dwFlags == other.dwFlags && self.cbSize == other.cbSize && self.cbMaxLookahead == other.cbMaxLookahead && self.cbAlignment == other.cbAlignment
    }
}
impl ::core::cmp::Eq for MFT_INPUT_STREAM_INFO {}
impl ::core::fmt::Debug for MFT_INPUT_STREAM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_INPUT_STREAM_INFO").field("hnsMaxLatency", &self.hnsMaxLatency).field("dwFlags", &self.dwFlags).field("cbSize", &self.cbSize).field("cbMaxLookahead", &self.cbMaxLookahead).field("cbAlignment", &self.cbAlignment).finish()
    }
}
impl ::core::default::Default for MFT_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFT_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFT_MESSAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFT_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_OUTPUT_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.dwStreamID == other.dwStreamID && self.pSample == other.pSample && self.dwStatus == other.dwStatus && self.pEvents == other.pEvents
    }
}
impl ::core::cmp::Eq for MFT_OUTPUT_DATA_BUFFER {}
impl ::core::fmt::Debug for MFT_OUTPUT_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_OUTPUT_DATA_BUFFER").field("dwStreamID", &self.dwStreamID).field("pSample", &self.pSample).field("dwStatus", &self.dwStatus).field("pEvents", &self.pEvents).finish()
    }
}
impl ::core::default::Default for MFT_OUTPUT_STREAM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_OUTPUT_STREAM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cbSize == other.cbSize && self.cbAlignment == other.cbAlignment
    }
}
impl ::core::cmp::Eq for MFT_OUTPUT_STREAM_INFO {}
impl ::core::fmt::Debug for MFT_OUTPUT_STREAM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_OUTPUT_STREAM_INFO").field("dwFlags", &self.dwFlags).field("cbSize", &self.cbSize).field("cbAlignment", &self.cbAlignment).finish()
    }
}
impl ::core::default::Default for MFT_REGISTER_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_REGISTER_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.guidMajorType == other.guidMajorType && self.guidSubtype == other.guidSubtype
    }
}
impl ::core::cmp::Eq for MFT_REGISTER_TYPE_INFO {}
impl ::core::fmt::Debug for MFT_REGISTER_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_REGISTER_TYPE_INFO").field("guidMajorType", &self.guidMajorType).field("guidSubtype", &self.guidSubtype).finish()
    }
}
impl ::core::default::Default for MFT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.clsid == other.clsid && self.guidCategory == other.guidCategory && self.uiFlags == other.uiFlags && self.pszName == other.pszName && self.cInTypes == other.cInTypes && self.pInTypes == other.pInTypes && self.cOutTypes == other.cOutTypes && self.pOutTypes == other.pOutTypes
    }
}
impl ::core::cmp::Eq for MFT_REGISTRATION_INFO {}
impl ::core::fmt::Debug for MFT_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_REGISTRATION_INFO").field("clsid", &self.clsid).field("guidCategory", &self.guidCategory).field("uiFlags", &self.uiFlags).field("pszName", &self.pszName).field("cInTypes", &self.cInTypes).field("pInTypes", &self.pInTypes).field("cOutTypes", &self.cOutTypes).field("pOutTypes", &self.pOutTypes).finish()
    }
}
impl ::core::default::Default for MFT_STREAM_STATE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_STREAM_STATE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.StreamId == other.StreamId && self.State == other.State
    }
}
impl ::core::cmp::Eq for MFT_STREAM_STATE_PARAM {}
impl ::core::fmt::Debug for MFT_STREAM_STATE_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_STREAM_STATE_PARAM").field("StreamId", &self.StreamId).field("State", &self.State).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MFVIDEOFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MFVP_MESSAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVP_MESSAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVP_MESSAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideo3DFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideo3DFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideo3DFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideo3DSampleFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideo3DSampleFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideo3DSampleFormat").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D9", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MFVideoAlphaBitmap {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MFVideoAlphaBitmapFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoAlphaBitmapFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoAlphaBitmapFlags").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MFVideoAlphaBitmapParams {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MFVideoAlphaBitmapParams {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.clrSrcKey == other.clrSrcKey && self.rcSrc == other.rcSrc && self.nrcDest == other.nrcDest && self.fAlpha == other.fAlpha && self.dwFilterMode == other.dwFilterMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MFVideoAlphaBitmapParams {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MFVideoAlphaBitmapParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFVideoAlphaBitmapParams").field("dwFlags", &self.dwFlags).field("clrSrcKey", &self.clrSrcKey).field("rcSrc", &self.rcSrc).field("nrcDest", &self.nrcDest).field("fAlpha", &self.fAlpha).field("dwFilterMode", &self.dwFilterMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MFVideoArea {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MFVideoArea {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetX == other.OffsetX && self.OffsetY == other.OffsetY && self.Area == other.Area
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MFVideoArea {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MFVideoArea {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFVideoArea").field("OffsetX", &self.OffsetX).field("OffsetY", &self.OffsetY).field("Area", &self.Area).finish()
    }
}
impl ::core::default::Default for MFVideoAspectRatioMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoAspectRatioMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoAspectRatioMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoChromaSubsampling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoChromaSubsampling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoChromaSubsampling").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoCompressedInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFVideoCompressedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.AvgBitrate == other.AvgBitrate && self.AvgBitErrorRate == other.AvgBitErrorRate && self.MaxKeyFrameSpacing == other.MaxKeyFrameSpacing
    }
}
impl ::core::cmp::Eq for MFVideoCompressedInfo {}
impl ::core::fmt::Debug for MFVideoCompressedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFVideoCompressedInfo").field("AvgBitrate", &self.AvgBitrate).field("AvgBitErrorRate", &self.AvgBitErrorRate).field("MaxKeyFrameSpacing", &self.MaxKeyFrameSpacing).finish()
    }
}
impl ::core::default::Default for MFVideoDRMFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoDRMFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoDRMFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoDSPMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoDSPMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoDSPMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoFlags").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MFVideoInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MFVideoInfo {
    fn eq(&self, other: &Self) -> bool {
        self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.PixelAspectRatio == other.PixelAspectRatio && self.SourceChromaSubsampling == other.SourceChromaSubsampling && self.InterlaceMode == other.InterlaceMode && self.TransferFunction == other.TransferFunction && self.ColorPrimaries == other.ColorPrimaries && self.TransferMatrix == other.TransferMatrix && self.SourceLighting == other.SourceLighting && self.FramesPerSecond == other.FramesPerSecond && self.NominalRange == other.NominalRange && self.GeometricAperture == other.GeometricAperture && self.MinimumDisplayAperture == other.MinimumDisplayAperture && self.PanScanAperture == other.PanScanAperture && self.VideoFlags == other.VideoFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MFVideoInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MFVideoInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFVideoInfo")
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("PixelAspectRatio", &self.PixelAspectRatio)
            .field("SourceChromaSubsampling", &self.SourceChromaSubsampling)
            .field("InterlaceMode", &self.InterlaceMode)
            .field("TransferFunction", &self.TransferFunction)
            .field("ColorPrimaries", &self.ColorPrimaries)
            .field("TransferMatrix", &self.TransferMatrix)
            .field("SourceLighting", &self.SourceLighting)
            .field("FramesPerSecond", &self.FramesPerSecond)
            .field("NominalRange", &self.NominalRange)
            .field("GeometricAperture", &self.GeometricAperture)
            .field("MinimumDisplayAperture", &self.MinimumDisplayAperture)
            .field("PanScanAperture", &self.PanScanAperture)
            .field("VideoFlags", &self.VideoFlags)
            .finish()
    }
}
impl ::core::default::Default for MFVideoInterlaceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoInterlaceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoInterlaceMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoLighting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoLighting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoLighting").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoMixPrefs {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoMixPrefs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoMixPrefs").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoNormalizedRect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFVideoNormalizedRect {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for MFVideoNormalizedRect {}
impl ::core::fmt::Debug for MFVideoNormalizedRect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFVideoNormalizedRect").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::default::Default for MFVideoPadFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoPadFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoPadFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoPrimaries {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoPrimaries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoPrimaries").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoRenderPrefs {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoRenderPrefs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoRenderPrefs").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoRotationFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoRotationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoRotationFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoSphericalFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoSphericalFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoSphericalFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoSphericalProjectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoSphericalProjectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoSphericalProjectionMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoSrcContentHintFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoSrcContentHintFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoSrcContentHintFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoSurfaceInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MFVideoTransferFunction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoTransferFunction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoTransferFunction").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVideoTransferMatrix {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVideoTransferMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVideoTransferMatrix").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVirtualCameraAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVirtualCameraAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVirtualCameraAccess").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVirtualCameraLifetime {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVirtualCameraLifetime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVirtualCameraLifetime").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFVirtualCameraType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFVirtualCameraType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFVirtualCameraType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFWaveFormatExConvertFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MFWaveFormatExConvertFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MFWaveFormatExConvertFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_ACTIVATE_CUSTOM_MIXER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_ACTIVATE_CUSTOM_MIXER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_ACTIVATE_CUSTOM_MIXER").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_ACTIVATE_CUSTOM_PRESENTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_ACTIVATE_CUSTOM_PRESENTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_ACTIVATE_CUSTOM_PRESENTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_ATTRIBUTES_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_ATTRIBUTES_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_ATTRIBUTES_MATCH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_ATTRIBUTE_SERIALIZE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_ATTRIBUTE_SERIALIZE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_ATTRIBUTE_SERIALIZE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_AUVRHP_ROOMMODEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_AUVRHP_ROOMMODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_AUVRHP_ROOMMODEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_BYTE_STREAM_CACHE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_BYTE_STREAM_CACHE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.qwStartOffset == other.qwStartOffset && self.qwEndOffset == other.qwEndOffset
    }
}
impl ::core::cmp::Eq for MF_BYTE_STREAM_CACHE_RANGE {}
impl ::core::fmt::Debug for MF_BYTE_STREAM_CACHE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_BYTE_STREAM_CACHE_RANGE").field("qwStartOffset", &self.qwStartOffset).field("qwEndOffset", &self.qwEndOffset).finish()
    }
}
impl ::core::default::Default for MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CAPTURE_ENGINE_AUDIO_PROCESSING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CAPTURE_ENGINE_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CAPTURE_ENGINE_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CAPTURE_ENGINE_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CAPTURE_ENGINE_MEDIA_CATEGORY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CAPTURE_ENGINE_SINK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CAPTURE_ENGINE_SINK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CAPTURE_ENGINE_SINK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CAPTURE_ENGINE_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CAPTURE_ENGINE_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CAPTURE_ENGINE_SOURCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CAPTURE_ENGINE_STREAM_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CAPTURE_ENGINE_STREAM_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CAPTURE_ENGINE_STREAM_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CONNECT_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CONNECT_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CONNECT_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CROSS_ORIGIN_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CROSS_ORIGIN_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CROSS_ORIGIN_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_CUSTOM_DECODE_UNIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_CUSTOM_DECODE_UNIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_CUSTOM_DECODE_UNIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_FILE_ACCESSMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_FILE_ACCESSMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_FILE_ACCESSMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_FILE_OPENMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_FILE_OPENMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_FILE_OPENMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_FLOAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_FLOAT2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for MF_FLOAT2 {}
impl ::core::fmt::Debug for MF_FLOAT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_FLOAT2").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for MF_FLOAT3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_FLOAT3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for MF_FLOAT3 {}
impl ::core::fmt::Debug for MF_FLOAT3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_FLOAT3").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::core::default::Default for MF_HDCP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_HDCP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_HDCP_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_LEAKY_BUCKET_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.dwBitrate == other.dwBitrate && self.msBufferWindow == other.msBufferWindow
    }
}
impl ::core::cmp::Eq for MF_LEAKY_BUCKET_PAIR {}
impl ::core::fmt::Debug for MF_LEAKY_BUCKET_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_LEAKY_BUCKET_PAIR").field("dwBitrate", &self.dwBitrate).field("msBufferWindow", &self.msBufferWindow).finish()
    }
}
impl ::core::default::Default for MF_MEDIAKEYSESSION_MESSAGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIAKEYSESSION_MESSAGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIAKEYSESSION_MESSAGETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIAKEYSESSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIAKEYSESSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIAKEYSESSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIAKEYS_REQUIREMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIAKEYS_REQUIREMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIAKEYS_REQUIREMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIAKEY_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIAKEY_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIAKEY_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_CANPLAY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_CANPLAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_CANPLAY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_CREATEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_CREATEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_CREATEFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_ERR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_ERR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_ERR").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_EXTENSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_EXTENSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_EXTENSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_KEYERR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_KEYERR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_KEYERR").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_NETWORK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_NETWORK").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_OPM_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_OPM_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_OPM_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_PRELOAD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_PRELOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_PRELOAD").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_PROTECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_PROTECTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_READY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_READY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_READY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_S3D_PACKING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_S3D_PACKING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_S3D_PACKING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_SEEK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_SEEK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_SEEK_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_STATISTIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_STATISTIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_STATISTIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_ENGINE_STREAMTYPE_FAILED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_ENGINE_STREAMTYPE_FAILED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_ENGINE_STREAMTYPE_FAILED").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MEDIA_SHARING_ENGINE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MEDIA_SHARING_ENGINE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MEDIA_SHARING_ENGINE_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MSE_APPEND_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MSE_APPEND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MSE_APPEND_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MSE_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MSE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MSE_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MSE_OPUS_SUPPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MSE_OPUS_SUPPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MSE_OPUS_SUPPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MSE_READY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MSE_READY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MSE_READY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MSE_VP9_SUPPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MSE_VP9_SUPPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MSE_VP9_SUPPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_MT_D3D_RESOURCE_VERSION_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_MT_D3D_RESOURCE_VERSION_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_MT_D3D_RESOURCE_VERSION_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_OPM_ACP_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_OPM_ACP_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_OPM_ACP_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_OPM_CGMSA_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_OPM_CGMSA_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_OPM_CGMSA_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_PLUGIN_CONTROL_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_PLUGIN_CONTROL_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_PLUGIN_CONTROL_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_Plugin_Type {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_Plugin_Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_Plugin_Type").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_QUALITY_ADVISE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_QUALITY_ADVISE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_QUALITY_ADVISE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_QUALITY_DROP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_QUALITY_DROP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_QUALITY_DROP_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_QUALITY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_QUALITY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_QUALITY_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_QUATERNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_QUATERNION {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl ::core::cmp::Eq for MF_QUATERNION {}
impl ::core::fmt::Debug for MF_QUATERNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_QUATERNION").field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w).finish()
    }
}
impl ::core::default::Default for MF_RESOLUTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_RESOLUTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_RESOLUTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MF_RESOLUTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MF_RESOLUTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MF_RESOLUTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MF_RESOLUTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MF_RESOLUTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MF_SERVICE_LOOKUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SERVICE_LOOKUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SERVICE_LOOKUP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_SHARING_ENGINE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SHARING_ENGINE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SHARING_ENGINE_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_SINK_WRITER_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SINK_WRITER_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SINK_WRITER_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_SINK_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_SINK_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb
            && self.llLastTimestampReceived == other.llLastTimestampReceived
            && self.llLastTimestampEncoded == other.llLastTimestampEncoded
            && self.llLastTimestampProcessed == other.llLastTimestampProcessed
            && self.llLastStreamTickReceived == other.llLastStreamTickReceived
            && self.llLastSinkSampleRequest == other.llLastSinkSampleRequest
            && self.qwNumSamplesReceived == other.qwNumSamplesReceived
            && self.qwNumSamplesEncoded == other.qwNumSamplesEncoded
            && self.qwNumSamplesProcessed == other.qwNumSamplesProcessed
            && self.qwNumStreamTicksReceived == other.qwNumStreamTicksReceived
            && self.dwByteCountQueued == other.dwByteCountQueued
            && self.qwByteCountProcessed == other.qwByteCountProcessed
            && self.dwNumOutstandingSinkSampleRequests == other.dwNumOutstandingSinkSampleRequests
            && self.dwAverageSampleRateReceived == other.dwAverageSampleRateReceived
            && self.dwAverageSampleRateEncoded == other.dwAverageSampleRateEncoded
            && self.dwAverageSampleRateProcessed == other.dwAverageSampleRateProcessed
    }
}
impl ::core::cmp::Eq for MF_SINK_WRITER_STATISTICS {}
impl ::core::fmt::Debug for MF_SINK_WRITER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_SINK_WRITER_STATISTICS")
            .field("cb", &self.cb)
            .field("llLastTimestampReceived", &self.llLastTimestampReceived)
            .field("llLastTimestampEncoded", &self.llLastTimestampEncoded)
            .field("llLastTimestampProcessed", &self.llLastTimestampProcessed)
            .field("llLastStreamTickReceived", &self.llLastStreamTickReceived)
            .field("llLastSinkSampleRequest", &self.llLastSinkSampleRequest)
            .field("qwNumSamplesReceived", &self.qwNumSamplesReceived)
            .field("qwNumSamplesEncoded", &self.qwNumSamplesEncoded)
            .field("qwNumSamplesProcessed", &self.qwNumSamplesProcessed)
            .field("qwNumStreamTicksReceived", &self.qwNumStreamTicksReceived)
            .field("dwByteCountQueued", &self.dwByteCountQueued)
            .field("qwByteCountProcessed", &self.qwByteCountProcessed)
            .field("dwNumOutstandingSinkSampleRequests", &self.dwNumOutstandingSinkSampleRequests)
            .field("dwAverageSampleRateReceived", &self.dwAverageSampleRateReceived)
            .field("dwAverageSampleRateEncoded", &self.dwAverageSampleRateEncoded)
            .field("dwAverageSampleRateProcessed", &self.dwAverageSampleRateProcessed)
            .finish()
    }
}
impl ::core::default::Default for MF_SOURCE_READER_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SOURCE_READER_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SOURCE_READER_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_SOURCE_READER_CONTROL_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SOURCE_READER_CONTROL_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SOURCE_READER_CONTROL_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_SOURCE_READER_CURRENT_TYPE_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SOURCE_READER_CURRENT_TYPE_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SOURCE_READER_CURRENT_TYPE_CONSTANTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_SOURCE_READER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_SOURCE_READER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_SOURCE_READER_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_STREAM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_STREAM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_STREAM_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_ALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_BOUTEN_POSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_BOUTEN_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_BOUTEN_POSITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_BOUTEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_BOUTEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_BOUTEN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_CUE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_CUE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_CUE_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_DECORATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_DECORATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_DECORATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_DISPLAY_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_DISPLAY_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_DISPLAY_ALIGNMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_ERROR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_ERROR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_ERROR_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_FONT_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_FONT_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_FONT_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_RUBY_ALIGN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_RUBY_ALIGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_RUBY_ALIGN").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_RUBY_POSITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_RUBY_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_RUBY_POSITION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_RUBY_RESERVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_RUBY_RESERVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_RUBY_RESERVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_SCROLL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_SCROLL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_SCROLL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_TRACK_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_TRACK_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_TRACK_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_TRACK_READY_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_TRACK_READY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_TRACK_READY_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_UNIT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_UNIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_UNIT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TIMED_TEXT_WRITING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TIMED_TEXT_WRITING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TIMED_TEXT_WRITING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TOPOLOGY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TOPOLOGY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TOPOLOGY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TOPONODE_DRAIN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TOPONODE_DRAIN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TOPONODE_DRAIN_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TOPONODE_FLUSH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TOPONODE_FLUSH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TOPONODE_FLUSH_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TOPOSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TOPOSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TOPOSTATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TRANSCODE_ADJUST_PROFILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TRANSCODE_ADJUST_PROFILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TRANSCODE_ADJUST_PROFILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_TRANSCODE_SINK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_TRANSCODE_SINK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVideoStreamID == other.dwVideoStreamID && self.pVideoMediaType == other.pVideoMediaType && self.dwAudioStreamID == other.dwAudioStreamID && self.pAudioMediaType == other.pAudioMediaType
    }
}
impl ::core::cmp::Eq for MF_TRANSCODE_SINK_INFO {}
impl ::core::fmt::Debug for MF_TRANSCODE_SINK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_TRANSCODE_SINK_INFO").field("dwVideoStreamID", &self.dwVideoStreamID).field("pVideoMediaType", &self.pVideoMediaType).field("dwAudioStreamID", &self.dwAudioStreamID).field("pAudioMediaType", &self.pAudioMediaType).finish()
    }
}
impl ::core::default::Default for MF_TRANSCODE_TOPOLOGYMODE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_TRANSCODE_TOPOLOGYMODE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_TRANSCODE_TOPOLOGYMODE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_URL_TRUST_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_URL_TRUST_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_URL_TRUST_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_VIDEO_PROCESSOR_ALGORITHM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_VIDEO_PROCESSOR_ALGORITHM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_VIDEO_PROCESSOR_ALGORITHM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_VIDEO_PROCESSOR_MIRROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_VIDEO_PROCESSOR_MIRROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_VIDEO_PROCESSOR_MIRROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_VIDEO_PROCESSOR_ROTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MF_VIDEO_PROCESSOR_ROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MF_VIDEO_PROCESSOR_ROTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    fn eq(&self, other: &Self) -> bool {
        self.iHeading == other.iHeading && self.iPitch == other.iPitch && self.iRoll == other.iRoll
    }
}
impl ::core::cmp::Eq for MF_VIDEO_SPHERICAL_VIEWDIRECTION {}
impl ::core::fmt::Debug for MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MF_VIDEO_SPHERICAL_VIEWDIRECTION").field("iHeading", &self.iHeading).field("iPitch", &self.iPitch).field("iRoll", &self.iRoll).finish()
    }
}
impl ::core::default::Default for MIC_ARRAY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MIC_ARRAY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIC_ARRAY_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOVEREGION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOVEREGION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FrameNumber == other.FrameNumber && self.NumMoveRegions == other.NumMoveRegions && self.MoveRegions == other.MoveRegions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOVEREGION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOVEREGION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVEREGION_INFO").field("FrameNumber", &self.FrameNumber).field("NumMoveRegions", &self.NumMoveRegions).field("MoveRegions", &self.MoveRegions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOVE_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.SourcePoint == other.SourcePoint && self.DestRect == other.DestRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOVE_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_RECT").field("SourcePoint", &self.SourcePoint).field("DestRect", &self.DestRect).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for MPEG1VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwStartTimeCode == other.dwStartTimeCode && self.cbSequenceHeader == other.cbSequenceHeader && self.bSequenceHeader == other.bSequenceHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for MPEG1VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for MPEG1VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPEG1VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("bSequenceHeader", &self.bSequenceHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for MPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MPEG2VIDEOINFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MPEG2VIDEOINFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPEG2VIDEOINFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MPEG2VIDEOINFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MPEG2VIDEOINFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MPEG2VIDEOINFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MPEG2VIDEOINFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MPEG2VIDEOINFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MT_ARBITRARY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MT_ARBITRARY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MT_ARBITRARY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MT_ARBITRARY_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MT_ARBITRARY_HEADER").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).finish()
    }
}
impl ::core::default::Default for MT_CUSTOM_VIDEO_PRIMARIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MT_CUSTOM_VIDEO_PRIMARIES {
    fn eq(&self, other: &Self) -> bool {
        self.fRx == other.fRx && self.fRy == other.fRy && self.fGx == other.fGx && self.fGy == other.fGy && self.fBx == other.fBx && self.fBy == other.fBy && self.fWx == other.fWx && self.fWy == other.fWy
    }
}
impl ::core::cmp::Eq for MT_CUSTOM_VIDEO_PRIMARIES {}
impl ::core::fmt::Debug for MT_CUSTOM_VIDEO_PRIMARIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MT_CUSTOM_VIDEO_PRIMARIES").field("fRx", &self.fRx).field("fRy", &self.fRy).field("fGx", &self.fGx).field("fGy", &self.fGy).field("fBx", &self.fBx).field("fBy", &self.fBy).field("fWx", &self.fWx).field("fWy", &self.fWy).finish()
    }
}
impl ::core::default::Default for OPM_ACP_AND_CGMSA_SIGNALING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_ACP_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_ACP_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_ACP_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D9")]
impl ::core::default::Default for OPM_ACTUAL_OUTPUT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_BUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_BUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_BUS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_CGMSA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_CGMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_CGMSA").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_CONFIGURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_CONNECTED_HDCP_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_CONNECTOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_CONNECTOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_CONNECTOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_COPP_COMPATIBLE_GET_INFO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_DPCP_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_DPCP_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_DPCP_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_DVI_CHARACTERISTIC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_DVI_CHARACTERISTIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_DVI_CHARACTERISTIC").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.abEncryptedInitializationParameters == other.abEncryptedInitializationParameters
    }
}
impl ::core::cmp::Eq for OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {}
impl ::core::fmt::Debug for OPM_ENCRYPTED_INITIALIZATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPM_ENCRYPTED_INITIALIZATION_PARAMETERS").field("abEncryptedInitializationParameters", &self.abEncryptedInitializationParameters).finish()
    }
}
impl ::core::default::Default for OPM_GET_CODEC_INFO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_GET_CODEC_INFO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_GET_INFO_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_HDCP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_HDCP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_HDCP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OPM_HDCP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPM_HDCP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPM_HDCP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPM_HDCP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPM_HDCP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for OPM_HDCP_KEY_SELECTION_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPM_HDCP_KEY_SELECTION_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.abKeySelectionVector == other.abKeySelectionVector
    }
}
impl ::core::cmp::Eq for OPM_HDCP_KEY_SELECTION_VECTOR {}
impl ::core::fmt::Debug for OPM_HDCP_KEY_SELECTION_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPM_HDCP_KEY_SELECTION_VECTOR").field("abKeySelectionVector", &self.abKeySelectionVector).finish()
    }
}
impl ::core::default::Default for OPM_HDCP_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_HDCP_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_HDCP_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_HDCP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_HDCP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_HDCP_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_HDCP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_HDCP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_HDCP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_IMAGE_ASPECT_RATIO_EN300294 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_IMAGE_ASPECT_RATIO_EN300294 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_IMAGE_ASPECT_RATIO_EN300294").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_OMAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPM_OMAC {
    fn eq(&self, other: &Self) -> bool {
        self.abOMAC == other.abOMAC
    }
}
impl ::core::cmp::Eq for OPM_OMAC {}
impl ::core::fmt::Debug for OPM_OMAC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPM_OMAC").field("abOMAC", &self.abOMAC).finish()
    }
}
impl ::core::default::Default for OPM_OUTPUT_HARDWARE_PROTECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_OUTPUT_HARDWARE_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_OUTPUT_HARDWARE_PROTECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_OUTPUT_ID_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_PROTECTION_STANDARD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_PROTECTION_STANDARD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_PROTECTION_STANDARD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_PROTECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_PROTECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_PROTECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_RANDOM_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPM_RANDOM_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.abRandomNumber == other.abRandomNumber
    }
}
impl ::core::cmp::Eq for OPM_RANDOM_NUMBER {}
impl ::core::fmt::Debug for OPM_RANDOM_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPM_RANDOM_NUMBER").field("abRandomNumber", &self.abRandomNumber).finish()
    }
}
impl ::core::default::Default for OPM_REQUESTED_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_SET_ACP_AND_CGMSA_SIGNALING_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_SET_HDCP_SRM_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_SET_PROTECTION_LEVEL_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for OPM_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_TYPE_ENFORCEMENT_HDCP_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for OPM_VIDEO_OUTPUT_SEMANTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPM_VIDEO_OUTPUT_SEMANTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPM_VIDEO_OUTPUT_SEMANTICS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PLAYTO_SOURCE_CREATEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PLAYTO_SOURCE_CREATEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PLAYTO_SOURCE_CREATEFLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ROI_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ROI_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.rect == other.rect && self.QPDelta == other.QPDelta
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ROI_AREA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ROI_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROI_AREA").field("rect", &self.rect).field("QPDelta", &self.QPDelta).finish()
    }
}
impl ::core::default::Default for SAMPLE_PROTECTION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SAMPLE_PROTECTION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SAMPLE_PROTECTION_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEEK_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEEK_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEEK_ORIGIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for SENSORPROFILEID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SENSORPROFILEID {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Index == other.Index && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for SENSORPROFILEID {}
impl ::core::fmt::Debug for SENSORPROFILEID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SENSORPROFILEID").field("Type", &self.Type).field("Index", &self.Index).field("Unused", &self.Unused).finish()
    }
}
impl ::core::default::Default for STREAM_MEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STREAM_MEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.gidMedium == other.gidMedium && self.unMediumInstance == other.unMediumInstance
    }
}
impl ::core::cmp::Eq for STREAM_MEDIUM {}
impl ::core::fmt::Debug for STREAM_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_MEDIUM").field("gidMedium", &self.gidMedium).field("unMediumInstance", &self.unMediumInstance).finish()
    }
}
impl ::core::default::Default for TOC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOC_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.guidID == other.guidID && self.wStreamNumber == other.wStreamNumber && self.guidType == other.guidType && self.wLanguageIndex == other.wLanguageIndex
    }
}
impl ::core::cmp::Eq for TOC_DESCRIPTOR {}
impl ::core::fmt::Debug for TOC_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOC_DESCRIPTOR").field("guidID", &self.guidID).field("wStreamNumber", &self.wStreamNumber).field("guidType", &self.guidType).field("wLanguageIndex", &self.wLanguageIndex).finish()
    }
}
impl ::core::default::Default for TOC_ENTRY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TOC_ENTRY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.qwStartTime == other.qwStartTime && self.qwEndTime == other.qwEndTime && self.qwStartPacketOffset == other.qwStartPacketOffset && self.qwEndPacketOffset == other.qwEndPacketOffset && self.qwRepresentativeFrameTime == other.qwRepresentativeFrameTime
    }
}
impl ::core::cmp::Eq for TOC_ENTRY_DESCRIPTOR {}
impl ::core::fmt::Debug for TOC_ENTRY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TOC_ENTRY_DESCRIPTOR").field("qwStartTime", &self.qwStartTime).field("qwEndTime", &self.qwEndTime).field("qwStartPacketOffset", &self.qwStartPacketOffset).field("qwEndPacketOffset", &self.qwEndPacketOffset).field("qwRepresentativeFrameTime", &self.qwRepresentativeFrameTime).finish()
    }
}
impl ::core::default::Default for TOC_POS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TOC_POS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TOC_POS_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for VIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for VIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for VIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for VIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for VIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WMT_PROP_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_PROP_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PROP_DATATYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMV_DYNAMIC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMV_DYNAMIC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMV_DYNAMIC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFP_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFP_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFP_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFP_MEDIAITEM_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFP_MEDIAITEM_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFP_MEDIAITEM_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_INPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_INPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_INPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_INPUT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_INPUT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_INPUT_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_INPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_INPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_INPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_OUTPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_OUTPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_OUTPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_OUTPUT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_OUTPUT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_OUTPUT_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_OUTPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_OUTPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_OUTPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_PROCESS_OUTPUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_PROCESS_OUTPUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_PROCESS_OUTPUT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_PROCESS_OUTPUT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_PROCESS_OUTPUT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_PROCESS_OUTPUT_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _MFT_SET_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _MFT_SET_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_MFT_SET_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVAudioChannelConfig {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVAudioChannelConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVAudioChannelConfig").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDDSurroundMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDDSurroundMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDDSurroundMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDSPLoudnessEqualization {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDSPLoudnessEqualization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDSPLoudnessEqualization").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDSPSpeakerFill {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDSPSpeakerFill {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDSPSpeakerFill").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecAACDownmixMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecAACDownmixMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecAACDownmixMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecAudioDualMono {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecAudioDualMono {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecAudioDualMono").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecAudioDualMonoReproMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecAudioDualMonoReproMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecAudioDualMonoReproMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecDDMatrixDecodingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecDDMatrixDecodingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecDDMatrixDecodingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecDDOperationalMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecDDOperationalMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecDDOperationalMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecDDStereoDownMixMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecDDStereoDownMixMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecDDStereoDownMixMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecHEAACDynamicRangeControl {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecHEAACDynamicRangeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecHEAACDynamicRangeControl").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoCodecType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoCodecType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoCodecType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoDXVABusEncryption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoDXVABusEncryption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoDXVABusEncryption").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoDXVAMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoDXVAMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoDXVAMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoH264ErrorConcealment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoH264ErrorConcealment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoH264ErrorConcealment").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoInputScanType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoInputScanType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoInputScanType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoMPEG2ErrorConcealment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoMPEG2ErrorConcealment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoMPEG2ErrorConcealment").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoSWPowerLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoSWPowerLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoSWPowerLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVDecVideoSoftwareDeinterlaceMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVDecVideoSoftwareDeinterlaceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVDecVideoSoftwareDeinterlaceMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncAdaptiveMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncAdaptiveMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncAdaptiveMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncAudioDualMono {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncAudioDualMono {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncAudioDualMono").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncAudioInputContent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncAudioInputContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncAudioInputContent").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncChromaEncodeMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncChromaEncodeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncChromaEncodeMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncCommonRateControlMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncCommonRateControlMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncCommonRateControlMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncCommonStreamEndHandling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncCommonStreamEndHandling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncCommonStreamEndHandling").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDAtoDConverterType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDAtoDConverterType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDAtoDConverterType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDDynamicRangeCompressionControl {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDDynamicRangeCompressionControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDDynamicRangeCompressionControl").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDHeadphoneMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDHeadphoneMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDHeadphoneMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDPreferredStereoDownMixMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDPreferredStereoDownMixMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDPreferredStereoDownMixMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDProductionRoomType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDProductionRoomType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDProductionRoomType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDService {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDService").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncDDSurroundExMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncDDSurroundExMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncDDSurroundExMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH263PictureType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH263PictureType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH263PictureType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH263VLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH263VLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH263VLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH263VProfile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH263VProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH263VProfile").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH264PictureType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH264PictureType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH264PictureType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH264VLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH264VLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH264VLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH264VProfile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH264VProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH264VProfile").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH265VLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH265VLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH265VLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncH265VProfile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncH265VProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncH265VProfile").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncInputVideoSystem {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncInputVideoSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncInputVideoSystem").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPACodingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPACodingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPACodingMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPAEmphasisType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPAEmphasisType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPAEmphasisType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPALayer {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPALayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPALayer").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVFrameFieldMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVFrameFieldMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVFrameFieldMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVIntraVLCTable {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVIntraVLCTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVIntraVLCTable").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVProfile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVProfile").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVQScaleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVQScaleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVQScaleType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVScanPattern {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVScanPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVScanPattern").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMPVSceneDetection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMPVSceneDetection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMPVSceneDetection").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncMuxOutput {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncMuxOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncMuxOutput").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVP9VProfile {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVP9VProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVP9VProfile").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoChromaResolution {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoChromaResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoChromaResolution").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoChromaSubsampling {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoChromaSubsampling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoChromaSubsampling").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoColorLighting {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoColorLighting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoColorLighting").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoColorNominalRange {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoColorNominalRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoColorNominalRange").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoColorPrimaries {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoColorPrimaries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoColorPrimaries").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoColorTransferFunction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoColorTransferFunction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoColorTransferFunction").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoColorTransferMatrix {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoColorTransferMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoColorTransferMatrix").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoContentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoContentType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoFilmContent {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoFilmContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoFilmContent").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoOutputFrameRateConversion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoOutputFrameRateConversion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoOutputFrameRateConversion").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoOutputScanType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoOutputScanType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoOutputScanType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVEncVideoSourceScanType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVEncVideoSourceScanType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVEncVideoSourceScanType").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVFastDecodeMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVFastDecodeMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVFastDecodeMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for eAVScenarioInfo {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eAVScenarioInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eAVScenarioInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for eVideoEncoderDisplayContentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for eVideoEncoderDisplayContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("eVideoEncoderDisplayContentType").field(&self.0).finish()
    }
}
