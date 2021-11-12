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
pub struct ContextInfo(i32);
pub struct ContextInfo2(i32);
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
pub struct IAppDomainHelper(i32);
pub struct IAssemblyLocator(i32);
pub struct IAsyncErrorNotify(i32);
pub struct ICOMAdminCatalog(i32);
pub struct ICOMAdminCatalog2(i32);
pub struct ICOMLBArguments(i32);
pub struct ICatalogCollection(i32);
pub struct ICatalogObject(i32);
pub struct ICheckSxsConfig(i32);
pub struct IComActivityEvents(i32);
pub struct IComApp2Events(i32);
pub struct IComAppEvents(i32);
pub struct IComCRMEvents(i32);
pub struct IComExceptionEvents(i32);
pub struct IComIdentityEvents(i32);
pub struct IComInstance2Events(i32);
pub struct IComInstanceEvents(i32);
pub struct IComLTxEvents(i32);
pub struct IComMethod2Events(i32);
pub struct IComMethodEvents(i32);
pub struct IComMtaThreadPoolKnobs(i32);
pub struct IComObjectConstruction2Events(i32);
pub struct IComObjectConstructionEvents(i32);
pub struct IComObjectEvents(i32);
pub struct IComObjectPool2Events(i32);
pub struct IComObjectPoolEvents(i32);
pub struct IComObjectPoolEvents2(i32);
pub struct IComQCEvents(i32);
pub struct IComResourceEvents(i32);
pub struct IComSecurityEvents(i32);
pub struct IComStaThreadPoolKnobs(i32);
pub struct IComStaThreadPoolKnobs2(i32);
pub struct IComThreadEvents(i32);
pub struct IComTrackingInfoCollection(i32);
pub struct IComTrackingInfoEvents(i32);
pub struct IComTrackingInfoObject(i32);
pub struct IComTrackingInfoProperties(i32);
pub struct IComTransaction2Events(i32);
pub struct IComTransactionEvents(i32);
pub struct IComUserEvent(i32);
pub struct IContextProperties(i32);
pub struct IContextSecurityPerimeter(i32);
pub struct IContextState(i32);
pub struct ICreateWithLocalTransaction(i32);
pub struct ICreateWithTipTransactionEx(i32);
pub struct ICreateWithTransactionEx(i32);
pub struct ICrmCompensator(i32);
pub struct ICrmCompensatorVariants(i32);
pub struct ICrmFormatLogRecords(i32);
pub struct ICrmLogControl(i32);
pub struct ICrmMonitor(i32);
pub struct ICrmMonitorClerks(i32);
pub struct ICrmMonitorLogRecords(i32);
pub struct IDispenserDriver(i32);
pub struct IDispenserManager(i32);
pub struct IEnumNames(i32);
pub struct IEventServerTrace(i32);
pub struct IGetAppTrackerData(i32);
pub struct IGetContextProperties(i32);
pub struct IGetSecurityCallContext(i32);
pub struct IHolder(i32);
pub struct ILBEvents(i32);
pub struct IMTSActivity(i32);
pub struct IMTSCall(i32);
pub struct IMTSLocator(i32);
pub struct IManagedActivationEvents(i32);
pub struct IManagedObjectInfo(i32);
pub struct IManagedPoolAction(i32);
pub struct IManagedPooledObj(i32);
pub struct IMessageMover(i32);
pub struct IMtsEventInfo(i32);
pub struct IMtsEvents(i32);
pub struct IMtsGrp(i32);
pub struct IObjPool(i32);
pub struct IObjectConstruct(i32);
pub struct IObjectConstructString(i32);
pub struct IObjectContext(i32);
pub struct IObjectContextActivity(i32);
pub struct IObjectContextInfo(i32);
pub struct IObjectContextInfo2(i32);
pub struct IObjectContextTip(i32);
pub struct IObjectControl(i32);
pub struct IPlaybackControl(i32);
pub struct IPoolManager(i32);
pub struct IProcessInitializer(i32);
pub struct ISecurityCallContext(i32);
pub struct ISecurityCallersColl(i32);
pub struct ISecurityIdentityColl(i32);
pub struct ISecurityProperty(i32);
pub struct ISelectCOMLBServer(i32);
pub struct ISendMethodEvents(i32);
pub struct IServiceActivity(i32);
pub struct IServiceCall(i32);
pub struct IServiceComTIIntrinsicsConfig(i32);
pub struct IServiceIISIntrinsicsConfig(i32);
pub struct IServiceInheritanceConfig(i32);
pub struct IServicePartitionConfig(i32);
pub struct IServicePool(i32);
pub struct IServicePoolConfig(i32);
pub struct IServiceSxsConfig(i32);
pub struct IServiceSynchronizationConfig(i32);
pub struct IServiceSysTxnConfig(i32);
pub struct IServiceThreadPoolConfig(i32);
pub struct IServiceTrackerConfig(i32);
pub struct IServiceTransactionConfig(i32);
pub struct IServiceTransactionConfigBase(i32);
pub struct ISharedProperty(i32);
pub struct ISharedPropertyGroup(i32);
pub struct ISharedPropertyGroupManager(i32);
pub struct ISystemAppEventData(i32);
pub struct IThreadPoolKnobs(i32);
pub struct ITransactionContext(i32);
pub struct ITransactionContextEx(i32);
pub struct ITransactionProperty(i32);
pub struct ITransactionProxy(i32);
pub struct ITransactionResourcePool(i32);
pub struct ITransactionStatus(i32);
pub struct ITxProxyHolder(i32);
pub struct LBEvents(i32);
pub struct LockModes(i32);
#[doc = "*Required features: `Win32_System_ComponentServices`*"]
pub const MTXDM_E_ENLISTRESOURCEFAILED: u32 = 2147803392u32;
pub struct MessageMover(i32);
pub struct MtsGrp(i32);
pub struct ObjectContext(i32);
pub struct ObjectControl(i32);
pub struct PoolMgr(i32);
pub struct RECYCLE_INFO(i32);
pub struct ReleaseModes(i32);
pub struct SecurityCallContext(i32);
pub struct SecurityCallers(i32);
pub struct SecurityIdentity(i32);
pub struct SecurityProperty(i32);
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
