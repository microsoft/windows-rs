#[cfg(feature = "guiddef")]
windows_link::link!("comsvcs.dll" "system" fn CoCreateActivity(piunknown : *mut core::ffi::c_void, riid : *const super::IID, ppobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "system" fn CoEnterServiceDomain(pconfigobject : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "system" fn CoLeaveServiceDomain(punkstatus : *mut core::ffi::c_void));
#[cfg(feature = "shared")]
windows_link::link!("comsvcs.dll" "system" fn GetManagedExtensions(dwexts : *mut super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("comsvcs.dll" "system" fn MTSCreateActivity(riid : *const super::IID, ppobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("comsvcs.dll" "C" fn RecycleSurrogate(lreasoncode : i32) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("comsvcs.dll" "C" fn SafeRef(rid : *const super::IID, punk : *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct APPDATA {
    pub m_idApp: super::DWORD,
    pub m_szAppGuid: [super::WCHAR; 40],
    pub m_dwAppProcessId: super::DWORD,
    pub m_AppStatistics: APPSTATISTICS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for APPDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct APPSTATISTICS {
    pub m_cTotalCalls: super::DWORD,
    pub m_cTotalInstances: super::DWORD,
    pub m_cTotalClasses: super::DWORD,
    pub m_cCallsPerSecond: super::DWORD,
}
pub const APPTYPE_LIBRARY: COMPLUS_APPTYPE = 0;
pub const APPTYPE_SERVER: COMPLUS_APPTYPE = 1;
pub const APPTYPE_SWC: COMPLUS_APPTYPE = 2;
pub const APPTYPE_UNKNOWN: COMPLUS_APPTYPE = -1;
pub const AppDomainHelper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef24f689_14f8_4d92_b4af_d7b1f0e70fd4);
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct ApplicationProcessRecycleInfo {
    pub IsRecyclable: windows_sys::core::BOOL,
    pub IsRecycled: windows_sys::core::BOOL,
    pub TimeRecycled: super::FILETIME,
    pub TimeToTerminate: super::FILETIME,
    pub RecycleReasonCode: i32,
    pub IsPendingRecycle: windows_sys::core::BOOL,
    pub HasAutomaticLifetimeRecycling: windows_sys::core::BOOL,
    pub TimeForAutomaticRecycling: super::FILETIME,
    pub MemoryLimitInKB: super::ULONG,
    pub MemoryUsageInKBLastCheck: super::ULONG,
    pub ActivationLimit: super::ULONG,
    pub NumActivationsLastReported: super::ULONG,
    pub CallLimit: super::ULONG,
    pub NumCallsLastReported: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ApplicationProcessStatistics {
    pub NumCallsOutstanding: super::ULONG,
    pub NumTrackedComponents: super::ULONG,
    pub NumComponentInstances: super::ULONG,
    pub AvgCallsPerSecond: super::ULONG,
    pub Reserved1: super::ULONG,
    pub Reserved2: super::ULONG,
    pub Reserved3: super::ULONG,
    pub Reserved4: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ApplicationProcessSummary {
    pub PartitionIdPrimaryApplication: windows_sys::core::GUID,
    pub ApplicationIdPrimaryApplication: windows_sys::core::GUID,
    pub ApplicationInstanceId: windows_sys::core::GUID,
    pub ProcessId: super::DWORD,
    pub Type: COMPLUS_APPTYPE,
    pub ProcessExeName: windows_sys::core::PWSTR,
    pub IsService: windows_sys::core::BOOL,
    pub IsPaused: windows_sys::core::BOOL,
    pub IsRecycled: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ApplicationSummary {
    pub ApplicationInstanceId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub ApplicationId: windows_sys::core::GUID,
    pub Type: COMPLUS_APPTYPE,
    pub ApplicationName: windows_sys::core::PWSTR,
    pub NumTrackedComponents: super::ULONG,
    pub NumComponentInstances: super::ULONG,
}
pub const ByotServerEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0aa_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct CLSIDDATA {
    pub m_clsid: super::CLSID,
    pub m_cReferences: super::DWORD,
    pub m_cBound: super::DWORD,
    pub m_cPooled: super::DWORD,
    pub m_cInCall: super::DWORD,
    pub m_dwRespTime: super::DWORD,
    pub m_cCallsCompleted: super::DWORD,
    pub m_cCallsFailed: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CLSIDDATA2 {
    pub m_clsid: super::CLSID,
    pub m_appid: windows_sys::core::GUID,
    pub m_partid: windows_sys::core::GUID,
    pub m_pwszAppName: *mut super::WCHAR,
    pub m_pwszCtxName: *mut super::WCHAR,
    pub m_eAppType: COMPLUS_APPTYPE,
    pub m_cReferences: super::DWORD,
    pub m_cBound: super::DWORD,
    pub m_cPooled: super::DWORD,
    pub m_cInCall: super::DWORD,
    pub m_dwRespTime: super::DWORD,
    pub m_cCallsCompleted: super::DWORD,
    pub m_cCallsFailed: super::DWORD,
}
pub const COMEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0ab_7f19_11d2_978e_0000f8757e2a);
pub type COMPLUS_APPTYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct COMSVCSEVENTINFO {
    pub cbSize: super::DWORD,
    pub dwPid: super::DWORD,
    pub lTime: super::LONGLONG,
    pub lMicroTime: super::LONG,
    pub perfCount: super::LONGLONG,
    pub guidApp: windows_sys::core::GUID,
    pub sMachineName: super::LPOLESTR,
}
pub const CRMClerk: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0bd_7f19_11d2_978e_0000f8757e2a);
pub type CRMFLAGS = i32;
pub const CRMFLAG_FORGETTARGET: CRMFLAGS = 1;
pub const CRMFLAG_REPLAYINPROGRESS: CRMFLAGS = 64;
pub const CRMFLAG_WRITTENDURINGABORT: CRMFLAGS = 8;
pub const CRMFLAG_WRITTENDURINGCOMMIT: CRMFLAGS = 4;
pub const CRMFLAG_WRITTENDURINGPREPARE: CRMFLAGS = 2;
pub const CRMFLAG_WRITTENDURINGRECOVERY: CRMFLAGS = 16;
pub const CRMFLAG_WRITTENDURINGREPLAY: CRMFLAGS = 32;
pub type CRMREGFLAGS = i32;
pub const CRMREGFLAG_ABORTPHASE: CRMREGFLAGS = 4;
pub const CRMREGFLAG_ALLPHASES: CRMREGFLAGS = 7;
pub const CRMREGFLAG_COMMITPHASE: CRMREGFLAGS = 2;
pub const CRMREGFLAG_FAILIFINDOUBTSREMAIN: CRMREGFLAGS = 16;
pub const CRMREGFLAG_PREPAREPHASE: CRMREGFLAGS = 1;
pub const CRMRecoveryClerk: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0be_7f19_11d2_978e_0000f8757e2a);
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294;
pub const CRR_CALL_LIMIT: u32 = 4294967293;
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295;
pub const CRR_MEMORY_LIMIT: u32 = 4294967292;
pub const CRR_NO_REASON_SUPPLIED: i32 = 0;
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291;
pub const CSC_BindToPoolThread: CSC_Binding = 1;
pub type CSC_Binding = i32;
pub type CSC_COMTIIntrinsicsConfig = i32;
pub const CSC_CreateTransactionIfNecessary: CSC_TransactionConfig = 2;
pub const CSC_DontUseTracker: CSC_TrackerConfig = 0;
pub type CSC_IISIntrinsicsConfig = i32;
pub const CSC_IfContainerIsSynchronized: CSC_SynchronizationConfig = 1;
pub const CSC_IfContainerIsTransactional: CSC_TransactionConfig = 1;
pub const CSC_Ignore: CSC_InheritanceConfig = 1;
pub const CSC_Inherit: CSC_InheritanceConfig = 0;
pub const CSC_InheritCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 1;
pub const CSC_InheritIISIntrinsics: CSC_IISIntrinsicsConfig = 1;
pub const CSC_InheritPartition: CSC_PartitionConfig = 1;
pub const CSC_InheritSxs: CSC_SxsConfig = 1;
pub type CSC_InheritanceConfig = i32;
pub const CSC_MTAThreadPool: CSC_ThreadPool = 3;
pub const CSC_NewPartition: CSC_PartitionConfig = 2;
pub const CSC_NewSxs: CSC_SxsConfig = 2;
pub const CSC_NewSynchronization: CSC_SynchronizationConfig = 3;
pub const CSC_NewSynchronizationIfNecessary: CSC_SynchronizationConfig = 2;
pub const CSC_NewTransaction: CSC_TransactionConfig = 3;
pub const CSC_NoBinding: CSC_Binding = 0;
pub const CSC_NoCOMTIIntrinsics: CSC_COMTIIntrinsicsConfig = 0;
pub const CSC_NoIISIntrinsics: CSC_IISIntrinsicsConfig = 0;
pub const CSC_NoPartition: CSC_PartitionConfig = 0;
pub const CSC_NoSxs: CSC_SxsConfig = 0;
pub const CSC_NoSynchronization: CSC_SynchronizationConfig = 0;
pub const CSC_NoTransaction: CSC_TransactionConfig = 0;
pub type CSC_PartitionConfig = i32;
pub const CSC_STAThreadPool: CSC_ThreadPool = 2;
pub type CSC_SxsConfig = i32;
pub type CSC_SynchronizationConfig = i32;
pub type CSC_ThreadPool = i32;
pub const CSC_ThreadPoolInherit: CSC_ThreadPool = 1;
pub const CSC_ThreadPoolNone: CSC_ThreadPool = 0;
pub type CSC_TrackerConfig = i32;
pub type CSC_TransactionConfig = i32;
pub const CSC_UseTracker: CSC_TrackerConfig = 1;
pub const CServiceConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c8_7f19_11d2_978e_0000f8757e2a);
pub const ClrAssemblyLocator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x458aa3b5_265a_4b75_bc05_9bea4630cf18);
pub const CoMTSLocator: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0ac_7f19_11d2_978e_0000f8757e2a);
pub const ComServiceEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c3_7f19_11d2_978e_0000f8757e2a);
pub const ComSystemAppEventData: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c6_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ComponentHangMonitorInfo {
    pub IsMonitored: windows_sys::core::BOOL,
    pub TerminateOnHang: windows_sys::core::BOOL,
    pub AvgCallThresholdInMs: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ComponentStatistics {
    pub NumInstances: super::ULONG,
    pub NumBoundReferences: super::ULONG,
    pub NumPooledObjects: super::ULONG,
    pub NumObjectsInCall: super::ULONG,
    pub AvgResponseTimeInMs: super::ULONG,
    pub NumCallsCompletedRecent: super::ULONG,
    pub NumCallsFailedRecent: super::ULONG,
    pub NumCallsCompletedTotal: super::ULONG,
    pub NumCallsFailedTotal: super::ULONG,
    pub Reserved1: super::ULONG,
    pub Reserved2: super::ULONG,
    pub Reserved3: super::ULONG,
    pub Reserved4: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Default)]
pub struct ComponentSummary {
    pub ApplicationInstanceId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub ApplicationId: windows_sys::core::GUID,
    pub Clsid: super::CLSID,
    pub ClassName: windows_sys::core::PWSTR,
    pub ApplicationName: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct CrmLogRecordRead {
    pub dwCrmFlags: super::DWORD,
    pub dwSequenceNumber: super::DWORD,
    pub blobUserData: super::BLOB,
}
pub type CrmTransactionState = i32;
pub const DATA_NOT_AVAILABLE: u32 = 4294967295;
pub type DUMPTYPE = i32;
pub const DUMPTYPE_FULL: DUMPTYPE = 0;
pub const DUMPTYPE_MINI: DUMPTYPE = 1;
pub const DUMPTYPE_NONE: DUMPTYPE = 2;
pub const DispenserManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c0_7f19_11d2_978e_0000f8757e2a);
pub const Dummy30040732: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a9_7f19_11d2_978e_0000f8757e2a);
pub type Error_Constants = i32;
pub const EventServer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabafbc_7f19_11d2_978e_0000f8757e2a);
pub const GATD_INCLUDE_APPLICATION_NAME: GetAppTrackerDataFlags = 16;
pub const GATD_INCLUDE_CLASS_NAME: GetAppTrackerDataFlags = 8;
pub const GATD_INCLUDE_LIBRARY_APPS: GetAppTrackerDataFlags = 2;
pub const GATD_INCLUDE_PROCESS_EXE_NAME: GetAppTrackerDataFlags = 1;
pub const GATD_INCLUDE_SWC: GetAppTrackerDataFlags = 4;
pub const GUID_STRING_SIZE: i32 = 40;
pub type GetAppTrackerDataFlags = i32;
pub const GetSecurityCallContextAppObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a8_7f19_11d2_978e_0000f8757e2a);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HANG_INFO {
    pub fAppHangMonitorEnabled: windows_sys::core::BOOL,
    pub fTerminateOnHang: windows_sys::core::BOOL,
    pub DumpType: DUMPTYPE,
    pub dwHangTimeout: super::DWORD,
    pub dwDumpCount: super::DWORD,
    pub dwInfoMsgCount: super::DWORD,
}
#[cfg(target_arch = "x86")]
pub type INSTID = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type INSTID = u64;
pub const LBEvents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c1_7f19_11d2_978e_0000f8757e2a);
pub const LockMethod: LockModes = 1;
pub type LockModes = i32;
pub const LockSetGet: LockModes = 0;
pub type MTS_OBJID = u64;
pub type MTS_RESID = u64;
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392;
pub const MessageMover: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0bf_7f19_11d2_978e_0000f8757e2a);
pub const MtsGrp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4b2e958d_0393_11d1_b1ab_00aa00ba3258);
pub const PoolMgr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabafb5_7f19_11d2_978e_0000f8757e2a);
pub const Process: ReleaseModes = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct RECYCLE_INFO {
    pub guidCombaseProcessIdentifier: windows_sys::core::GUID,
    pub ProcessStartTime: super::LONGLONG,
    pub dwRecycleLifetimeLimit: super::DWORD,
    pub dwRecycleMemoryLimit: super::DWORD,
    pub dwRecycleExpirationTimeout: super::DWORD,
}
#[cfg(target_arch = "x86")]
pub type RESID = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type RESID = u64;
pub type RESOURCERATING = u32;
#[cfg(target_arch = "x86")]
pub type RESTYPID = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type RESTYPID = u64;
pub type ReleaseModes = i32;
pub type SRESID = *mut u16;
pub const SecurityCallContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a7_7f19_11d2_978e_0000f8757e2a);
pub const SecurityCallers: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a6_7f19_11d2_978e_0000f8757e2a);
pub const SecurityIdentity: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0a5_7f19_11d2_978e_0000f8757e2a);
pub const ServicePool: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0c9_7f19_11d2_978e_0000f8757e2a);
pub const ServicePoolConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabb0ca_7f19_11d2_978e_0000f8757e2a);
pub const SharedProperty: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a005c05_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a005c0b_a5de_11cf_9e66_00aa00a3f464);
pub const SharedPropertyGroupManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a005c11_a5de_11cf_9e66_00aa00a3f464);
pub const Standard: ReleaseModes = 0;
pub type TIMEINSECS = i32;
pub const TRACKER_INIT_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("Global\\COM+ Tracker Init Event");
pub const TRACKER_STARTSTOP_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("Global\\COM+ Tracker Push Event");
pub type TRACKING_COLL_TYPE = i32;
#[cfg(target_arch = "x86")]
pub type TRANSID = u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type TRANSID = u64;
pub const TRKCOLL_APPLICATIONS: TRACKING_COLL_TYPE = 1;
pub const TRKCOLL_COMPONENTS: TRACKING_COLL_TYPE = 2;
pub const TRKCOLL_PROCESSES: TRACKING_COLL_TYPE = 0;
pub const TrackerServer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecabafb9_7f19_11d2_978e_0000f8757e2a);
pub const TransactionContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7999fc25_d3c6_11cf_acab_00a024a55aef);
pub const TransactionContextEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5cb66670_d3d4_11cf_acab_00a024a55aef);
pub type TransactionVote = i32;
pub const TxAbort: TransactionVote = 1;
pub const TxCommit: TransactionVote = 0;
pub const TxState_Aborted: CrmTransactionState = 2;
pub const TxState_Active: CrmTransactionState = 0;
pub const TxState_Committed: CrmTransactionState = 1;
pub const TxState_Indoubt: CrmTransactionState = 3;
pub const comQCErrApplicationNotQueued: Error_Constants = -2146368000;
pub const comQCErrNoQueueableInterfaces: Error_Constants = -2146367999;
pub const comQCErrQueueTransactMismatch: Error_Constants = -2146367997;
pub const comQCErrQueuingServiceNotAvailable: Error_Constants = -2146367998;
pub const comqcErrBadMarshaledObject: Error_Constants = -2146367914;
pub const comqcErrInvalidMessage: Error_Constants = -2146367920;
pub const comqcErrMarshaledObjSameTxn: Error_Constants = -2146367992;
pub const comqcErrMsgNotAuthenticated: Error_Constants = -2146367916;
pub const comqcErrMsmqConnectorUsed: Error_Constants = -2146367915;
pub const comqcErrMsmqServiceUnavailable: Error_Constants = -2146367917;
pub const comqcErrMsmqSidUnavailable: Error_Constants = -2146367919;
pub const comqcErrOutParam: Error_Constants = -2146367995;
pub const comqcErrPSLoad: Error_Constants = -2146367993;
pub const comqcErrRecorderMarshalled: Error_Constants = -2146367996;
pub const comqcErrRecorderNotTrusted: Error_Constants = -2146367994;
pub const comqcErrWrongMsgExtension: Error_Constants = -2146367918;
pub type constSRESID = *const u16;
pub const mtsErrCtxAborted: Error_Constants = -2147164158;
pub const mtsErrCtxAborting: Error_Constants = -2147164157;
pub const mtsErrCtxNoContext: Error_Constants = -2147164156;
pub const mtsErrCtxNoSecurity: Error_Constants = -2147164147;
pub const mtsErrCtxNotRegistered: Error_Constants = -2147164155;
pub const mtsErrCtxOldReference: Error_Constants = -2147164153;
pub const mtsErrCtxRoleNotFound: Error_Constants = -2147164148;
pub const mtsErrCtxSynchTimeout: Error_Constants = -2147164154;
pub const mtsErrCtxTMNotAvailable: Error_Constants = -2147164145;
pub const mtsErrCtxWrongThread: Error_Constants = -2147164146;
