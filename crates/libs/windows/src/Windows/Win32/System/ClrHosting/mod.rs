pub const APPDOMAIN_FORCE_TRIVIAL_WAIT_OPERATIONS: APPDOMAIN_SECURITY_FLAGS = 8i32;
pub const APPDOMAIN_SECURITY_DEFAULT: APPDOMAIN_SECURITY_FLAGS = 0i32;
pub const APPDOMAIN_SECURITY_FORBID_CROSSAD_REVERSE_PINVOKE: APPDOMAIN_SECURITY_FLAGS = 2i32;
pub const APPDOMAIN_SECURITY_SANDBOXED: APPDOMAIN_SECURITY_FLAGS = 1i32;
pub const BucketParamLength: u32 = 255u32;
pub const BucketParamsCount: u32 = 10u32;
pub const CLR_ASSEMBLY_BUILD_VERSION: u32 = 0u32;
pub const CLR_ASSEMBLY_IDENTITY_FLAGS_DEFAULT: ECLRAssemblyIdentityFlags = 0i32;
pub const CLR_ASSEMBLY_MAJOR_VERSION: u32 = 4u32;
pub const CLR_ASSEMBLY_MINOR_VERSION: u32 = 0u32;
pub const CLR_BUILD_VERSION: u32 = 22220u32;
pub const CLR_DEBUGGING_MANAGED_EVENT_DEBUGGER_LAUNCH: CLR_DEBUGGING_PROCESS_FLAGS = 2i32;
pub const CLR_DEBUGGING_MANAGED_EVENT_PENDING: CLR_DEBUGGING_PROCESS_FLAGS = 1i32;
pub const CLR_MAJOR_VERSION: u32 = 4u32;
pub const CLR_MINOR_VERSION: u32 = 0u32;
pub const CLSID_CLRDebugging: windows_core::GUID = windows_core::GUID::from_u128(0xbacc578d_fbdd_48a4_969f_02d932b74634);
pub const CLSID_CLRDebuggingLegacy: windows_core::GUID = windows_core::GUID::from_u128(0xdf8395b5_a4ba_450b_a77c_a9a47762c520);
pub const CLSID_CLRMetaHost: windows_core::GUID = windows_core::GUID::from_u128(0x9280188d_0e8e_4867_b30c_7fa83884e8de);
pub const CLSID_CLRMetaHostPolicy: windows_core::GUID = windows_core::GUID::from_u128(0x2ebcd49a_1b47_4a61_b13a_4a03701e594b);
pub const CLSID_CLRProfiling: windows_core::GUID = windows_core::GUID::from_u128(0xbd097ed8_733e_43fe_8ed7_a95ff9a8448c);
pub const CLSID_CLRStrongName: windows_core::GUID = windows_core::GUID::from_u128(0xb79b0acd_f5cd_409b_b5a5_a16244610b92);
pub const CLSID_RESOLUTION_DEFAULT: CLSID_RESOLUTION_FLAGS = 0i32;
pub const CLSID_RESOLUTION_REGISTERED: CLSID_RESOLUTION_FLAGS = 1i32;
pub const COR_GC_COUNTS: COR_GC_STAT_TYPES = 1i32;
pub const COR_GC_MEMORYUSAGE: COR_GC_STAT_TYPES = 2i32;
pub const COR_GC_THREAD_HAS_PROMOTED_BYTES: COR_GC_THREAD_STATS_TYPES = 1i32;
pub const DEPRECATED_CLR_API_MESG: windows_core::PCSTR = windows_core::s!("This API has been deprecated. Refer to https://go.microsoft.com/fwlink/?LinkId=143720 for more details.");
pub const DUMP_FLAVOR_CriticalCLRState: ECustomDumpFlavor = 1i32;
pub const DUMP_FLAVOR_Default: ECustomDumpFlavor = 0i32;
pub const DUMP_FLAVOR_Mini: ECustomDumpFlavor = 0i32;
pub const DUMP_FLAVOR_NonHeapCLRState: ECustomDumpFlavor = 2i32;
pub const DUMP_ITEM_None: ECustomDumpItemKind = 0i32;
pub const Event_ClrDisabled: EClrEvent = 1i32;
pub const Event_DomainUnload: EClrEvent = 0i32;
pub const Event_MDAFired: EClrEvent = 2i32;
pub const Event_StackOverflow: EClrEvent = 3i32;
pub const FAIL_AccessViolation: EClrFailure = 5i32;
pub const FAIL_CodeContract: EClrFailure = 6i32;
pub const FAIL_CriticalResource: EClrFailure = 1i32;
pub const FAIL_FatalRuntime: EClrFailure = 2i32;
pub const FAIL_NonCriticalResource: EClrFailure = 0i32;
pub const FAIL_OrphanedLock: EClrFailure = 3i32;
pub const FAIL_StackOverflow: EClrFailure = 4i32;
pub const HOST_APPLICATION_BINDING_POLICY: EHostApplicationPolicy = 1i32;
pub const HOST_BINDING_POLICY_MODIFY_CHAIN: EHostBindingPolicyModifyFlags = 1i32;
pub const HOST_BINDING_POLICY_MODIFY_DEFAULT: EHostBindingPolicyModifyFlags = 0i32;
pub const HOST_BINDING_POLICY_MODIFY_MAX: EHostBindingPolicyModifyFlags = 3i32;
pub const HOST_BINDING_POLICY_MODIFY_REMOVE: EHostBindingPolicyModifyFlags = 2i32;
pub const HOST_TYPE_APPLAUNCH: HOST_TYPE = 1i32;
pub const HOST_TYPE_CORFLAG: HOST_TYPE = 2i32;
pub const HOST_TYPE_DEFAULT: HOST_TYPE = 0i32;
pub const InvalidBucketParamIndex: BucketParameterIndex = 9i32;
pub const LIBID_mscoree: windows_core::GUID = windows_core::GUID::from_u128(0x5477469e_83b1_11d2_8b49_00a0c9b7c9c4);
pub const MALLOC_EXECUTABLE: MALLOC_TYPE = 2i32;
pub const MALLOC_THREADSAFE: MALLOC_TYPE = 1i32;
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_FALSE: METAHOST_CONFIG_FLAGS = 2i32;
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_MASK: METAHOST_CONFIG_FLAGS = 3i32;
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_TRUE: METAHOST_CONFIG_FLAGS = 1i32;
pub const METAHOST_CONFIG_FLAGS_LEGACY_V2_ACTIVATION_POLICY_UNSET: METAHOST_CONFIG_FLAGS = 0i32;
pub const METAHOST_POLICY_APPLY_UPGRADE_POLICY: METAHOST_POLICY_FLAGS = 8i32;
pub const METAHOST_POLICY_EMULATE_EXE_LAUNCH: METAHOST_POLICY_FLAGS = 16i32;
pub const METAHOST_POLICY_ENSURE_SKU_SUPPORTED: METAHOST_POLICY_FLAGS = 128i32;
pub const METAHOST_POLICY_HIGHCOMPAT: METAHOST_POLICY_FLAGS = 0i32;
pub const METAHOST_POLICY_IGNORE_ERROR_MODE: METAHOST_POLICY_FLAGS = 4096i32;
pub const METAHOST_POLICY_SHOW_ERROR_DIALOG: METAHOST_POLICY_FLAGS = 32i32;
pub const METAHOST_POLICY_USE_PROCESS_IMAGE_PATH: METAHOST_POLICY_FLAGS = 64i32;
pub const MaxClrEvent: EClrEvent = 4i32;
pub const MaxClrFailure: EClrFailure = 7i32;
pub const MaxClrOperation: EClrOperation = 7i32;
pub const MaxPolicyAction: EPolicyAction = 10i32;
pub const OPR_AppDomainRudeUnload: EClrOperation = 4i32;
pub const OPR_AppDomainUnload: EClrOperation = 3i32;
pub const OPR_FinalizerRun: EClrOperation = 6i32;
pub const OPR_ProcessExit: EClrOperation = 5i32;
pub const OPR_ThreadAbort: EClrOperation = 0i32;
pub const OPR_ThreadRudeAbortInCriticalRegion: EClrOperation = 2i32;
pub const OPR_ThreadRudeAbortInNonCriticalRegion: EClrOperation = 1i32;
pub const Parameter1: BucketParameterIndex = 0i32;
pub const Parameter2: BucketParameterIndex = 1i32;
pub const Parameter3: BucketParameterIndex = 2i32;
pub const Parameter4: BucketParameterIndex = 3i32;
pub const Parameter5: BucketParameterIndex = 4i32;
pub const Parameter6: BucketParameterIndex = 5i32;
pub const Parameter7: BucketParameterIndex = 6i32;
pub const Parameter8: BucketParameterIndex = 7i32;
pub const Parameter9: BucketParameterIndex = 8i32;
pub const RUNTIME_INFO_DONT_RETURN_DIRECTORY: RUNTIME_INFO_FLAGS = 16i32;
pub const RUNTIME_INFO_DONT_RETURN_VERSION: RUNTIME_INFO_FLAGS = 32i32;
pub const RUNTIME_INFO_DONT_SHOW_ERROR_DIALOG: RUNTIME_INFO_FLAGS = 64i32;
pub const RUNTIME_INFO_IGNORE_ERROR_MODE: RUNTIME_INFO_FLAGS = 4096i32;
pub const RUNTIME_INFO_REQUEST_AMD64: RUNTIME_INFO_FLAGS = 4i32;
pub const RUNTIME_INFO_REQUEST_ARM64: RUNTIME_INFO_FLAGS = 8192i32;
pub const RUNTIME_INFO_REQUEST_IA64: RUNTIME_INFO_FLAGS = 2i32;
pub const RUNTIME_INFO_REQUEST_X86: RUNTIME_INFO_FLAGS = 8i32;
pub const RUNTIME_INFO_UPGRADE_VERSION: RUNTIME_INFO_FLAGS = 1i32;
pub const SO_ClrEngine: StackOverflowType = 1i32;
pub const SO_Managed: StackOverflowType = 0i32;
pub const SO_Other: StackOverflowType = 2i32;
pub const STARTUP_ALWAYSFLOW_IMPERSONATION: STARTUP_FLAGS = 262144i32;
pub const STARTUP_ARM: STARTUP_FLAGS = 4194304i32;
pub const STARTUP_CONCURRENT_GC: STARTUP_FLAGS = 1i32;
pub const STARTUP_DISABLE_COMMITTHREADSTACK: STARTUP_FLAGS = 131072i32;
pub const STARTUP_ETW: STARTUP_FLAGS = 1048576i32;
pub const STARTUP_HOARD_GC_VM: STARTUP_FLAGS = 8192i32;
pub const STARTUP_LEGACY_IMPERSONATION: STARTUP_FLAGS = 65536i32;
pub const STARTUP_LOADER_OPTIMIZATION_MASK: STARTUP_FLAGS = 6i32;
pub const STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN: STARTUP_FLAGS = 4i32;
pub const STARTUP_LOADER_OPTIMIZATION_MULTI_DOMAIN_HOST: STARTUP_FLAGS = 6i32;
pub const STARTUP_LOADER_OPTIMIZATION_SINGLE_DOMAIN: STARTUP_FLAGS = 2i32;
pub const STARTUP_LOADER_SAFEMODE: STARTUP_FLAGS = 16i32;
pub const STARTUP_LOADER_SETPREFERENCE: STARTUP_FLAGS = 256i32;
pub const STARTUP_SERVER_GC: STARTUP_FLAGS = 4096i32;
pub const STARTUP_SINGLE_VERSION_HOSTING_INTERFACE: STARTUP_FLAGS = 16384i32;
pub const STARTUP_TRIM_GC_COMMIT: STARTUP_FLAGS = 524288i32;
pub const TT_ADUNLOAD: ETaskType = 128i32;
pub const TT_DEBUGGERHELPER: ETaskType = 1i32;
pub const TT_FINALIZER: ETaskType = 4i32;
pub const TT_GC: ETaskType = 2i32;
pub const TT_THREADPOOL_GATE: ETaskType = 16i32;
pub const TT_THREADPOOL_IOCOMPLETION: ETaskType = 64i32;
pub const TT_THREADPOOL_TIMER: ETaskType = 8i32;
pub const TT_THREADPOOL_WAIT: ETaskType = 512i32;
pub const TT_THREADPOOL_WORKER: ETaskType = 32i32;
pub const TT_UNKNOWN: ETaskType = -2147483648i32;
pub const TT_USER: ETaskType = 256i32;
pub const WAIT_ALERTABLE: WAIT_OPTION = 2i32;
pub const WAIT_MSGPUMP: WAIT_OPTION = 1i32;
pub const WAIT_NOTINDEADLOCK: WAIT_OPTION = 4i32;
pub const eAbortThread: EPolicyAction = 2i32;
pub const eAll: EApiCategories = 511i32;
pub const eAppDomainCritical: EMemoryCriticalLevel = 1i32;
pub const eCurrentContext: EContextType = 0i32;
pub const eDisableRuntime: EPolicyAction = 9i32;
pub const eExitProcess: EPolicyAction = 6i32;
pub const eExternalProcessMgmt: EApiCategories = 4i32;
pub const eExternalThreading: EApiCategories = 16i32;
pub const eFastExitProcess: EPolicyAction = 7i32;
pub const eHostDeterminedPolicy: EClrUnhandledException = 1i32;
pub const eInitializeNewDomainFlags_NoSecurityChanges: EInitializeNewDomainFlags = 2i32;
pub const eInitializeNewDomainFlags_None: EInitializeNewDomainFlags = 0i32;
pub const eMayLeakOnAbort: EApiCategories = 256i32;
pub const eMemoryAvailableHigh: EMemoryAvailable = 3i32;
pub const eMemoryAvailableLow: EMemoryAvailable = 1i32;
pub const eMemoryAvailableNeutral: EMemoryAvailable = 2i32;
pub const eNoAction: EPolicyAction = 0i32;
pub const eNoChecks: EApiCategories = 0i32;
pub const ePolicyLevelAdmin: EBindPolicyLevels = 32i32;
pub const ePolicyLevelApp: EBindPolicyLevels = 4i32;
pub const ePolicyLevelHost: EBindPolicyLevels = 16i32;
pub const ePolicyLevelNone: EBindPolicyLevels = 0i32;
pub const ePolicyLevelPublisher: EBindPolicyLevels = 8i32;
pub const ePolicyLevelRetargetable: EBindPolicyLevels = 1i32;
pub const ePolicyPortability: EBindPolicyLevels = 64i32;
pub const ePolicyUnifiedToCLR: EBindPolicyLevels = 2i32;
pub const eProcessCritical: EMemoryCriticalLevel = 2i32;
pub const eRestrictedContext: EContextType = 1i32;
pub const eRudeAbortThread: EPolicyAction = 3i32;
pub const eRudeExitProcess: EPolicyAction = 8i32;
pub const eRudeUnloadAppDomain: EPolicyAction = 5i32;
pub const eRuntimeDeterminedPolicy: EClrUnhandledException = 0i32;
pub const eSecurityInfrastructure: EApiCategories = 64i32;
pub const eSelfAffectingProcessMgmt: EApiCategories = 8i32;
pub const eSelfAffectingThreading: EApiCategories = 32i32;
pub const eSharedState: EApiCategories = 2i32;
pub const eSymbolReadingAlways: ESymbolReadingPolicy = 1i32;
pub const eSymbolReadingFullTrustOnly: ESymbolReadingPolicy = 2i32;
pub const eSymbolReadingNever: ESymbolReadingPolicy = 0i32;
pub const eSynchronization: EApiCategories = 1i32;
pub const eTaskCritical: EMemoryCriticalLevel = 0i32;
pub const eThrowException: EPolicyAction = 1i32;
pub const eUI: EApiCategories = 128i32;
pub const eUnloadAppDomain: EPolicyAction = 4i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPDOMAIN_SECURITY_FLAGS(pub i32);
impl windows_core::TypeKind for APPDOMAIN_SECURITY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BucketParameterIndex(pub i32);
impl windows_core::TypeKind for BucketParameterIndex {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CLR_DEBUGGING_PROCESS_FLAGS(pub i32);
impl windows_core::TypeKind for CLR_DEBUGGING_PROCESS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CLSID_RESOLUTION_FLAGS(pub i32);
impl windows_core::TypeKind for CLSID_RESOLUTION_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COR_GC_STAT_TYPES(pub i32);
impl windows_core::TypeKind for COR_GC_STAT_TYPES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COR_GC_THREAD_STATS_TYPES(pub i32);
impl windows_core::TypeKind for COR_GC_THREAD_STATS_TYPES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EApiCategories(pub i32);
impl windows_core::TypeKind for EApiCategories {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EBindPolicyLevels(pub i32);
impl windows_core::TypeKind for EBindPolicyLevels {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ECLRAssemblyIdentityFlags(pub i32);
impl windows_core::TypeKind for ECLRAssemblyIdentityFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EClrEvent(pub i32);
impl windows_core::TypeKind for EClrEvent {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EClrFailure(pub i32);
impl windows_core::TypeKind for EClrFailure {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EClrOperation(pub i32);
impl windows_core::TypeKind for EClrOperation {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EClrUnhandledException(pub i32);
impl windows_core::TypeKind for EClrUnhandledException {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EContextType(pub i32);
impl windows_core::TypeKind for EContextType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ECustomDumpFlavor(pub i32);
impl windows_core::TypeKind for ECustomDumpFlavor {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ECustomDumpItemKind(pub i32);
impl windows_core::TypeKind for ECustomDumpItemKind {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EHostApplicationPolicy(pub i32);
impl windows_core::TypeKind for EHostApplicationPolicy {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EHostBindingPolicyModifyFlags(pub i32);
impl windows_core::TypeKind for EHostBindingPolicyModifyFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EInitializeNewDomainFlags(pub i32);
impl windows_core::TypeKind for EInitializeNewDomainFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EMemoryAvailable(pub i32);
impl windows_core::TypeKind for EMemoryAvailable {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EMemoryCriticalLevel(pub i32);
impl windows_core::TypeKind for EMemoryCriticalLevel {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EPolicyAction(pub i32);
impl windows_core::TypeKind for EPolicyAction {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ESymbolReadingPolicy(pub i32);
impl windows_core::TypeKind for ESymbolReadingPolicy {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ETaskType(pub i32);
impl windows_core::TypeKind for ETaskType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HOST_TYPE(pub i32);
impl windows_core::TypeKind for HOST_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MALLOC_TYPE(pub i32);
impl windows_core::TypeKind for MALLOC_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct METAHOST_CONFIG_FLAGS(pub i32);
impl windows_core::TypeKind for METAHOST_CONFIG_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct METAHOST_POLICY_FLAGS(pub i32);
impl windows_core::TypeKind for METAHOST_POLICY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RUNTIME_INFO_FLAGS(pub i32);
impl windows_core::TypeKind for RUNTIME_INFO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STARTUP_FLAGS(pub i32);
impl windows_core::TypeKind for STARTUP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StackOverflowType(pub i32);
impl windows_core::TypeKind for StackOverflowType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WAIT_OPTION(pub i32);
impl windows_core::TypeKind for WAIT_OPTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AssemblyBindInfo {
    pub dwAppDomainId: u32,
    pub lpReferencedIdentity: windows_core::PCWSTR,
    pub lpPostPolicyIdentity: windows_core::PCWSTR,
    pub ePolicyLevel: u32,
}
impl Default for AssemblyBindInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AssemblyBindInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BucketParameters {
    pub fInited: super::super::Foundation::BOOL,
    pub pszEventTypeName: [u16; 255],
    pub pszParams: [u16; 2550],
}
impl Default for BucketParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BucketParameters {
    type TypeKind = windows_core::CloneType;
}
pub const CLRRuntimeHost: windows_core::GUID = windows_core::GUID::from_u128(0x90f1a06e_7712_4762_86b5_7a5eba6bdb02);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLR_DEBUGGING_VERSION {
    pub wStructVersion: u16,
    pub wMajor: u16,
    pub wMinor: u16,
    pub wBuild: u16,
    pub wRevision: u16,
}
impl Default for CLR_DEBUGGING_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CLR_DEBUGGING_VERSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_GC_STATS {
    pub Flags: u32,
    pub ExplicitGCCount: usize,
    pub GenCollectionsTaken: [usize; 3],
    pub CommittedKBytes: usize,
    pub ReservedKBytes: usize,
    pub Gen0HeapSizeKBytes: usize,
    pub Gen1HeapSizeKBytes: usize,
    pub Gen2HeapSizeKBytes: usize,
    pub LargeObjectHeapSizeKBytes: usize,
    pub KBytesPromotedFromGen0: usize,
    pub KBytesPromotedFromGen1: usize,
}
impl Default for COR_GC_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COR_GC_STATS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_GC_THREAD_STATS {
    pub PerThreadAllocation: u64,
    pub Flags: u32,
}
impl Default for COR_GC_THREAD_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COR_GC_THREAD_STATS {
    type TypeKind = windows_core::CopyType;
}
pub const ComCallUnmarshal: windows_core::GUID = windows_core::GUID::from_u128(0x3f281000_e95a_11d2_886b_00c04f869f04);
pub const ComCallUnmarshalV4: windows_core::GUID = windows_core::GUID::from_u128(0x45fb4600_e6e8_4928_b25e_50476ff79425);
pub const CorRuntimeHost: windows_core::GUID = windows_core::GUID::from_u128(0xcb2f6723_ab3a_11d2_9c40_00c04fa30a3e);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CustomDumpItem {
    pub itemKind: ECustomDumpItemKind,
    pub Anonymous: CustomDumpItem_0,
}
impl Default for CustomDumpItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CustomDumpItem {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union CustomDumpItem_0 {
    pub pReserved: usize,
}
impl Default for CustomDumpItem_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CustomDumpItem_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MDAInfo {
    pub lpMDACaption: windows_core::PCWSTR,
    pub lpMDAMessage: windows_core::PCWSTR,
    pub lpStackTrace: windows_core::PCWSTR,
}
impl Default for MDAInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MDAInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ModuleBindInfo {
    pub dwAppDomainId: u32,
    pub lpAssemblyIdentity: windows_core::PCWSTR,
    pub lpModuleName: windows_core::PCWSTR,
}
impl Default for ModuleBindInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ModuleBindInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StackOverflowInfo {
    pub soType: StackOverflowType,
    pub pExceptionInfo: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS,
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl Default for StackOverflowInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl windows_core::TypeKind for StackOverflowInfo {
    type TypeKind = windows_core::CopyType;
}
pub const TypeNameFactory: windows_core::GUID = windows_core::GUID::from_u128(0xb81ff171_20f3_11d2_8dcc_00a0c9b00525);
pub type CLRCreateInstanceFnPtr = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type CallbackThreadSetFnPtr = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type CallbackThreadUnsetFnPtr = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type CreateInterfaceFnPtr = Option<unsafe extern "system" fn(clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FExecuteInAppDomainCallback = Option<unsafe extern "system" fn(cookie: *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type FLockClrVersionCallback = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type PTLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(__midl____midl_itf_mscoree_0000_00040005: *mut core::ffi::c_void)>;
pub type RuntimeLoadedCallbackFnPtr = Option<unsafe extern "system" fn(pruntimeinfo: Option<ICLRRuntimeInfo>, pfncallbackthreadset: CallbackThreadSetFnPtr, pfncallbackthreadunset: CallbackThreadUnsetFnPtr)>;
