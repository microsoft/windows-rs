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
#[cfg(feature = "Win32_Foundation")]
pub struct DTC_GET_TRANSACTION_MANAGER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DTC_GET_TRANSACTION_MANAGER_EX_A(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct DTC_GET_TRANSACTION_MANAGER_EX_W(i32);
pub struct DTC_INSTALL_CLIENT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2u32;
pub struct DTC_STATUS_(i32);
pub struct IDtcLuConfigure(i32);
pub struct IDtcLuRecovery(i32);
pub struct IDtcLuRecoveryFactory(i32);
pub struct IDtcLuRecoveryInitiatedByDtc(i32);
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(i32);
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(i32);
pub struct IDtcLuRecoveryInitiatedByLu(i32);
pub struct IDtcLuRecoveryInitiatedByLuWork(i32);
pub struct IDtcLuRmEnlistment(i32);
pub struct IDtcLuRmEnlistmentFactory(i32);
pub struct IDtcLuRmEnlistmentSink(i32);
pub struct IDtcLuSubordinateDtc(i32);
pub struct IDtcLuSubordinateDtcFactory(i32);
pub struct IDtcLuSubordinateDtcSink(i32);
pub struct IDtcNetworkAccessConfig(i32);
pub struct IDtcNetworkAccessConfig2(i32);
pub struct IDtcNetworkAccessConfig3(i32);
pub struct IDtcToXaHelper(i32);
pub struct IDtcToXaHelperFactory(i32);
pub struct IDtcToXaHelperSinglePipe(i32);
pub struct IDtcToXaMapper(i32);
pub struct IGetDispenser(i32);
pub struct IKernelTransaction(i32);
pub struct ILastResourceManager(i32);
pub struct IPrepareInfo(i32);
pub struct IPrepareInfo2(i32);
pub struct IRMHelper(i32);
pub struct IResourceManager(i32);
pub struct IResourceManager2(i32);
pub struct IResourceManagerFactory(i32);
pub struct IResourceManagerFactory2(i32);
pub struct IResourceManagerRejoinable(i32);
pub struct IResourceManagerSink(i32);
pub struct ISOFLAG(i32);
pub struct ISOLATIONLEVEL(i32);
pub struct ITipHelper(i32);
pub struct ITipPullSink(i32);
pub struct ITipTransaction(i32);
pub struct ITmNodeName(i32);
pub struct ITransaction(i32);
pub struct ITransaction2(i32);
pub struct ITransactionCloner(i32);
pub struct ITransactionDispenser(i32);
pub struct ITransactionEnlistmentAsync(i32);
pub struct ITransactionExport(i32);
pub struct ITransactionExportFactory(i32);
pub struct ITransactionImport(i32);
pub struct ITransactionImportWhereabouts(i32);
pub struct ITransactionLastEnlistmentAsync(i32);
pub struct ITransactionLastResourceAsync(i32);
pub struct ITransactionOptions(i32);
pub struct ITransactionOutcomeEvents(i32);
pub struct ITransactionPhase0EnlistmentAsync(i32);
pub struct ITransactionPhase0Factory(i32);
pub struct ITransactionPhase0NotifyAsync(i32);
pub struct ITransactionReceiver(i32);
pub struct ITransactionReceiverFactory(i32);
pub struct ITransactionResource(i32);
pub struct ITransactionResourceAsync(i32);
pub struct ITransactionTransmitter(i32);
pub struct ITransactionTransmitterFactory(i32);
pub struct ITransactionVoterBallotAsync2(i32);
pub struct ITransactionVoterFactory2(i32);
pub struct ITransactionVoterNotifyAsync2(i32);
pub struct IXAConfig(i32);
pub struct IXAObtainRMInfo(i32);
pub struct IXATransLookup(i32);
pub struct IXATransLookup2(i32);
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
#[cfg(feature = "Win32_Foundation")]
pub struct XA_CLOSE_EPT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct XA_COMMIT_EPT(i32);
pub struct XA_COMPLETE_EPT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct XA_END_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_FMTID_DTC: u32 = 4478019u32;
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_FMTID_DTC_VER1: u32 = 21255235u32;
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub struct XA_OPEN_EPT(i32);
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub struct XA_RECOVER_EPT(i32);
#[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`*"]
pub const XA_RETRY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct XA_ROLLBACK_EPT(i32);
#[cfg(feature = "Win32_Foundation")]
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
