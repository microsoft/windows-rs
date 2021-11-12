#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterAttach(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR, dwcreatedinstancenamelength: u32, lpcreatedinstancename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterAttachAtAltitude(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpaltitude: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR, dwcreatedinstancenamelength: u32, lpcreatedinstancename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn FilterClose(hfilter: HFILTER) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn FilterConnectCommunicationPort(lpportname: super::super::Foundation::PWSTR, dwoptions: u32, lpcontext: *const ::core::ffi::c_void, wsizeofcontext: u16, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, hport: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterCreate(lpfiltername: super::super::Foundation::PWSTR, hfilter: *mut HFILTER) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterDetach(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterFindClose(hfilterfind: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn FilterFindFirst(dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterfind: *mut FilterFindHandle) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterFindNext(hfilterfind: super::super::Foundation::HANDLE, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterGetDosName(lpvolumename: super::super::Foundation::PWSTR, lpdosname: super::super::Foundation::PWSTR, dwdosnamebuffersize: u32) -> ::windows_sys::core::HRESULT;
    pub fn FilterGetInformation(hfilter: HFILTER, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
    pub fn FilterGetMessage(hport: super::super::Foundation::HANDLE, lpmessagebuffer: *mut FILTER_MESSAGE_HEADER, dwmessagebuffersize: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows_sys::core::HRESULT;
    pub fn FilterInstanceClose(hinstance: HFILTER_INSTANCE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceCreate(lpfiltername: super::super::Foundation::PWSTR, lpvolumename: super::super::Foundation::PWSTR, lpinstancename: super::super::Foundation::PWSTR, hinstance: *mut HFILTER_INSTANCE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindClose(hfilterinstancefind: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindFirst(lpfiltername: super::super::Foundation::PWSTR, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterinstancefind: *mut FilterInstanceFindHandle) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterInstanceFindNext(hfilterinstancefind: super::super::Foundation::HANDLE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn FilterInstanceGetInformation(hinstance: HFILTER_INSTANCE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterLoad(lpfiltername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterReplyMessage(hport: super::super::Foundation::HANDLE, lpreplybuffer: *const FILTER_REPLY_HEADER, dwreplybuffersize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterSendMessage(hport: super::super::Foundation::HANDLE, lpinbuffer: *const ::core::ffi::c_void, dwinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, dwoutbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterUnload(lpfiltername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeFindClose(hvolumefind: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn FilterVolumeFindFirst(dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumefind: *mut FilterVolumeFindHandle) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeFindNext(hvolumefind: super::super::Foundation::HANDLE, dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindClose(hvolumeinstancefind: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindFirst(lpvolumename: super::super::Foundation::PWSTR, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumeinstancefind: *mut FilterVolumeInstanceFindHandle) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FilterVolumeInstanceFindNext(hvolumeinstancefind: super::super::Foundation::HANDLE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows_sys::core::HRESULT;
}
pub struct FILTER_AGGREGATE_BASIC_INFORMATION(i32);
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION(i32);
pub struct FILTER_FULL_INFORMATION(i32);
pub struct FILTER_INFORMATION_CLASS(i32);
pub struct FILTER_MESSAGE_HEADER(i32);
pub const FILTER_NAME_MAX_CHARS: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
pub struct FILTER_REPLY_HEADER(i32);
pub struct FILTER_VOLUME_BASIC_INFORMATION(i32);
pub struct FILTER_VOLUME_INFORMATION_CLASS(i32);
pub struct FILTER_VOLUME_STANDARD_INFORMATION(i32);
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_ASI_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_ASI_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_IASIL_DETACHED_VOLUME: u32 = 1u32;
pub const FLTFL_IASIM_DETACHED_VOLUME: u32 = 1u32;
pub const FLTFL_IASI_IS_LEGACYFILTER: u32 = 2u32;
pub const FLTFL_IASI_IS_MINIFILTER: u32 = 1u32;
pub const FLTFL_VSI_DETACHED_VOLUME: u32 = 1u32;
pub struct FLT_FILESYSTEM_TYPE(i32);
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1u32;
pub struct FilterFindHandle(i32);
pub struct FilterInstanceFindHandle(i32);
pub struct FilterVolumeFindHandle(i32);
pub struct FilterVolumeInstanceFindHandle(i32);
pub struct HFILTER(i32);
pub struct HFILTER_INSTANCE(i32);
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION(i32);
pub struct INSTANCE_BASIC_INFORMATION(i32);
pub struct INSTANCE_FULL_INFORMATION(i32);
pub struct INSTANCE_INFORMATION_CLASS(i32);
pub const INSTANCE_NAME_MAX_CHARS: u32 = 255u32;
pub struct INSTANCE_PARTIAL_INFORMATION(i32);
pub const VOLUME_NAME_MAX_CHARS: u32 = 1024u32;
pub const WNNC_CRED_MANAGER: u32 = 4294901760u32;
pub const WNNC_NET_10NET: u32 = 327680u32;
pub const WNNC_NET_3IN1: u32 = 2555904u32;
pub const WNNC_NET_9P: u32 = 4718592u32;
pub const WNNC_NET_9TILES: u32 = 589824u32;
pub const WNNC_NET_APPLETALK: u32 = 1245184u32;
pub const WNNC_NET_AS400: u32 = 720896u32;
pub const WNNC_NET_AURISTOR_FS: u32 = 4587520u32;
pub const WNNC_NET_AVID: u32 = 1703936u32;
pub const WNNC_NET_AVID1: u32 = 3801088u32;
pub const WNNC_NET_BMC: u32 = 1572864u32;
pub const WNNC_NET_BWNFS: u32 = 1048576u32;
pub const WNNC_NET_CLEARCASE: u32 = 1441792u32;
pub const WNNC_NET_COGENT: u32 = 1114112u32;
pub const WNNC_NET_CSC: u32 = 2490368u32;
pub const WNNC_NET_DAV: u32 = 3014656u32;
pub const WNNC_NET_DCE: u32 = 1638400u32;
pub const WNNC_NET_DECORB: u32 = 2097152u32;
pub const WNNC_NET_DFS: u32 = 3866624u32;
pub const WNNC_NET_DISTINCT: u32 = 2293760u32;
pub const WNNC_NET_DOCUSHARE: u32 = 4521984u32;
pub const WNNC_NET_DOCUSPACE: u32 = 1769472u32;
pub const WNNC_NET_DRIVEONWEB: u32 = 4063232u32;
pub const WNNC_NET_EXIFS: u32 = 2949120u32;
pub const WNNC_NET_EXTENDNET: u32 = 2686976u32;
pub const WNNC_NET_FARALLON: u32 = 1179648u32;
pub const WNNC_NET_FJ_REDIR: u32 = 2228224u32;
pub const WNNC_NET_FOXBAT: u32 = 2818048u32;
pub const WNNC_NET_FRONTIER: u32 = 1507328u32;
pub const WNNC_NET_FTP_NFS: u32 = 786432u32;
pub const WNNC_NET_GOOGLE: u32 = 4390912u32;
pub const WNNC_NET_HOB_NFS: u32 = 3276800u32;
pub const WNNC_NET_IBMAL: u32 = 3407872u32;
pub const WNNC_NET_INTERGRAPH: u32 = 1310720u32;
pub const WNNC_NET_KNOWARE: u32 = 3080192u32;
pub const WNNC_NET_KWNP: u32 = 3932160u32;
pub const WNNC_NET_LANMAN: u32 = 131072u32;
pub const WNNC_NET_LANSTEP: u32 = 524288u32;
pub const WNNC_NET_LANTASTIC: u32 = 655360u32;
pub const WNNC_NET_LIFENET: u32 = 917504u32;
pub const WNNC_NET_LOCK: u32 = 3473408u32;
pub const WNNC_NET_LOCUS: u32 = 393216u32;
pub const WNNC_NET_MANGOSOFT: u32 = 1835008u32;
pub const WNNC_NET_MASFAX: u32 = 3211264u32;
pub const WNNC_NET_MFILES: u32 = 4259840u32;
pub const WNNC_NET_MSNET: u32 = 65536u32;
pub const WNNC_NET_MS_NFS: u32 = 4325376u32;
pub const WNNC_NET_NDFS: u32 = 4456448u32;
pub const WNNC_NET_NETWARE: u32 = 196608u32;
pub const WNNC_NET_OBJECT_DIRE: u32 = 3145728u32;
pub const WNNC_NET_OPENAFS: u32 = 3735552u32;
pub const WNNC_NET_PATHWORKS: u32 = 851968u32;
pub const WNNC_NET_POWERLAN: u32 = 983040u32;
pub const WNNC_NET_PROTSTOR: u32 = 2162688u32;
pub const WNNC_NET_QUINCY: u32 = 3670016u32;
pub const WNNC_NET_RDR2SAMPLE: u32 = 2424832u32;
pub const WNNC_NET_RIVERFRONT1: u32 = 1966080u32;
pub const WNNC_NET_RIVERFRONT2: u32 = 2031616u32;
pub const WNNC_NET_RSFX: u32 = 4194304u32;
pub const WNNC_NET_SECUREAGENT: u32 = 4653056u32;
pub const WNNC_NET_SERNET: u32 = 1900544u32;
pub const WNNC_NET_SHIVA: u32 = 3342336u32;
pub const WNNC_NET_SMB: u32 = 131072u32;
pub const WNNC_NET_SRT: u32 = 3604480u32;
pub const WNNC_NET_STAC: u32 = 2752512u32;
pub const WNNC_NET_SUN_PC_NFS: u32 = 458752u32;
pub const WNNC_NET_SYMFONET: u32 = 1376256u32;
pub const WNNC_NET_TERMSRV: u32 = 3538944u32;
pub const WNNC_NET_TWINS: u32 = 2359296u32;
pub const WNNC_NET_VINES: u32 = 262144u32;
pub const WNNC_NET_VMWARE: u32 = 4128768u32;
pub const WNNC_NET_YAHOO: u32 = 2883584u32;
pub const WNNC_NET_ZENWORKS: u32 = 3997696u32;
