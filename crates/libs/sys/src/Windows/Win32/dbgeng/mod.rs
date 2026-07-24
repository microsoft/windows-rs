windows_link::link!("dbgeng.dll" "system" fn DebugConnect(remoteoptions : windows_sys::core::PCSTR, interfaceid : *const windows_sys::core::GUID, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dbgeng.dll" "system" fn DebugConnectWide(remoteoptions : windows_sys::core::PCWSTR, interfaceid : *const windows_sys::core::GUID, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dbgeng.dll" "system" fn DebugCreate(interfaceid : *const windows_sys::core::GUID, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dbgeng.dll" "system" fn DebugCreateEx(interfaceid : *const windows_sys::core::GUID, dbgengoptions : u32, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const DBG_FRAME_DEFAULT: i32 = 0;
pub const DBG_FRAME_IGNORE_INLINE: u32 = 4294967295;
pub const DEBUG_ADDSYNTHMOD_DEFAULT: i32 = 0;
pub const DEBUG_ADDSYNTHMOD_ZEROBASE: i32 = 1;
pub const DEBUG_ADDSYNTHSYM_DEFAULT: i32 = 0;
pub const DEBUG_ANY_ID: u32 = 4294967295;
pub const DEBUG_ASMOPT_DEFAULT: i32 = 0;
pub const DEBUG_ASMOPT_IGNORE_OUTPUT_WIDTH: i32 = 4;
pub const DEBUG_ASMOPT_NO_CODE_BYTES: i32 = 2;
pub const DEBUG_ASMOPT_SOURCE_LINE_NUMBER: i32 = 8;
pub const DEBUG_ASMOPT_VERBOSE: i32 = 1;
pub const DEBUG_ATTACH_DEFAULT: i32 = 0;
pub const DEBUG_ATTACH_EXDI_DRIVER: i32 = 2;
pub const DEBUG_ATTACH_EXISTING: i32 = 2;
pub const DEBUG_ATTACH_INSTALL_DRIVER: i32 = 4;
pub const DEBUG_ATTACH_INVASIVE_NO_INITIAL_BREAK: i32 = 8;
pub const DEBUG_ATTACH_INVASIVE_RESUME_PROCESS: i32 = 16;
pub const DEBUG_ATTACH_KERNEL_CONNECTION: i32 = 0;
pub const DEBUG_ATTACH_LOCAL_KERNEL: i32 = 1;
pub const DEBUG_ATTACH_NONINVASIVE: i32 = 1;
pub const DEBUG_ATTACH_NONINVASIVE_ALLOW_PARTIAL: i32 = 32;
pub const DEBUG_ATTACH_NONINVASIVE_NO_SUSPEND: i32 = 4;
pub const DEBUG_BREAKPOINT_ADDER_ONLY: i32 = 8;
pub const DEBUG_BREAKPOINT_CODE: i32 = 0;
pub const DEBUG_BREAKPOINT_DATA: i32 = 1;
pub const DEBUG_BREAKPOINT_DEFERRED: i32 = 2;
pub const DEBUG_BREAKPOINT_ENABLED: i32 = 4;
pub const DEBUG_BREAKPOINT_GO_ONLY: i32 = 1;
pub const DEBUG_BREAKPOINT_INLINE: i32 = 3;
pub const DEBUG_BREAKPOINT_ONE_SHOT: i32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_BREAKPOINT_PARAMETERS {
    pub Offset: u64,
    pub Id: u32,
    pub BreakType: u32,
    pub ProcType: u32,
    pub Flags: u32,
    pub DataSize: u32,
    pub DataAccessType: u32,
    pub PassCount: u32,
    pub CurrentPassCount: u32,
    pub MatchThread: u32,
    pub CommandSize: u32,
    pub OffsetExpressionSize: u32,
}
pub const DEBUG_BREAKPOINT_TIME: i32 = 2;
pub const DEBUG_BREAK_EXECUTE: i32 = 4;
pub const DEBUG_BREAK_IO: i32 = 8;
pub const DEBUG_BREAK_READ: i32 = 1;
pub const DEBUG_BREAK_WRITE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_CACHED_SYMBOL_INFO {
    pub ModBase: u64,
    pub Arg1: u64,
    pub Arg2: u64,
    pub Id: u32,
    pub Arg3: u32,
}
pub const DEBUG_CDS_ALL: u32 = 4294967295;
pub const DEBUG_CDS_DATA: i32 = 2;
pub const DEBUG_CDS_REFRESH: i32 = 4;
pub const DEBUG_CDS_REFRESH_ADDBREAKPOINT: i32 = 4;
pub const DEBUG_CDS_REFRESH_EVALUATE: i32 = 1;
pub const DEBUG_CDS_REFRESH_EXECUTE: i32 = 2;
pub const DEBUG_CDS_REFRESH_EXECUTECOMMANDFILE: i32 = 3;
pub const DEBUG_CDS_REFRESH_INLINESTEP: i32 = 16;
pub const DEBUG_CDS_REFRESH_INLINESTEP_PSEUDO: i32 = 17;
pub const DEBUG_CDS_REFRESH_REMOVEBREAKPOINT: i32 = 5;
pub const DEBUG_CDS_REFRESH_SETSCOPE: i32 = 12;
pub const DEBUG_CDS_REFRESH_SETSCOPEFRAMEBYINDEX: i32 = 13;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMJITDEBUGINFO: i32 = 14;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMSTOREDEVENT: i32 = 15;
pub const DEBUG_CDS_REFRESH_SETVALUE: i32 = 10;
pub const DEBUG_CDS_REFRESH_SETVALUE2: i32 = 11;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL: i32 = 8;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL2: i32 = 9;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUAL: i32 = 6;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUALUNCACHED: i32 = 7;
pub const DEBUG_CDS_REGISTERS: i32 = 1;
pub const DEBUG_CES_ALL: u32 = 4294967295;
pub const DEBUG_CES_ASSEMBLY_OPTIONS: i32 = 4096;
pub const DEBUG_CES_BREAKPOINTS: i32 = 4;
pub const DEBUG_CES_CODE_LEVEL: i32 = 8;
pub const DEBUG_CES_CURRENT_THREAD: i32 = 1;
pub const DEBUG_CES_EFFECTIVE_PROCESSOR: i32 = 2;
pub const DEBUG_CES_ENGINE_OPTIONS: i32 = 32;
pub const DEBUG_CES_EVENT_FILTERS: i32 = 256;
pub const DEBUG_CES_EXECUTION_STATUS: i32 = 16;
pub const DEBUG_CES_EXPRESSION_SYNTAX: i32 = 8192;
pub const DEBUG_CES_EXTENSIONS: i32 = 1024;
pub const DEBUG_CES_LOG_FILE: i32 = 64;
pub const DEBUG_CES_PROCESS_OPTIONS: i32 = 512;
pub const DEBUG_CES_RADIX: i32 = 128;
pub const DEBUG_CES_STEP_FILTERS: i32 = 65536;
pub const DEBUG_CES_SYSTEMS: i32 = 2048;
pub const DEBUG_CES_TEXT_REPLACEMENTS: i32 = 16384;
pub const DEBUG_CES_VIEWS: i32 = 32768;
pub const DEBUG_CLASS_IMAGE_FILE: i32 = 3;
pub const DEBUG_CLASS_KERNEL: i32 = 1;
pub const DEBUG_CLASS_UNINITIALIZED: i32 = 0;
pub const DEBUG_CLASS_USER_WINDOWS: i32 = 2;
pub const DEBUG_CLIENT_CDB: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_CLIENT_CONTEXT {
    pub cbSize: u32,
    pub eClient: u32,
}
pub const DEBUG_CLIENT_KD: i32 = 5;
pub const DEBUG_CLIENT_NTKD: i32 = 3;
pub const DEBUG_CLIENT_NTSD: i32 = 2;
pub const DEBUG_CLIENT_UNKNOWN: i32 = 0;
pub const DEBUG_CLIENT_VSINT: i32 = 1;
pub const DEBUG_CLIENT_WINDBG: i32 = 6;
pub const DEBUG_CLIENT_WINIDE: i32 = 7;
pub const DEBUG_CMDEX_ADD_EVENT_STRING: i32 = 1;
pub const DEBUG_CMDEX_INVALID: i32 = 0;
pub const DEBUG_CMDEX_RESET_EVENT_STRINGS: i32 = 2;
pub const DEBUG_COMMAND_EXCEPTION_ID: u32 = 3688893886;
pub const DEBUG_CONNECT_SESSION_DEFAULT: i32 = 0;
pub const DEBUG_CONNECT_SESSION_NO_ANNOUNCE: i32 = 2;
pub const DEBUG_CONNECT_SESSION_NO_VERSION: i32 = 1;
pub const DEBUG_CREATE_PROCESS_NO_DEBUG_HEAP: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_CREATE_PROCESS_OPTIONS {
    pub CreateFlags: u32,
    pub EngCreateFlags: u32,
    pub VerifierFlags: u32,
    pub Reserved: u32,
}
pub const DEBUG_CREATE_PROCESS_THROUGH_RTL: i32 = 65536;
pub const DEBUG_CSS_ALL: u32 = 4294967295;
pub const DEBUG_CSS_COLLAPSE_CHILDREN: i32 = 64;
pub const DEBUG_CSS_LOADS: i32 = 1;
pub const DEBUG_CSS_PATHS: i32 = 8;
pub const DEBUG_CSS_SCOPE: i32 = 4;
pub const DEBUG_CSS_SYMBOL_OPTIONS: i32 = 16;
pub const DEBUG_CSS_TYPE_OPTIONS: i32 = 32;
pub const DEBUG_CSS_UNLOADS: i32 = 2;
pub const DEBUG_CURRENT_DEFAULT: i32 = 15;
pub const DEBUG_CURRENT_DISASM: i32 = 2;
pub const DEBUG_CURRENT_REGISTERS: i32 = 4;
pub const DEBUG_CURRENT_SOURCE_LINE: i32 = 8;
pub const DEBUG_CURRENT_SYMBOL: i32 = 1;
pub const DEBUG_DATA_BASE_TRANSLATION_VIRTUAL_OFFSET: i32 = 3;
pub const DEBUG_DATA_BreakpointWithStatusAddr: i32 = 32;
pub const DEBUG_DATA_CmNtCSDVersionAddr: i32 = 616;
pub const DEBUG_DATA_DumpAttributes: i32 = 100072;
pub const DEBUG_DATA_DumpFormatVersion: i32 = 100040;
pub const DEBUG_DATA_DumpMmStorage: i32 = 100064;
pub const DEBUG_DATA_DumpPowerState: i32 = 100056;
pub const DEBUG_DATA_DumpWriterStatus: i32 = 100032;
pub const DEBUG_DATA_DumpWriterVersion: i32 = 100048;
pub const DEBUG_DATA_EtwpDebuggerData: i32 = 816;
pub const DEBUG_DATA_ExpNumberOfPagedPoolsAddr: i32 = 112;
pub const DEBUG_DATA_ExpPagedPoolDescriptorAddr: i32 = 104;
pub const DEBUG_DATA_ExpSystemResourcesListAddr: i32 = 96;
pub const DEBUG_DATA_IopErrorLogListHeadAddr: i32 = 144;
pub const DEBUG_DATA_KPCR_OFFSET: i32 = 0;
pub const DEBUG_DATA_KPRCB_OFFSET: i32 = 1;
pub const DEBUG_DATA_KTHREAD_OFFSET: i32 = 2;
pub const DEBUG_DATA_KdPrintBufferSizeAddr: i32 = 720;
pub const DEBUG_DATA_KdPrintCircularBufferAddr: i32 = 480;
pub const DEBUG_DATA_KdPrintCircularBufferEndAddr: i32 = 488;
pub const DEBUG_DATA_KdPrintCircularBufferPtrAddr: i32 = 712;
pub const DEBUG_DATA_KdPrintRolloverCountAddr: i32 = 504;
pub const DEBUG_DATA_KdPrintWritePointerAddr: i32 = 496;
pub const DEBUG_DATA_KeBugCheckCallbackListHeadAddr: i32 = 128;
pub const DEBUG_DATA_KeTimeIncrementAddr: i32 = 120;
pub const DEBUG_DATA_KeUserCallbackDispatcherAddr: i32 = 64;
pub const DEBUG_DATA_KernBase: i32 = 24;
pub const DEBUG_DATA_KernelVerifierAddr: i32 = 576;
pub const DEBUG_DATA_KiBugcheckDataAddr: i32 = 136;
pub const DEBUG_DATA_KiCallUserModeAddr: i32 = 56;
pub const DEBUG_DATA_KiNormalSystemCall: i32 = 528;
pub const DEBUG_DATA_KiProcessorBlockAddr: i32 = 536;
pub const DEBUG_DATA_MmAllocatedNonPagedPoolAddr: i32 = 592;
pub const DEBUG_DATA_MmAvailablePagesAddr: i32 = 424;
pub const DEBUG_DATA_MmBadPagesDetected: i32 = 800;
pub const DEBUG_DATA_MmDriverCommitAddr: i32 = 352;
pub const DEBUG_DATA_MmExtendedCommitAddr: i32 = 376;
pub const DEBUG_DATA_MmFreePageListHeadAddr: i32 = 392;
pub const DEBUG_DATA_MmHighestPhysicalPageAddr: i32 = 240;
pub const DEBUG_DATA_MmHighestUserAddressAddr: i32 = 456;
pub const DEBUG_DATA_MmLastUnloadedDriverAddr: i32 = 552;
pub const DEBUG_DATA_MmLoadedUserImageListAddr: i32 = 512;
pub const DEBUG_DATA_MmLowestPhysicalPageAddr: i32 = 232;
pub const DEBUG_DATA_MmMaximumNonPagedPoolInBytesAddr: i32 = 256;
pub const DEBUG_DATA_MmModifiedNoWritePageListHeadAddr: i32 = 416;
pub const DEBUG_DATA_MmModifiedPageListHeadAddr: i32 = 408;
pub const DEBUG_DATA_MmNonPagedPoolEndAddr: i32 = 280;
pub const DEBUG_DATA_MmNonPagedPoolStartAddr: i32 = 272;
pub const DEBUG_DATA_MmNonPagedSystemStartAddr: i32 = 264;
pub const DEBUG_DATA_MmNumberOfPagingFilesAddr: i32 = 224;
pub const DEBUG_DATA_MmNumberOfPhysicalPagesAddr: i32 = 248;
pub const DEBUG_DATA_MmPageSize: i32 = 312;
pub const DEBUG_DATA_MmPagedPoolCommitAddr: i32 = 368;
pub const DEBUG_DATA_MmPagedPoolEndAddr: i32 = 296;
pub const DEBUG_DATA_MmPagedPoolInformationAddr: i32 = 304;
pub const DEBUG_DATA_MmPagedPoolStartAddr: i32 = 288;
pub const DEBUG_DATA_MmPeakCommitmentAddr: i32 = 600;
pub const DEBUG_DATA_MmPfnDatabaseAddr: i32 = 192;
pub const DEBUG_DATA_MmPhysicalMemoryBlockAddr: i32 = 624;
pub const DEBUG_DATA_MmProcessCommitAddr: i32 = 360;
pub const DEBUG_DATA_MmResidentAvailablePagesAddr: i32 = 432;
pub const DEBUG_DATA_MmSessionBase: i32 = 632;
pub const DEBUG_DATA_MmSessionSize: i32 = 640;
pub const DEBUG_DATA_MmSharedCommitAddr: i32 = 344;
pub const DEBUG_DATA_MmSizeOfPagedPoolInBytesAddr: i32 = 320;
pub const DEBUG_DATA_MmSpecialPoolTagAddr: i32 = 568;
pub const DEBUG_DATA_MmStandbyPageListHeadAddr: i32 = 400;
pub const DEBUG_DATA_MmSubsectionBaseAddr: i32 = 216;
pub const DEBUG_DATA_MmSystemCacheEndAddr: i32 = 176;
pub const DEBUG_DATA_MmSystemCacheStartAddr: i32 = 168;
pub const DEBUG_DATA_MmSystemCacheWsAddr: i32 = 184;
pub const DEBUG_DATA_MmSystemParentTablePage: i32 = 648;
pub const DEBUG_DATA_MmSystemPtesEndAddr: i32 = 208;
pub const DEBUG_DATA_MmSystemPtesStartAddr: i32 = 200;
pub const DEBUG_DATA_MmSystemRangeStartAddr: i32 = 464;
pub const DEBUG_DATA_MmTotalCommitLimitAddr: i32 = 328;
pub const DEBUG_DATA_MmTotalCommitLimitMaximumAddr: i32 = 608;
pub const DEBUG_DATA_MmTotalCommittedPagesAddr: i32 = 336;
pub const DEBUG_DATA_MmTriageActionTakenAddr: i32 = 560;
pub const DEBUG_DATA_MmUnloadedDriversAddr: i32 = 544;
pub const DEBUG_DATA_MmUserProbeAddressAddr: i32 = 472;
pub const DEBUG_DATA_MmVerifierDataAddr: i32 = 584;
pub const DEBUG_DATA_MmVirtualTranslationBase: i32 = 656;
pub const DEBUG_DATA_MmZeroedPageListHeadAddr: i32 = 384;
pub const DEBUG_DATA_NonPagedPoolDescriptorAddr: i32 = 448;
pub const DEBUG_DATA_NtBuildLabAddr: i32 = 520;
pub const DEBUG_DATA_ObpRootDirectoryObjectAddr: i32 = 152;
pub const DEBUG_DATA_ObpTypeObjectTypeAddr: i32 = 160;
pub const DEBUG_DATA_OffsetEprocessDirectoryTableBase: i32 = 686;
pub const DEBUG_DATA_OffsetEprocessParentCID: i32 = 684;
pub const DEBUG_DATA_OffsetEprocessPeb: i32 = 682;
pub const DEBUG_DATA_OffsetKThreadApcProcess: i32 = 672;
pub const DEBUG_DATA_OffsetKThreadBStore: i32 = 676;
pub const DEBUG_DATA_OffsetKThreadBStoreLimit: i32 = 678;
pub const DEBUG_DATA_OffsetKThreadInitialStack: i32 = 670;
pub const DEBUG_DATA_OffsetKThreadKernelStack: i32 = 668;
pub const DEBUG_DATA_OffsetKThreadNextProcessor: i32 = 664;
pub const DEBUG_DATA_OffsetKThreadState: i32 = 674;
pub const DEBUG_DATA_OffsetKThreadTeb: i32 = 666;
pub const DEBUG_DATA_OffsetPrcbCpuType: i32 = 696;
pub const DEBUG_DATA_OffsetPrcbCurrentThread: i32 = 692;
pub const DEBUG_DATA_OffsetPrcbDpcRoutine: i32 = 690;
pub const DEBUG_DATA_OffsetPrcbMhz: i32 = 694;
pub const DEBUG_DATA_OffsetPrcbNumber: i32 = 702;
pub const DEBUG_DATA_OffsetPrcbProcessorState: i32 = 700;
pub const DEBUG_DATA_OffsetPrcbVendorString: i32 = 698;
pub const DEBUG_DATA_PROCESSOR_IDENTIFICATION: i32 = 4;
pub const DEBUG_DATA_PROCESSOR_SPEED: i32 = 5;
pub const DEBUG_DATA_PaeEnabled: i32 = 100000;
pub const DEBUG_DATA_PagingLevels: i32 = 100080;
pub const DEBUG_DATA_PoolTrackTableAddr: i32 = 440;
pub const DEBUG_DATA_ProductType: i32 = 100016;
pub const DEBUG_DATA_PsActiveProcessHeadAddr: i32 = 80;
pub const DEBUG_DATA_PsLoadedModuleListAddr: i32 = 72;
pub const DEBUG_DATA_PspCidTableAddr: i32 = 88;
pub const DEBUG_DATA_PteBase: i32 = 864;
pub const DEBUG_DATA_SPACE_BUS_DATA: i32 = 5;
pub const DEBUG_DATA_SPACE_CONTROL: i32 = 2;
pub const DEBUG_DATA_SPACE_COUNT: i32 = 7;
pub const DEBUG_DATA_SPACE_DEBUGGER_DATA: i32 = 6;
pub const DEBUG_DATA_SPACE_IO: i32 = 3;
pub const DEBUG_DATA_SPACE_MSR: i32 = 4;
pub const DEBUG_DATA_SPACE_PHYSICAL: i32 = 1;
pub const DEBUG_DATA_SPACE_VIRTUAL: i32 = 0;
pub const DEBUG_DATA_SavedContextAddr: i32 = 40;
pub const DEBUG_DATA_SharedUserData: i32 = 100008;
pub const DEBUG_DATA_SizeEProcess: i32 = 680;
pub const DEBUG_DATA_SizeEThread: i32 = 704;
pub const DEBUG_DATA_SizePrcb: i32 = 688;
pub const DEBUG_DATA_SuiteMask: i32 = 100024;
pub const DEBUG_DISASM_EFFECTIVE_ADDRESS: i32 = 1;
pub const DEBUG_DISASM_MATCHING_SYMBOLS: i32 = 2;
pub const DEBUG_DISASM_SOURCE_FILE_NAME: i32 = 8;
pub const DEBUG_DISASM_SOURCE_LINE_NUMBER: i32 = 4;
pub const DEBUG_DUMP_ACTIVE: i32 = 1030;
pub const DEBUG_DUMP_DEFAULT: i32 = 1025;
pub const DEBUG_DUMP_FILE_BASE: u32 = 4294967295;
pub const DEBUG_DUMP_FILE_LOAD_FAILED_INDEX: u32 = 4294967295;
pub const DEBUG_DUMP_FILE_ORIGINAL_CAB_INDEX: u32 = 4294967294;
pub const DEBUG_DUMP_FILE_PAGE_FILE_DUMP: i32 = 0;
pub const DEBUG_DUMP_FULL: i32 = 1026;
pub const DEBUG_DUMP_IMAGE_FILE: i32 = 1027;
pub const DEBUG_DUMP_SMALL: i32 = 1024;
pub const DEBUG_DUMP_TRACE_LOG: i32 = 1028;
pub const DEBUG_DUMP_WINDOWS_CE: i32 = 1029;
pub const DEBUG_ECREATE_PROCESS_DEFAULT: i32 = 0;
pub const DEBUG_ECREATE_PROCESS_INHERIT_HANDLES: i32 = 1;
pub const DEBUG_ECREATE_PROCESS_USE_IMPLICIT_COMMAND_LINE: i32 = 4;
pub const DEBUG_ECREATE_PROCESS_USE_VERIFIER_FLAGS: i32 = 2;
pub const DEBUG_EINDEX_FROM_CURRENT: i32 = 2;
pub const DEBUG_EINDEX_FROM_END: i32 = 1;
pub const DEBUG_EINDEX_FROM_START: i32 = 0;
pub const DEBUG_EINDEX_NAME: i32 = 0;
pub const DEBUG_END_ACTIVE_DETACH: i32 = 2;
pub const DEBUG_END_ACTIVE_TERMINATE: i32 = 1;
pub const DEBUG_END_DISCONNECT: i32 = 4;
pub const DEBUG_END_PASSIVE: i32 = 0;
pub const DEBUG_END_REENTRANT: i32 = 3;
pub const DEBUG_ENGOPT_ALL: i32 = 32505855;
pub const DEBUG_ENGOPT_ALLOW_NETWORK_PATHS: i32 = 4;
pub const DEBUG_ENGOPT_ALLOW_READ_ONLY_BREAKPOINTS: i32 = 1024;
pub const DEBUG_ENGOPT_DEBUGGING_SENSITIVE_DATA: i32 = 4194304;
pub const DEBUG_ENGOPT_DISABLESQM: i32 = 524288;
pub const DEBUG_ENGOPT_DISABLE_EXECUTION_COMMANDS: i32 = 65536;
pub const DEBUG_ENGOPT_DISABLE_MANAGED_SUPPORT: i32 = 16384;
pub const DEBUG_ENGOPT_DISABLE_MODULE_SYMBOL_LOAD: i32 = 32768;
pub const DEBUG_ENGOPT_DISABLE_STEPLINES_OPTIONS: i32 = 2097152;
pub const DEBUG_ENGOPT_DISALLOW_IMAGE_FILE_MAPPING: i32 = 131072;
pub const DEBUG_ENGOPT_DISALLOW_NETWORK_PATHS: i32 = 8;
pub const DEBUG_ENGOPT_DISALLOW_SHELL_COMMANDS: i32 = 4096;
pub const DEBUG_ENGOPT_FAIL_INCOMPLETE_INFORMATION: i32 = 512;
pub const DEBUG_ENGOPT_FINAL_BREAK: i32 = 128;
pub const DEBUG_ENGOPT_IGNORE_DBGHELP_VERSION: i32 = 1;
pub const DEBUG_ENGOPT_IGNORE_EXTENSION_VERSIONS: i32 = 2;
pub const DEBUG_ENGOPT_IGNORE_LOADER_EXCEPTIONS: i32 = 16;
pub const DEBUG_ENGOPT_INITIAL_BREAK: i32 = 32;
pub const DEBUG_ENGOPT_INITIAL_MODULE_BREAK: i32 = 64;
pub const DEBUG_ENGOPT_KD_QUIET_MODE: i32 = 8192;
pub const DEBUG_ENGOPT_NETWORK_PATHS: i32 = 12;
pub const DEBUG_ENGOPT_NO_EXECUTE_REPEAT: i32 = 256;
pub const DEBUG_ENGOPT_PREFER_DML: i32 = 262144;
pub const DEBUG_ENGOPT_PREFER_TRACE_FILES: i32 = 8388608;
pub const DEBUG_ENGOPT_RESOLVE_SHADOWED_VARIABLES: i32 = 16777216;
pub const DEBUG_ENGOPT_SYNCHRONIZE_BREAKPOINTS: i32 = 2048;
pub const DEBUG_EVENT_BREAKPOINT: i32 = 1;
pub const DEBUG_EVENT_CHANGE_DEBUGGEE_STATE: i32 = 1024;
pub const DEBUG_EVENT_CHANGE_ENGINE_STATE: i32 = 2048;
pub const DEBUG_EVENT_CHANGE_SYMBOL_STATE: i32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_EVENT_CONTEXT {
    pub Size: u32,
    pub ProcessEngineId: u32,
    pub ThreadEngineId: u32,
    pub FrameEngineId: u32,
}
pub const DEBUG_EVENT_CREATE_PROCESS: i32 = 16;
pub const DEBUG_EVENT_CREATE_THREAD: i32 = 4;
pub const DEBUG_EVENT_EXCEPTION: i32 = 2;
pub const DEBUG_EVENT_EXIT_PROCESS: i32 = 32;
pub const DEBUG_EVENT_EXIT_THREAD: i32 = 8;
pub const DEBUG_EVENT_LOAD_MODULE: i32 = 64;
pub const DEBUG_EVENT_SERVICE_EXCEPTION: i32 = 8192;
pub const DEBUG_EVENT_SESSION_STATUS: i32 = 512;
pub const DEBUG_EVENT_SYSTEM_ERROR: i32 = 256;
pub const DEBUG_EVENT_UNLOAD_MODULE: i32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_EXCEPTION_FILTER_PARAMETERS {
    pub ExecutionOption: u32,
    pub ContinueOption: u32,
    pub TextSize: u32,
    pub CommandSize: u32,
    pub SecondCommandSize: u32,
    pub ExceptionCode: u32,
}
pub const DEBUG_EXECUTE_DEFAULT: i32 = 0;
pub const DEBUG_EXECUTE_ECHO: i32 = 1;
pub const DEBUG_EXECUTE_EVENT: i32 = 2048;
pub const DEBUG_EXECUTE_EXTENSION: i32 = 32;
pub const DEBUG_EXECUTE_HOTKEY: i32 = 1024;
pub const DEBUG_EXECUTE_INTERNAL: i32 = 64;
pub const DEBUG_EXECUTE_MENU: i32 = 512;
pub const DEBUG_EXECUTE_NOT_LOGGED: i32 = 2;
pub const DEBUG_EXECUTE_NO_REPEAT: i32 = 4;
pub const DEBUG_EXECUTE_SCRIPT: i32 = 128;
pub const DEBUG_EXECUTE_TOOLBAR: i32 = 256;
pub const DEBUG_EXECUTE_USER_CLICKED: i32 = 16;
pub const DEBUG_EXECUTE_USER_TYPED: i32 = 8;
pub const DEBUG_EXEC_FLAGS_NONBLOCK: i32 = 1;
pub const DEBUG_EXPR_CPLUSPLUS: i32 = 1;
pub const DEBUG_EXPR_MASM: i32 = 0;
pub const DEBUG_EXTENSION_AT_ENGINE: i32 = 0;
pub const DEBUG_EXTENSION_CONTINUE_SEARCH: i32 = -805305743;
pub const DEBUG_EXTENSION_RELOAD_EXTENSION: i32 = -805306130;
pub const DEBUG_EXTINIT_HAS_COMMAND_HELP: i32 = 1;
pub const DEBUG_EXT_PVALUE_DEFAULT: i32 = 0;
pub const DEBUG_EXT_PVTYPE_IS_POINTER: i32 = 1;
pub const DEBUG_EXT_PVTYPE_IS_VALUE: i32 = 0;
pub const DEBUG_EXT_QVALUE_DEFAULT: i32 = 0;
pub const DEBUG_FILTER_BREAK: i32 = 0;
pub const DEBUG_FILTER_CREATE_PROCESS: i32 = 2;
pub const DEBUG_FILTER_CREATE_THREAD: i32 = 0;
pub const DEBUG_FILTER_DEBUGGEE_OUTPUT: i32 = 9;
pub const DEBUG_FILTER_EXIT_PROCESS: i32 = 3;
pub const DEBUG_FILTER_EXIT_THREAD: i32 = 1;
pub const DEBUG_FILTER_GO_HANDLED: i32 = 0;
pub const DEBUG_FILTER_GO_NOT_HANDLED: i32 = 1;
pub const DEBUG_FILTER_IGNORE: i32 = 3;
pub const DEBUG_FILTER_INITIAL_BREAKPOINT: i32 = 7;
pub const DEBUG_FILTER_INITIAL_MODULE_LOAD: i32 = 8;
pub const DEBUG_FILTER_LOAD_MODULE: i32 = 4;
pub const DEBUG_FILTER_OUTPUT: i32 = 2;
pub const DEBUG_FILTER_REMOVE: i32 = 4;
pub const DEBUG_FILTER_SECOND_CHANCE_BREAK: i32 = 1;
pub const DEBUG_FILTER_SYSTEM_ERROR: i32 = 6;
pub const DEBUG_FILTER_UNLOAD_MODULE: i32 = 5;
pub const DEBUG_FIND_SOURCE_BEST_MATCH: i32 = 2;
pub const DEBUG_FIND_SOURCE_DEFAULT: i32 = 0;
pub const DEBUG_FIND_SOURCE_FULL_PATH: i32 = 1;
pub const DEBUG_FIND_SOURCE_NO_SRCSRV: i32 = 4;
pub const DEBUG_FIND_SOURCE_TOKEN_LOOKUP: i32 = 8;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM: i32 = 16;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM_STRICT: i32 = 32;
pub const DEBUG_FORMAT_CAB_SECONDARY_ALL_IMAGES: i32 = 268435456;
pub const DEBUG_FORMAT_CAB_SECONDARY_FILES: i32 = 1073741824;
pub const DEBUG_FORMAT_DEFAULT: i32 = 0;
pub const DEBUG_FORMAT_NO_OVERWRITE: u32 = 2147483648;
pub const DEBUG_FORMAT_USER_SMALL_ADD_AVX_XSTATE_CONTEXT: i32 = 131072;
pub const DEBUG_FORMAT_USER_SMALL_CODE_SEGMENTS: i32 = 4096;
pub const DEBUG_FORMAT_USER_SMALL_DATA_SEGMENTS: i32 = 16;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_MEMORY: i32 = 32;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_PATHS: i32 = 64;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_TRIAGE: i32 = 65536;
pub const DEBUG_FORMAT_USER_SMALL_FULL_AUXILIARY_STATE: i32 = 16384;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY: i32 = 1;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY_INFO: i32 = 1024;
pub const DEBUG_FORMAT_USER_SMALL_HANDLE_DATA: i32 = 2;
pub const DEBUG_FORMAT_USER_SMALL_IGNORE_INACCESSIBLE_MEM: i32 = 134217728;
pub const DEBUG_FORMAT_USER_SMALL_INDIRECT_MEMORY: i32 = 8;
pub const DEBUG_FORMAT_USER_SMALL_IPT_TRACE: i32 = 262144;
pub const DEBUG_FORMAT_USER_SMALL_MODULE_HEADERS: i32 = 32768;
pub const DEBUG_FORMAT_USER_SMALL_NO_AUXILIARY_STATE: i32 = 8192;
pub const DEBUG_FORMAT_USER_SMALL_NO_IGNORE_INACCESSIBLE_MEM: i32 = 67108864;
pub const DEBUG_FORMAT_USER_SMALL_NO_OPTIONAL_DATA: i32 = 512;
pub const DEBUG_FORMAT_USER_SMALL_PRIVATE_READ_WRITE_MEMORY: i32 = 256;
pub const DEBUG_FORMAT_USER_SMALL_PROCESS_THREAD_DATA: i32 = 128;
pub const DEBUG_FORMAT_USER_SMALL_SCAN_PARTIAL_PAGES: i32 = 268435456;
pub const DEBUG_FORMAT_USER_SMALL_THREAD_INFO: i32 = 2048;
pub const DEBUG_FORMAT_USER_SMALL_UNLOADED_MODULES: i32 = 4;
pub const DEBUG_FORMAT_WRITE_CAB: i32 = 536870912;
pub const DEBUG_FRAME_DEFAULT: i32 = 0;
pub const DEBUG_FRAME_IGNORE_INLINE: i32 = 1;
pub const DEBUG_GETFNENT_DEFAULT: i32 = 0;
pub const DEBUG_GETFNENT_RAW_ENTRY_ONLY: i32 = 1;
pub const DEBUG_GETMOD_DEFAULT: i32 = 0;
pub const DEBUG_GETMOD_NO_LOADED_MODULES: i32 = 1;
pub const DEBUG_GETMOD_NO_UNLOADED_MODULES: i32 = 2;
pub const DEBUG_GET_PROC_DEFAULT: i32 = 0;
pub const DEBUG_GET_PROC_FULL_MATCH: i32 = 1;
pub const DEBUG_GET_PROC_ONLY_MATCH: i32 = 2;
pub const DEBUG_GET_PROC_SERVICE_NAME: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_GET_TEXT_COMPLETIONS_IN {
    pub Flags: u32,
    pub MatchCountLimit: u32,
    pub Reserved: [u64; 3],
}
impl Default for DEBUG_GET_TEXT_COMPLETIONS_IN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_DOT_COMMAND: i32 = 1;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_EXTENSION_COMMAND: i32 = 2;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_SYMBOL: i32 = 4;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_DOT_COMMANDS: i32 = 1;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_EXTENSION_COMMANDS: i32 = 2;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_SYMBOLS: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_GET_TEXT_COMPLETIONS_OUT {
    pub Flags: u32,
    pub ReplaceIndex: u32,
    pub MatchCount: u32,
    pub Reserved1: u32,
    pub Reserved2: [u64; 2],
}
impl Default for DEBUG_GET_TEXT_COMPLETIONS_OUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_GSEL_ALLOW_HIGHER: i32 = 4;
pub const DEBUG_GSEL_ALLOW_LOWER: i32 = 2;
pub const DEBUG_GSEL_DEFAULT: i32 = 0;
pub const DEBUG_GSEL_INLINE_CALLSITE: i32 = 16;
pub const DEBUG_GSEL_NEAREST_ONLY: i32 = 8;
pub const DEBUG_GSEL_NO_SYMBOL_LOADS: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_HANDLE_DATA_BASIC {
    pub TypeNameSize: u32,
    pub ObjectNameSize: u32,
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
}
pub const DEBUG_HANDLE_DATA_TYPE_ALL_HANDLE_OPERATIONS: i32 = 10;
pub const DEBUG_HANDLE_DATA_TYPE_BASIC: i32 = 0;
pub const DEBUG_HANDLE_DATA_TYPE_HANDLE_COUNT: i32 = 3;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_EVENT_1: i32 = 13;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_1: i32 = 7;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_2: i32 = 8;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_1: i32 = 11;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_2: i32 = 12;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SECTION_1: i32 = 14;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SEMAPHORE_1: i32 = 15;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_THREAD_1: i32 = 6;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME: i32 = 2;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME_WIDE: i32 = 5;
pub const DEBUG_HANDLE_DATA_TYPE_PER_HANDLE_OPERATIONS: i32 = 9;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME: i32 = 1;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME_WIDE: i32 = 4;
pub const DEBUG_INTERRUPT_ACTIVE: i32 = 0;
pub const DEBUG_INTERRUPT_EXIT: i32 = 2;
pub const DEBUG_INTERRUPT_PASSIVE: i32 = 1;
pub const DEBUG_INVALID_OFFSET: u64 = 18446744073709551615;
pub const DEBUG_IOUTPUT_ADDR_TRANSLATE: i32 = 134217728;
pub const DEBUG_IOUTPUT_BREAKPOINT: i32 = 536870912;
pub const DEBUG_IOUTPUT_EVENT: i32 = 268435456;
pub const DEBUG_IOUTPUT_KD_PROTOCOL: u32 = 2147483648;
pub const DEBUG_IOUTPUT_REMOTING: i32 = 1073741824;
pub const DEBUG_KERNEL_ACTIVE_DUMP: i32 = 1030;
pub const DEBUG_KERNEL_CONNECTION: i32 = 0;
pub const DEBUG_KERNEL_DUMP: i32 = 1025;
pub const DEBUG_KERNEL_EXDI_DRIVER: i32 = 2;
pub const DEBUG_KERNEL_FULL_DUMP: i32 = 1026;
pub const DEBUG_KERNEL_IDNA: i32 = 3;
pub const DEBUG_KERNEL_INSTALL_DRIVER: i32 = 4;
pub const DEBUG_KERNEL_LOCAL: i32 = 1;
pub const DEBUG_KERNEL_REPT: i32 = 5;
pub const DEBUG_KERNEL_SMALL_DUMP: i32 = 1024;
pub const DEBUG_KERNEL_TRACE_LOG: i32 = 1028;
pub const DEBUG_KNOWN_STRUCT_GET_NAMES: i32 = 1;
pub const DEBUG_KNOWN_STRUCT_GET_SINGLE_LINE_OUTPUT: i32 = 2;
pub const DEBUG_KNOWN_STRUCT_SUPPRESS_TYPE_NAME: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_BREAKPOINT {
    pub Id: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_EXCEPTION {
    pub ExceptionRecord: super::EXCEPTION_RECORD64,
    pub FirstChance: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_EXIT_PROCESS {
    pub ExitCode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_EXIT_THREAD {
    pub ExitCode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_LOAD_MODULE {
    pub Base: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION {
    pub Kind: u32,
    pub DataSize: u32,
    pub Address: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR {
    pub Error: u32,
    pub Level: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE {
    pub Base: u64,
}
pub const DEBUG_LEVEL_ASSEMBLY: i32 = 1;
pub const DEBUG_LEVEL_SOURCE: i32 = 0;
pub const DEBUG_LIVE_USER_NON_INVASIVE: i32 = 33;
pub const DEBUG_LOG_APPEND: i32 = 1;
pub const DEBUG_LOG_DEFAULT: i32 = 0;
pub const DEBUG_LOG_DML: i32 = 4;
pub const DEBUG_LOG_UNICODE: i32 = 2;
pub const DEBUG_MANAGED_ALLOWED: i32 = 1;
pub const DEBUG_MANAGED_DISABLED: i32 = 0;
pub const DEBUG_MANAGED_DLL_LOADED: i32 = 2;
pub const DEBUG_MANRESET_DEFAULT: i32 = 0;
pub const DEBUG_MANRESET_LOAD_DLL: i32 = 1;
pub const DEBUG_MANSTR_LOADED_SUPPORT_DLL: i32 = 1;
pub const DEBUG_MANSTR_LOAD_STATUS: i32 = 2;
pub const DEBUG_MANSTR_NONE: i32 = 0;
pub const DEBUG_MODNAME_IMAGE: i32 = 0;
pub const DEBUG_MODNAME_LOADED_IMAGE: i32 = 2;
pub const DEBUG_MODNAME_MAPPED_IMAGE: i32 = 4;
pub const DEBUG_MODNAME_MODULE: i32 = 1;
pub const DEBUG_MODNAME_SYMBOL_FILE: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_MODULE_AND_ID {
    pub ModuleBase: u64,
    pub Id: u64,
}
pub const DEBUG_MODULE_EXE_MODULE: i32 = 4;
pub const DEBUG_MODULE_EXPLICIT: i32 = 8;
pub const DEBUG_MODULE_LOADED: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_MODULE_PARAMETERS {
    pub Base: u64,
    pub Size: u32,
    pub TimeDateStamp: u32,
    pub Checksum: u32,
    pub Flags: u32,
    pub SymbolType: u32,
    pub ImageNameSize: u32,
    pub ModuleNameSize: u32,
    pub LoadedImageNameSize: u32,
    pub SymbolFileNameSize: u32,
    pub MappedImageNameSize: u32,
    pub Reserved: [u64; 2],
}
impl Default for DEBUG_MODULE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_MODULE_SECONDARY: i32 = 16;
pub const DEBUG_MODULE_SYM_BAD_CHECKSUM: i32 = 65536;
pub const DEBUG_MODULE_SYNTHETIC: i32 = 32;
pub const DEBUG_MODULE_UNLOADED: i32 = 1;
pub const DEBUG_MODULE_USER_MODE: i32 = 2;
pub const DEBUG_NOTIFY_SESSION_ACCESSIBLE: i32 = 2;
pub const DEBUG_NOTIFY_SESSION_ACTIVE: i32 = 0;
pub const DEBUG_NOTIFY_SESSION_INACCESSIBLE: i32 = 3;
pub const DEBUG_NOTIFY_SESSION_INACTIVE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_OFFSET_REGION {
    pub Base: u64,
    pub Size: u64,
}
pub const DEBUG_OFFSINFO_VIRTUAL_SOURCE: i32 = 1;
pub const DEBUG_OUTCBF_COMBINED_EXPLICIT_FLUSH: i32 = 1;
pub const DEBUG_OUTCBF_DML_HAS_SPECIAL_CHARACTERS: i32 = 4;
pub const DEBUG_OUTCBF_DML_HAS_TAGS: i32 = 2;
pub const DEBUG_OUTCBI_ANY_FORMAT: i32 = 6;
pub const DEBUG_OUTCBI_DML: i32 = 4;
pub const DEBUG_OUTCBI_EXPLICIT_FLUSH: i32 = 1;
pub const DEBUG_OUTCBI_TEXT: i32 = 2;
pub const DEBUG_OUTCB_DML: i32 = 1;
pub const DEBUG_OUTCB_EXPLICIT_FLUSH: i32 = 2;
pub const DEBUG_OUTCB_TEXT: i32 = 0;
pub const DEBUG_OUTCTL_ALL_CLIENTS: i32 = 1;
pub const DEBUG_OUTCTL_ALL_OTHER_CLIENTS: i32 = 2;
pub const DEBUG_OUTCTL_AMBIENT: u32 = 4294967295;
pub const DEBUG_OUTCTL_AMBIENT_DML: u32 = 4294967294;
pub const DEBUG_OUTCTL_AMBIENT_TEXT: u32 = 4294967295;
pub const DEBUG_OUTCTL_DML: i32 = 32;
pub const DEBUG_OUTCTL_IGNORE: i32 = 3;
pub const DEBUG_OUTCTL_LOG_ONLY: i32 = 4;
pub const DEBUG_OUTCTL_NOT_LOGGED: i32 = 8;
pub const DEBUG_OUTCTL_OVERRIDE_MASK: i32 = 16;
pub const DEBUG_OUTCTL_SEND_MASK: i32 = 7;
pub const DEBUG_OUTCTL_THIS_CLIENT: i32 = 0;
pub const DEBUG_OUTPUT_DEBUGGEE: i32 = 128;
pub const DEBUG_OUTPUT_DEBUGGEE_PROMPT: i32 = 256;
pub const DEBUG_OUTPUT_ERROR: i32 = 2;
pub const DEBUG_OUTPUT_EXTENSION_WARNING: i32 = 64;
pub const DEBUG_OUTPUT_IDENTITY_DEFAULT: i32 = 0;
pub const DEBUG_OUTPUT_NAME_END: windows_sys::core::PCSTR = windows_sys::core::s!("**NAME**");
pub const DEBUG_OUTPUT_NAME_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**NAME**");
pub const DEBUG_OUTPUT_NORMAL: i32 = 1;
pub const DEBUG_OUTPUT_OFFSET_END: windows_sys::core::PCSTR = windows_sys::core::s!("**OFF**");
pub const DEBUG_OUTPUT_OFFSET_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**OFF**");
pub const DEBUG_OUTPUT_PROMPT: i32 = 16;
pub const DEBUG_OUTPUT_PROMPT_REGISTERS: i32 = 32;
pub const DEBUG_OUTPUT_STATUS: i32 = 1024;
pub const DEBUG_OUTPUT_SYMBOLS: i32 = 512;
pub const DEBUG_OUTPUT_SYMBOLS_DEFAULT: i32 = 0;
pub const DEBUG_OUTPUT_SYMBOLS_NO_NAMES: i32 = 1;
pub const DEBUG_OUTPUT_SYMBOLS_NO_OFFSETS: i32 = 2;
pub const DEBUG_OUTPUT_SYMBOLS_NO_TYPES: i32 = 16;
pub const DEBUG_OUTPUT_SYMBOLS_NO_VALUES: i32 = 4;
pub const DEBUG_OUTPUT_TYPE_END: windows_sys::core::PCSTR = windows_sys::core::s!("**TYPE**");
pub const DEBUG_OUTPUT_TYPE_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**TYPE**");
pub const DEBUG_OUTPUT_VALUE_END: windows_sys::core::PCSTR = windows_sys::core::s!("**VALUE**");
pub const DEBUG_OUTPUT_VALUE_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**VALUE**");
pub const DEBUG_OUTPUT_VERBOSE: i32 = 8;
pub const DEBUG_OUTPUT_WARNING: i32 = 4;
pub const DEBUG_OUTPUT_XML: i32 = 2048;
pub const DEBUG_OUTSYM_ALLOW_DISPLACEMENT: i32 = 4;
pub const DEBUG_OUTSYM_DEFAULT: i32 = 0;
pub const DEBUG_OUTSYM_FORCE_OFFSET: i32 = 1;
pub const DEBUG_OUTSYM_SOURCE_LINE: i32 = 2;
pub const DEBUG_OUTTYPE_ADDRESS_AT_END: i32 = 131072;
pub const DEBUG_OUTTYPE_ADDRESS_OF_FIELD: i32 = 65536;
pub const DEBUG_OUTTYPE_BLOCK_RECURSE: i32 = 2097152;
pub const DEBUG_OUTTYPE_COMPACT_OUTPUT: i32 = 8;
pub const DEBUG_OUTTYPE_DEFAULT: i32 = 0;
pub const DEBUG_OUTTYPE_NO_INDENT: i32 = 1;
pub const DEBUG_OUTTYPE_NO_OFFSET: i32 = 2;
pub const DEBUG_OUTTYPE_VERBOSE: i32 = 4;
pub const DEBUG_OUT_TEXT_REPL_DEFAULT: i32 = 0;
pub const DEBUG_PHYSICAL_CACHED: i32 = 1;
pub const DEBUG_PHYSICAL_DEFAULT: i32 = 0;
pub const DEBUG_PHYSICAL_UNCACHED: i32 = 2;
pub const DEBUG_PHYSICAL_WRITE_COMBINED: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUG_PROCESSOR_IDENTIFICATION_ALL {
    pub Alpha: DEBUG_PROCESSOR_IDENTIFICATION_ALPHA,
    pub Amd64: DEBUG_PROCESSOR_IDENTIFICATION_AMD64,
    pub Ia64: DEBUG_PROCESSOR_IDENTIFICATION_IA64,
    pub X86: DEBUG_PROCESSOR_IDENTIFICATION_X86,
    pub Arm: DEBUG_PROCESSOR_IDENTIFICATION_ARM,
    pub Arm64: DEBUG_PROCESSOR_IDENTIFICATION_ARM64,
}
impl Default for DEBUG_PROCESSOR_IDENTIFICATION_ALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ALPHA {
    pub Type: u32,
    pub Revision: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {
    pub Family: u32,
    pub Model: u32,
    pub Stepping: u32,
    pub VendorString: [i8; 16],
}
impl Default for DEBUG_PROCESSOR_IDENTIFICATION_AMD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ARM {
    pub Model: u32,
    pub Revision: u32,
    pub VendorString: [i8; 16],
}
impl Default for DEBUG_PROCESSOR_IDENTIFICATION_ARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {
    pub Model: u32,
    pub Revision: u32,
    pub VendorString: [i8; 16],
}
impl Default for DEBUG_PROCESSOR_IDENTIFICATION_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_IA64 {
    pub Model: u32,
    pub Revision: u32,
    pub Family: u32,
    pub ArchRev: u32,
    pub VendorString: [i8; 16],
}
impl Default for DEBUG_PROCESSOR_IDENTIFICATION_IA64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_X86 {
    pub Family: u32,
    pub Model: u32,
    pub Stepping: u32,
    pub VendorString: [i8; 16],
}
impl Default for DEBUG_PROCESSOR_IDENTIFICATION_X86 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_PROCESS_DETACH_ON_EXIT: i32 = 1;
pub const DEBUG_PROCESS_ONLY_THIS_PROCESS: i32 = 2;
pub const DEBUG_PROC_DESC_DEFAULT: i32 = 0;
pub const DEBUG_PROC_DESC_NO_COMMAND_LINE: i32 = 8;
pub const DEBUG_PROC_DESC_NO_MTS_PACKAGES: i32 = 4;
pub const DEBUG_PROC_DESC_NO_PATHS: i32 = 1;
pub const DEBUG_PROC_DESC_NO_SERVICES: i32 = 2;
pub const DEBUG_PROC_DESC_NO_SESSION_ID: i32 = 16;
pub const DEBUG_PROC_DESC_NO_USER_NAME: i32 = 32;
pub const DEBUG_PROC_DESC_WITH_ARCHITECTURE: i32 = 128;
pub const DEBUG_PROC_DESC_WITH_PACKAGEFAMILY: i32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_READ_USER_MINIDUMP_STREAM {
    pub StreamType: u32,
    pub Flags: u32,
    pub Offset: u64,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub BufferUsed: u32,
}
impl Default for DEBUG_READ_USER_MINIDUMP_STREAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_REGISTERS_ALL: i32 = 7;
pub const DEBUG_REGISTERS_DEFAULT: i32 = 0;
pub const DEBUG_REGISTERS_FLOAT: i32 = 4;
pub const DEBUG_REGISTERS_INT32: i32 = 1;
pub const DEBUG_REGISTERS_INT64: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_REGISTER_DESCRIPTION {
    pub Type: u32,
    pub Flags: u32,
    pub SubregMaster: u32,
    pub SubregLength: u32,
    pub SubregMask: u64,
    pub SubregShift: u32,
    pub Reserved0: u32,
}
pub const DEBUG_REGISTER_SUB_REGISTER: i32 = 1;
pub const DEBUG_REGSRC_DEBUGGEE: i32 = 0;
pub const DEBUG_REGSRC_EXPLICIT: i32 = 1;
pub const DEBUG_REGSRC_FRAME: i32 = 2;
pub const DEBUG_REQUEST_ADD_CACHED_SYMBOL_INFO: i32 = 16;
pub const DEBUG_REQUEST_CLOSE_TOKEN: i32 = 30;
pub const DEBUG_REQUEST_CURRENT_OUTPUT_CALLBACKS_ARE_DML_AWARE: i32 = 19;
pub const DEBUG_REQUEST_DUPLICATE_TOKEN: i32 = 28;
pub const DEBUG_REQUEST_EXT_TYPED_DATA_ANSI: i32 = 12;
pub const DEBUG_REQUEST_GET_ADDITIONAL_CREATE_OPTIONS: i32 = 4;
pub const DEBUG_REQUEST_GET_CACHED_SYMBOL_INFO: i32 = 15;
pub const DEBUG_REQUEST_GET_CAPTURED_EVENT_CODE_OFFSET: i32 = 10;
pub const DEBUG_REQUEST_GET_DUMP_HEADER: i32 = 21;
pub const DEBUG_REQUEST_GET_EXTENSION_SEARCH_PATH_WIDE: i32 = 13;
pub const DEBUG_REQUEST_GET_IMAGE_ARCHITECTURE: i32 = 39;
pub const DEBUG_REQUEST_GET_INSTRUMENTATION_VERSION: i32 = 37;
pub const DEBUG_REQUEST_GET_MODULE_ARCHITECTURE: i32 = 38;
pub const DEBUG_REQUEST_GET_OFFSET_UNWIND_INFORMATION: i32 = 20;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_ANSI: i32 = 18;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_WIDE: i32 = 14;
pub const DEBUG_REQUEST_GET_WIN32_MAJOR_MINOR_VERSIONS: i32 = 6;
pub const DEBUG_REQUEST_INLINE_QUERY: i32 = 35;
pub const DEBUG_REQUEST_MIDORI: i32 = 23;
pub const DEBUG_REQUEST_MISC_INFORMATION: i32 = 25;
pub const DEBUG_REQUEST_OPEN_PROCESS_TOKEN: i32 = 26;
pub const DEBUG_REQUEST_OPEN_THREAD_TOKEN: i32 = 27;
pub const DEBUG_REQUEST_PROCESS_DESCRIPTORS: i32 = 24;
pub const DEBUG_REQUEST_QUERY_INFO_TOKEN: i32 = 29;
pub const DEBUG_REQUEST_READ_CAPTURED_EVENT_CODE_STREAM: i32 = 11;
pub const DEBUG_REQUEST_READ_USER_MINIDUMP_STREAM: i32 = 7;
pub const DEBUG_REQUEST_REMOVE_CACHED_SYMBOL_INFO: i32 = 17;
pub const DEBUG_REQUEST_RESUME_THREAD: i32 = 34;
pub const DEBUG_REQUEST_SET_ADDITIONAL_CREATE_OPTIONS: i32 = 5;
pub const DEBUG_REQUEST_SET_DUMP_HEADER: i32 = 22;
pub const DEBUG_REQUEST_SET_LOCAL_IMPLICIT_COMMAND_LINE: i32 = 9;
pub const DEBUG_REQUEST_SET_PARENT_HWND: i32 = 40;
pub const DEBUG_REQUEST_SOURCE_PATH_HAS_SOURCE_SERVER: i32 = 0;
pub const DEBUG_REQUEST_TARGET_CAN_DETACH: i32 = 8;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_CONTEXT: i32 = 1;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_RECORD: i32 = 3;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_THREAD: i32 = 2;
pub const DEBUG_REQUEST_TL_INSTRUMENTATION_AWARE: i32 = 36;
pub const DEBUG_REQUEST_WOW_MODULE: i32 = 32;
pub const DEBUG_REQUEST_WOW_PROCESS: i32 = 31;
pub const DEBUG_SCOPE_GROUP_ALL: i32 = 3;
pub const DEBUG_SCOPE_GROUP_ARGUMENTS: i32 = 1;
pub const DEBUG_SCOPE_GROUP_BY_DATAMODEL: i32 = 4;
pub const DEBUG_SCOPE_GROUP_LOCALS: i32 = 2;
pub const DEBUG_SCOPE_GROUP_VALID_FLAGS: i32 = 7;
pub const DEBUG_SERVERS_ALL: i32 = 3;
pub const DEBUG_SERVERS_DEBUGGER: i32 = 1;
pub const DEBUG_SERVERS_PROCESS: i32 = 2;
pub const DEBUG_SESSION_ACTIVE: i32 = 0;
pub const DEBUG_SESSION_END: i32 = 4;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_DETACH: i32 = 2;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_TERMINATE: i32 = 1;
pub const DEBUG_SESSION_END_SESSION_PASSIVE: i32 = 3;
pub const DEBUG_SESSION_FAILURE: i32 = 7;
pub const DEBUG_SESSION_HIBERNATE: i32 = 6;
pub const DEBUG_SESSION_REBOOT: i32 = 5;
pub const DEBUG_SOURCE_IS_STATEMENT: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_SPECIFIC_FILTER_PARAMETERS {
    pub ExecutionOption: u32,
    pub ContinueOption: u32,
    pub TextSize: u32,
    pub CommandSize: u32,
    pub ArgumentSize: u32,
}
pub const DEBUG_SRCFILE_SYMBOL_CHECKSUMINFO: i32 = 2;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN: i32 = 0;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN_SOURCE_COMMAND_WIDE: i32 = 1;
pub const DEBUG_STACK_ARGUMENTS: i32 = 1;
pub const DEBUG_STACK_COLUMN_NAMES: i32 = 16;
pub const DEBUG_STACK_DML: i32 = 2048;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_STACK_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
    pub FuncTableEntry: u64,
    pub Params: [u64; 4],
    pub Reserved: [u64; 6],
    pub Virtual: windows_sys::core::BOOL,
    pub FrameNumber: u32,
}
impl Default for DEBUG_STACK_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_STACK_FRAME_ADDRESSES: i32 = 8;
pub const DEBUG_STACK_FRAME_ADDRESSES_RA_ONLY: i32 = 256;
pub const DEBUG_STACK_FRAME_ARCH: i32 = 16384;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_STACK_FRAME_EX {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
    pub FuncTableEntry: u64,
    pub Params: [u64; 4],
    pub Reserved: [u64; 6],
    pub Virtual: windows_sys::core::BOOL,
    pub FrameNumber: u32,
    pub InlineFrameContext: u32,
    pub FrameMachine: u32,
}
impl Default for DEBUG_STACK_FRAME_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEBUG_STACK_FRAME_MEMORY_USAGE: i32 = 512;
pub const DEBUG_STACK_FRAME_NUMBERS: i32 = 64;
pub const DEBUG_STACK_FRAME_OFFSETS: i32 = 4096;
pub const DEBUG_STACK_FUNCTION_INFO: i32 = 2;
pub const DEBUG_STACK_NONVOLATILE_REGISTERS: i32 = 32;
pub const DEBUG_STACK_PARAMETERS: i32 = 128;
pub const DEBUG_STACK_PARAMETERS_NEWLINE: i32 = 1024;
pub const DEBUG_STACK_PROVIDER: i32 = 8192;
pub const DEBUG_STACK_SOURCE_LINE: i32 = 4;
pub const DEBUG_STATUS_BREAK: i32 = 6;
pub const DEBUG_STATUS_GO: i32 = 1;
pub const DEBUG_STATUS_GO_HANDLED: i32 = 2;
pub const DEBUG_STATUS_GO_NOT_HANDLED: i32 = 3;
pub const DEBUG_STATUS_IGNORE_EVENT: i32 = 9;
pub const DEBUG_STATUS_INSIDE_WAIT: i64 = 4294967296;
pub const DEBUG_STATUS_MASK: i32 = 31;
pub const DEBUG_STATUS_NO_CHANGE: i32 = 0;
pub const DEBUG_STATUS_NO_DEBUGGEE: i32 = 7;
pub const DEBUG_STATUS_OUT_OF_SYNC: i32 = 15;
pub const DEBUG_STATUS_RESTART_REQUESTED: i32 = 10;
pub const DEBUG_STATUS_REVERSE_GO: i32 = 11;
pub const DEBUG_STATUS_REVERSE_STEP_BRANCH: i32 = 12;
pub const DEBUG_STATUS_REVERSE_STEP_INTO: i32 = 14;
pub const DEBUG_STATUS_REVERSE_STEP_OVER: i32 = 13;
pub const DEBUG_STATUS_STEP_BRANCH: i32 = 8;
pub const DEBUG_STATUS_STEP_INTO: i32 = 5;
pub const DEBUG_STATUS_STEP_OVER: i32 = 4;
pub const DEBUG_STATUS_TIMEOUT: i32 = 17;
pub const DEBUG_STATUS_WAIT_INPUT: i32 = 16;
pub const DEBUG_STATUS_WAIT_TIMEOUT: i64 = 8589934592;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_SYMBOL_ENTRY {
    pub ModuleBase: u64,
    pub Offset: u64,
    pub Id: u64,
    pub Arg64: u64,
    pub Size: u32,
    pub Flags: u32,
    pub TypeId: u32,
    pub NameSize: u32,
    pub Token: u32,
    pub Tag: u32,
    pub Arg32: u32,
    pub Reserved: u32,
}
pub const DEBUG_SYMBOL_EXPANDED: i32 = 16;
pub const DEBUG_SYMBOL_EXPANSION_LEVEL_MASK: i32 = 15;
pub const DEBUG_SYMBOL_IS_ARGUMENT: i32 = 256;
pub const DEBUG_SYMBOL_IS_ARRAY: i32 = 64;
pub const DEBUG_SYMBOL_IS_FLOAT: i32 = 128;
pub const DEBUG_SYMBOL_IS_LOCAL: i32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_SYMBOL_PARAMETERS {
    pub Module: u64,
    pub TypeId: u32,
    pub ParentSymbol: u32,
    pub SubElements: u32,
    pub Flags: u32,
    pub Reserved: u64,
}
pub const DEBUG_SYMBOL_READ_ONLY: i32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_SYMBOL_SOURCE_ENTRY {
    pub ModuleBase: u64,
    pub Offset: u64,
    pub FileNameId: u64,
    pub EngineInternal: u64,
    pub Size: u32,
    pub Flags: u32,
    pub FileNameSize: u32,
    pub StartLine: u32,
    pub EndLine: u32,
    pub StartColumn: u32,
    pub EndColumn: u32,
    pub Reserved: u32,
}
pub const DEBUG_SYMENT_IS_CODE: i32 = 1;
pub const DEBUG_SYMENT_IS_DATA: i32 = 2;
pub const DEBUG_SYMENT_IS_LOCAL: i32 = 8;
pub const DEBUG_SYMENT_IS_MANAGED: i32 = 16;
pub const DEBUG_SYMENT_IS_PARAMETER: i32 = 4;
pub const DEBUG_SYMENT_IS_SYNTHETIC: i32 = 32;
pub const DEBUG_SYMINFO_BREAKPOINT_SOURCE_LINE: i32 = 0;
pub const DEBUG_SYMINFO_GET_MODULE_SYMBOL_NAMES_AND_OFFSETS: i32 = 3;
pub const DEBUG_SYMINFO_GET_SYMBOL_NAME_BY_OFFSET_AND_TAG_WIDE: i32 = 2;
pub const DEBUG_SYMINFO_IMAGEHLP_MODULEW64: i32 = 1;
pub const DEBUG_SYMTYPE_CODEVIEW: i32 = 2;
pub const DEBUG_SYMTYPE_COFF: i32 = 1;
pub const DEBUG_SYMTYPE_DEFERRED: i32 = 5;
pub const DEBUG_SYMTYPE_DIA: i32 = 7;
pub const DEBUG_SYMTYPE_EXPORT: i32 = 4;
pub const DEBUG_SYMTYPE_NONE: i32 = 0;
pub const DEBUG_SYMTYPE_PDB: i32 = 3;
pub const DEBUG_SYMTYPE_SYM: i32 = 6;
pub const DEBUG_SYSOBJINFO_CURRENT_PROCESS_COOKIE: i32 = 2;
pub const DEBUG_SYSOBJINFO_THREAD_BASIC_INFORMATION: i32 = 0;
pub const DEBUG_SYSOBJINFO_THREAD_NAME_WIDE: i32 = 1;
pub const DEBUG_SYSVERSTR_BUILD: i32 = 1;
pub const DEBUG_SYSVERSTR_SERVICE_PACK: i32 = 0;
pub const DEBUG_TBINFO_AFFINITY: i32 = 32;
pub const DEBUG_TBINFO_ALL: i32 = 63;
pub const DEBUG_TBINFO_EXIT_STATUS: i32 = 1;
pub const DEBUG_TBINFO_PRIORITY: i32 = 4;
pub const DEBUG_TBINFO_PRIORITY_CLASS: i32 = 2;
pub const DEBUG_TBINFO_START_OFFSET: i32 = 16;
pub const DEBUG_TBINFO_TIMES: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_THREAD_BASIC_INFORMATION {
    pub Valid: u32,
    pub ExitStatus: u32,
    pub PriorityClass: u32,
    pub Priority: u32,
    pub CreateTime: u64,
    pub ExitTime: u64,
    pub KernelTime: u64,
    pub UserTime: u64,
    pub StartOffset: u64,
    pub Affinity: u64,
}
pub const DEBUG_TYPEOPTS_FORCERADIX_OUTPUT: i32 = 4;
pub const DEBUG_TYPEOPTS_LONGSTATUS_DISPLAY: i32 = 2;
pub const DEBUG_TYPEOPTS_MATCH_MAXSIZE: i32 = 8;
pub const DEBUG_TYPEOPTS_UNICODE_DISPLAY: i32 = 1;
pub const DEBUG_USER_WINDOWS_DUMP: i32 = 1025;
pub const DEBUG_USER_WINDOWS_DUMP_WINDOWS_CE: i32 = 1029;
pub const DEBUG_USER_WINDOWS_IDNA: i32 = 2;
pub const DEBUG_USER_WINDOWS_PROCESS: i32 = 0;
pub const DEBUG_USER_WINDOWS_PROCESS_SERVER: i32 = 1;
pub const DEBUG_USER_WINDOWS_REPT: i32 = 3;
pub const DEBUG_USER_WINDOWS_SMALL_DUMP: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_VALUE {
    pub Anonymous: DEBUG_VALUE_0,
    pub TailOfRawBytes: u32,
    pub Type: u32,
}
impl Default for DEBUG_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUG_VALUE_0 {
    pub I8: u8,
    pub I16: u16,
    pub I32: u32,
    pub Anonymous: DEBUG_VALUE_0_0,
    pub F32: f32,
    pub F64: f64,
    pub F80Bytes: [u8; 10],
    pub F82Bytes: [u8; 11],
    pub F128Bytes: [u8; 16],
    pub VI8: [u8; 16],
    pub VI16: [u16; 8],
    pub VI32: [u32; 4],
    pub VI64: [u64; 2],
    pub VF32: [f32; 4],
    pub VF64: [f64; 2],
    pub I64Parts32: DEBUG_VALUE_0_1,
    pub F128Parts64: DEBUG_VALUE_0_2,
    pub RawBytes: [u8; 24],
}
impl Default for DEBUG_VALUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_VALUE_0_0 {
    pub I64: u64,
    pub Nat: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_VALUE_0_1 {
    pub LowPart: u32,
    pub HighPart: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_VALUE_0_2 {
    pub LowPart: u64,
    pub HighPart: i64,
}
pub const DEBUG_VALUE_FLOAT128: i32 = 9;
pub const DEBUG_VALUE_FLOAT32: i32 = 5;
pub const DEBUG_VALUE_FLOAT64: i32 = 6;
pub const DEBUG_VALUE_FLOAT80: i32 = 7;
pub const DEBUG_VALUE_FLOAT82: i32 = 8;
pub const DEBUG_VALUE_INT16: i32 = 2;
pub const DEBUG_VALUE_INT32: i32 = 3;
pub const DEBUG_VALUE_INT64: i32 = 4;
pub const DEBUG_VALUE_INT8: i32 = 1;
pub const DEBUG_VALUE_INVALID: i32 = 0;
pub const DEBUG_VALUE_TYPES: i32 = 12;
pub const DEBUG_VALUE_VECTOR128: i32 = 11;
pub const DEBUG_VALUE_VECTOR64: i32 = 10;
pub const DEBUG_VSEARCH_DEFAULT: i32 = 0;
pub const DEBUG_VSEARCH_WRITABLE_ONLY: i32 = 1;
pub const DEBUG_VSOURCE_DEBUGGEE: i32 = 1;
pub const DEBUG_VSOURCE_DUMP_WITHOUT_MEMINFO: i32 = 3;
pub const DEBUG_VSOURCE_INVALID: i32 = 0;
pub const DEBUG_VSOURCE_MAPPED_IMAGE: i32 = 2;
pub const DEBUG_WAIT_DEFAULT: i32 = 0;
pub const ERROR_DBG_CANCELLED: u32 = 3221226695;
pub const ERROR_DBG_TIMEOUT: u32 = 3221226932;
pub const IMAGE_FILE_MACHINE_ARM64EC: i32 = 42561;
pub const IMAGE_FILE_MACHINE_ARM64X: i32 = 42574;
pub const IMAGE_FILE_MACHINE_CHPE_X86: i32 = 14948;
#[repr(C)]
#[derive(Clone, Copy)]
pub union INLINE_FRAME_CONTEXT {
    pub ContextValue: u32,
    pub Anonymous: INLINE_FRAME_CONTEXT_0,
}
impl Default for INLINE_FRAME_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INLINE_FRAME_CONTEXT_0 {
    pub FrameId: u8,
    pub FrameType: u8,
    pub FrameSignature: u16,
}
pub const MODULE_ORDERS_LOADTIME: i32 = 268435456;
pub const MODULE_ORDERS_MASK: u32 = 4026531840;
pub const MODULE_ORDERS_MODULENAME: i32 = 536870912;
pub type PDEBUG_BREAKPOINT_PARAMETERS = *mut DEBUG_BREAKPOINT_PARAMETERS;
pub type PDEBUG_CACHED_SYMBOL_INFO = *mut DEBUG_CACHED_SYMBOL_INFO;
pub type PDEBUG_CLIENT_CONTEXT = *mut DEBUG_CLIENT_CONTEXT;
pub type PDEBUG_CREATE_PROCESS_OPTIONS = *mut DEBUG_CREATE_PROCESS_OPTIONS;
pub type PDEBUG_ENTENSION_KNOWNSTRUCT = PDEBUG_EXTENSION_KNOWN_STRUCT;
pub type PDEBUG_EVENT_CONTEXT = *mut DEBUG_EVENT_CONTEXT;
pub type PDEBUG_EXCEPTION_FILTER_PARAMETERS = *mut DEBUG_EXCEPTION_FILTER_PARAMETERS;
pub type PDEBUG_EXTENSION_CALL = Option<unsafe extern "system" fn(client: *mut core::ffi::c_void, args: windows_sys::core::PCSTR) -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_CANUNLOAD = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_INITIALIZE = Option<unsafe extern "system" fn(version: *mut u32, flags: *mut u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_ISEXTENSIONAPI = Option<unsafe extern "system" fn(name: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PDEBUG_EXTENSION_KNOWN_STRUCT = Option<unsafe extern "system" fn(flags: u32, offset: u64, typename: windows_sys::core::PCSTR, buffer: windows_sys::core::PSTR, bufferchars: *mut u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_KNOWN_STRUCT_EX = Option<unsafe extern "system" fn(client: *mut core::ffi::c_void, flags: u32, offset: u64, typename: windows_sys::core::PCSTR, buffer: windows_sys::core::PSTR, bufferchars: *mut u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_NOTIFY = Option<unsafe extern "system" fn(notify: u32, argument: u64)>;
pub type PDEBUG_EXTENSION_PROVIDE_VALUE = Option<unsafe extern "system" fn(client: *mut core::ffi::c_void, flags: u32, name: windows_sys::core::PCWSTR, value: *mut u64, typemodbase: *mut u64, typeid: *mut u32, typeflags: *mut u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_QUERY_VALUE_NAMES = Option<unsafe extern "system" fn(client: *mut core::ffi::c_void, flags: u32, buffer: windows_sys::core::PWSTR, bufferchars: u32, bufferneeded: *mut u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_EXTENSION_UNINITIALIZE = Option<unsafe extern "system" fn()>;
pub type PDEBUG_EXTENSION_UNLOAD = Option<unsafe extern "system" fn()>;
pub type PDEBUG_GET_TEXT_COMPLETIONS_IN = *mut DEBUG_GET_TEXT_COMPLETIONS_IN;
pub type PDEBUG_GET_TEXT_COMPLETIONS_OUT = *mut DEBUG_GET_TEXT_COMPLETIONS_OUT;
pub type PDEBUG_HANDLE_DATA_BASIC = *mut DEBUG_HANDLE_DATA_BASIC;
pub type PDEBUG_LAST_EVENT_INFO_BREAKPOINT = *mut DEBUG_LAST_EVENT_INFO_BREAKPOINT;
#[cfg(feature = "winnt")]
pub type PDEBUG_LAST_EVENT_INFO_EXCEPTION = *mut DEBUG_LAST_EVENT_INFO_EXCEPTION;
pub type PDEBUG_LAST_EVENT_INFO_EXIT_PROCESS = *mut DEBUG_LAST_EVENT_INFO_EXIT_PROCESS;
pub type PDEBUG_LAST_EVENT_INFO_EXIT_THREAD = *mut DEBUG_LAST_EVENT_INFO_EXIT_THREAD;
pub type PDEBUG_LAST_EVENT_INFO_LOAD_MODULE = *mut DEBUG_LAST_EVENT_INFO_LOAD_MODULE;
pub type PDEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION = *mut DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION;
pub type PDEBUG_LAST_EVENT_INFO_SYSTEM_ERROR = *mut DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR;
pub type PDEBUG_LAST_EVENT_INFO_UNLOAD_MODULE = *mut DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE;
pub type PDEBUG_MODULE_AND_ID = *mut DEBUG_MODULE_AND_ID;
pub type PDEBUG_MODULE_PARAMETERS = *mut DEBUG_MODULE_PARAMETERS;
pub type PDEBUG_OFFSET_REGION = *mut DEBUG_OFFSET_REGION;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ALL = *mut DEBUG_PROCESSOR_IDENTIFICATION_ALL;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ALPHA = *mut DEBUG_PROCESSOR_IDENTIFICATION_ALPHA;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_AMD64 = *mut DEBUG_PROCESSOR_IDENTIFICATION_AMD64;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ARM = *mut DEBUG_PROCESSOR_IDENTIFICATION_ARM;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_ARM64 = *mut DEBUG_PROCESSOR_IDENTIFICATION_ARM64;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_IA64 = *mut DEBUG_PROCESSOR_IDENTIFICATION_IA64;
pub type PDEBUG_PROCESSOR_IDENTIFICATION_X86 = *mut DEBUG_PROCESSOR_IDENTIFICATION_X86;
pub type PDEBUG_READ_USER_MINIDUMP_STREAM = *mut DEBUG_READ_USER_MINIDUMP_STREAM;
pub type PDEBUG_REGISTER_DESCRIPTION = *mut DEBUG_REGISTER_DESCRIPTION;
pub type PDEBUG_SPECIFIC_FILTER_PARAMETERS = *mut DEBUG_SPECIFIC_FILTER_PARAMETERS;
pub type PDEBUG_STACK_FRAME = *mut DEBUG_STACK_FRAME;
pub type PDEBUG_STACK_FRAME_EX = *mut DEBUG_STACK_FRAME_EX;
pub type PDEBUG_STACK_PROVIDER_BEGINTHREADSTACKRECONSTRUCTION = Option<unsafe extern "system" fn(streamtype: u32, minidumpstreambuffer: *const core::ffi::c_void, buffersize: u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_STACK_PROVIDER_ENDTHREADSTACKRECONSTRUCTION = Option<unsafe extern "system" fn() -> windows_sys::core::HRESULT>;
pub type PDEBUG_STACK_PROVIDER_FREESTACKSYMFRAMES = Option<unsafe extern "system" fn(stacksymframes: *const STACK_SYM_FRAME_INFO) -> windows_sys::core::HRESULT>;
pub type PDEBUG_STACK_PROVIDER_RECONSTRUCTSTACK = Option<unsafe extern "system" fn(systemthreadid: u32, nativeframes: *const DEBUG_STACK_FRAME_EX, countnativeframes: u32, stacksymframes: *mut PSTACK_SYM_FRAME_INFO, stacksymframesfilled: *mut u32) -> windows_sys::core::HRESULT>;
pub type PDEBUG_SYMBOL_ENTRY = *mut DEBUG_SYMBOL_ENTRY;
pub type PDEBUG_SYMBOL_PARAMETERS = *mut DEBUG_SYMBOL_PARAMETERS;
pub type PDEBUG_SYMBOL_SOURCE_ENTRY = *mut DEBUG_SYMBOL_SOURCE_ENTRY;
pub type PDEBUG_THREAD_BASIC_INFORMATION = *mut DEBUG_THREAD_BASIC_INFORMATION;
pub type PDEBUG_VALUE = *mut DEBUG_VALUE;
pub type PPROCESS_NAME_ENTRY = *mut PROCESS_NAME_ENTRY;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_NAME_ENTRY {
    pub ProcessId: u32,
    pub NameOffset: u32,
    pub NameSize: u32,
    pub NextEntry: u32,
}
pub type PSTACK_SRC_INFO = *mut STACK_SRC_INFO;
pub type PSTACK_SYM_FRAME_INFO = *mut STACK_SYM_FRAME_INFO;
pub type PSYMBOL_INFO_EX = *mut SYMBOL_INFO_EX;
#[cfg(all(feature = "wdbgexts", feature = "winnt"))]
pub type PWINDBG_EXTENSION_APIS32 = *mut super::WINDBG_EXTENSION_APIS32;
#[cfg(all(feature = "wdbgexts", feature = "winnt"))]
pub type PWINDBG_EXTENSION_APIS64 = *mut super::WINDBG_EXTENSION_APIS64;
pub const STACK_FRAME_TYPE_IGNORE: i32 = 255;
pub const STACK_FRAME_TYPE_INIT: i32 = 0;
pub const STACK_FRAME_TYPE_INLINE: i32 = 2;
pub const STACK_FRAME_TYPE_RA: i32 = 128;
pub const STACK_FRAME_TYPE_STACK: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STACK_SRC_INFO {
    pub ImagePath: windows_sys::core::PCWSTR,
    pub ModuleName: windows_sys::core::PCWSTR,
    pub Function: windows_sys::core::PCWSTR,
    pub Displacement: u32,
    pub Row: u32,
    pub Column: u32,
}
impl Default for STACK_SRC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct STACK_SYM_FRAME_INFO {
    pub StackFrameEx: DEBUG_STACK_FRAME_EX,
    pub SrcInfo: STACK_SRC_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYMBOL_INFO_EX {
    pub SizeOfStruct: u32,
    pub TypeOfInfo: u32,
    pub Offset: u64,
    pub Line: u32,
    pub Displacement: u32,
    pub Reserved: [u32; 4],
}
impl Default for SYMBOL_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
