#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromMediaSink(pmediasink : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromURL(pwszoutputurl : windows_sys::core::PCWSTR, pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsinkwriter : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromByteStream(pbytestream : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromMediaSource(pmediasource : *mut core::ffi::c_void, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfreadwrite.dll" "system" fn MFCreateSourceReaderFromURL(pwszurl : windows_sys::core::PCWSTR, pattributes : *mut core::ffi::c_void, ppsourcereader : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const MF_SINK_WRITER_ALL_STREAMS: __MIDL___MIDL_itf_mfreadwrite_0000_0005_0001 = -2;
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: __MIDL___MIDL_itf_mfreadwrite_0000_0005_0001 = -1;
pub const MF_SINK_WRITER_MEDIASINK: __MIDL___MIDL_itf_mfreadwrite_0000_0005_0001 = -1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MF_SINK_WRITER_STATISTICS {
    pub cb: u32,
    pub llLastTimestampReceived: i64,
    pub llLastTimestampEncoded: i64,
    pub llLastTimestampProcessed: i64,
    pub llLastStreamTickReceived: i64,
    pub llLastSinkSampleRequest: i64,
    pub qwNumSamplesReceived: u64,
    pub qwNumSamplesEncoded: u64,
    pub qwNumSamplesProcessed: u64,
    pub qwNumStreamTicksReceived: u64,
    pub dwByteCountQueued: u32,
    pub qwByteCountProcessed: u64,
    pub dwNumOutstandingSinkSampleRequests: u32,
    pub dwAverageSampleRateReceived: u32,
    pub dwAverageSampleRateEncoded: u32,
    pub dwAverageSampleRateProcessed: u32,
}
pub const MF_SOURCE_READERF_ALLEFFECTSREMOVED: MF_SOURCE_READER_FLAG = 512;
pub const MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 32;
pub const MF_SOURCE_READERF_ENDOFSTREAM: MF_SOURCE_READER_FLAG = 2;
pub const MF_SOURCE_READERF_ERROR: MF_SOURCE_READER_FLAG = 1;
pub const MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED: MF_SOURCE_READER_FLAG = 16;
pub const MF_SOURCE_READERF_NEWSTREAM: MF_SOURCE_READER_FLAG = 4;
pub const MF_SOURCE_READERF_STREAMTICK: MF_SOURCE_READER_FLAG = 256;
pub const MF_SOURCE_READER_ALL_STREAMS: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = -2;
pub const MF_SOURCE_READER_ANY_STREAM: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = -2;
pub const MF_SOURCE_READER_CONTROLF_DRAIN: MF_SOURCE_READER_CONTROL_FLAG = 1;
pub type MF_SOURCE_READER_CONTROL_FLAG = u32;
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0002 = -1;
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = -3;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = -4;
pub type MF_SOURCE_READER_FLAG = u32;
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE: __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = -1;
pub type __MIDL___MIDL_itf_mfreadwrite_0000_0001_0001 = i32;
pub type __MIDL___MIDL_itf_mfreadwrite_0000_0001_0002 = i32;
pub type __MIDL___MIDL_itf_mfreadwrite_0000_0005_0001 = i32;
