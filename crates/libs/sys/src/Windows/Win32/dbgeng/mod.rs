windows_link::link!("dbgeng.dll" "system" fn DebugConnect(remoteoptions : windows_sys::core::PCSTR, interfaceid : *const windows_sys::core::GUID, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dbgeng.dll" "system" fn DebugConnectWide(remoteoptions : windows_sys::core::PCWSTR, interfaceid : *const windows_sys::core::GUID, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dbgeng.dll" "system" fn DebugCreate(interfaceid : *const windows_sys::core::GUID, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("dbgeng.dll" "system" fn DebugCreateEx(interfaceid : *const windows_sys::core::GUID, dbgengoptions : u32, interface : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub const DBG_FRAME_DEFAULT: u32 = 0;
pub const DBG_FRAME_IGNORE_INLINE: u32 = 4294967295;
pub const DEBUG_ADDSYNTHMOD_DEFAULT: u32 = 0;
pub const DEBUG_ADDSYNTHMOD_ZEROBASE: u32 = 1;
pub const DEBUG_ADDSYNTHSYM_DEFAULT: u32 = 0;
pub const DEBUG_ANY_ID: u32 = 4294967295;
pub const DEBUG_ASMOPT_DEFAULT: u32 = 0;
pub const DEBUG_ASMOPT_IGNORE_OUTPUT_WIDTH: u32 = 4;
pub const DEBUG_ASMOPT_NO_CODE_BYTES: u32 = 2;
pub const DEBUG_ASMOPT_SOURCE_LINE_NUMBER: u32 = 8;
pub const DEBUG_ASMOPT_VERBOSE: u32 = 1;
pub const DEBUG_ATTACH_DEFAULT: u32 = 0;
pub const DEBUG_ATTACH_EXDI_DRIVER: u32 = 2;
pub const DEBUG_ATTACH_EXISTING: u32 = 2;
pub const DEBUG_ATTACH_INSTALL_DRIVER: u32 = 4;
pub const DEBUG_ATTACH_INVASIVE_NO_INITIAL_BREAK: u32 = 8;
pub const DEBUG_ATTACH_INVASIVE_RESUME_PROCESS: u32 = 16;
pub const DEBUG_ATTACH_KERNEL_CONNECTION: u32 = 0;
pub const DEBUG_ATTACH_LOCAL_KERNEL: u32 = 1;
pub const DEBUG_ATTACH_NONINVASIVE: u32 = 1;
pub const DEBUG_ATTACH_NONINVASIVE_ALLOW_PARTIAL: u32 = 32;
pub const DEBUG_ATTACH_NONINVASIVE_NO_SUSPEND: u32 = 4;
pub const DEBUG_BREAKPOINT_ADDER_ONLY: u32 = 8;
pub const DEBUG_BREAKPOINT_CODE: u32 = 0;
pub const DEBUG_BREAKPOINT_DATA: u32 = 1;
pub const DEBUG_BREAKPOINT_DEFERRED: u32 = 2;
pub const DEBUG_BREAKPOINT_ENABLED: u32 = 4;
pub const DEBUG_BREAKPOINT_GO_ONLY: u32 = 1;
pub const DEBUG_BREAKPOINT_INLINE: u32 = 3;
pub const DEBUG_BREAKPOINT_ONE_SHOT: u32 = 16;
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
pub const DEBUG_BREAKPOINT_TIME: u32 = 2;
pub const DEBUG_BREAK_EXECUTE: u32 = 4;
pub const DEBUG_BREAK_IO: u32 = 8;
pub const DEBUG_BREAK_READ: u32 = 1;
pub const DEBUG_BREAK_WRITE: u32 = 2;
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
pub const DEBUG_CDS_DATA: u32 = 2;
pub const DEBUG_CDS_REFRESH: u32 = 4;
pub const DEBUG_CDS_REFRESH_ADDBREAKPOINT: u32 = 4;
pub const DEBUG_CDS_REFRESH_EVALUATE: u32 = 1;
pub const DEBUG_CDS_REFRESH_EXECUTE: u32 = 2;
pub const DEBUG_CDS_REFRESH_EXECUTECOMMANDFILE: u32 = 3;
pub const DEBUG_CDS_REFRESH_INLINESTEP: u32 = 16;
pub const DEBUG_CDS_REFRESH_INLINESTEP_PSEUDO: u32 = 17;
pub const DEBUG_CDS_REFRESH_REMOVEBREAKPOINT: u32 = 5;
pub const DEBUG_CDS_REFRESH_SETSCOPE: u32 = 12;
pub const DEBUG_CDS_REFRESH_SETSCOPEFRAMEBYINDEX: u32 = 13;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMJITDEBUGINFO: u32 = 14;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMSTOREDEVENT: u32 = 15;
pub const DEBUG_CDS_REFRESH_SETVALUE: u32 = 10;
pub const DEBUG_CDS_REFRESH_SETVALUE2: u32 = 11;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL: u32 = 8;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL2: u32 = 9;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUAL: u32 = 6;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUALUNCACHED: u32 = 7;
pub const DEBUG_CDS_REGISTERS: u32 = 1;
pub const DEBUG_CES_ALL: u32 = 4294967295;
pub const DEBUG_CES_ASSEMBLY_OPTIONS: u32 = 4096;
pub const DEBUG_CES_BREAKPOINTS: u32 = 4;
pub const DEBUG_CES_CODE_LEVEL: u32 = 8;
pub const DEBUG_CES_CURRENT_THREAD: u32 = 1;
pub const DEBUG_CES_EFFECTIVE_PROCESSOR: u32 = 2;
pub const DEBUG_CES_ENGINE_OPTIONS: u32 = 32;
pub const DEBUG_CES_EVENT_FILTERS: u32 = 256;
pub const DEBUG_CES_EXECUTION_STATUS: u32 = 16;
pub const DEBUG_CES_EXPRESSION_SYNTAX: u32 = 8192;
pub const DEBUG_CES_EXTENSIONS: u32 = 1024;
pub const DEBUG_CES_LOG_FILE: u32 = 64;
pub const DEBUG_CES_PROCESS_OPTIONS: u32 = 512;
pub const DEBUG_CES_RADIX: u32 = 128;
pub const DEBUG_CES_STEP_FILTERS: u32 = 65536;
pub const DEBUG_CES_SYSTEMS: u32 = 2048;
pub const DEBUG_CES_TEXT_REPLACEMENTS: u32 = 16384;
pub const DEBUG_CES_VIEWS: u32 = 32768;
pub const DEBUG_CLASS_IMAGE_FILE: u32 = 3;
pub const DEBUG_CLASS_KERNEL: u32 = 1;
pub const DEBUG_CLASS_UNINITIALIZED: u32 = 0;
pub const DEBUG_CLASS_USER_WINDOWS: u32 = 2;
pub const DEBUG_CLIENT_CDB: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_CLIENT_CONTEXT {
    pub cbSize: u32,
    pub eClient: u32,
}
pub const DEBUG_CLIENT_KD: u32 = 5;
pub const DEBUG_CLIENT_NTKD: u32 = 3;
pub const DEBUG_CLIENT_NTSD: u32 = 2;
pub const DEBUG_CLIENT_UNKNOWN: u32 = 0;
pub const DEBUG_CLIENT_VSINT: u32 = 1;
pub const DEBUG_CLIENT_WINDBG: u32 = 6;
pub const DEBUG_CLIENT_WINIDE: u32 = 7;
pub const DEBUG_CMDEX_ADD_EVENT_STRING: u32 = 1;
pub const DEBUG_CMDEX_INVALID: u32 = 0;
pub const DEBUG_CMDEX_RESET_EVENT_STRINGS: u32 = 2;
pub const DEBUG_COMMAND_EXCEPTION_ID: u32 = 3688893886;
pub const DEBUG_CONNECT_SESSION_DEFAULT: u32 = 0;
pub const DEBUG_CONNECT_SESSION_NO_ANNOUNCE: u32 = 2;
pub const DEBUG_CONNECT_SESSION_NO_VERSION: u32 = 1;
pub const DEBUG_CREATE_PROCESS_NO_DEBUG_HEAP: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_CREATE_PROCESS_OPTIONS {
    pub CreateFlags: u32,
    pub EngCreateFlags: u32,
    pub VerifierFlags: u32,
    pub Reserved: u32,
}
pub const DEBUG_CREATE_PROCESS_THROUGH_RTL: u32 = 65536;
pub const DEBUG_CSS_ALL: u32 = 4294967295;
pub const DEBUG_CSS_COLLAPSE_CHILDREN: u32 = 64;
pub const DEBUG_CSS_LOADS: u32 = 1;
pub const DEBUG_CSS_PATHS: u32 = 8;
pub const DEBUG_CSS_SCOPE: u32 = 4;
pub const DEBUG_CSS_SYMBOL_OPTIONS: u32 = 16;
pub const DEBUG_CSS_TYPE_OPTIONS: u32 = 32;
pub const DEBUG_CSS_UNLOADS: u32 = 2;
pub const DEBUG_CURRENT_DEFAULT: u32 = 15;
pub const DEBUG_CURRENT_DISASM: u32 = 2;
pub const DEBUG_CURRENT_REGISTERS: u32 = 4;
pub const DEBUG_CURRENT_SOURCE_LINE: u32 = 8;
pub const DEBUG_CURRENT_SYMBOL: u32 = 1;
pub const DEBUG_DATA_BASE_TRANSLATION_VIRTUAL_OFFSET: u32 = 3;
pub const DEBUG_DATA_BreakpointWithStatusAddr: u32 = 32;
pub const DEBUG_DATA_CmNtCSDVersionAddr: u32 = 616;
pub const DEBUG_DATA_DumpAttributes: u32 = 100072;
pub const DEBUG_DATA_DumpFormatVersion: u32 = 100040;
pub const DEBUG_DATA_DumpMmStorage: u32 = 100064;
pub const DEBUG_DATA_DumpPowerState: u32 = 100056;
pub const DEBUG_DATA_DumpWriterStatus: u32 = 100032;
pub const DEBUG_DATA_DumpWriterVersion: u32 = 100048;
pub const DEBUG_DATA_EtwpDebuggerData: u32 = 816;
pub const DEBUG_DATA_ExpNumberOfPagedPoolsAddr: u32 = 112;
pub const DEBUG_DATA_ExpPagedPoolDescriptorAddr: u32 = 104;
pub const DEBUG_DATA_ExpSystemResourcesListAddr: u32 = 96;
pub const DEBUG_DATA_IopErrorLogListHeadAddr: u32 = 144;
pub const DEBUG_DATA_KPCR_OFFSET: u32 = 0;
pub const DEBUG_DATA_KPRCB_OFFSET: u32 = 1;
pub const DEBUG_DATA_KTHREAD_OFFSET: u32 = 2;
pub const DEBUG_DATA_KdPrintBufferSizeAddr: u32 = 720;
pub const DEBUG_DATA_KdPrintCircularBufferAddr: u32 = 480;
pub const DEBUG_DATA_KdPrintCircularBufferEndAddr: u32 = 488;
pub const DEBUG_DATA_KdPrintCircularBufferPtrAddr: u32 = 712;
pub const DEBUG_DATA_KdPrintRolloverCountAddr: u32 = 504;
pub const DEBUG_DATA_KdPrintWritePointerAddr: u32 = 496;
pub const DEBUG_DATA_KeBugCheckCallbackListHeadAddr: u32 = 128;
pub const DEBUG_DATA_KeTimeIncrementAddr: u32 = 120;
pub const DEBUG_DATA_KeUserCallbackDispatcherAddr: u32 = 64;
pub const DEBUG_DATA_KernBase: u32 = 24;
pub const DEBUG_DATA_KernelVerifierAddr: u32 = 576;
pub const DEBUG_DATA_KiBugcheckDataAddr: u32 = 136;
pub const DEBUG_DATA_KiCallUserModeAddr: u32 = 56;
pub const DEBUG_DATA_KiNormalSystemCall: u32 = 528;
pub const DEBUG_DATA_KiProcessorBlockAddr: u32 = 536;
pub const DEBUG_DATA_MmAllocatedNonPagedPoolAddr: u32 = 592;
pub const DEBUG_DATA_MmAvailablePagesAddr: u32 = 424;
pub const DEBUG_DATA_MmBadPagesDetected: u32 = 800;
pub const DEBUG_DATA_MmDriverCommitAddr: u32 = 352;
pub const DEBUG_DATA_MmExtendedCommitAddr: u32 = 376;
pub const DEBUG_DATA_MmFreePageListHeadAddr: u32 = 392;
pub const DEBUG_DATA_MmHighestPhysicalPageAddr: u32 = 240;
pub const DEBUG_DATA_MmHighestUserAddressAddr: u32 = 456;
pub const DEBUG_DATA_MmLastUnloadedDriverAddr: u32 = 552;
pub const DEBUG_DATA_MmLoadedUserImageListAddr: u32 = 512;
pub const DEBUG_DATA_MmLowestPhysicalPageAddr: u32 = 232;
pub const DEBUG_DATA_MmMaximumNonPagedPoolInBytesAddr: u32 = 256;
pub const DEBUG_DATA_MmModifiedNoWritePageListHeadAddr: u32 = 416;
pub const DEBUG_DATA_MmModifiedPageListHeadAddr: u32 = 408;
pub const DEBUG_DATA_MmNonPagedPoolEndAddr: u32 = 280;
pub const DEBUG_DATA_MmNonPagedPoolStartAddr: u32 = 272;
pub const DEBUG_DATA_MmNonPagedSystemStartAddr: u32 = 264;
pub const DEBUG_DATA_MmNumberOfPagingFilesAddr: u32 = 224;
pub const DEBUG_DATA_MmNumberOfPhysicalPagesAddr: u32 = 248;
pub const DEBUG_DATA_MmPageSize: u32 = 312;
pub const DEBUG_DATA_MmPagedPoolCommitAddr: u32 = 368;
pub const DEBUG_DATA_MmPagedPoolEndAddr: u32 = 296;
pub const DEBUG_DATA_MmPagedPoolInformationAddr: u32 = 304;
pub const DEBUG_DATA_MmPagedPoolStartAddr: u32 = 288;
pub const DEBUG_DATA_MmPeakCommitmentAddr: u32 = 600;
pub const DEBUG_DATA_MmPfnDatabaseAddr: u32 = 192;
pub const DEBUG_DATA_MmPhysicalMemoryBlockAddr: u32 = 624;
pub const DEBUG_DATA_MmProcessCommitAddr: u32 = 360;
pub const DEBUG_DATA_MmResidentAvailablePagesAddr: u32 = 432;
pub const DEBUG_DATA_MmSessionBase: u32 = 632;
pub const DEBUG_DATA_MmSessionSize: u32 = 640;
pub const DEBUG_DATA_MmSharedCommitAddr: u32 = 344;
pub const DEBUG_DATA_MmSizeOfPagedPoolInBytesAddr: u32 = 320;
pub const DEBUG_DATA_MmSpecialPoolTagAddr: u32 = 568;
pub const DEBUG_DATA_MmStandbyPageListHeadAddr: u32 = 400;
pub const DEBUG_DATA_MmSubsectionBaseAddr: u32 = 216;
pub const DEBUG_DATA_MmSystemCacheEndAddr: u32 = 176;
pub const DEBUG_DATA_MmSystemCacheStartAddr: u32 = 168;
pub const DEBUG_DATA_MmSystemCacheWsAddr: u32 = 184;
pub const DEBUG_DATA_MmSystemParentTablePage: u32 = 648;
pub const DEBUG_DATA_MmSystemPtesEndAddr: u32 = 208;
pub const DEBUG_DATA_MmSystemPtesStartAddr: u32 = 200;
pub const DEBUG_DATA_MmSystemRangeStartAddr: u32 = 464;
pub const DEBUG_DATA_MmTotalCommitLimitAddr: u32 = 328;
pub const DEBUG_DATA_MmTotalCommitLimitMaximumAddr: u32 = 608;
pub const DEBUG_DATA_MmTotalCommittedPagesAddr: u32 = 336;
pub const DEBUG_DATA_MmTriageActionTakenAddr: u32 = 560;
pub const DEBUG_DATA_MmUnloadedDriversAddr: u32 = 544;
pub const DEBUG_DATA_MmUserProbeAddressAddr: u32 = 472;
pub const DEBUG_DATA_MmVerifierDataAddr: u32 = 584;
pub const DEBUG_DATA_MmVirtualTranslationBase: u32 = 656;
pub const DEBUG_DATA_MmZeroedPageListHeadAddr: u32 = 384;
pub const DEBUG_DATA_NonPagedPoolDescriptorAddr: u32 = 448;
pub const DEBUG_DATA_NtBuildLabAddr: u32 = 520;
pub const DEBUG_DATA_ObpRootDirectoryObjectAddr: u32 = 152;
pub const DEBUG_DATA_ObpTypeObjectTypeAddr: u32 = 160;
pub const DEBUG_DATA_OffsetEprocessDirectoryTableBase: u32 = 686;
pub const DEBUG_DATA_OffsetEprocessParentCID: u32 = 684;
pub const DEBUG_DATA_OffsetEprocessPeb: u32 = 682;
pub const DEBUG_DATA_OffsetKThreadApcProcess: u32 = 672;
pub const DEBUG_DATA_OffsetKThreadBStore: u32 = 676;
pub const DEBUG_DATA_OffsetKThreadBStoreLimit: u32 = 678;
pub const DEBUG_DATA_OffsetKThreadInitialStack: u32 = 670;
pub const DEBUG_DATA_OffsetKThreadKernelStack: u32 = 668;
pub const DEBUG_DATA_OffsetKThreadNextProcessor: u32 = 664;
pub const DEBUG_DATA_OffsetKThreadState: u32 = 674;
pub const DEBUG_DATA_OffsetKThreadTeb: u32 = 666;
pub const DEBUG_DATA_OffsetPrcbCpuType: u32 = 696;
pub const DEBUG_DATA_OffsetPrcbCurrentThread: u32 = 692;
pub const DEBUG_DATA_OffsetPrcbDpcRoutine: u32 = 690;
pub const DEBUG_DATA_OffsetPrcbMhz: u32 = 694;
pub const DEBUG_DATA_OffsetPrcbNumber: u32 = 702;
pub const DEBUG_DATA_OffsetPrcbProcessorState: u32 = 700;
pub const DEBUG_DATA_OffsetPrcbVendorString: u32 = 698;
pub const DEBUG_DATA_PROCESSOR_IDENTIFICATION: u32 = 4;
pub const DEBUG_DATA_PROCESSOR_SPEED: u32 = 5;
pub const DEBUG_DATA_PaeEnabled: u32 = 100000;
pub const DEBUG_DATA_PagingLevels: u32 = 100080;
pub const DEBUG_DATA_PoolTrackTableAddr: u32 = 440;
pub const DEBUG_DATA_ProductType: u32 = 100016;
pub const DEBUG_DATA_PsActiveProcessHeadAddr: u32 = 80;
pub const DEBUG_DATA_PsLoadedModuleListAddr: u32 = 72;
pub const DEBUG_DATA_PspCidTableAddr: u32 = 88;
pub const DEBUG_DATA_PteBase: u32 = 864;
pub const DEBUG_DATA_SPACE_BUS_DATA: u32 = 5;
pub const DEBUG_DATA_SPACE_CONTROL: u32 = 2;
pub const DEBUG_DATA_SPACE_COUNT: u32 = 7;
pub const DEBUG_DATA_SPACE_DEBUGGER_DATA: u32 = 6;
pub const DEBUG_DATA_SPACE_IO: u32 = 3;
pub const DEBUG_DATA_SPACE_MSR: u32 = 4;
pub const DEBUG_DATA_SPACE_PHYSICAL: u32 = 1;
pub const DEBUG_DATA_SPACE_VIRTUAL: u32 = 0;
pub const DEBUG_DATA_SavedContextAddr: u32 = 40;
pub const DEBUG_DATA_SharedUserData: u32 = 100008;
pub const DEBUG_DATA_SizeEProcess: u32 = 680;
pub const DEBUG_DATA_SizeEThread: u32 = 704;
pub const DEBUG_DATA_SizePrcb: u32 = 688;
pub const DEBUG_DATA_SuiteMask: u32 = 100024;
pub const DEBUG_DISASM_EFFECTIVE_ADDRESS: u32 = 1;
pub const DEBUG_DISASM_MATCHING_SYMBOLS: u32 = 2;
pub const DEBUG_DISASM_SOURCE_FILE_NAME: u32 = 8;
pub const DEBUG_DISASM_SOURCE_LINE_NUMBER: u32 = 4;
pub const DEBUG_DUMP_ACTIVE: u32 = 1030;
pub const DEBUG_DUMP_DEFAULT: u32 = 1025;
pub const DEBUG_DUMP_FILE_BASE: u32 = 4294967295;
pub const DEBUG_DUMP_FILE_LOAD_FAILED_INDEX: u32 = 4294967295;
pub const DEBUG_DUMP_FILE_ORIGINAL_CAB_INDEX: u32 = 4294967294;
pub const DEBUG_DUMP_FILE_PAGE_FILE_DUMP: u32 = 0;
pub const DEBUG_DUMP_FULL: u32 = 1026;
pub const DEBUG_DUMP_IMAGE_FILE: u32 = 1027;
pub const DEBUG_DUMP_SMALL: u32 = 1024;
pub const DEBUG_DUMP_TRACE_LOG: u32 = 1028;
pub const DEBUG_DUMP_WINDOWS_CE: u32 = 1029;
pub const DEBUG_ECREATE_PROCESS_DEFAULT: u32 = 0;
pub const DEBUG_ECREATE_PROCESS_INHERIT_HANDLES: u32 = 1;
pub const DEBUG_ECREATE_PROCESS_USE_IMPLICIT_COMMAND_LINE: u32 = 4;
pub const DEBUG_ECREATE_PROCESS_USE_VERIFIER_FLAGS: u32 = 2;
pub const DEBUG_EINDEX_FROM_CURRENT: u32 = 2;
pub const DEBUG_EINDEX_FROM_END: u32 = 1;
pub const DEBUG_EINDEX_FROM_START: u32 = 0;
pub const DEBUG_EINDEX_NAME: u32 = 0;
pub const DEBUG_END_ACTIVE_DETACH: u32 = 2;
pub const DEBUG_END_ACTIVE_TERMINATE: u32 = 1;
pub const DEBUG_END_DISCONNECT: u32 = 4;
pub const DEBUG_END_PASSIVE: u32 = 0;
pub const DEBUG_END_REENTRANT: u32 = 3;
pub const DEBUG_ENGOPT_ALL: u32 = 32505855;
pub const DEBUG_ENGOPT_ALLOW_NETWORK_PATHS: u32 = 4;
pub const DEBUG_ENGOPT_ALLOW_READ_ONLY_BREAKPOINTS: u32 = 1024;
pub const DEBUG_ENGOPT_DEBUGGING_SENSITIVE_DATA: u32 = 4194304;
pub const DEBUG_ENGOPT_DISABLESQM: u32 = 524288;
pub const DEBUG_ENGOPT_DISABLE_EXECUTION_COMMANDS: u32 = 65536;
pub const DEBUG_ENGOPT_DISABLE_MANAGED_SUPPORT: u32 = 16384;
pub const DEBUG_ENGOPT_DISABLE_MODULE_SYMBOL_LOAD: u32 = 32768;
pub const DEBUG_ENGOPT_DISABLE_STEPLINES_OPTIONS: u32 = 2097152;
pub const DEBUG_ENGOPT_DISALLOW_IMAGE_FILE_MAPPING: u32 = 131072;
pub const DEBUG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8;
pub const DEBUG_ENGOPT_DISALLOW_SHELL_COMMANDS: u32 = 4096;
pub const DEBUG_ENGOPT_FAIL_INCOMPLETE_INFORMATION: u32 = 512;
pub const DEBUG_ENGOPT_FINAL_BREAK: u32 = 128;
pub const DEBUG_ENGOPT_IGNORE_DBGHELP_VERSION: u32 = 1;
pub const DEBUG_ENGOPT_IGNORE_EXTENSION_VERSIONS: u32 = 2;
pub const DEBUG_ENGOPT_IGNORE_LOADER_EXCEPTIONS: u32 = 16;
pub const DEBUG_ENGOPT_INITIAL_BREAK: u32 = 32;
pub const DEBUG_ENGOPT_INITIAL_MODULE_BREAK: u32 = 64;
pub const DEBUG_ENGOPT_KD_QUIET_MODE: u32 = 8192;
pub const DEBUG_ENGOPT_NETWORK_PATHS: u32 = 12;
pub const DEBUG_ENGOPT_NO_EXECUTE_REPEAT: u32 = 256;
pub const DEBUG_ENGOPT_PREFER_DML: u32 = 262144;
pub const DEBUG_ENGOPT_PREFER_TRACE_FILES: u32 = 8388608;
pub const DEBUG_ENGOPT_RESOLVE_SHADOWED_VARIABLES: u32 = 16777216;
pub const DEBUG_ENGOPT_SYNCHRONIZE_BREAKPOINTS: u32 = 2048;
pub const DEBUG_EVENT_BREAKPOINT: u32 = 1;
pub const DEBUG_EVENT_CHANGE_DEBUGGEE_STATE: u32 = 1024;
pub const DEBUG_EVENT_CHANGE_ENGINE_STATE: u32 = 2048;
pub const DEBUG_EVENT_CHANGE_SYMBOL_STATE: u32 = 4096;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_EVENT_CONTEXT {
    pub Size: u32,
    pub ProcessEngineId: u32,
    pub ThreadEngineId: u32,
    pub FrameEngineId: u32,
}
pub const DEBUG_EVENT_CREATE_PROCESS: u32 = 16;
pub const DEBUG_EVENT_CREATE_THREAD: u32 = 4;
pub const DEBUG_EVENT_EXCEPTION: u32 = 2;
pub const DEBUG_EVENT_EXIT_PROCESS: u32 = 32;
pub const DEBUG_EVENT_EXIT_THREAD: u32 = 8;
pub const DEBUG_EVENT_LOAD_MODULE: u32 = 64;
pub const DEBUG_EVENT_SERVICE_EXCEPTION: u32 = 8192;
pub const DEBUG_EVENT_SESSION_STATUS: u32 = 512;
pub const DEBUG_EVENT_SYSTEM_ERROR: u32 = 256;
pub const DEBUG_EVENT_UNLOAD_MODULE: u32 = 128;
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
pub const DEBUG_EXECUTE_DEFAULT: u32 = 0;
pub const DEBUG_EXECUTE_ECHO: u32 = 1;
pub const DEBUG_EXECUTE_EVENT: u32 = 2048;
pub const DEBUG_EXECUTE_EXTENSION: u32 = 32;
pub const DEBUG_EXECUTE_HOTKEY: u32 = 1024;
pub const DEBUG_EXECUTE_INTERNAL: u32 = 64;
pub const DEBUG_EXECUTE_MENU: u32 = 512;
pub const DEBUG_EXECUTE_NOT_LOGGED: u32 = 2;
pub const DEBUG_EXECUTE_NO_REPEAT: u32 = 4;
pub const DEBUG_EXECUTE_SCRIPT: u32 = 128;
pub const DEBUG_EXECUTE_TOOLBAR: u32 = 256;
pub const DEBUG_EXECUTE_USER_CLICKED: u32 = 16;
pub const DEBUG_EXECUTE_USER_TYPED: u32 = 8;
pub const DEBUG_EXEC_FLAGS_NONBLOCK: u32 = 1;
pub const DEBUG_EXPR_CPLUSPLUS: u32 = 1;
pub const DEBUG_EXPR_MASM: u32 = 0;
pub const DEBUG_EXTENSION_AT_ENGINE: u32 = 0;
pub const DEBUG_EXTENSION_CONTINUE_SEARCH: i32 = -805305743;
pub const DEBUG_EXTENSION_RELOAD_EXTENSION: i32 = -805306130;
pub const DEBUG_EXTINIT_HAS_COMMAND_HELP: u32 = 1;
pub const DEBUG_EXT_PVALUE_DEFAULT: u32 = 0;
pub const DEBUG_EXT_PVTYPE_IS_POINTER: u32 = 1;
pub const DEBUG_EXT_PVTYPE_IS_VALUE: u32 = 0;
pub const DEBUG_EXT_QVALUE_DEFAULT: u32 = 0;
pub const DEBUG_FILTER_BREAK: u32 = 0;
pub const DEBUG_FILTER_CREATE_PROCESS: u32 = 2;
pub const DEBUG_FILTER_CREATE_THREAD: u32 = 0;
pub const DEBUG_FILTER_DEBUGGEE_OUTPUT: u32 = 9;
pub const DEBUG_FILTER_EXIT_PROCESS: u32 = 3;
pub const DEBUG_FILTER_EXIT_THREAD: u32 = 1;
pub const DEBUG_FILTER_GO_HANDLED: u32 = 0;
pub const DEBUG_FILTER_GO_NOT_HANDLED: u32 = 1;
pub const DEBUG_FILTER_IGNORE: u32 = 3;
pub const DEBUG_FILTER_INITIAL_BREAKPOINT: u32 = 7;
pub const DEBUG_FILTER_INITIAL_MODULE_LOAD: u32 = 8;
pub const DEBUG_FILTER_LOAD_MODULE: u32 = 4;
pub const DEBUG_FILTER_OUTPUT: u32 = 2;
pub const DEBUG_FILTER_REMOVE: u32 = 4;
pub const DEBUG_FILTER_SECOND_CHANCE_BREAK: u32 = 1;
pub const DEBUG_FILTER_SYSTEM_ERROR: u32 = 6;
pub const DEBUG_FILTER_UNLOAD_MODULE: u32 = 5;
pub const DEBUG_FIND_SOURCE_BEST_MATCH: u32 = 2;
pub const DEBUG_FIND_SOURCE_DEFAULT: u32 = 0;
pub const DEBUG_FIND_SOURCE_FULL_PATH: u32 = 1;
pub const DEBUG_FIND_SOURCE_NO_SRCSRV: u32 = 4;
pub const DEBUG_FIND_SOURCE_TOKEN_LOOKUP: u32 = 8;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM: u32 = 16;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM_STRICT: u32 = 32;
pub const DEBUG_FORMAT_CAB_SECONDARY_ALL_IMAGES: u32 = 268435456;
pub const DEBUG_FORMAT_CAB_SECONDARY_FILES: u32 = 1073741824;
pub const DEBUG_FORMAT_DEFAULT: u32 = 0;
pub const DEBUG_FORMAT_NO_OVERWRITE: u32 = 2147483648;
pub const DEBUG_FORMAT_USER_SMALL_ADD_AVX_XSTATE_CONTEXT: u32 = 131072;
pub const DEBUG_FORMAT_USER_SMALL_CODE_SEGMENTS: u32 = 4096;
pub const DEBUG_FORMAT_USER_SMALL_DATA_SEGMENTS: u32 = 16;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_MEMORY: u32 = 32;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_PATHS: u32 = 64;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_TRIAGE: u32 = 65536;
pub const DEBUG_FORMAT_USER_SMALL_FULL_AUXILIARY_STATE: u32 = 16384;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY: u32 = 1;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY_INFO: u32 = 1024;
pub const DEBUG_FORMAT_USER_SMALL_HANDLE_DATA: u32 = 2;
pub const DEBUG_FORMAT_USER_SMALL_IGNORE_INACCESSIBLE_MEM: u32 = 134217728;
pub const DEBUG_FORMAT_USER_SMALL_INDIRECT_MEMORY: u32 = 8;
pub const DEBUG_FORMAT_USER_SMALL_IPT_TRACE: u32 = 262144;
pub const DEBUG_FORMAT_USER_SMALL_MODULE_HEADERS: u32 = 32768;
pub const DEBUG_FORMAT_USER_SMALL_NO_AUXILIARY_STATE: u32 = 8192;
pub const DEBUG_FORMAT_USER_SMALL_NO_IGNORE_INACCESSIBLE_MEM: u32 = 67108864;
pub const DEBUG_FORMAT_USER_SMALL_NO_OPTIONAL_DATA: u32 = 512;
pub const DEBUG_FORMAT_USER_SMALL_PRIVATE_READ_WRITE_MEMORY: u32 = 256;
pub const DEBUG_FORMAT_USER_SMALL_PROCESS_THREAD_DATA: u32 = 128;
pub const DEBUG_FORMAT_USER_SMALL_SCAN_PARTIAL_PAGES: u32 = 268435456;
pub const DEBUG_FORMAT_USER_SMALL_THREAD_INFO: u32 = 2048;
pub const DEBUG_FORMAT_USER_SMALL_UNLOADED_MODULES: u32 = 4;
pub const DEBUG_FORMAT_WRITE_CAB: u32 = 536870912;
pub const DEBUG_FRAME_DEFAULT: u32 = 0;
pub const DEBUG_FRAME_IGNORE_INLINE: u32 = 1;
pub const DEBUG_GETFNENT_DEFAULT: u32 = 0;
pub const DEBUG_GETFNENT_RAW_ENTRY_ONLY: u32 = 1;
pub const DEBUG_GETMOD_DEFAULT: u32 = 0;
pub const DEBUG_GETMOD_NO_LOADED_MODULES: u32 = 1;
pub const DEBUG_GETMOD_NO_UNLOADED_MODULES: u32 = 2;
pub const DEBUG_GET_PROC_DEFAULT: u32 = 0;
pub const DEBUG_GET_PROC_FULL_MATCH: u32 = 1;
pub const DEBUG_GET_PROC_ONLY_MATCH: u32 = 2;
pub const DEBUG_GET_PROC_SERVICE_NAME: u32 = 4;
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
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_DOT_COMMAND: u32 = 1;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_EXTENSION_COMMAND: u32 = 2;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_SYMBOL: u32 = 4;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_DOT_COMMANDS: u32 = 1;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_EXTENSION_COMMANDS: u32 = 2;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_SYMBOLS: u32 = 4;
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
pub const DEBUG_GSEL_ALLOW_HIGHER: u32 = 4;
pub const DEBUG_GSEL_ALLOW_LOWER: u32 = 2;
pub const DEBUG_GSEL_DEFAULT: u32 = 0;
pub const DEBUG_GSEL_INLINE_CALLSITE: u32 = 16;
pub const DEBUG_GSEL_NEAREST_ONLY: u32 = 8;
pub const DEBUG_GSEL_NO_SYMBOL_LOADS: u32 = 1;
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
pub const DEBUG_HANDLE_DATA_TYPE_ALL_HANDLE_OPERATIONS: u32 = 10;
pub const DEBUG_HANDLE_DATA_TYPE_BASIC: u32 = 0;
pub const DEBUG_HANDLE_DATA_TYPE_HANDLE_COUNT: u32 = 3;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_EVENT_1: u32 = 13;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_1: u32 = 7;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_2: u32 = 8;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_1: u32 = 11;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_2: u32 = 12;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SECTION_1: u32 = 14;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SEMAPHORE_1: u32 = 15;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_THREAD_1: u32 = 6;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME: u32 = 2;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME_WIDE: u32 = 5;
pub const DEBUG_HANDLE_DATA_TYPE_PER_HANDLE_OPERATIONS: u32 = 9;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME: u32 = 1;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME_WIDE: u32 = 4;
pub const DEBUG_INTERRUPT_ACTIVE: u32 = 0;
pub const DEBUG_INTERRUPT_EXIT: u32 = 2;
pub const DEBUG_INTERRUPT_PASSIVE: u32 = 1;
pub const DEBUG_INVALID_OFFSET: u64 = 18446744073709551615;
pub const DEBUG_IOUTPUT_ADDR_TRANSLATE: u32 = 134217728;
pub const DEBUG_IOUTPUT_BREAKPOINT: u32 = 536870912;
pub const DEBUG_IOUTPUT_EVENT: u32 = 268435456;
pub const DEBUG_IOUTPUT_KD_PROTOCOL: u32 = 2147483648;
pub const DEBUG_IOUTPUT_REMOTING: u32 = 1073741824;
pub const DEBUG_KERNEL_ACTIVE_DUMP: u32 = 1030;
pub const DEBUG_KERNEL_CONNECTION: u32 = 0;
pub const DEBUG_KERNEL_DUMP: u32 = 1025;
pub const DEBUG_KERNEL_EXDI_DRIVER: u32 = 2;
pub const DEBUG_KERNEL_FULL_DUMP: u32 = 1026;
pub const DEBUG_KERNEL_IDNA: u32 = 3;
pub const DEBUG_KERNEL_INSTALL_DRIVER: u32 = 4;
pub const DEBUG_KERNEL_LOCAL: u32 = 1;
pub const DEBUG_KERNEL_REPT: u32 = 5;
pub const DEBUG_KERNEL_SMALL_DUMP: u32 = 1024;
pub const DEBUG_KERNEL_TRACE_LOG: u32 = 1028;
pub const DEBUG_KNOWN_STRUCT_GET_NAMES: u32 = 1;
pub const DEBUG_KNOWN_STRUCT_GET_SINGLE_LINE_OUTPUT: u32 = 2;
pub const DEBUG_KNOWN_STRUCT_SUPPRESS_TYPE_NAME: u32 = 3;
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
pub const DEBUG_LEVEL_ASSEMBLY: u32 = 1;
pub const DEBUG_LEVEL_SOURCE: u32 = 0;
pub const DEBUG_LIVE_USER_NON_INVASIVE: u32 = 33;
pub const DEBUG_LOG_APPEND: u32 = 1;
pub const DEBUG_LOG_DEFAULT: u32 = 0;
pub const DEBUG_LOG_DML: u32 = 4;
pub const DEBUG_LOG_UNICODE: u32 = 2;
pub const DEBUG_MANAGED_ALLOWED: u32 = 1;
pub const DEBUG_MANAGED_DISABLED: u32 = 0;
pub const DEBUG_MANAGED_DLL_LOADED: u32 = 2;
pub const DEBUG_MANRESET_DEFAULT: u32 = 0;
pub const DEBUG_MANRESET_LOAD_DLL: u32 = 1;
pub const DEBUG_MANSTR_LOADED_SUPPORT_DLL: u32 = 1;
pub const DEBUG_MANSTR_LOAD_STATUS: u32 = 2;
pub const DEBUG_MANSTR_NONE: u32 = 0;
pub const DEBUG_MODNAME_IMAGE: u32 = 0;
pub const DEBUG_MODNAME_LOADED_IMAGE: u32 = 2;
pub const DEBUG_MODNAME_MAPPED_IMAGE: u32 = 4;
pub const DEBUG_MODNAME_MODULE: u32 = 1;
pub const DEBUG_MODNAME_SYMBOL_FILE: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_MODULE_AND_ID {
    pub ModuleBase: u64,
    pub Id: u64,
}
pub const DEBUG_MODULE_EXE_MODULE: u32 = 4;
pub const DEBUG_MODULE_EXPLICIT: u32 = 8;
pub const DEBUG_MODULE_LOADED: u32 = 0;
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
pub const DEBUG_MODULE_SECONDARY: u32 = 16;
pub const DEBUG_MODULE_SYM_BAD_CHECKSUM: u32 = 65536;
pub const DEBUG_MODULE_SYNTHETIC: u32 = 32;
pub const DEBUG_MODULE_UNLOADED: u32 = 1;
pub const DEBUG_MODULE_USER_MODE: u32 = 2;
pub const DEBUG_NOTIFY_SESSION_ACCESSIBLE: u32 = 2;
pub const DEBUG_NOTIFY_SESSION_ACTIVE: u32 = 0;
pub const DEBUG_NOTIFY_SESSION_INACCESSIBLE: u32 = 3;
pub const DEBUG_NOTIFY_SESSION_INACTIVE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_OFFSET_REGION {
    pub Base: u64,
    pub Size: u64,
}
pub const DEBUG_OFFSINFO_VIRTUAL_SOURCE: u32 = 1;
pub const DEBUG_OUTCBF_COMBINED_EXPLICIT_FLUSH: u32 = 1;
pub const DEBUG_OUTCBF_DML_HAS_SPECIAL_CHARACTERS: u32 = 4;
pub const DEBUG_OUTCBF_DML_HAS_TAGS: u32 = 2;
pub const DEBUG_OUTCBI_ANY_FORMAT: u32 = 6;
pub const DEBUG_OUTCBI_DML: u32 = 4;
pub const DEBUG_OUTCBI_EXPLICIT_FLUSH: u32 = 1;
pub const DEBUG_OUTCBI_TEXT: u32 = 2;
pub const DEBUG_OUTCB_DML: u32 = 1;
pub const DEBUG_OUTCB_EXPLICIT_FLUSH: u32 = 2;
pub const DEBUG_OUTCB_TEXT: u32 = 0;
pub const DEBUG_OUTCTL_ALL_CLIENTS: u32 = 1;
pub const DEBUG_OUTCTL_ALL_OTHER_CLIENTS: u32 = 2;
pub const DEBUG_OUTCTL_AMBIENT: i32 = -1;
pub const DEBUG_OUTCTL_AMBIENT_DML: u32 = 4294967294;
pub const DEBUG_OUTCTL_AMBIENT_TEXT: u32 = 4294967295;
pub const DEBUG_OUTCTL_DML: u32 = 32;
pub const DEBUG_OUTCTL_IGNORE: u32 = 3;
pub const DEBUG_OUTCTL_LOG_ONLY: u32 = 4;
pub const DEBUG_OUTCTL_NOT_LOGGED: u32 = 8;
pub const DEBUG_OUTCTL_OVERRIDE_MASK: u32 = 16;
pub const DEBUG_OUTCTL_SEND_MASK: u32 = 7;
pub const DEBUG_OUTCTL_THIS_CLIENT: u32 = 0;
pub const DEBUG_OUTPUT_DEBUGGEE: u32 = 128;
pub const DEBUG_OUTPUT_DEBUGGEE_PROMPT: u32 = 256;
pub const DEBUG_OUTPUT_ERROR: u32 = 2;
pub const DEBUG_OUTPUT_EXTENSION_WARNING: u32 = 64;
pub const DEBUG_OUTPUT_IDENTITY_DEFAULT: u32 = 0;
pub const DEBUG_OUTPUT_NAME_END: windows_sys::core::PCSTR = windows_sys::core::s!("**NAME**");
pub const DEBUG_OUTPUT_NAME_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**NAME**");
pub const DEBUG_OUTPUT_NORMAL: u32 = 1;
pub const DEBUG_OUTPUT_OFFSET_END: windows_sys::core::PCSTR = windows_sys::core::s!("**OFF**");
pub const DEBUG_OUTPUT_OFFSET_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**OFF**");
pub const DEBUG_OUTPUT_PROMPT: u32 = 16;
pub const DEBUG_OUTPUT_PROMPT_REGISTERS: u32 = 32;
pub const DEBUG_OUTPUT_STATUS: u32 = 1024;
pub const DEBUG_OUTPUT_SYMBOLS: u32 = 512;
pub const DEBUG_OUTPUT_SYMBOLS_DEFAULT: u32 = 0;
pub const DEBUG_OUTPUT_SYMBOLS_NO_NAMES: u32 = 1;
pub const DEBUG_OUTPUT_SYMBOLS_NO_OFFSETS: u32 = 2;
pub const DEBUG_OUTPUT_SYMBOLS_NO_TYPES: u32 = 16;
pub const DEBUG_OUTPUT_SYMBOLS_NO_VALUES: u32 = 4;
pub const DEBUG_OUTPUT_TYPE_END: windows_sys::core::PCSTR = windows_sys::core::s!("**TYPE**");
pub const DEBUG_OUTPUT_TYPE_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**TYPE**");
pub const DEBUG_OUTPUT_VALUE_END: windows_sys::core::PCSTR = windows_sys::core::s!("**VALUE**");
pub const DEBUG_OUTPUT_VALUE_END_WIDE: windows_sys::core::PCWSTR = windows_sys::core::w!("**VALUE**");
pub const DEBUG_OUTPUT_VERBOSE: u32 = 8;
pub const DEBUG_OUTPUT_WARNING: u32 = 4;
pub const DEBUG_OUTPUT_XML: u32 = 2048;
pub const DEBUG_OUTSYM_ALLOW_DISPLACEMENT: u32 = 4;
pub const DEBUG_OUTSYM_DEFAULT: u32 = 0;
pub const DEBUG_OUTSYM_FORCE_OFFSET: u32 = 1;
pub const DEBUG_OUTSYM_SOURCE_LINE: u32 = 2;
pub const DEBUG_OUTTYPE_ADDRESS_AT_END: u32 = 131072;
pub const DEBUG_OUTTYPE_ADDRESS_OF_FIELD: u32 = 65536;
pub const DEBUG_OUTTYPE_BLOCK_RECURSE: u32 = 2097152;
pub const DEBUG_OUTTYPE_COMPACT_OUTPUT: u32 = 8;
pub const DEBUG_OUTTYPE_DEFAULT: u32 = 0;
pub const DEBUG_OUTTYPE_NO_INDENT: u32 = 1;
pub const DEBUG_OUTTYPE_NO_OFFSET: u32 = 2;
pub const DEBUG_OUTTYPE_VERBOSE: u32 = 4;
pub const DEBUG_OUT_TEXT_REPL_DEFAULT: u32 = 0;
pub const DEBUG_PHYSICAL_CACHED: u32 = 1;
pub const DEBUG_PHYSICAL_DEFAULT: u32 = 0;
pub const DEBUG_PHYSICAL_UNCACHED: u32 = 2;
pub const DEBUG_PHYSICAL_WRITE_COMBINED: u32 = 3;
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
pub const DEBUG_PROCESS_DETACH_ON_EXIT: u32 = 1;
pub const DEBUG_PROCESS_ONLY_THIS_PROCESS: u32 = 2;
pub const DEBUG_PROC_DESC_DEFAULT: u32 = 0;
pub const DEBUG_PROC_DESC_NO_COMMAND_LINE: u32 = 8;
pub const DEBUG_PROC_DESC_NO_MTS_PACKAGES: u32 = 4;
pub const DEBUG_PROC_DESC_NO_PATHS: u32 = 1;
pub const DEBUG_PROC_DESC_NO_SERVICES: u32 = 2;
pub const DEBUG_PROC_DESC_NO_SESSION_ID: u32 = 16;
pub const DEBUG_PROC_DESC_NO_USER_NAME: u32 = 32;
pub const DEBUG_PROC_DESC_WITH_ARCHITECTURE: u32 = 128;
pub const DEBUG_PROC_DESC_WITH_PACKAGEFAMILY: u32 = 64;
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
pub const DEBUG_REGISTERS_ALL: u32 = 7;
pub const DEBUG_REGISTERS_DEFAULT: u32 = 0;
pub const DEBUG_REGISTERS_FLOAT: u32 = 4;
pub const DEBUG_REGISTERS_INT32: u32 = 1;
pub const DEBUG_REGISTERS_INT64: u32 = 2;
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
pub const DEBUG_REGISTER_SUB_REGISTER: u32 = 1;
pub const DEBUG_REGSRC_DEBUGGEE: u32 = 0;
pub const DEBUG_REGSRC_EXPLICIT: u32 = 1;
pub const DEBUG_REGSRC_FRAME: u32 = 2;
pub const DEBUG_REQUEST_ADD_CACHED_SYMBOL_INFO: u32 = 16;
pub const DEBUG_REQUEST_CLOSE_TOKEN: u32 = 30;
pub const DEBUG_REQUEST_CURRENT_OUTPUT_CALLBACKS_ARE_DML_AWARE: u32 = 19;
pub const DEBUG_REQUEST_DUPLICATE_TOKEN: u32 = 28;
pub const DEBUG_REQUEST_EXT_TYPED_DATA_ANSI: u32 = 12;
pub const DEBUG_REQUEST_GET_ADDITIONAL_CREATE_OPTIONS: u32 = 4;
pub const DEBUG_REQUEST_GET_CACHED_SYMBOL_INFO: u32 = 15;
pub const DEBUG_REQUEST_GET_CAPTURED_EVENT_CODE_OFFSET: u32 = 10;
pub const DEBUG_REQUEST_GET_DUMP_HEADER: u32 = 21;
pub const DEBUG_REQUEST_GET_EXTENSION_SEARCH_PATH_WIDE: u32 = 13;
pub const DEBUG_REQUEST_GET_IMAGE_ARCHITECTURE: u32 = 39;
pub const DEBUG_REQUEST_GET_INSTRUMENTATION_VERSION: u32 = 37;
pub const DEBUG_REQUEST_GET_MODULE_ARCHITECTURE: u32 = 38;
pub const DEBUG_REQUEST_GET_OFFSET_UNWIND_INFORMATION: u32 = 20;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_ANSI: u32 = 18;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_WIDE: u32 = 14;
pub const DEBUG_REQUEST_GET_WIN32_MAJOR_MINOR_VERSIONS: u32 = 6;
pub const DEBUG_REQUEST_INLINE_QUERY: u32 = 35;
pub const DEBUG_REQUEST_MIDORI: u32 = 23;
pub const DEBUG_REQUEST_MISC_INFORMATION: u32 = 25;
pub const DEBUG_REQUEST_OPEN_PROCESS_TOKEN: u32 = 26;
pub const DEBUG_REQUEST_OPEN_THREAD_TOKEN: u32 = 27;
pub const DEBUG_REQUEST_PROCESS_DESCRIPTORS: u32 = 24;
pub const DEBUG_REQUEST_QUERY_INFO_TOKEN: u32 = 29;
pub const DEBUG_REQUEST_READ_CAPTURED_EVENT_CODE_STREAM: u32 = 11;
pub const DEBUG_REQUEST_READ_USER_MINIDUMP_STREAM: u32 = 7;
pub const DEBUG_REQUEST_REMOVE_CACHED_SYMBOL_INFO: u32 = 17;
pub const DEBUG_REQUEST_RESUME_THREAD: u32 = 34;
pub const DEBUG_REQUEST_SET_ADDITIONAL_CREATE_OPTIONS: u32 = 5;
pub const DEBUG_REQUEST_SET_DUMP_HEADER: u32 = 22;
pub const DEBUG_REQUEST_SET_LOCAL_IMPLICIT_COMMAND_LINE: u32 = 9;
pub const DEBUG_REQUEST_SET_PARENT_HWND: u32 = 40;
pub const DEBUG_REQUEST_SOURCE_PATH_HAS_SOURCE_SERVER: u32 = 0;
pub const DEBUG_REQUEST_TARGET_CAN_DETACH: u32 = 8;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_CONTEXT: u32 = 1;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_RECORD: u32 = 3;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_THREAD: u32 = 2;
pub const DEBUG_REQUEST_TL_INSTRUMENTATION_AWARE: u32 = 36;
pub const DEBUG_REQUEST_WOW_MODULE: u32 = 32;
pub const DEBUG_REQUEST_WOW_PROCESS: u32 = 31;
pub const DEBUG_SCOPE_GROUP_ALL: u32 = 3;
pub const DEBUG_SCOPE_GROUP_ARGUMENTS: u32 = 1;
pub const DEBUG_SCOPE_GROUP_BY_DATAMODEL: u32 = 4;
pub const DEBUG_SCOPE_GROUP_LOCALS: u32 = 2;
pub const DEBUG_SCOPE_GROUP_VALID_FLAGS: u32 = 7;
pub const DEBUG_SERVERS_ALL: u32 = 3;
pub const DEBUG_SERVERS_DEBUGGER: u32 = 1;
pub const DEBUG_SERVERS_PROCESS: u32 = 2;
pub const DEBUG_SESSION_ACTIVE: u32 = 0;
pub const DEBUG_SESSION_END: u32 = 4;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_DETACH: u32 = 2;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_TERMINATE: u32 = 1;
pub const DEBUG_SESSION_END_SESSION_PASSIVE: u32 = 3;
pub const DEBUG_SESSION_FAILURE: u32 = 7;
pub const DEBUG_SESSION_HIBERNATE: u32 = 6;
pub const DEBUG_SESSION_REBOOT: u32 = 5;
pub const DEBUG_SOURCE_IS_STATEMENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_SPECIFIC_FILTER_PARAMETERS {
    pub ExecutionOption: u32,
    pub ContinueOption: u32,
    pub TextSize: u32,
    pub CommandSize: u32,
    pub ArgumentSize: u32,
}
pub const DEBUG_SRCFILE_SYMBOL_CHECKSUMINFO: u32 = 2;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN: u32 = 0;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN_SOURCE_COMMAND_WIDE: u32 = 1;
pub const DEBUG_STACK_ARGUMENTS: u32 = 1;
pub const DEBUG_STACK_COLUMN_NAMES: u32 = 16;
pub const DEBUG_STACK_DML: u32 = 2048;
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
pub const DEBUG_STACK_FRAME_ADDRESSES: u32 = 8;
pub const DEBUG_STACK_FRAME_ADDRESSES_RA_ONLY: u32 = 256;
pub const DEBUG_STACK_FRAME_ARCH: u32 = 16384;
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
pub const DEBUG_STACK_FRAME_MEMORY_USAGE: u32 = 512;
pub const DEBUG_STACK_FRAME_NUMBERS: u32 = 64;
pub const DEBUG_STACK_FRAME_OFFSETS: u32 = 4096;
pub const DEBUG_STACK_FUNCTION_INFO: u32 = 2;
pub const DEBUG_STACK_NONVOLATILE_REGISTERS: u32 = 32;
pub const DEBUG_STACK_PARAMETERS: u32 = 128;
pub const DEBUG_STACK_PARAMETERS_NEWLINE: u32 = 1024;
pub const DEBUG_STACK_PROVIDER: u32 = 8192;
pub const DEBUG_STACK_SOURCE_LINE: u32 = 4;
pub const DEBUG_STATUS_BREAK: u32 = 6;
pub const DEBUG_STATUS_GO: u32 = 1;
pub const DEBUG_STATUS_GO_HANDLED: u32 = 2;
pub const DEBUG_STATUS_GO_NOT_HANDLED: u32 = 3;
pub const DEBUG_STATUS_IGNORE_EVENT: u32 = 9;
pub const DEBUG_STATUS_INSIDE_WAIT: u64 = 4294967296;
pub const DEBUG_STATUS_MASK: u32 = 31;
pub const DEBUG_STATUS_NO_CHANGE: u32 = 0;
pub const DEBUG_STATUS_NO_DEBUGGEE: u32 = 7;
pub const DEBUG_STATUS_OUT_OF_SYNC: u32 = 15;
pub const DEBUG_STATUS_RESTART_REQUESTED: u32 = 10;
pub const DEBUG_STATUS_REVERSE_GO: u32 = 11;
pub const DEBUG_STATUS_REVERSE_STEP_BRANCH: u32 = 12;
pub const DEBUG_STATUS_REVERSE_STEP_INTO: u32 = 14;
pub const DEBUG_STATUS_REVERSE_STEP_OVER: u32 = 13;
pub const DEBUG_STATUS_STEP_BRANCH: u32 = 8;
pub const DEBUG_STATUS_STEP_INTO: u32 = 5;
pub const DEBUG_STATUS_STEP_OVER: u32 = 4;
pub const DEBUG_STATUS_TIMEOUT: u32 = 17;
pub const DEBUG_STATUS_WAIT_INPUT: u32 = 16;
pub const DEBUG_STATUS_WAIT_TIMEOUT: u64 = 8589934592;
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
pub const DEBUG_SYMBOL_EXPANDED: u32 = 16;
pub const DEBUG_SYMBOL_EXPANSION_LEVEL_MASK: u32 = 15;
pub const DEBUG_SYMBOL_IS_ARGUMENT: u32 = 256;
pub const DEBUG_SYMBOL_IS_ARRAY: u32 = 64;
pub const DEBUG_SYMBOL_IS_FLOAT: u32 = 128;
pub const DEBUG_SYMBOL_IS_LOCAL: u32 = 512;
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
pub const DEBUG_SYMBOL_READ_ONLY: u32 = 32;
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
pub const DEBUG_SYMENT_IS_CODE: u32 = 1;
pub const DEBUG_SYMENT_IS_DATA: u32 = 2;
pub const DEBUG_SYMENT_IS_LOCAL: u32 = 8;
pub const DEBUG_SYMENT_IS_MANAGED: u32 = 16;
pub const DEBUG_SYMENT_IS_PARAMETER: u32 = 4;
pub const DEBUG_SYMENT_IS_SYNTHETIC: u32 = 32;
pub const DEBUG_SYMINFO_BREAKPOINT_SOURCE_LINE: u32 = 0;
pub const DEBUG_SYMINFO_GET_MODULE_SYMBOL_NAMES_AND_OFFSETS: u32 = 3;
pub const DEBUG_SYMINFO_GET_SYMBOL_NAME_BY_OFFSET_AND_TAG_WIDE: u32 = 2;
pub const DEBUG_SYMINFO_IMAGEHLP_MODULEW64: u32 = 1;
pub const DEBUG_SYMTYPE_CODEVIEW: u32 = 2;
pub const DEBUG_SYMTYPE_COFF: u32 = 1;
pub const DEBUG_SYMTYPE_DEFERRED: u32 = 5;
pub const DEBUG_SYMTYPE_DIA: u32 = 7;
pub const DEBUG_SYMTYPE_EXPORT: u32 = 4;
pub const DEBUG_SYMTYPE_NONE: u32 = 0;
pub const DEBUG_SYMTYPE_PDB: u32 = 3;
pub const DEBUG_SYMTYPE_SYM: u32 = 6;
pub const DEBUG_SYSOBJINFO_CURRENT_PROCESS_COOKIE: u32 = 2;
pub const DEBUG_SYSOBJINFO_THREAD_BASIC_INFORMATION: u32 = 0;
pub const DEBUG_SYSOBJINFO_THREAD_NAME_WIDE: u32 = 1;
pub const DEBUG_SYSVERSTR_BUILD: u32 = 1;
pub const DEBUG_SYSVERSTR_SERVICE_PACK: u32 = 0;
pub const DEBUG_TBINFO_AFFINITY: u32 = 32;
pub const DEBUG_TBINFO_ALL: u32 = 63;
pub const DEBUG_TBINFO_EXIT_STATUS: u32 = 1;
pub const DEBUG_TBINFO_PRIORITY: u32 = 4;
pub const DEBUG_TBINFO_PRIORITY_CLASS: u32 = 2;
pub const DEBUG_TBINFO_START_OFFSET: u32 = 16;
pub const DEBUG_TBINFO_TIMES: u32 = 8;
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
pub const DEBUG_TYPEOPTS_FORCERADIX_OUTPUT: u32 = 4;
pub const DEBUG_TYPEOPTS_LONGSTATUS_DISPLAY: u32 = 2;
pub const DEBUG_TYPEOPTS_MATCH_MAXSIZE: u32 = 8;
pub const DEBUG_TYPEOPTS_UNICODE_DISPLAY: u32 = 1;
pub const DEBUG_USER_WINDOWS_DUMP: u32 = 1025;
pub const DEBUG_USER_WINDOWS_DUMP_WINDOWS_CE: u32 = 1029;
pub const DEBUG_USER_WINDOWS_IDNA: u32 = 2;
pub const DEBUG_USER_WINDOWS_PROCESS: u32 = 0;
pub const DEBUG_USER_WINDOWS_PROCESS_SERVER: u32 = 1;
pub const DEBUG_USER_WINDOWS_REPT: u32 = 3;
pub const DEBUG_USER_WINDOWS_SMALL_DUMP: u32 = 1024;
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
pub const DEBUG_VALUE_FLOAT128: u32 = 9;
pub const DEBUG_VALUE_FLOAT32: u32 = 5;
pub const DEBUG_VALUE_FLOAT64: u32 = 6;
pub const DEBUG_VALUE_FLOAT80: u32 = 7;
pub const DEBUG_VALUE_FLOAT82: u32 = 8;
pub const DEBUG_VALUE_INT16: u32 = 2;
pub const DEBUG_VALUE_INT32: u32 = 3;
pub const DEBUG_VALUE_INT64: u32 = 4;
pub const DEBUG_VALUE_INT8: u32 = 1;
pub const DEBUG_VALUE_INVALID: u32 = 0;
pub const DEBUG_VALUE_TYPES: u32 = 12;
pub const DEBUG_VALUE_VECTOR128: u32 = 11;
pub const DEBUG_VALUE_VECTOR64: u32 = 10;
pub const DEBUG_VSEARCH_DEFAULT: u32 = 0;
pub const DEBUG_VSEARCH_WRITABLE_ONLY: u32 = 1;
pub const DEBUG_VSOURCE_DEBUGGEE: u32 = 1;
pub const DEBUG_VSOURCE_DUMP_WITHOUT_MEMINFO: u32 = 3;
pub const DEBUG_VSOURCE_INVALID: u32 = 0;
pub const DEBUG_VSOURCE_MAPPED_IMAGE: u32 = 2;
pub const DEBUG_WAIT_DEFAULT: u32 = 0;
pub const ERROR_DBG_CANCELLED: u32 = 3221226695;
pub const ERROR_DBG_TIMEOUT: u32 = 3221226932;
pub const IMAGE_FILE_MACHINE_ARM64EC: u32 = 42561;
pub const IMAGE_FILE_MACHINE_ARM64X: u32 = 42574;
pub const IMAGE_FILE_MACHINE_CHPE_X86: u32 = 14948;
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
pub const MODULE_ORDERS_LOADTIME: u32 = 268435456;
pub const MODULE_ORDERS_MASK: u32 = 4026531840;
pub const MODULE_ORDERS_MODULENAME: u32 = 536870912;
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
pub const STACK_FRAME_TYPE_IGNORE: u32 = 255;
pub const STACK_FRAME_TYPE_INIT: u32 = 0;
pub const STACK_FRAME_TYPE_INLINE: u32 = 2;
pub const STACK_FRAME_TYPE_RA: u32 = 128;
pub const STACK_FRAME_TYPE_STACK: u32 = 1;
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
