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
pub const MF_SINK_WRITER_ALL_STREAMS: i32 = -2;
pub const MF_SINK_WRITER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SINK_WRITER_MEDIASINK: i32 = -1;
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
pub const MF_SOURCE_READER_ALL_STREAMS: i32 = -2;
pub const MF_SOURCE_READER_ANY_STREAM: i32 = -2;
pub const MF_SOURCE_READER_CONTROLF_DRAIN: MF_SOURCE_READER_CONTROL_FLAG = 1;
pub type MF_SOURCE_READER_CONTROL_FLAG = u32;
pub const MF_SOURCE_READER_CURRENT_TYPE_INDEX: i32 = -1;
pub const MF_SOURCE_READER_FIRST_AUDIO_STREAM: i32 = -3;
pub const MF_SOURCE_READER_FIRST_VIDEO_STREAM: i32 = -4;
pub type MF_SOURCE_READER_FLAG = u32;
pub const MF_SOURCE_READER_INVALID_STREAM_INDEX: i32 = -1;
pub const MF_SOURCE_READER_MEDIASOURCE: i32 = -1;
