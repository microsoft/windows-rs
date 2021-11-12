#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_Audio_Apo")]
pub mod Apo;
#[cfg(feature = "Win32_Media_Audio_DirectMusic")]
pub mod DirectMusic;
#[cfg(feature = "Win32_Media_Audio_DirectSound")]
pub mod DirectSound;
#[cfg(feature = "Win32_Media_Audio_Endpoints")]
pub mod Endpoints;
#[cfg(feature = "Win32_Media_Audio_XAudio2")]
pub mod XAudio2;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_DRIVER_ABOUT: u32 = 24587u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_DRIVER_DETAILS: u32 = 24586u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_DRIVER_NOTIFY: u32 = 24577u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FILTERTAG_DETAILS: u32 = 24626u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FILTER_DETAILS: u32 = 24627u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FORMATTAG_DETAILS: u32 = 24601u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FORMAT_DETAILS: u32 = 24602u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_FORMAT_SUGGEST: u32 = 24603u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_INPUT: u32 = 24596u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_HARDWARE_WAVE_CAPS_OUTPUT: u32 = 24597u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_RESERVED_HIGH: u32 = 28671u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_RESERVED_LOW: u32 = 24576u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_CLOSE: u32 = 24653u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_CONVERT: u32 = 24655u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_OPEN: u32 = 24652u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_PREPARE: u32 = 24657u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_RESET: u32 = 24656u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_SIZE: u32 = 24654u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_UNPREPARE: u32 = 24658u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_STREAM_UPDATE: u32 = 24659u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDM_USER: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_COPYRIGHT_CHARS: u32 = 80u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_FEATURES_CHARS: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_LICENSING_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_LONGNAME_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SHORTNAME_CHARS: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_ASYNC: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CODEC: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_CONVERTER: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_FILTER: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_HARDWARE: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMDRIVERDETAILS_SUPPORTF_LOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_BASE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_BUSY: u32 = 513u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_CANCELED: u32 = 515u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_NOTPOSSIBLE: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMERR_UNPREPARED: u32 = 514u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_INITTOFILTERSTRUCT: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERDETAILS_FILTER_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFILTERTAGDETAILS_FILTERTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_CONTEXTHELP: i32 = 128i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLEHOOK: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATE: i32 = 16i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_ENABLETEMPLATEHANDLE: i32 = 32i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_INITTOWFXSTRUCT: i32 = 64i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATCHOOSE_STYLEF_SHOWHELP: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATDETAILS_FORMAT_CHARS: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMFORMATTAGDETAILS_FORMATTAG_CHARS: u32 = 48u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMSTREAMHEADER_STATUSF_DONE: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMSTREAMHEADER_STATUSF_INQUEUE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACMSTREAMHEADER_STATUSF_PREPARED: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_FUNCTION: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_GLOBAL: i32 = 8i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_LOCAL: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_NAME: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_NOTIFYHWND: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERADDF_TYPEMASK: i32 = 7i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERENUMF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERENUMF_NOLOCAL: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_ABLEMASK: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_BEGIN: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_DEFERMASK: i32 = 196608i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_DISABLE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_ENABLE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_DRIVERPRIORITYF_END: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERDETAILSF_FILTER: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERENUMF_DWFILTERTAG: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_FILTERTAG: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FILTERTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATDETAILSF_FORMAT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_CONVERT: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_HARDWARE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_INPUT: i32 = 8388608i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_OUTPUT: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_SUGGEST: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATENUMF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_NCHANNELS: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_NSAMPLESPERSEC: i32 = 262144i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_TYPEMASK: i32 = 16711680i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_WBITSPERSAMPLE: i32 = 524288i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATSUGGESTF_WFORMATTAG: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_FORMATTAG: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_INDEX: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_LARGESTSIZE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_FORMATTAGDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_CODECS: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_CONVERTERS: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_DISABLED: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_DRIVERS: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_FILTERS: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_HARDWARE: u32 = 6u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_CODECS: u32 = 21u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_CONVERTERS: u32 = 22u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_DISABLED: u32 = 24u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_DRIVERS: u32 = 20u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_COUNT_LOCAL_FILTERS: u32 = 23u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_DRIVER_PRIORITY: u32 = 101u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_DRIVER_SUPPORT: u32 = 100u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_HARDWARE_WAVE_INPUT: u32 = 30u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_HARDWARE_WAVE_OUTPUT: u32 = 31u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_MAX_SIZE_FILTER: u32 = 51u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_METRIC_MAX_SIZE_FORMAT: u32 = 50u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMCONVERTF_BLOCKALIGN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMCONVERTF_END: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMCONVERTF_START: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMOPENF_ASYNC: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMOPENF_NONREALTIME: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMOPENF_QUERY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMSIZEF_DESTINATION: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMSIZEF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ACM_STREAMSIZEF_SOURCE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287486i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287469i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287464i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287477i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287466i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287463i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_BUFFER_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287482i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287465i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_DEVICE_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287484i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_DEVICE_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287478i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287423i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287422i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287473i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287454i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287447i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287448i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287471i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287468i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287474i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287470i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_HEADTRACKING_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287440i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287424i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287467i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287456i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INVALID_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287479i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_INVALID_STREAM_FLAG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287455i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287451i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287487i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_NOT_STOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287483i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287452i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287453i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_OUT_OF_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287481i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287449i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_RESOURCES_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287450i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287472i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287476i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287480i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287485i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_SESSIONFLAGS_DISPLAY_HIDEWHENEXPIRED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_SESSIONFLAGS_EXPIREWHENUNOWNED: u32 = 268435456u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_CROSSPROCESS: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_EVENTCALLBACK: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_NOPERSIST: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_RATEADJUST: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_STREAMFLAGS_SRC_DEFAULT_QUALITY: u32 = 134217728u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_S_BUFFER_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(143196161i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_S_POSITION_STALLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(143196163i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(143196162i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_AUXIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_CDAUDIO: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const AUXCAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_DISABLED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_NOTPRESENT: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DEVICE_STATE_UNPLUGGED: u32 = 8u32;
pub const DEVINTERFACE_AUDIO_CAPTURE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 787448254, data2: 13306, data3: 18432, data4: [150, 112, 28, 212, 116, 151, 44, 63] };
pub const DEVINTERFACE_AUDIO_RENDER: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3862068397,
    data2: 56556,
    data3: 18761,
    data4: [174, 138, 153, 30, 151, 106, 121, 210],
};
pub const DEVINTERFACE_MIDI_INPUT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1347150636, data2: 52470, data3: 19756, data4: [183, 63, 111, 139, 55, 71, 226, 43] };
pub const DEVINTERFACE_MIDI_OUTPUT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1841443616,
    data2: 43827,
    data3: 19684,
    data4: [128, 212, 187, 179, 235, 191, 40, 20],
};
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRVM_MAPPER: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRVM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRV_MAPPER_PREFERRED_INPUT_GET: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const DRV_MAPPER_PREFERRED_OUTPUT_GET: u32 = 16386u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_FORMAT_RESET_MIX_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0u32;
pub const EVENTCONTEXT_VOLUMESLIDER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3804424670, data2: 2481, data3: 19204, data4: [132, 229, 7, 147, 18, 37, 238, 4] };
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_FILTERTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_FILTER_VERIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FILTERCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_CUSTOM_VERIFY: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_FORMATTAG_VERIFY: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_FORMAT_VERIFY: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const FORMATCHOOSE_MESSAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MEVT_F_CALLBACK: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MEVT_F_LONG: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MEVT_F_SHORT: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_INQUEUE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_ISSTRM: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_CACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_LRVOLUME: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_STREAM: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDICAPS_VOLUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_BADOPENMODE: u32 = 70u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_DONT_CONTINUE: u32 = 71u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_INVALIDSETUP: u32 = 69u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_LASTERROR: u32 = 71u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_NODEVICE: u32 = 68u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_NOMAP: u32 = 66u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_NOTREADY: u32 = 67u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_STILLPLAYING: u32 = 65u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIERR_UNPREPARED: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPATCHSIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_GET: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_SET: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_TEMPO: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDIPROP_TIMEDIV: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDISTRM_ERROR: i32 = -2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_CACHE_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_CACHE_BESTFIT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_CACHE_QUERY: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIDI_UNCACHE: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLF_DISABLED: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLF_MULTIPLE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLF_UNIFORM: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS: u32 = 1342373890u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BASS_BOOST: u32 = 536945271u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEAN: u32 = 536936448u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BOOLEANMETER: u32 = 268500992u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_BUTTON: u32 = 553713664u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_CUSTOM: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_DECIBELS: u32 = 805568512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_EQUALIZER: u32 = 1342373892u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_FADER: u32 = 1342373888u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_LOUDNESS: u32 = 536936452u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MICROTIME: u32 = 1610809344u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MILLITIME: u32 = 1627586560u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MIXER: u32 = 1895890945u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MONO: u32 = 536936451u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MULTIPLESELECT: u32 = 1895890944u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUTE: u32 = 536936450u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_MUX: u32 = 1879113729u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_ONOFF: u32 = 536936449u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_PAN: u32 = 1073872897u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_PEAKMETER: u32 = 268566529u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_PERCENT: u32 = 805634048u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_QSOUNDPAN: u32 = 1073872898u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNED: u32 = 805437440u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SIGNEDMETER: u32 = 268566528u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SINGLESELECT: u32 = 1879113728u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_SLIDER: u32 = 1073872896u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_STEREOENH: u32 = 536936453u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_TREBLE: u32 = 1342373891u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNED: u32 = 805502976u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_UNSIGNEDMETER: u32 = 268632064u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CONTROLTYPE_VOLUME: u32 = 1342373889u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_FADER: i32 = 1342177280i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_LIST: i32 = 1879048192i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_MASK: i32 = -268435456i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_METER: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_NUMBER: i32 = 805306368i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_SLIDER: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_SWITCH: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_CLASS_TIME: i32 = 1610612736i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_LIST_MULTIPLE: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_LIST_SINGLE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_METER_POLLED: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BOOLEAN: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_SWITCH_BUTTON: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_TIME_MICROSECS: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SC_TIME_MILLISECS: i32 = 16777216i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_SUBCLASS_MASK: i32 = 251658240i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_BOOLEAN: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_CUSTOM: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_DECIBELS: i32 = 262144i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_MASK: i32 = 16711680i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_PERCENT: i32 = 327680i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_SIGNED: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERCONTROL_CT_UNITS_UNSIGNED: i32 = 196608i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_FIRST: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_DST_LAST: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_FIRST: i32 = 4096i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_COMPONENTTYPE_SRC_LAST: u32 = 4106u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_LINEF_ACTIVE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_LINEF_DISCONNECTED: i32 = 32768i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_LINEF_SOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_AUX: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_UNDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_WAVEIN: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERLINE_TARGETTYPE_WAVEOUT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_INVALCONTROL: u32 = 1025u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_INVALLINE: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_INVALVALUE: u32 = 1026u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXERR_LASTERROR: u32 = 1026u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETCONTROLDETAILSF_LISTTEXT: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_ALL: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYID: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_ONEBYTYPE: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINECONTROLSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_COMPONENTTYPE: i32 = 3i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_DESTINATION: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_LINEID: i32 = 2i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_SOURCE: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_GETLINEINFOF_TARGETTYPE: i32 = 4i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_LONG_NAME_CHARS: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_AUX: i32 = 1342177280i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_HANDLE: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_MIDIIN: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_MIDIOUT: i32 = 805306368i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_MIXER: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_WAVEIN: i32 = 536870912i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_OBJECTF_WAVEOUT: i32 = 268435456i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SETCONTROLDETAILSF_CUSTOM: i32 = 1i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SETCONTROLDETAILSF_QUERYMASK: i32 = 15i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SETCONTROLDETAILSF_VALUE: i32 = 0i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MIXER_SHORT_NAME_CHARS: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MM_ACM_FILTERCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MM_ACM_FORMATCHOOSE: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_FMSYNTH: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_MAPPER: u32 = 5u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_MIDIPORT: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_SQSYNTH: u32 = 3u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_SWSYNTH: u32 = 7u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_SYNTH: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const MOD_WAVETABLE: u32 = 6u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointLogo_IconEffects: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4054546445,
        data2: 8208,
        data3: 20179,
        data4: [163, 166, 139, 135, 240, 240, 196, 118],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointLogo_IconPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4054546445,
        data2: 8208,
        data3: 20179,
        data4: [163, 166, 139, 135, 240, 240, 196, 118],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 337911810, data2: 800, data3: 19940, data4: [149, 85, 167, 216, 43, 115, 194, 134] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpointSettings_MenuText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 337911810, data2: 800, data3: 19940, data4: [149, 85, 167, 216, 43, 115, 194, 134] },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Association: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_FormFactor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_JackSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 497408003,
        data2: 54418,
        data3: 20189,
        data4: [140, 35, 224, 192, 255, 238, 127, 14],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEngine_DeviceFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4053730893,
        data2: 2092,
        data3: 20007,
        data4: [188, 115, 104, 130, 161, 187, 142, 76],
    },
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AudioEngine_OEMFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 3834056230, data2: 15557, data3: 19666, data4: [186, 70, 202, 10, 154, 112, 237, 4] },
    pid: 3u32,
};
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ALIAS: i32 = 65536i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ALIAS_ID: i32 = 1114112i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ALIAS_START: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_APPLICATION: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_ASYNC: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_FILENAME: i32 = 131072i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_LOOP: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_MEMORY: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_NODEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_NOSTOP: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_NOWAIT: i32 = 8192i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_PURGE: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_RESOURCE: i32 = 262148i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_RING: i32 = 1048576i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_SENTRY: i32 = 524288i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_SYNC: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SND_SYSTEM: i32 = 2097152i32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPATIAL_AUDIO_POSITION: u32 = 200u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPATIAL_AUDIO_STANDARD_COMMANDS_START: u32 = 200u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_DESTROYED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287232i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287227i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_INTERNAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287219i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_INVALID_LICENSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287224i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287226i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287229i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287220i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287231i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287228i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287230i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287221i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287225i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004287222i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ATTACH_FAILED_INTERNAL_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286956i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_ALREADY_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286969i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_NOT_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286968i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_BUFFER_STILL_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286940i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_ALREADY_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286942i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_COMMAND_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286976i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_DETACH_FAILED_INTERNAL_BUFFER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286955i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_FORMAT_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286941i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMECOUNT_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286967i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_FRAMEOFFSET_OUT_OF_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286952i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_INVALID_ARGS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286974i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_ALREADY_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286957i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEMS_LOCKED_FOR_WRITING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286939i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_COPY_OVERFLOW: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286959i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_ITEM_MUST_HAVE_COMMANDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286951i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_MEMORY_BOUNDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286971i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_METADATA_FORMAT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286973i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_BUFFER_ATTACHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286954i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMOFFSET_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286944i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286960i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_OPEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286958i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_ITEMS_WRITTEN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286943i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_COMMANDS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286970i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_NO_MORE_ITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286953i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_OBJECT_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286975i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const SPTLAUD_MD_CLNT_E_VALUE_BUFFER_INCORRECT_SIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2004286972i32 as _);
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_LRVOLUME: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_PITCH: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_PLAYBACKRATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_SAMPLEACCURATE: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_SYNC: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVECAPS_VOLUME: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEIN_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEIN_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEIN_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEOUT_MAPPER_STATUS_DEVICE: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEOUT_MAPPER_STATUS_FORMAT: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVEOUT_MAPPER_STATUS_MAPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_BADFORMAT: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_LASTERROR: u32 = 35u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_STILLPLAYING: u32 = 33u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_SYNC: u32 = 35u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVERR_UNPREPARED: u32 = 34u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1M08: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1M16: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1S08: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_1S16: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2M08: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2M16: u32 = 64u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2S08: u32 = 32u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_2S16: u32 = 128u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44M08: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44M16: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44S08: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_44S16: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48M08: u32 = 4096u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48M16: u32 = 16384u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48S08: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_48S16: u32 = 32768u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4M08: u32 = 256u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4M16: u32 = 1024u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4S08: u32 = 512u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_4S16: u32 = 2048u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96M08: u32 = 65536u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96M16: u32 = 262144u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96S08: u32 = 131072u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_96S16: u32 = 524288u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_FORMAT_PCM: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_INVALIDFORMAT: u32 = 0u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WAVE_MAPPER: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_BEGINLOOP: u32 = 4u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_DONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_ENDLOOP: u32 = 8u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_INQUEUE: u32 = 16u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WHDR_PREPARED: u32 = 2u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WIDM_MAPPER_STATUS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Media_Audio`*"]
pub const WODM_MAPPER_STATUS: u32 = 8192u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ActivateAudioInterfaceAsync(deviceinterfacepath: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, completionhandler: IActivateAudioInterfaceCompletionHandler, activationoperation: *mut IActivateAudioInterfaceAsyncOperation) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CoRegisterMessageFilter(lpmessagefilter: IMessageFilter, lplpmessagefilter: *mut IMessageFilter) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitor(audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateCaptureAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitor(audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitorForCategory(category: AUDIO_STREAM_CATEGORY, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceId(category: AUDIO_STREAM_CATEGORY, deviceid: super::super::Foundation::PWSTR, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn CreateRenderAudioStateMonitorForCategoryAndDeviceRole(category: AUDIO_STREAM_CATEGORY, role: ERole, audiostatemonitor: *mut IAudioStateMonitor) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundA(pszsound: super::super::Foundation::PSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PlaySoundW(pszsound: super::super::Foundation::PWSTR, hmod: super::super::Foundation::HINSTANCE, fdwsound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddA(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverAddW(phadid: *mut isize, hinstmodule: super::super::Foundation::HINSTANCE, lparam: super::super::Foundation::LPARAM, dwpriority: u32, fdwadd: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverClose(had: HACMDRIVER, fdwclose: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn acmDriverDetailsA(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn acmDriverDetailsW(hadid: HACMDRIVERID, padd: *mut ACMDRIVERDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverEnum(fncallback: ACMDRIVERENUMCB, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverID(hao: HACMOBJ, phadid: *mut isize, fdwdriverid: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmDriverMessage(had: HACMDRIVER, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverOpen(phad: *mut isize, hadid: HACMDRIVERID, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverPriority(hadid: HACMDRIVERID, dwpriority: u32, fdwpriority: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmDriverRemove(hadid: HACMDRIVERID, fdwremove: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseA(pafltrc: *mut ACMFILTERCHOOSEA) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterChooseW(pafltrc: *mut ACMFILTERCHOOSEW) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterDetailsA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFilterDetailsW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumA(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSA, fncallback: ACMFILTERENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterEnumW(had: HACMDRIVER, pafd: *mut ACMFILTERDETAILSW, fncallback: ACMFILTERENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFilterTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumA(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSA, fncallback: ACMFILTERTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFilterTagEnumW(had: HACMDRIVER, paftd: *mut ACMFILTERTAGDETAILSW, fncallback: ACMFILTERTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseA(pafmtc: *mut ACMFORMATCHOOSEA) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatChooseW(pafmtc: *mut ACMFORMATCHOOSEW) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatDetailsA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatDetailsW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumA(had: HACMDRIVER, pafd: *mut ACMFORMATDETAILSA, fncallback: ACMFORMATENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatEnumW(had: HACMDRIVER, pafd: *mut tACMFORMATDETAILSW, fncallback: ACMFORMATENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatSuggest(had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, cbwfxdst: u32, fdwsuggest: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagDetailsA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmFormatTagDetailsW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumA(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSA, fncallback: ACMFORMATTAGENUMCBA, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmFormatTagEnumW(had: HACMDRIVER, paftd: *mut ACMFORMATTAGDETAILSW, fncallback: ACMFORMATTAGENUMCBW, dwinstance: usize, fdwenum: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmGetVersion() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmMetrics(hao: HACMOBJ, umetric: u32, pmetric: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamClose(has: HACMSTREAM, fdwclose: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamConvert(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwconvert: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn acmStreamMessage(has: HACMSTREAM, umsg: u32, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamOpen(phas: *mut isize, had: HACMDRIVER, pwfxsrc: *mut WAVEFORMATEX, pwfxdst: *mut WAVEFORMATEX, pwfltr: *mut WAVEFILTER, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamPrepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwprepare: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamReset(has: HACMSTREAM, fdwreset: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamSize(has: HACMSTREAM, cbinput: u32, pdwoutputbytes: *mut u32, fdwsize: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn acmStreamUnprepareHeader(has: HACMSTREAM, pash: *mut ACMSTREAMHEADER, fdwunprepare: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn auxGetDevCapsA(udeviceid: usize, pac: *mut AUXCAPSA, cbac: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetDevCapsW(udeviceid: usize, pac: *mut AUXCAPSW, cbac: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxGetVolume(udeviceid: u32, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxOutMessage(udeviceid: u32, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn auxSetVolume(udeviceid: u32, dwvolume: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiConnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiDisconnect(hmi: HMIDI, hmo: HMIDIOUT, preserved: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInAddBuffer(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInClose(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetDevCapsA(udeviceid: usize, pmic: *mut MIDIINCAPSA, cbmic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetDevCapsW(udeviceid: usize, pmic: *mut MIDIINCAPSW, cbmic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetID(hmi: HMIDIIN, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInMessage(hmi: HMIDIIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInOpen(phmi: *mut HMIDIIN, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInPrepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInReset(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInStart(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiInStop(hmi: HMIDIIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiInUnprepareHeader(hmi: HMIDIIN, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutCacheDrumPatches(hmo: HMIDIOUT, upatch: u32, pwkya: *const u16, fucache: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutCachePatches(hmo: HMIDIOUT, ubank: u32, pwpa: *const u16, fucache: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutClose(hmo: HMIDIOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetDevCapsA(udeviceid: usize, pmoc: *mut MIDIOUTCAPSA, cbmoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetDevCapsW(udeviceid: usize, pmoc: *mut MIDIOUTCAPSW, cbmoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetID(hmo: HMIDIOUT, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutGetVolume(hmo: HMIDIOUT, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutLongMsg(hmo: HMIDIOUT, pmh: *const MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutMessage(hmo: HMIDIOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutOpen(phmo: *mut HMIDIOUT, udeviceid: u32, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutPrepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutReset(hmo: HMIDIOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutSetVolume(hmo: HMIDIOUT, dwvolume: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiOutShortMsg(hmo: HMIDIOUT, dwmsg: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiOutUnprepareHeader(hmo: HMIDIOUT, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamClose(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamOpen(phms: *mut HMIDISTRM, pudeviceid: *mut u32, cmidi: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn midiStreamOut(hms: HMIDISTRM, pmh: *mut MIDIHDR, cbmh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamPause(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamPosition(hms: HMIDISTRM, lpmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamProperty(hms: HMIDISTRM, lppropdata: *mut u8, dwproperty: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamRestart(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn midiStreamStop(hms: HMIDISTRM) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerClose(hmx: HMIXER) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsA(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetControlDetailsW(hmxobj: HMIXEROBJ, pmxcd: *mut MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetDevCapsA(umxid: usize, pmxcaps: *mut MIXERCAPSA, cbmxcaps: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetDevCapsW(umxid: usize, pmxcaps: *mut MIXERCAPSW, cbmxcaps: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetID(hmxobj: HMIXEROBJ, pumxid: *mut u32, fdwid: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineControlsA(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSA, fdwcontrols: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetLineControlsW(hmxobj: HMIXEROBJ, pmxlc: *mut MIXERLINECONTROLSW, fdwcontrols: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerGetLineInfoA(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEA, fdwinfo: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetLineInfoW(hmxobj: HMIXEROBJ, pmxl: *mut MIXERLINEW, fdwinfo: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerMessage(hmx: HMIXER, umsg: u32, dwparam1: usize, dwparam2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn mixerOpen(phmx: *mut isize, umxid: u32, dwcallback: usize, dwinstance: usize, fdwopen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn mixerSetControlDetails(hmxobj: HMIXEROBJ, pmxcd: *const MIXERCONTROLDETAILS, fdwdetails: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundA(pszsound: super::super::Foundation::PSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn sndPlaySoundW(pszsound: super::super::Foundation::PWSTR, fusound: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInAddBuffer(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInClose(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetDevCapsA(udeviceid: usize, pwic: *mut WAVEINCAPSA, cbwic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetDevCapsW(udeviceid: usize, pwic: *mut WAVEINCAPSW, cbwic: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetID(hwi: HWAVEIN, pudeviceid: *const u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInGetPosition(hwi: HWAVEIN, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInMessage(hwi: HWAVEIN, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInOpen(phwi: *mut HWAVEIN, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInPrepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInReset(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInStart(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveInStop(hwi: HWAVEIN) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveInUnprepareHeader(hwi: HWAVEIN, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutBreakLoop(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutClose(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetDevCapsA(udeviceid: usize, pwoc: *mut WAVEOUTCAPSA, cbwoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetDevCapsW(udeviceid: usize, pwoc: *mut WAVEOUTCAPSW, cbwoc: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextA(mmrerror: u32, psztext: super::super::Foundation::PSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutGetErrorTextW(mmrerror: u32, psztext: super::super::Foundation::PWSTR, cchtext: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetID(hwo: HWAVEOUT, pudeviceid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetNumDevs() -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPitch(hwo: HWAVEOUT, pdwpitch: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPlaybackRate(hwo: HWAVEOUT, pdwrate: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetPosition(hwo: HWAVEOUT, pmmt: *mut super::MMTIME, cbmmt: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutGetVolume(hwo: HWAVEOUT, pdwvolume: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutMessage(hwo: HWAVEOUT, umsg: u32, dw1: usize, dw2: usize) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutOpen(phwo: *mut HWAVEOUT, udeviceid: u32, pwfx: *const WAVEFORMATEX, dwcallback: usize, dwinstance: usize, fdwopen: MIDI_WAVE_OPEN_TYPE) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutPause(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutPrepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutReset(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutRestart(hwo: HWAVEOUT) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetPitch(hwo: HWAVEOUT, dwpitch: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetPlaybackRate(hwo: HWAVEOUT, dwrate: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`*"]
    pub fn waveOutSetVolume(hwo: HWAVEOUT, dwvolume: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutUnprepareHeader(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_Audio`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn waveOutWrite(hwo: HWAVEOUT, pwh: *mut WAVEHDR, cbwh: u32) -> u32;
}
