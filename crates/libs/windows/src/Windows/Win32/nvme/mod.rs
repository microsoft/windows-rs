#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTIVE_LATENCY_CONFIGURATION {
    pub Anonymous: ACTIVE_LATENCY_CONFIGURATION_0,
}
impl Default for ACTIVE_LATENCY_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union ACTIVE_LATENCY_CONFIGURATION_0 {
    pub Anonymous: ACTIVE_LATENCY_CONFIGURATION_0_0,
    pub AsUshort: u16,
}
impl Default for ACTIVE_LATENCY_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ACTIVE_LATENCY_CONFIGURATION_0_0 {
    pub _bitfield: u16,
}
impl ACTIVE_LATENCY_CONFIGURATION_0_0 {
    pub fn Read0(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Read0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Write0(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Write0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Trim0(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Trim0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Read1(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Read1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Write1(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Write1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Trim1(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Trim1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn Read2(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_Read2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn Write2(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Write2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Trim2(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_Trim2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn Read3(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_Read3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn Write3(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_Write3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn Trim3(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_Trim3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BUCKET_COUNTER {
    pub Reserved: u32,
    pub Trim: u32,
    pub Write: u32,
    pub Read: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_BIT_FIELD {
    pub _bitfield: u16,
}
impl DEBUG_BIT_FIELD {
    pub fn Read0(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Read0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Write0(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Write0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Trim0(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Trim0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Read1(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Read1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Write1(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Write1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Trim1(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Trim1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn Read2(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_Read2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn Write2(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Write2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Trim2(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_Trim2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn Read3(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_Read3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn Write3(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_Write3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn Trim3(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_Trim3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSSD_POWER_STATE_DESCRIPTOR {
    pub _bitfield: u8,
}
impl DSSD_POWER_STATE_DESCRIPTOR {
    pub fn NvmePowerState(&self) -> u8 {
        (self._bitfield << 3) >> 3
    }
    pub fn set_NvmePowerState(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn Reserved(&self) -> u8 {
        (self._bitfield << 1) >> 6
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 5)) | ((value & 3) << 5);
    }
    pub fn ValidDSSDPowerState(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_ValidDSSDPowerState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct FIRMWARE_ACTIVATION_HISTORY_ENTRY {
    pub VersionNumber: u8,
    pub Length: u8,
    pub Reserved0: u16,
    pub ActivationCount: u16,
    pub Timestamp: u64,
    pub Reserved1: u64,
    pub PowerCycleCount: u64,
    pub PreviousFirmware: u64,
    pub NewFirmware: u64,
    pub SlotNumber: u8,
    pub CommitActionType: u8,
    pub Result: u16,
    pub Reserved2: [u8; 14],
}
impl Default for FIRMWARE_ACTIVATION_HISTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIRMWARE_ACTIVATION_HISTORY_ENTRY_VERSION_1: u32 = 1;
pub const GUID_MFND_CHILD_CONTROLLER_EVENT_LOG_PAGE: windows_core::GUID = windows_core::GUID::from_u128(0x98bcce18_a5f0_bf35_a544_d97f259d669c);
pub const GUID_MFND_CHILD_CONTROLLER_QOS_STAT_LOG_PAGE: windows_core::GUID = windows_core::GUID::from_u128(0x9cb5fa26_0652_4644_873e_400084575f0f);
pub const GUID_OCP_DEVICE_DEVICE_CAPABILITIES: windows_core::GUID = windows_core::GUID::from_u128(0x0d054297_e1d1_98c9_5d49_584b913c05b7);
pub const GUID_OCP_DEVICE_ERROR_RECOVERY: windows_core::GUID = windows_core::GUID::from_u128(0x2131d944_30fe_ae34_ab4d_fd3dba83195a);
pub const GUID_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x769a796d_dab4_a3f6_e24d_b28aacf31cd1);
pub const GUID_OCP_DEVICE_LATENCY_MONITOR: windows_core::GUID = windows_core::GUID::from_u128(0x8cc07a92_84d0_9c6c_7043_e6d4585ed485);
pub const GUID_OCP_DEVICE_SMART_INFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x2810afc5_bfea_a4f2_9c4f_6f7cc914d5af);
pub const GUID_OCP_DEVICE_TCG_CONFIGURATION: windows_core::GUID = windows_core::GUID::from_u128(0xbd244006_e07e_83e6_c047_54fa9d2ae054);
pub const GUID_OCP_DEVICE_TCG_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x704b513e_09c6_9490_274e_d0969690d788);
pub const GUID_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS: windows_core::GUID = windows_core::GUID::from_u128(0x0e9c722f_2399_bb2c_6348_32d0b798bbc7);
pub const GUID_OCP_DSSD_SPECIFIC_INFORMATION: windows_core::GUID = windows_core::GUID::from_u128(0x5bd594c1_94e0_9447_a21d_29998f56be6f);
pub const GUID_WCS_DEVICE_ERROR_RECOVERY: windows_core::GUID = windows_core::GUID::from_u128(0x2131d944_30fe_ae34_ab4d_fd3dba83195a);
pub const GUID_WCS_DEVICE_SMART_ATTRIBUTES: windows_core::GUID = windows_core::GUID::from_u128(0x2810afc5_bfea_a4f2_9c4f_6f7cc914d5af);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_COMMAND_SET_VECTOR {
    pub _bitfield: u64,
}
impl IO_COMMAND_SET_VECTOR {
    pub fn NVMCommandSet(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_NVMCommandSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u64);
    }
    pub fn KVCommandSet(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_KVCommandSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u64) << 1);
    }
    pub fn ZNCommandSet(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ZNCommandSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u64) << 2);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(2305843009213693951 << 3)) | ((value & 2305843009213693951) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LATENCY_MONITOR_FEATURE_STATUS {
    pub Anonymous: LATENCY_MONITOR_FEATURE_STATUS_0,
}
impl Default for LATENCY_MONITOR_FEATURE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LATENCY_MONITOR_FEATURE_STATUS_0 {
    pub Anonymous: LATENCY_MONITOR_FEATURE_STATUS_0_0,
    pub AsUchar: u8,
}
impl Default for LATENCY_MONITOR_FEATURE_STATUS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LATENCY_MONITOR_FEATURE_STATUS_0_0 {
    pub _bitfield: u8,
}
impl LATENCY_MONITOR_FEATURE_STATUS_0_0 {
    pub fn FeatureEnabled(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_FeatureEnabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn ActiveLatencyMode(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ActiveLatencyMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn ActiveMeasuredLatency(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ActiveMeasuredLatency(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_STAMP {
    pub Trim3: u64,
    pub Write3: u64,
    pub Read3: u64,
    pub Trim2: u64,
    pub Write2: u64,
    pub Read2: u64,
    pub Trim1: u64,
    pub Write1: u64,
    pub Read1: u64,
    pub Trim0: u64,
    pub Write0: u64,
    pub Read0: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_STAMP_UNITS {
    pub _bitfield: u16,
}
impl LATENCY_STAMP_UNITS {
    pub fn Read0(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Read0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Write0(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Write0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Trim0(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Trim0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Read1(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Read1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Write1(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_Write1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Trim1(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Trim1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn Read2(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_Read2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn Write2(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Write2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Trim2(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_Trim2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn Read3(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_Read3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn Write3(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_Write3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn Trim3(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_Trim3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEASURED_LATENCY {
    pub Trim3: u16,
    pub Write3: u16,
    pub Read3: u16,
    pub Trim2: u16,
    pub Write2: u16,
    pub Read2: u16,
    pub Trim1: u16,
    pub Write1: u16,
    pub Read1: u16,
    pub Trim0: u16,
    pub Write0: u16,
    pub Read0: u16,
}
pub type NVMEOF_ADDRESS_FAMILY = i32;
pub const NVMEOF_ADMINQ_MAX_DEPTH: u32 = 4096;
pub const NVMEOF_ADMINQ_MIN_DEPTH: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_CHALLENGE {
    pub AUTH_TYPE: u8,
    pub AUTH_ID: u8,
    pub Reserved0: u16,
    pub T_ID: u16,
    pub HL: u8,
    pub Reserved1: u8,
    pub HashID: u8,
    pub DHgID: u8,
    pub DHVLEN: u16,
    pub SEQNUM: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_DESCRIPTOR {
    pub AuthId: u8,
    pub Reserved0: u8,
    pub HALEN: u8,
    pub DHLEN: u8,
    pub IdList: [u8; 60],
}
impl Default for NVMEOF_AUTH_DHCHAP_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVMEOF_AUTH_DHCHAP_GROUP_ID = i32;
pub type NVMEOF_AUTH_DHCHAP_HASH_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_REPLY {
    pub AUTH_TYPE: u8,
    pub AUTH_ID: u8,
    pub Reserved0: u16,
    pub T_ID: u16,
    pub HL: u8,
    pub Reserved1: u8,
    pub CVALID: u8,
    pub Reserved2: u8,
    pub DHVLEN: u16,
    pub SEQNUM: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_SUCCESS1 {
    pub AUTH_TYPE: u8,
    pub AUTH_ID: u8,
    pub Reserved0: u16,
    pub T_ID: u16,
    pub HL: u8,
    pub Reserved1: u8,
    pub RVALID: u8,
    pub Reserved2: [u8; 7],
}
impl Default for NVMEOF_AUTH_DHCHAP_SUCCESS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_SUCCESS2 {
    pub AUTH_TYPE: u8,
    pub AUTH_ID: u8,
    pub Reserved0: u16,
    pub T_ID: u16,
    pub Reserved1: [u8; 10],
}
impl Default for NVMEOF_AUTH_DHCHAP_SUCCESS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_FAILURE {
    pub AUTH_TYPE: u8,
    pub AUTH_ID: u8,
    pub Reserved0: u16,
    pub T_ID: u16,
    pub ReasonCode: u8,
    pub ReasonExplanation: u8,
}
pub type NVMEOF_AUTH_FAIL_REASON_CODE = i32;
pub type NVMEOF_AUTH_FAIL_REASON_EXPLANATION = i32;
pub type NVMEOF_AUTH_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_NEGOTIATE {
    pub AUTH_TYPE: u8,
    pub AUTH_ID: u8,
    pub Reserved0: u16,
    pub T_ID: u16,
    pub SC_C: u8,
    pub NAPD: u8,
}
pub type NVMEOF_AUTH_PROTOCOL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_AUTH_RECEIVE_COMMAND {
    pub OPC: u8,
    pub Reserved0: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved1: [u8; 19],
    pub SGL1: NVME_SGL_DESC,
    pub Reserved2: u8,
    pub SPSP0: u8,
    pub SPSP1: u8,
    pub SECP: u8,
    pub AL: u32,
    pub Reserved3: [u8; 16],
}
impl Default for NVMEOF_AUTH_RECEIVE_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_RECEIVE_RESPONSE {
    pub Reserved0: u64,
    pub SQHD: u16,
    pub Reserved1: u16,
    pub CID: u16,
    pub STS: u16,
}
pub type NVMEOF_AUTH_SECURE_CHANNEL = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_AUTH_SEND_COMMAND {
    pub OPC: u8,
    pub Reserved0: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved1: [u8; 19],
    pub SGL1: NVME_SGL_DESC,
    pub Reserved2: u8,
    pub SPSP0: u8,
    pub SPSP1: u8,
    pub SECP: u8,
    pub TL: u32,
    pub Reserved3: [u8; 16],
}
impl Default for NVMEOF_AUTH_SEND_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_SEND_RESPONSE {
    pub Reserved0: u64,
    pub SQHD: u16,
    pub Reserved1: u16,
    pub CID: u16,
    pub STS: u16,
}
pub type NVMEOF_AUTH_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_CONNECT_COMMAND {
    pub OPC: u8,
    pub Reserved0: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved1: [u8; 19],
    pub SGL1: NVME_SGL_DESC,
    pub RECFMT: u16,
    pub QID: u16,
    pub SQSIZE: u16,
    pub CATTR: NVMEOF_CONNECT_COMMAND_0,
    pub Reserved2: u8,
    pub KATO: u32,
    pub Reserved3: [u8; 12],
}
impl Default for NVMEOF_CONNECT_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVMEOF_CONNECT_COMMAND_0 {
    pub Anonymous: NVMEOF_CONNECT_COMMAND_0_0,
    pub AsUchar: u8,
}
impl Default for NVMEOF_CONNECT_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_CONNECT_COMMAND_0_0 {
    pub _bitfield: u8,
}
impl NVMEOF_CONNECT_COMMAND_0_0 {
    pub fn PriorityClass(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_PriorityClass(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn SqFlowControlDisable(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SqFlowControlDisable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn IoQueueDeletion(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_IoQueueDeletion(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_CONNECT_DATA {
    pub HOSTID: [u8; 16],
    pub CNTLID: u16,
    pub Reserved0: [u8; 238],
    pub SUBNQN: [u8; 256],
    pub HOSTNQN: [u8; 256],
    pub Reserved1: [u8; 256],
}
impl Default for NVMEOF_CONNECT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_CONNECT_RESPONSE {
    pub SCSpecific: NVMEOF_CONNECT_RESPONSE_0,
    pub Reserved0: u32,
    pub SQHD: u16,
    pub Reserved1: u16,
    pub CID: u16,
    pub STS: u16,
}
impl Default for NVMEOF_CONNECT_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVMEOF_CONNECT_RESPONSE_0 {
    pub Success: NVMEOF_CONNECT_RESPONSE_0_0,
    pub AsUlong: u32,
}
impl Default for NVMEOF_CONNECT_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_CONNECT_RESPONSE_0_0 {
    pub CNTLID: u16,
    pub AUTHREQ: NVMEOF_CONNECT_RESPONSE_0_0_0,
}
impl Default for NVMEOF_CONNECT_RESPONSE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVMEOF_CONNECT_RESPONSE_0_0_0 {
    pub Anonymous: NVMEOF_CONNECT_RESPONSE_0_0_0_0,
    pub AsUshort: u16,
}
impl Default for NVMEOF_CONNECT_RESPONSE_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_CONNECT_RESPONSE_0_0_0_0 {
    pub _bitfield: u16,
}
impl NVMEOF_CONNECT_RESPONSE_0_0_0_0 {
    pub fn Obsolete(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Obsolete(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn ATR(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ATR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn ASCR(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ASCR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
pub const NVMEOF_DHCHAP_PREFIX_V1: windows_core::PCSTR = windows_core::s!("DHHC-1:");
pub const NVMEOF_DHCHAP_PROTOCOL_ID: u32 = 1;
pub const NVMEOF_DHCHAP_REPLY_CVAL_NOTVALID: u32 = 0;
pub const NVMEOF_DHCHAP_REPLY_CVAL_VALID: u32 = 1;
pub const NVMEOF_DHCHAP_SUCCESS1_RVAL_NOTVALID: u32 = 0;
pub const NVMEOF_DHCHAP_SUCCESS1_RVAL_VALID: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_DISCONNECT_COMMAND {
    pub OPC: u8,
    pub Reserved0: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved1: [u8; 19],
    pub SGL1: NVME_SGL_DATABLOCK_DESC,
    pub RECFMT: u16,
    pub Reserved2: [u8; 22],
}
impl Default for NVMEOF_DISCONNECT_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_DISCONNECT_RESPONSE {
    pub Reserved0: u64,
    pub SQHD: u16,
    pub Reserved1: u16,
    pub CID: u16,
    pub STS: u16,
}
pub const NVMEOF_DISCOVERY_LOG_VERSION_0: u32 = 0;
pub const NVMEOF_DISCOVERY_NQN: windows_core::PCSTR = windows_core::s!("nqn.2014-08.org.nvmexpress.discovery");
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_FABRICS_COMMAND {
    pub OPC: u8,
    pub PSDT: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved: [u8; 35],
    pub Specific: [u8; 24],
}
impl Default for NVMEOF_FABRICS_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_FABRICS_RESPONSE {
    pub Specific: [u8; 8],
    pub SQHD: u16,
    pub Reserved: u16,
    pub CID: u16,
    pub STS: u16,
}
impl Default for NVMEOF_FABRICS_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVMEOF_IOQ_MAX_DEPTH: u32 = 65536;
pub const NVMEOF_IOQ_MIN_DEPTH: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_GET_COMMAND {
    pub OPC: u8,
    pub Reserved0: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved1: [u8; 35],
    pub ATTRIB: NVMEOF_PROPERTY_GET_COMMAND_0,
    pub Reserved2: [u8; 3],
    pub OFST: u32,
    pub Reserved3: [u8; 16],
}
impl Default for NVMEOF_PROPERTY_GET_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_GET_COMMAND_0 {
    pub _bitfield: u8,
}
impl NVMEOF_PROPERTY_GET_COMMAND_0 {
    pub fn PropertySize(&self) -> u8 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_PropertySize(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_PROPERTY_GET_RESPONSE {
    pub VALUE: NVMEOF_PROPERTY_GET_RESPONSE_0,
    pub SQHD: u16,
    pub Reserved0: u16,
    pub CID: u16,
    pub STS: u16,
}
impl Default for NVMEOF_PROPERTY_GET_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVMEOF_PROPERTY_GET_RESPONSE_0 {
    pub FourBytes: NVMEOF_PROPERTY_GET_RESPONSE_0_0,
    pub EightBytes: u64,
}
impl Default for NVMEOF_PROPERTY_GET_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_GET_RESPONSE_0_0 {
    pub Value: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVMEOF_PROPERTY_SET_COMMAND {
    pub OPC: u8,
    pub Reserved0: u8,
    pub CID: u16,
    pub FCTYPE: u8,
    pub Reserved1: [u8; 35],
    pub ATTRIB: NVMEOF_PROPERTY_SET_COMMAND_0,
    pub Reserved2: [u8; 3],
    pub OFST: u32,
    pub VALUE: NVMEOF_PROPERTY_SET_COMMAND_1,
    pub Reserved3: [u8; 8],
}
impl Default for NVMEOF_PROPERTY_SET_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_SET_COMMAND_0 {
    pub _bitfield: u8,
}
impl NVMEOF_PROPERTY_SET_COMMAND_0 {
    pub fn PropertySize(&self) -> u8 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_PropertySize(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVMEOF_PROPERTY_SET_COMMAND_1 {
    pub FourBytes: NVMEOF_PROPERTY_SET_COMMAND_1_0,
    pub EightBytes: u64,
}
impl Default for NVMEOF_PROPERTY_SET_COMMAND_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_SET_COMMAND_1_0 {
    pub Value: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_SET_RESPONSE {
    pub Reserved0: u64,
    pub SQHD: u16,
    pub Reserved1: u16,
    pub CID: u16,
    pub STS: u16,
}
pub const NVMEOF_PROPERTY_SIZE_4Bytes: u32 = 0;
pub const NVMEOF_PROPERTY_SIZE_8Bytes: u32 = 1;
pub type NVMEOF_SECURE_CHANNEL = i32;
pub type NVMEOF_SECURE_CHANNEL_PROTOCOL = i32;
pub type NVMEOF_SUBSYSTEM_TYPE = i32;
pub const NVMEOF_TRANSPORT_ADDR_MAX_LEN: u32 = 256;
pub const NVMEOF_TRANSPORT_SAS_MAX_LEN: u32 = 256;
pub const NVMEOF_TRANSPORT_SERVID_MAX_LEN: u32 = 32;
pub type NVMEOF_TRANSPORT_TYPE = i32;
pub type NVME_ACCESS_FREQUENCIES = i32;
pub const NVME_ACCESS_FREQUENCY_FR_WRITE_FR_READ: NVME_ACCESS_FREQUENCIES = 5;
pub const NVME_ACCESS_FREQUENCY_FR_WRITE_INFR_READ: NVME_ACCESS_FREQUENCIES = 4;
pub const NVME_ACCESS_FREQUENCY_INFR_WRITE_FR_READ: NVME_ACCESS_FREQUENCIES = 3;
pub const NVME_ACCESS_FREQUENCY_INFR_WRITE_INFR_READ: NVME_ACCESS_FREQUENCIES = 2;
pub const NVME_ACCESS_FREQUENCY_NONE: NVME_ACCESS_FREQUENCIES = 0;
pub const NVME_ACCESS_FREQUENCY_ONE_TIME_READ: NVME_ACCESS_FREQUENCIES = 6;
pub const NVME_ACCESS_FREQUENCY_SPECULATIVE_READ: NVME_ACCESS_FREQUENCIES = 7;
pub const NVME_ACCESS_FREQUENCY_TYPICAL: NVME_ACCESS_FREQUENCIES = 1;
pub const NVME_ACCESS_FREQUENCY_WILL_BE_OVERWRITTEN: NVME_ACCESS_FREQUENCIES = 8;
pub type NVME_ACCESS_LATENCIES = i32;
pub const NVME_ACCESS_LATENCY_IDLE: NVME_ACCESS_LATENCIES = 1;
pub const NVME_ACCESS_LATENCY_LOW: NVME_ACCESS_LATENCIES = 3;
pub const NVME_ACCESS_LATENCY_NONE: NVME_ACCESS_LATENCIES = 0;
pub const NVME_ACCESS_LATENCY_NORMAL: NVME_ACCESS_LATENCIES = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ACTIVE_NAMESPACE_ID_LIST {
    pub NSID: [u32; 1024],
}
impl Default for NVME_ACTIVE_NAMESPACE_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_ADMINQ_ID: u32 = 0;
pub type NVME_ADMIN_COMMANDS = i32;
pub const NVME_ADMIN_COMMAND_ABORT: NVME_ADMIN_COMMANDS = 8;
pub const NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST: NVME_ADMIN_COMMANDS = 12;
pub const NVME_ADMIN_COMMAND_CREATE_IO_CQ: NVME_ADMIN_COMMANDS = 5;
pub const NVME_ADMIN_COMMAND_CREATE_IO_SQ: NVME_ADMIN_COMMANDS = 1;
pub const NVME_ADMIN_COMMAND_DELETE_IO_CQ: NVME_ADMIN_COMMANDS = 4;
pub const NVME_ADMIN_COMMAND_DELETE_IO_SQ: NVME_ADMIN_COMMANDS = 0;
pub const NVME_ADMIN_COMMAND_DEVICE_SELF_TEST: NVME_ADMIN_COMMANDS = 20;
pub const NVME_ADMIN_COMMAND_DIRECTIVE_RECEIVE: NVME_ADMIN_COMMANDS = 26;
pub const NVME_ADMIN_COMMAND_DIRECTIVE_SEND: NVME_ADMIN_COMMANDS = 25;
pub const NVME_ADMIN_COMMAND_DISCOVERY_INFO_MGMT: NVME_ADMIN_COMMANDS = 33;
pub const NVME_ADMIN_COMMAND_DOORBELL_BUFFER_CONFIG: NVME_ADMIN_COMMANDS = 124;
pub const NVME_ADMIN_COMMAND_FABRICS: NVME_ADMIN_COMMANDS = 127;
pub const NVME_ADMIN_COMMAND_FIRMWARE_ACTIVATE: NVME_ADMIN_COMMANDS = 16;
pub const NVME_ADMIN_COMMAND_FIRMWARE_COMMIT: NVME_ADMIN_COMMANDS = 16;
pub const NVME_ADMIN_COMMAND_FIRMWARE_IMAGE_DOWNLOAD: NVME_ADMIN_COMMANDS = 17;
pub const NVME_ADMIN_COMMAND_FORMAT_NVM: NVME_ADMIN_COMMANDS = 128;
pub const NVME_ADMIN_COMMAND_GET_FEATURES: NVME_ADMIN_COMMANDS = 10;
pub const NVME_ADMIN_COMMAND_GET_LBA_STATUS: NVME_ADMIN_COMMANDS = 134;
pub const NVME_ADMIN_COMMAND_GET_LOG_PAGE: NVME_ADMIN_COMMANDS = 2;
pub const NVME_ADMIN_COMMAND_IDENTIFY: NVME_ADMIN_COMMANDS = 6;
pub const NVME_ADMIN_COMMAND_KEEP_ALIVE: NVME_ADMIN_COMMANDS = 24;
pub const NVME_ADMIN_COMMAND_NAMESPACE_ATTACHMENT: NVME_ADMIN_COMMANDS = 21;
pub const NVME_ADMIN_COMMAND_NAMESPACE_MANAGEMENT: NVME_ADMIN_COMMANDS = 13;
pub const NVME_ADMIN_COMMAND_NVME_MI_RECEIVE: NVME_ADMIN_COMMANDS = 30;
pub const NVME_ADMIN_COMMAND_NVME_MI_SEND: NVME_ADMIN_COMMANDS = 29;
pub const NVME_ADMIN_COMMAND_SANITIZE: NVME_ADMIN_COMMANDS = 132;
pub const NVME_ADMIN_COMMAND_SECURITY_RECEIVE: NVME_ADMIN_COMMANDS = 130;
pub const NVME_ADMIN_COMMAND_SECURITY_SEND: NVME_ADMIN_COMMANDS = 129;
pub const NVME_ADMIN_COMMAND_SET_FEATURES: NVME_ADMIN_COMMANDS = 9;
pub const NVME_ADMIN_COMMAND_VIRTUALIZATION_MANAGEMENT: NVME_ADMIN_COMMANDS = 28;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    pub Anonymous: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0 {
    pub _bitfield: u64,
}
impl NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0 {
    pub fn Reserved0(&self) -> u64 {
        (self._bitfield << 52) >> 52
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn ACQB(&self) -> u64 {
        self._bitfield >> 12
    }
    pub fn set_ACQB(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4503599627370495 << 12)) | ((value & 4503599627370495) << 12);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_QUEUE_ATTRIBUTES {
    pub Anonymous: NVME_ADMIN_QUEUE_ATTRIBUTES_0,
    pub AsUlong: u32,
}
impl Default for NVME_ADMIN_QUEUE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ADMIN_QUEUE_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
impl NVME_ADMIN_QUEUE_ATTRIBUTES_0 {
    pub fn ASQS(&self) -> u32 {
        (self._bitfield << 20) >> 20
    }
    pub fn set_ASQS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 28
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn ACQS(&self) -> u32 {
        (self._bitfield << 4) >> 20
    }
    pub fn set_ACQS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(4095 << 16)) | ((value & 4095) << 16);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 28
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 28)) | ((value & 15) << 28);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    pub Anonymous: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0 {
    pub _bitfield: u64,
}
impl NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0 {
    pub fn Reserved0(&self) -> u64 {
        (self._bitfield << 52) >> 52
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn ASQB(&self) -> u64 {
        self._bitfield >> 12
    }
    pub fn set_ASQB(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4503599627370495 << 12)) | ((value & 4503599627370495) << 12);
    }
}
pub type NVME_AMS_OPTION = i32;
pub const NVME_AMS_ROUND_ROBIN: NVME_AMS_OPTION = 0;
pub const NVME_AMS_WEIGHTED_ROUND_ROBIN_URGENT: NVME_AMS_OPTION = 1;
pub const NVME_ASYNC_ERROR_DIAG_FAILURE: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 2;
pub const NVME_ASYNC_ERROR_FIRMWARE_IMAGE_LOAD_ERROR: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 5;
pub const NVME_ASYNC_ERROR_INVALID_DOORBELL_WRITE_VALUE: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 1;
pub const NVME_ASYNC_ERROR_PERSISTENT_INTERNAL_DEVICE_ERROR: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 3;
pub const NVME_ASYNC_ERROR_TRANSIENT_INTERNAL_DEVICE_ERROR: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 4;
pub const NVME_ASYNC_ERROR_WRITE_TO_INVALID_DOORBELL_REGISTER: NVME_ASYNC_EVENT_ERROR_STATUS_CODES = 0;
pub type NVME_ASYNC_EVENT_ERROR_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_IMMEDIATE_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = i32;
pub type NVME_ASYNC_EVENT_NOTICE_CODES = i32;
pub type NVME_ASYNC_EVENT_TYPES = i32;
pub const NVME_ASYNC_EVENT_TYPE_ERROR_STATUS: NVME_ASYNC_EVENT_TYPES = 0;
pub const NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS: NVME_ASYNC_EVENT_TYPES = 1;
pub const NVME_ASYNC_EVENT_TYPE_IMMEDIATE: NVME_ASYNC_EVENT_TYPES = 3;
pub const NVME_ASYNC_EVENT_TYPE_IO_COMMAND_SET_STATUS: NVME_ASYNC_EVENT_TYPES = 6;
pub const NVME_ASYNC_EVENT_TYPE_NOTICE: NVME_ASYNC_EVENT_TYPES = 2;
pub const NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC: NVME_ASYNC_EVENT_TYPES = 7;
pub type NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES = i32;
pub const NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_DEVICE_PANIC: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES = 1;
pub const NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_RESERVED: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES = 0;
pub const NVME_ASYNC_HEALTH_NVM_SUBSYSTEM_RELIABILITY: NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = 0;
pub const NVME_ASYNC_HEALTH_SPARE_BELOW_THRESHOLD: NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = 2;
pub const NVME_ASYNC_HEALTH_TEMPERATURE_THRESHOLD: NVME_ASYNC_EVENT_HEALTH_STATUS_CODES = 1;
pub const NVME_ASYNC_IMMEDIATE_NVM_SUBSYSTEM_NORMAL_SHUTDOWN: NVME_ASYNC_EVENT_IMMEDIATE_STATUS_CODES = 0;
pub const NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED: NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = 1;
pub const NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED_WITH_UNEXPECTED_DEALLOCATION: NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = 2;
pub const NVME_ASYNC_IO_CMD_SET_RESERVATION_LOG_PAGE_AVAILABLE: NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES = 0;
pub const NVME_ASYNC_NOTICE_ASYMMETRIC_ACCESS_CHANGE: NVME_ASYNC_EVENT_NOTICE_CODES = 3;
pub const NVME_ASYNC_NOTICE_DISCOVERY_LOG_PAGE_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 240;
pub const NVME_ASYNC_NOTICE_ENDURANCE_GROUP_EVENT_AGGREGATE_LOG_CHANGE: NVME_ASYNC_EVENT_NOTICE_CODES = 6;
pub const NVME_ASYNC_NOTICE_FIRMWARE_ACTIVATION_STARTING: NVME_ASYNC_EVENT_NOTICE_CODES = 1;
pub const NVME_ASYNC_NOTICE_LBA_STATUS_INFORMATION_ALERT: NVME_ASYNC_EVENT_NOTICE_CODES = 5;
pub const NVME_ASYNC_NOTICE_NAMESPACE_ATTRIBUTE_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 0;
pub const NVME_ASYNC_NOTICE_PREDICTABLE_LATENCY_EVENT_AGGREGATE_LOG_CHANGE: NVME_ASYNC_EVENT_NOTICE_CODES = 4;
pub const NVME_ASYNC_NOTICE_TELEMETRY_LOG_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 2;
pub const NVME_ASYNC_NOTICE_ZONE_DESCRIPTOR_CHANGED: NVME_ASYNC_EVENT_NOTICE_CODES = 239;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    pub _bitfield: u32,
    pub Reserved1: u32,
}
impl NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IdleTransitionPowerState(&self) -> u32 {
        (self._bitfield << 24) >> 27
    }
    pub fn set_IdleTransitionPowerState(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
    pub fn IdleTimePriorToTransition(&self) -> u32 {
        self._bitfield >> 8
    }
    pub fn set_IdleTimePriorToTransition(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_BOOT_PARTITION_INFORMATION {
    pub Anonymous: NVME_BOOT_PARTITION_INFORMATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_BOOT_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_BOOT_PARTITION_INFORMATION_0 {
    pub _bitfield: u32,
}
impl NVME_BOOT_PARTITION_INFORMATION_0 {
    pub fn BPSZ(&self) -> u32 {
        (self._bitfield << 17) >> 17
    }
    pub fn set_BPSZ(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !32767) | (value & 32767);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 8) >> 23
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(511 << 15)) | ((value & 511) << 15);
    }
    pub fn BRS(&self) -> u32 {
        (self._bitfield << 6) >> 30
    }
    pub fn set_BRS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 24)) | ((value & 3) << 24);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 1) >> 27
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(31 << 26)) | ((value & 31) << 26);
    }
    pub fn ABPID(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_ABPID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_BOOT_PARTITION_LOG {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 3],
    pub BpInfo: NVME_BOOT_PARTITION_INFORMATION,
    pub Reserved1: [u8; 8],
    pub BootPartitionData: [u8; 1],
}
impl Default for NVME_BOOT_PARTITION_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_CC_SHN_ABRUPT_SHUTDOWN: NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = 2;
pub const NVME_CC_SHN_NORMAL_SHUTDOWN: NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = 1;
pub const NVME_CC_SHN_NO_NOTIFICATION: NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = 0;
pub type NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_DSSD_POWER_STATE {
    pub Anonymous: NVME_CDW0_FEATURE_DSSD_POWER_STATE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW0_FEATURE_DSSD_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_FEATURE_DSSD_POWER_STATE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW0_FEATURE_DSSD_POWER_STATE_0 {
    pub fn DSSDPowerState(&self) -> u32 {
        (self._bitfield << 25) >> 25
    }
    pub fn set_DSSDPowerState(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 7
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    pub Anonymous: NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub _bitfield: u32,
}
impl NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub fn Enabled(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_Enabled(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
pub type NVME_CDW0_FEATURE_ERROR_INJECTION = NVME_CDW11_FEATURE_ERROR_INJECTION;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub Anonymous: NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub fn EOLBehavior(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_EOLBehavior(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_RESERVATION_PERSISTENCE {
    pub _bitfield: u32,
}
impl NVME_CDW0_RESERVATION_PERSISTENCE {
    pub fn PTPL(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PTPL(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_ABORT {
    pub Anonymous: NVME_CDW10_ABORT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_ABORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ABORT_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_ABORT_0 {
    pub fn SQID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_SQID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn CID(&self) -> u32 {
        (self._bitfield << 8) >> 16
    }
    pub fn set_CID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 8)) | ((value & 65535) << 8);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_CREATE_IO_QUEUE {
    pub Anonymous: NVME_CDW10_CREATE_IO_QUEUE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_CREATE_IO_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_CREATE_IO_QUEUE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_CREATE_IO_QUEUE_0 {
    pub fn QID(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_QID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn QSIZE(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_QSIZE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DATASET_MANAGEMENT {
    pub Anonymous: NVME_CDW10_DATASET_MANAGEMENT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_DATASET_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DATASET_MANAGEMENT_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_DATASET_MANAGEMENT_0 {
    pub fn NR(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_NR(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DELETE_IO_QUEUE {
    pub Anonymous: NVME_CDW10_DELETE_IO_QUEUE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_DELETE_IO_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DELETE_IO_QUEUE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_DELETE_IO_QUEUE_0 {
    pub fn QID(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_QID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DEVICE_SELF_TEST {
    pub Anonymous: NVME_CDW10_DEVICE_SELF_TEST_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_DEVICE_SELF_TEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DEVICE_SELF_TEST_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_DEVICE_SELF_TEST_0 {
    pub fn STC(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_STC(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DIRECTIVE_RECEIVE {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DIRECTIVE_SEND {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DISCOVERY_INFO_MGMT {
    pub Anonymous: NVME_CDW10_DISCOVERY_INFO_MGMT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_DISCOVERY_INFO_MGMT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DISCOVERY_INFO_MGMT_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_DISCOVERY_INFO_MGMT_0 {
    pub fn TAS(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_TAS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_FIRMWARE_ACTIVATE {
    pub Anonymous: NVME_CDW10_FIRMWARE_ACTIVATE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_FIRMWARE_ACTIVATE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_FIRMWARE_ACTIVATE_0 {
    pub fn FS(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_FS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn AA(&self) -> u32 {
        (self._bitfield << 26) >> 29
    }
    pub fn set_AA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 1) >> 7
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(33554431 << 6)) | ((value & 33554431) << 6);
    }
    pub fn BPID(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_BPID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_FIRMWARE_DOWNLOAD {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_FORMAT_NVM {
    pub Anonymous: NVME_CDW10_FORMAT_NVM_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_FORMAT_NVM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_FORMAT_NVM_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_FORMAT_NVM_0 {
    pub fn LBAF(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_LBAF(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MS(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_MS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn PI(&self) -> u32 {
        (self._bitfield << 24) >> 29
    }
    pub fn set_PI(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn PIL(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_PIL(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn SES(&self) -> u32 {
        (self._bitfield << 20) >> 29
    }
    pub fn set_SES(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 9)) | ((value & 7) << 9);
    }
    pub fn ZF(&self) -> u32 {
        (self._bitfield << 18) >> 30
    }
    pub fn set_ZF(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 12)) | ((value & 3) << 12);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 14
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(262143 << 14)) | ((value & 262143) << 14);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_FEATURES {
    pub Anonymous: NVME_CDW10_GET_FEATURES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_FEATURES_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_GET_FEATURES_0 {
    pub fn FID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_FID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn SEL(&self) -> u32 {
        (self._bitfield << 21) >> 29
    }
    pub fn set_SEL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 8)) | ((value & 7) << 8);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 11
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2097151 << 11)) | ((value & 2097151) << 11);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_GET_LOG_PAGE_0 {
    pub fn LID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_LID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn NUMD(&self) -> u32 {
        (self._bitfield << 4) >> 20
    }
    pub fn set_NUMD(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(4095 << 16)) | ((value & 4095) << 16);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 28
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 28)) | ((value & 15) << 28);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V121 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V121_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_LOG_PAGE_V121 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_V121_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_GET_LOG_PAGE_V121_0 {
    pub fn LID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_LID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn NUMDL(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_NUMDL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V13 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V13_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_LOG_PAGE_V13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_V13_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_GET_LOG_PAGE_V13_0 {
    pub fn LID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_LID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn LSP(&self) -> u32 {
        (self._bitfield << 20) >> 28
    }
    pub fn set_LSP(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 17) >> 29
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 12)) | ((value & 7) << 12);
    }
    pub fn RAE(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_RAE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u32) << 15);
    }
    pub fn NUMDL(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_NUMDL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V20 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V20_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_GET_LOG_PAGE_V20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_V20_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_GET_LOG_PAGE_V20_0 {
    pub fn LID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_LID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn LSP(&self) -> u32 {
        (self._bitfield << 17) >> 25
    }
    pub fn set_LSP(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(127 << 8)) | ((value & 127) << 8);
    }
    pub fn RAE(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_RAE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u32) << 15);
    }
    pub fn NUMDL(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_NUMDL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_IDENTIFY {
    pub Anonymous: NVME_CDW10_IDENTIFY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_IDENTIFY_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_IDENTIFY_0 {
    pub fn CNS(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_CNS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn CNTID(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_CNTID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
pub const NVME_CDW10_LSP_ACTION_ESTABLISH_CONTEXT_AND_READ_512_BYTES_OF_HEADER: u32 = 3;
pub const NVME_CDW10_LSP_ACTION_ESTABLISH_CONTEXT_AND_READ_LOG_DATA: u32 = 1;
pub const NVME_CDW10_LSP_ACTION_READ_LOG_DATA: u32 = 0;
pub const NVME_CDW10_LSP_ACTION_RELEASE_CONTEXT: u32 = 2;
pub const NVME_CDW10_LSP_BOOT_PARTITION_0: u32 = 0;
pub const NVME_CDW10_LSP_BOOT_PARTITION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_ACQUIRE {
    pub Anonymous: NVME_CDW10_RESERVATION_ACQUIRE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_ACQUIRE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_ACQUIRE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_RESERVATION_ACQUIRE_0 {
    pub fn RACQA(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_RACQA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IEKEY(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_IEKEY(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 24) >> 28
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn RTYPE(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_RTYPE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_REGISTER {
    pub Anonymous: NVME_CDW10_RESERVATION_REGISTER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_REGISTER_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_RESERVATION_REGISTER_0 {
    pub fn RREGA(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_RREGA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IEKEY(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_IEKEY(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 2) >> 6
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(67108863 << 4)) | ((value & 67108863) << 4);
    }
    pub fn CPTPL(&self) -> u32 {
        self._bitfield >> 30
    }
    pub fn set_CPTPL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 30)) | ((value & 3) << 30);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_RELEASE {
    pub Anonymous: NVME_CDW10_RESERVATION_RELEASE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_RELEASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_RELEASE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_RESERVATION_RELEASE_0 {
    pub fn RRELA(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_RRELA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IEKEY(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_IEKEY(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 24) >> 28
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn RTYPE(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_RTYPE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_REPORT {
    pub Anonymous: NVME_CDW10_RESERVATION_REPORT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_RESERVATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_REPORT_0 {
    pub NUMD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SANITIZE {
    pub Anonymous: NVME_CDW10_SANITIZE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_SANITIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_SANITIZE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_SANITIZE_0 {
    pub fn SANACT(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_SANACT(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn AUSE(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_AUSE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn OWPASS(&self) -> u32 {
        (self._bitfield << 24) >> 28
    }
    pub fn set_OWPASS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn OIPBP(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_OIPBP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn NDAS(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_NDAS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 10
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(4194303 << 10)) | ((value & 4194303) << 10);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SECURITY_SEND_RECEIVE {
    pub Anonymous: NVME_CDW10_SECURITY_SEND_RECEIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_SECURITY_SEND_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_SECURITY_SEND_RECEIVE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_SECURITY_SEND_RECEIVE_0 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn SPSP(&self) -> u32 {
        (self._bitfield << 8) >> 16
    }
    pub fn set_SPSP(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 8)) | ((value & 65535) << 8);
    }
    pub fn SECP(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_SECP(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SET_FEATURES {
    pub Anonymous: NVME_CDW10_SET_FEATURES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW10_SET_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_SET_FEATURES_0 {
    pub _bitfield: u32,
}
impl NVME_CDW10_SET_FEATURES_0 {
    pub fn FID(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_FID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 1) >> 9
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8388607 << 8)) | ((value & 8388607) << 8);
    }
    pub fn SV(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_SV(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ZONE_APPEND {
    pub SLBA: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_RECEIVE {
    pub SLBA: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_SEND {
    pub SLBA: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_CREATE_IO_CQ {
    pub Anonymous: NVME_CDW11_CREATE_IO_CQ_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_CREATE_IO_CQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_CREATE_IO_CQ_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_CREATE_IO_CQ_0 {
    pub fn PC(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn IEN(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_IEN(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 18
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
    pub fn IV(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_IV(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_CREATE_IO_SQ {
    pub Anonymous: NVME_CDW11_CREATE_IO_SQ_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_CREATE_IO_SQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_CREATE_IO_SQ_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_CREATE_IO_SQ_0 {
    pub fn PC(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn QPRIO(&self) -> u32 {
        (self._bitfield << 29) >> 30
    }
    pub fn set_QPRIO(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 1)) | ((value & 3) << 1);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 19
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
    pub fn CQID(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_CQID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DATASET_MANAGEMENT {
    pub Anonymous: NVME_CDW11_DATASET_MANAGEMENT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_DATASET_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_DATASET_MANAGEMENT_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_DATASET_MANAGEMENT_0 {
    pub fn IDR(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IDR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn IDW(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_IDW(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn AD(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_AD(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DIRECTIVE_RECEIVE {
    pub Anonymous: NVME_CDW11_DIRECTIVE_RECEIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_DIRECTIVE_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_DIRECTIVE_RECEIVE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_DIRECTIVE_RECEIVE_0 {
    pub fn DOPER(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_DOPER(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn DTYPE(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_DTYPE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn DSPEC(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_DSPEC(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DIRECTIVE_SEND {
    pub Anonymous: NVME_CDW11_DIRECTIVE_SEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_DIRECTIVE_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_DIRECTIVE_SEND_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_DIRECTIVE_SEND_0 {
    pub fn DOPER(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_DOPER(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn DTYPE(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_DTYPE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn DSPEC(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_DSPEC(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURES {
    pub NumberOfQueues: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES,
    pub InterruptCoalescing: NVME_CDW11_FEATURE_INTERRUPT_COALESCING,
    pub InterruptVectorConfig: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG,
    pub LbaRangeType: NVME_CDW11_FEATURE_LBA_RANGE_TYPE,
    pub Arbitration: NVME_CDW11_FEATURE_ARBITRATION,
    pub VolatileWriteCache: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE,
    pub AsyncEventConfig: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG,
    pub PowerManagement: NVME_CDW11_FEATURE_POWER_MANAGEMENT,
    pub AutoPowerStateTransition: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION,
    pub TemperatureThreshold: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD,
    pub ErrorRecovery: NVME_CDW11_FEATURE_ERROR_RECOVERY,
    pub HostMemoryBuffer: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER,
    pub WriteAtomicityNormal: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL,
    pub NonOperationalPowerState: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE,
    pub IoCommandSetProfile: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE,
    pub ErrorInjection: NVME_CDW11_FEATURE_ERROR_INJECTION,
    pub HostIdentifier: NVME_CDW11_FEATURE_HOST_IDENTIFIER,
    pub ReservationPersistence: NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE,
    pub ReservationNotificationMask: NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK,
    pub GetHostMetadata: NVME_CDW11_FEATURE_GET_HOST_METADATA,
    pub SetHostMetadata: NVME_CDW11_FEATURE_SET_HOST_METADATA,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ARBITRATION {
    pub Anonymous: NVME_CDW11_FEATURE_ARBITRATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ARBITRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ARBITRATION_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_ARBITRATION_0 {
    pub fn AB(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_AB(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 27
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
    pub fn LPW(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_LPW(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn MPW(&self) -> u32 {
        (self._bitfield << 8) >> 24
    }
    pub fn set_MPW(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn HPW(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_HPW(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    pub Anonymous: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0 {
    pub fn CriticalWarnings(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_CriticalWarnings(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn NsAttributeNotices(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_NsAttributeNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn FwActivationNotices(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_FwActivationNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn TelemetryLogNotices(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_TelemetryLogNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u32) << 10);
    }
    pub fn ANAChangeNotices(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_ANAChangeNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u32) << 11);
    }
    pub fn PredictableLogChangeNotices(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_PredictableLogChangeNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u32) << 12);
    }
    pub fn LBAStatusNotices(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_LBAStatusNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u32) << 13);
    }
    pub fn EnduranceEventNotices(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_EnduranceEventNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u32) << 14);
    }
    pub fn NormalNVMSubsystemShutdown(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_NormalNVMSubsystemShutdown(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u32) << 15);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 5) >> 21
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2047 << 16)) | ((value & 2047) << 16);
    }
    pub fn ZoneDescriptorNotices(&self) -> bool {
        (self._bitfield >> 27) & 1 != 0
    }
    pub fn set_ZoneDescriptorNotices(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value as u32) << 27);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 1) >> 29
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 28)) | ((value & 7) << 28);
    }
    pub fn DiscoveryLogPageChange(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_DiscoveryLogPageChange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    pub Anonymous: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0 {
    pub fn APSTE(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_APSTE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    pub Anonymous: NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 1) >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !2147483647) | (value & 2147483647);
    }
    pub fn Clear(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_Clear(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    pub Anonymous: NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 1) >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !2147483647) | (value & 2147483647);
    }
    pub fn Clear(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_Clear(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    pub Anonymous: NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 1) >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !2147483647) | (value & 2147483647);
    }
    pub fn Enable(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_Enable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ERROR_INJECTION {
    pub Anonymous: NVME_CDW11_FEATURE_ERROR_INJECTION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ERROR_INJECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ERROR_INJECTION_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_ERROR_INJECTION_0 {
    pub fn NUM(&self) -> u32 {
        (self._bitfield << 25) >> 25
    }
    pub fn set_NUM(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 7
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ERROR_RECOVERY {
    pub Anonymous: NVME_CDW11_FEATURE_ERROR_RECOVERY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_ERROR_RECOVERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ERROR_RECOVERY_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_ERROR_RECOVERY_0 {
    pub fn TLER(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_TLER(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn DULBE(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_DULBE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u32) << 16);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 17
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(32767 << 17)) | ((value & 32767) << 17);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_GET_HOST_METADATA {
    pub Anonymous: NVME_CDW11_FEATURE_GET_HOST_METADATA_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_GET_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_GET_HOST_METADATA_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_GET_HOST_METADATA_0 {
    pub fn GDHM(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_GDHM(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    pub fn EXHID(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EXHID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub fn EHM(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EHM(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn MR(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_MR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    pub Anonymous: NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0 {
    pub fn THR(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_THR(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn TIME(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_TIME(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    pub Anonymous: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0 {
    pub fn IV(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_IV(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn CD(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_CD(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u32) << 16);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 17
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(32767 << 17)) | ((value & 32767) << 17);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    pub Anonymous: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0 {
    pub fn IOCSCI(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_IOCSCI(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    pub Anonymous: NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0 {
    pub fn NUM(&self) -> u32 {
        (self._bitfield << 26) >> 26
    }
    pub fn set_NUM(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !63) | (value & 63);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 6
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    pub Anonymous: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0 {
    pub fn NOPPME(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_NOPPME(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    pub Anonymous: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0 {
    pub fn NSQ(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NSQ(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn NCQ(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_NCQ(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    pub Anonymous: NVME_CDW11_FEATURE_POWER_MANAGEMENT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_POWER_MANAGEMENT_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_POWER_MANAGEMENT_0 {
    pub fn PS(&self) -> u32 {
        (self._bitfield << 27) >> 27
    }
    pub fn set_PS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 5
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(134217727 << 5)) | ((value & 134217727) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub Anonymous: NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 2) >> 2
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !1073741823) | (value & 1073741823);
    }
    pub fn EOLBehavior(&self) -> u32 {
        self._bitfield >> 30
    }
    pub fn set_EOLBehavior(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 30)) | ((value & 3) << 30);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    pub fn Reserved(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn REGPRE(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_REGPRE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn RESREL(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_RESREL(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn RESPRE(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_RESPRE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    pub fn PTPL(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PTPL(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_SET_HOST_METADATA {
    pub Anonymous: NVME_CDW11_FEATURE_SET_HOST_METADATA_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_SET_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_SET_HOST_METADATA_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_SET_HOST_METADATA_0 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 19) >> 19
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !8191) | (value & 8191);
    }
    pub fn EA(&self) -> u32 {
        (self._bitfield << 17) >> 30
    }
    pub fn set_EA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 13)) | ((value & 3) << 13);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 15
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(131071 << 15)) | ((value & 131071) << 15);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    pub Anonymous: NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0 {
    pub fn SAVE(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SAVE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn NSS(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NSS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn MOD(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_MOD(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    pub Anonymous: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0 {
    pub fn TMPTH(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_TMPTH(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn TMPSEL(&self) -> u32 {
        (self._bitfield << 12) >> 28
    }
    pub fn set_TMPSEL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 16)) | ((value & 15) << 16);
    }
    pub fn THSEL(&self) -> u32 {
        (self._bitfield << 10) >> 30
    }
    pub fn set_THSEL(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 20)) | ((value & 3) << 20);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 22
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1023 << 22)) | ((value & 1023) << 22);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    pub Anonymous: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0 {
    pub fn WCE(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_WCE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    pub Anonymous: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0 {
    pub fn DN(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_DN(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FIRMWARE_DOWNLOAD {
    pub OFST: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW11_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_GET_LOG_PAGE_0 {
    pub fn NUMDU(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NUMDU(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn LogSpecificIdentifier(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_LogSpecificIdentifier(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_IDENTIFY {
    pub Anonymous: NVME_CDW11_IDENTIFY_0,
    pub Anonymous2: NVME_CDW11_IDENTIFY_1,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_IDENTIFY_0 {
    pub NVMSETID: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_IDENTIFY_1 {
    pub _bitfield: u32,
}
impl NVME_CDW11_IDENTIFY_1 {
    pub fn CNSID(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_CNSID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved2(&self) -> u32 {
        (self._bitfield << 8) >> 24
    }
    pub fn set_Reserved2(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn CSI(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_CSI(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_RESERVATION_REPORT {
    pub Anonymous: NVME_CDW11_RESERVATION_REPORT_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_RESERVATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_RESERVATION_REPORT_0 {
    pub _bitfield: u32,
}
impl NVME_CDW11_RESERVATION_REPORT_0 {
    pub fn EDS(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EDS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW11_SANITIZE {
    pub Anonymous: NVME_CDW11_SANITIZE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW11_SANITIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_SANITIZE_0 {
    pub OVRPAT: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_SECURITY_RECEIVE {
    pub AL: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_SECURITY_SEND {
    pub TL: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE {
    pub AllocateResources: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub Anonymous: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub _bitfield: u32,
}
impl NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub fn NSR(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NSR(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND {
    pub EnableDirective: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    pub Anonymous: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0 {
    pub fn ENDIR(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ENDIR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 25
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn DTYPE(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_DTYPE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURES {
    pub HostMemoryBuffer: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HSIZE: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_GET_LOG_PAGE {
    pub LPOL: u32,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_READ_WRITE {
    pub Anonymous: NVME_CDW12_READ_WRITE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_READ_WRITE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW12_READ_WRITE_0 {
    pub fn NLB(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NLB(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 12) >> 28
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 16)) | ((value & 15) << 16);
    }
    pub fn DTYPE(&self) -> u32 {
        (self._bitfield << 8) >> 28
    }
    pub fn set_DTYPE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 20)) | ((value & 15) << 20);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 6) >> 30
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 24)) | ((value & 3) << 24);
    }
    pub fn PRINFO(&self) -> u32 {
        (self._bitfield << 2) >> 28
    }
    pub fn set_PRINFO(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 26)) | ((value & 15) << 26);
    }
    pub fn FUA(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_FUA(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u32) << 30);
    }
    pub fn LR(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_LR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_VERIFYCOMMAND {
    pub Anonymous: NVME_CDW12_VERIFYCOMMAND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_VERIFYCOMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_VERIFYCOMMAND_0 {
    pub _bitfield: u32,
}
impl NVME_CDW12_VERIFYCOMMAND_0 {
    pub fn NLB(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NLB(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 6) >> 22
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1023 << 16)) | ((value & 1023) << 16);
    }
    pub fn PRINFO(&self) -> u32 {
        (self._bitfield << 2) >> 28
    }
    pub fn set_PRINFO(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 26)) | ((value & 15) << 26);
    }
    pub fn FUA(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_FUA(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u32) << 30);
    }
    pub fn LR(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_LR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW12_ZONE_APPEND {
    pub Anonymous: NVME_CDW12_ZONE_APPEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW12_ZONE_APPEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_ZONE_APPEND_0 {
    pub _bitfield: u32,
}
impl NVME_CDW12_ZONE_APPEND_0 {
    pub fn NLB(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NLB(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 7) >> 23
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(511 << 16)) | ((value & 511) << 16);
    }
    pub fn PIREMAP(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_PIREMAP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u32) << 25);
    }
    pub fn PRINFO(&self) -> u32 {
        (self._bitfield << 2) >> 28
    }
    pub fn set_PRINFO(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 26)) | ((value & 15) << 26);
    }
    pub fn FUA(&self) -> bool {
        (self._bitfield >> 30) & 1 != 0
    }
    pub fn set_FUA(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value as u32) << 30);
    }
    pub fn LR(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_LR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURES {
    pub HostMemoryBuffer: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub _bitfield: u32,
}
impl NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn HMDLLA(&self) -> u32 {
        self._bitfield >> 4
    }
    pub fn set_HMDLLA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_GET_LOG_PAGE {
    pub LPOU: u32,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_READ_WRITE {
    pub Anonymous: NVME_CDW13_READ_WRITE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_READ_WRITE_0 {
    pub DSM: NVME_CDW13_READ_WRITE_0_0,
    pub Reserved: u8,
    pub DSPEC: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_READ_WRITE_0_0 {
    pub _bitfield: u8,
}
impl NVME_CDW13_READ_WRITE_0_0 {
    pub fn AccessFrequency(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_AccessFrequency(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn AccessLatency(&self) -> u8 {
        (self._bitfield << 2) >> 6
    }
    pub fn set_AccessLatency(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn SequentialRequest(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_SequentialRequest(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u8) << 6);
    }
    pub fn Incompressible(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Incompressible(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    pub Anonymous: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0 {
    pub fn ZRA(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_ZRA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn ZRASpecific(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_ZRASpecific(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Partial(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_Partial(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u32) << 16);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 17
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(32767 << 17)) | ((value & 32767) << 17);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW13_ZONE_MANAGEMENT_SEND {
    pub Anonymous: NVME_CDW13_ZONE_MANAGEMENT_SEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW13_ZONE_MANAGEMENT_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_SEND_0 {
    pub _bitfield: u32,
}
impl NVME_CDW13_ZONE_MANAGEMENT_SEND_0 {
    pub fn ZSA(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_ZSA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn SelectAll(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_SelectAll(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 9
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8388607 << 9)) | ((value & 8388607) << 9);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURES {
    pub HostMemoryBuffer: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HMDLUA: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW14_GET_LOG_PAGE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW14_GET_LOG_PAGE_0 {
    pub fn UUIDIndex(&self) -> u32 {
        (self._bitfield << 25) >> 25
    }
    pub fn set_UUIDIndex(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 9) >> 16
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 7)) | ((value & 65535) << 7);
    }
    pub fn OT(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_OT(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u32) << 23);
    }
    pub fn CommandSetIdentifier(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_CommandSetIdentifier(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_GET_LOG_PAGE_V20 {
    pub Anonymous: NVME_CDW14_GET_LOG_PAGE_V20_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_GET_LOG_PAGE_V20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_GET_LOG_PAGE_V20_0 {
    pub _bitfield: u32,
}
impl NVME_CDW14_GET_LOG_PAGE_V20_0 {
    pub fn UUIDIndex(&self) -> u32 {
        (self._bitfield << 25) >> 25
    }
    pub fn set_UUIDIndex(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 9) >> 16
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 7)) | ((value & 65535) << 7);
    }
    pub fn OffsetType(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_OffsetType(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u32) << 23);
    }
    pub fn CommandSetIdentifier(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_CommandSetIdentifier(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW14_IDENTIFY {
    pub Anonymous: NVME_CDW14_IDENTIFY_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW14_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_IDENTIFY_0 {
    pub _bitfield: u32,
}
impl NVME_CDW14_IDENTIFY_0 {
    pub fn UUIDIndex(&self) -> u32 {
        (self._bitfield << 25) >> 25
    }
    pub fn set_UUIDIndex(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURES {
    pub HostMemoryBuffer: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HMDLEC: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_READ_WRITE {
    pub Anonymous: NVME_CDW15_READ_WRITE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_READ_WRITE_0 {
    pub _bitfield: u32,
}
impl NVME_CDW15_READ_WRITE_0 {
    pub fn ELBAT(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_ELBAT(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn ELBATM(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_ELBATM(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_VERIFY_COMMAND {
    pub Anonymous: NVME_CDW15_VERIFY_COMMAND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_VERIFY_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_VERIFY_COMMAND_0 {
    pub _bitfield: u32,
}
impl NVME_CDW15_VERIFY_COMMAND_0 {
    pub fn ELBAT(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_ELBAT(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn ELBATM(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_ELBATM(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CDW15_ZONE_APPEND {
    pub Anonymous: NVME_CDW15_ZONE_APPEND_0,
    pub AsUlong: u32,
}
impl Default for NVME_CDW15_ZONE_APPEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_ZONE_APPEND_0 {
    pub _bitfield: u32,
}
impl NVME_CDW15_ZONE_APPEND_0 {
    pub fn LBAT(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_LBAT(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn LBATM(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_LBATM(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_CHANGED_NAMESPACE_LIST_LOG {
    pub NSID: [u32; 1024],
}
impl Default for NVME_CHANGED_NAMESPACE_LIST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_CHANGED_ZONE_LIST_LOG {
    pub ZoneIdentifiersCount: u16,
    pub Reserved: [u8; 6],
    pub ZoneIdentifier: [u64; 511],
}
impl Default for NVME_CHANGED_ZONE_LIST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_CMBSZ_SIZE_UNITS = i32;
pub const NVME_CMBSZ_SIZE_UNITS_16MB: NVME_CMBSZ_SIZE_UNITS = 3;
pub const NVME_CMBSZ_SIZE_UNITS_1MB: NVME_CMBSZ_SIZE_UNITS = 2;
pub const NVME_CMBSZ_SIZE_UNITS_256MB: NVME_CMBSZ_SIZE_UNITS = 4;
pub const NVME_CMBSZ_SIZE_UNITS_4GB: NVME_CMBSZ_SIZE_UNITS = 5;
pub const NVME_CMBSZ_SIZE_UNITS_4KB: NVME_CMBSZ_SIZE_UNITS = 0;
pub const NVME_CMBSZ_SIZE_UNITS_64GB: NVME_CMBSZ_SIZE_UNITS = 6;
pub const NVME_CMBSZ_SIZE_UNITS_64KB: NVME_CMBSZ_SIZE_UNITS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND {
    pub CDW0: NVME_COMMAND_DWORD0,
    pub NSID: u32,
    pub Reserved0: [u32; 2],
    pub MPTR: u64,
    pub Anonymous: NVME_COMMAND_0,
    pub u: NVME_COMMAND_1,
}
impl Default for NVME_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_0 {
    pub Anonymous: NVME_COMMAND_0_0,
    pub SGL1: [u64; 2],
}
impl Default for NVME_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_0_0 {
    pub PRP1: u64,
    pub PRP2: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1 {
    pub GENERAL: NVME_COMMAND_1_0,
    pub IDENTIFY: NVME_COMMAND_1_1,
    pub ABORT: NVME_COMMAND_1_2,
    pub GETFEATURES: NVME_COMMAND_1_3,
    pub SETFEATURES: NVME_COMMAND_1_4,
    pub GETLOGPAGE: NVME_COMMAND_1_5,
    pub CREATEIOCQ: NVME_COMMAND_1_6,
    pub CREATEIOSQ: NVME_COMMAND_1_7,
    pub DELETEIOQUEUE: NVME_COMMAND_1_8,
    pub DATASETMANAGEMENT: NVME_COMMAND_1_9,
    pub SECURITYSEND: NVME_COMMAND_1_10,
    pub SECURITYRECEIVE: NVME_COMMAND_1_11,
    pub FIRMWAREDOWNLOAD: NVME_COMMAND_1_12,
    pub FIRMWAREACTIVATE: NVME_COMMAND_1_13,
    pub FORMATNVM: NVME_COMMAND_1_14,
    pub DIRECTIVERECEIVE: NVME_COMMAND_1_15,
    pub DIRECTIVESEND: NVME_COMMAND_1_16,
    pub SANITIZE: NVME_COMMAND_1_17,
    pub READWRITE: NVME_COMMAND_1_18,
    pub RESERVATIONACQUIRE: NVME_COMMAND_1_19,
    pub RESERVATIONREGISTER: NVME_COMMAND_1_20,
    pub RESERVATIONRELEASE: NVME_COMMAND_1_21,
    pub RESERVATIONREPORT: NVME_COMMAND_1_22,
    pub ZONEMANAGEMENTSEND: NVME_COMMAND_1_23,
    pub ZONEMANAGEMENTRECEIVE: NVME_COMMAND_1_24,
    pub ZONEAPPEND: NVME_COMMAND_1_25,
    pub DEVICESELFTEST: NVME_COMMAND_1_26,
    pub DISCOVERYINFOMGMT: NVME_COMMAND_1_27,
    pub VENDORSPECIFIC: NVME_COMMAND_1_28,
    pub VERIFYCOMMAND: NVME_COMMAND_1_29,
}
impl Default for NVME_COMMAND_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_1_0 {
    pub CDW10: u32,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_1 {
    pub CDW10: NVME_CDW10_IDENTIFY,
    pub CDW11: NVME_CDW11_IDENTIFY,
    pub CDW12: u32,
    pub CDW13: u32,
    pub Anonymous: NVME_COMMAND_1_1_0,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1_1_0 {
    pub CDW14: u32,
    pub CDW14_V20: NVME_CDW14_IDENTIFY,
}
impl Default for NVME_COMMAND_1_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_10 {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_SEND,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_11 {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_RECEIVE,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_1_12 {
    pub CDW10: NVME_CDW10_FIRMWARE_DOWNLOAD,
    pub CDW11: NVME_CDW11_FIRMWARE_DOWNLOAD,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_13 {
    pub CDW10: NVME_CDW10_FIRMWARE_ACTIVATE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_14 {
    pub CDW10: NVME_CDW10_FORMAT_NVM,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_15 {
    pub CDW10: NVME_CDW10_DIRECTIVE_RECEIVE,
    pub CDW11: NVME_CDW11_DIRECTIVE_RECEIVE,
    pub CDW12: NVME_CDW12_DIRECTIVE_RECEIVE,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_16 {
    pub CDW10: NVME_CDW10_DIRECTIVE_SEND,
    pub CDW11: NVME_CDW11_DIRECTIVE_SEND,
    pub CDW12: NVME_CDW12_DIRECTIVE_SEND,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_17 {
    pub CDW10: NVME_CDW10_SANITIZE,
    pub CDW11: NVME_CDW11_SANITIZE,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_18 {
    pub LBALOW: u32,
    pub LBAHIGH: u32,
    pub CDW12: NVME_CDW12_READ_WRITE,
    pub CDW13: NVME_CDW13_READ_WRITE,
    pub CDW14: u32,
    pub CDW15: NVME_CDW15_READ_WRITE,
}
impl Default for NVME_COMMAND_1_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_19 {
    pub CDW10: NVME_CDW10_RESERVATION_ACQUIRE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_2 {
    pub CDW10: NVME_CDW10_ABORT,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_20 {
    pub CDW10: NVME_CDW10_RESERVATION_REGISTER,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_21 {
    pub CDW10: NVME_CDW10_RESERVATION_RELEASE,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_22 {
    pub CDW10: NVME_CDW10_RESERVATION_REPORT,
    pub CDW11: NVME_CDW11_RESERVATION_REPORT,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_23 {
    pub CDW1011: NVME_CDW10_ZONE_MANAGEMENT_SEND,
    pub CDW12: u32,
    pub CDW13: NVME_CDW13_ZONE_MANAGEMENT_SEND,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_24 {
    pub CDW1011: NVME_CDW10_ZONE_MANAGEMENT_RECEIVE,
    pub DWORDCOUNT: u32,
    pub CDW13: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_25 {
    pub CDW1011: NVME_CDW10_ZONE_APPEND,
    pub CDW12: NVME_CDW12_ZONE_APPEND,
    pub CDW13: u32,
    pub ILBRT: u32,
    pub CDW15: NVME_CDW15_ZONE_APPEND,
}
impl Default for NVME_COMMAND_1_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_26 {
    pub CDW10: NVME_CDW10_DEVICE_SELF_TEST,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_26 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_27 {
    pub CDW10: NVME_CDW10_DISCOVERY_INFO_MGMT,
    pub CDW11: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_27 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_1_28 {
    pub NDT: u32,
    pub NDM: u32,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_29 {
    pub LBALOW: u32,
    pub LBAHIGH: u32,
    pub CDW12: NVME_CDW12_VERIFYCOMMAND,
    pub CDW13: u32,
    pub EILBRT: u32,
    pub CDW15: NVME_CDW15_VERIFY_COMMAND,
}
impl Default for NVME_COMMAND_1_29 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_3 {
    pub CDW10: NVME_CDW10_GET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_4 {
    pub CDW10: NVME_CDW10_SET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: NVME_CDW12_FEATURES,
    pub CDW13: NVME_CDW13_FEATURES,
    pub CDW14: NVME_CDW14_FEATURES,
    pub CDW15: NVME_CDW15_FEATURES,
}
impl Default for NVME_COMMAND_1_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_5 {
    pub Anonymous: NVME_COMMAND_1_5_0,
    pub CDW11: NVME_CDW11_GET_LOG_PAGE,
    pub CDW12: NVME_CDW12_GET_LOG_PAGE,
    pub CDW13: NVME_CDW13_GET_LOG_PAGE,
    pub Anonymous2: NVME_COMMAND_1_5_1,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1_5_0 {
    pub CDW10: NVME_CDW10_GET_LOG_PAGE,
    pub CDW10_V121: NVME_CDW10_GET_LOG_PAGE_V121,
    pub CDW10_V13: NVME_CDW10_GET_LOG_PAGE_V13,
    pub CDW10_V20: NVME_CDW10_GET_LOG_PAGE_V20,
}
impl Default for NVME_COMMAND_1_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1_5_1 {
    pub CDW14: NVME_CDW14_GET_LOG_PAGE,
    pub CDW14_V20: NVME_CDW14_GET_LOG_PAGE_V20,
}
impl Default for NVME_COMMAND_1_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_6 {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_CQ,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_7 {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_SQ,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_8 {
    pub CDW10: NVME_CDW10_DELETE_IO_QUEUE,
}
impl Default for NVME_COMMAND_1_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_9 {
    pub CDW10: NVME_CDW10_DATASET_MANAGEMENT,
    pub CDW11: NVME_CDW11_DATASET_MANAGEMENT,
    pub CDW12: u32,
    pub CDW13: u32,
    pub CDW14: u32,
    pub CDW15: u32,
}
impl Default for NVME_COMMAND_1_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_DWORD0 {
    pub Anonymous: NVME_COMMAND_DWORD0_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMMAND_DWORD0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_DWORD0_0 {
    pub _bitfield: u32,
}
impl NVME_COMMAND_DWORD0_0 {
    pub fn OPC(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_OPC(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn FUSE(&self) -> u32 {
        (self._bitfield << 22) >> 30
    }
    pub fn set_FUSE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 8)) | ((value & 3) << 8);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 18) >> 28
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 10)) | ((value & 15) << 10);
    }
    pub fn PSDT(&self) -> u32 {
        (self._bitfield << 16) >> 30
    }
    pub fn set_PSDT(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
    pub fn CID(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_CID(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_EFFECTS_DATA {
    pub Anonymous: NVME_COMMAND_EFFECTS_DATA_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMMAND_EFFECTS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_EFFECTS_DATA_0 {
    pub _bitfield: u32,
}
impl NVME_COMMAND_EFFECTS_DATA_0 {
    pub fn CSUPP(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CSUPP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn LBCC(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_LBCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn NCC(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_NCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn NIC(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_NIC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn CCC(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_CCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 16) >> 21
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
    pub fn CSE(&self) -> u32 {
        (self._bitfield << 13) >> 29
    }
    pub fn set_CSE(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 16)) | ((value & 7) << 16);
    }
    pub fn UUIDSelectionSupported(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_UUIDSelectionSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u32) << 19);
    }
    pub fn CSPNamespace(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_CSPNamespace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u32) << 20);
    }
    pub fn CSPController(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_CSPController(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u32) << 21);
    }
    pub fn CSPNVMSet(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_CSPNVMSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u32) << 22);
    }
    pub fn CSPEnduranceGroup(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_CSPEnduranceGroup(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u32) << 23);
    }
    pub fn CSPDomain(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_CSPDomain(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u32) << 24);
    }
    pub fn CSPNVMSubsystem(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_CSPNVMSubsystem(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u32) << 25);
    }
    pub fn CSPReserved(&self) -> u32 {
        self._bitfield >> 26
    }
    pub fn set_CSPReserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(63 << 26)) | ((value & 63) << 26);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_EFFECTS_LOG {
    pub ACS: [NVME_COMMAND_EFFECTS_DATA; 256],
    pub IOCS: [NVME_COMMAND_EFFECTS_DATA; 256],
    pub Reserved: [u8; 2048],
}
impl Default for NVME_COMMAND_EFFECTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = i32;
pub const NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_NONE: NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = 0;
pub const NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_SINGLE_PER_CONTROLLER: NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = 2;
pub const NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_SINGLE_PER_NAMESPACE: NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS = 1;
pub type NVME_COMMAND_SET_IDENTIFIERS = i32;
pub const NVME_COMMAND_SET_KEY_VALUE: NVME_COMMAND_SET_IDENTIFIERS = 1;
pub const NVME_COMMAND_SET_NVM: NVME_COMMAND_SET_IDENTIFIERS = 0;
pub const NVME_COMMAND_SET_ZONED_NAMESPACE: NVME_COMMAND_SET_IDENTIFIERS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_STATUS {
    pub Anonymous: NVME_COMMAND_STATUS_0,
    pub AsUshort: u16,
}
impl Default for NVME_COMMAND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_STATUS_0 {
    pub _bitfield: u16,
}
impl NVME_COMMAND_STATUS_0 {
    pub fn P(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_P(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn SC(&self) -> u16 {
        (self._bitfield << 7) >> 8
    }
    pub fn set_SC(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(255 << 1)) | ((value & 255) << 1);
    }
    pub fn SCT(&self) -> u16 {
        (self._bitfield << 4) >> 13
    }
    pub fn set_SCT(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 9)) | ((value & 7) << 9);
    }
    pub fn CRD(&self) -> u16 {
        (self._bitfield << 2) >> 14
    }
    pub fn set_CRD(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(3 << 12)) | ((value & 3) << 12);
    }
    pub fn M(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_M(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn DNR(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_DNR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    pub _bitfield: u32,
}
impl NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    pub fn AsyncEventType(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_AsyncEventType(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 27
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
    pub fn AsyncEventInfo(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_AsyncEventInfo(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn LogPage(&self) -> u32 {
        (self._bitfield << 8) >> 24
    }
    pub fn set_LogPage(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub Anonymous: NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0,
    pub AsUlong: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub _bitfield: u32,
}
impl NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub fn NSA(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_NSA(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY {
    pub DW0: u32,
    pub DW1: u32,
    pub DW2: NVME_COMPLETION_ENTRY_0,
    pub DW3: NVME_COMPLETION_ENTRY_1,
}
impl Default for NVME_COMPLETION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_ENTRY_0 {
    pub Anonymous: NVME_COMPLETION_ENTRY_0_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMPLETION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_ENTRY_0_0 {
    pub SQHD: u16,
    pub SQID: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_ENTRY_1 {
    pub Anonymous: NVME_COMPLETION_ENTRY_1_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMPLETION_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY_1_0 {
    pub CID: u16,
    pub Status: NVME_COMMAND_STATUS,
}
impl Default for NVME_COMPLETION_ENTRY_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    pub Anonymous: NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0,
    pub AsUlong: u32,
}
impl Default for NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0 {
    pub _bitfield: u32,
}
impl NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0 {
    pub fn CQH(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_CQH(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTEXT_ATTRIBUTES {
    pub Anonymous: NVME_CONTEXT_ATTRIBUTES_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTEXT_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
impl NVME_CONTEXT_ATTRIBUTES_0 {
    pub fn AccessFrequency(&self) -> u32 {
        (self._bitfield << 28) >> 28
    }
    pub fn set_AccessFrequency(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn AccessLatency(&self) -> u32 {
        (self._bitfield << 26) >> 30
    }
    pub fn set_AccessLatency(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 30
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn SequentialReadRange(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_SequentialReadRange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn SequentialWriteRange(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_SequentialWriteRange(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn WritePrepare(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_WritePrepare(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u32) << 10);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 8) >> 19
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8191 << 11)) | ((value & 8191) << 11);
    }
    pub fn CommandAccessSize(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_CommandAccessSize(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_CAPABILITIES {
    pub Anonymous: NVME_CONTROLLER_CAPABILITIES_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_CONTROLLER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_CAPABILITIES_0 {
    pub _bitfield: u64,
}
impl NVME_CONTROLLER_CAPABILITIES_0 {
    pub fn MQES(&self) -> u64 {
        (self._bitfield << 48) >> 48
    }
    pub fn set_MQES(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn CQR(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_CQR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u64) << 16);
    }
    pub fn AMS_WeightedRoundRobinWithUrgent(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_AMS_WeightedRoundRobinWithUrgent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u64) << 17);
    }
    pub fn AMS_VendorSpecific(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_AMS_VendorSpecific(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u64) << 18);
    }
    pub fn Reserved0(&self) -> u64 {
        (self._bitfield << 40) >> 59
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(31 << 19)) | ((value & 31) << 19);
    }
    pub fn TO(&self) -> u64 {
        (self._bitfield << 32) >> 56
    }
    pub fn set_TO(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
    pub fn DSTRD(&self) -> u64 {
        (self._bitfield << 28) >> 60
    }
    pub fn set_DSTRD(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 32)) | ((value & 15) << 32);
    }
    pub fn NSSRS(&self) -> bool {
        (self._bitfield >> 36) & 1 != 0
    }
    pub fn set_NSSRS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 36)) | ((value as u64) << 36);
    }
    pub fn CSS_NVM(&self) -> bool {
        (self._bitfield >> 37) & 1 != 0
    }
    pub fn set_CSS_NVM(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 37)) | ((value as u64) << 37);
    }
    pub fn CSS_Reserved0(&self) -> bool {
        (self._bitfield >> 38) & 1 != 0
    }
    pub fn set_CSS_Reserved0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 38)) | ((value as u64) << 38);
    }
    pub fn CSS_Reserved1(&self) -> bool {
        (self._bitfield >> 39) & 1 != 0
    }
    pub fn set_CSS_Reserved1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 39)) | ((value as u64) << 39);
    }
    pub fn CSS_Reserved2(&self) -> bool {
        (self._bitfield >> 40) & 1 != 0
    }
    pub fn set_CSS_Reserved2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 40)) | ((value as u64) << 40);
    }
    pub fn CSS_Reserved3(&self) -> bool {
        (self._bitfield >> 41) & 1 != 0
    }
    pub fn set_CSS_Reserved3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 41)) | ((value as u64) << 41);
    }
    pub fn CSS_Reserved4(&self) -> bool {
        (self._bitfield >> 42) & 1 != 0
    }
    pub fn set_CSS_Reserved4(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 42)) | ((value as u64) << 42);
    }
    pub fn CSS_MultipleIo(&self) -> bool {
        (self._bitfield >> 43) & 1 != 0
    }
    pub fn set_CSS_MultipleIo(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 43)) | ((value as u64) << 43);
    }
    pub fn CSS_AdminOnly(&self) -> bool {
        (self._bitfield >> 44) & 1 != 0
    }
    pub fn set_CSS_AdminOnly(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 44)) | ((value as u64) << 44);
    }
    pub fn BPS(&self) -> bool {
        (self._bitfield >> 45) & 1 != 0
    }
    pub fn set_BPS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 45)) | ((value as u64) << 45);
    }
    pub fn CPS(&self) -> u64 {
        (self._bitfield << 16) >> 62
    }
    pub fn set_CPS(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(3 << 46)) | ((value & 3) << 46);
    }
    pub fn MPSMIN(&self) -> u64 {
        (self._bitfield << 12) >> 60
    }
    pub fn set_MPSMIN(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 48)) | ((value & 15) << 48);
    }
    pub fn MPSMAX(&self) -> u64 {
        (self._bitfield << 8) >> 60
    }
    pub fn set_MPSMAX(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(15 << 52)) | ((value & 15) << 52);
    }
    pub fn PMRS(&self) -> bool {
        (self._bitfield >> 56) & 1 != 0
    }
    pub fn set_PMRS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 56)) | ((value as u64) << 56);
    }
    pub fn CMBS(&self) -> bool {
        (self._bitfield >> 57) & 1 != 0
    }
    pub fn set_CMBS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 57)) | ((value as u64) << 57);
    }
    pub fn NSSS(&self) -> bool {
        (self._bitfield >> 58) & 1 != 0
    }
    pub fn set_NSSS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 58)) | ((value as u64) << 58);
    }
    pub fn CRWMS(&self) -> bool {
        (self._bitfield >> 59) & 1 != 0
    }
    pub fn set_CRWMS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 59)) | ((value as u64) << 59);
    }
    pub fn CRIMS(&self) -> bool {
        (self._bitfield >> 60) & 1 != 0
    }
    pub fn set_CRIMS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 60)) | ((value as u64) << 60);
    }
    pub fn Reserved2(&self) -> u64 {
        self._bitfield >> 61
    }
    pub fn set_Reserved2(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(7 << 61)) | ((value & 7) << 61);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_CONFIGURATION {
    pub Anonymous: NVME_CONTROLLER_CONFIGURATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_CONFIGURATION_0 {
    pub _bitfield: u32,
}
impl NVME_CONTROLLER_CONFIGURATION_0 {
    pub fn EN(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EN(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 28) >> 29
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn CSS(&self) -> u32 {
        (self._bitfield << 25) >> 29
    }
    pub fn set_CSS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 4)) | ((value & 7) << 4);
    }
    pub fn MPS(&self) -> u32 {
        (self._bitfield << 21) >> 28
    }
    pub fn set_MPS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 7)) | ((value & 15) << 7);
    }
    pub fn AMS(&self) -> u32 {
        (self._bitfield << 18) >> 29
    }
    pub fn set_AMS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 11)) | ((value & 7) << 11);
    }
    pub fn SHN(&self) -> u32 {
        (self._bitfield << 16) >> 30
    }
    pub fn set_SHN(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
    pub fn IOSQES(&self) -> u32 {
        (self._bitfield << 12) >> 28
    }
    pub fn set_IOSQES(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 16)) | ((value & 15) << 16);
    }
    pub fn IOCQES(&self) -> u32 {
        (self._bitfield << 8) >> 28
    }
    pub fn set_IOCQES(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 20)) | ((value & 15) << 20);
    }
    pub fn CRIME(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_CRIME(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u32) << 24);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 25
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(127 << 25)) | ((value & 127) << 25);
    }
}
pub const NVME_CONTROLLER_ID_DYN: u32 = 65535;
pub const NVME_CONTROLLER_ID_MAX: u32 = 65519;
pub const NVME_CONTROLLER_ID_MIN: u32 = 0;
pub const NVME_CONTROLLER_ID_STAT_PERSIST: u32 = 65534;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_CONTROLLER_LIST {
    pub NumberOfIdentifiers: u16,
    pub ControllerID: [u16; 2047],
}
impl Default for NVME_CONTROLLER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    pub Anonymous: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0 {
    pub _bitfield: u32,
}
impl NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0 {
    pub fn BIR(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_BIR(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 20) >> 23
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(511 << 3)) | ((value & 511) << 3);
    }
    pub fn OFST(&self) -> u32 {
        self._bitfield >> 12
    }
    pub fn set_OFST(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1048575 << 12)) | ((value & 1048575) << 12);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    pub Anonymous: NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0 {
    pub _bitfield: u32,
}
impl NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0 {
    pub fn SQS(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SQS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn CQS(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CQS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn LISTS(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_LISTS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn RDS(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_RDS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn WDS(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_WDS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 24) >> 29
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn SZU(&self) -> u32 {
        (self._bitfield << 20) >> 28
    }
    pub fn set_SZU(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn SZ(&self) -> u32 {
        self._bitfield >> 12
    }
    pub fn set_SZ(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1048575 << 12)) | ((value & 1048575) << 12);
    }
}
pub const NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 8;
pub const NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 9;
pub const NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 14;
pub const NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 15;
pub type NVME_CONTROLLER_METADATA_ELEMENT_TYPES = i32;
pub const NVME_CONTROLLER_METADATA_FIRMWARE_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 12;
pub const NVME_CONTROLLER_METADATA_HOST_DETERMINED_FAILURE_RECORD: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 16;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_CONTROLLER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 1;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_FILENAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 13;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 2;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 3;
pub const NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_NAME_AND_BUILD: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 10;
pub const NVME_CONTROLLER_METADATA_PREBOOT_CONTROLLER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 4;
pub const NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 5;
pub const NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_VERSION: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 6;
pub const NVME_CONTROLLER_METADATA_SYSTEM_PROCESSOR_MODEL: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 7;
pub const NVME_CONTROLLER_METADATA_SYSTEM_PRODUCT_NAME: NVME_CONTROLLER_METADATA_ELEMENT_TYPES = 11;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_READY_TIMEOUTS {
    pub Anonymous: NVME_CONTROLLER_READY_TIMEOUTS_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_READY_TIMEOUTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_READY_TIMEOUTS_0 {
    pub CRWMT: u16,
    pub CRIMT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_CONTROLLER_REGISTERS {
    pub CAP: NVME_CONTROLLER_CAPABILITIES,
    pub VS: NVME_VERSION,
    pub INTMS: u32,
    pub INTMC: u32,
    pub CC: NVME_CONTROLLER_CONFIGURATION,
    pub Reserved0: u32,
    pub CSTS: NVME_CONTROLLER_STATUS,
    pub NSSR: NVME_NVM_SUBSYSTEM_RESET,
    pub AQA: NVME_ADMIN_QUEUE_ATTRIBUTES,
    pub ASQ: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS,
    pub ACQ: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS,
    pub CMBLOC: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION,
    pub CMBSZ: NVME_CONTROLLER_MEMORY_BUFFER_SIZE,
    pub BPINFO: NVME_BOOT_PARTITION_INFORMATION,
    pub Reserved1: [u32; 8],
    pub NSSD: NVME_NVM_SUBSYSTEM_SHUTDOWN,
    pub CRTO: NVME_CONTROLLER_READY_TIMEOUTS,
    pub Reserved2: [u32; 933],
    pub Reserved3: [u32; 64],
    pub Doorbells: [u32; 0],
}
impl Default for NVME_CONTROLLER_REGISTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_STATUS {
    pub Anonymous: NVME_CONTROLLER_STATUS_0,
    pub AsUlong: u32,
}
impl Default for NVME_CONTROLLER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_STATUS_0 {
    pub _bitfield: u32,
}
impl NVME_CONTROLLER_STATUS_0 {
    pub fn RDY(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_RDY(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn CFS(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CFS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn SHST(&self) -> u32 {
        (self._bitfield << 28) >> 30
    }
    pub fn set_SHST(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 2)) | ((value & 3) << 2);
    }
    pub fn NSSRO(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_NSSRO(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn PP(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_PP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u32) << 5);
    }
    pub fn ST(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_ST(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u32) << 6);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 7
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
pub type NVME_CONTROLLER_TYPE = i32;
pub const NVME_CPS_CONTROLLER_SCOPE: NVME_CPS_VALUE = 1;
pub const NVME_CPS_DOMAIN_SCOPE: NVME_CPS_VALUE = 2;
pub const NVME_CPS_NOT_REPORTED: NVME_CPS_VALUE = 0;
pub const NVME_CPS_SUBSYSTEM_SCOPE: NVME_CPS_VALUE = 3;
pub type NVME_CPS_VALUE = i32;
pub const NVME_CSS_ADMIN_COMMAND_SET_ONLY: NVME_CSS_COMMAND_SETS = 7;
pub const NVME_CSS_ALL_SUPPORTED_IO_COMMAND_SET: NVME_CSS_COMMAND_SETS = 6;
pub type NVME_CSS_COMMAND_SETS = i32;
pub const NVME_CSS_NVM_COMMAND_SET: NVME_CSS_COMMAND_SETS = 0;
pub const NVME_CSTS_SHST_NO_SHUTDOWN: NVME_CSTS_SHST_SHUTDOWN_STATUS = 0;
pub const NVME_CSTS_SHST_SHUTDOWN_COMPLETED: NVME_CSTS_SHST_SHUTDOWN_STATUS = 2;
pub const NVME_CSTS_SHST_SHUTDOWN_IN_PROCESS: NVME_CSTS_SHST_SHUTDOWN_STATUS = 1;
pub type NVME_CSTS_SHST_SHUTDOWN_STATUS = i32;
pub type NVME_DEALLOCATE_READ_BEHAVIOR = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DEVICE_SELF_TEST_LOG {
    pub CurrentOperation: NVME_DEVICE_SELF_TEST_LOG_0,
    pub CurrentCompletion: NVME_DEVICE_SELF_TEST_LOG_1,
    pub Reserved: [u8; 2],
    pub ResultData: [NVME_DEVICE_SELF_TEST_RESULT_DATA; 20],
}
impl Default for NVME_DEVICE_SELF_TEST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DEVICE_SELF_TEST_LOG_0 {
    pub _bitfield: u8,
}
impl NVME_DEVICE_SELF_TEST_LOG_0 {
    pub fn Status(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_Status(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DEVICE_SELF_TEST_LOG_1 {
    pub _bitfield: u8,
}
impl NVME_DEVICE_SELF_TEST_LOG_1 {
    pub fn CompletePercent(&self) -> u8 {
        (self._bitfield << 1) >> 1
    }
    pub fn set_CompletePercent(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA {
    pub Status: NVME_DEVICE_SELF_TEST_RESULT_DATA_0,
    pub SegmentNumber: u8,
    pub ValidDiagnostics: NVME_DEVICE_SELF_TEST_RESULT_DATA_1,
    pub Reserved: u8,
    pub POH: u64,
    pub NSID: u32,
    pub FailingLBA: u64,
    pub StatusCodeType: NVME_DEVICE_SELF_TEST_RESULT_DATA_2,
    pub StatusCode: u8,
    pub VendorSpecific: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_0 {
    pub _bitfield: u8,
}
impl NVME_DEVICE_SELF_TEST_RESULT_DATA_0 {
    pub fn Result(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_Result(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn CodeValue(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_CodeValue(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_1 {
    pub _bitfield: u8,
}
impl NVME_DEVICE_SELF_TEST_RESULT_DATA_1 {
    pub fn NSIDValid(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_NSIDValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn FLBAValid(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_FLBAValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn SCTValid(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SCTValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn SCValid(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_SCValid(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_2 {
    pub _bitfield: u8,
}
impl NVME_DEVICE_SELF_TEST_RESULT_DATA_2 {
    pub fn AdditionalInfo(&self) -> u8 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_AdditionalInfo(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS {
    pub DirectivesSupported: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    pub DirectivesEnabled: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    pub _bitfield: u8,
    pub Reserved1: [u8; 31],
}
impl NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    pub fn Identify(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Identify(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Streams(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Streams(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Reserved0(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved0(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
impl Default for NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS = i32;
pub const NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATION_RETURN_PARAMETERS: NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS = 1;
pub type NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = i32;
pub const NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_ALLOCATE_RESOURCES: NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = 3;
pub const NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_GET_STATUS: NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = 2;
pub const NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_RETURN_PARAMETERS: NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS = 1;
pub type NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS = i32;
pub const NVME_DIRECTIVE_SEND_IDENTIFY_OPERATION_ENABLE_DIRECTIVE: NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS = 1;
pub type NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS = i32;
pub const NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_IDENTIFIER: NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS = 1;
pub const NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_RESOURCES: NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    pub OpenStreamCount: u16,
    pub StreamIdentifiers: [u16; 65535],
}
impl Default for NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    pub MSL: u16,
    pub NSSA: u16,
    pub NSSO: u16,
    pub Reserved0: [u8; 10],
    pub SWS: u32,
    pub SGS: u16,
    pub NSA: u16,
    pub NSO: u16,
    pub Reserved1: [u8; 6],
}
impl Default for NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_DIRECTIVE_TYPES = i32;
pub const NVME_DIRECTIVE_TYPE_IDENTIFY: NVME_DIRECTIVE_TYPES = 0;
pub const NVME_DIRECTIVE_TYPE_STREAMS: NVME_DIRECTIVE_TYPES = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DISCOVERY_ENTRY {
    pub TRTYPE: u8,
    pub ADRFAM: u8,
    pub SUBTYPE: u8,
    pub TREQ: NVME_DISCOVERY_ENTRY_0,
    pub PORTID: u16,
    pub CNTLID: u16,
    pub ASQSZ: u16,
    pub EFLAGS: NVME_DISCOVERY_ENTRY_1,
    pub Reserved0: [u8; 20],
    pub TRSVCID: [u8; 32],
    pub Reserved1: [u8; 192],
    pub NQN: [u8; 256],
    pub TRADDR: [u8; 256],
    pub TSAS: [u8; 256],
}
impl Default for NVME_DISCOVERY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_ENTRY_0 {
    pub Anonymous: NVME_DISCOVERY_ENTRY_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_DISCOVERY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_ENTRY_0_0 {
    pub _bitfield: u8,
}
impl NVME_DISCOVERY_ENTRY_0_0 {
    pub fn SecureChannel(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_SecureChannel(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn SqFlowControlDisable(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SqFlowControlDisable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn ZeroHostIdSupport(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ZeroHostIdSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn AuthAndSecureChannel(&self) -> u8 {
        (self._bitfield << 2) >> 6
    }
    pub fn set_AuthAndSecureChannel(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_ENTRY_1 {
    pub Anonymous: NVME_DISCOVERY_ENTRY_1_0,
    pub AsUshort: u16,
}
impl Default for NVME_DISCOVERY_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_ENTRY_1_0 {
    pub _bitfield: u16,
}
impl NVME_DISCOVERY_ENTRY_1_0 {
    pub fn DuplicateReturnedInfo(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_DuplicateReturnedInfo(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn ExplicitPersistentConnectionSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ExplicitPersistentConnectionSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn NoCDCConnectivity(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_NoCDCConnectivity(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DISCOVERY_HEADER {
    pub GENCTR: u64,
    pub NUMREC: u64,
    pub RECFMT: u16,
    pub DLPF: NVME_DISCOVERY_HEADER_0,
    pub Reserved0: u8,
    pub TDLPL: u32,
    pub Reserved1: [u8; 1000],
}
impl Default for NVME_DISCOVERY_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_HEADER_0 {
    pub Anonymous: NVME_DISCOVERY_HEADER_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_DISCOVERY_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_HEADER_0_0 {
    pub _bitfield: u8,
}
impl NVME_DISCOVERY_HEADER_0_0 {
    pub fn Extended(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Extended(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn PortLocal(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_PortLocal(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn AllSubsystems(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_AllSubsystems(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
pub type NVME_DISCOVERY_INFO_ENTITY_TYPES = i32;
pub const NVME_DISCOVERY_INFO_ENTITY_TYPE_CDC: NVME_DISCOVERY_INFO_ENTITY_TYPES = 3;
pub const NVME_DISCOVERY_INFO_ENTITY_TYPE_DDC: NVME_DISCOVERY_INFO_ENTITY_TYPES = 2;
pub const NVME_DISCOVERY_INFO_ENTITY_TYPE_HOST: NVME_DISCOVERY_INFO_ENTITY_TYPES = 1;
pub const NVME_DISCOVERY_INFO_ENTITY_TYPE_RESERVED: NVME_DISCOVERY_INFO_ENTITY_TYPES = 0;
pub type NVME_DISCOVERY_INFO_ENTRY_FORMATS = i32;
pub const NVME_DISCOVERY_INFO_ENTRY_FORMAT_BASIC: NVME_DISCOVERY_INFO_ENTRY_FORMATS = 1;
pub const NVME_DISCOVERY_INFO_ENTRY_FORMAT_EXTENDED: NVME_DISCOVERY_INFO_ENTRY_FORMATS = 2;
pub const NVME_DISCOVERY_INFO_ENTRY_FORMAT_RESERVED: NVME_DISCOVERY_INFO_ENTRY_FORMATS = 0;
pub const NVME_DISCOVERY_INFO_MGMT_EKTYPE_PORTID: u32 = 63;
pub const NVME_DISCOVERY_INFO_MGMT_EKTYPE_TRADDR: u32 = 95;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_DISCOVERY_INFO_MGMT_HEADER {
    pub TDL: u32,
    pub Reserved0: u32,
    pub NUMENT: u64,
    pub ENTFMT: u16,
    pub ETYPE: u16,
    pub PORTLCL: u8,
    pub Reserved1: u8,
    pub EKTYPE: NVME_DISCOVERY_INFO_MGMT_HEADER_0,
    pub EID: [u8; 256],
    pub ENAME: [u8; 256],
    pub EVER: [u8; 64],
    pub Reserved2: [u8; 424],
}
impl Default for NVME_DISCOVERY_INFO_MGMT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_INFO_MGMT_HEADER_0 {
    pub Anonymous: NVME_DISCOVERY_INFO_MGMT_HEADER_0_0,
    pub AsUshort: u16,
}
impl Default for NVME_DISCOVERY_INFO_MGMT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_INFO_MGMT_HEADER_0_0 {
    pub _bitfield: u16,
}
impl NVME_DISCOVERY_INFO_MGMT_HEADER_0_0 {
    pub fn NQN(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_NQN(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn TSAS(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_TSAS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn TRSVCID(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_TRSVCID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn ADRFAM(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ADRFAM(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn TRTYPE(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_TRTYPE(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn PORTID(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_PORTID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn TRADDR(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_TRADDR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(511 << 7)) | ((value & 511) << 7);
    }
}
pub type NVME_DISCOVERY_INFO_MGMT_TASK = i32;
pub const NVME_DISCOVERY_INFO_MGMT_TASK_DEREGISTER: NVME_DISCOVERY_INFO_MGMT_TASK = 1;
pub const NVME_DISCOVERY_INFO_MGMT_TASK_REGISTER: NVME_DISCOVERY_INFO_MGMT_TASK = 0;
pub const NVME_DISCOVERY_INFO_MGMT_TASK_UPDATE: NVME_DISCOVERY_INFO_MGMT_TASK = 2;
pub type NVME_DISC_CTRL_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_ENDURANCE_GROUP_LOG {
    pub Reserved0: u32,
    pub AvailableSpareThreshold: u8,
    pub PercentageUsed: u8,
    pub Reserved1: [u8; 26],
    pub EnduranceEstimate: [u8; 16],
    pub DataUnitsRead: [u8; 16],
    pub DataUnitsWritten: [u8; 16],
    pub MediaUnitsWritten: [u8; 16],
    pub Reserved2: [u8; 416],
}
impl Default for NVME_ENDURANCE_GROUP_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ERROR_INFO_LOG {
    pub ErrorCount: u64,
    pub SQID: u16,
    pub CMDID: u16,
    pub Status: NVME_COMMAND_STATUS,
    pub ParameterErrorLocation: NVME_ERROR_INFO_LOG_0,
    pub Lba: u64,
    pub NameSpace: u32,
    pub VendorInfoAvailable: u8,
    pub TRTYPE: u8,
    pub Reserved0: [u8; 2],
    pub CommandSpecificInfo: u64,
    pub TransportTypeSpecificInfo: u16,
    pub Reserved1: [u8; 22],
}
impl Default for NVME_ERROR_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ERROR_INFO_LOG_0 {
    pub _bitfield: u16,
}
impl NVME_ERROR_INFO_LOG_0 {
    pub fn Byte(&self) -> u16 {
        (self._bitfield << 8) >> 8
    }
    pub fn set_Byte(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Bit(&self) -> u16 {
        (self._bitfield << 5) >> 13
    }
    pub fn set_Bit(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(7 << 8)) | ((value & 7) << 8);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 11
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(31 << 11)) | ((value & 31) << 11);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_ERROR_INJECTION_ENTRY {
    pub Flags: NVME_ERROR_INJECTION_ENTRY_0,
    pub Reserved1: u8,
    pub ErrorInjectionType: u16,
    pub ErrorInjectionTypeSpecific: [u8; 28],
}
impl Default for NVME_ERROR_INJECTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_ERROR_INJECTION_ENTRY_0 {
    pub Anonymous: NVME_ERROR_INJECTION_ENTRY_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_ERROR_INJECTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ERROR_INJECTION_ENTRY_0_0 {
    pub _bitfield: u8,
}
impl NVME_ERROR_INJECTION_ENTRY_0_0 {
    pub fn Enable(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Enable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn SingleInstance(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_SingleInstance(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Reserved0(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved0(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
pub type NVME_ERROR_INJECTION_TYPES = i32;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_CPU_CONTROLLER_HANG: NVME_ERROR_INJECTION_TYPES = 1;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_CRITICAL: NVME_ERROR_INJECTION_TYPES = 5;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_NONCRITICAL: NVME_ERROR_INJECTION_TYPES = 6;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_HW_MALFUNCTION: NVME_ERROR_INJECTION_TYPES = 9;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_LOGICAL_FW_ERROR: NVME_ERROR_INJECTION_TYPES = 4;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_CORRUPTION: NVME_ERROR_INJECTION_TYPES = 7;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_HANG: NVME_ERROR_INJECTION_TYPES = 2;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_PLP_DEFECT: NVME_ERROR_INJECTION_TYPES = 3;
pub const NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_SRAM_CORRUPTION: NVME_ERROR_INJECTION_TYPES = 8;
pub const NVME_ERROR_INJECTION_TYPE_MAX: NVME_ERROR_INJECTION_TYPES = 65535;
pub const NVME_ERROR_INJECTION_TYPE_RESERVED0: NVME_ERROR_INJECTION_TYPES = 0;
pub const NVME_ERROR_INJECTION_TYPE_RESERVED1: NVME_ERROR_INJECTION_TYPES = 10;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_EXTENDED_ATTR {
    pub EXATTYPE: u16,
    pub EXATLEN: u16,
    pub EXATVAL: [u8; 0],
}
impl Default for NVME_EXTENDED_ATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_EXTENDED_ATTR_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_EXTENDED_DISCOVERY_ENTRY {
    pub TRTYPE: u8,
    pub ADRFAM: u8,
    pub SUBTYPE: u8,
    pub TREQ: NVME_EXTENDED_DISCOVERY_ENTRY_0,
    pub PORTID: u16,
    pub CNTLID: u16,
    pub ASQSZ: u16,
    pub EFLAGS: NVME_EXTENDED_DISCOVERY_ENTRY_1,
    pub Reserved0: [u8; 20],
    pub TRSVCID: [u8; 32],
    pub Reserved1: [u8; 192],
    pub NQN: [u8; 256],
    pub TRADDR: [u8; 256],
    pub TSAS: [u8; 256],
    pub TEL: u32,
    pub NUMEXAT: u16,
    pub Reserved2: u16,
}
impl Default for NVME_EXTENDED_DISCOVERY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_EXTENDED_DISCOVERY_ENTRY_0 {
    pub Anonymous: NVME_EXTENDED_DISCOVERY_ENTRY_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_EXTENDED_DISCOVERY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_EXTENDED_DISCOVERY_ENTRY_0_0 {
    pub _bitfield: u8,
}
impl NVME_EXTENDED_DISCOVERY_ENTRY_0_0 {
    pub fn SecureChannel(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_SecureChannel(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn SqFlowControlDisable(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SqFlowControlDisable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn ZeroHostIdSupport(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ZeroHostIdSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn AuthAndSecureChannel(&self) -> u8 {
        (self._bitfield << 2) >> 6
    }
    pub fn set_AuthAndSecureChannel(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_EXTENDED_DISCOVERY_ENTRY_1 {
    pub Anonymous: NVME_EXTENDED_DISCOVERY_ENTRY_1_0,
    pub AsUshort: u16,
}
impl Default for NVME_EXTENDED_DISCOVERY_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_EXTENDED_DISCOVERY_ENTRY_1_0 {
    pub _bitfield: u16,
}
impl NVME_EXTENDED_DISCOVERY_ENTRY_1_0 {
    pub fn DuplicateReturnedInfo(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_DuplicateReturnedInfo(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn ExplicitPersistentConnectionSupport(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ExplicitPersistentConnectionSupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn NoCDCConnectivity(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_NoCDCConnectivity(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
pub const NVME_EXTENDED_HOST_IDENTIFIER_SIZE: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_EXTENDED_REPORT_ZONE_INFO {
    pub ZoneCount: u64,
    pub Reserved: [u64; 7],
    pub Desc: [NVME_ZONE_EXTENDED_REPORT_ZONE_DESC; 1],
}
impl Default for NVME_EXTENDED_REPORT_ZONE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_FABRICS_COMMAND_AUTH_RECV: NVME_FABRICS_COMMAND_TYPE = 6;
pub const NVME_FABRICS_COMMAND_AUTH_SEND: NVME_FABRICS_COMMAND_TYPE = 5;
pub const NVME_FABRICS_COMMAND_CONNECT: NVME_FABRICS_COMMAND_TYPE = 1;
pub const NVME_FABRICS_COMMAND_DISCONNECT: NVME_FABRICS_COMMAND_TYPE = 8;
pub const NVME_FABRICS_COMMAND_PROPERTY_GET: NVME_FABRICS_COMMAND_TYPE = 4;
pub const NVME_FABRICS_COMMAND_PROPERTY_SET: NVME_FABRICS_COMMAND_TYPE = 0;
pub type NVME_FABRICS_COMMAND_TYPE = i32;
pub type NVME_FEATURES = i32;
pub const NVME_FEATURE_ARBITRATION: NVME_FEATURES = 1;
pub const NVME_FEATURE_ASYNC_EVENT_CONFIG: NVME_FEATURES = 11;
pub const NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION: NVME_FEATURES = 12;
pub const NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY: NVME_FEATURES = 193;
pub const NVME_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS: NVME_FEATURES = 195;
pub const NVME_FEATURE_CONTROLLER_METADATA: NVME_FEATURES = 126;
pub const NVME_FEATURE_DSSD_POWER_STATE: NVME_FEATURES = 199;
pub const NVME_FEATURE_ENABLE_IEEE1667_SILO: NVME_FEATURES = 196;
pub const NVME_FEATURE_ENDURANCE_GROUP_EVENT_CONFIG: NVME_FEATURES = 24;
pub const NVME_FEATURE_ENHANCED_CONTROLLER_METADATA: NVME_FEATURES = 125;
pub const NVME_FEATURE_ERROR_INJECTION: NVME_FEATURES = 192;
pub const NVME_FEATURE_ERROR_RECOVERY: NVME_FEATURES = 5;
pub const NVME_FEATURE_HOST_BEHAVIOR_SUPPORT: NVME_FEATURES = 22;
pub const NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT: NVME_FEATURES = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FEATURE_HOST_IDENTIFIER_DATA {
    pub HOSTID: [u8; 16],
}
impl Default for NVME_FEATURE_HOST_IDENTIFIER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_FEATURE_HOST_MEMORY_BUFFER: NVME_FEATURES = 13;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FEATURE_HOST_METADATA_DATA {
    pub NumberOfMetadataElementDescriptors: u8,
    pub Reserved0: u8,
    pub MetadataElementDescriptors: [u8; 4094],
}
impl Default for NVME_FEATURE_HOST_METADATA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FEATURE_IDENTIFIERS_EFFECTS_LOG {
    pub FeatureIdentifierSupported: [NVME_FID_SUPPORTED_AND_EFFECTS; 256],
}
impl Default for NVME_FEATURE_IDENTIFIERS_EFFECTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_FEATURE_INTERRUPT_COALESCING: NVME_FEATURES = 8;
pub const NVME_FEATURE_INTERRUPT_VECTOR_CONFIG: NVME_FEATURES = 9;
pub const NVME_FEATURE_IO_COMMAND_SET_PROFILE: NVME_FEATURES = 25;
pub const NVME_FEATURE_KEEP_ALIVE: NVME_FEATURES = 15;
pub const NVME_FEATURE_LATENCY_MONITOR: NVME_FEATURES = 197;
pub const NVME_FEATURE_LBA_RANGE_TYPE: NVME_FEATURES = 3;
pub const NVME_FEATURE_LBA_STATUS_INFORMATION_REPORT_INTERVAL: NVME_FEATURES = 21;
pub const NVME_FEATURE_NAMESPACE_METADATA: NVME_FEATURES = 127;
pub const NVME_FEATURE_NONOPERATIONAL_POWER_STATE: NVME_FEATURES = 17;
pub const NVME_FEATURE_NUMBER_OF_QUEUES: NVME_FEATURES = 7;
pub const NVME_FEATURE_NVM_HOST_IDENTIFIER: NVME_FEATURES = 129;
pub const NVME_FEATURE_NVM_NAMESPACE_WRITE_PROTECTION_CONFIG: NVME_FEATURES = 132;
pub const NVME_FEATURE_NVM_RESERVATION_NOTIFICATION_MASK: NVME_FEATURES = 130;
pub const NVME_FEATURE_NVM_RESERVATION_PERSISTANCE: NVME_FEATURES = 131;
pub const NVME_FEATURE_NVM_SOFTWARE_PROGRESS_MARKER: NVME_FEATURES = 128;
pub const NVME_FEATURE_PLP_HEALTH_CHECK_INTERVAL: NVME_FEATURES = 198;
pub const NVME_FEATURE_POWER_MANAGEMENT: NVME_FEATURES = 2;
pub const NVME_FEATURE_PREDICTABLE_LATENCY_MODE_CONFIG: NVME_FEATURES = 19;
pub const NVME_FEATURE_PREDICTABLE_LATENCY_MODE_WINDOW: NVME_FEATURES = 20;
pub const NVME_FEATURE_READONLY_WRITETHROUGH_MODE: NVME_FEATURES = 194;
pub const NVME_FEATURE_READ_RECOVERY_LEVEL_CONFIG: NVME_FEATURES = 18;
pub const NVME_FEATURE_SANITIZE_CONFIG: NVME_FEATURES = 23;
pub const NVME_FEATURE_TEMPERATURE_THRESHOLD: NVME_FEATURES = 4;
pub const NVME_FEATURE_TIMESTAMP: NVME_FEATURES = 14;
pub type NVME_FEATURE_VALUE_CODES = i32;
pub const NVME_FEATURE_VALUE_CURRENT: NVME_FEATURE_VALUE_CODES = 0;
pub const NVME_FEATURE_VALUE_DEFAULT: NVME_FEATURE_VALUE_CODES = 1;
pub const NVME_FEATURE_VALUE_SAVED: NVME_FEATURE_VALUE_CODES = 2;
pub const NVME_FEATURE_VALUE_SUPPORTED_CAPABILITIES: NVME_FEATURE_VALUE_CODES = 3;
pub const NVME_FEATURE_VOLATILE_WRITE_CACHE: NVME_FEATURES = 6;
pub const NVME_FEATURE_WRITE_ATOMICITY: NVME_FEATURES = 10;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_FID_SUPPORTED_AND_EFFECTS {
    pub _bitfield: u32,
}
impl NVME_FID_SUPPORTED_AND_EFFECTS {
    pub fn FSUPP(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_FSUPP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn UDCC(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_UDCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn NCC(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_NCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn NIC(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_NIC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn CCC(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_CCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 13) >> 18
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16383 << 5)) | ((value & 16383) << 5);
    }
    pub fn UUIDSelectionSupported(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_UUIDSelectionSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u32) << 19);
    }
    pub fn FSPNamespace(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_FSPNamespace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u32) << 20);
    }
    pub fn FSPController(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_FSPController(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u32) << 21);
    }
    pub fn FSPNVMSet(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_FSPNVMSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u32) << 22);
    }
    pub fn FSPEnduranceGroup(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_FSPEnduranceGroup(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u32) << 23);
    }
    pub fn FSPDomain(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_FSPDomain(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u32) << 24);
    }
    pub fn FSPNVMSubsystem(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_FSPNVMSubsystem(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u32) << 25);
    }
    pub fn FSPReserved(&self) -> u32 {
        self._bitfield >> 26
    }
    pub fn set_FSPReserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(63 << 26)) | ((value & 63) << 26);
    }
}
pub type NVME_FIRMWARE_ACTIVATE_ACTIONS = i32;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_ACTIVATE: NVME_FIRMWARE_ACTIVATE_ACTIONS = 2;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_ACTIVATE_BOOT_PARTITION: NVME_FIRMWARE_ACTIVATE_ACTIONS = 7;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT: NVME_FIRMWARE_ACTIVATE_ACTIONS = 0;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE: NVME_FIRMWARE_ACTIVATE_ACTIONS = 1;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE_IMMEDIATE: NVME_FIRMWARE_ACTIVATE_ACTIONS = 3;
pub const NVME_FIRMWARE_ACTIVATE_ACTION_REPLACE_BOOT_PARTITION: NVME_FIRMWARE_ACTIVATE_ACTIONS = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG {
    pub AFI: NVME_FIRMWARE_SLOT_INFO_LOG_0,
    pub Reserved0: [u8; 7],
    pub FRS: [u64; 7],
    pub Reserved1: [u8; 448],
}
impl Default for NVME_FIRMWARE_SLOT_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG_0 {
    pub _bitfield: u8,
}
impl NVME_FIRMWARE_SLOT_INFO_LOG_0 {
    pub fn ActiveSlot(&self) -> u8 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_ActiveSlot(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_Reserved0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn PendingActivateSlot(&self) -> u8 {
        (self._bitfield << 1) >> 5
    }
    pub fn set_PendingActivateSlot(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 4)) | ((value & 7) << 4);
    }
    pub fn Reserved1(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Reserved1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
pub type NVME_FUSED_OPERATION_CODES = i32;
pub const NVME_FUSED_OPERATION_FIRST_CMD: NVME_FUSED_OPERATION_CODES = 1;
pub const NVME_FUSED_OPERATION_NORMAL: NVME_FUSED_OPERATION_CODES = 0;
pub const NVME_FUSED_OPERATION_SECOND_CMD: NVME_FUSED_OPERATION_CODES = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_GET_FEATURE_TIMESTAMP {
    pub Anonymous: NVME_GET_FEATURE_TIMESTAMP_0,
    pub AsUlonglong: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_GET_FEATURE_TIMESTAMP_0 {
    pub _bitfield: u64,
}
impl NVME_GET_FEATURE_TIMESTAMP_0 {
    pub fn Timestamp(&self) -> u64 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_Timestamp(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !281474976710655) | (value & 281474976710655);
    }
    pub fn Synch(&self) -> bool {
        (self._bitfield >> 48) & 1 != 0
    }
    pub fn set_Synch(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 48)) | ((value as u64) << 48);
    }
    pub fn Origin(&self) -> u64 {
        (self._bitfield << 12) >> 61
    }
    pub fn set_Origin(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(7 << 49)) | ((value & 7) << 49);
    }
    pub fn Reserved(&self) -> u64 {
        self._bitfield >> 52
    }
    pub fn set_Reserved(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4095 << 52)) | ((value & 4095) << 52);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_HEALTH_INFO_LOG {
    pub CriticalWarning: NVME_HEALTH_INFO_LOG_0,
    pub Temperature: [u8; 2],
    pub AvailableSpare: u8,
    pub AvailableSpareThreshold: u8,
    pub PercentageUsed: u8,
    pub Reserved0: [u8; 26],
    pub DataUnitRead: [u8; 16],
    pub DataUnitWritten: [u8; 16],
    pub HostReadCommands: [u8; 16],
    pub HostWrittenCommands: [u8; 16],
    pub ControllerBusyTime: [u8; 16],
    pub PowerCycle: [u8; 16],
    pub PowerOnHours: [u8; 16],
    pub UnsafeShutdowns: [u8; 16],
    pub MediaErrors: [u8; 16],
    pub ErrorInfoLogEntryCount: [u8; 16],
    pub WarningCompositeTemperatureTime: u32,
    pub CriticalCompositeTemperatureTime: u32,
    pub TemperatureSensor1: u16,
    pub TemperatureSensor2: u16,
    pub TemperatureSensor3: u16,
    pub TemperatureSensor4: u16,
    pub TemperatureSensor5: u16,
    pub TemperatureSensor6: u16,
    pub TemperatureSensor7: u16,
    pub TemperatureSensor8: u16,
    pub Reserved1: [u8; 296],
}
impl Default for NVME_HEALTH_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_HEALTH_INFO_LOG_0 {
    pub Anonymous: NVME_HEALTH_INFO_LOG_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_HEALTH_INFO_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_HEALTH_INFO_LOG_0_0 {
    pub _bitfield: u8,
}
impl NVME_HEALTH_INFO_LOG_0_0 {
    pub fn AvailableSpaceLow(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_AvailableSpaceLow(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn TemperatureThreshold(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_TemperatureThreshold(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn ReliabilityDegraded(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ReliabilityDegraded(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn ReadOnly(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ReadOnly(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn VolatileMemoryBackupDeviceFailed(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_VolatileMemoryBackupDeviceFailed(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_HOST_BEHAVIOR_SUPPORT_DATA {
    pub ACRE: u8,
    pub ETDAS: u8,
    pub LBAFEE: u8,
    pub Reserved: [u8; 509],
}
impl Default for NVME_HOST_BEHAVIOR_SUPPORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_HOST_IDENTIFIER_SIZE: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY {
    pub BADD: u64,
    pub BSIZE: u32,
    pub Reserved: u32,
}
pub const NVME_HOST_METADATA_ADD_ENTRY_MULTIPLE: NVME_HOST_METADATA_ELEMENT_ACTIONS = 2;
pub const NVME_HOST_METADATA_ADD_REPLACE_ENTRY: NVME_HOST_METADATA_ELEMENT_ACTIONS = 0;
pub const NVME_HOST_METADATA_DELETE_ENTRY_MULTIPLE: NVME_HOST_METADATA_ELEMENT_ACTIONS = 1;
pub type NVME_HOST_METADATA_ELEMENT_ACTIONS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    pub _bitfield: u32,
    pub EVAL: [u8; 1],
}
impl NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    pub fn ET(&self) -> u32 {
        (self._bitfield << 26) >> 26
    }
    pub fn set_ET(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !63) | (value & 63);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 30
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn ER(&self) -> u32 {
        (self._bitfield << 20) >> 28
    }
    pub fn set_ER(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn Reserved1(&self) -> u32 {
        (self._bitfield << 16) >> 28
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn ELEN(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_ELEN(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
impl Default for NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_IDENTIFIER_TYPE = i32;
pub const NVME_IDENTIFIER_TYPE_CSI: NVME_IDENTIFIER_TYPE = 4;
pub const NVME_IDENTIFIER_TYPE_CSI_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 1;
pub const NVME_IDENTIFIER_TYPE_EUI64: NVME_IDENTIFIER_TYPE = 1;
pub const NVME_IDENTIFIER_TYPE_EUI64_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 8;
pub type NVME_IDENTIFIER_TYPE_LENGTH = i32;
pub const NVME_IDENTIFIER_TYPE_NGUID: NVME_IDENTIFIER_TYPE = 2;
pub const NVME_IDENTIFIER_TYPE_NGUID_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 16;
pub const NVME_IDENTIFIER_TYPE_UUID: NVME_IDENTIFIER_TYPE = 3;
pub const NVME_IDENTIFIER_TYPE_UUID_LENGTH: NVME_IDENTIFIER_TYPE_LENGTH = 16;
pub const NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES: NVME_IDENTIFY_CNS_CODES = 2;
pub const NVME_IDENTIFY_CNS_ACTIVE_NAMESPACE_LIST_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 7;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE: NVME_IDENTIFY_CNS_CODES = 17;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 27;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST: NVME_IDENTIFY_CNS_CODES = 16;
pub const NVME_IDENTIFY_CNS_ALLOCATED_NAMSPACE_LIST_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 26;
pub type NVME_IDENTIFY_CNS_CODES = i32;
pub const NVME_IDENTIFY_CNS_CONTROLLER: NVME_IDENTIFY_CNS_CODES = 1;
pub const NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID: NVME_IDENTIFY_CNS_CODES = 18;
pub const NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM: NVME_IDENTIFY_CNS_CODES = 19;
pub const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE: NVME_IDENTIFY_CNS_CODES = 3;
pub const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE_SIZE: u32 = 4096;
pub const NVME_IDENTIFY_CNS_DOMAIN_LIST: NVME_IDENTIFY_CNS_CODES = 24;
pub const NVME_IDENTIFY_CNS_ENDURANCE_GROUP_LIST: NVME_IDENTIFY_CNS_CODES = 25;
pub const NVME_IDENTIFY_CNS_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 28;
pub const NVME_IDENTIFY_CNS_NAMESPACE_GRANULARITY_LIST: NVME_IDENTIFY_CNS_CODES = 22;
pub const NVME_IDENTIFY_CNS_NVM_SET: NVME_IDENTIFY_CNS_CODES = 4;
pub const NVME_IDENTIFY_CNS_PRIMARY_CONTROLLER_CAPABILITIES: NVME_IDENTIFY_CNS_CODES = 20;
pub const NVME_IDENTIFY_CNS_SECONDARY_CONTROLLER_LIST: NVME_IDENTIFY_CNS_CODES = 21;
pub const NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 6;
pub const NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE: NVME_IDENTIFY_CNS_CODES = 0;
pub const NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE_IO_COMMAND_SET: NVME_IDENTIFY_CNS_CODES = 5;
pub const NVME_IDENTIFY_CNS_UUID_LIST: NVME_IDENTIFY_CNS_CODES = 23;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA {
    pub VID: u16,
    pub SSVID: u16,
    pub SN: [u8; 20],
    pub MN: [u8; 40],
    pub FR: [u8; 8],
    pub RAB: u8,
    pub IEEE: [u8; 3],
    pub CMIC: NVME_IDENTIFY_CONTROLLER_DATA_0,
    pub MDTS: u8,
    pub CNTLID: u16,
    pub VER: u32,
    pub RTD3R: u32,
    pub RTD3E: u32,
    pub OAES: NVME_IDENTIFY_CONTROLLER_DATA_1,
    pub CTRATT: NVME_IDENTIFY_CONTROLLER_DATA_2,
    pub RRLS: NVME_IDENTIFY_CONTROLLER_DATA_3,
    pub Reserved0: [u8; 9],
    pub CNTRLTYPE: u8,
    pub FGUID: [u8; 16],
    pub CRDT1: u16,
    pub CRDT2: u16,
    pub CRDT3: u16,
    pub Reserved1: [u8; 106],
    pub ReservedForManagement: [u8; 13],
    pub NVMSR: u8,
    pub VWCI: u8,
    pub MEC: u8,
    pub OACS: NVME_IDENTIFY_CONTROLLER_DATA_4,
    pub ACL: u8,
    pub AERL: u8,
    pub FRMW: NVME_IDENTIFY_CONTROLLER_DATA_5,
    pub LPA: NVME_IDENTIFY_CONTROLLER_DATA_6,
    pub ELPE: u8,
    pub NPSS: u8,
    pub AVSCC: NVME_IDENTIFY_CONTROLLER_DATA_7,
    pub APSTA: NVME_IDENTIFY_CONTROLLER_DATA_8,
    pub WCTEMP: u16,
    pub CCTEMP: u16,
    pub MTFA: u16,
    pub HMPRE: u32,
    pub HMMIN: u32,
    pub TNVMCAP: [u8; 16],
    pub UNVMCAP: [u8; 16],
    pub RPMBS: NVME_IDENTIFY_CONTROLLER_DATA_9,
    pub EDSTT: u16,
    pub DSTO: u8,
    pub FWUG: u8,
    pub KAS: u16,
    pub HCTMA: NVME_IDENTIFY_CONTROLLER_DATA_10,
    pub MNTMT: u16,
    pub MXTMT: u16,
    pub SANICAP: NVME_IDENTIFY_CONTROLLER_DATA_11,
    pub HMMINDS: u32,
    pub HMMAXD: u16,
    pub NSETIDMAX: u16,
    pub ENDGIDMAX: u16,
    pub ANATT: u8,
    pub ANACAP: NVME_IDENTIFY_CONTROLLER_DATA_12,
    pub ANAGRPMAX: u32,
    pub NANAGRPID: u32,
    pub PELS: u32,
    pub DomainId: u16,
    pub Reserved2: [u8; 10],
    pub MEGCAP: [u8; 16],
    pub TMPTHHA: u8,
    pub Reserved3: u8,
    pub CQT: u16,
    pub Reserved4: [u8; 124],
    pub SQES: NVME_IDENTIFY_CONTROLLER_DATA_13,
    pub CQES: NVME_IDENTIFY_CONTROLLER_DATA_14,
    pub MAXCMD: u16,
    pub NN: u32,
    pub ONCS: NVME_IDENTIFY_CONTROLLER_DATA_15,
    pub FUSES: NVME_IDENTIFY_CONTROLLER_DATA_16,
    pub FNA: NVME_IDENTIFY_CONTROLLER_DATA_17,
    pub VWC: NVME_IDENTIFY_CONTROLLER_DATA_18,
    pub AWUN: u16,
    pub AWUPF: u16,
    pub NVSCC: NVME_IDENTIFY_CONTROLLER_DATA_19,
    pub NWPC: NVME_IDENTIFY_CONTROLLER_DATA_20,
    pub ACWU: u16,
    pub CopyDescFormats: u16,
    pub SGLS: NVME_IDENTIFY_CONTROLLER_DATA_21,
    pub MNAN: u32,
    pub MAXDNA: [u8; 16],
    pub MAXCNA: u32,
    pub Reserved6: [u8; 204],
    pub SUBNQN: [u8; 256],
    pub Reserved7: [u8; 768],
    pub IOCCSZ: u32,
    pub IORCSZ: u32,
    pub ICDOFF: u16,
    pub FCATT: NVME_IDENTIFY_CONTROLLER_DATA_22,
    pub MSDBD: u8,
    pub OFCS: NVME_IDENTIFY_CONTROLLER_DATA_23,
    pub DCTYPE: u8,
    pub Reserved8: [u8; 241],
    pub PDS: [NVME_POWER_STATE_DESC; 32],
    pub VS: [u8; 1024],
}
impl Default for NVME_IDENTIFY_CONTROLLER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_0 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_0 {
    pub fn MultiPorts(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_MultiPorts(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn MultiControllers(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_MultiControllers(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn SRIOV(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_SRIOV(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn ANAR(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ANAR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_1 {
    pub _bitfield: u32,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_1 {
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn NamespaceAttributeChanged(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_NamespaceAttributeChanged(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn FirmwareActivation(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_FirmwareActivation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn Reserved1(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_Reserved1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u32) << 10);
    }
    pub fn AsymmetricAccessChanged(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_AsymmetricAccessChanged(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u32) << 11);
    }
    pub fn PredictableLatencyAggregateLogChanged(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_PredictableLatencyAggregateLogChanged(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u32) << 12);
    }
    pub fn LbaStatusChanged(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_LbaStatusChanged(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u32) << 13);
    }
    pub fn EnduranceGroupAggregateLogChanged(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_EnduranceGroupAggregateLogChanged(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u32) << 14);
    }
    pub fn NormalNvmSubsystemShutdown(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_NormalNvmSubsystemShutdown(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u32) << 15);
    }
    pub fn Reserved2(&self) -> u32 {
        (self._bitfield << 5) >> 21
    }
    pub fn set_Reserved2(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(2047 << 16)) | ((value & 2047) << 16);
    }
    pub fn ZoneInformation(&self) -> bool {
        (self._bitfield >> 27) & 1 != 0
    }
    pub fn set_ZoneInformation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value as u32) << 27);
    }
    pub fn Reserved3(&self) -> u32 {
        (self._bitfield << 1) >> 29
    }
    pub fn set_Reserved3(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 28)) | ((value & 7) << 28);
    }
    pub fn DiscoveryLogChanged(&self) -> bool {
        (self._bitfield >> 31) & 1 != 0
    }
    pub fn set_DiscoveryLogChanged(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value as u32) << 31);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_10 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_10 {
    pub fn Supported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_11 {
    pub _bitfield: u32,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_11 {
    pub fn CryptoErase(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CryptoErase(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn BlockErase(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_BlockErase(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Overwrite(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Overwrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 3) >> 6
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(67108863 << 3)) | ((value & 67108863) << 3);
    }
    pub fn NDI(&self) -> bool {
        (self._bitfield >> 29) & 1 != 0
    }
    pub fn set_NDI(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 29)) | ((value as u32) << 29);
    }
    pub fn NODMMAS(&self) -> u32 {
        self._bitfield >> 30
    }
    pub fn set_NODMMAS(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 30)) | ((value & 3) << 30);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_12 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_12 {
    pub fn OptimizedState(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_OptimizedState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn NonOptimizedState(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NonOptimizedState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn InaccessibleState(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_InaccessibleState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn PersistentLossState(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_PersistentLossState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn ChangeState(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_ChangeState(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u8) << 5);
    }
    pub fn StaticANAGRPID(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_StaticANAGRPID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u8) << 6);
    }
    pub fn SupportNonZeroANAGRPID(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_SupportNonZeroANAGRPID(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_13 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_13 {
    pub fn RequiredEntrySize(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_RequiredEntrySize(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MaxEntrySize(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_MaxEntrySize(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_14 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_14 {
    pub fn RequiredEntrySize(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_RequiredEntrySize(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MaxEntrySize(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_MaxEntrySize(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_15 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_15 {
    pub fn Compare(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Compare(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn WriteUncorrectable(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_WriteUncorrectable(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn DatasetManagement(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_DatasetManagement(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn WriteZeroes(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_WriteZeroes(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn FeatureField(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_FeatureField(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Reservations(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Reservations(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn Timestamp(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_Timestamp(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn Verify(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Verify(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_16 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_16 {
    pub fn CompareAndWrite(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CompareAndWrite(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_17 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_17 {
    pub fn FormatApplyToAll(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_FormatApplyToAll(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn SecureEraseApplyToAll(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_SecureEraseApplyToAll(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn CryptographicEraseSupported(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_CryptographicEraseSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn FormatSupportNSIDAllF(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_FormatSupportNSIDAllF(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_18 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_18 {
    pub fn Present(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Present(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn FlushBehavior(&self) -> u8 {
        (self._bitfield << 5) >> 6
    }
    pub fn set_FlushBehavior(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 1)) | ((value & 3) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_19 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_19 {
    pub fn CommandFormatInSpec(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CommandFormatInSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_2 {
    pub _bitfield: u32,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_2 {
    pub fn HostIdentifier128Bit(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_HostIdentifier128Bit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn NOPSPMode(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NOPSPMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn NVMSets(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_NVMSets(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn ReadRecoveryLevels(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ReadRecoveryLevels(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn EnduranceGroups(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_EnduranceGroups(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn PredictableLatencyMode(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_PredictableLatencyMode(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u32) << 5);
    }
    pub fn TBKAS(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_TBKAS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u32) << 6);
    }
    pub fn NamespaceGranularity(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_NamespaceGranularity(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u32) << 7);
    }
    pub fn SQAssociations(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_SQAssociations(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u32) << 8);
    }
    pub fn UUIDList(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_UUIDList(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u32) << 9);
    }
    pub fn MultiDomainSubsystem(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_MultiDomainSubsystem(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u32) << 10);
    }
    pub fn FixedCapacityManagement(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_FixedCapacityManagement(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u32) << 11);
    }
    pub fn VariableCapacityManagement(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_VariableCapacityManagement(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u32) << 12);
    }
    pub fn DeleteEnduranceGroup(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_DeleteEnduranceGroup(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u32) << 13);
    }
    pub fn DeleteNVMSet(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_DeleteNVMSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u32) << 14);
    }
    pub fn ELBAS(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_ELBAS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u32) << 15);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_20 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_20 {
    pub fn WriteProtect(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_WriteProtect(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn UntilPowerCycle(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_UntilPowerCycle(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Permanent(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Permanent(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_21 {
    pub _bitfield: u32,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_21 {
    pub fn SGLSupported(&self) -> u32 {
        (self._bitfield << 30) >> 30
    }
    pub fn set_SGLSupported(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn KeyedSGLData(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_KeyedSGLData(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 19
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
    pub fn BitBucketDescrSupported(&self) -> bool {
        (self._bitfield >> 16) & 1 != 0
    }
    pub fn set_BitBucketDescrSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value as u32) << 16);
    }
    pub fn ByteAlignedContiguousPhysicalBuffer(&self) -> bool {
        (self._bitfield >> 17) & 1 != 0
    }
    pub fn set_ByteAlignedContiguousPhysicalBuffer(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value as u32) << 17);
    }
    pub fn SGLLengthLargerThanDataLength(&self) -> bool {
        (self._bitfield >> 18) & 1 != 0
    }
    pub fn set_SGLLengthLargerThanDataLength(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value as u32) << 18);
    }
    pub fn MPTRSGLDescriptor(&self) -> bool {
        (self._bitfield >> 19) & 1 != 0
    }
    pub fn set_MPTRSGLDescriptor(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value as u32) << 19);
    }
    pub fn AddressFieldSGLDataBlock(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_AddressFieldSGLDataBlock(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u32) << 20);
    }
    pub fn TransportSGLData(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_TransportSGLData(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u32) << 21);
    }
    pub fn Reserved1(&self) -> u32 {
        self._bitfield >> 22
    }
    pub fn set_Reserved1(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1023 << 22)) | ((value & 1023) << 22);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_22 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_22 {
    pub fn StaticControllerModel(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_StaticControllerModel(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_23 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_23 {
    pub fn IOQueueDeletion(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_IOQueueDeletion(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_3 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_3 {
    pub fn ReadRecoveryLevel0(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ReadRecoveryLevel0(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn ReadRecoveryLevel1(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn ReadRecoveryLevel2(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn ReadRecoveryLevel3(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn ReadRecoveryLevel4(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel4(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn ReadRecoveryLevel5(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel5(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn ReadRecoveryLevel6(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel6(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn ReadRecoveryLevel7(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel7(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn ReadRecoveryLevel8(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel8(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn ReadRecoveryLevel9(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel9(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn ReadRecoveryLevel10(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel10(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn ReadRecoveryLevel11(&self) -> bool {
        (self._bitfield >> 11) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel11(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value as u16) << 11);
    }
    pub fn ReadRecoveryLevel12(&self) -> bool {
        (self._bitfield >> 12) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel12(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value as u16) << 12);
    }
    pub fn ReadRecoveryLevel13(&self) -> bool {
        (self._bitfield >> 13) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel13(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value as u16) << 13);
    }
    pub fn ReadRecoveryLevel14(&self) -> bool {
        (self._bitfield >> 14) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel14(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value as u16) << 14);
    }
    pub fn ReadRecoveryLevel15(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_ReadRecoveryLevel15(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_4 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_4 {
    pub fn SecurityCommands(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SecurityCommands(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn FormatNVM(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_FormatNVM(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn FirmwareCommands(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_FirmwareCommands(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn NamespaceCommands(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_NamespaceCommands(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn DeviceSelfTest(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_DeviceSelfTest(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Directives(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_Directives(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u16) << 5);
    }
    pub fn NVMeMICommands(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_NVMeMICommands(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u16) << 6);
    }
    pub fn VirtualizationMgmt(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_VirtualizationMgmt(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn DoorBellBufferConfig(&self) -> bool {
        (self._bitfield >> 8) & 1 != 0
    }
    pub fn set_DoorBellBufferConfig(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value as u16) << 8);
    }
    pub fn GetLBAStatus(&self) -> bool {
        (self._bitfield >> 9) & 1 != 0
    }
    pub fn set_GetLBAStatus(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value as u16) << 9);
    }
    pub fn CommandFeatureLockdown(&self) -> bool {
        (self._bitfield >> 10) & 1 != 0
    }
    pub fn set_CommandFeatureLockdown(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value as u16) << 10);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 11
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(31 << 11)) | ((value & 31) << 11);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_5 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_5 {
    pub fn Slot1ReadOnly(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Slot1ReadOnly(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn SlotCount(&self) -> u8 {
        (self._bitfield << 4) >> 5
    }
    pub fn set_SlotCount(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn ActivationWithoutReset(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_ActivationWithoutReset(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_6 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_6 {
    pub fn SmartPagePerNamespace(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SmartPagePerNamespace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn CommandEffectsLog(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CommandEffectsLog(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn LogPageExtendedData(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_LogPageExtendedData(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn TelemetrySupport(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_TelemetrySupport(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn PersistentEventLog(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_PersistentEventLog(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn SupportedLogPages(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_SupportedLogPages(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u8) << 5);
    }
    pub fn TelemetryDataArea4(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_TelemetryDataArea4(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u8) << 6);
    }
    pub fn Reserved1(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Reserved1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_7 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_7 {
    pub fn CommandFormatInSpec(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CommandFormatInSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_8 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_8 {
    pub fn Supported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_9 {
    pub _bitfield: u32,
}
impl NVME_IDENTIFY_CONTROLLER_DATA_9 {
    pub fn RPMBUnitCount(&self) -> u32 {
        (self._bitfield << 29) >> 29
    }
    pub fn set_RPMBUnitCount(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn AuthenticationMethod(&self) -> u32 {
        (self._bitfield << 26) >> 29
    }
    pub fn set_AuthenticationMethod(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn Reserved0(&self) -> u32 {
        (self._bitfield << 16) >> 22
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1023 << 6)) | ((value & 1023) << 6);
    }
    pub fn TotalSize(&self) -> u32 {
        (self._bitfield << 8) >> 24
    }
    pub fn set_TotalSize(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn AccessSize(&self) -> u32 {
        self._bitfield >> 24
    }
    pub fn set_AccessSize(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_IO_COMMAND_SET {
    pub IOCommandSetVector: [IO_COMMAND_SET_VECTOR; 512],
}
impl Default for NVME_IDENTIFY_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA {
    pub NSZE: u64,
    pub NCAP: u64,
    pub NUSE: u64,
    pub NSFEAT: NVME_IDENTIFY_NAMESPACE_DATA_0,
    pub NLBAF: u8,
    pub FLBAS: NVME_IDENTIFY_NAMESPACE_DATA_1,
    pub MC: NVME_IDENTIFY_NAMESPACE_DATA_2,
    pub DPC: NVME_IDENTIFY_NAMESPACE_DATA_3,
    pub DPS: NVME_IDENTIFY_NAMESPACE_DATA_4,
    pub NMIC: NVME_IDENTIFY_NAMESPACE_DATA_5,
    pub RESCAP: NVM_RESERVATION_CAPABILITIES,
    pub FPI: NVME_IDENTIFY_NAMESPACE_DATA_6,
    pub DLFEAT: NVME_IDENTIFY_NAMESPACE_DATA_7,
    pub NAWUN: u16,
    pub NAWUPF: u16,
    pub NACWU: u16,
    pub NABSN: u16,
    pub NABO: u16,
    pub NABSPF: u16,
    pub NOIOB: u16,
    pub NVMCAP: [u8; 16],
    pub NPWG: u16,
    pub NPWA: u16,
    pub NPDG: u16,
    pub NPDA: u16,
    pub NOWS: u16,
    pub MSSRL: u16,
    pub MCL: u32,
    pub MSRC: u8,
    pub Reserved2: [u8; 11],
    pub ANAGRPID: u32,
    pub Reserved3: [u8; 3],
    pub NSATTR: NVME_IDENTIFY_NAMESPACE_DATA_8,
    pub NVMSETID: u16,
    pub ENDGID: u16,
    pub NGUID: [u8; 16],
    pub EUI64: [u8; 8],
    pub LBAF: [NVME_LBA_FORMAT; 64],
    pub VS: [u8; 3712],
}
impl Default for NVME_IDENTIFY_NAMESPACE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_0 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_0 {
    pub fn ThinProvisioning(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ThinProvisioning(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn NameSpaceAtomicWriteUnit(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NameSpaceAtomicWriteUnit(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn DeallocatedOrUnwrittenError(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_DeallocatedOrUnwrittenError(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn SkipReuseUI(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_SkipReuseUI(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn NameSpaceIoOptimization(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_NameSpaceIoOptimization(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_1 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_1 {
    pub fn LbaFormatIndex(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_LbaFormatIndex(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MetadataInExtendedDataLBA(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_MetadataInExtendedDataLBA(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn LbaFormatIndexMS(&self) -> u8 {
        (self._bitfield << 1) >> 6
    }
    pub fn set_LbaFormatIndexMS(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 5)) | ((value & 3) << 5);
    }
    pub fn Reserved(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_2 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_2 {
    pub fn MetadataInExtendedDataLBA(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_MetadataInExtendedDataLBA(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn MetadataInSeparateBuffer(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_MetadataInSeparateBuffer(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_3 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_3 {
    pub fn ProtectionInfoType1(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ProtectionInfoType1(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn ProtectionInfoType2(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ProtectionInfoType2(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn ProtectionInfoType3(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ProtectionInfoType3(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn InfoAtBeginningOfMetadata(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_InfoAtBeginningOfMetadata(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn InfoAtEndOfMetadata(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_InfoAtEndOfMetadata(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_4 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_4 {
    pub fn ProtectionInfoTypeEnabled(&self) -> u8 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_ProtectionInfoTypeEnabled(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn InfoAtBeginningOfMetadata(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_InfoAtBeginningOfMetadata(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_5 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_5 {
    pub fn SharedNameSpace(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_SharedNameSpace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_6 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_6 {
    pub fn PercentageRemained(&self) -> u8 {
        (self._bitfield << 1) >> 1
    }
    pub fn set_PercentageRemained(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Supported(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_7 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_7 {
    pub fn ReadBehavior(&self) -> u8 {
        (self._bitfield << 5) >> 5
    }
    pub fn set_ReadBehavior(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn WriteZeroes(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_WriteZeroes(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn GuardFieldWithCRC(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_GuardFieldWithCRC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_8 {
    pub _bitfield: u8,
}
impl NVME_IDENTIFY_NAMESPACE_DATA_8 {
    pub fn WriteProtected(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_WriteProtected(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    pub NIDT: u8,
    pub NIDL: u8,
    pub Reserved: [u8; 2],
    pub NID: [u8; 1],
}
impl Default for NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub VSL: u8,
    pub WZSL: u8,
    pub WUSL: u8,
    pub DMRL: u8,
    pub DMRSL: u32,
    pub DMSL: u64,
    pub Reserved: [u8; 4080],
}
impl Default for NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    pub ZOC: NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0,
    pub OZCS: NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1,
    pub MAR: u32,
    pub MOR: u32,
    pub RRL: u32,
    pub FRL: u32,
    pub Reserved0: [u8; 2796],
    pub LBAEF: [NVME_LBA_ZONE_FORMAT; 16],
    pub Reserved1: [u8; 768],
    pub VS: [u8; 256],
}
impl Default for NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0 {
    pub fn VariableZoneCapacity(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_VariableZoneCapacity(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn ZoneExcursions(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_ZoneExcursions(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1 {
    pub _bitfield: u16,
}
impl NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1 {
    pub fn ReadAcrossZoneBoundaries(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ReadAcrossZoneBoundaries(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub ZASL: u8,
    pub Reserved: [u8; 4095],
}
impl Default for NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_IO_COMMAND_SET_COMBINATION_REJECTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 43;
pub const NVME_IO_COMMAND_SET_INVALID: NVME_STATUS_COMMAND_SPECIFIC_CODES = 44;
pub const NVME_IO_COMMAND_SET_NOT_ENABLED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 42;
pub const NVME_IO_COMMAND_SET_NOT_SUPPORTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 41;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_LATENCY_MONITORING_ENTRY {
    pub ActiveBucketTimerThreshold: u16,
    pub ActiveThresholdA: u8,
    pub ActiveThresholdB: u8,
    pub ActiveThresholdC: u8,
    pub ActiveThresholdD: u8,
    pub ActiveLatencyConfig: u16,
    pub ActiveLatencyMinimumWindow: u8,
    pub DebugLogTriggerEnable: u16,
    pub DiscardDebugLog: u8,
    pub LatencyMonitorFeatureEnable: u8,
    pub Reserved0: [u8; 4083],
}
impl Default for NVME_LATENCY_MONITORING_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_LBA_FORMAT {
    pub Anonymous: NVME_LBA_FORMAT_0,
    pub AsUlong: u32,
}
impl Default for NVME_LBA_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LBA_FORMAT_0 {
    pub MS: u16,
    pub LBADS: u8,
    pub _bitfield: u8,
}
impl NVME_LBA_FORMAT_0 {
    pub fn RP(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_RP(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Reserved0(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved0(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_LBA_RANGE {
    pub Attributes: NVME_CONTEXT_ATTRIBUTES,
    pub LogicalBlockCount: u32,
    pub StartingLBA: u64,
}
impl Default for NVME_LBA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_LBA_RANGET_TYPE_ENTRY {
    pub Type: u8,
    pub Attributes: NVME_LBA_RANGET_TYPE_ENTRY_0,
    pub Reserved0: [u8; 14],
    pub SLBA: u64,
    pub NLB: u64,
    pub GUID: [u8; 16],
    pub Reserved1: [u8; 16],
}
impl Default for NVME_LBA_RANGET_TYPE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LBA_RANGET_TYPE_ENTRY_0 {
    pub _bitfield: u8,
}
impl NVME_LBA_RANGET_TYPE_ENTRY_0 {
    pub fn MayOverwritten(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_MayOverwritten(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Hidden(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_Hidden(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
pub type NVME_LBA_RANGE_TYPES = i32;
pub const NVME_LBA_RANGE_TYPE_CACHE: NVME_LBA_RANGE_TYPES = 3;
pub const NVME_LBA_RANGE_TYPE_FILESYSTEM: NVME_LBA_RANGE_TYPES = 1;
pub const NVME_LBA_RANGE_TYPE_PAGE_SWAP_FILE: NVME_LBA_RANGE_TYPES = 4;
pub const NVME_LBA_RANGE_TYPE_RAID: NVME_LBA_RANGE_TYPES = 2;
pub const NVME_LBA_RANGE_TYPE_RESERVED: NVME_LBA_RANGE_TYPES = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_LBA_ZONE_FORMAT {
    pub ZoneSize: u64,
    pub ZDES: u8,
    pub Reserved: [u8; 7],
}
impl Default for NVME_LBA_ZONE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG {
    pub _bitfield: u16,
}
impl NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG {
    pub fn EstablishContextAndRead512BytesOfHeaderSupported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_EstablishContextAndRead512BytesOfHeaderSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LID_SUPPORTED_AND_EFFECTS {
    pub _bitfield: u32,
}
impl NVME_LID_SUPPORTED_AND_EFFECTS {
    pub fn LSUPP(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_LSUPP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn IOS(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_IOS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 16) >> 18
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
    pub fn LIDSpecific(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_LIDSpecific(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
pub type NVME_LOG_PAGES = i32;
pub const NVME_LOG_PAGE_ASYMMETRIC_NAMESPACE_ACCESS: NVME_LOG_PAGES = 12;
pub const NVME_LOG_PAGE_BOOT_PARTITION: NVME_LOG_PAGES = 21;
pub const NVME_LOG_PAGE_CHANGED_NAMESPACE_LIST: NVME_LOG_PAGES = 4;
pub const NVME_LOG_PAGE_CHANGED_ZONE_LIST: NVME_LOG_PAGES = 191;
pub const NVME_LOG_PAGE_COMMAND_AND_FEATURE_LOCKDOWN: NVME_LOG_PAGES = 20;
pub const NVME_LOG_PAGE_COMMAND_EFFECTS: NVME_LOG_PAGES = 5;
pub const NVME_LOG_PAGE_DEVICE_SELF_TEST: NVME_LOG_PAGES = 6;
pub const NVME_LOG_PAGE_DISCOVERY: NVME_LOG_PAGES = 112;
pub const NVME_LOG_PAGE_ENDURANCE_GROUP_EVENT_AGGREGATE: NVME_LOG_PAGES = 15;
pub const NVME_LOG_PAGE_ENDURANCE_GROUP_INFORMATION: NVME_LOG_PAGES = 9;
pub const NVME_LOG_PAGE_ERROR_INFO: NVME_LOG_PAGES = 1;
pub const NVME_LOG_PAGE_FEATURE_IDENTIFIERS_SUPPORTED_AND_EFFECTS: NVME_LOG_PAGES = 18;
pub const NVME_LOG_PAGE_FIRMWARE_SLOT_INFO: NVME_LOG_PAGES = 3;
pub const NVME_LOG_PAGE_HEALTH_INFO: NVME_LOG_PAGES = 2;
pub const NVME_LOG_PAGE_LBA_STATUS_INFORMATION: NVME_LOG_PAGES = 14;
pub const NVME_LOG_PAGE_MEDIA_UNIT_STATUS: NVME_LOG_PAGES = 16;
pub const NVME_LOG_PAGE_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS: NVME_LOG_PAGES = 19;
pub const NVME_LOG_PAGE_OCP_DEVICE_CAPABILITIES: NVME_VENDOR_LOG_PAGES = 196;
pub const NVME_LOG_PAGE_OCP_DEVICE_ERROR_RECOVERY: NVME_VENDOR_LOG_PAGES = 193;
pub const NVME_LOG_PAGE_OCP_DEVICE_SMART_INFORMATION: NVME_VENDOR_LOG_PAGES = 192;
pub const NVME_LOG_PAGE_OCP_FIRMWARE_ACTIVATION_HISTORY: NVME_VENDOR_LOG_PAGES = 194;
pub const NVME_LOG_PAGE_OCP_LATENCY_MONITOR: NVME_VENDOR_LOG_PAGES = 195;
pub const NVME_LOG_PAGE_OCP_TCG_CONFIGURATION: NVME_VENDOR_LOG_PAGES = 200;
pub const NVME_LOG_PAGE_OCP_TCG_HISTORY: NVME_VENDOR_LOG_PAGES = 201;
pub const NVME_LOG_PAGE_OCP_UNSUPPORTED_REQUIREMENTS: NVME_VENDOR_LOG_PAGES = 197;
pub const NVME_LOG_PAGE_PERSISTENT_EVENT_LOG: NVME_LOG_PAGES = 13;
pub const NVME_LOG_PAGE_PREDICTABLE_LATENCY_EVENT_AGGREGATE: NVME_LOG_PAGES = 11;
pub const NVME_LOG_PAGE_PREDICTABLE_LATENCY_NVM_SET: NVME_LOG_PAGES = 10;
pub const NVME_LOG_PAGE_RESERVATION_NOTIFICATION: NVME_LOG_PAGES = 128;
pub const NVME_LOG_PAGE_ROTATIONAL_MEDIA_INFORMATION: NVME_LOG_PAGES = 22;
pub const NVME_LOG_PAGE_SANITIZE_STATUS: NVME_LOG_PAGES = 129;
pub const NVME_LOG_PAGE_SUPPORTED_CAPACITY_CONFIGURATION_LIST: NVME_LOG_PAGES = 17;
pub const NVME_LOG_PAGE_SUPPORTED_LOG_PAGES: NVME_LOG_PAGES = 0;
pub const NVME_LOG_PAGE_TELEMETRY_CTLR_INITIATED: NVME_LOG_PAGES = 8;
pub const NVME_LOG_PAGE_TELEMETRY_HOST_INITIATED: NVME_LOG_PAGES = 7;
pub const NVME_LOG_PAGE_WCS_DEVICE_ERROR_RECOVERY: u32 = 193;
pub const NVME_LOG_PAGE_WCS_DEVICE_SMART_ATTRIBUTES: u32 = 192;
pub const NVME_MAX_HOST_IDENTIFIER_SIZE: u32 = 16;
pub const NVME_MAX_LOG_PAGE_IDENTIFIER: u32 = 255;
pub const NVME_MAX_LOG_SIZE: u32 = 4096;
pub const NVME_MAX_UUID_INDEX: u32 = 127;
pub const NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE_NOT_DEFINED: NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = 0;
pub const NVME_MEDIA_ADDITIONALLY_MOFIDIED_AFTER_SANITIZE: NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = 2;
pub const NVME_MEDIA_NOT_ADDITIONALLY_MODIFIED_AFTER_SANITIZE: NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = 1;
pub const NVME_NAMESPACE_ALL: u32 = 4294967295;
pub type NVME_NAMESPACE_METADATA_ELEMENT_TYPES = i32;
pub const NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 1;
pub const NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_1: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 3;
pub const NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_2: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 4;
pub const NVME_NAMESPACE_METADATA_PREBOOT_NAMESPACE_NAME: NVME_NAMESPACE_METADATA_ELEMENT_TYPES = 2;
pub type NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = i32;
pub const NVME_NQN_MAX_LEN: u32 = 256;
pub const NVME_NQN_NAME_MAX_LEN: u32 = 223;
pub const NVME_NUM_FID_SUPPORTED: u32 = 256;
pub const NVME_NUM_LOG_PAGE_IDENTIFIERS: u32 = 256;
pub const NVME_NUM_NVME_MI_COMMANDS_SUPPORTED: u32 = 256;
pub const NVME_NUM_UUID_LIST_ENTRIES: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS {
    pub _bitfield: u32,
}
impl NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS {
    pub fn CSUPP(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CSUPP(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn UDCC(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_UDCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn NCC(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_NCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn NIC(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_NIC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u32) << 3);
    }
    pub fn CCC(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_CCC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u32) << 4);
    }
    pub fn Reserved(&self) -> u32 {
        (self._bitfield << 12) >> 17
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(32767 << 5)) | ((value & 32767) << 5);
    }
    pub fn CSPNamespace(&self) -> bool {
        (self._bitfield >> 20) & 1 != 0
    }
    pub fn set_CSPNamespace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value as u32) << 20);
    }
    pub fn CSPController(&self) -> bool {
        (self._bitfield >> 21) & 1 != 0
    }
    pub fn set_CSPController(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value as u32) << 21);
    }
    pub fn CSPNVMSet(&self) -> bool {
        (self._bitfield >> 22) & 1 != 0
    }
    pub fn set_CSPNVMSet(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value as u32) << 22);
    }
    pub fn CSPEnduranceGroup(&self) -> bool {
        (self._bitfield >> 23) & 1 != 0
    }
    pub fn set_CSPEnduranceGroup(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value as u32) << 23);
    }
    pub fn CSPDomain(&self) -> bool {
        (self._bitfield >> 24) & 1 != 0
    }
    pub fn set_CSPDomain(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value as u32) << 24);
    }
    pub fn CSPNVMSubsystem(&self) -> bool {
        (self._bitfield >> 25) & 1 != 0
    }
    pub fn set_CSPNVMSubsystem(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value as u32) << 25);
    }
    pub fn CSPReserved(&self) -> u32 {
        self._bitfield >> 26
    }
    pub fn set_CSPReserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(63 << 26)) | ((value & 63) << 26);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG {
    pub ManagementInterfaceCommandSupported: [NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS; 256],
    pub Reserved: [u8; 3072],
}
impl Default for NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_NVM_COMMANDS = i32;
pub const NVME_NVM_COMMAND_COMPARE: NVME_NVM_COMMANDS = 5;
pub const NVME_NVM_COMMAND_COPY: NVME_NVM_COMMANDS = 25;
pub const NVME_NVM_COMMAND_DATASET_MANAGEMENT: NVME_NVM_COMMANDS = 9;
pub const NVME_NVM_COMMAND_FLUSH: NVME_NVM_COMMANDS = 0;
pub const NVME_NVM_COMMAND_READ: NVME_NVM_COMMANDS = 2;
pub const NVME_NVM_COMMAND_RESERVATION_ACQUIRE: NVME_NVM_COMMANDS = 17;
pub const NVME_NVM_COMMAND_RESERVATION_REGISTER: NVME_NVM_COMMANDS = 13;
pub const NVME_NVM_COMMAND_RESERVATION_RELEASE: NVME_NVM_COMMANDS = 21;
pub const NVME_NVM_COMMAND_RESERVATION_REPORT: NVME_NVM_COMMANDS = 14;
pub const NVME_NVM_COMMAND_VERIFY: NVME_NVM_COMMANDS = 12;
pub const NVME_NVM_COMMAND_WRITE: NVME_NVM_COMMANDS = 1;
pub const NVME_NVM_COMMAND_WRITE_UNCORRECTABLE: NVME_NVM_COMMANDS = 4;
pub const NVME_NVM_COMMAND_WRITE_ZEROES: NVME_NVM_COMMANDS = 8;
pub const NVME_NVM_COMMAND_ZONE_APPEND: NVME_NVM_COMMANDS = 125;
pub const NVME_NVM_COMMAND_ZONE_MANAGEMENT_RECEIVE: NVME_NVM_COMMANDS = 122;
pub const NVME_NVM_COMMAND_ZONE_MANAGEMENT_SEND: NVME_NVM_COMMANDS = 121;
pub type NVME_NVM_QUEUE_PRIORITIES = i32;
pub const NVME_NVM_QUEUE_PRIORITY_HIGH: NVME_NVM_QUEUE_PRIORITIES = 1;
pub const NVME_NVM_QUEUE_PRIORITY_LOW: NVME_NVM_QUEUE_PRIORITIES = 3;
pub const NVME_NVM_QUEUE_PRIORITY_MEDIUM: NVME_NVM_QUEUE_PRIORITIES = 2;
pub const NVME_NVM_QUEUE_PRIORITY_URGENT: NVME_NVM_QUEUE_PRIORITIES = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_NVM_SUBSYSTEM_RESET {
    pub NSSRC: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_NVM_SUBSYSTEM_SHUTDOWN {
    pub NSSC: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG {
    pub PciePorts: u16,
    pub OobMgmtSupport: NVME_OCP_DEVICE_CAPABILITIES_LOG_0,
    pub WriteZeroesCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_1,
    pub SanitizeCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_2,
    pub DatasetMgmtCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_3,
    pub WriteUncorrectableCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_4,
    pub FusedCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_5,
    pub MinimumValidDSSDPowerState: u16,
    pub Reserved0: u8,
    pub DssdDescriptors: [DSSD_POWER_STATE_DESCRIPTOR; 127],
    pub Reserved1: [u8; 3934],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0 {
    pub _bitfield: u16,
}
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0 {
    pub fn MctpOverSMBusSupported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_MctpOverSMBusSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn MctpOverPcieVDMSupported(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_MctpOverPcieVDMSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn BasicMgmtCommandSupported(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_BasicMgmtCommandSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 4
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(4095 << 3)) | ((value & 4095) << 3);
    }
    pub fn CompliesWithSpec(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CompliesWithSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_1 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0 {
    pub _bitfield: u16,
}
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0 {
    pub fn Supported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn DEACBitSupported(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_DEACBitSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn FUABitSupported(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_FUABitSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn NvmeIo5Met(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_NvmeIo5Met(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn NvmeIo6Met(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_NvmeIo6Met(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 6
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1023 << 5)) | ((value & 1023) << 5);
    }
    pub fn CompliesWithSpec(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CompliesWithSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_2 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0 {
    pub _bitfield: u16,
}
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0 {
    pub fn Supported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn CryptoEraseSupported(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CryptoEraseSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn BlockEraseSupported(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_BlockEraseSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn OverwriteSupported(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_OverwriteSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn DeallocateLbaSupported(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_DeallocateLbaSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 6
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(1023 << 5)) | ((value & 1023) << 5);
    }
    pub fn CompliesWithSpec(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CompliesWithSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_3 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0 {
    pub _bitfield: u16,
}
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0 {
    pub fn Supported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn AttribDeallocateSupported(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_AttribDeallocateSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 3
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(8191 << 2)) | ((value & 8191) << 2);
    }
    pub fn CompliesWithSpec(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CompliesWithSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_4 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0 {
    pub _bitfield: u16,
}
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0 {
    pub fn Supported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_Supported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn SingleLBASupported(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_SingleLBASupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn MaxLBASupported(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_MaxLBASupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn NvmeIo14Met(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_NvmeIo14Met(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 5
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(2047 << 4)) | ((value & 2047) << 4);
    }
    pub fn CompliesWithSpec(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CompliesWithSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_5 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0,
    pub AsUshort: u16,
}
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0 {
    pub _bitfield: u16,
}
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0 {
    pub fn CWFusedSupported(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CWFusedSupported(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 2
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(16383 << 1)) | ((value & 16383) << 1);
    }
    pub fn CompliesWithSpec(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_CompliesWithSpec(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
pub const NVME_OCP_DEVICE_CAPABILITIES_LOG_VERSION_1: u32 = 1;
pub const NVME_OCP_DEVICE_DSSD_SPEC_MAJOR_VERSION_0: u32 = 0;
pub const NVME_OCP_DEVICE_DSSD_SPEC_MAJOR_VERSION_2: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {
    pub PanicResetWaitTime: u16,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DeviceRecoveryAction1: u8,
    pub PanicId: u64,
    pub DeviceCapabilitiesA: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: u8,
    pub Reserved0: [u8; 3],
    pub VendorSpecificCommandCDW12: u32,
    pub VendorSpecificCommandCDW13: u32,
    pub VendorSpecificCommandTimeout: u8,
    pub DeviceRecoveryAction2: u8,
    pub DeviceRecoveryAction2Timeout: u8,
    pub Reserved1: [u8; 463],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_VERSION_2: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {
    pub LID: u8,
    pub Reserved0: [u8; 3],
    pub ValidNumberOfEntries: u32,
    pub Entries: [FIRMWARE_ACTIVATION_HISTORY_ENTRY; 20],
    pub Reserved1: [u8; 2790],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG_VERSION_1: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {
    pub FeatureStatus: LATENCY_MONITOR_FEATURE_STATUS,
    pub Reserved0: u8,
    pub ActiveBucketTimer: u16,
    pub ActiveBucketTimerThreshold: u16,
    pub ActiveThresholdA: u8,
    pub ActiveThresholdB: u8,
    pub ActiveThresholdC: u8,
    pub ActiveThresholdD: u8,
    pub ActiveLatencyConfig: ACTIVE_LATENCY_CONFIGURATION,
    pub ActiveLatencyMinimumWindow: u8,
    pub Reserved1: [u8; 19],
    pub ActiveBucketCounter0: BUCKET_COUNTER,
    pub ActiveBucketCounter1: BUCKET_COUNTER,
    pub ActiveBucketCounter2: BUCKET_COUNTER,
    pub ActiveBucketCounter3: BUCKET_COUNTER,
    pub ActiveLatencyStamp: LATENCY_STAMP,
    pub ActiveMeasuredLatency: MEASURED_LATENCY,
    pub ActiveLatencyStampUnits: LATENCY_STAMP_UNITS,
    pub Reserved2: [u8; 22],
    pub StaticBucketCounter0: BUCKET_COUNTER,
    pub StaticBucketCounter1: BUCKET_COUNTER,
    pub StaticBucketCounter2: BUCKET_COUNTER,
    pub StaticBucketCounter3: BUCKET_COUNTER,
    pub StaticLatencyStamp: LATENCY_STAMP,
    pub StaticMeasuredLatency: MEASURED_LATENCY,
    pub StaticLatencyStampUnits: LATENCY_STAMP_UNITS,
    pub Reserved3: [u8; 22],
    pub DebugLogTriggerEnable: DEBUG_BIT_FIELD,
    pub DebugLogMeasuredLatency: u16,
    pub DebugLogLatencyStamp: u64,
    pub DebugLogPointer: u16,
    pub DebugCounterTriggerSource: DEBUG_BIT_FIELD,
    pub DebugLogStampUnits: NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0,
    pub Reserved4: [u8; 29],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0 {
    pub _bitfield: u8,
}
impl NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0 {
    pub fn BasedOnTimestamp(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_BasedOnTimestamp(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
pub const NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_VERSION_1: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {
    pub MediaUnitsWritten: [u8; 16],
    pub MediaUnitsRead: [u8; 16],
    pub BadUserNANDBlockCount: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0,
    pub BadSystemNANDBlockCount: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1,
    pub XORRecoveryCount: u64,
    pub UnrecoverableReadErrorCount: u64,
    pub SoftECCErrorCount: u64,
    pub EndToEndCorrectionCounts: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_2,
    pub PercentageSystemDataUsed: u8,
    pub RefreshCount: [u8; 7],
    pub UserDataEraseCounts: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_3,
    pub ThermalThrottling: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_4,
    pub DSSDSpecVersion: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_5,
    pub PCIeCorrectableErrorCount: u64,
    pub IncompleteShutdownCount: u32,
    pub Reserved1: u32,
    pub PercentageFreeBlocks: u8,
    pub Reserved2: [u8; 7],
    pub CapacitorHealth: u16,
    pub NvmeErrata: u8,
    pub Reserved3: [u8; 5],
    pub UnalignedIOCount: u64,
    pub SecurityVersionNumber: u64,
    pub NUSE: u64,
    pub PLPStartCount: [u8; 16],
    pub EnduranceEstimate: [u8; 16],
    pub PCIeLinkRetrainingCount: u64,
    pub PowerStateChangeCount: u64,
    pub Reserved4: [u8; 286],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_2 {
    pub DetectedCounts: u32,
    pub CorrectedCounts: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_3 {
    pub MaximumCount: u32,
    pub MinimumCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_4 {
    pub EventCount: u8,
    pub Status: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_5 {
    pub Errata: u8,
    pub PointVersion: u16,
    pub MinorVersion: u16,
    pub MajorVersion: u8,
}
pub const NVME_OCP_DEVICE_SMART_INFORMATION_LOG_VERSION_3: u32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {
    pub State: NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0,
    pub Reserved0: [u8; 3],
    pub LSPActivationCount: u8,
    pub TPRevertCount: u8,
    pub LSPRevertCount: u8,
    pub LOCount: u8,
    pub SUMLOCount: u8,
    pub RPLOCount: u8,
    pub NPLOCount: u8,
    pub RLLOCount: u8,
    pub WLLOCount: u8,
    pub RULOCount: u8,
    pub WULOCount: u8,
    pub Reserved1: u8,
    pub SIDAuthTryCount: u32,
    pub SIDAuthTryLimit: u32,
    pub ResetCount: u32,
    pub ResetLockCount: u32,
    pub Reserved2: [u8; 462],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0 {
    pub _bitfield: u8,
}
impl NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0 {
    pub fn CPINSIDValue(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_CPINSIDValue(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn CPINSIDBlocked(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_CPINSIDBlocked(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn LockingEnabled(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_LockingEnabled(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn SUMOwner(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_SUMOwner(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
pub const NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_VERSION_1: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_TCG_HISTORY_LOG {
    pub LID: u8,
    pub Reserved0: [u8; 3],
    pub HistoryEntryCount: u32,
    pub HistoryEntries: [TCG_HISTORY_ENTRY; 84],
    pub Reserved1: [u8; 38],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_TCG_HISTORY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_TCG_HISTORY_LOG_VERSION_1: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {
    pub UnsupportedCount: u16,
    pub Reserved0: [u8; 14],
    pub UnsupportedReqList: [UNSUPPORTED_REQUIREMENT; 253],
    pub Reserved1: [u8; 14],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG_VERSION_1: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    pub EventType: u8,
    pub EventTypeRevision: u8,
    pub EventHeaderLength: u8,
    pub Reserved0: u8,
    pub ControllerIdentifier: u16,
    pub EventTimestamp: u64,
    pub Reserved1: [u8; 6],
    pub VendorSpecificInformationLength: u16,
    pub EventLength: u16,
}
impl Default for NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_HEADER {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 3],
    pub TotalNumberOfEvents: u32,
    pub TotalLogLength: u64,
    pub LogRevision: u8,
    pub Reserved1: u8,
    pub LogHeaderLength: u16,
    pub Timestamp: u64,
    pub PowerOnHours: [u8; 16],
    pub PowerCycleCount: u64,
    pub PciVendorId: u16,
    pub PciSubsystemVendorId: u16,
    pub SerialNumber: [u8; 20],
    pub ModelNumber: [u8; 40],
    pub NVMSubsystemNVMeQualifiedName: [u8; 256],
    pub Reserved: [u8; 108],
    pub SupportedEventsBitmap: [u8; 32],
}
impl Default for NVME_PERSISTENT_EVENT_LOG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_PERSISTENT_EVENT_TYPE_CHANGE_NAMESPACE: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 6;
pub const NVME_PERSISTENT_EVENT_TYPE_FIRMWARE_COMMIT: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 2;
pub const NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_COMPLETION: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 8;
pub const NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_START: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 7;
pub const NVME_PERSISTENT_EVENT_TYPE_MAX: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 255;
pub const NVME_PERSISTENT_EVENT_TYPE_NVM_SUBSYSTEM_HARDWARE_ERROR: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 5;
pub const NVME_PERSISTENT_EVENT_TYPE_POWER_ON_OR_RESET: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 4;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED0: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 0;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED1_BEGIN: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 14;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED1_END: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 221;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED2_BEGIN: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 224;
pub const NVME_PERSISTENT_EVENT_TYPE_RESERVED2_END: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 255;
pub const NVME_PERSISTENT_EVENT_TYPE_SANITIZE_COMPLETION: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 10;
pub const NVME_PERSISTENT_EVENT_TYPE_SANITIZE_START: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 9;
pub const NVME_PERSISTENT_EVENT_TYPE_SET_FEATURE: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 11;
pub const NVME_PERSISTENT_EVENT_TYPE_SMART_HEALTH_LOG_SNAPSHOT: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 1;
pub const NVME_PERSISTENT_EVENT_TYPE_TCG_DEFINED: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 223;
pub const NVME_PERSISTENT_EVENT_TYPE_TELEMETRY_LOG_CREATED: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 12;
pub const NVME_PERSISTENT_EVENT_TYPE_THERMAL_EXCURSION: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 13;
pub const NVME_PERSISTENT_EVENT_TYPE_TIMESTAMP_CHANGE: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 3;
pub const NVME_PERSISTENT_EVENT_TYPE_VENDOR_SPECIFIC_EVENT: NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = 222;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_POWER_STATE_DESC {
    pub MP: u16,
    pub Reserved0: u8,
    pub _bitfield1: u8,
    pub ENLAT: u32,
    pub EXLAT: u32,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
    pub _bitfield5: u8,
    pub IDLP: u16,
    pub _bitfield6: u8,
    pub Reserved7: u8,
    pub ACTP: u16,
    pub _bitfield7: u8,
    pub Reserved9: [u8; 9],
}
impl NVME_POWER_STATE_DESC {
    pub fn MPS(&self) -> bool {
        self._bitfield1 & 1 != 0
    }
    pub fn set_MPS(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !1) | (value as u8);
    }
    pub fn NOPS(&self) -> bool {
        (self._bitfield1 >> 1) & 1 != 0
    }
    pub fn set_NOPS(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Reserved1(&self) -> u8 {
        self._bitfield1 >> 2
    }
    pub fn set_Reserved1(&mut self, value: u8) {
        self._bitfield1 = (self._bitfield1 & !(63 << 2)) | ((value & 63) << 2);
    }
    pub fn RRT(&self) -> u8 {
        (self._bitfield2 << 3) >> 3
    }
    pub fn set_RRT(&mut self, value: u8) {
        self._bitfield2 = (self._bitfield2 & !31) | (value & 31);
    }
    pub fn Reserved2(&self) -> u8 {
        self._bitfield2 >> 5
    }
    pub fn set_Reserved2(&mut self, value: u8) {
        self._bitfield2 = (self._bitfield2 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn RRL(&self) -> u8 {
        (self._bitfield3 << 3) >> 3
    }
    pub fn set_RRL(&mut self, value: u8) {
        self._bitfield3 = (self._bitfield3 & !31) | (value & 31);
    }
    pub fn Reserved3(&self) -> u8 {
        self._bitfield3 >> 5
    }
    pub fn set_Reserved3(&mut self, value: u8) {
        self._bitfield3 = (self._bitfield3 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn RWT(&self) -> u8 {
        (self._bitfield4 << 3) >> 3
    }
    pub fn set_RWT(&mut self, value: u8) {
        self._bitfield4 = (self._bitfield4 & !31) | (value & 31);
    }
    pub fn Reserved4(&self) -> u8 {
        self._bitfield4 >> 5
    }
    pub fn set_Reserved4(&mut self, value: u8) {
        self._bitfield4 = (self._bitfield4 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn RWL(&self) -> u8 {
        (self._bitfield5 << 3) >> 3
    }
    pub fn set_RWL(&mut self, value: u8) {
        self._bitfield5 = (self._bitfield5 & !31) | (value & 31);
    }
    pub fn Reserved5(&self) -> u8 {
        self._bitfield5 >> 5
    }
    pub fn set_Reserved5(&mut self, value: u8) {
        self._bitfield5 = (self._bitfield5 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn Reserved6(&self) -> u8 {
        (self._bitfield6 << 2) >> 2
    }
    pub fn set_Reserved6(&mut self, value: u8) {
        self._bitfield6 = (self._bitfield6 & !63) | (value & 63);
    }
    pub fn IPS(&self) -> u8 {
        self._bitfield6 >> 6
    }
    pub fn set_IPS(&mut self, value: u8) {
        self._bitfield6 = (self._bitfield6 & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn APW(&self) -> u8 {
        (self._bitfield7 << 5) >> 5
    }
    pub fn set_APW(&mut self, value: u8) {
        self._bitfield7 = (self._bitfield7 & !7) | (value & 7);
    }
    pub fn Reserved8(&self) -> u8 {
        (self._bitfield7 << 2) >> 5
    }
    pub fn set_Reserved8(&mut self, value: u8) {
        self._bitfield7 = (self._bitfield7 & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn APS(&self) -> u8 {
        self._bitfield7 >> 6
    }
    pub fn set_APS(&mut self, value: u8) {
        self._bitfield7 = (self._bitfield7 & !(3 << 6)) | ((value & 3) << 6);
    }
}
impl Default for NVME_POWER_STATE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_PROPERTY_OFFSET = i32;
pub const NVME_PROTECTION_INFORMATION_NOT_ENABLED: NVME_PROTECTION_INFORMATION_TYPES = 0;
pub const NVME_PROTECTION_INFORMATION_TYPE1: NVME_PROTECTION_INFORMATION_TYPES = 1;
pub const NVME_PROTECTION_INFORMATION_TYPE2: NVME_PROTECTION_INFORMATION_TYPES = 2;
pub const NVME_PROTECTION_INFORMATION_TYPE3: NVME_PROTECTION_INFORMATION_TYPES = 3;
pub type NVME_PROTECTION_INFORMATION_TYPES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_PRP_ENTRY {
    pub Anonymous: NVME_PRP_ENTRY_0,
    pub AsUlonglong: u64,
}
impl Default for NVME_PRP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_PRP_ENTRY_0 {
    pub _bitfield: u64,
}
impl NVME_PRP_ENTRY_0 {
    pub fn Reserved0(&self) -> u64 {
        (self._bitfield << 62) >> 62
    }
    pub fn set_Reserved0(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn PBAO(&self) -> u64 {
        self._bitfield >> 2
    }
    pub fn set_PBAO(&mut self, value: u64) {
        self._bitfield = (self._bitfield & !(4611686018427387903 << 2)) | ((value & 4611686018427387903) << 2);
    }
}
pub const NVME_PSDT_XFER_PRP: u32 = 0;
pub const NVME_PSDT_XFER_RESERVED: u32 = 3;
pub const NVME_PSDT_XFER_SGL_BYTE: u32 = 1;
pub const NVME_PSDT_XFER_SGL_QWORD: u32 = 2;
pub type NVME_RDMA_KEYED_SGL_DESC_SUBTYPE = i32;
pub const NVME_READ_BEHAVIOR_NOT_REPORTED: NVME_DEALLOCATE_READ_BEHAVIOR = 0;
pub const NVME_READ_BEHAVIOR_RETURN_ONES: NVME_DEALLOCATE_READ_BEHAVIOR = 2;
pub const NVME_READ_BEHAVIOR_RETURN_ZERO: NVME_DEALLOCATE_READ_BEHAVIOR = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_DATA {
    pub CNTLID: u16,
    pub RCSTS: NVME_REGISTERED_CONTROLLER_DATA_0,
    pub Reserved: [u8; 5],
    pub HOSTID: [u8; 8],
    pub RKEY: u64,
}
impl Default for NVME_REGISTERED_CONTROLLER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_DATA_0 {
    pub _bitfield: u8,
}
impl NVME_REGISTERED_CONTROLLER_DATA_0 {
    pub fn HoldReservation(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_HoldReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    pub CNTLID: u16,
    pub RCSTS: NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0,
    pub Reserved: [u8; 5],
    pub RKEY: u64,
    pub HOSTID: [u8; 16],
    pub Reserved1: [u8; 32],
}
impl Default for NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0 {
    pub _bitfield: u8,
}
impl NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0 {
    pub fn HoldReservation(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_HoldReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_REPORT_ZONE_INFO {
    pub ZoneCount: u64,
    pub Reserved: [u64; 7],
    pub ZoneDescriptor: [NVME_ZONE_DESCRIPTOR; 1],
}
impl Default for NVME_REPORT_ZONE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_RESERVATION_ACQUIRE_ACTIONS = i32;
pub const NVME_RESERVATION_ACQUIRE_ACTION_ACQUIRE: NVME_RESERVATION_ACQUIRE_ACTIONS = 0;
pub const NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT: NVME_RESERVATION_ACQUIRE_ACTIONS = 1;
pub const NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT_AND_ABORT: NVME_RESERVATION_ACQUIRE_ACTIONS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE {
    pub CRKEY: u64,
    pub PRKEY: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_RESERVATION_NOTIFICATION_LOG {
    pub LogPageCount: u64,
    pub LogPageType: u8,
    pub AvailableLogPageCount: u8,
    pub Reserved0: [u8; 2],
    pub NameSpaceId: u32,
    pub Reserved1: [u8; 48],
}
impl Default for NVME_RESERVATION_NOTIFICATION_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_RESERVATION_NOTIFICATION_TYPES = i32;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_EMPTY_LOG_PAGE: NVME_RESERVATION_NOTIFICATION_TYPES = 0;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_PREEMPTED: NVME_RESERVATION_NOTIFICATION_TYPES = 1;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_RELEASED: NVME_RESERVATION_NOTIFICATION_TYPES = 2;
pub const NVME_RESERVATION_NOTIFICATION_TYPE_RESERVATION_PREEPMPTED: NVME_RESERVATION_NOTIFICATION_TYPES = 3;
pub type NVME_RESERVATION_REGISTER_ACTIONS = i32;
pub const NVME_RESERVATION_REGISTER_ACTION_REGISTER: NVME_RESERVATION_REGISTER_ACTIONS = 0;
pub const NVME_RESERVATION_REGISTER_ACTION_REPLACE: NVME_RESERVATION_REGISTER_ACTIONS = 2;
pub const NVME_RESERVATION_REGISTER_ACTION_UNREGISTER: NVME_RESERVATION_REGISTER_ACTIONS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_RESERVATION_REGISTER_DATA_STRUCTURE {
    pub CRKEY: u64,
    pub NRKEY: u64,
}
pub type NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = i32;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_NO_CHANGE: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 0;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_RESERVED: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 1;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_0: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 2;
pub const NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_1: NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES = 3;
pub type NVME_RESERVATION_RELEASE_ACTIONS = i32;
pub const NVME_RESERVATION_RELEASE_ACTION_CLEAR: NVME_RESERVATION_RELEASE_ACTIONS = 1;
pub const NVME_RESERVATION_RELEASE_ACTION_RELEASE: NVME_RESERVATION_RELEASE_ACTIONS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_RESERVATION_RELEASE_DATA_STRUCTURE {
    pub CRKEY: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub RegisteredControllersData: [NVME_REGISTERED_CONTROLLER_DATA; 1],
}
impl Default for NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub Reserved1: [u8; 40],
    pub RegisteredControllersExtendedData: [NVME_REGISTERED_CONTROLLER_EXTENDED_DATA; 1],
}
impl Default for NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_HEADER {
    pub GEN: u32,
    pub RTYPE: u8,
    pub REGCTL: u16,
    pub Reserved: [u8; 2],
    pub PTPLS: u8,
    pub Reserved1: [u8; 14],
}
impl Default for NVME_RESERVATION_REPORT_STATUS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_RESERVATION_TYPES = i32;
pub const NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS: NVME_RESERVATION_TYPES = 2;
pub const NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_ALL_REGISTRANTS: NVME_RESERVATION_TYPES = 6;
pub const NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_REGISTRANTS_ONLY: NVME_RESERVATION_TYPES = 4;
pub const NVME_RESERVATION_TYPE_RESERVED: NVME_RESERVATION_TYPES = 0;
pub const NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE: NVME_RESERVATION_TYPES = 1;
pub const NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_ALL_REGISTRANTS: NVME_RESERVATION_TYPES = 5;
pub const NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_REGISTRANTS_ONLY: NVME_RESERVATION_TYPES = 3;
pub type NVME_SANITIZE_ACTION = i32;
pub const NVME_SANITIZE_ACTION_EXIT_FAILURE_MODE: NVME_SANITIZE_ACTION = 1;
pub const NVME_SANITIZE_ACTION_RESERVED: NVME_SANITIZE_ACTION = 0;
pub const NVME_SANITIZE_ACTION_START_BLOCK_ERASE_SANITIZE: NVME_SANITIZE_ACTION = 2;
pub const NVME_SANITIZE_ACTION_START_CRYPTO_ERASE_SANITIZE: NVME_SANITIZE_ACTION = 4;
pub const NVME_SANITIZE_ACTION_START_OVERWRITE_SANITIZE: NVME_SANITIZE_ACTION = 3;
pub const NVME_SANITIZE_OPERATION_FAILED: NVME_SANITIZE_OPERATION_STATUS = 3;
pub const NVME_SANITIZE_OPERATION_IN_PROGRESS: NVME_SANITIZE_OPERATION_STATUS = 2;
pub const NVME_SANITIZE_OPERATION_NONE: NVME_SANITIZE_OPERATION_STATUS = 0;
pub type NVME_SANITIZE_OPERATION_STATUS = i32;
pub const NVME_SANITIZE_OPERATION_SUCCEEDED: NVME_SANITIZE_OPERATION_STATUS = 1;
pub const NVME_SANITIZE_OPERATION_SUCCEEDED_WITH_FORCED_DEALLOCATION: NVME_SANITIZE_OPERATION_STATUS = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SANITIZE_STATUS {
    pub _bitfield: u16,
}
impl NVME_SANITIZE_STATUS {
    pub fn MostRecentSanitizeOperationStatus(&self) -> u16 {
        (self._bitfield << 13) >> 13
    }
    pub fn set_MostRecentSanitizeOperationStatus(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn NumberCompletedPassesOfOverwrite(&self) -> u16 {
        (self._bitfield << 9) >> 12
    }
    pub fn set_NumberCompletedPassesOfOverwrite(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(15 << 3)) | ((value & 15) << 3);
    }
    pub fn GlobalDataErased(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_GlobalDataErased(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u16) << 7);
    }
    pub fn Reserved(&self) -> u16 {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SANITIZE_STATUS_LOG {
    pub SPROG: u16,
    pub SSTAT: NVME_SANITIZE_STATUS,
    pub SCDW10: u32,
    pub EstimatedTimeForOverwrite: u32,
    pub EstimatedTimeForBlockErase: u32,
    pub EstimatedTimeForCryptoErase: u32,
    pub EstimatedTimeForOverwriteWithNoDeallocateMediaModification: u32,
    pub EstimatedTimeForBlockEraseWithNoDeallocateMediaModification: u32,
    pub EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification: u32,
    pub Reserved: [u8; 480],
}
impl Default for NVME_SANITIZE_STATUS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SCSI_NAME_STRING {
    pub PCIVendorID: [i8; 4],
    pub ModelNumber: [i8; 40],
    pub NamespaceID: [i8; 4],
    pub SerialNumber: [i8; 20],
}
impl Default for NVME_SCSI_NAME_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_SECURE_ERASE_CRYPTOGRAPHIC: NVME_SECURE_ERASE_SETTINGS = 2;
pub const NVME_SECURE_ERASE_NONE: NVME_SECURE_ERASE_SETTINGS = 0;
pub type NVME_SECURE_ERASE_SETTINGS = i32;
pub const NVME_SECURE_ERASE_USER_DATA: NVME_SECURE_ERASE_SETTINGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SET_ATTRIBUTES_ENTRY {
    pub Identifier: u16,
    pub ENDGID: u16,
    pub Reserved1: u32,
    pub Random4KBReadTypical: u32,
    pub OptimalWriteSize: u32,
    pub TotalCapacity: [u8; 16],
    pub UnallocatedCapacity: [u8; 16],
    pub Reserved2: [u8; 80],
}
impl Default for NVME_SET_ATTRIBUTES_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_BITBUCKET_DESC {
    pub Reserved0: u64,
    pub Length: u32,
    pub Reserved1: [u8; 3],
    pub Identifier: NVME_SGL_BITBUCKET_DESC_0,
}
impl Default for NVME_SGL_BITBUCKET_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_BITBUCKET_DESC_0 {
    pub Anonymous: NVME_SGL_BITBUCKET_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_BITBUCKET_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_BITBUCKET_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_BITBUCKET_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_DATABLOCK_DESC {
    pub Address: u64,
    pub Length: u32,
    pub Reserved0: [u8; 3],
    pub Identifier: NVME_SGL_DATABLOCK_DESC_0,
}
impl Default for NVME_SGL_DATABLOCK_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_DATABLOCK_DESC_0 {
    pub Anonymous: NVME_SGL_DATABLOCK_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_DATABLOCK_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_DATABLOCK_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_DATABLOCK_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_DESC {
    pub Reserved0: [u8; 15],
    pub Identifier: NVME_SGL_DESC_0,
}
impl Default for NVME_SGL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_DESC_0 {
    pub Anonymous: NVME_SGL_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
pub type NVME_SGL_DESC_SUBTYPE = i32;
pub type NVME_SGL_DESC_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_KEYDATABLOCK_DESC {
    pub Address: u64,
    pub Length: [u8; 3],
    pub Key: [u8; 4],
    pub Identifier: NVME_SGL_KEYDATABLOCK_DESC_0,
}
impl Default for NVME_SGL_KEYDATABLOCK_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_KEYDATABLOCK_DESC_0 {
    pub Anonymous: NVME_SGL_KEYDATABLOCK_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_KEYDATABLOCK_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_KEYDATABLOCK_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_KEYDATABLOCK_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_LASTSEG_DESC {
    pub Address: u64,
    pub Length: u32,
    pub Reserved0: [u8; 3],
    pub Identifier: NVME_SGL_LASTSEG_DESC_0,
}
impl Default for NVME_SGL_LASTSEG_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_LASTSEG_DESC_0 {
    pub Anonymous: NVME_SGL_LASTSEG_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_LASTSEG_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_LASTSEG_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_LASTSEG_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_SEGMENT_DESC {
    pub Address: u64,
    pub Length: u32,
    pub Reserved0: [u8; 3],
    pub Identifier: NVME_SGL_SEGMENT_DESC_0,
}
impl Default for NVME_SGL_SEGMENT_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_SEGMENT_DESC_0 {
    pub Anonymous: NVME_SGL_SEGMENT_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_SEGMENT_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_SEGMENT_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_SEGMENT_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_SGL_TRANSPORTDATA_DESC {
    pub Reserved0: u64,
    pub Length: u32,
    pub Reserved1: [u8; 3],
    pub Identifier: NVME_SGL_TRANSPORTDATA_DESC_0,
}
impl Default for NVME_SGL_TRANSPORTDATA_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SGL_TRANSPORTDATA_DESC_0 {
    pub Anonymous: NVME_SGL_TRANSPORTDATA_DESC_0_0,
    pub AsUchar: u8,
}
impl Default for NVME_SGL_TRANSPORTDATA_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_TRANSPORTDATA_DESC_0_0 {
    pub _bitfield: u8,
}
impl NVME_SGL_TRANSPORTDATA_DESC_0_0 {
    pub fn SubType(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_SubType(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
pub const NVME_STATE_ZSC: ZONE_STATE = 4;
pub const NVME_STATE_ZSE: ZONE_STATE = 1;
pub const NVME_STATE_ZSEO: ZONE_STATE = 3;
pub const NVME_STATE_ZSF: ZONE_STATE = 14;
pub const NVME_STATE_ZSIO: ZONE_STATE = 2;
pub const NVME_STATE_ZSO: ZONE_STATE = 15;
pub const NVME_STATE_ZSRO: ZONE_STATE = 13;
pub const NVME_STATUS_ABORT_COMMAND_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 3;
pub const NVME_STATUS_ADMIN_COMMAND_MEDIA_NOT_READY: NVME_STATUS_GENERIC_COMMAND_CODES = 36;
pub const NVME_STATUS_ANA_ATTACH_FAILED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 37;
pub const NVME_STATUS_ASYMMETRIC_ACCESS_INACCESSIBLE: NVME_STATUS_PATH_ERROR_CODES = 2;
pub const NVME_STATUS_ASYMMETRIC_ACCESS_PERSISTENT_LOSS: NVME_STATUS_PATH_ERROR_CODES = 1;
pub const NVME_STATUS_ASYMMETRIC_ACCESS_TRANSITION: NVME_STATUS_PATH_ERROR_CODES = 3;
pub const NVME_STATUS_ASYNC_EVENT_REQUEST_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 5;
pub const NVME_STATUS_ATOMIC_WRITE_UNIT_EXCEEDED: NVME_STATUS_GENERIC_COMMAND_CODES = 20;
pub const NVME_STATUS_AUTHENTICATION_REQUIRED: NVME_STATUS_FABRIC_COMMAND_CODES = 145;
pub const NVME_STATUS_BOOT_PARTITION_WRITE_PROHIBITED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 30;
pub const NVME_STATUS_COMMAND_ABORTED_BY_HOST: NVME_STATUS_PATH_ERROR_CODES = 113;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_FUSED_COMMAND: NVME_STATUS_GENERIC_COMMAND_CODES = 9;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_MISSING_COMMAND: NVME_STATUS_GENERIC_COMMAND_CODES = 10;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_POWER_LOSS_NOTIFICATION: NVME_STATUS_GENERIC_COMMAND_CODES = 5;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_PREEMPT_ABORT: NVME_STATUS_GENERIC_COMMAND_CODES = 27;
pub const NVME_STATUS_COMMAND_ABORTED_DUE_TO_SQ_DELETION: NVME_STATUS_GENERIC_COMMAND_CODES = 8;
pub const NVME_STATUS_COMMAND_ABORT_REQUESTED: NVME_STATUS_GENERIC_COMMAND_CODES = 7;
pub const NVME_STATUS_COMMAND_ID_CONFLICT: NVME_STATUS_GENERIC_COMMAND_CODES = 3;
pub const NVME_STATUS_COMMAND_INTERRUPTED: NVME_STATUS_GENERIC_COMMAND_CODES = 33;
pub const NVME_STATUS_COMMAND_NOT_SUPPORTED_FOR_QUEUE_IN_CMB: NVME_STATUS_GENERIC_COMMAND_CODES = 31;
pub const NVME_STATUS_COMMAND_PROHIBITED_BY_LOCKDOWN: NVME_STATUS_GENERIC_COMMAND_CODES = 35;
pub const NVME_STATUS_COMMAND_SEQUENCE_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 12;
pub type NVME_STATUS_COMMAND_SPECIFIC_CODES = i32;
pub const NVME_STATUS_COMPLETION_QUEUE_INVALID: NVME_STATUS_COMMAND_SPECIFIC_CODES = 0;
pub const NVME_STATUS_CONNECT_INVALID_HOST: NVME_STATUS_FABRIC_COMMAND_CODES = 132;
pub const NVME_STATUS_CONNECT_INVALID_PARAMETERS: NVME_STATUS_FABRIC_COMMAND_CODES = 130;
pub const NVME_STATUS_CONNECT_RESTART_DISCOVERY: NVME_STATUS_FABRIC_COMMAND_CODES = 131;
pub const NVME_STATUS_CONTROLLER_BUSY: NVME_STATUS_FABRIC_COMMAND_CODES = 129;
pub const NVME_STATUS_CONTROLLER_LIST_INVALID: NVME_STATUS_COMMAND_SPECIFIC_CODES = 28;
pub const NVME_STATUS_CONTROLLER_PATHING_ERROR: NVME_STATUS_PATH_ERROR_CODES = 96;
pub const NVME_STATUS_DATA_SGL_LENGTH_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 15;
pub const NVME_STATUS_DATA_TRANSFER_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 4;
pub const NVME_STATUS_DEVICE_SELF_TEST_IN_PROGRESS: NVME_STATUS_COMMAND_SPECIFIC_CODES = 29;
pub const NVME_STATUS_DIRECTIVE_ID_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 113;
pub const NVME_STATUS_DIRECTIVE_TYPE_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 112;
pub const NVME_STATUS_DISCOVER_RESTART: NVME_STATUS_FABRIC_COMMAND_CODES = 144;
pub type NVME_STATUS_FABRIC_COMMAND_CODES = i32;
pub const NVME_STATUS_FEATURE_ID_NOT_SAVEABLE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 13;
pub const NVME_STATUS_FEATURE_NOT_CHANGEABLE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 14;
pub const NVME_STATUS_FEATURE_NOT_NAMESPACE_SPECIFIC: NVME_STATUS_COMMAND_SPECIFIC_CODES = 15;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_PROHIBITED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 19;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_CONVENTIONAL_RESET: NVME_STATUS_COMMAND_SPECIFIC_CODES = 11;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_MAX_TIME_VIOLATION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 18;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_NVM_SUBSYSTEM_RESET: NVME_STATUS_COMMAND_SPECIFIC_CODES = 16;
pub const NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_RESET: NVME_STATUS_COMMAND_SPECIFIC_CODES = 17;
pub const NVME_STATUS_FORMAT_IN_PROGRESS: NVME_STATUS_GENERIC_COMMAND_CODES = 132;
pub type NVME_STATUS_GENERIC_COMMAND_CODES = i32;
pub const NVME_STATUS_HOST_IDENTIFIER_INCONSISTENT_FORMAT: NVME_STATUS_GENERIC_COMMAND_CODES = 24;
pub const NVME_STATUS_HOST_PATHING_ERROR: NVME_STATUS_PATH_ERROR_CODES = 112;
pub const NVME_STATUS_INCOMPATIBLE_FORMAT: NVME_STATUS_FABRIC_COMMAND_CODES = 128;
pub const NVME_STATUS_INSUFFICIENT_CAPACITY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 38;
pub const NVME_STATUS_INSUFFICIENT_DISCOVERY_RESOURCES: NVME_STATUS_COMMAND_SPECIFIC_CODES = 50;
pub const NVME_STATUS_INTERNAL_DEVICE_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 6;
pub const NVME_STATUS_INTERNAL_PATH_ERROR: NVME_STATUS_PATH_ERROR_CODES = 0;
pub const NVME_STATUS_INVALID_ANA_GROUP_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 36;
pub const NVME_STATUS_INVALID_COMMAND_OPCODE: NVME_STATUS_GENERIC_COMMAND_CODES = 1;
pub const NVME_STATUS_INVALID_CONTROLLER_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 31;
pub const NVME_STATUS_INVALID_DISCOVERY_INFORMATION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 47;
pub const NVME_STATUS_INVALID_FIELD_IN_COMMAND: NVME_STATUS_GENERIC_COMMAND_CODES = 2;
pub const NVME_STATUS_INVALID_FIRMWARE_IMAGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 7;
pub const NVME_STATUS_INVALID_FIRMWARE_SLOT: NVME_STATUS_COMMAND_SPECIFIC_CODES = 6;
pub const NVME_STATUS_INVALID_FORMAT: NVME_STATUS_COMMAND_SPECIFIC_CODES = 10;
pub const NVME_STATUS_INVALID_INTERRUPT_VECTOR: NVME_STATUS_COMMAND_SPECIFIC_CODES = 8;
pub const NVME_STATUS_INVALID_LOG_PAGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 9;
pub const NVME_STATUS_INVALID_NAMESPACE_OR_FORMAT: NVME_STATUS_GENERIC_COMMAND_CODES = 11;
pub const NVME_STATUS_INVALID_NUMBER_OF_CONTROLLER_RESOURCES: NVME_STATUS_COMMAND_SPECIFIC_CODES = 33;
pub const NVME_STATUS_INVALID_NUMBER_OF_SGL_DESCR: NVME_STATUS_GENERIC_COMMAND_CODES = 14;
pub const NVME_STATUS_INVALID_QUEUE_DELETION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 12;
pub const NVME_STATUS_INVALID_QUEUE_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 1;
pub const NVME_STATUS_INVALID_QUEUE_TYPE: NVME_STATUS_FABRIC_COMMAND_CODES = 133;
pub const NVME_STATUS_INVALID_RESOURCE_IDENTIFIER: NVME_STATUS_COMMAND_SPECIFIC_CODES = 34;
pub const NVME_STATUS_INVALID_SECONDARY_CONTROLLER_STATE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 32;
pub const NVME_STATUS_INVALID_SGL_LAST_SEGMENT_DESCR: NVME_STATUS_GENERIC_COMMAND_CODES = 13;
pub const NVME_STATUS_INVALID_USE_OF_CONTROLLER_MEMORY_BUFFER: NVME_STATUS_GENERIC_COMMAND_CODES = 18;
pub const NVME_STATUS_KEEP_ALIVE_TIMEOUT_EXPIRED: NVME_STATUS_GENERIC_COMMAND_CODES = 25;
pub const NVME_STATUS_KEEP_ALIVE_TIMEOUT_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 26;
pub const NVME_STATUS_MAX_QUEUE_SIZE_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 2;
pub type NVME_STATUS_MEDIA_ERROR_CODES = i32;
pub const NVME_STATUS_METADATA_SGL_LENGTH_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 16;
pub const NVME_STATUS_NAMESPACE_ALREADY_ATTACHED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 24;
pub const NVME_STATUS_NAMESPACE_ATTACHMENT_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 39;
pub const NVME_STATUS_NAMESPACE_IDENTIFIER_UNAVAILABLE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 22;
pub const NVME_STATUS_NAMESPACE_INSUFFICIENT_CAPACITY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 21;
pub const NVME_STATUS_NAMESPACE_IS_PRIVATE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 25;
pub const NVME_STATUS_NAMESPACE_IS_WRITE_PROTECTED: NVME_STATUS_GENERIC_COMMAND_CODES = 32;
pub const NVME_STATUS_NAMESPACE_NOT_ATTACHED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 26;
pub const NVME_STATUS_NAMESPACE_THIN_PROVISIONING_NOT_SUPPORTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 27;
pub const NVME_STATUS_NVM_ACCESS_DENIED: NVME_STATUS_MEDIA_ERROR_CODES = 134;
pub const NVME_STATUS_NVM_ATTEMPTED_WRITE_TO_READ_ONLY_RANGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 130;
pub const NVME_STATUS_NVM_CAPACITY_EXCEEDED: NVME_STATUS_GENERIC_COMMAND_CODES = 129;
pub const NVME_STATUS_NVM_COMMAND_SIZE_LIMIT_EXCEEDED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 131;
pub const NVME_STATUS_NVM_COMPARE_FAILURE: NVME_STATUS_MEDIA_ERROR_CODES = 133;
pub const NVME_STATUS_NVM_CONFLICTING_ATTRIBUTES: NVME_STATUS_COMMAND_SPECIFIC_CODES = 128;
pub const NVME_STATUS_NVM_DEALLOCATED_OR_UNWRITTEN_LOGICAL_BLOCK: NVME_STATUS_MEDIA_ERROR_CODES = 135;
pub const NVME_STATUS_NVM_END_TO_END_APPLICATION_TAG_CHECK_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 131;
pub const NVME_STATUS_NVM_END_TO_END_GUARD_CHECK_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 130;
pub const NVME_STATUS_NVM_END_TO_END_REFERENCE_TAG_CHECK_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 132;
pub const NVME_STATUS_NVM_INVALID_PROTECTION_INFORMATION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 129;
pub const NVME_STATUS_NVM_LBA_OUT_OF_RANGE: NVME_STATUS_GENERIC_COMMAND_CODES = 128;
pub const NVME_STATUS_NVM_NAMESPACE_NOT_READY: NVME_STATUS_GENERIC_COMMAND_CODES = 130;
pub const NVME_STATUS_NVM_RESERVATION_CONFLICT: NVME_STATUS_GENERIC_COMMAND_CODES = 131;
pub const NVME_STATUS_NVM_UNRECOVERED_READ_ERROR: NVME_STATUS_MEDIA_ERROR_CODES = 129;
pub const NVME_STATUS_NVM_WRITE_FAULT: NVME_STATUS_MEDIA_ERROR_CODES = 128;
pub const NVME_STATUS_OPERATION_DENIED: NVME_STATUS_GENERIC_COMMAND_CODES = 21;
pub const NVME_STATUS_OVERLAPPING_RANGE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 20;
pub type NVME_STATUS_PATH_ERROR_CODES = i32;
pub const NVME_STATUS_PROHIBITION_NOT_SUPPORTED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 40;
pub const NVME_STATUS_PRP_OFFSET_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 19;
pub const NVME_STATUS_RESERVED: NVME_STATUS_GENERIC_COMMAND_CODES = 23;
pub const NVME_STATUS_SANITIZE_FAILED: NVME_STATUS_GENERIC_COMMAND_CODES = 28;
pub const NVME_STATUS_SANITIZE_IN_PROGRESS: NVME_STATUS_GENERIC_COMMAND_CODES = 29;
pub const NVME_STATUS_SANITIZE_PROHIBITED_ON_PERSISTENT_MEMORY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 35;
pub const NVME_STATUS_SGL_DATA_BLOCK_GRANULARITY_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 30;
pub const NVME_STATUS_SGL_DESCR_TYPE_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 17;
pub const NVME_STATUS_SGL_OFFSET_INVALID: NVME_STATUS_GENERIC_COMMAND_CODES = 22;
pub const NVME_STATUS_STREAM_RESOURCE_ALLOCATION_FAILED: NVME_STATUS_COMMAND_SPECIFIC_CODES = 127;
pub const NVME_STATUS_SUCCESS_COMPLETION: NVME_STATUS_GENERIC_COMMAND_CODES = 0;
pub const NVME_STATUS_TRANSIENT_TRANSPORT_ERROR: NVME_STATUS_GENERIC_COMMAND_CODES = 34;
pub type NVME_STATUS_TYPES = i32;
pub const NVME_STATUS_TYPE_COMMAND_SPECIFIC: NVME_STATUS_TYPES = 1;
pub const NVME_STATUS_TYPE_GENERIC_COMMAND: NVME_STATUS_TYPES = 0;
pub const NVME_STATUS_TYPE_MEDIA_ERROR: NVME_STATUS_TYPES = 2;
pub const NVME_STATUS_TYPE_PATH_RELATED: NVME_STATUS_TYPES = 3;
pub const NVME_STATUS_TYPE_VENDOR_SPECIFIC: NVME_STATUS_TYPES = 7;
pub const NVME_STATUS_ZONE_BOUNDARY_ERROR: NVME_STATUS_COMMAND_SPECIFIC_CODES = 184;
pub const NVME_STATUS_ZONE_FULL: NVME_STATUS_COMMAND_SPECIFIC_CODES = 185;
pub const NVME_STATUS_ZONE_INVALID_FORMAT: NVME_STATUS_COMMAND_SPECIFIC_CODES = 127;
pub const NVME_STATUS_ZONE_INVALID_STATE_TRANSITION: NVME_STATUS_COMMAND_SPECIFIC_CODES = 191;
pub const NVME_STATUS_ZONE_INVALID_WRITE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 188;
pub const NVME_STATUS_ZONE_OFFLINE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 187;
pub const NVME_STATUS_ZONE_READ_ONLY: NVME_STATUS_COMMAND_SPECIFIC_CODES = 186;
pub const NVME_STATUS_ZONE_TOO_MANY_ACTIVE: NVME_STATUS_COMMAND_SPECIFIC_CODES = 189;
pub const NVME_STATUS_ZONE_TOO_MANY_OPEN: NVME_STATUS_COMMAND_SPECIFIC_CODES = 190;
pub const NVME_STREAMS_GET_STATUS_MAX_IDS: u32 = 65535;
pub const NVME_STREAMS_ID_MAX: u32 = 65535;
pub const NVME_STREAMS_ID_MIN: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    pub Anonymous: NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0,
    pub AsUlong: u32,
}
impl Default for NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0 {
    pub _bitfield: u32,
}
impl NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0 {
    pub fn SQT(&self) -> u32 {
        (self._bitfield << 16) >> 16
    }
    pub fn set_SQT(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved0(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SUPPORTED_LOG_PAGES_LOG {
    pub LogPageIdentifierSupported: [NVME_LID_SUPPORTED_AND_EFFECTS; 256],
}
impl Default for NVME_SUPPORTED_LOG_PAGES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 4],
    pub OrganizationID: [u8; 3],
    pub Area1LastBlock: u16,
    pub Area2LastBlock: u16,
    pub Area3LastBlock: u16,
    pub Reserved1: [u8; 2],
    pub Area4LastBlock: u32,
    pub Reserved2: [u8; 362],
    pub ControllerInitiatedDataAvailable: u8,
    pub ControllerInitiatedDataGenerationNumber: u8,
    pub ReasonIdentifier: [u8; 128],
}
impl Default for NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_TELEMETRY_DATA_BLOCK_SIZE: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_TELEMETRY_HOST_INITIATED_LOG {
    pub LogIdentifier: u8,
    pub Reserved0: [u8; 4],
    pub OrganizationID: [u8; 3],
    pub Area1LastBlock: u16,
    pub Area2LastBlock: u16,
    pub Area3LastBlock: u16,
    pub Reserved1: [u8; 2],
    pub Area4LastBlock: u32,
    pub Reserved2: [u8; 361],
    pub HostInitiatedDataGenerationNumber: u8,
    pub ControllerInitiatedDataAvailable: u8,
    pub ControllerInitiatedDataGenerationNumber: u8,
    pub ReasonIdentifier: [u8; 128],
}
impl Default for NVME_TELEMETRY_HOST_INITIATED_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_TEMPERATURE_OVER_THRESHOLD: NVME_TEMPERATURE_THRESHOLD_TYPES = 0;
pub type NVME_TEMPERATURE_THRESHOLD_TYPES = i32;
pub const NVME_TEMPERATURE_UNDER_THRESHOLD: NVME_TEMPERATURE_THRESHOLD_TYPES = 1;
pub const NVME_UUID_ASSOCIATION_NONE: u32 = 0;
pub const NVME_UUID_ASSOCIATION_PCI_SUBSYSTEM_VID: u32 = 2;
pub const NVME_UUID_ASSOCIATION_PCI_VID: u32 = 1;
pub const NVME_UUID_ASSOCIATION_RESERVED: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_UUID_LIST {
    pub UUID: [NVME_UUID_LIST_ENTRY; 128],
}
impl Default for NVME_UUID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_UUID_LIST_ENTRY {
    pub _bitfield: u8,
    pub Reserved1: [u8; 15],
    pub UUID: [u8; 16],
}
impl NVME_UUID_LIST_ENTRY {
    pub fn IdentifierAssociation(&self) -> u8 {
        (self._bitfield << 6) >> 6
    }
    pub fn set_IdentifierAssociation(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
impl Default for NVME_UUID_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_VENDOR_LOG_PAGES = i32;
pub const NVME_VENDOR_SPECIFIC_ADMIN_COMMAND_MAX_OPCODE: u32 = 255;
pub const NVME_VENDOR_SPECIFIC_ADMIN_COMMAND_MIN_OPCODE: u32 = 192;
pub const NVME_VENDOR_SPECIFIC_FEATURE_MAX_IDENTIFIER: u32 = 255;
pub const NVME_VENDOR_SPECIFIC_FEATURE_MIN_IDENTIFIER: u32 = 192;
pub const NVME_VENDOR_SPECIFIC_LOG_PAGE_MAX_IDENTIFIER: u32 = 255;
pub const NVME_VENDOR_SPECIFIC_LOG_PAGE_MIN_IDENTIFIER: u32 = 192;
pub const NVME_VENDOR_SPECIFIC_NVM_COMMAND_MAX_OPCODE: u32 = 255;
pub const NVME_VENDOR_SPECIFIC_NVM_COMMAND_MIN_OPCODE: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_VERSION {
    pub Anonymous: NVME_VERSION_0,
    pub AsUlong: u32,
}
impl Default for NVME_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_VERSION_0 {
    pub _bitfield: u32,
}
impl NVME_VERSION_0 {
    pub fn TER(&self) -> u32 {
        (self._bitfield << 24) >> 24
    }
    pub fn set_TER(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn MNR(&self) -> u32 {
        (self._bitfield << 16) >> 24
    }
    pub fn set_MNR(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn MJR(&self) -> u32 {
        self._bitfield >> 16
    }
    pub fn set_MJR(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_CAPABILITIES {
    pub Anonymous: NVME_WCS_DEVICE_CAPABILITIES_0,
}
impl Default for NVME_WCS_DEVICE_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_WCS_DEVICE_CAPABILITIES_0 {
    pub Anonymous: NVME_WCS_DEVICE_CAPABILITIES_0_0,
    pub AsULONG: u32,
}
impl Default for NVME_WCS_DEVICE_CAPABILITIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_CAPABILITIES_0_0 {
    pub _bitfield: u32,
}
impl NVME_WCS_DEVICE_CAPABILITIES_0_0 {
    pub fn PanicAEN(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PanicAEN(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u32);
    }
    pub fn PanicCFS(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_PanicCFS(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u32) << 1);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    pub PanicResetWaitTime: u16,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DriveRecoveryAction: u8,
    pub PanicId: u64,
    pub DeviceCapabilitiesA: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: u8,
    pub Reserved0: [u8; 3],
    pub VendorSpecificCommandCDW12: u32,
    pub VendorSpecificCommandCDW13: u32,
    pub Reserved1: [u8; 466],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1 = NVME_WCS_DEVICE_ERROR_RECOVERY_LOG;
pub const NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_VERSION_1: u32 = 1;
pub type NVME_WCS_DEVICE_RECOVERY_ACTION1 = i32;
pub type NVME_WCS_DEVICE_RECOVERY_ACTION2 = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_RESET_ACTION {
    pub Anonymous: NVME_WCS_DEVICE_RESET_ACTION_0,
}
impl Default for NVME_WCS_DEVICE_RESET_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVME_WCS_DEVICE_RESET_ACTION_0 {
    pub Anonymous: NVME_WCS_DEVICE_RESET_ACTION_0_0,
    pub AsUCHAR: u8,
}
impl Default for NVME_WCS_DEVICE_RESET_ACTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_RESET_ACTION_0_0 {
    pub _bitfield: u8,
}
impl NVME_WCS_DEVICE_RESET_ACTION_0_0 {
    pub fn ControllerReset(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ControllerReset(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn NVMeSubsystemReset(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_NVMeSubsystemReset(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn PCIeFLR(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_PCIeFLR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn PERST(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_PERST(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn PowerCycle(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_PowerCycle(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn PCIeConventionalHotReset(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_PCIeConventionalHotReset(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u8) << 5);
    }
    pub fn Reserved(&self) -> u8 {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    pub VersionSpecificData: [u8; 494],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    pub MediaUnitsWritten: [u8; 16],
    pub MediaUnitsRead: [u8; 16],
    pub BadUserNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0,
    pub BadSystemNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1,
    pub XORRecoveryCount: u64,
    pub UnrecoverableReadErrorCount: u64,
    pub SoftECCErrorCount: u64,
    pub EndToEndCorrectionCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_2,
    pub PercentageSystemDataUsed: u8,
    pub RefreshCount: [u8; 7],
    pub UserDataEraseCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_3,
    pub ThermalThrottling: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_4,
    pub Reserved0: [u8; 6],
    pub PCIeCorrectableErrorCount: u64,
    pub IncompleteShutdownCount: u32,
    pub Reserved1: u32,
    pub PercentageFreeBlocks: u8,
    pub Reserved2: [u8; 7],
    pub CapacitorHealth: u16,
    pub Reserved3: [u8; 6],
    pub UnalignedIOCount: u64,
    pub SecurityVersionNumber: u64,
    pub NUSE: u64,
    pub PLPStartCount: [u8; 16],
    pub EnduranceEstimate: [u8; 16],
    pub Reserved4: [u8; 302],
    pub LogPageVersionNumber: u16,
    pub LogPageGUID: windows_core::GUID,
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1 {
    pub RawCount: [u8; 6],
    pub Normalized: [u8; 2],
}
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_2 {
    pub DetectedCounts: u32,
    pub CorrectedCounts: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_3 {
    pub MaximumCount: u32,
    pub MinimumCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_4 {
    pub EventCount: u8,
    pub Status: u8,
}
pub const NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_VERSION_2: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR {
    pub Anonymous: NVME_ZONE_DESCRIPTOR_0,
    pub Anonymous2: NVME_ZONE_DESCRIPTOR_1,
    pub ZA: NVME_ZONE_DESCRIPTOR_2,
    pub Reserved3: [u8; 5],
    pub ZCAP: u64,
    pub ZSLBA: u64,
    pub WritePointer: u64,
    pub Reserved4: [u8; 32],
}
impl Default for NVME_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_0 {
    pub _bitfield: u8,
}
impl NVME_ZONE_DESCRIPTOR_0 {
    pub fn ZT(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_ZT(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved1(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_Reserved1(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_1 {
    pub _bitfield: u8,
}
impl NVME_ZONE_DESCRIPTOR_1 {
    pub fn Reserved2(&self) -> u8 {
        (self._bitfield << 4) >> 4
    }
    pub fn set_Reserved2(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn ZS(&self) -> u8 {
        self._bitfield >> 4
    }
    pub fn set_ZS(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_2 {
    pub _bitfield: u8,
}
impl NVME_ZONE_DESCRIPTOR_2 {
    pub fn ZFC(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_ZFC(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn FZR(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_FZR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn RZR(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_RZR(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Reserved(&self) -> u8 {
        (self._bitfield << 1) >> 4
    }
    pub fn set_Reserved(&mut self, value: u8) {
        self._bitfield = (self._bitfield & !(15 << 3)) | ((value & 15) << 3);
    }
    pub fn ZDEV(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_ZDEV(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_EXTENSION {
    pub ZoneDescriptorExtensionInfo: [u8; 64],
}
impl Default for NVME_ZONE_DESCRIPTOR_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ZONE_EXTENDED_REPORT_ZONE_DESC {
    pub ZoneDescriptor: NVME_ZONE_DESCRIPTOR,
    pub ZoneDescriptorExtension: [NVME_ZONE_DESCRIPTOR_EXTENSION; 1],
}
impl Default for NVME_ZONE_EXTENDED_REPORT_ZONE_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_ZONE_RECEIVE_ACTION = i32;
pub type NVME_ZONE_RECEIVE_ACTION_SPECIFIC = i32;
pub const NVME_ZONE_RECEIVE_EXTENDED_REPORT_ZONES: NVME_ZONE_RECEIVE_ACTION = 1;
pub const NVME_ZONE_RECEIVE_REPORT_ZONES: NVME_ZONE_RECEIVE_ACTION = 0;
pub type NVME_ZONE_SEND_ACTION = i32;
pub const NVME_ZONE_SEND_CLOSE: NVME_ZONE_SEND_ACTION = 1;
pub const NVME_ZONE_SEND_FINISH: NVME_ZONE_SEND_ACTION = 2;
pub const NVME_ZONE_SEND_OFFLINE: NVME_ZONE_SEND_ACTION = 5;
pub const NVME_ZONE_SEND_OPEN: NVME_ZONE_SEND_ACTION = 3;
pub const NVME_ZONE_SEND_RESET: NVME_ZONE_SEND_ACTION = 4;
pub const NVME_ZONE_SEND_SET_ZONE_DESCRIPTOR: NVME_ZONE_SEND_ACTION = 16;
pub const NVME_ZRA_ALL_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 0;
pub const NVME_ZRA_CLOSED_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 4;
pub const NVME_ZRA_EMPTY_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 1;
pub const NVME_ZRA_EO_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 3;
pub const NVME_ZRA_FULL_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 5;
pub const NVME_ZRA_IO_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 2;
pub const NVME_ZRA_OFFLINE_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 7;
pub const NVME_ZRA_RO_STATE_ZONES: NVME_ZONE_RECEIVE_ACTION_SPECIFIC = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub union NVM_RESERVATION_CAPABILITIES {
    pub Anonymous: NVM_RESERVATION_CAPABILITIES_0,
    pub AsUchar: u8,
}
impl Default for NVM_RESERVATION_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVM_RESERVATION_CAPABILITIES_0 {
    pub _bitfield: u8,
}
impl NVM_RESERVATION_CAPABILITIES_0 {
    pub fn PersistThroughPowerLoss(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_PersistThroughPowerLoss(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u8);
    }
    pub fn WriteExclusiveReservation(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_WriteExclusiveReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn ExclusiveAccessReservation(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_ExclusiveAccessReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn WriteExclusiveRegistrantsOnlyReservation(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_WriteExclusiveRegistrantsOnlyReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u8) << 3);
    }
    pub fn ExclusiveAccessRegistrantsOnlyReservation(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_ExclusiveAccessRegistrantsOnlyReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn WriteExclusiveAllRegistrantsReservation(&self) -> bool {
        (self._bitfield >> 5) & 1 != 0
    }
    pub fn set_WriteExclusiveAllRegistrantsReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value as u8) << 5);
    }
    pub fn ExclusiveAccessAllRegistrantsReservation(&self) -> bool {
        (self._bitfield >> 6) & 1 != 0
    }
    pub fn set_ExclusiveAccessAllRegistrantsReservation(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value as u8) << 6);
    }
    pub fn Reserved(&self) -> bool {
        (self._bitfield >> 7) & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVM_SET_LIST {
    pub IdentifierCount: u8,
    pub Reserved: [u8; 127],
    pub Entry: [NVME_SET_ATTRIBUTES_ENTRY; 1],
}
impl Default for NVM_SET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVM_SUBSYSTEM_SHUTDOWN_ABRUPT: u32 = 1096970356;
pub const NVM_SUBSYSTEM_SHUTDOWN_NORMAL: u32 = 1316121964;
pub const NVMeDeviceRecovery1Max: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 15;
pub const NVMeDeviceRecovery2Max: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 15;
pub const NVMeDeviceRecoveryControllerReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 0;
pub const NVMeDeviceRecoveryDeviceReplacement: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 4;
pub const NVMeDeviceRecoveryFormatNVM: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 1;
pub const NVMeDeviceRecoveryNoAction: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 0;
pub const NVMeDeviceRecoveryPERST: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 3;
pub const NVMeDeviceRecoveryPcieFunctionReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 2;
pub const NVMeDeviceRecoveryPcieHotReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 5;
pub const NVMeDeviceRecoveryPowerCycle: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 4;
pub const NVMeDeviceRecoverySanitize: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 5;
pub const NVMeDeviceRecoverySubsystemReset: NVME_WCS_DEVICE_RECOVERY_ACTION2 = 1;
pub const NVMeDeviceRecoveryVendorAnalysis: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 3;
pub const NVMeDeviceRecoveryVendorSpecificCommand: NVME_WCS_DEVICE_RECOVERY_ACTION1 = 2;
pub const NvmeCtrlAdmin: NVME_CONTROLLER_TYPE = 3;
pub const NvmeCtrlDiscovery: NVME_CONTROLLER_TYPE = 2;
pub const NvmeCtrlIO: NVME_CONTROLLER_TYPE = 1;
pub const NvmeCtrlNotReported: NVME_CONTROLLER_TYPE = 0;
pub const NvmeCtrlReservedMax: NVME_CONTROLLER_TYPE = 255;
pub const NvmeCtrlReservedMin: NVME_CONTROLLER_TYPE = 4;
pub const NvmeDiscCtrlTypeCDC: NVME_DISC_CTRL_TYPE = 2;
pub const NvmeDiscCtrlTypeDDC: NVME_DISC_CTRL_TYPE = 1;
pub const NvmeDiscCtrlTypeReserved1: NVME_DISC_CTRL_TYPE = 3;
pub const NvmeDiscCtrlTypeReservedMax: NVME_DISC_CTRL_TYPE = 255;
pub const NvmeDiscCtrlTypeUnspecified: NVME_DISC_CTRL_TYPE = 0;
pub const NvmeExtAttrAdminLabelAscii: NVME_EXTENDED_ATTR_TYPE = 2;
pub const NvmeExtAttrAdminLabelUtf8: NVME_EXTENDED_ATTR_TYPE = 3;
pub const NvmeExtAttrHostId: NVME_EXTENDED_ATTR_TYPE = 1;
pub const NvmeExtAttrReserved0: NVME_EXTENDED_ATTR_TYPE = 0;
pub const NvmeExtAttrReservedEnd: NVME_EXTENDED_ATTR_TYPE = 65279;
pub const NvmeExtAttrReservedStart: NVME_EXTENDED_ATTR_TYPE = 4;
pub const NvmeExtAttrVendorEnd: NVME_EXTENDED_ATTR_TYPE = 65535;
pub const NvmeExtAttrVendorStart: NVME_EXTENDED_ATTR_TYPE = 65280;
pub const NvmePropACQ: NVME_PROPERTY_OFFSET = 48;
pub const NvmePropAQA: NVME_PROPERTY_OFFSET = 36;
pub const NvmePropASQ: NVME_PROPERTY_OFFSET = 40;
pub const NvmePropBPINFO: NVME_PROPERTY_OFFSET = 64;
pub const NvmePropBPMBL: NVME_PROPERTY_OFFSET = 72;
pub const NvmePropBPRSEL: NVME_PROPERTY_OFFSET = 68;
pub const NvmePropCAP: NVME_PROPERTY_OFFSET = 0;
pub const NvmePropCC: NVME_PROPERTY_OFFSET = 20;
pub const NvmePropCMBEBS: NVME_PROPERTY_OFFSET = 92;
pub const NvmePropCMBLOC: NVME_PROPERTY_OFFSET = 56;
pub const NvmePropCMBMSC: NVME_PROPERTY_OFFSET = 80;
pub const NvmePropCMBSTS: NVME_PROPERTY_OFFSET = 88;
pub const NvmePropCMBSWTP: NVME_PROPERTY_OFFSET = 96;
pub const NvmePropCMBSZ: NVME_PROPERTY_OFFSET = 60;
pub const NvmePropCRTO: NVME_PROPERTY_OFFSET = 104;
pub const NvmePropCSTS: NVME_PROPERTY_OFFSET = 28;
pub const NvmePropINTMC: NVME_PROPERTY_OFFSET = 16;
pub const NvmePropINTMS: NVME_PROPERTY_OFFSET = 12;
pub const NvmePropNSSD: NVME_PROPERTY_OFFSET = 100;
pub const NvmePropNSSR: NVME_PROPERTY_OFFSET = 32;
pub const NvmePropPMRCAP: NVME_PROPERTY_OFFSET = 3584;
pub const NvmePropPMRCTL: NVME_PROPERTY_OFFSET = 3588;
pub const NvmePropPMREBS: NVME_PROPERTY_OFFSET = 3596;
pub const NvmePropPMRMSCL: NVME_PROPERTY_OFFSET = 3604;
pub const NvmePropPMRMSCU: NVME_PROPERTY_OFFSET = 3608;
pub const NvmePropPMRSTS: NVME_PROPERTY_OFFSET = 3592;
pub const NvmePropPMRSWTP: NVME_PROPERTY_OFFSET = 3600;
pub const NvmePropVS: NVME_PROPERTY_OFFSET = 8;
pub const NvmeRdmaKeyedSglDescSubtypeInvalidate: NVME_RDMA_KEYED_SGL_DESC_SUBTYPE = 15;
pub const NvmeSglDescSubtypeAddress: NVME_SGL_DESC_SUBTYPE = 0;
pub const NvmeSglDescSubtypeOffset: NVME_SGL_DESC_SUBTYPE = 1;
pub const NvmeSglDescSubtypeTransportA: NVME_SGL_DESC_SUBTYPE = 10;
pub const NvmeSglDescSubtypeTransportB: NVME_SGL_DESC_SUBTYPE = 11;
pub const NvmeSglDescSubtypeTransportC: NVME_SGL_DESC_SUBTYPE = 12;
pub const NvmeSglDescSubtypeTransportD: NVME_SGL_DESC_SUBTYPE = 13;
pub const NvmeSglDescSubtypeTransportE: NVME_SGL_DESC_SUBTYPE = 14;
pub const NvmeSglDescSubtypeTransportF: NVME_SGL_DESC_SUBTYPE = 15;
pub const NvmeSglDescTypeBitBucket: NVME_SGL_DESC_TYPE = 1;
pub const NvmeSglDescTypeDataBlock: NVME_SGL_DESC_TYPE = 0;
pub const NvmeSglDescTypeKeyedDataBlock: NVME_SGL_DESC_TYPE = 4;
pub const NvmeSglDescTypeLastSegment: NVME_SGL_DESC_TYPE = 3;
pub const NvmeSglDescTypeMax: NVME_SGL_DESC_TYPE = 15;
pub const NvmeSglDescTypeSegment: NVME_SGL_DESC_TYPE = 2;
pub const NvmeSglDescTypeTransportDataBlock: NVME_SGL_DESC_TYPE = 5;
pub const NvmeofAddressFC: NVMEOF_ADDRESS_FAMILY = 4;
pub const NvmeofAddressIB: NVMEOF_ADDRESS_FAMILY = 3;
pub const NvmeofAddressIPv4: NVMEOF_ADDRESS_FAMILY = 1;
pub const NvmeofAddressIPv6: NVMEOF_ADDRESS_FAMILY = 2;
pub const NvmeofAddressLoopback: NVMEOF_ADDRESS_FAMILY = 254;
pub const NvmeofAddressMax: NVMEOF_ADDRESS_FAMILY = 255;
pub const NvmeofAddressUnknown: NVMEOF_ADDRESS_FAMILY = 0;
pub const NvmeofAuthDHCHAPGroup2048: NVMEOF_AUTH_DHCHAP_GROUP_ID = 1;
pub const NvmeofAuthDHCHAPGroup3072: NVMEOF_AUTH_DHCHAP_GROUP_ID = 2;
pub const NvmeofAuthDHCHAPGroup4096: NVMEOF_AUTH_DHCHAP_GROUP_ID = 3;
pub const NvmeofAuthDHCHAPGroup6144: NVMEOF_AUTH_DHCHAP_GROUP_ID = 4;
pub const NvmeofAuthDHCHAPGroup8192: NVMEOF_AUTH_DHCHAP_GROUP_ID = 5;
pub const NvmeofAuthDHCHAPGroupMax: NVMEOF_AUTH_DHCHAP_GROUP_ID = 255;
pub const NvmeofAuthDHCHAPGroupNull: NVMEOF_AUTH_DHCHAP_GROUP_ID = 0;
pub const NvmeofAuthDHCHAPHashMax: NVMEOF_AUTH_DHCHAP_HASH_ID = 255;
pub const NvmeofAuthDHCHAPHashReserved: NVMEOF_AUTH_DHCHAP_HASH_ID = 0;
pub const NvmeofAuthDHCHAPHashSha256: NVMEOF_AUTH_DHCHAP_HASH_ID = 1;
pub const NvmeofAuthDHCHAPHashSha384: NVMEOF_AUTH_DHCHAP_HASH_ID = 2;
pub const NvmeofAuthDHCHAPHashSha512: NVMEOF_AUTH_DHCHAP_HASH_ID = 3;
pub const NvmeofAuthDHGroupNotUsable: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 5;
pub const NvmeofAuthFailed: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 1;
pub const NvmeofAuthFailureReasonFailed: NVMEOF_AUTH_FAIL_REASON_CODE = 1;
pub const NvmeofAuthHashFunctionNotUsable: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 4;
pub const NvmeofAuthIdChallenge: NVMEOF_AUTH_ID = 1;
pub const NvmeofAuthIdFailure1: NVMEOF_AUTH_ID = 241;
pub const NvmeofAuthIdFailure2: NVMEOF_AUTH_ID = 240;
pub const NvmeofAuthIdNegotiate: NVMEOF_AUTH_ID = 0;
pub const NvmeofAuthIdReply: NVMEOF_AUTH_ID = 2;
pub const NvmeofAuthIdSuccess1: NVMEOF_AUTH_ID = 3;
pub const NvmeofAuthIdSuccess2: NVMEOF_AUTH_ID = 4;
pub const NvmeofAuthIncorrectPayload: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 6;
pub const NvmeofAuthIncorrectProtocolMessage: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 7;
pub const NvmeofAuthProtocolDHCHAP: NVMEOF_AUTH_PROTOCOL = 233;
pub const NvmeofAuthProtocolNotUsable: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 2;
pub const NvmeofAuthSCAuthConcatSCRequired: NVMEOF_AUTH_SECURE_CHANNEL = 2;
pub const NvmeofAuthSCAuthRequired: NVMEOF_AUTH_SECURE_CHANNEL = 1;
pub const NvmeofAuthSCReserved: NVMEOF_AUTH_SECURE_CHANNEL = 3;
pub const NvmeofAuthSCUnspecified: NVMEOF_AUTH_SECURE_CHANNEL = 0;
pub const NvmeofAuthSecureChannelConcatMismatch: NVMEOF_AUTH_FAIL_REASON_EXPLANATION = 3;
pub const NvmeofAuthTypeCommonMessages: NVMEOF_AUTH_TYPE = 0;
pub const NvmeofAuthTypeDHCHAPMessages: NVMEOF_AUTH_TYPE = 1;
pub const NvmeofSCNotRequired: NVMEOF_SECURE_CHANNEL = 2;
pub const NvmeofSCRequired: NVMEOF_SECURE_CHANNEL = 1;
pub const NvmeofSCReserved: NVMEOF_SECURE_CHANNEL = 3;
pub const NvmeofSCUnspecified: NVMEOF_SECURE_CHANNEL = 0;
pub const NvmeofSecureChannelConcatNone: NVMEOF_SECURE_CHANNEL_PROTOCOL = 0;
pub const NvmeofSecureChannelConcatWithTLS: NVMEOF_SECURE_CHANNEL_PROTOCOL = 1;
pub const NvmeofSecureChannelNewTLSPSK: NVMEOF_SECURE_CHANNEL_PROTOCOL = 2;
pub const NvmeofSecureChannelReplaceTLSPSK: NVMEOF_SECURE_CHANNEL_PROTOCOL = 2;
pub const NvmeofSubsysTypeDiscCurrent: NVMEOF_SUBSYSTEM_TYPE = 3;
pub const NvmeofSubsysTypeDiscReferral: NVMEOF_SUBSYSTEM_TYPE = 1;
pub const NvmeofSubsysTypeIo: NVMEOF_SUBSYSTEM_TYPE = 2;
pub const NvmeofSubsysTypeMax: NVMEOF_SUBSYSTEM_TYPE = 255;
pub const NvmeofSubsysTypeUnknown: NVMEOF_SUBSYSTEM_TYPE = 0;
pub const NvmeofTransportFC: NVMEOF_TRANSPORT_TYPE = 2;
pub const NvmeofTransportLoopback: NVMEOF_TRANSPORT_TYPE = 254;
pub const NvmeofTransportMax: NVMEOF_TRANSPORT_TYPE = 255;
pub const NvmeofTransportRdma: NVMEOF_TRANSPORT_TYPE = 1;
pub const NvmeofTransportTcp: NVMEOF_TRANSPORT_TYPE = 3;
pub const NvmeofTransportUnknown: NVMEOF_TRANSPORT_TYPE = 0;
pub type PACTIVE_LATENCY_CONFIGURATION = *mut ACTIVE_LATENCY_CONFIGURATION;
pub type PBUCKET_COUNTER = *mut BUCKET_COUNTER;
pub type PDEBUG_BIT_FIELD = *mut DEBUG_BIT_FIELD;
pub type PDSSD_POWER_STATE_DESCRIPTOR = *mut DSSD_POWER_STATE_DESCRIPTOR;
pub type PFIRMWARE_ACTIVATION_HISTORY_ENTRY = *mut FIRMWARE_ACTIVATION_HISTORY_ENTRY;
pub type PIO_COMMAND_SET_VECTOR = *mut IO_COMMAND_SET_VECTOR;
pub type PLATENCY_MONITOR_FEATURE_STATUS = *mut LATENCY_MONITOR_FEATURE_STATUS;
pub type PLATENCY_STAMP = *mut LATENCY_STAMP;
pub type PLATENCY_STAMP_UNITS = *mut LATENCY_STAMP_UNITS;
pub type PMEASURED_LATENCY = *mut MEASURED_LATENCY;
pub type PNVMEOF_AUTH_DHCHAP_CHALLENGE = *mut NVMEOF_AUTH_DHCHAP_CHALLENGE;
pub type PNVMEOF_AUTH_DHCHAP_DESCRIPTOR = *mut NVMEOF_AUTH_DHCHAP_DESCRIPTOR;
pub type PNVMEOF_AUTH_DHCHAP_REPLY = *mut NVMEOF_AUTH_DHCHAP_REPLY;
pub type PNVMEOF_AUTH_DHCHAP_SUCCESS1 = *mut NVMEOF_AUTH_DHCHAP_SUCCESS1;
pub type PNVMEOF_AUTH_DHCHAP_SUCCESS2 = *mut NVMEOF_AUTH_DHCHAP_SUCCESS2;
pub type PNVMEOF_AUTH_FAILURE = *mut NVMEOF_AUTH_FAILURE;
pub type PNVMEOF_AUTH_NEGOTIATE = *mut NVMEOF_AUTH_NEGOTIATE;
pub type PNVMEOF_AUTH_RECEIVE_COMMAND = *mut NVMEOF_AUTH_RECEIVE_COMMAND;
pub type PNVMEOF_AUTH_RECEIVE_RESPONSE = *mut NVMEOF_AUTH_RECEIVE_RESPONSE;
pub type PNVMEOF_AUTH_SEND_COMMAND = *mut NVMEOF_AUTH_SEND_COMMAND;
pub type PNVMEOF_AUTH_SEND_RESPONSE = *mut NVMEOF_AUTH_SEND_RESPONSE;
pub type PNVMEOF_CONNECT_COMMAND = *mut NVMEOF_CONNECT_COMMAND;
pub type PNVMEOF_CONNECT_DATA = *mut NVMEOF_CONNECT_DATA;
pub type PNVMEOF_CONNECT_RESPONSE = *mut NVMEOF_CONNECT_RESPONSE;
pub type PNVMEOF_DISCONNECT_COMMAND = *mut NVMEOF_DISCONNECT_COMMAND;
pub type PNVMEOF_DISCONNECT_RESPONSE = *mut NVMEOF_DISCONNECT_RESPONSE;
pub type PNVMEOF_FABRICS_COMMAND = *mut NVMEOF_FABRICS_COMMAND;
pub type PNVMEOF_FABRICS_RESPONSE = *mut NVMEOF_FABRICS_RESPONSE;
pub type PNVMEOF_PROPERTY_GET_COMMAND = *mut NVMEOF_PROPERTY_GET_COMMAND;
pub type PNVMEOF_PROPERTY_GET_RESPONSE = *mut NVMEOF_PROPERTY_GET_RESPONSE;
pub type PNVMEOF_PROPERTY_SET_COMMAND = *mut NVMEOF_PROPERTY_SET_COMMAND;
pub type PNVMEOF_PROPERTY_SET_RESPONSE = *mut NVMEOF_PROPERTY_SET_RESPONSE;
pub type PNVME_ACTIVE_NAMESPACE_ID_LIST = *mut NVME_ACTIVE_NAMESPACE_ID_LIST;
pub type PNVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS = *mut NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS;
pub type PNVME_ADMIN_QUEUE_ATTRIBUTES = *mut NVME_ADMIN_QUEUE_ATTRIBUTES;
pub type PNVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS = *mut NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS;
pub type PNVME_AUTO_POWER_STATE_TRANSITION_ENTRY = *mut NVME_AUTO_POWER_STATE_TRANSITION_ENTRY;
pub type PNVME_BOOT_PARTITION_INFORMATION = *mut NVME_BOOT_PARTITION_INFORMATION;
pub type PNVME_BOOT_PARTITION_LOG = *mut NVME_BOOT_PARTITION_LOG;
pub type PNVME_CDW0_FEATURE_DSSD_POWER_STATE = *mut NVME_CDW0_FEATURE_DSSD_POWER_STATE;
pub type PNVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO = *mut NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO;
pub type PNVME_CDW0_FEATURE_ERROR_INJECTION = *mut NVME_CDW11_FEATURE_ERROR_INJECTION;
pub type PNVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE = *mut NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE;
pub type PNVME_CDW0_RESERVATION_PERSISTENCE = *mut NVME_CDW0_RESERVATION_PERSISTENCE;
pub type PNVME_CDW10_ABORT = *mut NVME_CDW10_ABORT;
pub type PNVME_CDW10_CREATE_IO_QUEUE = *mut NVME_CDW10_CREATE_IO_QUEUE;
pub type PNVME_CDW10_DATASET_MANAGEMENT = *mut NVME_CDW10_DATASET_MANAGEMENT;
pub type PNVME_CDW10_DELETE_IO_QUEUE = *mut NVME_CDW10_DELETE_IO_QUEUE;
pub type PNVME_CDW10_DEVICE_SELF_TEST = *mut NVME_CDW10_DEVICE_SELF_TEST;
pub type PNVME_CDW10_DIRECTIVE_RECEIVE = *mut NVME_CDW10_DIRECTIVE_RECEIVE;
pub type PNVME_CDW10_DIRECTIVE_SEND = *mut NVME_CDW10_DIRECTIVE_SEND;
pub type PNVME_CDW10_DISCOVERY_INFO_MGMT = *mut NVME_CDW10_DISCOVERY_INFO_MGMT;
pub type PNVME_CDW10_FIRMWARE_ACTIVATE = *mut NVME_CDW10_FIRMWARE_ACTIVATE;
pub type PNVME_CDW10_FIRMWARE_DOWNLOAD = *mut NVME_CDW10_FIRMWARE_DOWNLOAD;
pub type PNVME_CDW10_FORMAT_NVM = *mut NVME_CDW10_FORMAT_NVM;
pub type PNVME_CDW10_GET_FEATURES = *mut NVME_CDW10_GET_FEATURES;
pub type PNVME_CDW10_GET_LOG_PAGE = *mut NVME_CDW10_GET_LOG_PAGE;
pub type PNVME_CDW10_GET_LOG_PAGE_V121 = *mut NVME_CDW10_GET_LOG_PAGE_V121;
pub type PNVME_CDW10_GET_LOG_PAGE_V13 = *mut NVME_CDW10_GET_LOG_PAGE_V13;
pub type PNVME_CDW10_GET_LOG_PAGE_V20 = *mut NVME_CDW10_GET_LOG_PAGE_V20;
pub type PNVME_CDW10_IDENTIFY = *mut NVME_CDW10_IDENTIFY;
pub type PNVME_CDW10_RESERVATION_ACQUIRE = *mut NVME_CDW10_RESERVATION_ACQUIRE;
pub type PNVME_CDW10_RESERVATION_REGISTER = *mut NVME_CDW10_RESERVATION_REGISTER;
pub type PNVME_CDW10_RESERVATION_RELEASE = *mut NVME_CDW10_RESERVATION_RELEASE;
pub type PNVME_CDW10_RESERVATION_REPORT = *mut NVME_CDW10_RESERVATION_REPORT;
pub type PNVME_CDW10_SANITIZE = *mut NVME_CDW10_SANITIZE;
pub type PNVME_CDW10_SECURITY_SEND_RECEIVE = *mut NVME_CDW10_SECURITY_SEND_RECEIVE;
pub type PNVME_CDW10_SET_FEATURES = *mut NVME_CDW10_SET_FEATURES;
pub type PNVME_CDW10_ZONE_APPEND = *mut NVME_CDW10_ZONE_APPEND;
pub type PNVME_CDW10_ZONE_MANAGEMENT_RECEIVE = *mut NVME_CDW10_ZONE_MANAGEMENT_RECEIVE;
pub type PNVME_CDW10_ZONE_MANAGEMENT_SEND = *mut NVME_CDW10_ZONE_MANAGEMENT_SEND;
pub type PNVME_CDW11_CREATE_IO_CQ = *mut NVME_CDW11_CREATE_IO_CQ;
pub type PNVME_CDW11_CREATE_IO_SQ = *mut NVME_CDW11_CREATE_IO_SQ;
pub type PNVME_CDW11_DATASET_MANAGEMENT = *mut NVME_CDW11_DATASET_MANAGEMENT;
pub type PNVME_CDW11_DIRECTIVE_RECEIVE = *mut NVME_CDW11_DIRECTIVE_RECEIVE;
pub type PNVME_CDW11_DIRECTIVE_SEND = *mut NVME_CDW11_DIRECTIVE_SEND;
pub type PNVME_CDW11_FEATURES = *mut NVME_CDW11_FEATURES;
pub type PNVME_CDW11_FEATURE_ARBITRATION = *mut NVME_CDW11_FEATURE_ARBITRATION;
pub type PNVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG = *mut NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG;
pub type PNVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION = *mut NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION;
pub type PNVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY = *mut NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY;
pub type PNVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS = *mut NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS;
pub type PNVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO = *mut NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO;
pub type PNVME_CDW11_FEATURE_ERROR_INJECTION = *mut NVME_CDW11_FEATURE_ERROR_INJECTION;
pub type PNVME_CDW11_FEATURE_ERROR_RECOVERY = *mut NVME_CDW11_FEATURE_ERROR_RECOVERY;
pub type PNVME_CDW11_FEATURE_GET_HOST_METADATA = *mut NVME_CDW11_FEATURE_GET_HOST_METADATA;
pub type PNVME_CDW11_FEATURE_HOST_IDENTIFIER = *mut NVME_CDW11_FEATURE_HOST_IDENTIFIER;
pub type PNVME_CDW11_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER;
pub type PNVME_CDW11_FEATURE_INTERRUPT_COALESCING = *mut NVME_CDW11_FEATURE_INTERRUPT_COALESCING;
pub type PNVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG = *mut NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG;
pub type PNVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE = *mut NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE;
pub type PNVME_CDW11_FEATURE_LBA_RANGE_TYPE = *mut NVME_CDW11_FEATURE_LBA_RANGE_TYPE;
pub type PNVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE = *mut NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE;
pub type PNVME_CDW11_FEATURE_NUMBER_OF_QUEUES = *mut NVME_CDW11_FEATURE_NUMBER_OF_QUEUES;
pub type PNVME_CDW11_FEATURE_POWER_MANAGEMENT = *mut NVME_CDW11_FEATURE_POWER_MANAGEMENT;
pub type PNVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE = *mut NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE;
pub type PNVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK = *mut NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK;
pub type PNVME_CDW11_FEATURE_RESERVATION_PERSISTENCE = *mut NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE;
pub type PNVME_CDW11_FEATURE_SET_HOST_METADATA = *mut NVME_CDW11_FEATURE_SET_HOST_METADATA;
pub type PNVME_CDW11_FEATURE_SUPPORTED_CAPABILITY = *mut NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY;
pub type PNVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD = *mut NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD;
pub type PNVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE = *mut NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE;
pub type PNVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL = *mut NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL;
pub type PNVME_CDW11_FIRMWARE_DOWNLOAD = *mut NVME_CDW11_FIRMWARE_DOWNLOAD;
pub type PNVME_CDW11_GET_LOG_PAGE = *mut NVME_CDW11_GET_LOG_PAGE;
pub type PNVME_CDW11_IDENTIFY = *mut NVME_CDW11_IDENTIFY;
pub type PNVME_CDW11_RESERVATION_REPORT = *mut NVME_CDW11_RESERVATION_REPORT;
pub type PNVME_CDW11_SECURITY_RECEIVE = *mut NVME_CDW11_SECURITY_RECEIVE;
pub type PNVME_CDW11_SECURITY_SEND = *mut NVME_CDW11_SECURITY_SEND;
pub type PNVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES = *mut NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES;
pub type PNVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE = *mut NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE;
pub type PNVME_CDW12_FEATURES = *mut NVME_CDW12_FEATURES;
pub type PNVME_CDW12_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER;
pub type PNVME_CDW12_GET_LOG_PAGE = *mut NVME_CDW12_GET_LOG_PAGE;
pub type PNVME_CDW12_READ_WRITE = *mut NVME_CDW12_READ_WRITE;
pub type PNVME_CDW12_VERIFYCOMMAND = *mut NVME_CDW12_VERIFYCOMMAND;
pub type PNVME_CDW12_ZONE_APPEND = *mut NVME_CDW12_ZONE_APPEND;
pub type PNVME_CDW13_FEATURES = *mut NVME_CDW13_FEATURES;
pub type PNVME_CDW13_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER;
pub type PNVME_CDW13_GET_LOG_PAGE = *mut NVME_CDW13_GET_LOG_PAGE;
pub type PNVME_CDW13_READ_WRITE = *mut NVME_CDW13_READ_WRITE;
pub type PNVME_CDW13_ZONE_MANAGEMENT_RECEIVE = *mut NVME_CDW13_ZONE_MANAGEMENT_RECEIVE;
pub type PNVME_CDW13_ZONE_MANAGEMENT_SEND = *mut NVME_CDW13_ZONE_MANAGEMENT_SEND;
pub type PNVME_CDW14_FEATURES = *mut NVME_CDW14_FEATURES;
pub type PNVME_CDW14_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER;
pub type PNVME_CDW14_GET_LOG_PAGE = *mut NVME_CDW14_GET_LOG_PAGE;
pub type PNVME_CDW14_GET_LOG_PAGE_V20 = *mut NVME_CDW14_GET_LOG_PAGE_V20;
pub type PNVME_CDW14_IDENTIFY = *mut NVME_CDW14_IDENTIFY;
pub type PNVME_CDW15_FEATURES = *mut NVME_CDW15_FEATURES;
pub type PNVME_CDW15_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER;
pub type PNVME_CDW15_READ_WRITE = *mut NVME_CDW15_READ_WRITE;
pub type PNVME_CDW15_VERIFY_COMMAND = *mut NVME_CDW15_VERIFY_COMMAND;
pub type PNVME_CDW15_ZONE_APPEND = *mut NVME_CDW15_ZONE_APPEND;
pub type PNVME_CHANGED_NAMESPACE_LIST_LOG = *mut NVME_CHANGED_NAMESPACE_LIST_LOG;
pub type PNVME_CHANGED_ZONE_LIST_LOG = *mut NVME_CHANGED_ZONE_LIST_LOG;
pub type PNVME_COMMAND = *mut NVME_COMMAND;
pub type PNVME_COMMAND_DWORD0 = *mut NVME_COMMAND_DWORD0;
pub type PNVME_COMMAND_EFFECTS_DATA = *mut NVME_COMMAND_EFFECTS_DATA;
pub type PNVME_COMMAND_EFFECTS_LOG = *mut NVME_COMMAND_EFFECTS_LOG;
pub type PNVME_COMMAND_STATUS = *mut NVME_COMMAND_STATUS;
pub type PNVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST = *mut NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST;
pub type PNVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES = *mut NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES;
pub type PNVME_COMPLETION_ENTRY = *mut NVME_COMPLETION_ENTRY;
pub type PNVME_COMPLETION_QUEUE_HEAD_DOORBELL = *mut NVME_COMPLETION_QUEUE_HEAD_DOORBELL;
pub type PNVME_CONTEXT_ATTRIBUTES = *mut NVME_CONTEXT_ATTRIBUTES;
pub type PNVME_CONTROLLER_CAPABILITIES = *mut NVME_CONTROLLER_CAPABILITIES;
pub type PNVME_CONTROLLER_CONFIGURATION = *mut NVME_CONTROLLER_CONFIGURATION;
pub type PNVME_CONTROLLER_LIST = *mut NVME_CONTROLLER_LIST;
pub type PNVME_CONTROLLER_MEMORY_BUFFER_LOCATION = *mut NVME_CONTROLLER_MEMORY_BUFFER_LOCATION;
pub type PNVME_CONTROLLER_MEMORY_BUFFER_SIZE = *mut NVME_CONTROLLER_MEMORY_BUFFER_SIZE;
pub type PNVME_CONTROLLER_READY_TIMEOUTS = *mut NVME_CONTROLLER_READY_TIMEOUTS;
pub type PNVME_CONTROLLER_REGISTERS = *mut NVME_CONTROLLER_REGISTERS;
pub type PNVME_CONTROLLER_STATUS = *mut NVME_CONTROLLER_STATUS;
pub type PNVME_DEVICE_SELF_TEST_LOG = *mut NVME_DEVICE_SELF_TEST_LOG;
pub type PNVME_DEVICE_SELF_TEST_RESULT_DATA = *mut NVME_DEVICE_SELF_TEST_RESULT_DATA;
pub type PNVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS = *mut NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS;
pub type PNVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR = *mut NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR;
pub type PNVME_DIRECTIVE_STREAMS_GET_STATUS_DATA = *mut NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA;
pub type PNVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS = *mut NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS;
pub type PNVME_DISCOVERY_ENTRY = *mut NVME_DISCOVERY_ENTRY;
pub type PNVME_DISCOVERY_HEADER = *mut NVME_DISCOVERY_HEADER;
pub type PNVME_DISCOVERY_INFO_MGMT_HEADER = *mut NVME_DISCOVERY_INFO_MGMT_HEADER;
pub type PNVME_ENDURANCE_GROUP_LOG = *mut NVME_ENDURANCE_GROUP_LOG;
pub type PNVME_ERROR_INFO_LOG = *mut NVME_ERROR_INFO_LOG;
pub type PNVME_ERROR_INJECTION_ENTRY = *mut NVME_ERROR_INJECTION_ENTRY;
pub type PNVME_EXTENDED_ATTR = *mut NVME_EXTENDED_ATTR;
pub type PNVME_EXTENDED_DISCOVERY_ENTRY = *mut NVME_EXTENDED_DISCOVERY_ENTRY;
pub type PNVME_EXTENDED_REPORT_ZONE_INFO = *mut NVME_EXTENDED_REPORT_ZONE_INFO;
pub type PNVME_FEATURE_HOST_IDENTIFIER_DATA = *mut NVME_FEATURE_HOST_IDENTIFIER_DATA;
pub type PNVME_FEATURE_HOST_METADATA_DATA = *mut NVME_FEATURE_HOST_METADATA_DATA;
pub type PNVME_FEATURE_IDENTIFIERS_EFFECTS_LOG = *mut NVME_FEATURE_IDENTIFIERS_EFFECTS_LOG;
pub type PNVME_FID_SUPPORTED_AND_EFFECTS = *mut NVME_FID_SUPPORTED_AND_EFFECTS;
pub type PNVME_FIRMWARE_SLOT_INFO_LOG = *mut NVME_FIRMWARE_SLOT_INFO_LOG;
pub type PNVME_GET_FEATURE_TIMESTAMP = *mut NVME_GET_FEATURE_TIMESTAMP;
pub type PNVME_HEALTH_INFO_LOG = *mut NVME_HEALTH_INFO_LOG;
pub type PNVME_HOST_BEHAVIOR_SUPPORT_DATA = *mut NVME_HOST_BEHAVIOR_SUPPORT_DATA;
pub type PNVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY = *mut NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY;
pub type PNVME_HOST_METADATA_ELEMENT_DESCRIPTOR = *mut NVME_HOST_METADATA_ELEMENT_DESCRIPTOR;
pub type PNVME_IDENTIFY_CONTROLLER_DATA = *mut NVME_IDENTIFY_CONTROLLER_DATA;
pub type PNVME_IDENTIFY_IO_COMMAND_SET = *mut NVME_IDENTIFY_IO_COMMAND_SET;
pub type PNVME_IDENTIFY_NAMESPACE_DATA = *mut NVME_IDENTIFY_NAMESPACE_DATA;
pub type PNVME_IDENTIFY_NAMESPACE_DESCRIPTOR = *mut NVME_IDENTIFY_NAMESPACE_DESCRIPTOR;
pub type PNVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET = *mut NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET;
pub type PNVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET = *mut NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET;
pub type PNVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET = *mut NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET;
pub type PNVME_LATENCY_MONITORING_ENTRY = *mut NVME_LATENCY_MONITORING_ENTRY;
pub type PNVME_LBA_FORMAT = *mut NVME_LBA_FORMAT;
pub type PNVME_LBA_RANGE = *mut NVME_LBA_RANGE;
pub type PNVME_LBA_RANGET_TYPE_ENTRY = *mut NVME_LBA_RANGET_TYPE_ENTRY;
pub type PNVME_LBA_ZONE_FORMAT = *mut NVME_LBA_ZONE_FORMAT;
pub type PNVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG = *mut NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG;
pub type PNVME_LID_SUPPORTED_AND_EFFECTS = *mut NVME_LID_SUPPORTED_AND_EFFECTS;
pub type PNVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = *mut NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE;
pub type PNVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS = *mut NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS;
pub type PNVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG = *mut NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG;
pub type PNVME_NVM_SUBSYSTEM_RESET = *mut NVME_NVM_SUBSYSTEM_RESET;
pub type PNVME_NVM_SUBSYSTEM_SHUTDOWN = *mut NVME_NVM_SUBSYSTEM_SHUTDOWN;
pub type PNVME_OCP_DEVICE_CAPABILITIES_LOG = *mut NVME_OCP_DEVICE_CAPABILITIES_LOG;
pub type PNVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 = *mut NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2;
pub type PNVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG = *mut NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG;
pub type PNVME_OCP_DEVICE_LATENCY_MONITOR_LOG = *mut NVME_OCP_DEVICE_LATENCY_MONITOR_LOG;
pub type PNVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 = *mut NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3;
pub type PNVME_OCP_DEVICE_TCG_CONFIGURATION_LOG = *mut NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG;
pub type PNVME_OCP_DEVICE_TCG_HISTORY_LOG = *mut NVME_OCP_DEVICE_TCG_HISTORY_LOG;
pub type PNVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG = *mut NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG;
pub type PNVME_PERSISTENT_EVENT_LOG_EVENT_HEADER = *mut NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER;
pub type PNVME_PERSISTENT_EVENT_LOG_HEADER = *mut NVME_PERSISTENT_EVENT_LOG_HEADER;
pub type PNVME_POWER_STATE_DESC = *mut NVME_POWER_STATE_DESC;
pub type PNVME_PRP_ENTRY = *mut NVME_PRP_ENTRY;
pub type PNVME_REGISTERED_CONTROLLER_DATA = *mut NVME_REGISTERED_CONTROLLER_DATA;
pub type PNVME_REGISTERED_CONTROLLER_EXTENDED_DATA = *mut NVME_REGISTERED_CONTROLLER_EXTENDED_DATA;
pub type PNVME_REPORT_ZONE_INFO = *mut NVME_REPORT_ZONE_INFO;
pub type PNVME_RESERVATION_ACQUIRE_DATA_STRUCTURE = *mut NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE;
pub type PNVME_RESERVATION_CAPABILITIES = *mut NVM_RESERVATION_CAPABILITIES;
pub type PNVME_RESERVATION_NOTIFICATION_LOG = *mut NVME_RESERVATION_NOTIFICATION_LOG;
pub type PNVME_RESERVATION_REGISTER_DATA_STRUCTURE = *mut NVME_RESERVATION_REGISTER_DATA_STRUCTURE;
pub type PNVME_RESERVATION_RELEASE_DATA_STRUCTURE = *mut NVME_RESERVATION_RELEASE_DATA_STRUCTURE;
pub type PNVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE = *mut NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE;
pub type PNVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE = *mut NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE;
pub type PNVME_RESERVATION_REPORT_STATUS_HEADER = *mut NVME_RESERVATION_REPORT_STATUS_HEADER;
pub type PNVME_SANITIZE_ACTION = *mut NVME_SANITIZE_ACTION;
pub type PNVME_SANITIZE_OPERATION_STATUS = *mut NVME_SANITIZE_OPERATION_STATUS;
pub type PNVME_SANITIZE_STATUS = *mut NVME_SANITIZE_STATUS;
pub type PNVME_SANITIZE_STATUS_LOG = *mut NVME_SANITIZE_STATUS_LOG;
pub type PNVME_SCSI_NAME_STRING = *mut NVME_SCSI_NAME_STRING;
pub type PNVME_SET_ATTRIBUTES_ENTRY = *mut NVME_SET_ATTRIBUTES_ENTRY;
pub type PNVME_SGL_BITBUCKET_DESC = *mut NVME_SGL_BITBUCKET_DESC;
pub type PNVME_SGL_DATABLOCK_DESC = *mut NVME_SGL_DATABLOCK_DESC;
pub type PNVME_SGL_DESC = *mut NVME_SGL_DESC;
pub type PNVME_SGL_KEYDATABLOCK_DESC = *mut NVME_SGL_KEYDATABLOCK_DESC;
pub type PNVME_SGL_LASTSEG_DESC = *mut NVME_SGL_LASTSEG_DESC;
pub type PNVME_SGL_SEGMENT_DESC = *mut NVME_SGL_SEGMENT_DESC;
pub type PNVME_SGL_TRANSPORTDATA_DESC = *mut NVME_SGL_TRANSPORTDATA_DESC;
pub type PNVME_SUBMISSION_QUEUE_TAIL_DOORBELL = *mut NVME_SUBMISSION_QUEUE_TAIL_DOORBELL;
pub type PNVME_SUPPORTED_LOG_PAGES_LOG = *mut NVME_SUPPORTED_LOG_PAGES_LOG;
pub type PNVME_TELEMETRY_CONTROLLER_INITIATED_LOG = *mut NVME_TELEMETRY_CONTROLLER_INITIATED_LOG;
pub type PNVME_TELEMETRY_HOST_INITIATED_LOG = *mut NVME_TELEMETRY_HOST_INITIATED_LOG;
pub type PNVME_UUID_LIST = *mut NVME_UUID_LIST;
pub type PNVME_UUID_LIST_ENTRY = *mut NVME_UUID_LIST_ENTRY;
pub type PNVME_VERSION = *mut NVME_VERSION;
pub type PNVME_WCS_DEVICE_CAPABILITIES = *mut NVME_WCS_DEVICE_CAPABILITIES;
pub type PNVME_WCS_DEVICE_ERROR_RECOVERY_LOG = *mut NVME_WCS_DEVICE_ERROR_RECOVERY_LOG;
pub type PNVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1 = *mut NVME_WCS_DEVICE_ERROR_RECOVERY_LOG;
pub type PNVME_WCS_DEVICE_RECOVERY_ACTION1 = *mut NVME_WCS_DEVICE_RECOVERY_ACTION1;
pub type PNVME_WCS_DEVICE_RECOVERY_ACTION2 = *mut NVME_WCS_DEVICE_RECOVERY_ACTION2;
pub type PNVME_WCS_DEVICE_RESET_ACTION = *mut NVME_WCS_DEVICE_RESET_ACTION;
pub type PNVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG = *mut NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG;
pub type PNVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 = *mut NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2;
pub type PNVME_ZONE_DESCRIPTOR = *mut NVME_ZONE_DESCRIPTOR;
pub type PNVME_ZONE_DESCRIPTOR_EXTENSION = *mut NVME_ZONE_DESCRIPTOR_EXTENSION;
pub type PNVME_ZONE_EXTENDED_REPORT_ZONE_DESC = *mut NVME_ZONE_EXTENDED_REPORT_ZONE_DESC;
pub type PNVM_SET_LIST = *mut NVM_SET_LIST;
pub type PTCG_ACTIVATE_METHOD_SPECIFIC = *mut TCG_ACTIVATE_METHOD_SPECIFIC;
pub type PTCG_ASSIGN_METHOD_SPECIFIC = *mut TCG_ASSIGN_METHOD_SPECIFIC;
pub type PTCG_AUTH_METHOD_SPECIFIC = *mut TCG_AUTH_METHOD_SPECIFIC;
pub type PTCG_BLOCKSID_METHOD_SPECIFIC = *mut TCG_BLOCKSID_METHOD_SPECIFIC;
pub type PTCG_HISTORY_ENTRY = *mut TCG_HISTORY_ENTRY;
pub type PTCG_REACTIVATE_METHOD_SPECIFIC = *mut TCG_REACTIVATE_METHOD_SPECIFIC;
pub type PUNSUPPORTED_REQUIREMENT = *mut UNSUPPORTED_REQUIREMENT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCG_ACTIVATE_METHOD_SPECIFIC {
    pub RangeStartLengthPolicy: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TCG_ASSIGN_METHOD_SPECIFIC {
    pub NamespaceId: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct TCG_AUTH_METHOD_SPECIFIC {
    pub AuthorityId: u64,
    pub TriesCount: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCG_BLOCKSID_METHOD_SPECIFIC {
    pub ClearEvents: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct TCG_HISTORY_ENTRY {
    pub VersionNumber: u8,
    pub EntryLength: u8,
    pub PowerCycleCount: u16,
    pub TcgCommandCount: u32,
    pub TcgCommandCompletionTS: u64,
    pub InvokingId: u64,
    pub MethodId: u64,
    pub ComId: u16,
    pub ProtocolId: u8,
    pub TcgStatus: u8,
    pub ProcessTime: u16,
    pub CommandSpecific: [u8; 10],
}
impl Default for TCG_HISTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TCG_HISTORY_ENTRY_VERSION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCG_REACTIVATE_METHOD_SPECIFIC {
    pub RangeStartLengthPolicy: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNSUPPORTED_REQUIREMENT {
    pub ReqId: [u8; 16],
}
impl Default for UNSUPPORTED_REQUIREMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ZDES_SIZE_MULTIPLIER_IN_BYTES: u32 = 64;
pub type ZONE_STATE = i32;
