#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoCreateActivity(piunknown: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoEnterServiceDomain(pconfigobject: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CoGetDefaultContext(apttype: super::Com::APTTYPE, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoLeaveServiceDomain(punkstatus: ::windows_sys::core::IUnknown);
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetDispenserManager(param0: *mut IDispenserManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetManagedExtensions(dwexts: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn MTSCreateActivity(riid: *const ::windows_sys::core::GUID, ppobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn RecycleSurrogate(lreasoncode: i32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn SafeRef(rid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown) -> *mut ::core::ffi::c_void;
}
pub struct AppDomainHelper(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessRecycleInfo(i32);
pub struct ApplicationProcessStatistics(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationProcessSummary(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ApplicationSummary(i32);
pub struct AutoSvcs_Error_Constants(i32);
pub struct ByotServerEx(i32);
pub struct CAppData(i32);
pub struct CAppStatistics(i32);
pub struct CCLSIDData(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CCLSIDData2(i32);
pub struct COMAdminAccessChecksLevelOptions(i32);
pub struct COMAdminActivationOptions(i32);
pub struct COMAdminApplicationExportOptions(i32);
pub struct COMAdminApplicationInstallOptions(i32);
pub struct COMAdminAuthenticationCapabilitiesOptions(i32);
pub struct COMAdminAuthenticationLevelOptions(i32);
pub struct COMAdminCatalog(i32);
pub struct COMAdminCatalogCollection(i32);
pub struct COMAdminCatalogObject(i32);
pub struct COMAdminComponentFlags(i32);
pub struct COMAdminComponentType(i32);
pub struct COMAdminErrorCodes(i32);
pub struct COMAdminFileFlags(i32);
pub struct COMAdminImpersonationLevelOptions(i32);
pub struct COMAdminInUse(i32);
pub struct COMAdminOS(i32);
pub struct COMAdminQCMessageAuthenticateOptions(i32);
pub struct COMAdminServiceOptions(i32);
pub struct COMAdminServiceStatusOptions(i32);
pub struct COMAdminSynchronizationOptions(i32);
pub struct COMAdminThreadingModels(i32);
pub struct COMAdminTransactionOptions(i32);
pub struct COMAdminTxIsolationLevelOptions(i32);
pub struct COMEvents(i32);
pub struct COMPLUS_APPTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct COMSVCSEVENTINFO(i32);
pub struct CRMClerk(i32);
pub struct CRMFLAGS(i32);
pub struct CRMREGFLAGS(i32);
pub struct CRMRecoveryClerk(i32);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_ACTIVATION_LIMIT: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_CALL_LIMIT: u32 = 4294967293u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_LIFETIME_LIMIT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_MEMORY_LIMIT: u32 = 4294967292u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_NO_REASON_SUPPLIED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const CRR_RECYCLED_FROM_UI: u32 = 4294967291u32;
pub struct CSC_Binding(i32);
pub struct CSC_COMTIIntrinsicsConfig(i32);
pub struct CSC_IISIntrinsicsConfig(i32);
pub struct CSC_InheritanceConfig(i32);
pub struct CSC_PartitionConfig(i32);
pub struct CSC_SxsConfig(i32);
pub struct CSC_SynchronizationConfig(i32);
pub struct CSC_ThreadPool(i32);
pub struct CSC_TrackerConfig(i32);
pub struct CSC_TransactionConfig(i32);
pub struct CServiceConfig(i32);
pub struct ClrAssemblyLocator(i32);
pub struct CoMTSLocator(i32);
pub struct ComServiceEvents(i32);
pub struct ComSystemAppEventData(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ComponentHangMonitorInfo(i32);
pub struct ComponentStatistics(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct ComponentSummary(i32);
pub struct ContextInfo(pub *mut ::core::ffi::c_void);
pub struct ContextInfo2(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_System_Com")]
pub struct CrmLogRecordRead(i32);
pub struct CrmTransactionState(i32);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const DATA_NOT_AVAILABLE: u32 = 4294967295u32;
pub struct DUMPTYPE(i32);
pub struct DispenserManager(i32);
pub struct Dummy30040732(i32);
pub struct EventServer(i32);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const GUID_STRING_SIZE: u32 = 40u32;
pub struct GetAppTrackerDataFlags(i32);
pub struct GetSecurityCallContextAppObject(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HANG_INFO(i32);
pub struct IAppDomainHelper(pub *mut ::core::ffi::c_void);
pub struct IAssemblyLocator(pub *mut ::core::ffi::c_void);
pub struct IAsyncErrorNotify(pub *mut ::core::ffi::c_void);
pub struct ICOMAdminCatalog(pub *mut ::core::ffi::c_void);
pub struct ICOMAdminCatalog2(pub *mut ::core::ffi::c_void);
pub struct ICOMLBArguments(pub *mut ::core::ffi::c_void);
pub struct ICatalogCollection(pub *mut ::core::ffi::c_void);
pub struct ICatalogObject(pub *mut ::core::ffi::c_void);
pub struct ICheckSxsConfig(pub *mut ::core::ffi::c_void);
pub struct IComActivityEvents(pub *mut ::core::ffi::c_void);
pub struct IComApp2Events(pub *mut ::core::ffi::c_void);
pub struct IComAppEvents(pub *mut ::core::ffi::c_void);
pub struct IComCRMEvents(pub *mut ::core::ffi::c_void);
pub struct IComExceptionEvents(pub *mut ::core::ffi::c_void);
pub struct IComIdentityEvents(pub *mut ::core::ffi::c_void);
pub struct IComInstance2Events(pub *mut ::core::ffi::c_void);
pub struct IComInstanceEvents(pub *mut ::core::ffi::c_void);
pub struct IComLTxEvents(pub *mut ::core::ffi::c_void);
pub struct IComMethod2Events(pub *mut ::core::ffi::c_void);
pub struct IComMethodEvents(pub *mut ::core::ffi::c_void);
pub struct IComMtaThreadPoolKnobs(pub *mut ::core::ffi::c_void);
pub struct IComObjectConstruction2Events(pub *mut ::core::ffi::c_void);
pub struct IComObjectConstructionEvents(pub *mut ::core::ffi::c_void);
pub struct IComObjectEvents(pub *mut ::core::ffi::c_void);
pub struct IComObjectPool2Events(pub *mut ::core::ffi::c_void);
pub struct IComObjectPoolEvents(pub *mut ::core::ffi::c_void);
pub struct IComObjectPoolEvents2(pub *mut ::core::ffi::c_void);
pub struct IComQCEvents(pub *mut ::core::ffi::c_void);
pub struct IComResourceEvents(pub *mut ::core::ffi::c_void);
pub struct IComSecurityEvents(pub *mut ::core::ffi::c_void);
pub struct IComStaThreadPoolKnobs(pub *mut ::core::ffi::c_void);
pub struct IComStaThreadPoolKnobs2(pub *mut ::core::ffi::c_void);
pub struct IComThreadEvents(pub *mut ::core::ffi::c_void);
pub struct IComTrackingInfoCollection(pub *mut ::core::ffi::c_void);
pub struct IComTrackingInfoEvents(pub *mut ::core::ffi::c_void);
pub struct IComTrackingInfoObject(pub *mut ::core::ffi::c_void);
pub struct IComTrackingInfoProperties(pub *mut ::core::ffi::c_void);
pub struct IComTransaction2Events(pub *mut ::core::ffi::c_void);
pub struct IComTransactionEvents(pub *mut ::core::ffi::c_void);
pub struct IComUserEvent(pub *mut ::core::ffi::c_void);
pub struct IContextProperties(pub *mut ::core::ffi::c_void);
pub struct IContextSecurityPerimeter(pub *mut ::core::ffi::c_void);
pub struct IContextState(pub *mut ::core::ffi::c_void);
pub struct ICreateWithLocalTransaction(pub *mut ::core::ffi::c_void);
pub struct ICreateWithTipTransactionEx(pub *mut ::core::ffi::c_void);
pub struct ICreateWithTransactionEx(pub *mut ::core::ffi::c_void);
pub struct ICrmCompensator(pub *mut ::core::ffi::c_void);
pub struct ICrmCompensatorVariants(pub *mut ::core::ffi::c_void);
pub struct ICrmFormatLogRecords(pub *mut ::core::ffi::c_void);
pub struct ICrmLogControl(pub *mut ::core::ffi::c_void);
pub struct ICrmMonitor(pub *mut ::core::ffi::c_void);
pub struct ICrmMonitorClerks(pub *mut ::core::ffi::c_void);
pub struct ICrmMonitorLogRecords(pub *mut ::core::ffi::c_void);
pub struct IDispenserDriver(pub *mut ::core::ffi::c_void);
pub struct IDispenserManager(pub *mut ::core::ffi::c_void);
pub struct IEnumNames(pub *mut ::core::ffi::c_void);
pub struct IEventServerTrace(pub *mut ::core::ffi::c_void);
pub struct IGetAppTrackerData(pub *mut ::core::ffi::c_void);
pub struct IGetContextProperties(pub *mut ::core::ffi::c_void);
pub struct IGetSecurityCallContext(pub *mut ::core::ffi::c_void);
pub struct IHolder(pub *mut ::core::ffi::c_void);
pub struct ILBEvents(pub *mut ::core::ffi::c_void);
pub struct IMTSActivity(pub *mut ::core::ffi::c_void);
pub struct IMTSCall(pub *mut ::core::ffi::c_void);
pub struct IMTSLocator(pub *mut ::core::ffi::c_void);
pub struct IManagedActivationEvents(pub *mut ::core::ffi::c_void);
pub struct IManagedObjectInfo(pub *mut ::core::ffi::c_void);
pub struct IManagedPoolAction(pub *mut ::core::ffi::c_void);
pub struct IManagedPooledObj(pub *mut ::core::ffi::c_void);
pub struct IMessageMover(pub *mut ::core::ffi::c_void);
pub struct IMtsEventInfo(pub *mut ::core::ffi::c_void);
pub struct IMtsEvents(pub *mut ::core::ffi::c_void);
pub struct IMtsGrp(pub *mut ::core::ffi::c_void);
pub struct IObjPool(pub *mut ::core::ffi::c_void);
pub struct IObjectConstruct(pub *mut ::core::ffi::c_void);
pub struct IObjectConstructString(pub *mut ::core::ffi::c_void);
pub struct IObjectContext(pub *mut ::core::ffi::c_void);
pub struct IObjectContextActivity(pub *mut ::core::ffi::c_void);
pub struct IObjectContextInfo(pub *mut ::core::ffi::c_void);
pub struct IObjectContextInfo2(pub *mut ::core::ffi::c_void);
pub struct IObjectContextTip(pub *mut ::core::ffi::c_void);
pub struct IObjectControl(pub *mut ::core::ffi::c_void);
pub struct IPlaybackControl(pub *mut ::core::ffi::c_void);
pub struct IPoolManager(pub *mut ::core::ffi::c_void);
pub struct IProcessInitializer(pub *mut ::core::ffi::c_void);
pub struct ISecurityCallContext(pub *mut ::core::ffi::c_void);
pub struct ISecurityCallersColl(pub *mut ::core::ffi::c_void);
pub struct ISecurityIdentityColl(pub *mut ::core::ffi::c_void);
pub struct ISecurityProperty(pub *mut ::core::ffi::c_void);
pub struct ISelectCOMLBServer(pub *mut ::core::ffi::c_void);
pub struct ISendMethodEvents(pub *mut ::core::ffi::c_void);
pub struct IServiceActivity(pub *mut ::core::ffi::c_void);
pub struct IServiceCall(pub *mut ::core::ffi::c_void);
pub struct IServiceComTIIntrinsicsConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceIISIntrinsicsConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceInheritanceConfig(pub *mut ::core::ffi::c_void);
pub struct IServicePartitionConfig(pub *mut ::core::ffi::c_void);
pub struct IServicePool(pub *mut ::core::ffi::c_void);
pub struct IServicePoolConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceSxsConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceSynchronizationConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceSysTxnConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceThreadPoolConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceTrackerConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceTransactionConfig(pub *mut ::core::ffi::c_void);
pub struct IServiceTransactionConfigBase(pub *mut ::core::ffi::c_void);
pub struct ISharedProperty(pub *mut ::core::ffi::c_void);
pub struct ISharedPropertyGroup(pub *mut ::core::ffi::c_void);
pub struct ISharedPropertyGroupManager(pub *mut ::core::ffi::c_void);
pub struct ISystemAppEventData(pub *mut ::core::ffi::c_void);
pub struct IThreadPoolKnobs(pub *mut ::core::ffi::c_void);
pub struct ITransactionContext(pub *mut ::core::ffi::c_void);
pub struct ITransactionContextEx(pub *mut ::core::ffi::c_void);
pub struct ITransactionProperty(pub *mut ::core::ffi::c_void);
pub struct ITransactionProxy(pub *mut ::core::ffi::c_void);
pub struct ITransactionResourcePool(pub *mut ::core::ffi::c_void);
pub struct ITransactionStatus(pub *mut ::core::ffi::c_void);
pub struct ITxProxyHolder(pub *mut ::core::ffi::c_void);
pub struct LBEvents(i32);
pub struct LockModes(i32);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub struct MessageMover(i32);
pub struct MtsGrp(i32);
pub struct ObjectContext(pub *mut ::core::ffi::c_void);
pub struct ObjectControl(pub *mut ::core::ffi::c_void);
pub struct PoolMgr(i32);
pub struct RECYCLE_INFO(i32);
pub struct ReleaseModes(i32);
pub struct SecurityCallContext(i32);
pub struct SecurityCallers(i32);
pub struct SecurityIdentity(i32);
pub struct SecurityProperty(pub *mut ::core::ffi::c_void);
pub struct ServicePool(i32);
pub struct ServicePoolConfig(i32);
pub struct SharedProperty(i32);
pub struct SharedPropertyGroup(i32);
pub struct SharedPropertyGroupManager(i32);
pub struct TRACKING_COLL_TYPE(i32);
pub struct TrackerServer(i32);
pub struct TransactionContext(i32);
pub struct TransactionContextEx(i32);
pub struct TransactionVote(i32);
