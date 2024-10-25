#[cfg(feature = "Win32_System_Com_CallObj")]
pub mod CallObj;
#[cfg(feature = "Win32_System_Com_ChannelCredentials")]
pub mod ChannelCredentials;
#[cfg(feature = "Win32_System_Com_Events")]
pub mod Events;
#[cfg(feature = "Win32_System_Com_Marshal")]
pub mod Marshal;
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub mod StructuredStorage;
#[cfg(feature = "Win32_System_Com_UI")]
pub mod UI;
#[cfg(feature = "Win32_System_Com_Urlmon")]
pub mod Urlmon;
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16i32;
pub const ADVFCACHE_NOHANDLER: ADVF = 8i32;
pub const ADVFCACHE_ONSAVE: ADVF = 32i32;
pub const ADVF_DATAONSTOP: ADVF = 64i32;
pub const ADVF_NODATA: ADVF = 1i32;
pub const ADVF_ONLYONCE: ADVF = 4i32;
pub const ADVF_PRIMEFIRST: ADVF = 2i32;
pub const APPIDREGFLAGS_AAA_NO_IMPLICIT_ACTIVATE_AS_IU: u32 = 2048u32;
pub const APPIDREGFLAGS_ACTIVATE_IUSERVER_INDESKTOP: u32 = 1u32;
pub const APPIDREGFLAGS_ISSUE_ACTIVATION_RPC_AT_IDENTIFY: u32 = 4u32;
pub const APPIDREGFLAGS_IUSERVER_ACTIVATE_IN_CLIENT_SESSION_ONLY: u32 = 32u32;
pub const APPIDREGFLAGS_IUSERVER_SELF_SID_IN_LAUNCH_PERMISSION: u32 = 16u32;
pub const APPIDREGFLAGS_IUSERVER_UNMODIFIED_LOGON_TOKEN: u32 = 8u32;
pub const APPIDREGFLAGS_RESERVED1: u32 = 64u32;
pub const APPIDREGFLAGS_RESERVED2: u32 = 128u32;
pub const APPIDREGFLAGS_RESERVED3: u32 = 256u32;
pub const APPIDREGFLAGS_RESERVED4: u32 = 512u32;
pub const APPIDREGFLAGS_RESERVED5: u32 = 1024u32;
pub const APPIDREGFLAGS_RESERVED7: u32 = 4096u32;
pub const APPIDREGFLAGS_RESERVED8: u32 = 8192u32;
pub const APPIDREGFLAGS_RESERVED9: u32 = 16384u32;
pub const APPIDREGFLAGS_SECURE_SERVER_PROCESS_SD_AND_BIND: u32 = 2u32;
pub const APTTYPEQUALIFIER_APPLICATION_STA: APTTYPEQUALIFIER = 6i32;
pub const APTTYPEQUALIFIER_IMPLICIT_MTA: APTTYPEQUALIFIER = 1i32;
pub const APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: APTTYPEQUALIFIER = 4i32;
pub const APTTYPEQUALIFIER_NA_ON_MAINSTA: APTTYPEQUALIFIER = 5i32;
pub const APTTYPEQUALIFIER_NA_ON_MTA: APTTYPEQUALIFIER = 2i32;
pub const APTTYPEQUALIFIER_NA_ON_STA: APTTYPEQUALIFIER = 3i32;
pub const APTTYPEQUALIFIER_NONE: APTTYPEQUALIFIER = 0i32;
pub const APTTYPEQUALIFIER_RESERVED_1: APTTYPEQUALIFIER = 7i32;
pub const APTTYPE_CURRENT: APTTYPE = -1i32;
pub const APTTYPE_MAINSTA: APTTYPE = 3i32;
pub const APTTYPE_MTA: APTTYPE = 1i32;
pub const APTTYPE_NA: APTTYPE = 2i32;
pub const APTTYPE_STA: APTTYPE = 0i32;
pub const ASYNC_MODE_COMPATIBILITY: i32 = 1i32;
pub const ASYNC_MODE_DEFAULT: i32 = 0i32;
pub const BINDINFOF_URLENCODEDEXTRAINFO: BINDINFOF = 2i32;
pub const BINDINFOF_URLENCODESTGMEDDATA: BINDINFOF = 1i32;
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2i32;
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1i32;
pub const CALLTYPE_ASYNC: CALLTYPE = 3i32;
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = 5i32;
pub const CALLTYPE_NESTED: CALLTYPE = 2i32;
pub const CALLTYPE_TOPLEVEL: CALLTYPE = 1i32;
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = 4i32;
pub const CC_CDECL: CALLCONV = 1i32;
pub const CC_FASTCALL: CALLCONV = 0i32;
pub const CC_FPFASTCALL: CALLCONV = 5i32;
pub const CC_MACPASCAL: CALLCONV = 3i32;
pub const CC_MAX: CALLCONV = 9i32;
pub const CC_MPWCDECL: CALLCONV = 7i32;
pub const CC_MPWPASCAL: CALLCONV = 8i32;
pub const CC_MSCPASCAL: CALLCONV = 2i32;
pub const CC_PASCAL: CALLCONV = 2i32;
pub const CC_STDCALL: CALLCONV = 4i32;
pub const CC_SYSCALL: CALLCONV = 6i32;
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144u32;
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288u32;
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608u32;
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432u32;
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144u32;
pub const CLSCTX_ALL: CLSCTX = 23u32;
pub const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: CLSCTX = 67108864u32;
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304u32;
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768u32;
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536u32;
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576u32;
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192u32;
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072u32;
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2u32;
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32u32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1u32;
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8u32;
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4u32;
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024u32;
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096u32;
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384u32;
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648u32;
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16u32;
pub const CLSCTX_RESERVED1: CLSCTX = 64u32;
pub const CLSCTX_RESERVED2: CLSCTX = 128u32;
pub const CLSCTX_RESERVED3: CLSCTX = 256u32;
pub const CLSCTX_RESERVED4: CLSCTX = 512u32;
pub const CLSCTX_RESERVED5: CLSCTX = 2048u32;
pub const CLSCTX_RESERVED6: CLSCTX = 16777216u32;
pub const CLSCTX_SERVER: CLSCTX = 21u32;
pub const CLSID_GlobalOptions: windows_core::GUID = windows_core::GUID::from_u128(0x0000034b_0000_0000_c000_000000000046);
pub const COINITBASE_MULTITHREADED: COINITBASE = 0i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2i32;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4i32;
pub const COINIT_MULTITHREADED: COINIT = 0i32;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8i32;
pub const COLE_DEFAULT_AUTHINFO: i32 = -1i32;
pub const COLE_DEFAULT_PRINCIPAL: windows_core::PCWSTR = -1i32 as _;
pub const COMBND_RESERVED1: RPCOPT_PROPERTIES = 4i32;
pub const COMBND_RESERVED2: RPCOPT_PROPERTIES = 5i32;
pub const COMBND_RESERVED3: RPCOPT_PROPERTIES = 8i32;
pub const COMBND_RESERVED4: RPCOPT_PROPERTIES = 16i32;
pub const COMBND_RPCTIMEOUT: RPCOPT_PROPERTIES = 1i32;
pub const COMBND_SERVER_LOCALITY: RPCOPT_PROPERTIES = 2i32;
pub const COMGLB_APPID: GLOBALOPT_PROPERTIES = 2i32;
pub const COMGLB_EXCEPTION_DONOT_HANDLE: GLOBALOPT_EH_VALUES = 1i32;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_ANY: GLOBALOPT_EH_VALUES = 2i32;
pub const COMGLB_EXCEPTION_DONOT_HANDLE_FATAL: GLOBALOPT_EH_VALUES = 1i32;
pub const COMGLB_EXCEPTION_HANDLE: GLOBALOPT_EH_VALUES = 0i32;
pub const COMGLB_EXCEPTION_HANDLING: GLOBALOPT_PROPERTIES = 1i32;
pub const COMGLB_FAST_RUNDOWN: GLOBALOPT_RO_FLAGS = 8i32;
pub const COMGLB_PROPERTIES_RESERVED1: GLOBALOPT_PROPERTIES = 6i32;
pub const COMGLB_PROPERTIES_RESERVED2: GLOBALOPT_PROPERTIES = 7i32;
pub const COMGLB_PROPERTIES_RESERVED3: GLOBALOPT_PROPERTIES = 8i32;
pub const COMGLB_RESERVED1: GLOBALOPT_RO_FLAGS = 16i32;
pub const COMGLB_RESERVED2: GLOBALOPT_RO_FLAGS = 32i32;
pub const COMGLB_RESERVED3: GLOBALOPT_RO_FLAGS = 64i32;
pub const COMGLB_RESERVED4: GLOBALOPT_RO_FLAGS = 256i32;
pub const COMGLB_RESERVED5: GLOBALOPT_RO_FLAGS = 512i32;
pub const COMGLB_RESERVED6: GLOBALOPT_RO_FLAGS = 1024i32;
pub const COMGLB_RO_SETTINGS: GLOBALOPT_PROPERTIES = 4i32;
pub const COMGLB_RPC_THREADPOOL_SETTING: GLOBALOPT_PROPERTIES = 3i32;
pub const COMGLB_RPC_THREADPOOL_SETTING_DEFAULT_POOL: GLOBALOPT_RPCTP_VALUES = 0i32;
pub const COMGLB_RPC_THREADPOOL_SETTING_PRIVATE_POOL: GLOBALOPT_RPCTP_VALUES = 1i32;
pub const COMGLB_STA_MODALLOOP_REMOVE_TOUCH_MESSAGES: GLOBALOPT_RO_FLAGS = 1i32;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_DONOT_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 4i32;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REMOVE_INPUT_MESSAGES: GLOBALOPT_RO_FLAGS = 2i32;
pub const COMGLB_STA_MODALLOOP_SHARED_QUEUE_REORDER_POINTER_MESSAGES: GLOBALOPT_RO_FLAGS = 128i32;
pub const COMGLB_UNMARSHALING_POLICY: GLOBALOPT_PROPERTIES = 5i32;
pub const COMGLB_UNMARSHALING_POLICY_HYBRID: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 2i32;
pub const COMGLB_UNMARSHALING_POLICY_NORMAL: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 0i32;
pub const COMGLB_UNMARSHALING_POLICY_STRONG: GLOBALOPT_UNMARSHALING_POLICY_VALUES = 1i32;
pub const COM_RIGHTS_ACTIVATE_LOCAL: u32 = 8u32;
pub const COM_RIGHTS_ACTIVATE_REMOTE: u32 = 16u32;
pub const COM_RIGHTS_EXECUTE: u32 = 1u32;
pub const COM_RIGHTS_EXECUTE_LOCAL: u32 = 2u32;
pub const COM_RIGHTS_EXECUTE_REMOTE: u32 = 4u32;
pub const COM_RIGHTS_RESERVED1: u32 = 32u32;
pub const COM_RIGHTS_RESERVED2: u32 = 64u32;
pub const COWAIT_ALERTABLE: COWAIT_FLAGS = 2i32;
pub const COWAIT_DEFAULT: COWAIT_FLAGS = 0i32;
pub const COWAIT_DISPATCH_CALLS: COWAIT_FLAGS = 8i32;
pub const COWAIT_DISPATCH_WINDOW_MESSAGES: COWAIT_FLAGS = 16i32;
pub const COWAIT_INPUTAVAILABLE: COWAIT_FLAGS = 4i32;
pub const COWAIT_WAITALL: COWAIT_FLAGS = 1i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_1: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483648i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_10: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483639i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_11: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483638i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_12: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483637i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_13: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483636i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_14: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483635i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_15: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483634i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_16: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483633i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_17: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483632i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_18: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483631i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_2: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483647i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_3: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483646i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_4: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483645i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_5: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483644i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_6: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483643i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_7: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483642i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_8: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483641i32;
pub const CO_MARSHALING_CONTEXT_ATTRIBUTE_RESERVED_9: CO_MARSHALING_CONTEXT_ATTRIBUTES = -2147483640i32;
pub const CO_MARSHALING_SOURCE_IS_APP_CONTAINER: CO_MARSHALING_CONTEXT_ATTRIBUTES = 0i32;
pub const CWMO_DEFAULT: CWMO_FLAGS = 0i32;
pub const CWMO_DISPATCH_CALLS: CWMO_FLAGS = 1i32;
pub const CWMO_DISPATCH_WINDOW_MESSAGES: CWMO_FLAGS = 2i32;
pub const CWMO_MAX_HANDLES: u32 = 56u32;
pub const DATADIR_GET: DATADIR = 1i32;
pub const DATADIR_SET: DATADIR = 2i32;
pub const DCOMSCM_ACTIVATION_DISALLOW_UNSECURE_CALL: u32 = 2u32;
pub const DCOMSCM_ACTIVATION_USE_ALL_AUTHNSERVICES: u32 = 1u32;
pub const DCOMSCM_PING_DISALLOW_UNSECURE_CALL: u32 = 32u32;
pub const DCOMSCM_PING_USE_MID_AUTHNSERVICE: u32 = 16u32;
pub const DCOMSCM_RESOLVE_DISALLOW_UNSECURE_CALL: u32 = 8u32;
pub const DCOMSCM_RESOLVE_USE_ALL_AUTHNSERVICES: u32 = 4u32;
pub const DCOM_CALL_CANCELED: DCOM_CALL_STATE = 2i32;
pub const DCOM_CALL_COMPLETE: DCOM_CALL_STATE = 1i32;
pub const DCOM_NONE: DCOM_CALL_STATE = 0i32;
pub const DESCKIND_FUNCDESC: DESCKIND = 1i32;
pub const DESCKIND_IMPLICITAPPOBJ: DESCKIND = 4i32;
pub const DESCKIND_MAX: DESCKIND = 5i32;
pub const DESCKIND_NONE: DESCKIND = 0i32;
pub const DESCKIND_TYPECOMP: DESCKIND = 3i32;
pub const DESCKIND_VARDESC: DESCKIND = 2i32;
pub const DISPATCH_METHOD: DISPATCH_FLAGS = 1u16;
pub const DISPATCH_PROPERTYGET: DISPATCH_FLAGS = 2u16;
pub const DISPATCH_PROPERTYPUT: DISPATCH_FLAGS = 4u16;
pub const DISPATCH_PROPERTYPUTREF: DISPATCH_FLAGS = 8u16;
pub const DMUS_ERRBASE: u32 = 4096u32;
pub const DVASPECT_CONTENT: DVASPECT = 1u32;
pub const DVASPECT_DOCPRINT: DVASPECT = 8u32;
pub const DVASPECT_ICON: DVASPECT = 4u32;
pub const DVASPECT_OPAQUE: DVASPECT = 16u32;
pub const DVASPECT_THUMBNAIL: DVASPECT = 2u32;
pub const DVASPECT_TRANSPARENT: DVASPECT = 32u32;
pub const EOAC_ACCESS_CONTROL: EOLE_AUTHENTICATION_CAPABILITIES = 4i32;
pub const EOAC_ANY_AUTHORITY: EOLE_AUTHENTICATION_CAPABILITIES = 128i32;
pub const EOAC_APPID: EOLE_AUTHENTICATION_CAPABILITIES = 8i32;
pub const EOAC_AUTO_IMPERSONATE: EOLE_AUTHENTICATION_CAPABILITIES = 1024i32;
pub const EOAC_DEFAULT: EOLE_AUTHENTICATION_CAPABILITIES = 2048i32;
pub const EOAC_DISABLE_AAA: EOLE_AUTHENTICATION_CAPABILITIES = 4096i32;
pub const EOAC_DYNAMIC: EOLE_AUTHENTICATION_CAPABILITIES = 16i32;
pub const EOAC_DYNAMIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 64i32;
pub const EOAC_MAKE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 256i32;
pub const EOAC_MUTUAL_AUTH: EOLE_AUTHENTICATION_CAPABILITIES = 1i32;
pub const EOAC_NONE: EOLE_AUTHENTICATION_CAPABILITIES = 0i32;
pub const EOAC_NO_CUSTOM_MARSHAL: EOLE_AUTHENTICATION_CAPABILITIES = 8192i32;
pub const EOAC_REQUIRE_FULLSIC: EOLE_AUTHENTICATION_CAPABILITIES = 512i32;
pub const EOAC_RESERVED1: EOLE_AUTHENTICATION_CAPABILITIES = 16384i32;
pub const EOAC_SECURE_REFS: EOLE_AUTHENTICATION_CAPABILITIES = 2i32;
pub const EOAC_STATIC_CLOAKING: EOLE_AUTHENTICATION_CAPABILITIES = 32i32;
pub const EXTCONN_CALLABLE: EXTCONN = 4i32;
pub const EXTCONN_STRONG: EXTCONN = 1i32;
pub const EXTCONN_WEAK: EXTCONN = 2i32;
pub const FADF_AUTO: ADVANCED_FEATURE_FLAGS = 1u16;
pub const FADF_BSTR: ADVANCED_FEATURE_FLAGS = 256u16;
pub const FADF_DISPATCH: ADVANCED_FEATURE_FLAGS = 1024u16;
pub const FADF_EMBEDDED: ADVANCED_FEATURE_FLAGS = 4u16;
pub const FADF_FIXEDSIZE: ADVANCED_FEATURE_FLAGS = 16u16;
pub const FADF_HAVEIID: ADVANCED_FEATURE_FLAGS = 64u16;
pub const FADF_HAVEVARTYPE: ADVANCED_FEATURE_FLAGS = 128u16;
pub const FADF_RECORD: ADVANCED_FEATURE_FLAGS = 32u16;
pub const FADF_RESERVED: ADVANCED_FEATURE_FLAGS = 61448u16;
pub const FADF_STATIC: ADVANCED_FEATURE_FLAGS = 2u16;
pub const FADF_UNKNOWN: ADVANCED_FEATURE_FLAGS = 512u16;
pub const FADF_VARIANT: ADVANCED_FEATURE_FLAGS = 2048u16;
pub const FUNCFLAG_FBINDABLE: FUNCFLAGS = 4u16;
pub const FUNCFLAG_FDEFAULTBIND: FUNCFLAGS = 32u16;
pub const FUNCFLAG_FDEFAULTCOLLELEM: FUNCFLAGS = 256u16;
pub const FUNCFLAG_FDISPLAYBIND: FUNCFLAGS = 16u16;
pub const FUNCFLAG_FHIDDEN: FUNCFLAGS = 64u16;
pub const FUNCFLAG_FIMMEDIATEBIND: FUNCFLAGS = 4096u16;
pub const FUNCFLAG_FNONBROWSABLE: FUNCFLAGS = 1024u16;
pub const FUNCFLAG_FREPLACEABLE: FUNCFLAGS = 2048u16;
pub const FUNCFLAG_FREQUESTEDIT: FUNCFLAGS = 8u16;
pub const FUNCFLAG_FRESTRICTED: FUNCFLAGS = 1u16;
pub const FUNCFLAG_FSOURCE: FUNCFLAGS = 2u16;
pub const FUNCFLAG_FUIDEFAULT: FUNCFLAGS = 512u16;
pub const FUNCFLAG_FUSESGETLASTERROR: FUNCFLAGS = 128u16;
pub const FUNC_DISPATCH: FUNCKIND = 4i32;
pub const FUNC_NONVIRTUAL: FUNCKIND = 2i32;
pub const FUNC_PUREVIRTUAL: FUNCKIND = 1i32;
pub const FUNC_STATIC: FUNCKIND = 3i32;
pub const FUNC_VIRTUAL: FUNCKIND = 0i32;
pub const ForcedShutdown: ShutdownType = 1i32;
pub const IDLFLAG_FIN: IDLFLAGS = 1u16;
pub const IDLFLAG_FLCID: IDLFLAGS = 4u16;
pub const IDLFLAG_FOUT: IDLFLAGS = 2u16;
pub const IDLFLAG_FRETVAL: IDLFLAGS = 8u16;
pub const IDLFLAG_NONE: IDLFLAGS = 0u16;
pub const IMPLTYPEFLAG_FDEFAULT: IMPLTYPEFLAGS = 1i32;
pub const IMPLTYPEFLAG_FDEFAULTVTABLE: IMPLTYPEFLAGS = 8i32;
pub const IMPLTYPEFLAG_FRESTRICTED: IMPLTYPEFLAGS = 4i32;
pub const IMPLTYPEFLAG_FSOURCE: IMPLTYPEFLAGS = 2i32;
pub const INVOKE_FUNC: INVOKEKIND = 1i32;
pub const INVOKE_PROPERTYGET: INVOKEKIND = 2i32;
pub const INVOKE_PROPERTYPUT: INVOKEKIND = 4i32;
pub const INVOKE_PROPERTYPUTREF: INVOKEKIND = 8i32;
pub const IdleShutdown: ShutdownType = 0i32;
pub const LOCK_EXCLUSIVE: LOCKTYPE = 2i32;
pub const LOCK_ONLYONCE: LOCKTYPE = 4i32;
pub const LOCK_WRITE: LOCKTYPE = 1i32;
pub const LibraryApplication: ApplicationType = 1i32;
pub const MARSHALINTERFACE_MIN: u32 = 500u32;
pub const MAXLSN: u64 = 9223372036854775807u64;
pub const MEMCTX_MACSYSTEM: MEMCTX = 3i32;
pub const MEMCTX_SAME: MEMCTX = -2i32;
pub const MEMCTX_SHARED: MEMCTX = 2i32;
pub const MEMCTX_TASK: MEMCTX = 1i32;
pub const MEMCTX_UNKNOWN: MEMCTX = -1i32;
pub const MKRREDUCE_ALL: MKRREDUCE = 0i32;
pub const MKRREDUCE_ONE: MKRREDUCE = 196608i32;
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = 65536i32;
pub const MKRREDUCE_TOUSER: MKRREDUCE = 131072i32;
pub const MKSYS_ANTIMONIKER: MKSYS = 3i32;
pub const MKSYS_CLASSMONIKER: MKSYS = 7i32;
pub const MKSYS_FILEMONIKER: MKSYS = 2i32;
pub const MKSYS_GENERICCOMPOSITE: MKSYS = 1i32;
pub const MKSYS_ITEMMONIKER: MKSYS = 4i32;
pub const MKSYS_LUAMONIKER: MKSYS = 10i32;
pub const MKSYS_NONE: MKSYS = 0i32;
pub const MKSYS_OBJREFMONIKER: MKSYS = 8i32;
pub const MKSYS_POINTERMONIKER: MKSYS = 5i32;
pub const MKSYS_SESSIONMONIKER: MKSYS = 9i32;
pub const MSHCTX_CONTAINER: MSHCTX = 5i32;
pub const MSHCTX_CROSSCTX: MSHCTX = 4i32;
pub const MSHCTX_DIFFERENTMACHINE: MSHCTX = 2i32;
pub const MSHCTX_INPROC: MSHCTX = 3i32;
pub const MSHCTX_LOCAL: MSHCTX = 0i32;
pub const MSHCTX_NOSHAREDMEM: MSHCTX = 1i32;
pub const MSHLFLAGS_NOPING: MSHLFLAGS = 4i32;
pub const MSHLFLAGS_NORMAL: MSHLFLAGS = 0i32;
pub const MSHLFLAGS_RESERVED1: MSHLFLAGS = 8i32;
pub const MSHLFLAGS_RESERVED2: MSHLFLAGS = 16i32;
pub const MSHLFLAGS_RESERVED3: MSHLFLAGS = 32i32;
pub const MSHLFLAGS_RESERVED4: MSHLFLAGS = 64i32;
pub const MSHLFLAGS_TABLESTRONG: MSHLFLAGS = 1i32;
pub const MSHLFLAGS_TABLEWEAK: MSHLFLAGS = 2i32;
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = 0i32;
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = 2i32;
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = 1i32;
pub const PENDINGTYPE_NESTED: PENDINGTYPE = 2i32;
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = 1i32;
pub const REGCLS_AGILE: REGCLS = 16i32;
pub const REGCLS_MULTIPLEUSE: REGCLS = 1i32;
pub const REGCLS_MULTI_SEPARATE: REGCLS = 2i32;
pub const REGCLS_SINGLEUSE: REGCLS = 0i32;
pub const REGCLS_SURROGATE: REGCLS = 8i32;
pub const REGCLS_SUSPENDED: REGCLS = 4i32;
pub const ROTFLAGS_ALLOWANYCLIENT: ROT_FLAGS = 2u32;
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: ROT_FLAGS = 1u32;
pub const ROTREGFLAGS_ALLOWANYCLIENT: u32 = 1u32;
pub const RPC_C_AUTHN_LEVEL_CALL: RPC_C_AUTHN_LEVEL = 3u32;
pub const RPC_C_AUTHN_LEVEL_CONNECT: RPC_C_AUTHN_LEVEL = 2u32;
pub const RPC_C_AUTHN_LEVEL_DEFAULT: RPC_C_AUTHN_LEVEL = 0u32;
pub const RPC_C_AUTHN_LEVEL_NONE: RPC_C_AUTHN_LEVEL = 1u32;
pub const RPC_C_AUTHN_LEVEL_PKT: RPC_C_AUTHN_LEVEL = 4u32;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: RPC_C_AUTHN_LEVEL = 5u32;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: RPC_C_AUTHN_LEVEL = 6u32;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: RPC_C_IMP_LEVEL = 1u32;
pub const RPC_C_IMP_LEVEL_DEFAULT: RPC_C_IMP_LEVEL = 0u32;
pub const RPC_C_IMP_LEVEL_DELEGATE: RPC_C_IMP_LEVEL = 4u32;
pub const RPC_C_IMP_LEVEL_IDENTIFY: RPC_C_IMP_LEVEL = 2u32;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: RPC_C_IMP_LEVEL = 3u32;
pub const SD_ACCESSPERMISSIONS: COMSD = 1i32;
pub const SD_ACCESSRESTRICTIONS: COMSD = 3i32;
pub const SD_LAUNCHPERMISSIONS: COMSD = 0i32;
pub const SD_LAUNCHRESTRICTIONS: COMSD = 2i32;
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0i32;
pub const SERVERCALL_REJECTED: SERVERCALL = 1i32;
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2i32;
pub const SERVER_LOCALITY_MACHINE_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 1i32;
pub const SERVER_LOCALITY_PROCESS_LOCAL: RPCOPT_SERVER_LOCALITY_VALUES = 0i32;
pub const SERVER_LOCALITY_REMOTE: RPCOPT_SERVER_LOCALITY_VALUES = 2i32;
pub const STATFLAG_DEFAULT: STATFLAG = 0i32;
pub const STATFLAG_NONAME: STATFLAG = 1i32;
pub const STATFLAG_NOOPEN: STATFLAG = 2i32;
pub const STGC_CONSOLIDATE: STGC = 8i32;
pub const STGC_DANGEROUSLYCOMMITMERELYTODISKCACHE: STGC = 4i32;
pub const STGC_DEFAULT: STGC = 0i32;
pub const STGC_ONLYIFCURRENT: STGC = 2i32;
pub const STGC_OVERWRITE: STGC = 1i32;
pub const STGM_CONVERT: STGM = 131072u32;
pub const STGM_CREATE: STGM = 4096u32;
pub const STGM_DELETEONRELEASE: STGM = 67108864u32;
pub const STGM_DIRECT: STGM = 0u32;
pub const STGM_DIRECT_SWMR: STGM = 4194304u32;
pub const STGM_FAILIFTHERE: STGM = 0u32;
pub const STGM_NOSCRATCH: STGM = 1048576u32;
pub const STGM_NOSNAPSHOT: STGM = 2097152u32;
pub const STGM_PRIORITY: STGM = 262144u32;
pub const STGM_READ: STGM = 0u32;
pub const STGM_READWRITE: STGM = 2u32;
pub const STGM_SHARE_DENY_NONE: STGM = 64u32;
pub const STGM_SHARE_DENY_READ: STGM = 48u32;
pub const STGM_SHARE_DENY_WRITE: STGM = 32u32;
pub const STGM_SHARE_EXCLUSIVE: STGM = 16u32;
pub const STGM_SIMPLE: STGM = 134217728u32;
pub const STGM_TRANSACTED: STGM = 65536u32;
pub const STGM_WRITE: STGM = 1u32;
pub const STGTY_LOCKBYTES: STGTY = 3i32;
pub const STGTY_PROPERTY: STGTY = 4i32;
pub const STGTY_REPEAT: i32 = 256i32;
pub const STGTY_STORAGE: STGTY = 1i32;
pub const STGTY_STREAM: STGTY = 2i32;
pub const STG_LAYOUT_INTERLEAVED: i32 = 1i32;
pub const STG_LAYOUT_SEQUENTIAL: i32 = 0i32;
pub const STG_TOEND: i32 = -1i32;
pub const STREAM_SEEK_CUR: STREAM_SEEK = 1u32;
pub const STREAM_SEEK_END: STREAM_SEEK = 2u32;
pub const STREAM_SEEK_SET: STREAM_SEEK = 0u32;
pub const SYS_MAC: SYSKIND = 2i32;
pub const SYS_WIN16: SYSKIND = 0i32;
pub const SYS_WIN32: SYSKIND = 1i32;
pub const SYS_WIN64: SYSKIND = 3i32;
pub const ServerApplication: ApplicationType = 0i32;
pub const THDTYPE_BLOCKMESSAGES: THDTYPE = 0i32;
pub const THDTYPE_PROCESSMESSAGES: THDTYPE = 1i32;
pub const TKIND_ALIAS: TYPEKIND = 6i32;
pub const TKIND_COCLASS: TYPEKIND = 5i32;
pub const TKIND_DISPATCH: TYPEKIND = 4i32;
pub const TKIND_ENUM: TYPEKIND = 0i32;
pub const TKIND_INTERFACE: TYPEKIND = 3i32;
pub const TKIND_MAX: TYPEKIND = 8i32;
pub const TKIND_MODULE: TYPEKIND = 2i32;
pub const TKIND_RECORD: TYPEKIND = 1i32;
pub const TKIND_UNION: TYPEKIND = 7i32;
pub const TYMED_ENHMF: TYMED = 64i32;
pub const TYMED_FILE: TYMED = 2i32;
pub const TYMED_GDI: TYMED = 16i32;
pub const TYMED_HGLOBAL: TYMED = 1i32;
pub const TYMED_ISTORAGE: TYMED = 8i32;
pub const TYMED_ISTREAM: TYMED = 4i32;
pub const TYMED_MFPICT: TYMED = 32i32;
pub const TYMED_NULL: TYMED = 0i32;
pub const TYSPEC_CLSID: TYSPEC = 0i32;
pub const TYSPEC_FILEEXT: TYSPEC = 1i32;
pub const TYSPEC_FILENAME: TYSPEC = 3i32;
pub const TYSPEC_MIMETYPE: TYSPEC = 2i32;
pub const TYSPEC_OBJECTID: TYSPEC = 6i32;
pub const TYSPEC_PACKAGENAME: TYSPEC = 5i32;
pub const TYSPEC_PROGID: TYSPEC = 4i32;
pub const Uri_CREATE_ALLOW_IMPLICIT_FILE_SCHEME: URI_CREATE_FLAGS = 4u32;
pub const Uri_CREATE_ALLOW_IMPLICIT_WILDCARD_SCHEME: URI_CREATE_FLAGS = 2u32;
pub const Uri_CREATE_ALLOW_RELATIVE: URI_CREATE_FLAGS = 1u32;
pub const Uri_CREATE_CANONICALIZE: URI_CREATE_FLAGS = 256u32;
pub const Uri_CREATE_CANONICALIZE_ABSOLUTE: URI_CREATE_FLAGS = 131072u32;
pub const Uri_CREATE_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 512u32;
pub const Uri_CREATE_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 64u32;
pub const Uri_CREATE_FILE_USE_DOS_PATH: URI_CREATE_FLAGS = 32u32;
pub const Uri_CREATE_IE_SETTINGS: URI_CREATE_FLAGS = 8192u32;
pub const Uri_CREATE_NOFRAG: URI_CREATE_FLAGS = 8u32;
pub const Uri_CREATE_NORMALIZE_INTL_CHARACTERS: URI_CREATE_FLAGS = 65536u32;
pub const Uri_CREATE_NO_CANONICALIZE: URI_CREATE_FLAGS = 16u32;
pub const Uri_CREATE_NO_CRACK_UNKNOWN_SCHEMES: URI_CREATE_FLAGS = 1024u32;
pub const Uri_CREATE_NO_DECODE_EXTRA_INFO: URI_CREATE_FLAGS = 128u32;
pub const Uri_CREATE_NO_ENCODE_FORBIDDEN_CHARACTERS: URI_CREATE_FLAGS = 32768u32;
pub const Uri_CREATE_NO_IE_SETTINGS: URI_CREATE_FLAGS = 16384u32;
pub const Uri_CREATE_NO_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 4096u32;
pub const Uri_CREATE_PRE_PROCESS_HTML_URI: URI_CREATE_FLAGS = 2048u32;
pub const Uri_PROPERTY_ABSOLUTE_URI: Uri_PROPERTY = 0i32;
pub const Uri_PROPERTY_AUTHORITY: Uri_PROPERTY = 1i32;
pub const Uri_PROPERTY_DISPLAY_URI: Uri_PROPERTY = 2i32;
pub const Uri_PROPERTY_DOMAIN: Uri_PROPERTY = 3i32;
pub const Uri_PROPERTY_DWORD_LAST: Uri_PROPERTY = 18i32;
pub const Uri_PROPERTY_DWORD_START: Uri_PROPERTY = 15i32;
pub const Uri_PROPERTY_EXTENSION: Uri_PROPERTY = 4i32;
pub const Uri_PROPERTY_FRAGMENT: Uri_PROPERTY = 5i32;
pub const Uri_PROPERTY_HOST: Uri_PROPERTY = 6i32;
pub const Uri_PROPERTY_HOST_TYPE: Uri_PROPERTY = 15i32;
pub const Uri_PROPERTY_PASSWORD: Uri_PROPERTY = 7i32;
pub const Uri_PROPERTY_PATH: Uri_PROPERTY = 8i32;
pub const Uri_PROPERTY_PATH_AND_QUERY: Uri_PROPERTY = 9i32;
pub const Uri_PROPERTY_PORT: Uri_PROPERTY = 16i32;
pub const Uri_PROPERTY_QUERY: Uri_PROPERTY = 10i32;
pub const Uri_PROPERTY_RAW_URI: Uri_PROPERTY = 11i32;
pub const Uri_PROPERTY_SCHEME: Uri_PROPERTY = 17i32;
pub const Uri_PROPERTY_SCHEME_NAME: Uri_PROPERTY = 12i32;
pub const Uri_PROPERTY_STRING_LAST: Uri_PROPERTY = 14i32;
pub const Uri_PROPERTY_STRING_START: Uri_PROPERTY = 0i32;
pub const Uri_PROPERTY_USER_INFO: Uri_PROPERTY = 13i32;
pub const Uri_PROPERTY_USER_NAME: Uri_PROPERTY = 14i32;
pub const Uri_PROPERTY_ZONE: Uri_PROPERTY = 18i32;
pub const VARFLAG_FBINDABLE: VARFLAGS = 4u16;
pub const VARFLAG_FDEFAULTBIND: VARFLAGS = 32u16;
pub const VARFLAG_FDEFAULTCOLLELEM: VARFLAGS = 256u16;
pub const VARFLAG_FDISPLAYBIND: VARFLAGS = 16u16;
pub const VARFLAG_FHIDDEN: VARFLAGS = 64u16;
pub const VARFLAG_FIMMEDIATEBIND: VARFLAGS = 4096u16;
pub const VARFLAG_FNONBROWSABLE: VARFLAGS = 1024u16;
pub const VARFLAG_FREADONLY: VARFLAGS = 1u16;
pub const VARFLAG_FREPLACEABLE: VARFLAGS = 2048u16;
pub const VARFLAG_FREQUESTEDIT: VARFLAGS = 8u16;
pub const VARFLAG_FRESTRICTED: VARFLAGS = 128u16;
pub const VARFLAG_FSOURCE: VARFLAGS = 2u16;
pub const VARFLAG_FUIDEFAULT: VARFLAGS = 512u16;
pub const VAR_CONST: VARKIND = 2i32;
pub const VAR_DISPATCH: VARKIND = 3i32;
pub const VAR_PERINSTANCE: VARKIND = 0i32;
pub const VAR_STATIC: VARKIND = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ADVANCED_FEATURE_FLAGS(pub u16);
impl windows_core::TypeKind for ADVANCED_FEATURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ADVF(pub i32);
impl windows_core::TypeKind for ADVF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APTTYPE(pub i32);
impl windows_core::TypeKind for APTTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct APTTYPEQUALIFIER(pub i32);
impl windows_core::TypeKind for APTTYPEQUALIFIER {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ApplicationType(pub i32);
impl windows_core::TypeKind for ApplicationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BINDINFOF(pub i32);
impl windows_core::TypeKind for BINDINFOF {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BIND_FLAGS(pub i32);
impl windows_core::TypeKind for BIND_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CALLCONV(pub i32);
impl windows_core::TypeKind for CALLCONV {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CALLTYPE(pub i32);
impl windows_core::TypeKind for CALLTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CLSCTX(pub u32);
impl windows_core::TypeKind for CLSCTX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COINIT(pub i32);
impl windows_core::TypeKind for COINIT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COINITBASE(pub i32);
impl windows_core::TypeKind for COINITBASE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COMSD(pub i32);
impl windows_core::TypeKind for COMSD {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COWAIT_FLAGS(pub i32);
impl windows_core::TypeKind for COWAIT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CO_MARSHALING_CONTEXT_ATTRIBUTES(pub i32);
impl windows_core::TypeKind for CO_MARSHALING_CONTEXT_ATTRIBUTES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CWMO_FLAGS(pub i32);
impl windows_core::TypeKind for CWMO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DATADIR(pub i32);
impl windows_core::TypeKind for DATADIR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DCOM_CALL_STATE(pub i32);
impl windows_core::TypeKind for DCOM_CALL_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DESCKIND(pub i32);
impl windows_core::TypeKind for DESCKIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DISPATCH_FLAGS(pub u16);
impl windows_core::TypeKind for DISPATCH_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DVASPECT(pub u32);
impl windows_core::TypeKind for DVASPECT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EOLE_AUTHENTICATION_CAPABILITIES(pub i32);
impl windows_core::TypeKind for EOLE_AUTHENTICATION_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EXTCONN(pub i32);
impl windows_core::TypeKind for EXTCONN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FUNCFLAGS(pub u16);
impl windows_core::TypeKind for FUNCFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FUNCKIND(pub i32);
impl windows_core::TypeKind for FUNCKIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GLOBALOPT_EH_VALUES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_EH_VALUES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GLOBALOPT_PROPERTIES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GLOBALOPT_RO_FLAGS(pub i32);
impl windows_core::TypeKind for GLOBALOPT_RO_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GLOBALOPT_RPCTP_VALUES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_RPCTP_VALUES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GLOBALOPT_UNMARSHALING_POLICY_VALUES(pub i32);
impl windows_core::TypeKind for GLOBALOPT_UNMARSHALING_POLICY_VALUES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IDLFLAGS(pub u16);
impl windows_core::TypeKind for IDLFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IMPLTYPEFLAGS(pub i32);
impl windows_core::TypeKind for IMPLTYPEFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct INVOKEKIND(pub i32);
impl windows_core::TypeKind for INVOKEKIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LOCKTYPE(pub i32);
impl windows_core::TypeKind for LOCKTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MEMCTX(pub i32);
impl windows_core::TypeKind for MEMCTX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MKRREDUCE(pub i32);
impl windows_core::TypeKind for MKRREDUCE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MKSYS(pub i32);
impl windows_core::TypeKind for MKSYS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MSHCTX(pub i32);
impl windows_core::TypeKind for MSHCTX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MSHLFLAGS(pub i32);
impl windows_core::TypeKind for MSHLFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PENDINGMSG(pub i32);
impl windows_core::TypeKind for PENDINGMSG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PENDINGTYPE(pub i32);
impl windows_core::TypeKind for PENDINGTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct REGCLS(pub i32);
impl windows_core::TypeKind for REGCLS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ROT_FLAGS(pub u32);
impl windows_core::TypeKind for ROT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPCOPT_PROPERTIES(pub i32);
impl windows_core::TypeKind for RPCOPT_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPCOPT_SERVER_LOCALITY_VALUES(pub i32);
impl windows_core::TypeKind for RPCOPT_SERVER_LOCALITY_VALUES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_AUTHN_LEVEL(pub u32);
impl windows_core::TypeKind for RPC_C_AUTHN_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_IMP_LEVEL(pub u32);
impl windows_core::TypeKind for RPC_C_IMP_LEVEL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SERVERCALL(pub i32);
impl windows_core::TypeKind for SERVERCALL {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STATFLAG(pub i32);
impl windows_core::TypeKind for STATFLAG {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STGC(pub i32);
impl windows_core::TypeKind for STGC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STGM(pub u32);
impl windows_core::TypeKind for STGM {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STGTY(pub i32);
impl windows_core::TypeKind for STGTY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STREAM_SEEK(pub u32);
impl windows_core::TypeKind for STREAM_SEEK {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYSKIND(pub i32);
impl windows_core::TypeKind for SYSKIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShutdownType(pub i32);
impl windows_core::TypeKind for ShutdownType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct THDTYPE(pub i32);
impl windows_core::TypeKind for THDTYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TYMED(pub i32);
impl windows_core::TypeKind for TYMED {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TYPEKIND(pub i32);
impl windows_core::TypeKind for TYPEKIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TYSPEC(pub i32);
impl windows_core::TypeKind for TYSPEC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct URI_CREATE_FLAGS(pub u32);
impl windows_core::TypeKind for URI_CREATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Uri_PROPERTY(pub i32);
impl windows_core::TypeKind for Uri_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VARFLAGS(pub u16);
impl windows_core::TypeKind for VARFLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VARKIND(pub i32);
impl windows_core::TypeKind for VARKIND {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUTHENTICATEINFO {
    pub dwFlags: u32,
    pub dwReserved: u32,
}
impl Default for AUTHENTICATEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for AUTHENTICATEINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BINDINFO {
    pub cbSize: u32,
    pub szExtraInfo: windows_core::PWSTR,
    pub stgmedData: STGMEDIUM,
    pub grfBindInfoF: u32,
    pub dwBindVerb: u32,
    pub szCustomVerb: windows_core::PWSTR,
    pub cbstgmedData: u32,
    pub dwOptions: u32,
    pub dwOptionsFlags: u32,
    pub dwCodePage: u32,
    pub securityAttributes: super::super::Security::SECURITY_ATTRIBUTES,
    pub iid: windows_core::GUID,
    pub pUnk: Option<windows_core::IUnknown>,
    pub dwReserved: u32,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for BINDINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for BINDINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union BINDPTR {
    pub lpfuncdesc: *mut FUNCDESC,
    pub lpvardesc: *mut VARDESC,
    pub lptcomp: Option<ITypeComp>,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for BINDPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for BINDPTR {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
impl Default for BIND_OPTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BIND_OPTS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: u32,
    pub pServerInfo: *mut COSERVERINFO,
}
impl Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BIND_OPTS2 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::super::Foundation::HWND,
}
impl Default for BIND_OPTS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BIND_OPTS3 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
impl Default for BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BLOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BYTE_BLOB {
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl Default for BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BYTE_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BYTE_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u8,
}
impl Default for BYTE_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BYTE_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CATEGORYINFO {
    pub catid: windows_core::GUID,
    pub lcid: u32,
    pub szDescription: [u16; 128],
}
impl Default for CATEGORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CATEGORYINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COAUTHIDENTITY {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for COAUTHIDENTITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COAUTHIDENTITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COAUTHINFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pwszServerPrincName: windows_core::PWSTR,
    pub dwAuthnLevel: u32,
    pub dwImpersonationLevel: u32,
    pub pAuthIdentityData: *mut COAUTHIDENTITY,
    pub dwCapabilities: u32,
}
impl Default for COAUTHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COAUTHINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONNECTDATA {
    pub pUnk: Option<windows_core::IUnknown>,
    pub dwCookie: u32,
}
impl Default for CONNECTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CONNECTDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COSERVERINFO {
    pub dwReserved1: u32,
    pub pwszName: windows_core::PWSTR,
    pub pAuthInfo: *mut COAUTHINFO,
    pub dwReserved2: u32,
}
impl Default for COSERVERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COSERVERINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CSPLATFORM {
    pub dwPlatformId: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwProcessorArch: u32,
}
impl Default for CSPLATFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CSPLATFORM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CUSTDATA {
    pub cCustData: u32,
    pub prgCustData: *mut CUSTDATAITEM,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for CUSTDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for CUSTDATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CUSTDATAITEM {
    pub guid: windows_core::GUID,
    pub varValue: super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for CUSTDATAITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for CUSTDATAITEM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CY {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
impl Default for CY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CY_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComCallData {
    pub dwDispid: u32,
    pub dwReserved: u32,
    pub pUserDefined: *mut core::ffi::c_void,
}
impl Default for ComCallData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ComCallData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContextProperty {
    pub policyId: windows_core::GUID,
    pub flags: u32,
    pub pUnk: Option<windows_core::IUnknown>,
}
impl Default for ContextProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ContextProperty {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPPARAMS {
    pub rgvarg: *mut super::Variant::VARIANT,
    pub rgdispidNamedArgs: *mut i32,
    pub cArgs: u32,
    pub cNamedArgs: u32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for DISPPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for DISPPARAMS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DVTARGETDEVICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWORD_BLOB {
    pub clSize: u32,
    pub alData: [u32; 1],
}
impl Default for DWORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DWORD_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DWORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u32,
}
impl Default for DWORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DWORD_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ELEMDESC {
    pub tdesc: TYPEDESC,
    pub Anonymous: ELEMDESC_0,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for ELEMDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for ELEMDESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union ELEMDESC_0 {
    pub idldesc: IDLDESC,
    pub paramdesc: super::Ole::PARAMDESC,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for ELEMDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for ELEMDESC_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXCEPINFO {
    pub wCode: u16,
    pub wReserved: u16,
    pub bstrSource: windows_core::BSTR,
    pub bstrDescription: windows_core::BSTR,
    pub bstrHelpFile: windows_core::BSTR,
    pub dwHelpContext: u32,
    pub pvReserved: *mut core::ffi::c_void,
    pub pfnDeferredFillIn: LPEXCEPFINO_DEFERRED_FILLIN,
    pub scode: i32,
}
impl Default for EXCEPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for EXCEPINFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLAGGED_BYTE_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub abData: [u8; 1],
}
impl Default for FLAGGED_BYTE_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FLAGGED_BYTE_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLAGGED_WORD_BLOB {
    pub fFlags: u32,
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for FLAGGED_WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FLAGGED_WORD_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for FLAG_STGMEDIUM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORMATETC {
    pub cfFormat: u16,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
impl Default for FORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FORMATETC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FUNCDESC {
    pub memid: i32,
    pub lprgscode: *mut i32,
    pub lprgelemdescParam: *mut ELEMDESC,
    pub funckind: FUNCKIND,
    pub invkind: INVOKEKIND,
    pub callconv: CALLCONV,
    pub cParams: i16,
    pub cParamsOpt: i16,
    pub oVft: i16,
    pub cScodes: i16,
    pub elemdescFunc: ELEMDESC,
    pub wFuncFlags: FUNCFLAGS,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for FUNCDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for FUNCDESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for GDI_OBJECT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union GDI_OBJECT_0 {
    pub hBitmap: *mut super::SystemServices::userHBITMAP,
    pub hPalette: *mut super::SystemServices::userHPALETTE,
    pub hGeneric: *mut super::SystemServices::userHGLOBAL,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for GDI_OBJECT_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HYPER_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut i64,
}
impl Default for HYPER_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HYPER_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IDLDESC {
    pub dwReserved: usize,
    pub wIDLFlags: IDLFLAGS,
}
impl Default for IDLDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IDLDESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INTERFACEINFO {
    pub pUnk: Option<windows_core::IUnknown>,
    pub iid: windows_core::GUID,
    pub wMethod: u16,
}
impl Default for INTERFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INTERFACEINFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MULTI_QI {
    pub pIID: *const windows_core::GUID,
    pub pItf: Option<windows_core::IUnknown>,
    pub hr: windows_core::HRESULT,
}
impl Default for MULTI_QI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MULTI_QI {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QUERYCONTEXT {
    pub dwContext: u32,
    pub Platform: CSPLATFORM,
    pub Locale: u32,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
}
impl Default for QUERYCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for QUERYCONTEXT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPCOLEMESSAGE {
    pub reserved1: *mut core::ffi::c_void,
    pub dataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub cbBuffer: u32,
    pub iMethod: u32,
    pub reserved2: [*mut core::ffi::c_void; 5],
    pub rpcFlags: u32,
}
impl Default for RPCOLEMESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPCOLEMESSAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RemSTGMEDIUM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SAFEARRAY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
impl Default for SAFEARRAYBOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SAFEARRAYBOUND {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SChannelHookCallInfo {
    pub iid: windows_core::GUID,
    pub cbSize: u32,
    pub uCausality: windows_core::GUID,
    pub dwServerPid: u32,
    pub iMethod: u32,
    pub pObject: *mut core::ffi::c_void,
}
impl Default for SChannelHookCallInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SChannelHookCallInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_INFO {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pAuthInfo: *mut core::ffi::c_void,
}
impl Default for SOLE_AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SOLE_AUTHENTICATION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_LIST {
    pub cAuthInfo: u32,
    pub aAuthInfo: *mut SOLE_AUTHENTICATION_INFO,
}
impl Default for SOLE_AUTHENTICATION_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SOLE_AUTHENTICATION_LIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOLE_AUTHENTICATION_SERVICE {
    pub dwAuthnSvc: u32,
    pub dwAuthzSvc: u32,
    pub pPrincipalName: windows_core::PWSTR,
    pub hr: windows_core::HRESULT,
}
impl Default for SOLE_AUTHENTICATION_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SOLE_AUTHENTICATION_SERVICE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: Option<IAdviseSink>,
    pub dwConnection: u32,
}
impl Default for STATDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STATDATA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STATSTG {
    pub pwcsName: windows_core::PWSTR,
    pub r#type: u32,
    pub cbSize: u64,
    pub mtime: super::super::Foundation::FILETIME,
    pub ctime: super::super::Foundation::FILETIME,
    pub atime: super::super::Foundation::FILETIME,
    pub grfMode: STGM,
    pub grfLocksSupported: u32,
    pub clsid: windows_core::GUID,
    pub grfStateBits: u32,
    pub reserved: u32,
}
impl Default for STATSTG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STATSTG {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub u: STGMEDIUM_0,
    pub pUnkForRelease: Option<windows_core::IUnknown>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for STGMEDIUM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union STGMEDIUM_0 {
    pub hBitmap: super::super::Graphics::Gdi::HBITMAP,
    pub hMetaFilePict: *mut core::ffi::c_void,
    pub hEnhMetaFile: super::super::Graphics::Gdi::HENHMETAFILE,
    pub hGlobal: super::super::Foundation::HGLOBAL,
    pub lpszFileName: windows_core::PWSTR,
    pub pstm: Option<IStream>,
    pub pstg: Option<StructuredStorage::IStorage>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl windows_core::TypeKind for STGMEDIUM_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: windows_core::PWSTR,
    pub cOffset: i64,
    pub cBytes: i64,
}
impl Default for StorageLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for StorageLayout {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TLIBATTR {
    pub guid: windows_core::GUID,
    pub lcid: u32,
    pub syskind: SYSKIND,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub wLibFlags: u16,
}
impl Default for TLIBATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TLIBATTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TYPEATTR {
    pub guid: windows_core::GUID,
    pub lcid: u32,
    pub dwReserved: u32,
    pub memidConstructor: i32,
    pub memidDestructor: i32,
    pub lpstrSchema: windows_core::PWSTR,
    pub cbSizeInstance: u32,
    pub typekind: TYPEKIND,
    pub cFuncs: u16,
    pub cVars: u16,
    pub cImplTypes: u16,
    pub cbSizeVft: u16,
    pub cbAlignment: u16,
    pub wTypeFlags: u16,
    pub wMajorVerNum: u16,
    pub wMinorVerNum: u16,
    pub tdescAlias: TYPEDESC,
    pub idldescType: IDLDESC,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for TYPEATTR {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TYPEDESC {
    pub Anonymous: TYPEDESC_0,
    pub vt: super::Variant::VARENUM,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for TYPEDESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union TYPEDESC_0 {
    pub lptdesc: *mut TYPEDESC,
    pub lpadesc: *mut super::Ole::ARRAYDESC,
    pub hreftype: u32,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for TYPEDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for TYPEDESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VARDESC {
    pub memid: i32,
    pub lpstrSchema: windows_core::PWSTR,
    pub Anonymous: VARDESC_0,
    pub elemdescVar: ELEMDESC,
    pub wVarFlags: VARFLAGS,
    pub varkind: VARKIND,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for VARDESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union VARDESC_0 {
    pub oInst: u32,
    pub lpvarValue: *mut super::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl Default for VARDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::TypeKind for VARDESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WORD_BLOB {
    pub clSize: u32,
    pub asData: [u16; 1],
}
impl Default for WORD_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WORD_BLOB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WORD_SIZEDARR {
    pub clSize: u32,
    pub pData: *mut u16,
}
impl Default for WORD_SIZEDARR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for WORD_SIZEDARR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct uCLSSPEC {
    pub tyspec: u32,
    pub tagged_union: uCLSSPEC_0,
}
impl Default for uCLSSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for uCLSSPEC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union uCLSSPEC_0 {
    pub clsid: windows_core::GUID,
    pub pFileExt: windows_core::PWSTR,
    pub pMimeType: windows_core::PWSTR,
    pub pProgId: windows_core::PWSTR,
    pub pFileName: windows_core::PWSTR,
    pub ByName: uCLSSPEC_0_0,
    pub ByObjectId: uCLSSPEC_0_1,
}
impl Default for uCLSSPEC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for uCLSSPEC_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct uCLSSPEC_0_0 {
    pub pPackageName: windows_core::PWSTR,
    pub PolicyId: windows_core::GUID,
}
impl Default for uCLSSPEC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for uCLSSPEC_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct uCLSSPEC_0_1 {
    pub ObjectId: windows_core::GUID,
    pub PolicyId: windows_core::GUID,
}
impl Default for uCLSSPEC_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for uCLSSPEC_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userFLAG_STGMEDIUM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct userSTGMEDIUM {
    pub u: userSTGMEDIUM_0,
    pub pUnkForRelease: Option<windows_core::IUnknown>,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userSTGMEDIUM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userSTGMEDIUM_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: *mut super::SystemServices::userHMETAFILEPICT,
    pub hHEnhMetaFile: *mut super::SystemServices::userHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: *mut super::SystemServices::userHGLOBAL,
    pub lpszFileName: windows_core::PWSTR,
    pub pstm: *mut BYTE_BLOB,
    pub pstg: *mut BYTE_BLOB,
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_SystemServices"))]
impl windows_core::TypeKind for userSTGMEDIUM_0_0 {
    type TypeKind = windows_core::CopyType;
}
pub type LPEXCEPFINO_DEFERRED_FILLIN = Option<unsafe extern "system" fn(pexcepinfo: *mut EXCEPINFO) -> windows_core::HRESULT>;
pub type LPFNCANUNLOADNOW = Option<unsafe extern "system" fn() -> windows_core::HRESULT>;
pub type LPFNGETCLASSOBJECT = Option<unsafe extern "system" fn(param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void) -> windows_core::HRESULT>;
pub type PFNCONTEXTCALL = Option<unsafe extern "system" fn(pparam: *mut ComCallData) -> windows_core::HRESULT>;
