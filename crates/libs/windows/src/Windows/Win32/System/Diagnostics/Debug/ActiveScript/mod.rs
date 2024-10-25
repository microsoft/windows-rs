pub const ACTIVPROF_E_PROFILER_ABSENT: windows_core::HRESULT = 0x80040201_u32 as _;
pub const ACTIVPROF_E_PROFILER_PRESENT: windows_core::HRESULT = 0x80040200_u32 as _;
pub const ACTIVPROF_E_UNABLE_TO_APPLY_ACTION: windows_core::HRESULT = 0x80040202_u32 as _;
pub const APPBREAKFLAG_DEBUGGER_BLOCK: u32 = 1u32;
pub const APPBREAKFLAG_DEBUGGER_HALT: u32 = 2u32;
pub const APPBREAKFLAG_IN_BREAKPOINT: u32 = 2147483648u32;
pub const APPBREAKFLAG_NESTED: u32 = 131072u32;
pub const APPBREAKFLAG_STEP: u32 = 65536u32;
pub const APPBREAKFLAG_STEPTYPE_BYTECODE: u32 = 1048576u32;
pub const APPBREAKFLAG_STEPTYPE_MACHINE: u32 = 2097152u32;
pub const APPBREAKFLAG_STEPTYPE_MASK: u32 = 15728640u32;
pub const APPBREAKFLAG_STEPTYPE_SOURCE: u32 = 0u32;
pub const BREAKPOINT_DELETED: BREAKPOINT_STATE = 0i32;
pub const BREAKPOINT_DISABLED: BREAKPOINT_STATE = 1i32;
pub const BREAKPOINT_ENABLED: BREAKPOINT_STATE = 2i32;
pub const BREAKREASON_BREAKPOINT: BREAKREASON = 1i32;
pub const BREAKREASON_DEBUGGER_BLOCK: BREAKREASON = 2i32;
pub const BREAKREASON_DEBUGGER_HALT: BREAKREASON = 5i32;
pub const BREAKREASON_ERROR: BREAKREASON = 6i32;
pub const BREAKREASON_HOST_INITIATED: BREAKREASON = 3i32;
pub const BREAKREASON_JIT: BREAKREASON = 7i32;
pub const BREAKREASON_LANGUAGE_INITIATED: BREAKREASON = 4i32;
pub const BREAKREASON_MUTATION_BREAKPOINT: BREAKREASON = 8i32;
pub const BREAKREASON_STEP: BREAKREASON = 0i32;
pub const BREAKRESUMEACTION_ABORT: BREAKRESUMEACTION = 0i32;
pub const BREAKRESUMEACTION_CONTINUE: BREAKRESUMEACTION = 1i32;
pub const BREAKRESUMEACTION_IGNORE: BREAKRESUMEACTION = 5i32;
pub const BREAKRESUMEACTION_STEP_DOCUMENT: BREAKRESUMEACTION = 6i32;
pub const BREAKRESUMEACTION_STEP_INTO: BREAKRESUMEACTION = 2i32;
pub const BREAKRESUMEACTION_STEP_OUT: BREAKRESUMEACTION = 4i32;
pub const BREAKRESUMEACTION_STEP_OVER: BREAKRESUMEACTION = 3i32;
pub const CATID_ActiveScript: windows_core::GUID = windows_core::GUID::from_u128(0xf0b7a1a1_9847_11cf_8f20_00805f2cd064);
pub const CATID_ActiveScriptAuthor: windows_core::GUID = windows_core::GUID::from_u128(0x0aee2a92_bcbb_11d0_8c72_00c04fc2b085);
pub const CATID_ActiveScriptEncode: windows_core::GUID = windows_core::GUID::from_u128(0xf0b7a1a3_9847_11cf_8f20_00805f2cd064);
pub const CATID_ActiveScriptParse: windows_core::GUID = windows_core::GUID::from_u128(0xf0b7a1a2_9847_11cf_8f20_00805f2cd064);
pub const DEBUG_TEXT_ALLOWBREAKPOINTS: u32 = 8u32;
pub const DEBUG_TEXT_ALLOWERRORREPORT: u32 = 16u32;
pub const DEBUG_TEXT_EVALUATETOCODECONTEXT: u32 = 32u32;
pub const DEBUG_TEXT_ISEXPRESSION: u32 = 1u32;
pub const DEBUG_TEXT_ISNONUSERCODE: u32 = 64u32;
pub const DEBUG_TEXT_NOSIDEEFFECTS: u32 = 4u32;
pub const DEBUG_TEXT_RETURNVALUE: u32 = 2u32;
pub const DEIT_ASMJS_FAILED: DEBUG_EVENT_INFO_TYPE = 3i32;
pub const DEIT_ASMJS_IN_DEBUGGING: DEBUG_EVENT_INFO_TYPE = 1i32;
pub const DEIT_ASMJS_SUCCEEDED: DEBUG_EVENT_INFO_TYPE = 2i32;
pub const DEIT_GENERAL: DEBUG_EVENT_INFO_TYPE = 0i32;
pub const DOCUMENTNAMETYPE_APPNODE: DOCUMENTNAMETYPE = 0i32;
pub const DOCUMENTNAMETYPE_FILE_TAIL: DOCUMENTNAMETYPE = 2i32;
pub const DOCUMENTNAMETYPE_SOURCE_MAP_URL: DOCUMENTNAMETYPE = 5i32;
pub const DOCUMENTNAMETYPE_TITLE: DOCUMENTNAMETYPE = 1i32;
pub const DOCUMENTNAMETYPE_UNIQUE_TITLE: DOCUMENTNAMETYPE = 4i32;
pub const DOCUMENTNAMETYPE_URL: DOCUMENTNAMETYPE = 3i32;
pub const DST_INTERNAL_FRAME: DEBUG_STACKFRAME_TYPE = 1i32;
pub const DST_INVOCATION_FRAME: DEBUG_STACKFRAME_TYPE = 2i32;
pub const DST_SCRIPT_FRAME: DEBUG_STACKFRAME_TYPE = 0i32;
pub const ERRORRESUMEACTION_AbortCallAndReturnErrorToCaller: ERRORRESUMEACTION = 1i32;
pub const ERRORRESUMEACTION_ReexecuteErrorStatement: ERRORRESUMEACTION = 0i32;
pub const ERRORRESUMEACTION_SkipErrorStatement: ERRORRESUMEACTION = 2i32;
pub const ETK_FIRST_CHANCE: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = 0i32;
pub const ETK_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = 2i32;
pub const ETK_USER_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = 1i32;
pub const E_JsDEBUG_INVALID_MEMORY_ADDRESS: windows_core::HRESULT = 0x8DC70005_u32 as _;
pub const E_JsDEBUG_MISMATCHED_RUNTIME: windows_core::HRESULT = 0x8DC70001_u32 as _;
pub const E_JsDEBUG_OUTSIDE_OF_VM: windows_core::HRESULT = 0x8DC70004_u32 as _;
pub const E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE: windows_core::HRESULT = 0x8DC70007_u32 as _;
pub const E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND: windows_core::HRESULT = 0x8DC70006_u32 as _;
pub const E_JsDEBUG_UNKNOWN_THREAD: windows_core::HRESULT = 0x8DC70002_u32 as _;
pub const FACILITY_JsDEBUG: u32 = 3527u32;
pub const FILTER_EXCLUDE_ANONYMOUS_CODE: APPLICATION_NODE_EVENT_FILTER = 1i32;
pub const FILTER_EXCLUDE_EVAL_CODE: APPLICATION_NODE_EVENT_FILTER = 2i32;
pub const FILTER_EXCLUDE_NOTHING: APPLICATION_NODE_EVENT_FILTER = 0i32;
pub const GETATTRFLAG_HUMANTEXT: u32 = 32768u32;
pub const GETATTRFLAG_THIS: u32 = 256u32;
pub const GETATTRTYPE_DEPSCAN: u32 = 1u32;
pub const GETATTRTYPE_NORMAL: u32 = 0u32;
pub const JS_PROPERTY_ATTRIBUTE_NONE: JS_PROPERTY_ATTRIBUTES = 0i32;
pub const JS_PROPERTY_FAKE: JS_PROPERTY_ATTRIBUTES = 2i32;
pub const JS_PROPERTY_FRAME_INCATCHBLOCK: JS_PROPERTY_ATTRIBUTES = 64i32;
pub const JS_PROPERTY_FRAME_INFINALLYBLOCK: JS_PROPERTY_ATTRIBUTES = 128i32;
pub const JS_PROPERTY_FRAME_INTRYBLOCK: JS_PROPERTY_ATTRIBUTES = 32i32;
pub const JS_PROPERTY_HAS_CHILDREN: JS_PROPERTY_ATTRIBUTES = 1i32;
pub const JS_PROPERTY_MEMBERS_ALL: JS_PROPERTY_MEMBERS = 0i32;
pub const JS_PROPERTY_MEMBERS_ARGUMENTS: JS_PROPERTY_MEMBERS = 1i32;
pub const JS_PROPERTY_METHOD: JS_PROPERTY_ATTRIBUTES = 4i32;
pub const JS_PROPERTY_NATIVE_WINRT_POINTER: JS_PROPERTY_ATTRIBUTES = 16i32;
pub const JS_PROPERTY_READONLY: JS_PROPERTY_ATTRIBUTES = 8i32;
pub const OID_JSSIP: windows_core::GUID = windows_core::GUID::from_u128(0x06c9e010_38ce_11d4_a2a3_00104bd35090);
pub const OID_VBSSIP: windows_core::GUID = windows_core::GUID::from_u128(0x1629f04e_2799_4db5_8fe5_ace10f17ebab);
pub const OID_WSFSIP: windows_core::GUID = windows_core::GUID::from_u128(0x1a610570_38ce_11d4_a2a3_00104bd35090);
pub const PROFILER_EVENT_MASK_TRACE_ALL: PROFILER_EVENT_MASK = 3i32;
pub const PROFILER_EVENT_MASK_TRACE_ALL_WITH_DOM: PROFILER_EVENT_MASK = 7i32;
pub const PROFILER_EVENT_MASK_TRACE_DOM_FUNCTION_CALL: PROFILER_EVENT_MASK = 4i32;
pub const PROFILER_EVENT_MASK_TRACE_NATIVE_FUNCTION_CALL: PROFILER_EVENT_MASK = 2i32;
pub const PROFILER_EVENT_MASK_TRACE_SCRIPT_FUNCTION_CALL: PROFILER_EVENT_MASK = 1i32;
pub const PROFILER_HEAP_ENUM_FLAGS_NONE: PROFILER_HEAP_ENUM_FLAGS = 0i32;
pub const PROFILER_HEAP_ENUM_FLAGS_RELATIONSHIP_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = 3i32;
pub const PROFILER_HEAP_ENUM_FLAGS_STORE_RELATIONSHIP_FLAGS: PROFILER_HEAP_ENUM_FLAGS = 1i32;
pub const PROFILER_HEAP_ENUM_FLAGS_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = 2i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL: PROFILER_HEAP_OBJECT_FLAGS = 8i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_DISPATCH: PROFILER_HEAP_OBJECT_FLAGS = 32i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_UNKNOWN: PROFILER_HEAP_OBJECT_FLAGS = 16i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_IS_ROOT: PROFILER_HEAP_OBJECT_FLAGS = 2i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_OBJECT: PROFILER_HEAP_OBJECT_FLAGS = 1i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_STATE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = 256i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_SITE_CLOSED: PROFILER_HEAP_OBJECT_FLAGS = 4i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_APPROXIMATE: PROFILER_HEAP_OBJECT_FLAGS = 64i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = 128i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_DELEGATE: PROFILER_HEAP_OBJECT_FLAGS = 2048i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_INSTANCE: PROFILER_HEAP_OBJECT_FLAGS = 512i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_NAMESPACE: PROFILER_HEAP_OBJECT_FLAGS = 4096i32;
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_RUNTIMECLASS: PROFILER_HEAP_OBJECT_FLAGS = 1024i32;
pub const PROFILER_HEAP_OBJECT_NAME_ID_UNAVAILABLE: u32 = 4294967295u32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_ATTRIBUTES_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 7i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_TEXT_CHILDREN_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 8i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_FUNCTION_NAME: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 2i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INDEX_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 6i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INTERNAL_PROPERTY: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 4i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 12i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAX_VALUE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 13i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_NAME_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 5i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_PROTOTYPE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 1i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_RELATIONSHIPS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 9i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SCOPE_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 3i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SET_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 13i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WEAKMAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 11i32;
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WINRTEVENTS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = 10i32;
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_CONST_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = 524288i32;
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_GET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = 65536i32;
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_SET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = 131072i32;
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_LET_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = 262144i32;
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_NONE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = 0i32;
pub const PROFILER_HEAP_SUMMARY_VERSION_1: PROFILER_HEAP_SUMMARY_VERSION = 1i32;
pub const PROFILER_PROPERTY_TYPE_BSTR: PROFILER_RELATIONSHIP_INFO = 5i32;
pub const PROFILER_PROPERTY_TYPE_EXTERNAL_OBJECT: PROFILER_RELATIONSHIP_INFO = 4i32;
pub const PROFILER_PROPERTY_TYPE_HEAP_OBJECT: PROFILER_RELATIONSHIP_INFO = 3i32;
pub const PROFILER_PROPERTY_TYPE_NUMBER: PROFILER_RELATIONSHIP_INFO = 1i32;
pub const PROFILER_PROPERTY_TYPE_STRING: PROFILER_RELATIONSHIP_INFO = 2i32;
pub const PROFILER_PROPERTY_TYPE_SUBSTRING: PROFILER_RELATIONSHIP_INFO = 6i32;
pub const PROFILER_SCRIPT_TYPE_DOM: PROFILER_SCRIPT_TYPE = 3i32;
pub const PROFILER_SCRIPT_TYPE_DYNAMIC: PROFILER_SCRIPT_TYPE = 1i32;
pub const PROFILER_SCRIPT_TYPE_NATIVE: PROFILER_SCRIPT_TYPE = 2i32;
pub const PROFILER_SCRIPT_TYPE_USER: PROFILER_SCRIPT_TYPE = 0i32;
pub const SCRIPTGCTYPE_EXHAUSTIVE: SCRIPTGCTYPE = 1i32;
pub const SCRIPTGCTYPE_NORMAL: SCRIPTGCTYPE = 0i32;
pub const SCRIPTINFO_ITYPEINFO: u32 = 2u32;
pub const SCRIPTINFO_IUNKNOWN: u32 = 1u32;
pub const SCRIPTINTERRUPT_DEBUG: u32 = 1u32;
pub const SCRIPTINTERRUPT_RAISEEXCEPTION: u32 = 2u32;
pub const SCRIPTITEM_CODEONLY: u32 = 512u32;
pub const SCRIPTITEM_GLOBALMEMBERS: u32 = 8u32;
pub const SCRIPTITEM_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTITEM_ISSOURCE: u32 = 4u32;
pub const SCRIPTITEM_ISVISIBLE: u32 = 2u32;
pub const SCRIPTITEM_NOCODE: u32 = 1024u32;
pub const SCRIPTLANGUAGEVERSION_5_7: SCRIPTLANGUAGEVERSION = 1i32;
pub const SCRIPTLANGUAGEVERSION_5_8: SCRIPTLANGUAGEVERSION = 2i32;
pub const SCRIPTLANGUAGEVERSION_DEFAULT: SCRIPTLANGUAGEVERSION = 0i32;
pub const SCRIPTLANGUAGEVERSION_MAX: SCRIPTLANGUAGEVERSION = 255i32;
pub const SCRIPTPROC_HOSTMANAGESSOURCE: u32 = 128u32;
pub const SCRIPTPROC_IMPLICIT_PARENTS: u32 = 512u32;
pub const SCRIPTPROC_IMPLICIT_THIS: u32 = 256u32;
pub const SCRIPTPROC_ISEXPRESSION: u32 = 32u32;
pub const SCRIPTPROC_ISXDOMAIN: u32 = 1024u32;
pub const SCRIPTPROP_ABBREVIATE_GLOBALNAME_RESOLUTION: u32 = 1879048194u32;
pub const SCRIPTPROP_BUILDNUMBER: u32 = 3u32;
pub const SCRIPTPROP_CATCHEXCEPTION: u32 = 4097u32;
pub const SCRIPTPROP_CONVERSIONLCID: u32 = 4098u32;
pub const SCRIPTPROP_DEBUGGER: u32 = 4352u32;
pub const SCRIPTPROP_DELAYEDEVENTSINKING: u32 = 4096u32;
pub const SCRIPTPROP_GCCONTROLSOFTCLOSE: u32 = 8192u32;
pub const SCRIPTPROP_HACK_FIBERSUPPORT: u32 = 1879048192u32;
pub const SCRIPTPROP_HACK_TRIDENTEVENTSINK: u32 = 1879048193u32;
pub const SCRIPTPROP_HOSTKEEPALIVE: u32 = 1879048196u32;
pub const SCRIPTPROP_HOSTSTACKREQUIRED: u32 = 4099u32;
pub const SCRIPTPROP_INTEGERMODE: u32 = 12288u32;
pub const SCRIPTPROP_INVOKEVERSIONING: u32 = 16384u32;
pub const SCRIPTPROP_JITDEBUG: u32 = 4353u32;
pub const SCRIPTPROP_MAJORVERSION: u32 = 1u32;
pub const SCRIPTPROP_MINORVERSION: u32 = 2u32;
pub const SCRIPTPROP_NAME: u32 = 0u32;
pub const SCRIPTPROP_SCRIPTSAREFULLYTRUSTED: u32 = 4100u32;
pub const SCRIPTPROP_STRINGCOMPAREINSTANCE: u32 = 12289u32;
pub const SCRIPTSTATE_CLOSED: SCRIPTSTATE = 4i32;
pub const SCRIPTSTATE_CONNECTED: SCRIPTSTATE = 2i32;
pub const SCRIPTSTATE_DISCONNECTED: SCRIPTSTATE = 3i32;
pub const SCRIPTSTATE_INITIALIZED: SCRIPTSTATE = 5i32;
pub const SCRIPTSTATE_STARTED: SCRIPTSTATE = 1i32;
pub const SCRIPTSTATE_UNINITIALIZED: SCRIPTSTATE = 0i32;
pub const SCRIPTSTAT_INSTRUCTION_COUNT: u32 = 2u32;
pub const SCRIPTSTAT_INTSTRUCTION_TIME: u32 = 3u32;
pub const SCRIPTSTAT_STATEMENT_COUNT: u32 = 1u32;
pub const SCRIPTSTAT_TOTAL_TIME: u32 = 4u32;
pub const SCRIPTTEXT_DELAYEXECUTION: u32 = 1u32;
pub const SCRIPTTEXT_HOSTMANAGESSOURCE: u32 = 128u32;
pub const SCRIPTTEXT_ISEXPRESSION: u32 = 32u32;
pub const SCRIPTTEXT_ISNONUSERCODE: u32 = 512u32;
pub const SCRIPTTEXT_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTTEXT_ISVISIBLE: u32 = 2u32;
pub const SCRIPTTEXT_ISXDOMAIN: u32 = 256u32;
pub const SCRIPTTHREADSTATE_NOTINSCRIPT: SCRIPTTHREADSTATE = 0i32;
pub const SCRIPTTHREADSTATE_RUNNING: SCRIPTTHREADSTATE = 1i32;
pub const SCRIPTTRACEINFO_COMCALLEND: SCRIPTTRACEINFO = 3i32;
pub const SCRIPTTRACEINFO_COMCALLSTART: SCRIPTTRACEINFO = 2i32;
pub const SCRIPTTRACEINFO_CREATEOBJEND: SCRIPTTRACEINFO = 5i32;
pub const SCRIPTTRACEINFO_CREATEOBJSTART: SCRIPTTRACEINFO = 4i32;
pub const SCRIPTTRACEINFO_GETOBJEND: SCRIPTTRACEINFO = 7i32;
pub const SCRIPTTRACEINFO_GETOBJSTART: SCRIPTTRACEINFO = 6i32;
pub const SCRIPTTRACEINFO_SCRIPTEND: SCRIPTTRACEINFO = 1i32;
pub const SCRIPTTRACEINFO_SCRIPTSTART: SCRIPTTRACEINFO = 0i32;
pub const SCRIPTTYPELIB_ISCONTROL: u32 = 16u32;
pub const SCRIPTTYPELIB_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTUICHANDLING_ALLOW: SCRIPTUICHANDLING = 0i32;
pub const SCRIPTUICHANDLING_NOUIDEFAULT: SCRIPTUICHANDLING = 2i32;
pub const SCRIPTUICHANDLING_NOUIERROR: SCRIPTUICHANDLING = 1i32;
pub const SCRIPTUICITEM_INPUTBOX: SCRIPTUICITEM = 1i32;
pub const SCRIPTUICITEM_MSGBOX: SCRIPTUICITEM = 2i32;
pub const SCRIPT_CMPL_COMMIT: u32 = 4u32;
pub const SCRIPT_CMPL_ENUMLIST: u32 = 2u32;
pub const SCRIPT_CMPL_ENUM_TRIGGER: u32 = 1u32;
pub const SCRIPT_CMPL_GLOBALLIST: u32 = 8u32;
pub const SCRIPT_CMPL_MEMBERLIST: u32 = 1u32;
pub const SCRIPT_CMPL_MEMBER_TRIGGER: u32 = 2u32;
pub const SCRIPT_CMPL_NOLIST: u32 = 0u32;
pub const SCRIPT_CMPL_PARAMTIP: u32 = 4u32;
pub const SCRIPT_CMPL_PARAM_TRIGGER: u32 = 3u32;
pub const SCRIPT_ENCODE_DEFAULT_LANGUAGE: u32 = 1u32;
pub const SCRIPT_ENCODE_NO_ASP_LANGUAGE: u32 = 2u32;
pub const SCRIPT_ENCODE_SECTION: u32 = 1u32;
pub const SCRIPT_E_PROPAGATE: i32 = -2147352318i32;
pub const SCRIPT_E_RECORDED: i32 = -2040119292i32;
pub const SCRIPT_E_REPORTED: i32 = -2147352319i32;
pub const SDO_ENABLE_FIRST_CHANCE_EXCEPTIONS: SCRIPT_DEBUGGER_OPTIONS = 1i32;
pub const SDO_ENABLE_LIBRARY_STACK_FRAME: SCRIPT_DEBUGGER_OPTIONS = 8i32;
pub const SDO_ENABLE_NONUSER_CODE_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = 4i32;
pub const SDO_ENABLE_WEB_WORKER_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = 2i32;
pub const SDO_NONE: SCRIPT_DEBUGGER_OPTIONS = 0i32;
pub const SICT_Event: SCRIPT_INVOCATION_CONTEXT_TYPE = 0i32;
pub const SICT_MutationObserverCheckpoint: SCRIPT_INVOCATION_CONTEXT_TYPE = 6i32;
pub const SICT_RequestAnimationFrame: SCRIPT_INVOCATION_CONTEXT_TYPE = 4i32;
pub const SICT_SetImmediate: SCRIPT_INVOCATION_CONTEXT_TYPE = 3i32;
pub const SICT_SetInterval: SCRIPT_INVOCATION_CONTEXT_TYPE = 2i32;
pub const SICT_SetTimeout: SCRIPT_INVOCATION_CONTEXT_TYPE = 1i32;
pub const SICT_ToString: SCRIPT_INVOCATION_CONTEXT_TYPE = 5i32;
pub const SICT_WWAExecAtPriority: SCRIPT_INVOCATION_CONTEXT_TYPE = 8i32;
pub const SICT_WWAExecUnsafeLocalFunction: SCRIPT_INVOCATION_CONTEXT_TYPE = 7i32;
pub const SOURCETEXT_ATTR_COMMENT: u32 = 2u32;
pub const SOURCETEXT_ATTR_FUNCTION_START: u32 = 64u32;
pub const SOURCETEXT_ATTR_HUMANTEXT: u32 = 32768u32;
pub const SOURCETEXT_ATTR_IDENTIFIER: u32 = 256u32;
pub const SOURCETEXT_ATTR_KEYWORD: u32 = 1u32;
pub const SOURCETEXT_ATTR_MEMBERLOOKUP: u32 = 512u32;
pub const SOURCETEXT_ATTR_NONSOURCE: u32 = 4u32;
pub const SOURCETEXT_ATTR_NUMBER: u32 = 16u32;
pub const SOURCETEXT_ATTR_OPERATOR: u32 = 8u32;
pub const SOURCETEXT_ATTR_STRING: u32 = 32u32;
pub const SOURCETEXT_ATTR_THIS: u32 = 1024u32;
pub const TEXT_DOC_ATTR_READONLY: u32 = 1u32;
pub const TEXT_DOC_ATTR_TYPE_PRIMARY: u32 = 2u32;
pub const TEXT_DOC_ATTR_TYPE_SCRIPT: u32 = 8u32;
pub const TEXT_DOC_ATTR_TYPE_WORKER: u32 = 4u32;
pub const THREAD_BLOCKED: u32 = 4u32;
pub const THREAD_OUT_OF_CONTEXT: u32 = 8u32;
pub const THREAD_STATE_RUNNING: u32 = 1u32;
pub const THREAD_STATE_SUSPENDED: u32 = 2u32;
pub const fasaCaseSensitive: u32 = 4u32;
pub const fasaPreferInternalHandler: u32 = 1u32;
pub const fasaSupportInternalHandler: u32 = 2u32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APPLICATION_NODE_EVENT_FILTER(pub i32);
impl windows_core::TypeKind for APPLICATION_NODE_EVENT_FILTER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BREAKPOINT_STATE(pub i32);
impl windows_core::TypeKind for BREAKPOINT_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BREAKREASON(pub i32);
impl windows_core::TypeKind for BREAKREASON {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BREAKRESUMEACTION(pub i32);
impl windows_core::TypeKind for BREAKRESUMEACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEBUG_EVENT_INFO_TYPE(pub i32);
impl windows_core::TypeKind for DEBUG_EVENT_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DEBUG_STACKFRAME_TYPE(pub i32);
impl windows_core::TypeKind for DEBUG_STACKFRAME_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DOCUMENTNAMETYPE(pub i32);
impl windows_core::TypeKind for DOCUMENTNAMETYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ERRORRESUMEACTION(pub i32);
impl windows_core::TypeKind for ERRORRESUMEACTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JS_PROPERTY_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for JS_PROPERTY_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JS_PROPERTY_MEMBERS(pub i32);
impl windows_core::TypeKind for JS_PROPERTY_MEMBERS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct JsDebugReadMemoryFlags(pub i32);
impl JsDebugReadMemoryFlags {
    pub const None: Self = Self(0i32);
    pub const JsDebugAllowPartialRead: Self = Self(1i32);
}
impl windows_core::TypeKind for JsDebugReadMemoryFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_EVENT_MASK(pub i32);
impl windows_core::TypeKind for PROFILER_EVENT_MASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_HEAP_ENUM_FLAGS(pub i32);
impl windows_core::TypeKind for PROFILER_HEAP_ENUM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_FLAGS(pub i32);
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(pub i32);
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(pub i32);
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_HEAP_SUMMARY_VERSION(pub i32);
impl windows_core::TypeKind for PROFILER_HEAP_SUMMARY_VERSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_RELATIONSHIP_INFO(pub i32);
impl windows_core::TypeKind for PROFILER_RELATIONSHIP_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROFILER_SCRIPT_TYPE(pub i32);
impl windows_core::TypeKind for PROFILER_SCRIPT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTGCTYPE(pub i32);
impl windows_core::TypeKind for SCRIPTGCTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTLANGUAGEVERSION(pub i32);
impl windows_core::TypeKind for SCRIPTLANGUAGEVERSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTSTATE(pub i32);
impl windows_core::TypeKind for SCRIPTSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTTHREADSTATE(pub i32);
impl windows_core::TypeKind for SCRIPTTHREADSTATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTTRACEINFO(pub i32);
impl windows_core::TypeKind for SCRIPTTRACEINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTUICHANDLING(pub i32);
impl windows_core::TypeKind for SCRIPTUICHANDLING {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPTUICITEM(pub i32);
impl windows_core::TypeKind for SCRIPTUICITEM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPT_DEBUGGER_OPTIONS(pub i32);
impl windows_core::TypeKind for SCRIPT_DEBUGGER_OPTIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(pub i32);
impl windows_core::TypeKind for SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCRIPT_INVOCATION_CONTEXT_TYPE(pub i32);
impl windows_core::TypeKind for SCRIPT_INVOCATION_CONTEXT_TYPE {
    type TypeKind = windows_core::CopyType;
}
pub const CDebugDocumentHelper: windows_core::GUID = windows_core::GUID::from_u128(0x83b8bca6_687c_11d0_a405_00aa0060275c);
pub const DebugHelper: windows_core::GUID = windows_core::GUID::from_u128(0x0bfcc060_8c1d_11d0_accd_00aa0060275c);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugStackFrameDescriptor {
    pub pdsf: Option<IDebugStackFrame>,
    pub dwMin: u32,
    pub dwLim: u32,
    pub fFinal: super::super::super::super::Foundation::BOOL,
    pub punkFinal: Option<windows_core::IUnknown>,
}
impl Default for DebugStackFrameDescriptor {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DebugStackFrameDescriptor {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugStackFrameDescriptor64 {
    pub pdsf: Option<IDebugStackFrame>,
    pub dwMin: u64,
    pub dwLim: u64,
    pub fFinal: super::super::super::super::Foundation::BOOL,
    pub punkFinal: Option<windows_core::IUnknown>,
}
impl Default for DebugStackFrameDescriptor64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DebugStackFrameDescriptor64 {
    type TypeKind = windows_core::CloneType;
}
pub const DefaultDebugSessionProvider: windows_core::GUID = windows_core::GUID::from_u128(0x834128a2_51f4_11d0_8f20_00805f2cd064);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JS_NATIVE_FRAME {
    pub InstructionOffset: u64,
    pub ReturnOffset: u64,
    pub FrameOffset: u64,
    pub StackOffset: u64,
}
impl Default for JS_NATIVE_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for JS_NATIVE_FRAME {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JsDebugPropertyInfo {
    pub name: windows_core::BSTR,
    pub r#type: windows_core::BSTR,
    pub value: windows_core::BSTR,
    pub fullName: windows_core::BSTR,
    pub attr: JS_PROPERTY_ATTRIBUTES,
}
impl Default for JsDebugPropertyInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for JsDebugPropertyInfo {
    type TypeKind = windows_core::CopyType;
}
pub const MachineDebugManager_DEBUG: windows_core::GUID = windows_core::GUID::from_u128(0x49769cec_3a55_4bb0_b697_88fede77e8ea);
pub const MachineDebugManager_RETAIL: windows_core::GUID = windows_core::GUID::from_u128(0x0c0a3666_30c9_11d0_8f20_00805f2cd064);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT {
    pub size: u32,
    pub Anonymous: PROFILER_HEAP_OBJECT_0,
    pub typeNameId: u32,
    pub flags: u32,
    pub unused: u16,
    pub optionalInfoCount: u16,
}
impl Default for PROFILER_HEAP_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union PROFILER_HEAP_OBJECT_0 {
    pub objectId: usize,
    pub externalObjectAddress: *mut core::ffi::c_void,
}
impl Default for PROFILER_HEAP_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    pub infoType: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE,
    pub Anonymous: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0,
}
impl Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_OPTIONAL_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    pub prototype: usize,
    pub functionName: windows_core::PCWSTR,
    pub elementAttributesSize: u32,
    pub elementTextChildrenSize: u32,
    pub scopeList: *mut PROFILER_HEAP_OBJECT_SCOPE_LIST,
    pub internalProperty: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP,
    pub namePropertyList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub indexPropertyList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub relationshipList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub eventList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub weakMapCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub mapCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
    pub setCollectionList: *mut PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST,
}
impl Default for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_OPTIONAL_INFO_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP {
    pub relationshipId: u32,
    pub relationshipInfo: PROFILER_RELATIONSHIP_INFO,
    pub Anonymous: PROFILER_HEAP_OBJECT_RELATIONSHIP_0,
}
impl Default for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    pub numberValue: f64,
    pub stringValue: windows_core::PCWSTR,
    pub bstrValue: windows_core::BSTR,
    pub objectId: usize,
    pub externalObjectAddress: *mut core::ffi::c_void,
    pub subString: *mut PROFILER_PROPERTY_TYPE_SUBSTRING_INFO,
}
impl Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    pub count: u32,
    pub elements: [PROFILER_HEAP_OBJECT_RELATIONSHIP; 1],
}
impl Default for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_OBJECT_SCOPE_LIST {
    pub count: u32,
    pub scopes: [usize; 1],
}
impl Default for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_OBJECT_SCOPE_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_HEAP_SUMMARY {
    pub version: PROFILER_HEAP_SUMMARY_VERSION,
    pub totalHeapSize: u32,
}
impl Default for PROFILER_HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_HEAP_SUMMARY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    pub length: u32,
    pub value: windows_core::PCWSTR,
}
impl Default for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROFILER_PROPERTY_TYPE_SUBSTRING_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const ProcessDebugManager: windows_core::GUID = windows_core::GUID::from_u128(0x78a51822_51f4_11d0_8f20_00805f2cd064);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TEXT_DOCUMENT_ARRAY {
    pub dwCount: u32,
    pub Members: *mut Option<IDebugDocumentText>,
}
impl Default for TEXT_DOCUMENT_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TEXT_DOCUMENT_ARRAY {
    type TypeKind = windows_core::CopyType;
}
