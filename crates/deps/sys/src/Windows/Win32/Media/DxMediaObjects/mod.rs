#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn DMOEnum(guidcategory: *const ::windows_sys::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE, ppenum: *mut IEnumDMO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMOGetName(clsiddmo: *const ::windows_sys::core::GUID, szname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn DMOGetTypes(clsiddmo: *const ::windows_sys::core::GUID, ulinputtypesrequested: u32, pulinputtypessupplied: *mut u32, pinputtypes: *mut DMO_PARTIAL_MEDIATYPE, uloutputtypesrequested: u32, puloutputtypessupplied: *mut u32, poutputtypes: *mut DMO_PARTIAL_MEDIATYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DMORegister(szname: super::super::Foundation::PWSTR, clsiddmo: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID, dwflags: u32, cintypes: u32, pintypes: *const DMO_PARTIAL_MEDIATYPE, couttypes: u32, pouttypes: *const DMO_PARTIAL_MEDIATYPE) -> ::windows_sys::core::HRESULT;
    pub fn DMOUnregister(clsiddmo: *const ::windows_sys::core::GUID, guidcategory: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCopyMediaType(pmtdest: *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoCreateMediaType(ppmt: *mut *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDeleteMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoDuplicateMediaType(ppmtdest: *mut *mut DMO_MEDIA_TYPE, pmtsrc: *const DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoFreeMediaType(pmt: *mut DMO_MEDIA_TYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoInitMediaType(pmt: *mut DMO_MEDIA_TYPE, cbformat: u32) -> ::windows_sys::core::HRESULT;
}
pub const DMOCATEGORY_ACOUSTIC_ECHO_CANCEL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3214294400, data2: 50521, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const DMOCATEGORY_AGC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3901528992, data2: 50519, data3: 4560, data4: [138, 43, 0, 160, 201, 37, 90, 193] };
pub const DMOCATEGORY_AUDIO_CAPTURE_EFFECT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4133857978, data2: 15881, data3: 18720, data4: [170, 95, 33, 152, 17, 20, 143, 9] };
pub const DMOCATEGORY_AUDIO_DECODER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1475533707, data2: 59067, data3: 17683, data4: [157, 67, 220, 210, 166, 89, 49, 37] };
pub const DMOCATEGORY_AUDIO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4083166015,
    data2: 1426,
    data3: 18655,
    data4: [164, 205, 103, 71, 33, 231, 235, 235],
};
pub const DMOCATEGORY_AUDIO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 869902177, data2: 37064, data3: 4560, data4: [189, 67, 0, 160, 201, 17, 206, 134] };
pub const DMOCATEGORY_AUDIO_NOISE_SUPPRESS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3766456383,
    data2: 25341,
    data3: 20064,
    data4: [140, 221, 222, 167, 35, 102, 101, 181],
};
pub const DMOCATEGORY_VIDEO_DECODER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1248441410,
    data2: 10430,
    data3: 18833,
    data4: [150, 156, 181, 0, 173, 245, 216, 168],
};
pub const DMOCATEGORY_VIDEO_EFFECT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3650154004,
    data2: 30572,
    data3: 18211,
    data4: [190, 70, 61, 162, 245, 111, 16, 185],
};
pub const DMOCATEGORY_VIDEO_ENCODER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 869902176, data2: 37064, data3: 4560, data4: [189, 67, 0, 160, 201, 17, 206, 134] };
pub struct DMO_ENUM_FLAGS(i32);
pub const DMO_E_INVALIDSTREAMINDEX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const DMO_E_INVALIDTYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
pub const DMO_E_NOTACCEPTING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220988i32 as _);
pub const DMO_E_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220986i32 as _);
pub const DMO_E_TYPE_NOT_ACCEPTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220987i32 as _);
pub const DMO_E_TYPE_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220989i32 as _);
#[cfg(feature = "Win32_Foundation")]
pub struct DMO_MEDIA_TYPE(i32);
pub struct DMO_OUTPUT_DATA_BUFFER(i32);
pub struct DMO_PARTIAL_MEDIATYPE(i32);
pub struct DMO_REGISTER_FLAGS(i32);
#[repr(transparent)]
pub struct IDMOQualityControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDMOVideoOutputOptimizations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDMO(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaObjectInPlace(pub *mut ::core::ffi::c_void);
pub struct _DMO_INPLACE_PROCESS_FLAGS(i32);
pub struct _DMO_INPUT_DATA_BUFFER_FLAGS(i32);
pub struct _DMO_INPUT_STATUS_FLAGS(i32);
pub struct _DMO_INPUT_STREAM_INFO_FLAGS(i32);
pub struct _DMO_OUTPUT_DATA_BUFFER_FLAGS(i32);
pub struct _DMO_OUTPUT_STREAM_INFO_FLAGS(i32);
pub struct _DMO_PROCESS_OUTPUT_FLAGS(i32);
pub struct _DMO_QUALITY_STATUS_FLAGS(i32);
pub struct _DMO_SET_TYPE_FLAGS(i32);
pub struct _DMO_VIDEO_OUTPUT_STREAM_FLAGS(i32);
