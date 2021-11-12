#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct MediaFrameReaderAcquisitionMode(i32);
#[repr(C)]
pub struct MediaFrameReaderStartStatus(i32);
#[repr(transparent)]
pub struct MediaFrameReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceGetPropertyResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaFrameSourceGetPropertyStatus(i32);
#[repr(transparent)]
pub struct MediaFrameSourceGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaFrameSourceInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaFrameSourceKind(i32);
#[repr(C)]
pub struct MediaFrameSourceSetPropertyStatus(i32);
#[repr(transparent)]
pub struct MultiSourceMediaFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MultiSourceMediaFrameReader(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MultiSourceMediaFrameReaderStartStatus(i32);
#[repr(transparent)]
pub struct MultiSourceMediaFrameReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoMediaFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VideoMediaFrameFormat(pub *mut ::core::ffi::c_void);
