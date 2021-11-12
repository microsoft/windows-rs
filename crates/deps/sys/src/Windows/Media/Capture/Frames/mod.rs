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
#[repr(transparent)]
pub struct MediaFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyResult(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MediaFrameSourceInfo(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MultiSourceMediaFrameReader(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct VideoMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoMediaFrameFormat(pub *mut ::core::ffi::c_void);
