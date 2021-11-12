#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManager(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerC(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_dwreserved1: u32, i_wcbreserved2: u16, i_pvreserved2: *const ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerExA(i_pszhost: super::super::Foundation::PSTR, i_psztmname: super::super::Foundation::PSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerExW(i_pwszhost: super::super::Foundation::PWSTR, i_pwsztmname: super::super::Foundation::PWSTR, i_riid: *const ::windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut ::core::ffi::c_void, o_ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
pub struct APPLICATIONTYPE(i32);
pub struct AUTHENTICATION_LEVEL(i32);
pub struct BOID(i32);
pub const CLSID_MSDtcTransaction: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 972609387, data2: 2344, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
pub const CLSID_MSDtcTransactionManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1528343393, data2: 2333, data3: 4561, data4: [151, 223, 0, 192, 79, 185, 97, 138] };
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385i32;
pub struct DTC_GET_TRANSACTION_MANAGER(i32);
pub struct DTC_GET_TRANSACTION_MANAGER_EX_A(i32);
pub struct DTC_GET_TRANSACTION_MANAGER_EX_W(i32);
pub struct DTC_INSTALL_CLIENT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
pub struct DTC_STATUS_(i32);
pub struct IDtcLuConfigure(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecovery(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecoveryFactory(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecoveryInitiatedByDtc(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecoveryInitiatedByLu(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRecoveryInitiatedByLuWork(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRmEnlistment(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRmEnlistmentFactory(pub *mut ::core::ffi::c_void);
pub struct IDtcLuRmEnlistmentSink(pub *mut ::core::ffi::c_void);
pub struct IDtcLuSubordinateDtc(pub *mut ::core::ffi::c_void);
pub struct IDtcLuSubordinateDtcFactory(pub *mut ::core::ffi::c_void);
pub struct IDtcLuSubordinateDtcSink(pub *mut ::core::ffi::c_void);
pub struct IDtcNetworkAccessConfig(pub *mut ::core::ffi::c_void);
pub struct IDtcNetworkAccessConfig2(pub *mut ::core::ffi::c_void);
pub struct IDtcNetworkAccessConfig3(pub *mut ::core::ffi::c_void);
pub struct IDtcToXaHelper(pub *mut ::core::ffi::c_void);
pub struct IDtcToXaHelperFactory(pub *mut ::core::ffi::c_void);
pub struct IDtcToXaHelperSinglePipe(pub *mut ::core::ffi::c_void);
pub struct IDtcToXaMapper(pub *mut ::core::ffi::c_void);
pub struct IGetDispenser(pub *mut ::core::ffi::c_void);
pub struct IKernelTransaction(pub *mut ::core::ffi::c_void);
pub struct ILastResourceManager(pub *mut ::core::ffi::c_void);
pub struct IPrepareInfo(pub *mut ::core::ffi::c_void);
pub struct IPrepareInfo2(pub *mut ::core::ffi::c_void);
pub struct IRMHelper(pub *mut ::core::ffi::c_void);
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
pub struct IResourceManager2(pub *mut ::core::ffi::c_void);
pub struct IResourceManagerFactory(pub *mut ::core::ffi::c_void);
pub struct IResourceManagerFactory2(pub *mut ::core::ffi::c_void);
pub struct IResourceManagerRejoinable(pub *mut ::core::ffi::c_void);
pub struct IResourceManagerSink(pub *mut ::core::ffi::c_void);
pub struct ISOFLAG(i32);
pub struct ISOLATIONLEVEL(i32);
pub struct ITipHelper(pub *mut ::core::ffi::c_void);
pub struct ITipPullSink(pub *mut ::core::ffi::c_void);
pub struct ITipTransaction(pub *mut ::core::ffi::c_void);
pub struct ITmNodeName(pub *mut ::core::ffi::c_void);
pub struct ITransaction(pub *mut ::core::ffi::c_void);
pub struct ITransaction2(pub *mut ::core::ffi::c_void);
pub struct ITransactionCloner(pub *mut ::core::ffi::c_void);
pub struct ITransactionDispenser(pub *mut ::core::ffi::c_void);
pub struct ITransactionEnlistmentAsync(pub *mut ::core::ffi::c_void);
pub struct ITransactionExport(pub *mut ::core::ffi::c_void);
pub struct ITransactionExportFactory(pub *mut ::core::ffi::c_void);
pub struct ITransactionImport(pub *mut ::core::ffi::c_void);
pub struct ITransactionImportWhereabouts(pub *mut ::core::ffi::c_void);
pub struct ITransactionLastEnlistmentAsync(pub *mut ::core::ffi::c_void);
pub struct ITransactionLastResourceAsync(pub *mut ::core::ffi::c_void);
pub struct ITransactionOptions(pub *mut ::core::ffi::c_void);
pub struct ITransactionOutcomeEvents(pub *mut ::core::ffi::c_void);
pub struct ITransactionPhase0EnlistmentAsync(pub *mut ::core::ffi::c_void);
pub struct ITransactionPhase0Factory(pub *mut ::core::ffi::c_void);
pub struct ITransactionPhase0NotifyAsync(pub *mut ::core::ffi::c_void);
pub struct ITransactionReceiver(pub *mut ::core::ffi::c_void);
pub struct ITransactionReceiverFactory(pub *mut ::core::ffi::c_void);
pub struct ITransactionResource(pub *mut ::core::ffi::c_void);
pub struct ITransactionResourceAsync(pub *mut ::core::ffi::c_void);
pub struct ITransactionTransmitter(pub *mut ::core::ffi::c_void);
pub struct ITransactionTransmitterFactory(pub *mut ::core::ffi::c_void);
pub struct ITransactionVoterBallotAsync2(pub *mut ::core::ffi::c_void);
pub struct ITransactionVoterFactory2(pub *mut ::core::ffi::c_void);
pub struct ITransactionVoterNotifyAsync2(pub *mut ::core::ffi::c_void);
pub struct IXAConfig(pub *mut ::core::ffi::c_void);
pub struct IXAObtainRMInfo(pub *mut ::core::ffi::c_void);
pub struct IXATransLookup(pub *mut ::core::ffi::c_void);
pub struct IXATransLookup2(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const MAXBQUALSIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const MAXGTRIDSIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const MAXINFOSIZE: u32 = 256u32;
pub struct OLE_TM_CONFIG_PARAMS_V1(i32);
pub struct OLE_TM_CONFIG_PARAMS_V2(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const RMNAMESZ: u32 = 32u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMASYNC: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMENDRSCAN: i32 = 8388608i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMER_INVAL: i32 = -2i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMER_PROTO: i32 = -3i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMER_TMERR: i32 = -1i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMFAIL: i32 = 536870912i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMJOIN: i32 = 2097152i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMMIGRATE: i32 = 1048576i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMMULTIPLE: i32 = 4194304i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMNOFLAGS: i32 = 0i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMNOMIGRATE: i32 = 2i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMNOWAIT: i32 = 268435456i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMONEPHASE: i32 = 1073741824i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMREGISTER: i32 = 1i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMRESUME: i32 = 134217728i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMSTARTRSCAN: i32 = 16777216i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMSUCCESS: i32 = 67108864i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMSUSPEND: i32 = 33554432i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TMUSEASYNC: i32 = 4i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TM_JOIN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TM_OK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const TM_RESUME: u32 = 1u32;
pub struct TX_MISC_CONSTANTS(i32);
pub struct XACTCONST(i32);
pub struct XACTHEURISTIC(i32);
pub struct XACTOPT(i32);
pub struct XACTRM(i32);
pub struct XACTSTAT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct XACTSTATS(i32);
pub struct XACTTC(i32);
pub struct XACTTRANSINFO(i32);
pub struct XACT_DTC_CONSTANTS(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_ASYNC: i32 = -2i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_DUPID: i32 = -8i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_INVAL: i32 = -5i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_NOTA: i32 = -4i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_OUTSIDE: i32 = -9i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_PROTO: i32 = -6i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_RMERR: i32 = -3i32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XAER_RMFAIL: i32 = -7i32;
pub struct XA_CLOSE_EPT(i32);
pub struct XA_COMMIT_EPT(i32);
pub struct XA_COMPLETE_EPT(i32);
pub struct XA_END_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_FMTID_DTC: u32 = 4478019u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
pub struct XA_FORGET_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_HEURCOM: u32 = 7u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_HEURHAZ: u32 = 8u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_HEURMIX: u32 = 5u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_HEURRB: u32 = 6u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_NOMIGRATE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_OK: u32 = 0u32;
pub struct XA_OPEN_EPT(i32);
pub struct XA_PREPARE_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBBASE: u32 = 100u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBCOMMFAIL: u32 = 101u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBDEADLOCK: u32 = 102u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBEND: u32 = 107u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBINTEGRITY: u32 = 103u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBOTHER: u32 = 104u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBPROTO: u32 = 105u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBROLLBACK: u32 = 100u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBTIMEOUT: u32 = 106u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RBTRANSIENT: u32 = 107u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RDONLY: u32 = 3u32;
pub struct XA_RECOVER_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RETRY: u32 = 4u32;
pub struct XA_ROLLBACK_EPT(i32);
pub struct XA_START_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_SWITCH_F_DTC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XIDDATASIZE: u32 = 128u32;
pub struct _DtcLu_CompareState(i32);
pub struct _DtcLu_CompareStates_Confirmation(i32);
pub struct _DtcLu_CompareStates_Error(i32);
pub struct _DtcLu_CompareStates_Response(i32);
pub struct _DtcLu_LocalRecovery_Work(i32);
pub struct _DtcLu_Xln(i32);
pub struct _DtcLu_Xln_Confirmation(i32);
pub struct _DtcLu_Xln_Error(i32);
pub struct _DtcLu_Xln_Response(i32);
pub struct _ProxyConfigParams(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct xa_switch_t(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct xid_t(i32);
