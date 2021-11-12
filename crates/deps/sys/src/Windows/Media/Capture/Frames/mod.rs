#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AudioMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AudioMediaFrame {}
impl ::core::clone::Clone for AudioMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BufferMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BufferMediaFrame {}
impl ::core::clone::Clone for BufferMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DepthMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DepthMediaFrame {}
impl ::core::clone::Clone for DepthMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DepthMediaFrameFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DepthMediaFrameFormat {}
impl ::core::clone::Clone for DepthMediaFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAudioMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAudioMediaFrame {}
impl ::core::clone::Clone for IAudioMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBufferMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBufferMediaFrame {}
impl ::core::clone::Clone for IBufferMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDepthMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDepthMediaFrame {}
impl ::core::clone::Clone for IDepthMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDepthMediaFrame2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDepthMediaFrame2 {}
impl ::core::clone::Clone for IDepthMediaFrame2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDepthMediaFrameFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDepthMediaFrameFormat {}
impl ::core::clone::Clone for IDepthMediaFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInfraredMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInfraredMediaFrame {}
impl ::core::clone::Clone for IInfraredMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for IMediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameFormat {}
impl ::core::clone::Clone for IMediaFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameFormat2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameFormat2 {}
impl ::core::clone::Clone for IMediaFrameFormat2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameReader {}
impl ::core::clone::Clone for IMediaFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameReader2 {}
impl ::core::clone::Clone for IMediaFrameReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameReference {}
impl ::core::clone::Clone for IMediaFrameReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameReference2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameReference2 {}
impl ::core::clone::Clone for IMediaFrameReference2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSource {}
impl ::core::clone::Clone for IMediaFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceController {}
impl ::core::clone::Clone for IMediaFrameSourceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceController2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceController2 {}
impl ::core::clone::Clone for IMediaFrameSourceController2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceController3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceController3 {}
impl ::core::clone::Clone for IMediaFrameSourceController3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceGetPropertyResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceGetPropertyResult {}
impl ::core::clone::Clone for IMediaFrameSourceGetPropertyResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceGroup {}
impl ::core::clone::Clone for IMediaFrameSourceGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceGroupStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceGroupStatics {}
impl ::core::clone::Clone for IMediaFrameSourceGroupStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceInfo {}
impl ::core::clone::Clone for IMediaFrameSourceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceInfo2 {}
impl ::core::clone::Clone for IMediaFrameSourceInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaFrameSourceInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaFrameSourceInfo3 {}
impl ::core::clone::Clone for IMediaFrameSourceInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultiSourceMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultiSourceMediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for IMultiSourceMediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultiSourceMediaFrameReader {}
impl ::core::clone::Clone for IMultiSourceMediaFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultiSourceMediaFrameReader2 {}
impl ::core::clone::Clone for IMultiSourceMediaFrameReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMultiSourceMediaFrameReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMultiSourceMediaFrameReference {}
impl ::core::clone::Clone for IMultiSourceMediaFrameReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoMediaFrame {}
impl ::core::clone::Clone for IVideoMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVideoMediaFrameFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVideoMediaFrameFormat {}
impl ::core::clone::Clone for IVideoMediaFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InfraredMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InfraredMediaFrame {}
impl ::core::clone::Clone for InfraredMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for MediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameFormat {}
impl ::core::clone::Clone for MediaFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameReader {}
impl ::core::clone::Clone for MediaFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameReaderAcquisitionMode(pub i32);
impl MediaFrameReaderAcquisitionMode {
    pub const Realtime: Self = Self(0i32);
    pub const Buffered: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaFrameReaderAcquisitionMode {}
impl ::core::clone::Clone for MediaFrameReaderAcquisitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameReaderStartStatus(pub i32);
impl MediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const DeviceNotAvailable: Self = Self(2i32);
    pub const OutputFormatNotSupported: Self = Self(3i32);
    pub const ExclusiveControlNotAvailable: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameReference {}
impl ::core::clone::Clone for MediaFrameReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameSource {}
impl ::core::clone::Clone for MediaFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameSourceController {}
impl ::core::clone::Clone for MediaFrameSourceController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameSourceGetPropertyResult {}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyStatus(pub i32);
impl MediaFrameSourceGetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const MaxPropertyValueSizeTooSmall: Self = Self(4i32);
    pub const MaxPropertyValueSizeRequired: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceGetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceGetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameSourceGroup {}
impl ::core::clone::Clone for MediaFrameSourceGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaFrameSourceInfo {}
impl ::core::clone::Clone for MediaFrameSourceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceKind(pub i32);
impl MediaFrameSourceKind {
    pub const Custom: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const Infrared: Self = Self(2i32);
    pub const Depth: Self = Self(3i32);
    pub const Audio: Self = Self(4i32);
    pub const Image: Self = Self(5i32);
    pub const Metadata: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaFrameSourceKind {}
impl ::core::clone::Clone for MediaFrameSourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaFrameSourceSetPropertyStatus(pub i32);
impl MediaFrameSourceSetPropertyStatus {
    pub const Success: Self = Self(0i32);
    pub const UnknownFailure: Self = Self(1i32);
    pub const NotSupported: Self = Self(2i32);
    pub const InvalidValue: Self = Self(3i32);
    pub const DeviceNotAvailable: Self = Self(4i32);
    pub const NotInControl: Self = Self(5i32);
}
impl ::core::marker::Copy for MediaFrameSourceSetPropertyStatus {}
impl ::core::clone::Clone for MediaFrameSourceSetPropertyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MultiSourceMediaFrameArrivedEventArgs {}
impl ::core::clone::Clone for MultiSourceMediaFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MultiSourceMediaFrameReader {}
impl ::core::clone::Clone for MultiSourceMediaFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameReaderStartStatus(pub i32);
impl MultiSourceMediaFrameReaderStartStatus {
    pub const Success: Self = Self(0i32);
    pub const NotSupported: Self = Self(1i32);
    pub const InsufficientResources: Self = Self(2i32);
    pub const DeviceNotAvailable: Self = Self(3i32);
    pub const UnknownFailure: Self = Self(4i32);
}
impl ::core::marker::Copy for MultiSourceMediaFrameReaderStartStatus {}
impl ::core::clone::Clone for MultiSourceMediaFrameReaderStartStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MultiSourceMediaFrameReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MultiSourceMediaFrameReference {}
impl ::core::clone::Clone for MultiSourceMediaFrameReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoMediaFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoMediaFrame {}
impl ::core::clone::Clone for VideoMediaFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VideoMediaFrameFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VideoMediaFrameFormat {}
impl ::core::clone::Clone for VideoMediaFrameFormat {
    fn clone(&self) -> Self {
        *self
    }
}
