windows_link::link!("xolehlp.dll" "C" fn DtcGetTransactionManager(i_pszhost : windows_sys::core::PCSTR, i_psztmname : windows_sys::core::PCSTR, i_riid : *const windows_sys::core::GUID, i_dwreserved1 : u32, i_wcbreserved2 : u16, i_pvreserved2 : *const core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("xolehlp.dll" "C" fn DtcGetTransactionManagerC(i_pszhost : windows_sys::core::PCSTR, i_psztmname : windows_sys::core::PCSTR, i_riid : *const windows_sys::core::GUID, i_dwreserved1 : u32, i_wcbreserved2 : u16, i_pvreserved2 : *const core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("xolehlp.dll" "C" fn DtcGetTransactionManagerExA(i_pszhost : windows_sys::core::PCSTR, i_psztmname : windows_sys::core::PCSTR, i_riid : *const windows_sys::core::GUID, i_grfoptions : u32, i_pvconfigparams : *mut core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("xolehlp.dll" "C" fn DtcGetTransactionManagerExW(i_pwszhost : windows_sys::core::PCWSTR, i_pwsztmname : windows_sys::core::PCWSTR, i_riid : *const windows_sys::core::GUID, i_grfoptions : u32, i_pvconfigparams : *mut core::ffi::c_void, o_ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type APPLICATIONTYPE = i32;
pub type AUTHENTICATION_LEVEL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOID {
    pub rgb: [u8; 16],
}
impl Default for BOID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLSID_MSDtcTransaction: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39f8d76b_0928_11d1_97df_00c04fb9618a);
pub const CLSID_MSDtcTransactionManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b18ab61_091d_11d1_97df_00c04fb9618a);
pub const CLUSTERRESOURCE_APPLICATIONTYPE: APPLICATIONTYPE = 1;
pub type DTCINITIATEDRECOVERYWORK = i32;
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: DTCINITIATEDRECOVERYWORK = 1;
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: DTCINITIATEDRECOVERYWORK = 3;
pub const DTCINITIATEDRECOVERYWORK_TRANS: DTCINITIATEDRECOVERYWORK = 2;
pub const DTCINSTALL_E_CLIENT_ALREADY_INSTALLED: i32 = 384;
pub const DTCINSTALL_E_SERVER_ALREADY_INSTALLED: i32 = 385;
pub type DTCLUCOMPARESTATE = i32;
pub type DTCLUCOMPARESTATESCONFIRMATION = i32;
pub const DTCLUCOMPARESTATESCONFIRMATION_CONFIRM: DTCLUCOMPARESTATESCONFIRMATION = 1;
pub const DTCLUCOMPARESTATESCONFIRMATION_PROTOCOL: DTCLUCOMPARESTATESCONFIRMATION = 2;
pub type DTCLUCOMPARESTATESERROR = i32;
pub const DTCLUCOMPARESTATESERROR_PROTOCOL: DTCLUCOMPARESTATESERROR = 1;
pub type DTCLUCOMPARESTATESRESPONSE = i32;
pub const DTCLUCOMPARESTATESRESPONSE_OK: DTCLUCOMPARESTATESRESPONSE = 1;
pub const DTCLUCOMPARESTATESRESPONSE_PROTOCOL: DTCLUCOMPARESTATESRESPONSE = 2;
pub const DTCLUCOMPARESTATE_COMMITTED: DTCLUCOMPARESTATE = 1;
pub const DTCLUCOMPARESTATE_HEURISTICCOMMITTED: DTCLUCOMPARESTATE = 2;
pub const DTCLUCOMPARESTATE_HEURISTICMIXED: DTCLUCOMPARESTATE = 3;
pub const DTCLUCOMPARESTATE_HEURISTICRESET: DTCLUCOMPARESTATE = 4;
pub const DTCLUCOMPARESTATE_INDOUBT: DTCLUCOMPARESTATE = 5;
pub const DTCLUCOMPARESTATE_RESET: DTCLUCOMPARESTATE = 6;
pub type DTCLUXLN = i32;
pub type DTCLUXLNCONFIRMATION = i32;
pub const DTCLUXLNCONFIRMATION_COLDWARMMISMATCH: DTCLUXLNCONFIRMATION = 3;
pub const DTCLUXLNCONFIRMATION_CONFIRM: DTCLUXLNCONFIRMATION = 1;
pub const DTCLUXLNCONFIRMATION_LOGNAMEMISMATCH: DTCLUXLNCONFIRMATION = 2;
pub const DTCLUXLNCONFIRMATION_OBSOLETE: DTCLUXLNCONFIRMATION = 4;
pub type DTCLUXLNERROR = i32;
pub const DTCLUXLNERROR_COLDWARMMISMATCH: DTCLUXLNERROR = 3;
pub const DTCLUXLNERROR_LOGNAMEMISMATCH: DTCLUXLNERROR = 2;
pub const DTCLUXLNERROR_PROTOCOL: DTCLUXLNERROR = 1;
pub type DTCLUXLNRESPONSE = i32;
pub const DTCLUXLNRESPONSE_COLDWARMMISMATCH: DTCLUXLNRESPONSE = 4;
pub const DTCLUXLNRESPONSE_LOGNAMEMISMATCH: DTCLUXLNRESPONSE = 3;
pub const DTCLUXLNRESPONSE_OK_SENDCONFIRMATION: DTCLUXLNRESPONSE = 2;
pub const DTCLUXLNRESPONSE_OK_SENDOURXLNBACK: DTCLUXLNRESPONSE = 1;
pub const DTCLUXLN_COLD: DTCLUXLN = 1;
pub const DTCLUXLN_WARM: DTCLUXLN = 2;
pub type DTC_GET_TRANSACTION_MANAGER = Option<unsafe extern "C" fn(pszhost: windows_sys::core::PCSTR, psztmname: windows_sys::core::PCSTR, rid: *const windows_sys::core::GUID, dwreserved1: u32, wcbreserved2: u16, pvreserved2: *mut core::ffi::c_void, ppvobject: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_A = Option<unsafe extern "C" fn(i_pszhost: windows_sys::core::PCSTR, i_psztmname: windows_sys::core::PCSTR, i_riid: *const windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type DTC_GET_TRANSACTION_MANAGER_EX_W = Option<unsafe extern "C" fn(i_pwszhost: windows_sys::core::PCWSTR, i_pwsztmname: windows_sys::core::PCWSTR, i_riid: *const windows_sys::core::GUID, i_grfoptions: u32, i_pvconfigparams: *mut core::ffi::c_void, o_ppvobject: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type DTC_INSTALL_CLIENT = Option<unsafe extern "system" fn(i_pszremotetmhostname: *mut i8, i_dwprotocol: u32, i_dwoverwrite: u32) -> windows_sys::core::HRESULT>;
pub const DTC_INSTALL_OVERWRITE_CLIENT: u32 = 1;
pub const DTC_INSTALL_OVERWRITE_SERVER: u32 = 2;
pub type DTC_STATUS_ = i32;
pub const DTC_STATUS_CONTINUING: DTC_STATUS_ = 5;
pub const DTC_STATUS_E_CANTCONTROL: DTC_STATUS_ = 8;
pub const DTC_STATUS_FAILED: DTC_STATUS_ = 9;
pub const DTC_STATUS_PAUSED: DTC_STATUS_ = 4;
pub const DTC_STATUS_PAUSING: DTC_STATUS_ = 3;
pub const DTC_STATUS_STARTED: DTC_STATUS_ = 2;
pub const DTC_STATUS_STARTING: DTC_STATUS_ = 1;
pub const DTC_STATUS_STOPPED: DTC_STATUS_ = 7;
pub const DTC_STATUS_STOPPING: DTC_STATUS_ = 6;
pub const DTC_STATUS_UNKNOWN: DTC_STATUS_ = 0;
pub const INCOMING_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 1;
pub type ISOFLAG = i32;
pub const ISOFLAG_OPTIMISTIC: ISOFLAG = 16;
pub const ISOFLAG_READONLY: ISOFLAG = 32;
pub const ISOFLAG_RETAIN_ABORT: ISOFLAG = 8;
pub const ISOFLAG_RETAIN_ABORT_DC: ISOFLAG = 4;
pub const ISOFLAG_RETAIN_ABORT_NO: ISOFLAG = 12;
pub const ISOFLAG_RETAIN_BOTH: ISOFLAG = 10;
pub const ISOFLAG_RETAIN_COMMIT: ISOFLAG = 2;
pub const ISOFLAG_RETAIN_COMMIT_DC: ISOFLAG = 1;
pub const ISOFLAG_RETAIN_COMMIT_NO: ISOFLAG = 3;
pub const ISOFLAG_RETAIN_DONTCARE: ISOFLAG = 5;
pub const ISOFLAG_RETAIN_NONE: ISOFLAG = 15;
pub type ISOLATIONLEVEL = i32;
pub const ISOLATIONLEVEL_BROWSE: ISOLATIONLEVEL = 256;
pub const ISOLATIONLEVEL_CHAOS: ISOLATIONLEVEL = 16;
pub const ISOLATIONLEVEL_CURSORSTABILITY: ISOLATIONLEVEL = 4096;
pub const ISOLATIONLEVEL_ISOLATED: ISOLATIONLEVEL = 1048576;
pub const ISOLATIONLEVEL_READCOMMITTED: ISOLATIONLEVEL = 4096;
pub const ISOLATIONLEVEL_READUNCOMMITTED: ISOLATIONLEVEL = 256;
pub const ISOLATIONLEVEL_REPEATABLEREAD: ISOLATIONLEVEL = 65536;
pub const ISOLATIONLEVEL_SERIALIZABLE: ISOLATIONLEVEL = 1048576;
pub const ISOLATIONLEVEL_UNSPECIFIED: ISOLATIONLEVEL = -1;
pub const LOCAL_APPLICATIONTYPE: APPLICATIONTYPE = 0;
pub const MAXBQUALSIZE: u32 = 64;
pub const MAXGTRIDSIZE: u32 = 64;
pub const MAXINFOSIZE: u32 = 256;
pub const MAX_TRAN_DESC: TX_MISC_CONSTANTS = 40;
pub const MUTUAL_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 2;
pub const NO_AUTHENTICATION_REQUIRED: AUTHENTICATION_LEVEL = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OLE_TM_CONFIG_PARAMS_V1 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OLE_TM_CONFIG_PARAMS_V2 {
    pub dwVersion: u32,
    pub dwcConcurrencyHint: u32,
    pub applicationType: APPLICATIONTYPE,
    pub clusterResourceId: windows_sys::core::GUID,
}
pub const OLE_TM_CONFIG_VERSION_1: u32 = 1;
pub const OLE_TM_CONFIG_VERSION_2: u32 = 2;
pub const OLE_TM_FLAG_INTERNAL_TO_TM: u32 = 1073741824;
pub const OLE_TM_FLAG_NOAGILERECOVERY: u32 = 2;
pub const OLE_TM_FLAG_NODEMANDSTART: u32 = 1;
pub const OLE_TM_FLAG_NONE: u32 = 0;
pub const OLE_TM_FLAG_QUERY_SERVICE_LOCKSTATUS: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROXY_CONFIG_PARAMS {
    pub wcThreadsMax: u16,
}
pub const RMNAMESZ: u32 = 32;
pub const TMASYNC: i32 = -2147483648;
pub const TMENDRSCAN: i32 = 8388608;
pub const TMER_INVAL: i32 = -2;
pub const TMER_PROTO: i32 = -3;
pub const TMER_TMERR: i32 = -1;
pub const TMFAIL: i32 = 536870912;
pub const TMJOIN: i32 = 2097152;
pub const TMMIGRATE: i32 = 1048576;
pub const TMMULTIPLE: i32 = 4194304;
pub const TMNOFLAGS: i32 = 0;
pub const TMNOMIGRATE: i32 = 2;
pub const TMNOWAIT: i32 = 268435456;
pub const TMONEPHASE: i32 = 1073741824;
pub const TMREGISTER: i32 = 1;
pub const TMRESUME: i32 = 134217728;
pub const TMSTARTRSCAN: i32 = 16777216;
pub const TMSUCCESS: i32 = 67108864;
pub const TMSUSPEND: i32 = 33554432;
pub const TMUSEASYNC: i32 = 4;
pub const TM_JOIN: u32 = 2;
pub const TM_OK: u32 = 0;
pub const TM_RESUME: u32 = 1;
pub type TX_MISC_CONSTANTS = i32;
pub type XACTCONST = i32;
pub const XACTCONST_TIMEOUTINFINITE: XACTCONST = 0;
pub type XACTHEURISTIC = i32;
pub const XACTHEURISTIC_ABORT: XACTHEURISTIC = 1;
pub const XACTHEURISTIC_COMMIT: XACTHEURISTIC = 2;
pub const XACTHEURISTIC_DAMAGE: XACTHEURISTIC = 3;
pub const XACTHEURISTIC_DANGER: XACTHEURISTIC = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XACTOPT {
    pub ulTimeout: u32,
    pub szDescription: [u8; 40],
}
impl Default for XACTOPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type XACTRM = i32;
pub const XACTRM_NOREADONLYPREPARES: XACTRM = 2;
pub const XACTRM_OPTIMISTICLASTWINS: XACTRM = 1;
pub type XACTSTAT = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const XACTSTAT_ABORTED: XACTSTAT = 512;
pub const XACTSTAT_ABORTING: XACTSTAT = 256;
pub const XACTSTAT_ALL: XACTSTAT = 524287;
pub const XACTSTAT_CLOSED: XACTSTAT = 262144;
pub const XACTSTAT_COMMITRETAINING: XACTSTAT = 128;
pub const XACTSTAT_COMMITTED: XACTSTAT = 1024;
pub const XACTSTAT_COMMITTING: XACTSTAT = 64;
pub const XACTSTAT_FORCED_ABORT: XACTSTAT = 32768;
pub const XACTSTAT_FORCED_COMMIT: XACTSTAT = 65536;
pub const XACTSTAT_HEURISTIC_ABORT: XACTSTAT = 2048;
pub const XACTSTAT_HEURISTIC_COMMIT: XACTSTAT = 4096;
pub const XACTSTAT_HEURISTIC_DAMAGE: XACTSTAT = 8192;
pub const XACTSTAT_HEURISTIC_DANGER: XACTSTAT = 16384;
pub const XACTSTAT_INDOUBT: XACTSTAT = 131072;
pub const XACTSTAT_NONE: XACTSTAT = 0;
pub const XACTSTAT_NOTPREPARED: XACTSTAT = 524227;
pub const XACTSTAT_OPEN: XACTSTAT = 3;
pub const XACTSTAT_OPENNORMAL: XACTSTAT = 1;
pub const XACTSTAT_OPENREFUSED: XACTSTAT = 2;
pub const XACTSTAT_PREPARED: XACTSTAT = 8;
pub const XACTSTAT_PREPARERETAINED: XACTSTAT = 32;
pub const XACTSTAT_PREPARERETAINING: XACTSTAT = 16;
pub const XACTSTAT_PREPARING: XACTSTAT = 4;
pub type XACTTC = i32;
pub const XACTTC_ASYNC: XACTTC = 4;
pub const XACTTC_ASYNC_PHASEONE: XACTTC = 4;
pub const XACTTC_NONE: XACTTC = 0;
pub const XACTTC_SYNC: XACTTC = 2;
pub const XACTTC_SYNC_PHASEONE: XACTTC = 1;
pub const XACTTC_SYNC_PHASETWO: XACTTC = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XACTTRANSINFO {
    pub uow: BOID,
    pub isoLevel: i32,
    pub isoFlags: u32,
    pub grfTCSupported: u32,
    pub grfRMSupported: u32,
    pub grfTCSupportedRetaining: u32,
    pub grfRMSupportedRetaining: u32,
}
pub type XACT_DTC_CONSTANTS = i32;
pub const XACT_E_CONNECTION_REQUEST_DENIED: XACT_DTC_CONSTANTS = -2147168000;
pub const XACT_E_DUPLICATE_GUID: XACT_DTC_CONSTANTS = -2147167998;
pub const XACT_E_DUPLICATE_LU: XACT_DTC_CONSTANTS = -2147167991;
pub const XACT_E_DUPLICATE_TRANSID: XACT_DTC_CONSTANTS = -2147167989;
pub const XACT_E_LRMRECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167982;
pub const XACT_E_LU_BUSY: XACT_DTC_CONSTANTS = -2147167988;
pub const XACT_E_LU_DOWN: XACT_DTC_CONSTANTS = -2147167986;
pub const XACT_E_LU_NOT_CONNECTED: XACT_DTC_CONSTANTS = -2147167990;
pub const XACT_E_LU_NOT_FOUND: XACT_DTC_CONSTANTS = -2147167992;
pub const XACT_E_LU_NO_RECOVERY_PROCESS: XACT_DTC_CONSTANTS = -2147167987;
pub const XACT_E_LU_RECOVERING: XACT_DTC_CONSTANTS = -2147167985;
pub const XACT_E_LU_RECOVERY_MISMATCH: XACT_DTC_CONSTANTS = -2147167984;
pub const XACT_E_NOLASTRESOURCEINTERFACE: XACT_DTC_CONSTANTS = -2147167981;
pub const XACT_E_NOTSINGLEPHASE: XACT_DTC_CONSTANTS = -2147167997;
pub const XACT_E_PROTOCOL: XACT_DTC_CONSTANTS = -2147167995;
pub const XACT_E_RECOVERYALREADYDONE: XACT_DTC_CONSTANTS = -2147167996;
pub const XACT_E_RECOVERY_FAILED: XACT_DTC_CONSTANTS = -2147167993;
pub const XACT_E_RM_FAILURE: XACT_DTC_CONSTANTS = -2147167994;
pub const XACT_E_RM_UNAVAILABLE: XACT_DTC_CONSTANTS = -2147167983;
pub const XACT_E_TOOMANY_ENLISTMENTS: XACT_DTC_CONSTANTS = -2147167999;
pub const XACT_OK_NONOTIFY: XACT_DTC_CONSTANTS = 315649;
pub const XACT_S_NONOTIFY: XACT_DTC_CONSTANTS = 315648;
pub const XAER_ASYNC: i32 = -2;
pub const XAER_DUPID: i32 = -8;
pub const XAER_INVAL: i32 = -5;
pub const XAER_NOTA: i32 = -4;
pub const XAER_OUTSIDE: i32 = -9;
pub const XAER_PROTO: i32 = -6;
pub const XAER_RMERR: i32 = -3;
pub const XAER_RMFAIL: i32 = -7;
pub type XA_CLOSE_EPT = Option<unsafe extern "C" fn(param0: windows_sys::core::PCSTR, param1: i32, param2: i32) -> i32>;
pub type XA_COMMIT_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_COMPLETE_EPT = Option<unsafe extern "C" fn(param0: *mut i32, param1: *mut i32, param2: i32, param3: i32) -> i32>;
pub type XA_END_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_FMTID_DTC: u32 = 4478019;
pub const XA_FMTID_DTC_VER1: u32 = 21255235;
pub type XA_FORGET_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_HEURCOM: u32 = 7;
pub const XA_HEURHAZ: u32 = 8;
pub const XA_HEURMIX: u32 = 5;
pub const XA_HEURRB: u32 = 6;
pub const XA_NOMIGRATE: u32 = 9;
pub const XA_OK: u32 = 0;
pub type XA_OPEN_EPT = Option<unsafe extern "C" fn(param0: windows_sys::core::PCSTR, param1: i32, param2: i32) -> i32>;
pub type XA_PREPARE_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_RBBASE: u32 = 100;
pub const XA_RBCOMMFAIL: u32 = 101;
pub const XA_RBDEADLOCK: u32 = 102;
pub const XA_RBEND: u32 = 107;
pub const XA_RBINTEGRITY: u32 = 103;
pub const XA_RBOTHER: u32 = 104;
pub const XA_RBPROTO: u32 = 105;
pub const XA_RBROLLBACK: u32 = 100;
pub const XA_RBTIMEOUT: u32 = 106;
pub const XA_RBTRANSIENT: u32 = 107;
pub const XA_RDONLY: u32 = 3;
pub type XA_RECOVER_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32, param3: i32) -> i32>;
pub const XA_RETRY: u32 = 4;
pub type XA_ROLLBACK_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub type XA_START_EPT = Option<unsafe extern "C" fn(param0: *mut XID, param1: i32, param2: i32) -> i32>;
pub const XA_SWITCH_F_DTC: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XID {
    pub formatID: i32,
    pub gtrid_length: i32,
    pub bqual_length: i32,
    pub data: [i8; 128],
}
impl Default for XID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XIDDATASIZE: u32 = 128;
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = 65535;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xa_switch_t {
    pub name: [i8; 32],
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
impl Default for xa_switch_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
