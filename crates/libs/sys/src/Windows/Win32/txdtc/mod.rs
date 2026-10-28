pub type DTCINITIATEDRECOVERYWORK = i32;
pub const DTCINITIATEDRECOVERYWORK_CHECKLUSTATUS: DTCINITIATEDRECOVERYWORK = 1;
pub const DTCINITIATEDRECOVERYWORK_TMDOWN: DTCINITIATEDRECOVERYWORK = 3;
pub const DTCINITIATEDRECOVERYWORK_TRANS: DTCINITIATEDRECOVERYWORK = 2;
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
pub const IID_IDtcLuConfigure: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e760_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecovery: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac2b8ad2_d6f0_11d0_b386_00a0c9083365);
pub const IID_IDtcLuRecoveryFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e762_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByDtc: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e764_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByDtcStatusWork: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e766_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByDtcTransWork: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e765_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByLu: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e768_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRecoveryInitiatedByLuWork: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac2b8ad1_d6f0_11d0_b386_00a0c9083365);
pub const IID_IDtcLuRmEnlistment: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e769_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRmEnlistmentFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e771_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuRmEnlistmentSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e770_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuSubordinateDtc: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e773_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuSubordinateDtcFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e775_1aea_11d0_944b_00a0c905416e);
pub const IID_IDtcLuSubordinateDtcSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4131e774_1aea_11d0_944b_00a0c905416e);
pub const IID_IGetDispenser: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc23cc370_87ef_11ce_8081_0080c758527e);
pub const IID_ILastResourceManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4d964ad4_5b33_11d3_8a91_00c04f79eb6d);
pub const IID_IPrepareInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x80c7bfd0_87ee_11ce_8081_0080c758527e);
pub const IID_IPrepareInfo2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5fab2547_9779_11d1_b886_00c04fb9618a);
pub const IID_IRMHelper: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe793f6d1_f53d_11cf_a60d_00a0c905416e);
pub const IID_IResourceManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a868ac_0ad4_11ce_b03a_0020af0ba770);
pub const IID_IResourceManager2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd136c69a_f749_11d1_8f47_00c04f8ee57d);
pub const IID_IResourceManagerFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13741d20_87eb_11ce_8081_0080c758527e);
pub const IID_IResourceManagerFactory2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6b369c21_fbd2_11d1_8f47_00c04f8ee57d);
pub const IID_IResourceManagerRejoinable: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f6de620_b5df_4f3e_9cfa_c8aebd05172b);
pub const IID_IResourceManagerSink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d563181_defb_11ce_aed1_00aa0051e2c4);
pub const IID_ITransactionPhase0EnlistmentAsync: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82dc88e1_a954_11d1_8f88_00600895e7d5);
pub const IID_ITransactionPhase0Factory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82dc88e0_a954_11d1_8f88_00600895e7d5);
pub const IID_ITransactionPhase0NotifyAsync: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef081809_0c76_11d2_87a6_00c04f990f34);
pub const IID_ITransactionReceiver: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59313e03_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionReceiverFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59313e02_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionTransmitter: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59313e01_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionTransmitterFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x59313e00_b36c_11cf_a539_00aa006887c3);
pub const IID_ITransactionVoterBallotAsync2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5433376c_414d_11d3_b206_00c04fc2f3ef);
pub const IID_ITransactionVoterFactory2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5433376a_414d_11d3_b206_00c04fc2f3ef);
pub const IID_ITransactionVoterNotifyAsync2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5433376b_414d_11d3_b206_00c04fc2f3ef);
pub const IID_IXAConfig: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc8a6e3a1_9a8c_11cf_a308_00a0c905416e);
pub const IID_IXAObtainRMInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe793f6d2_f53d_11cf_a60d_00a0c905416e);
pub const IID_IXATransLookup: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf3b1f131_eeda_11ce_aed4_00aa0051e2c4);
pub const IID_IXATransLookup2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf193c85_0d1a_4290_b88f_d2cb8873d1e7);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROXY_CONFIG_PARAMS {
    pub wcThreadsMax: super::WORD,
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
pub const dwUSER_MS_SQLSERVER: XACT_DTC_CONSTANTS = 65535;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct xa_switch_t {
    pub name: [i8; 32],
    pub flags: i32,
    pub version: i32,
    pub xa_open_entry: Option<unsafe extern "C" fn(*mut i8, i32, i32) -> i32>,
    pub xa_close_entry: Option<unsafe extern "C" fn(*mut i8, i32, i32) -> i32>,
    pub xa_start_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_end_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_rollback_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_prepare_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_commit_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_recover_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32, i32) -> i32>,
    pub xa_forget_entry: Option<unsafe extern "C" fn(*mut XID, i32, i32) -> i32>,
    pub xa_complete_entry: Option<unsafe extern "C" fn(*mut i32, *mut i32, i32, i32) -> i32>,
}
impl Default for xa_switch_t {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
