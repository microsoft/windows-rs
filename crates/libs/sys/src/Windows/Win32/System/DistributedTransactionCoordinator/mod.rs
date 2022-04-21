#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManager(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManagerC(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManagerExA(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    pub fn DtcGetTransactionManagerExW(i_pwszhost: ::windows_sys::core::PCWSTR, i_pwsztmname: ::windows_sys::core::PCWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type APPLICATIONTYPE = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type AUTHENTICATION_LEVEL = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl ::core::marker::Copy for BOID {}
impl ::core::clone::Clone for BOID {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_MSDtcTransaction: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 972609387, data2: 2344, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
pub const CLSID_MSDtcTransactionManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1528343393, data2: 2333, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER = ::core::option::Option<unsafe extern "system" fn(pszhost: ::windows_sys::core::PCSTR, psztmname: ::windows_sys::core::PCSTR, rid: *const ::windows_sys::core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = ::core::option::Option<unsafe extern "system" fn(i_pszhost: ::windows_sys::core::PCSTR, i_psztmname: ::windows_sys::core::PCSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = ::core::option::Option<unsafe extern "system" fn(i_pwszhost: ::windows_sys::core::PCWSTR, i_pwsztmname: ::windows_sys::core::PCWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_INSTALL_CLIENT = ::core::option::Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type DTC_STATUS_ = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STARTING: DTC_STATUS_ = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STARTED: DTC_STATUS_ = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = 5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = 6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = 7i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = 8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTC_STATUS_FAILED: DTC_STATUS_ = 9i32;
pub type IDtcLuConfigure = *mut ::core::ffi::c_void;
pub type IDtcLuRecovery = *mut ::core::ffi::c_void;
pub type IDtcLuRecoveryFactory = *mut ::core::ffi::c_void;
pub type IDtcLuRecoveryInitiatedByDtc = *mut ::core::ffi::c_void;
pub type IDtcLuRecoveryInitiatedByDtcStatusWork = *mut ::core::ffi::c_void;
pub type IDtcLuRecoveryInitiatedByDtcTransWork = *mut ::core::ffi::c_void;
pub type IDtcLuRecoveryInitiatedByLu = *mut ::core::ffi::c_void;
pub type IDtcLuRecoveryInitiatedByLuWork = *mut ::core::ffi::c_void;
pub type IDtcLuRmEnlistment = *mut ::core::ffi::c_void;
pub type IDtcLuRmEnlistmentFactory = *mut ::core::ffi::c_void;
pub type IDtcLuRmEnlistmentSink = *mut ::core::ffi::c_void;
pub type IDtcLuSubordinateDtc = *mut ::core::ffi::c_void;
pub type IDtcLuSubordinateDtcFactory = *mut ::core::ffi::c_void;
pub type IDtcLuSubordinateDtcSink = *mut ::core::ffi::c_void;
pub type IDtcNetworkAccessConfig = *mut ::core::ffi::c_void;
pub type IDtcNetworkAccessConfig2 = *mut ::core::ffi::c_void;
pub type IDtcNetworkAccessConfig3 = *mut ::core::ffi::c_void;
pub type IDtcToXaHelper = *mut ::core::ffi::c_void;
pub type IDtcToXaHelperFactory = *mut ::core::ffi::c_void;
pub type IDtcToXaHelperSinglePipe = *mut ::core::ffi::c_void;
pub type IDtcToXaMapper = *mut ::core::ffi::c_void;
pub type IGetDispenser = *mut ::core::ffi::c_void;
pub type IKernelTransaction = *mut ::core::ffi::c_void;
pub type ILastResourceManager = *mut ::core::ffi::c_void;
pub type IPrepareInfo = *mut ::core::ffi::c_void;
pub type IPrepareInfo2 = *mut ::core::ffi::c_void;
pub type IRMHelper = *mut ::core::ffi::c_void;
pub type IResourceManager = *mut ::core::ffi::c_void;
pub type IResourceManager2 = *mut ::core::ffi::c_void;
pub type IResourceManagerFactory = *mut ::core::ffi::c_void;
pub type IResourceManagerFactory2 = *mut ::core::ffi::c_void;
pub type IResourceManagerRejoinable = *mut ::core::ffi::c_void;
pub type IResourceManagerSink = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type ISOFLAG = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = 8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = 12i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = 5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = 10i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = 15i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = 16i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOFLAG_READONLY: ISOFLAG = 32i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type ISOLATIONLEVEL = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = -1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = 16i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = 256i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = 256i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = 4096i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = 4096i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = 65536i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = 1048576i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = 1048576i32;
pub type ITipHelper = *mut ::core::ffi::c_void;
pub type ITipPullSink = *mut ::core::ffi::c_void;
pub type ITipTransaction = *mut ::core::ffi::c_void;
pub type ITmNodeName = *mut ::core::ffi::c_void;
pub type ITransaction = *mut ::core::ffi::c_void;
pub type ITransaction2 = *mut ::core::ffi::c_void;
pub type ITransactionCloner = *mut ::core::ffi::c_void;
pub type ITransactionDispenser = *mut ::core::ffi::c_void;
pub type ITransactionEnlistmentAsync = *mut ::core::ffi::c_void;
pub type ITransactionExport = *mut ::core::ffi::c_void;
pub type ITransactionExportFactory = *mut ::core::ffi::c_void;
pub type ITransactionImport = *mut ::core::ffi::c_void;
pub type ITransactionImportWhereabouts = *mut ::core::ffi::c_void;
pub type ITransactionLastEnlistmentAsync = *mut ::core::ffi::c_void;
pub type ITransactionLastResourceAsync = *mut ::core::ffi::c_void;
pub type ITransactionOptions = *mut ::core::ffi::c_void;
pub type ITransactionOutcomeEvents = *mut ::core::ffi::c_void;
pub type ITransactionPhase0EnlistmentAsync = *mut ::core::ffi::c_void;
pub type ITransactionPhase0Factory = *mut ::core::ffi::c_void;
pub type ITransactionPhase0NotifyAsync = *mut ::core::ffi::c_void;
pub type ITransactionReceiver = *mut ::core::ffi::c_void;
pub type ITransactionReceiverFactory = *mut ::core::ffi::c_void;
pub type ITransactionResource = *mut ::core::ffi::c_void;
pub type ITransactionResourceAsync = *mut ::core::ffi::c_void;
pub type ITransactionTransmitter = *mut ::core::ffi::c_void;
pub type ITransactionTransmitterFactory = *mut ::core::ffi::c_void;
pub type ITransactionVoterBallotAsync2 = *mut ::core::ffi::c_void;
pub type ITransactionVoterFactory2 = *mut ::core::ffi::c_void;
pub type ITransactionVoterNotifyAsync2 = *mut ::core::ffi::c_void;
pub type IXAConfig = *mut ::core::ffi::c_void;
pub type IXAObtainRMInfo = *mut ::core::ffi::c_void;
pub type IXATransLookup = *mut ::core::ffi::c_void;
pub type IXATransLookup2 = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXBQUALSIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXGTRIDSIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAXINFOSIZE: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V1 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for OLE_TM_CONFIG_PARAMS_V2 {}
impl ::core::clone::Clone for OLE_TM_CONFIG_PARAMS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const RMNAMESZ: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMASYNC: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMENDRSCAN: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_INVAL: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_PROTO: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMER_TMERR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMFAIL: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMJOIN: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMMIGRATE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMMULTIPLE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOFLAGS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOMIGRATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMNOWAIT: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMONEPHASE: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMREGISTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMRESUME: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSTARTRSCAN: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSUCCESS: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMSUSPEND: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TMUSEASYNC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_JOIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const TM_RESUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type TX_MISC_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = 40i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTCONST = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTHEURISTIC = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl ::core::marker::Copy for XACTOPT {}
impl ::core::clone::Clone for XACTOPT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTRM = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTRM_NOREADONLYPREPARES: XACTRM = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTSTAT = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_NONE: XACTSTAT = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPENNORMAL: XACTSTAT = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPENREFUSED: XACTSTAT = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARING: XACTSTAT = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARED: XACTSTAT = 8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = 16i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = 32i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITTING: XACTSTAT = 64i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = 128i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ABORTING: XACTSTAT = 256i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ABORTED: XACTSTAT = 512i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_COMMITTED: XACTSTAT = 1024i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = 2048i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = 4096i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = 8192i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = 16384i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = 32768i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = 65536i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_INDOUBT: XACTSTAT = 131072i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_CLOSED: XACTSTAT = 262144i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_OPEN: XACTSTAT = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_NOTPREPARED: XACTSTAT = 524227i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTSTAT_ALL: XACTSTAT = 524287i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct XACTSTATS {
    pub cOpen: u32,
    pub cCommitting: u32,
    pub cCommitted: u32,
    pub cAborting: u32,
    pub cAborted: u32,
    pub cInDoubt: u32,
    pub cHeuristicDecision: u32,
    pub timeTransactionsUp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for XACTSTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for XACTSTATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACTTC = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_NONE: XACTTC = 0i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC_PHASEONE: XACTTC = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC_PHASETWO: XACTTC = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_SYNC: XACTTC = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_ASYNC_PHASEONE: XACTTC = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACTTC_ASYNC: XACTTC = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
impl ::core::marker::Copy for XACTTRANSINFO {}
impl ::core::clone::Clone for XACTTRANSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XACT_DTC_CONSTANTS = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = -2147168000i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = -2147167999i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = -2147167998i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = -2147167997i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167996i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = -2147167995i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = -2147167994i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = -2147167993i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = -2147167992i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = -2147167991i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = -2147167990i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = -2147167989i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = -2147167988i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = -2147167987i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = -2147167986i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = -2147167985i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = -2147167984i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = -2147167983i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167982i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = -2147167981i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = 315648i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = 315649i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = 65535i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_ASYNC: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_DUPID: i32 = -8i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_INVAL: i32 = -5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_NOTA: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_OUTSIDE: i32 = -9i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_PROTO: i32 = -6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_RMERR: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XAER_RMFAIL: i32 = -7i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_CLOSE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_COMMIT_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_COMPLETE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_END_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_FMTID_DTC: u32 = 4478019u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_FORGET_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURCOM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURHAZ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURMIX: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_HEURRB: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_NOMIGRATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type XA_OPEN_EPT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_PREPARE_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBBASE: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBCOMMFAIL: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBDEADLOCK: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBEND: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBINTEGRITY: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBOTHER: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBPROTO: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBROLLBACK: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBTIMEOUT: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RBTRANSIENT: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RDONLY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_RECOVER_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32, param3: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_RETRY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_ROLLBACK_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type XA_START_EPT = ::core::option::Option<unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32>;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XA_SWITCH_F_DTC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const XIDDATASIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareState = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = 5i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = 6i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareStates_Confirmation = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareStates_Error = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_CompareStates_Response = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_LocalRecovery_Work = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLN_COLD: _DtcLu_Xln = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLN_WARM: _DtcLu_Xln = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln_Confirmation = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = 4i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln_Error = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub type _DtcLu_Xln_Response = i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = 1i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = 2i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = 3i32;
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
pub struct _ProxyConfigParams {
    pub wcThreadsMax: u16,
}
impl ::core::marker::Copy for _ProxyConfigParams {}
impl ::core::clone::Clone for _ProxyConfigParams {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct xa_switch_t {
    pub name: [super::super::Foundation::CHAR; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: isize,
    pub xa_close_entry: isize,
    pub xa_start_entry: isize,
    pub xa_end_entry: isize,
    pub xa_rollback_entry: isize,
    pub xa_prepare_entry: isize,
    pub xa_commit_entry: isize,
    pub xa_recover_entry: isize,
    pub xa_forget_entry: isize,
    pub xa_complete_entry: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xa_switch_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xa_switch_t {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct xid_t {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [super::super::Foundation::CHAR; 128],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for xid_t {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for xid_t {
    fn clone(&self) -> Self {
        *self
    }
}
