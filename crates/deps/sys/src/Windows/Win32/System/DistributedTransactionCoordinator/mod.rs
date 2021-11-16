#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const LOCAL_APPLICATIONTYPE: i32 = 0i32;
pub const CLUSTERRESOURCE_APPLICATIONTYPE: i32 = 1i32;
pub const NO_AUTHENTICATION_REQUIRED: i32 = 0i32;
pub const INCOMING_AUTHENTICATION_REQUIRED: i32 = 1i32;
pub const MUTUAL_AUTHENTICATION_REQUIRED: i32 = 2i32;
#[repr(C)]
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
pub const DTC_STATUS_UNKNOWN: i32 = 0i32;
pub const DTC_STATUS_STARTING: i32 = 1i32;
pub const DTC_STATUS_STARTED: i32 = 2i32;
pub const DTC_STATUS_PAUSING: i32 = 3i32;
pub const DTC_STATUS_PAUSED: i32 = 4i32;
pub const DTC_STATUS_CONTINUING: i32 = 5i32;
pub const DTC_STATUS_STOPPING: i32 = 6i32;
pub const DTC_STATUS_STOPPED: i32 = 7i32;
pub const DTC_STATUS_E_CANTCONTROL: i32 = 8i32;
pub const DTC_STATUS_FAILED: i32 = 9i32;
#[repr(transparent)]
pub struct IDtcLuConfigure(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuConfigure {}
impl ::core::clone::Clone for IDtcLuConfigure {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecovery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecovery {}
impl ::core::clone::Clone for IDtcLuRecovery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecoveryFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecoveryFactory {}
impl ::core::clone::Clone for IDtcLuRecoveryFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecoveryInitiatedByDtc {}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcStatusWork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecoveryInitiatedByDtcStatusWork {}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtcStatusWork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByDtcTransWork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecoveryInitiatedByDtcTransWork {}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByDtcTransWork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecoveryInitiatedByLu {}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByLu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRecoveryInitiatedByLuWork(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRecoveryInitiatedByLuWork {}
impl ::core::clone::Clone for IDtcLuRecoveryInitiatedByLuWork {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRmEnlistment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRmEnlistment {}
impl ::core::clone::Clone for IDtcLuRmEnlistment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRmEnlistmentFactory {}
impl ::core::clone::Clone for IDtcLuRmEnlistmentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuRmEnlistmentSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuRmEnlistmentSink {}
impl ::core::clone::Clone for IDtcLuRmEnlistmentSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuSubordinateDtc(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuSubordinateDtc {}
impl ::core::clone::Clone for IDtcLuSubordinateDtc {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuSubordinateDtcFactory {}
impl ::core::clone::Clone for IDtcLuSubordinateDtcFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcLuSubordinateDtcSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcLuSubordinateDtcSink {}
impl ::core::clone::Clone for IDtcLuSubordinateDtcSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcNetworkAccessConfig {}
impl ::core::clone::Clone for IDtcNetworkAccessConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcNetworkAccessConfig2 {}
impl ::core::clone::Clone for IDtcNetworkAccessConfig2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcNetworkAccessConfig3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcNetworkAccessConfig3 {}
impl ::core::clone::Clone for IDtcNetworkAccessConfig3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcToXaHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcToXaHelper {}
impl ::core::clone::Clone for IDtcToXaHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcToXaHelperFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcToXaHelperFactory {}
impl ::core::clone::Clone for IDtcToXaHelperFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcToXaHelperSinglePipe(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcToXaHelperSinglePipe {}
impl ::core::clone::Clone for IDtcToXaHelperSinglePipe {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDtcToXaMapper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDtcToXaMapper {}
impl ::core::clone::Clone for IDtcToXaMapper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetDispenser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetDispenser {}
impl ::core::clone::Clone for IGetDispenser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKernelTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKernelTransaction {}
impl ::core::clone::Clone for IKernelTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILastResourceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILastResourceManager {}
impl ::core::clone::Clone for ILastResourceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrepareInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrepareInfo {}
impl ::core::clone::Clone for IPrepareInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrepareInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrepareInfo2 {}
impl ::core::clone::Clone for IPrepareInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRMHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRMHelper {}
impl ::core::clone::Clone for IRMHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManager {}
impl ::core::clone::Clone for IResourceManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManager2 {}
impl ::core::clone::Clone for IResourceManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManagerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManagerFactory {}
impl ::core::clone::Clone for IResourceManagerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManagerFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManagerFactory2 {}
impl ::core::clone::Clone for IResourceManagerFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManagerRejoinable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManagerRejoinable {}
impl ::core::clone::Clone for IResourceManagerRejoinable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IResourceManagerSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IResourceManagerSink {}
impl ::core::clone::Clone for IResourceManagerSink {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ISOFLAG_RETAIN_COMMIT_DC: i32 = 1i32;
pub const ISOFLAG_RETAIN_COMMIT: i32 = 2i32;
pub const ISOFLAG_RETAIN_COMMIT_NO: i32 = 3i32;
pub const ISOFLAG_RETAIN_ABORT_DC: i32 = 4i32;
pub const ISOFLAG_RETAIN_ABORT: i32 = 8i32;
pub const ISOFLAG_RETAIN_ABORT_NO: i32 = 12i32;
pub const ISOFLAG_RETAIN_DONTCARE: i32 = 5i32;
pub const ISOFLAG_RETAIN_BOTH: i32 = 10i32;
pub const ISOFLAG_RETAIN_NONE: i32 = 15i32;
pub const ISOFLAG_OPTIMISTIC: i32 = 16i32;
pub const ISOFLAG_READONLY: i32 = 32i32;
pub const ISOLATIONLEVEL_UNSPECIFIED: i32 = -1i32;
pub const ISOLATIONLEVEL_CHAOS: i32 = 16i32;
pub const ISOLATIONLEVEL_READUNCOMMITTED: i32 = 256i32;
pub const ISOLATIONLEVEL_BROWSE: i32 = 256i32;
pub const ISOLATIONLEVEL_CURSORSTABILITY: i32 = 4096i32;
pub const ISOLATIONLEVEL_READCOMMITTED: i32 = 4096i32;
pub const ISOLATIONLEVEL_REPEATABLEREAD: i32 = 65536i32;
pub const ISOLATIONLEVEL_SERIALIZABLE: i32 = 1048576i32;
pub const ISOLATIONLEVEL_ISOLATED: i32 = 1048576i32;
#[repr(transparent)]
pub struct ITipHelper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITipHelper {}
impl ::core::clone::Clone for ITipHelper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITipPullSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITipPullSink {}
impl ::core::clone::Clone for ITipPullSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITipTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITipTransaction {}
impl ::core::clone::Clone for ITipTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITmNodeName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITmNodeName {}
impl ::core::clone::Clone for ITmNodeName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransaction {}
impl ::core::clone::Clone for ITransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransaction2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransaction2 {}
impl ::core::clone::Clone for ITransaction2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionCloner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionCloner {}
impl ::core::clone::Clone for ITransactionCloner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionDispenser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionDispenser {}
impl ::core::clone::Clone for ITransactionDispenser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionEnlistmentAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionEnlistmentAsync {}
impl ::core::clone::Clone for ITransactionEnlistmentAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionExport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionExport {}
impl ::core::clone::Clone for ITransactionExport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionExportFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionExportFactory {}
impl ::core::clone::Clone for ITransactionExportFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionImport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionImport {}
impl ::core::clone::Clone for ITransactionImport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionImportWhereabouts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionImportWhereabouts {}
impl ::core::clone::Clone for ITransactionImportWhereabouts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionLastEnlistmentAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionLastEnlistmentAsync {}
impl ::core::clone::Clone for ITransactionLastEnlistmentAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionLastResourceAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionLastResourceAsync {}
impl ::core::clone::Clone for ITransactionLastResourceAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionOptions {}
impl ::core::clone::Clone for ITransactionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionOutcomeEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionOutcomeEvents {}
impl ::core::clone::Clone for ITransactionOutcomeEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionPhase0EnlistmentAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionPhase0EnlistmentAsync {}
impl ::core::clone::Clone for ITransactionPhase0EnlistmentAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionPhase0Factory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionPhase0Factory {}
impl ::core::clone::Clone for ITransactionPhase0Factory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionPhase0NotifyAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionPhase0NotifyAsync {}
impl ::core::clone::Clone for ITransactionPhase0NotifyAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionReceiver {}
impl ::core::clone::Clone for ITransactionReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionReceiverFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionReceiverFactory {}
impl ::core::clone::Clone for ITransactionReceiverFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionResource {}
impl ::core::clone::Clone for ITransactionResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionResourceAsync(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionResourceAsync {}
impl ::core::clone::Clone for ITransactionResourceAsync {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionTransmitter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionTransmitter {}
impl ::core::clone::Clone for ITransactionTransmitter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionTransmitterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionTransmitterFactory {}
impl ::core::clone::Clone for ITransactionTransmitterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionVoterBallotAsync2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionVoterBallotAsync2 {}
impl ::core::clone::Clone for ITransactionVoterBallotAsync2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionVoterFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionVoterFactory2 {}
impl ::core::clone::Clone for ITransactionVoterFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITransactionVoterNotifyAsync2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITransactionVoterNotifyAsync2 {}
impl ::core::clone::Clone for ITransactionVoterNotifyAsync2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXAConfig(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXAConfig {}
impl ::core::clone::Clone for IXAConfig {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXAObtainRMInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXAObtainRMInfo {}
impl ::core::clone::Clone for IXAObtainRMInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXATransLookup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXATransLookup {}
impl ::core::clone::Clone for IXATransLookup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXATransLookup2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXATransLookup2 {}
impl ::core::clone::Clone for IXATransLookup2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAXBQUALSIZE: u32 = 64u32;
pub const MAXGTRIDSIZE: u32 = 64u32;
pub const MAXINFOSIZE: u32 = 256u32;
#[repr(C)]
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
pub const MAX_TRAN_DESC: i32 = 40i32;
pub const XACTCONST_TIMEOUTINFINITE: i32 = 0i32;
pub const XACTHEURISTIC_ABORT: i32 = 1i32;
pub const XACTHEURISTIC_COMMIT: i32 = 2i32;
pub const XACTHEURISTIC_DAMAGE: i32 = 3i32;
pub const XACTHEURISTIC_DANGER: i32 = 4i32;
#[repr(C)]
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
pub const XACTRM_OPTIMISTICLASTWINS: i32 = 1i32;
pub const XACTRM_NOREADONLYPREPARES: i32 = 2i32;
pub const XACTSTAT_NONE: i32 = 0i32;
pub const XACTSTAT_OPENNORMAL: i32 = 1i32;
pub const XACTSTAT_OPENREFUSED: i32 = 2i32;
pub const XACTSTAT_PREPARING: i32 = 4i32;
pub const XACTSTAT_PREPARED: i32 = 8i32;
pub const XACTSTAT_PREPARERETAINING: i32 = 16i32;
pub const XACTSTAT_PREPARERETAINED: i32 = 32i32;
pub const XACTSTAT_COMMITTING: i32 = 64i32;
pub const XACTSTAT_COMMITRETAINING: i32 = 128i32;
pub const XACTSTAT_ABORTING: i32 = 256i32;
pub const XACTSTAT_ABORTED: i32 = 512i32;
pub const XACTSTAT_COMMITTED: i32 = 1024i32;
pub const XACTSTAT_HEURISTIC_ABORT: i32 = 2048i32;
pub const XACTSTAT_HEURISTIC_COMMIT: i32 = 4096i32;
pub const XACTSTAT_HEURISTIC_DAMAGE: i32 = 8192i32;
pub const XACTSTAT_HEURISTIC_DANGER: i32 = 16384i32;
pub const XACTSTAT_FORCED_ABORT: i32 = 32768i32;
pub const XACTSTAT_FORCED_COMMIT: i32 = 65536i32;
pub const XACTSTAT_INDOUBT: i32 = 131072i32;
pub const XACTSTAT_CLOSED: i32 = 262144i32;
pub const XACTSTAT_OPEN: i32 = 3i32;
pub const XACTSTAT_NOTPREPARED: i32 = 524227i32;
pub const XACTSTAT_ALL: i32 = 524287i32;
#[repr(C)]
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
pub const XACTTC_NONE: i32 = 0i32;
pub const XACTTC_SYNC_PHASEONE: i32 = 1i32;
pub const XACTTC_SYNC_PHASETWO: i32 = 2i32;
pub const XACTTC_SYNC: i32 = 2i32;
pub const XACTTC_ASYNC_PHASEONE: i32 = 4i32;
pub const XACTTC_ASYNC: i32 = 4i32;
#[repr(C)]
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
pub const XACT_E_CONNECTION_REQUEST_DENIED: i32 = -2147168000i32;
pub const XACT_E_TOOMANY_ENLISTMENTS: i32 = -2147167999i32;
pub const XACT_E_DUPLICATE_GUID: i32 = -2147167998i32;
pub const XACT_E_NOTSINGLEPHASE: i32 = -2147167997i32;
pub const XACT_E_RECOVERYALREADYDONE: i32 = -2147167996i32;
pub const XACT_E_PROTOCOL: i32 = -2147167995i32;
pub const XACT_E_RM_FAILURE: i32 = -2147167994i32;
pub const XACT_E_RECOVERY_FAILED: i32 = -2147167993i32;
pub const XACT_E_LU_NOT_FOUND: i32 = -2147167992i32;
pub const XACT_E_DUPLICATE_LU: i32 = -2147167991i32;
pub const XACT_E_LU_NOT_CONNECTED: i32 = -2147167990i32;
pub const XACT_E_DUPLICATE_TRANSID: i32 = -2147167989i32;
pub const XACT_E_LU_BUSY: i32 = -2147167988i32;
pub const XACT_E_LU_NO_RECOVERY_PROCESS: i32 = -2147167987i32;
pub const XACT_E_LU_DOWN: i32 = -2147167986i32;
pub const XACT_E_LU_RECOVERING: i32 = -2147167985i32;
pub const XACT_E_LU_RECOVERY_MISMATCH: i32 = -2147167984i32;
pub const XACT_E_RM_UNAVAILABLE: i32 = -2147167983i32;
pub const XACT_E_LRMRECOVERYALREADYDONE: i32 = -2147167982i32;
pub const XACT_E_NOLASTRESOURCEINTERFACE: i32 = -2147167981i32;
pub const XACT_S_NONOTIFY: i32 = 315648i32;
pub const XACT_OK_NONOTIFY: i32 = 315649i32;
pub const dwUSER_MS_SQLSERVER: i32 = 65535i32;
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
pub const DTCLUCOMPARESTATE_COMMITTED: i32 = 1i32;
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: i32 = 2i32;
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: i32 = 3i32;
pub const DTCLUCOMPARESTATE_HEURISTICRESET: i32 = 4i32;
pub const DTCLUCOMPARESTATE_INDOUBT: i32 = 5i32;
pub const DTCLUCOMPARESTATE_RESET: i32 = 6i32;
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: i32 = 1i32;
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: i32 = 2i32;
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: i32 = 1i32;
pub const DTCLUCOMPARESTATESRESPONSE_OK: i32 = 1i32;
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: i32 = 2i32;
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: i32 = 1i32;
pub const DTCINITIATEDRECOVERYWORK_TRANS: i32 = 2i32;
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: i32 = 3i32;
pub const DTCLUXLN_COLD: i32 = 1i32;
pub const DTCLUXLN_WARM: i32 = 2i32;
pub const DTCLUXLNCONFIRMATION_CONFIRM: i32 = 1i32;
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: i32 = 2i32;
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: i32 = 3i32;
pub const DTCLUXLNCONFIRMATION_OBSOLETE: i32 = 4i32;
pub const DTCLUXLNERROR_PROTOCOL: i32 = 1i32;
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: i32 = 2i32;
pub const DTCLUXLNERROR_COLDWARMMISMATCH: i32 = 3i32;
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: i32 = 1i32;
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: i32 = 2i32;
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: i32 = 3i32;
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: i32 = 4i32;
#[repr(C)]
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
