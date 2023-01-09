impl ::core::default::Default for DMO_ENUM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DMO_ENUM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_ENUM_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DMO_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DMO_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DMO_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DMO_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
impl ::core::default::Default for DMO_OUTPUT_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMO_OUTPUT_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.dwStatus == other.dwStatus && self.rtTimestamp == other.rtTimestamp && self.rtTimelength == other.rtTimelength
    }
}
impl ::core::cmp::Eq for DMO_OUTPUT_DATA_BUFFER {}
impl ::core::fmt::Debug for DMO_OUTPUT_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_OUTPUT_DATA_BUFFER").field("pBuffer", &self.pBuffer).field("dwStatus", &self.dwStatus).field("rtTimestamp", &self.rtTimestamp).field("rtTimelength", &self.rtTimelength).finish()
    }
}
impl ::core::default::Default for DMO_PARTIAL_MEDIATYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DMO_PARTIAL_MEDIATYPE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.subtype == other.subtype
    }
}
impl ::core::cmp::Eq for DMO_PARTIAL_MEDIATYPE {}
impl ::core::fmt::Debug for DMO_PARTIAL_MEDIATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DMO_PARTIAL_MEDIATYPE").field("type", &self.r#type).field("subtype", &self.subtype).finish()
    }
}
impl ::core::default::Default for DMO_REGISTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DMO_REGISTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DMO_REGISTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDMOQualityControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMOQualityControl {}
impl ::core::fmt::Debug for IDMOQualityControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMOQualityControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDMOVideoOutputOptimizations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDMOVideoOutputOptimizations {}
impl ::core::fmt::Debug for IDMOVideoOutputOptimizations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDMOVideoOutputOptimizations").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDMO {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDMO {}
impl ::core::fmt::Debug for IEnumDMO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDMO").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMediaBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaBuffer {}
impl ::core::fmt::Debug for IMediaBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMediaObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaObject {}
impl ::core::fmt::Debug for IMediaObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMediaObjectInPlace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaObjectInPlace {}
impl ::core::fmt::Debug for IMediaObjectInPlace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaObjectInPlace").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_INPLACE_PROCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_INPLACE_PROCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPLACE_PROCESS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_INPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_INPUT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_INPUT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_INPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_INPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_OUTPUT_DATA_BUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_DATA_BUFFER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_OUTPUT_STREAM_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_OUTPUT_STREAM_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_PROCESS_OUTPUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_PROCESS_OUTPUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_PROCESS_OUTPUT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_QUALITY_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_QUALITY_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_QUALITY_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_SET_TYPE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_SET_TYPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_SET_TYPE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _DMO_VIDEO_OUTPUT_STREAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DMO_VIDEO_OUTPUT_STREAM_FLAGS").field(&self.0).finish()
    }
}
