#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManager(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerC(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerExA(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerExW(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct APPLICATIONTYPE(pub i32);
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(0i32);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = APPLICATIONTYPE(1i32);
#[repr(transparent)]
pub struct AUTHENTICATION_LEVEL(pub i32);
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(0i32);
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(1i32);
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = AUTHENTICATION_LEVEL(2i32);
#[repr(C)]
pub struct BOID(i32);
pub const CLSID_MSDtcTransaction: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 972609387, data2: 2344, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
pub const CLSID_MSDtcTransactionManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1528343393, data2: 2333, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER = unsafe extern "system" fn(pszhost: super::super::Foundation::PSTR, psztmname: super::super::Foundation::PSTR, rid: *const ::windows_sys::core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = unsafe extern "system" fn(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = unsafe extern "system" fn(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type DTC_INSTALL_CLIENT = unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> ::windows_sys::core::HRESULT;
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
#[repr(transparent)]
pub struct DTC_STATUS_(pub i32);
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = DTC_STATUS_(0i32);
pub const DTC_STATUS_STARTING: DTC_STATUS_ = DTC_STATUS_(1i32);
pub const DTC_STATUS_STARTED: DTC_STATUS_ = DTC_STATUS_(2i32);
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = DTC_STATUS_(3i32);
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = DTC_STATUS_(4i32);
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = DTC_STATUS_(5i32);
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = DTC_STATUS_(6i32);
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = DTC_STATUS_(7i32);
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = DTC_STATUS_(8i32);
pub const DTC_STATUS_FAILED: DTC_STATUS_ = DTC_STATUS_(9i32);
#[repr(transparent)]
pub struct IDtcLuConfigure(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecovery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecoveryFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLuWork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRmEnlistment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuSubordinateDtc(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcToXaHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcToXaHelperFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcToXaHelperSinglePipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDtcToXaMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetDispenser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKernelTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILastResourceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrepareInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrepareInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRMHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManagerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManagerFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManagerRejoinable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResourceManagerSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISOFLAG(pub i32);
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = ISOFLAG(1i32);
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = ISOFLAG(2i32);
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = ISOFLAG(3i32);
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = ISOFLAG(4i32);
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = ISOFLAG(8i32);
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = ISOFLAG(12i32);
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = ISOFLAG(5i32);
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = ISOFLAG(10i32);
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = ISOFLAG(15i32);
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = ISOFLAG(16i32);
pub const ISOFLAG_READONLY: ISOFLAG = ISOFLAG(32i32);
#[repr(transparent)]
pub struct ISOLATIONLEVEL(pub i32);
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = ISOLATIONLEVEL(-1i32);
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = ISOLATIONLEVEL(16i32);
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = ISOLATIONLEVEL(256i32);
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = ISOLATIONLEVEL(4096i32);
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = ISOLATIONLEVEL(65536i32);
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = ISOLATIONLEVEL(1048576i32);
#[repr(transparent)]
pub struct ITipHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITipPullSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITipTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITmNodeName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransaction2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionCloner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionDispenser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionEnlistmentAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionExport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionExportFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionImport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionImportWhereabouts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionLastEnlistmentAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionLastResourceAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionOutcomeEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionPhase0EnlistmentAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionPhase0Factory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionPhase0NotifyAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionReceiverFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionResourceAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionTransmitter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionTransmitterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionVoterBallotAsync2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionVoterFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransactionVoterNotifyAsync2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXAObtainRMInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXATransLookup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXATransLookup2(pub *mut ::core::ffi::c_void);
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V1(i32);
#[repr(C)]
pub struct OLE_TM_CONFIG_PARAMS_V2(i32);
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
pub const RMNAMESZ: u32 = 32u32;
pub const TMASYNC: i32 = -2147483648i32;
pub const TMENDRSCAN: i32 = 8388608i32;
pub const TMER_INVAL: i32 = -2i32;
pub const TMER_PROTO: i32 = -3i32;
pub const TMER_TMERR: i32 = -1i32;
pub const TMFAIL: i32 = 536870912i32;
pub const TMJOIN: i32 = 2097152i32;
pub const TMMIGRATE: i32 = 1048576i32;
pub const TMMULTIPLE: i32 = 4194304i32;
pub const TMNOFLAGS: i32 = 0i32;
pub const TMNOMIGRATE: i32 = 2i32;
pub const TMNOWAIT: i32 = 268435456i32;
pub const TMONEPHASE: i32 = 1073741824i32;
pub const TMREGISTER: i32 = 1i32;
pub const TMRESUME: i32 = 134217728i32;
pub const TMSTARTRSCAN: i32 = 16777216i32;
pub const TMSUCCESS: i32 = 67108864i32;
pub const TMSUSPEND: i32 = 33554432i32;
pub const TMUSEASYNC: i32 = 4i32;
pub const TM_JOIN: u32 = 2u32;
pub const TM_OK: u32 = 0u32;
pub const TM_RESUME: u32 = 1u32;
#[repr(transparent)]
pub struct TX_MISC_CONSTANTS(pub i32);
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = TX_MISC_CONSTANTS(40i32);
#[repr(transparent)]
pub struct XACTCONST(pub i32);
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = XACTCONST(0i32);
#[repr(transparent)]
pub struct XACTHEURISTIC(pub i32);
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = XACTHEURISTIC(1i32);
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = XACTHEURISTIC(2i32);
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = XACTHEURISTIC(3i32);
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = XACTHEURISTIC(4i32);
#[repr(C)]
pub struct XACTOPT(i32);
#[repr(transparent)]
pub struct XACTRM(pub i32);
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = XACTRM(1i32);
pub const XACTRM_NOREADONLYPREPARES: XACTRM = XACTRM(2i32);
#[repr(transparent)]
pub struct XACTSTAT(pub i32);
pub const XACTSTAT_NONE: XACTSTAT = XACTSTAT(0i32);
pub const XACTSTAT_OPENNORMAL: XACTSTAT = XACTSTAT(1i32);
pub const XACTSTAT_OPENREFUSED: XACTSTAT = XACTSTAT(2i32);
pub const XACTSTAT_PREPARING: XACTSTAT = XACTSTAT(4i32);
pub const XACTSTAT_PREPARED: XACTSTAT = XACTSTAT(8i32);
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = XACTSTAT(16i32);
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = XACTSTAT(32i32);
pub const XACTSTAT_COMMITTING: XACTSTAT = XACTSTAT(64i32);
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = XACTSTAT(128i32);
pub const XACTSTAT_ABORTING: XACTSTAT = XACTSTAT(256i32);
pub const XACTSTAT_ABORTED: XACTSTAT = XACTSTAT(512i32);
pub const XACTSTAT_COMMITTED: XACTSTAT = XACTSTAT(1024i32);
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = XACTSTAT(2048i32);
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = XACTSTAT(4096i32);
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = XACTSTAT(8192i32);
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = XACTSTAT(16384i32);
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = XACTSTAT(32768i32);
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = XACTSTAT(65536i32);
pub const XACTSTAT_INDOUBT: XACTSTAT = XACTSTAT(131072i32);
pub const XACTSTAT_CLOSED: XACTSTAT = XACTSTAT(262144i32);
pub const XACTSTAT_OPEN: XACTSTAT = XACTSTAT(3i32);
pub const XACTSTAT_NOTPREPARED: XACTSTAT = XACTSTAT(524227i32);
pub const XACTSTAT_ALL: XACTSTAT = XACTSTAT(524287i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct XACTSTATS(i32);
#[repr(transparent)]
pub struct XACTTC(pub i32);
pub const XACTTC_NONE: XACTTC = XACTTC(0i32);
pub const XACTTC_SYNC_PHASEONE: XACTTC = XACTTC(1i32);
pub const XACTTC_SYNC_PHASETWO: XACTTC = XACTTC(2i32);
pub const XACTTC_SYNC: XACTTC = XACTTC(2i32);
pub const XACTTC_ASYNC_PHASEONE: XACTTC = XACTTC(4i32);
pub const XACTTC_ASYNC: XACTTC = XACTTC(4i32);
#[repr(C)]
pub struct XACTTRANSINFO(i32);
#[repr(transparent)]
pub struct XACT_DTC_CONSTANTS(pub i32);
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147168000i32);
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167999i32);
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167998i32);
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167997i32);
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167996i32);
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167995i32);
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167994i32);
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167993i32);
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167992i32);
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167991i32);
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167990i32);
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167989i32);
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167988i32);
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167987i32);
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167986i32);
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167985i32);
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167984i32);
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167983i32);
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167982i32);
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(-2147167981i32);
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315648i32);
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(315649i32);
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = XACT_DTC_CONSTANTS(65535i32);
pub const XAER_ASYNC: i32 = -2i32;
pub const XAER_DUPID: i32 = -8i32;
pub const XAER_INVAL: i32 = -5i32;
pub const XAER_NOTA: i32 = -4i32;
pub const XAER_OUTSIDE: i32 = -9i32;
pub const XAER_PROTO: i32 = -6i32;
pub const XAER_RMERR: i32 = -3i32;
pub const XAER_RMFAIL: i32 = -7i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_CLOSE_EPT = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: i32, param2: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_COMMIT_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub type XA_COMPLETE_EPT = unsafe extern "system" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_END_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_FMTID_DTC: u32 = 4478019u32;
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_FORGET_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_HEURCOM: u32 = 7u32;
pub const XA_HEURHAZ: u32 = 8u32;
pub const XA_HEURMIX: u32 = 5u32;
pub const XA_HEURRB: u32 = 6u32;
pub const XA_NOMIGRATE: u32 = 9u32;
pub const XA_OK: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_OPEN_EPT = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: i32, param2: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_PREPARE_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_RBBASE: u32 = 100u32;
pub const XA_RBCOMMFAIL: u32 = 101u32;
pub const XA_RBDEADLOCK: u32 = 102u32;
pub const XA_RBEND: u32 = 107u32;
pub const XA_RBINTEGRITY: u32 = 103u32;
pub const XA_RBOTHER: u32 = 104u32;
pub const XA_RBPROTO: u32 = 105u32;
pub const XA_RBROLLBACK: u32 = 100u32;
pub const XA_RBTIMEOUT: u32 = 106u32;
pub const XA_RBTRANSIENT: u32 = 107u32;
pub const XA_RDONLY: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_RECOVER_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32, param3: i32) -> i32;
pub const XA_RETRY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_ROLLBACK_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type XA_START_EPT = unsafe extern "system" fn(param0: *mut xid_t, param1: i32, param2: i32) -> i32;
pub const XA_SWITCH_F_DTC: u32 = 1u32;
pub const XIDDATASIZE: u32 = 128u32;
#[repr(transparent)]
pub struct _DtcLu_CompareState(pub i32);
pub const DTCLUCOMPARESTATE_COMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(1i32);
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: _DtcLu_CompareState = _DtcLu_CompareState(2i32);
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: _DtcLu_CompareState = _DtcLu_CompareState(3i32);
pub const DTCLUCOMPARESTATE_HEURISTICRESET: _DtcLu_CompareState = _DtcLu_CompareState(4i32);
pub const DTCLUCOMPARESTATE_INDOUBT: _DtcLu_CompareState = _DtcLu_CompareState(5i32);
pub const DTCLUCOMPARESTATE_RESET: _DtcLu_CompareState = _DtcLu_CompareState(6i32);
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Confirmation(pub i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(1i32);
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: _DtcLu_CompareStates_Confirmation = _DtcLu_CompareStates_Confirmation(2i32);
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Error(pub i32);
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: _DtcLu_CompareStates_Error = _DtcLu_CompareStates_Error(1i32);
#[repr(transparent)]
pub struct _DtcLu_CompareStates_Response(pub i32);
pub const DTCLUCOMPARESTATESRESPONSE_OK: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(1i32);
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: _DtcLu_CompareStates_Response = _DtcLu_CompareStates_Response(2i32);
#[repr(transparent)]
pub struct _DtcLu_LocalRecovery_Work(pub i32);
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(1i32);
pub const DTCINITIATEDRECOVERYWORK_TRANS: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(2i32);
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: _DtcLu_LocalRecovery_Work = _DtcLu_LocalRecovery_Work(3i32);
#[repr(transparent)]
pub struct _DtcLu_Xln(pub i32);
pub const DTCLUXLN_COLD: _DtcLu_Xln = _DtcLu_Xln(1i32);
pub const DTCLUXLN_WARM: _DtcLu_Xln = _DtcLu_Xln(2i32);
#[repr(transparent)]
pub struct _DtcLu_Xln_Confirmation(pub i32);
pub const DTCLUXLNCONFIRMATION_CONFIRM: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(1i32);
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(2i32);
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(3i32);
pub const DTCLUXLNCONFIRMATION_OBSOLETE: _DtcLu_Xln_Confirmation = _DtcLu_Xln_Confirmation(4i32);
#[repr(transparent)]
pub struct _DtcLu_Xln_Error(pub i32);
pub const DTCLUXLNERROR_PROTOCOL: _DtcLu_Xln_Error = _DtcLu_Xln_Error(1i32);
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(2i32);
pub const DTCLUXLNERROR_COLDWARMMISMATCH: _DtcLu_Xln_Error = _DtcLu_Xln_Error(3i32);
#[repr(transparent)]
pub struct _DtcLu_Xln_Response(pub i32);
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: _DtcLu_Xln_Response = _DtcLu_Xln_Response(1i32);
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: _DtcLu_Xln_Response = _DtcLu_Xln_Response(2i32);
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(3i32);
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: _DtcLu_Xln_Response = _DtcLu_Xln_Response(4i32);
#[repr(C)]
pub struct _ProxyConfigParams(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct xa_switch_t(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct xid_t(i32);
