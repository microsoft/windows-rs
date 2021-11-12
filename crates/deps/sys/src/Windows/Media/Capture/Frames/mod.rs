#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BufferMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DepthMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DepthMediaFrameFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBufferMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDepthMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDepthMediaFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDepthMediaFrameFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInfraredMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameFormat2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameReference2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceController2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceController3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceGetPropertyResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceGroupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaFrameSourceInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiSourceMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVideoMediaFrameFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InfraredMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: MediaFrameReaderAcquisitionMode = MediaFrameReaderAcquisitionMode(0i32);
    pub const Buffered: MediaFrameReaderAcquisitionMode = MediaFrameReaderAcquisitionMode(1i32);
}
#[repr(transparent)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(0i32);
    pub const UnknownFailure: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(1i32);
    pub const DeviceNotAvailable: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(2i32);
    pub const OutputFormatNotSupported: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(3i32);
    pub const ExclusiveControlNotAvailable: MediaFrameReaderStartStatus = MediaFrameReaderStartStatus(4i32);
}
#[repr(transparent)]
pub struct MediaFrameReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(0i32);
    pub const UnknownFailure: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(1i32);
    pub const NotSupported: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(2i32);
    pub const DeviceNotAvailable: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(3i32);
    pub const MaxPropertyValueSizeTooSmall: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(4i32);
    pub const MaxPropertyValueSizeRequired: MediaFrameSourceGetPropertyStatus = MediaFrameSourceGetPropertyStatus(5i32);
}
#[repr(transparent)]
pub struct MediaFrameSourceGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: MediaFrameSourceKind = MediaFrameSourceKind(0i32);
    pub const Color: MediaFrameSourceKind = MediaFrameSourceKind(1i32);
    pub const Infrared: MediaFrameSourceKind = MediaFrameSourceKind(2i32);
    pub const Depth: MediaFrameSourceKind = MediaFrameSourceKind(3i32);
    pub const Audio: MediaFrameSourceKind = MediaFrameSourceKind(4i32);
    pub const Image: MediaFrameSourceKind = MediaFrameSourceKind(5i32);
    pub const Metadata: MediaFrameSourceKind = MediaFrameSourceKind(6i32);
}
#[repr(transparent)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(0i32);
    pub const UnknownFailure: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(1i32);
    pub const NotSupported: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(2i32);
    pub const InvalidValue: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(3i32);
    pub const DeviceNotAvailable: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(4i32);
    pub const NotInControl: MediaFrameSourceSetPropertyStatus = MediaFrameSourceSetPropertyStatus(5i32);
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MultiSourceMediaFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(0i32);
    pub const NotSupported: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(1i32);
    pub const InsufficientResources: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(2i32);
    pub const DeviceNotAvailable: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(3i32);
    pub const UnknownFailure: MultiSourceMediaFrameReaderStartStatus = MultiSourceMediaFrameReaderStartStatus(4i32);
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoMediaFrameFormat(pub *mut ::core::ffi::c_void);
