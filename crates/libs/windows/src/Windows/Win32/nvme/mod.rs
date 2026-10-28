#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct ACTIVE_LATENCY_CONFIGURATION {
    pub Anonymous: ACTIVE_LATENCY_CONFIGURATION_0,
}
#[cfg(feature = "shared")]
impl Default for ACTIVE_LATENCY_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union ACTIVE_LATENCY_CONFIGURATION_0 {
    pub Anonymous: ACTIVE_LATENCY_CONFIGURATION_0_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for ACTIVE_LATENCY_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ACTIVE_LATENCY_CONFIGURATION_0_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl ACTIVE_LATENCY_CONFIGURATION_0_0 {
    pub fn Read0(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Read0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Write0(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Write0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Trim0(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_Trim0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Read1(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_Read1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Write1(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_Write1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Trim1(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Trim1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Read2(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_Read2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Write2(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_Write2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn Trim2(&self) -> super::USHORT {
        (self._bitfield >> 8) & 1
    }
    pub fn set_Trim2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn Read3(&self) -> super::USHORT {
        (self._bitfield >> 9) & 1
    }
    pub fn set_Read3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn Write3(&self) -> super::USHORT {
        (self._bitfield >> 10) & 1
    }
    pub fn set_Write3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn Trim3(&self) -> super::USHORT {
        (self._bitfield >> 11) & 1
    }
    pub fn set_Trim3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BUCKET_COUNTER {
    pub Reserved: super::ULONG,
    pub Trim: super::ULONG,
    pub Write: super::ULONG,
    pub Read: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_BIT_FIELD {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl DEBUG_BIT_FIELD {
    pub fn Read0(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Read0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Write0(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Write0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Trim0(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_Trim0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Read1(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_Read1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Write1(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_Write1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Trim1(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Trim1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Read2(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_Read2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Write2(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_Write2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn Trim2(&self) -> super::USHORT {
        (self._bitfield >> 8) & 1
    }
    pub fn set_Trim2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn Read3(&self) -> super::USHORT {
        (self._bitfield >> 9) & 1
    }
    pub fn set_Read3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn Write3(&self) -> super::USHORT {
        (self._bitfield >> 10) & 1
    }
    pub fn set_Write3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn Trim3(&self) -> super::USHORT {
        (self._bitfield >> 11) & 1
    }
    pub fn set_Trim3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DSSD_POWER_STATE_DESCRIPTOR {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl DSSD_POWER_STATE_DESCRIPTOR {
    pub fn NvmePowerState(&self) -> super::UCHAR {
        (self._bitfield) & 31
    }
    pub fn set_NvmePowerState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 5)) | ((value & 3) << 5);
    }
    pub fn ValidDSSDPowerState(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_ValidDSSDPowerState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FIRMWARE_ACTIVATION_HISTORY_ENTRY {
    pub VersionNumber: super::UCHAR,
    pub Length: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub ActivationCount: super::USHORT,
    pub Timestamp: super::ULONGLONG,
    pub Reserved1: super::ULONGLONG,
    pub PowerCycleCount: super::ULONGLONG,
    pub PreviousFirmware: super::ULONGLONG,
    pub NewFirmware: super::ULONGLONG,
    pub SlotNumber: super::UCHAR,
    pub CommitActionType: super::UCHAR,
    pub Result: super::USHORT,
    pub Reserved2: [super::UCHAR; 14],
}
#[cfg(feature = "shared")]
impl Default for FIRMWARE_ACTIVATION_HISTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIRMWARE_ACTIVATION_HISTORY_ENTRY_VERSION_1: i32 = 1;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_COMMAND_SET_VECTOR {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl IO_COMMAND_SET_VECTOR {
    pub fn NVMCommandSet(&self) -> super::ULONGLONG {
        (self._bitfield) & 1
    }
    pub fn set_NVMCommandSet(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn KVCommandSet(&self) -> super::ULONGLONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_KVCommandSet(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ZNCommandSet(&self) -> super::ULONGLONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ZNCommandSet(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::ULONGLONG {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(2305843009213693951 << 3)) | ((value & 2305843009213693951) << 3);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct LATENCY_MONITOR_FEATURE_STATUS {
    pub Anonymous: LATENCY_MONITOR_FEATURE_STATUS_0,
}
#[cfg(feature = "shared")]
impl Default for LATENCY_MONITOR_FEATURE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union LATENCY_MONITOR_FEATURE_STATUS_0 {
    pub Anonymous: LATENCY_MONITOR_FEATURE_STATUS_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for LATENCY_MONITOR_FEATURE_STATUS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_MONITOR_FEATURE_STATUS_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl LATENCY_MONITOR_FEATURE_STATUS_0_0 {
    pub fn FeatureEnabled(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_FeatureEnabled(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ActiveLatencyMode(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ActiveLatencyMode(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ActiveMeasuredLatency(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ActiveMeasuredLatency(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_STAMP {
    pub Trim3: super::ULONGLONG,
    pub Write3: super::ULONGLONG,
    pub Read3: super::ULONGLONG,
    pub Trim2: super::ULONGLONG,
    pub Write2: super::ULONGLONG,
    pub Read2: super::ULONGLONG,
    pub Trim1: super::ULONGLONG,
    pub Write1: super::ULONGLONG,
    pub Read1: super::ULONGLONG,
    pub Trim0: super::ULONGLONG,
    pub Write0: super::ULONGLONG,
    pub Read0: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct LATENCY_STAMP_UNITS {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl LATENCY_STAMP_UNITS {
    pub fn Read0(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Read0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Write0(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Write0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Trim0(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_Trim0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Read1(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_Read1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Write1(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_Write1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Trim1(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Trim1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Read2(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_Read2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Write2(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_Write2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn Trim2(&self) -> super::USHORT {
        (self._bitfield >> 8) & 1
    }
    pub fn set_Trim2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn Read3(&self) -> super::USHORT {
        (self._bitfield >> 9) & 1
    }
    pub fn set_Read3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn Write3(&self) -> super::USHORT {
        (self._bitfield >> 10) & 1
    }
    pub fn set_Write3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn Trim3(&self) -> super::USHORT {
        (self._bitfield >> 11) & 1
    }
    pub fn set_Trim3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 12
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEASURED_LATENCY {
    pub Trim3: super::USHORT,
    pub Write3: super::USHORT,
    pub Read3: super::USHORT,
    pub Trim2: super::USHORT,
    pub Write2: super::USHORT,
    pub Read2: super::USHORT,
    pub Trim1: super::USHORT,
    pub Write1: super::USHORT,
    pub Read1: super::USHORT,
    pub Trim0: super::USHORT,
    pub Write0: super::USHORT,
    pub Read0: super::USHORT,
}
pub type NVMEOF_ADDRESS_FAMILY = i32;
pub const NVMEOF_ADMINQ_MAX_DEPTH: i32 = 4096;
pub const NVMEOF_ADMINQ_MIN_DEPTH: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_CHALLENGE {
    pub AUTH_TYPE: super::UCHAR,
    pub AUTH_ID: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub T_ID: super::USHORT,
    pub HL: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub HashID: super::UCHAR,
    pub DHgID: super::UCHAR,
    pub DHVLEN: super::USHORT,
    pub SEQNUM: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_DESCRIPTOR {
    pub AuthId: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub HALEN: super::UCHAR,
    pub DHLEN: super::UCHAR,
    pub IdList: [super::UCHAR; 60],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_AUTH_DHCHAP_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVMEOF_AUTH_DHCHAP_GROUP_ID = i32;
pub type NVMEOF_AUTH_DHCHAP_HASH_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_REPLY {
    pub AUTH_TYPE: super::UCHAR,
    pub AUTH_ID: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub T_ID: super::USHORT,
    pub HL: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub CVALID: super::UCHAR,
    pub Reserved2: super::UCHAR,
    pub DHVLEN: super::USHORT,
    pub SEQNUM: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_SUCCESS1 {
    pub AUTH_TYPE: super::UCHAR,
    pub AUTH_ID: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub T_ID: super::USHORT,
    pub HL: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub RVALID: super::UCHAR,
    pub Reserved2: [super::UCHAR; 7],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_AUTH_DHCHAP_SUCCESS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_AUTH_DHCHAP_SUCCESS2 {
    pub AUTH_TYPE: super::UCHAR,
    pub AUTH_ID: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub T_ID: super::USHORT,
    pub Reserved1: [super::UCHAR; 10],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_AUTH_DHCHAP_SUCCESS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_FAILURE {
    pub AUTH_TYPE: super::UCHAR,
    pub AUTH_ID: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub T_ID: super::USHORT,
    pub ReasonCode: super::UCHAR,
    pub ReasonExplanation: super::UCHAR,
}
pub type NVMEOF_AUTH_FAIL_REASON_CODE = i32;
pub type NVMEOF_AUTH_FAIL_REASON_EXPLANATION = i32;
pub type NVMEOF_AUTH_ID = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_NEGOTIATE {
    pub AUTH_TYPE: super::UCHAR,
    pub AUTH_ID: super::UCHAR,
    pub Reserved0: super::USHORT,
    pub T_ID: super::USHORT,
    pub SC_C: super::UCHAR,
    pub NAPD: super::UCHAR,
}
pub type NVMEOF_AUTH_PROTOCOL = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_AUTH_RECEIVE_COMMAND {
    pub OPC: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub CID: super::USHORT,
    pub FCTYPE: super::UCHAR,
    pub Reserved1: [super::UCHAR; 19],
    pub SGL1: NVME_SGL_DESC,
    pub Reserved2: super::UCHAR,
    pub SPSP0: super::UCHAR,
    pub SPSP1: super::UCHAR,
    pub SECP: super::UCHAR,
    pub AL: super::ULONG,
    pub Reserved3: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_AUTH_RECEIVE_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_RECEIVE_RESPONSE {
    pub Reserved0: super::ULONGLONG,
    pub SQHD: super::USHORT,
    pub Reserved1: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
pub type NVMEOF_AUTH_SECURE_CHANNEL = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_AUTH_SEND_COMMAND {
    pub OPC: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub CID: super::USHORT,
    pub FCTYPE: super::UCHAR,
    pub Reserved1: [super::UCHAR; 19],
    pub SGL1: NVME_SGL_DESC,
    pub Reserved2: super::UCHAR,
    pub SPSP0: super::UCHAR,
    pub SPSP1: super::UCHAR,
    pub SECP: super::UCHAR,
    pub TL: super::ULONG,
    pub Reserved3: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_AUTH_SEND_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_AUTH_SEND_RESPONSE {
    pub Reserved0: super::ULONGLONG,
    pub SQHD: super::USHORT,
    pub Reserved1: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
pub type NVMEOF_AUTH_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_CONNECT_COMMAND {
    pub OPC: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub CID: super::USHORT,
    pub FCTYPE: super::UCHAR,
    pub Reserved1: [super::UCHAR; 19],
    pub SGL1: NVME_SGL_DESC,
    pub RECFMT: super::USHORT,
    pub QID: super::USHORT,
    pub SQSIZE: super::USHORT,
    pub CATTR: NVMEOF_CONNECT_COMMAND_0,
    pub Reserved2: super::UCHAR,
    pub KATO: super::ULONG,
    pub Reserved3: [super::UCHAR; 12],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVMEOF_CONNECT_COMMAND_0 {
    pub Anonymous: NVMEOF_CONNECT_COMMAND_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_CONNECT_COMMAND_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVMEOF_CONNECT_COMMAND_0_0 {
    pub fn PriorityClass(&self) -> super::UCHAR {
        (self._bitfield) & 3
    }
    pub fn set_PriorityClass(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn SqFlowControlDisable(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_SqFlowControlDisable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn IoQueueDeletion(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_IoQueueDeletion(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_CONNECT_DATA {
    pub HOSTID: [super::UCHAR; 16],
    pub CNTLID: super::USHORT,
    pub Reserved0: [super::UCHAR; 238],
    pub SUBNQN: [super::UCHAR; 256],
    pub HOSTNQN: [super::UCHAR; 256],
    pub Reserved1: [super::UCHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_CONNECT_RESPONSE {
    pub SCSpecific: NVMEOF_CONNECT_RESPONSE_0,
    pub Reserved0: super::ULONG,
    pub SQHD: super::USHORT,
    pub Reserved1: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVMEOF_CONNECT_RESPONSE_0 {
    pub Success: NVMEOF_CONNECT_RESPONSE_0_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_CONNECT_RESPONSE_0_0 {
    pub CNTLID: super::USHORT,
    pub AUTHREQ: NVMEOF_CONNECT_RESPONSE_0_0_0,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_RESPONSE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVMEOF_CONNECT_RESPONSE_0_0_0 {
    pub Anonymous: NVMEOF_CONNECT_RESPONSE_0_0_0_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_CONNECT_RESPONSE_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_CONNECT_RESPONSE_0_0_0_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVMEOF_CONNECT_RESPONSE_0_0_0_0 {
    pub fn Obsolete(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Obsolete(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ATR(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ATR(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ASCR(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ASCR(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
pub const NVMEOF_DHCHAP_PREFIX_V1: windows_core::PCSTR = windows_core::s!("DHHC-1:");
pub const NVMEOF_DHCHAP_PROTOCOL_ID: i32 = 1;
pub const NVMEOF_DHCHAP_REPLY_CVAL_NOTVALID: i32 = 0;
pub const NVMEOF_DHCHAP_REPLY_CVAL_VALID: i32 = 1;
pub const NVMEOF_DHCHAP_SUCCESS1_RVAL_NOTVALID: i32 = 0;
pub const NVMEOF_DHCHAP_SUCCESS1_RVAL_VALID: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_DISCONNECT_RESPONSE {
    pub Reserved0: super::ULONGLONG,
    pub SQHD: super::USHORT,
    pub Reserved1: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
pub const NVMEOF_DISCOVERY_LOG_VERSION_0: i32 = 0;
pub const NVMEOF_DISCOVERY_NQN: windows_core::PCSTR = windows_core::s!("nqn.2014-08.org.nvmexpress.discovery");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_FABRICS_COMMAND {
    pub OPC: super::UCHAR,
    pub PSDT: super::UCHAR,
    pub CID: super::USHORT,
    pub FCTYPE: super::UCHAR,
    pub Reserved: [super::UCHAR; 35],
    pub Specific: [super::UCHAR; 24],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_FABRICS_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_FABRICS_RESPONSE {
    pub Specific: [super::UCHAR; 8],
    pub SQHD: super::USHORT,
    pub Reserved: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_FABRICS_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVMEOF_IOQ_MAX_DEPTH: i32 = 65536;
pub const NVMEOF_IOQ_MIN_DEPTH: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_GET_COMMAND {
    pub OPC: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub CID: super::USHORT,
    pub FCTYPE: super::UCHAR,
    pub Reserved1: [super::UCHAR; 35],
    pub ATTRIB: NVMEOF_PROPERTY_GET_COMMAND_0,
    pub Reserved2: [super::UCHAR; 3],
    pub OFST: super::ULONG,
    pub Reserved3: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_PROPERTY_GET_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_GET_COMMAND_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVMEOF_PROPERTY_GET_COMMAND_0 {
    pub fn PropertySize(&self) -> super::UCHAR {
        (self._bitfield) & 7
    }
    pub fn set_PropertySize(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_PROPERTY_GET_RESPONSE {
    pub VALUE: NVMEOF_PROPERTY_GET_RESPONSE_0,
    pub SQHD: super::USHORT,
    pub Reserved0: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_PROPERTY_GET_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVMEOF_PROPERTY_GET_RESPONSE_0 {
    pub FourBytes: NVMEOF_PROPERTY_GET_RESPONSE_0_0,
    pub EightBytes: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_PROPERTY_GET_RESPONSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_GET_RESPONSE_0_0 {
    pub Value: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVMEOF_PROPERTY_SET_COMMAND {
    pub OPC: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub CID: super::USHORT,
    pub FCTYPE: super::UCHAR,
    pub Reserved1: [super::UCHAR; 35],
    pub ATTRIB: NVMEOF_PROPERTY_SET_COMMAND_0,
    pub Reserved2: [super::UCHAR; 3],
    pub OFST: super::ULONG,
    pub VALUE: NVMEOF_PROPERTY_SET_COMMAND_1,
    pub Reserved3: [super::UCHAR; 8],
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_PROPERTY_SET_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_SET_COMMAND_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVMEOF_PROPERTY_SET_COMMAND_0 {
    pub fn PropertySize(&self) -> super::UCHAR {
        (self._bitfield) & 7
    }
    pub fn set_PropertySize(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVMEOF_PROPERTY_SET_COMMAND_1 {
    pub FourBytes: NVMEOF_PROPERTY_SET_COMMAND_1_0,
    pub EightBytes: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVMEOF_PROPERTY_SET_COMMAND_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_SET_COMMAND_1_0 {
    pub Value: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVMEOF_PROPERTY_SET_RESPONSE {
    pub Reserved0: super::ULONGLONG,
    pub SQHD: super::USHORT,
    pub Reserved1: super::USHORT,
    pub CID: super::USHORT,
    pub STS: super::USHORT,
}
pub const NVMEOF_PROPERTY_SIZE_4Bytes: i32 = 0;
pub const NVMEOF_PROPERTY_SIZE_8Bytes: i32 = 1;
pub type NVMEOF_SECURE_CHANNEL = i32;
pub type NVMEOF_SECURE_CHANNEL_PROTOCOL = i32;
pub type NVMEOF_SUBSYSTEM_TYPE = i32;
pub const NVMEOF_TRANSPORT_ADDR_MAX_LEN: i32 = 256;
pub const NVMEOF_TRANSPORT_SAS_MAX_LEN: i32 = 256;
pub const NVMEOF_TRANSPORT_SERVID_MAX_LEN: i32 = 32;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ACTIVE_NAMESPACE_ID_LIST {
    pub NSID: [super::ULONG; 1024],
}
#[cfg(feature = "shared")]
impl Default for NVME_ACTIVE_NAMESPACE_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_ADMINQ_ID: i32 = 0;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    pub Anonymous: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0,
    pub AsUlonglong: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS_0 {
    pub fn Reserved0(&self) -> super::ULONGLONG {
        (self._bitfield) & 4095
    }
    pub fn set_Reserved0(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn ACQB(&self) -> super::ULONGLONG {
        self._bitfield >> 12
    }
    pub fn set_ACQB(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(4503599627370495 << 12)) | ((value & 4503599627370495) << 12);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_QUEUE_ATTRIBUTES {
    pub Anonymous: NVME_ADMIN_QUEUE_ATTRIBUTES_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_ADMIN_QUEUE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ADMIN_QUEUE_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_ADMIN_QUEUE_ATTRIBUTES_0 {
    pub fn ASQS(&self) -> super::ULONG {
        (self._bitfield) & 4095
    }
    pub fn set_ASQS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 12) & 15
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn ACQS(&self) -> super::ULONG {
        (self._bitfield >> 16) & 4095
    }
    pub fn set_ACQS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(4095 << 16)) | ((value & 4095) << 16);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 28
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 28)) | ((value & 15) << 28);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    pub Anonymous: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0,
    pub AsUlonglong: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS_0 {
    pub fn Reserved0(&self) -> super::ULONGLONG {
        (self._bitfield) & 4095
    }
    pub fn set_Reserved0(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !4095) | (value & 4095);
    }
    pub fn ASQB(&self) -> super::ULONGLONG {
        self._bitfield >> 12
    }
    pub fn set_ASQB(&mut self, value: super::ULONGLONG) {
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    pub _bitfield: u32,
    pub Reserved1: super::ULONG,
}
#[cfg(feature = "shared")]
impl NVME_AUTO_POWER_STATE_TRANSITION_ENTRY {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IdleTransitionPowerState(&self) -> super::ULONG {
        (self._bitfield >> 3) & 31
    }
    pub fn set_IdleTransitionPowerState(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
    pub fn IdleTimePriorToTransition(&self) -> super::ULONG {
        self._bitfield >> 8
    }
    pub fn set_IdleTimePriorToTransition(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_BOOT_PARTITION_INFORMATION {
    pub Anonymous: NVME_BOOT_PARTITION_INFORMATION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_BOOT_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_BOOT_PARTITION_INFORMATION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_BOOT_PARTITION_INFORMATION_0 {
    pub fn BPSZ(&self) -> super::ULONG {
        (self._bitfield) & 32767
    }
    pub fn set_BPSZ(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !32767) | (value & 32767);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 15) & 511
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(511 << 15)) | ((value & 511) << 15);
    }
    pub fn BRS(&self) -> super::ULONG {
        (self._bitfield >> 24) & 3
    }
    pub fn set_BRS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 24)) | ((value & 3) << 24);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        (self._bitfield >> 26) & 31
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(31 << 26)) | ((value & 31) << 26);
    }
    pub fn ABPID(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_ABPID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_BOOT_PARTITION_LOG {
    pub LogIdentifier: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub BpInfo: NVME_BOOT_PARTITION_INFORMATION,
    pub Reserved1: [super::UCHAR; 8],
    pub BootPartitionData: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_DSSD_POWER_STATE {
    pub Anonymous: NVME_CDW0_FEATURE_DSSD_POWER_STATE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW0_FEATURE_DSSD_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_FEATURE_DSSD_POWER_STATE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW0_FEATURE_DSSD_POWER_STATE_0 {
    pub fn DSSDPowerState(&self) -> super::ULONG {
        (self._bitfield) & 127
    }
    pub fn set_DSSDPowerState(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 7
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    pub Anonymous: NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub fn Enabled(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_Enabled(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[cfg(feature = "shared")]
pub type NVME_CDW0_FEATURE_ERROR_INJECTION = NVME_CDW11_FEATURE_ERROR_INJECTION;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub Anonymous: NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub fn EOLBehavior(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_EOLBehavior(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW0_RESERVATION_PERSISTENCE {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW0_RESERVATION_PERSISTENCE {
    pub fn PTPL(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_PTPL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_ABORT {
    pub Anonymous: NVME_CDW10_ABORT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_ABORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ABORT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_ABORT_0 {
    pub fn SQID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_SQID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn CID(&self) -> super::ULONG {
        (self._bitfield >> 8) & 65535
    }
    pub fn set_CID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 8)) | ((value & 65535) << 8);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_CREATE_IO_QUEUE {
    pub Anonymous: NVME_CDW10_CREATE_IO_QUEUE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_CREATE_IO_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_CREATE_IO_QUEUE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_CREATE_IO_QUEUE_0 {
    pub fn QID(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_QID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn QSIZE(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_QSIZE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DATASET_MANAGEMENT {
    pub Anonymous: NVME_CDW10_DATASET_MANAGEMENT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_DATASET_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DATASET_MANAGEMENT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_DATASET_MANAGEMENT_0 {
    pub fn NR(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_NR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DELETE_IO_QUEUE {
    pub Anonymous: NVME_CDW10_DELETE_IO_QUEUE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_DELETE_IO_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DELETE_IO_QUEUE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_DELETE_IO_QUEUE_0 {
    pub fn QID(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_QID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DEVICE_SELF_TEST {
    pub Anonymous: NVME_CDW10_DEVICE_SELF_TEST_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_DEVICE_SELF_TEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DEVICE_SELF_TEST_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_DEVICE_SELF_TEST_0 {
    pub fn STC(&self) -> super::ULONG {
        (self._bitfield) & 15
    }
    pub fn set_STC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DIRECTIVE_RECEIVE {
    pub NUMD: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DIRECTIVE_SEND {
    pub NUMD: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_DISCOVERY_INFO_MGMT {
    pub Anonymous: NVME_CDW10_DISCOVERY_INFO_MGMT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_DISCOVERY_INFO_MGMT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_DISCOVERY_INFO_MGMT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_DISCOVERY_INFO_MGMT_0 {
    pub fn TAS(&self) -> super::ULONG {
        (self._bitfield) & 15
    }
    pub fn set_TAS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_FIRMWARE_ACTIVATE {
    pub Anonymous: NVME_CDW10_FIRMWARE_ACTIVATE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_FIRMWARE_ACTIVATE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_FIRMWARE_ACTIVATE_0 {
    pub fn FS(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_FS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn AA(&self) -> super::ULONG {
        (self._bitfield >> 3) & 7
    }
    pub fn set_AA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 6) & 33554431
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(33554431 << 6)) | ((value & 33554431) << 6);
    }
    pub fn BPID(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_BPID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_FIRMWARE_DOWNLOAD {
    pub NUMD: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_FORMAT_NVM {
    pub Anonymous: NVME_CDW10_FORMAT_NVM_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_FORMAT_NVM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_FORMAT_NVM_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_FORMAT_NVM_0 {
    pub fn LBAF(&self) -> super::ULONG {
        (self._bitfield) & 15
    }
    pub fn set_LBAF(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MS(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_MS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn PI(&self) -> super::ULONG {
        (self._bitfield >> 5) & 7
    }
    pub fn set_PI(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn PIL(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_PIL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn SES(&self) -> super::ULONG {
        (self._bitfield >> 9) & 7
    }
    pub fn set_SES(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 9)) | ((value & 7) << 9);
    }
    pub fn ZF(&self) -> super::ULONG {
        (self._bitfield >> 12) & 3
    }
    pub fn set_ZF(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 12)) | ((value & 3) << 12);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 14
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(262143 << 14)) | ((value & 262143) << 14);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_FEATURES {
    pub Anonymous: NVME_CDW10_GET_FEATURES_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_GET_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_FEATURES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_GET_FEATURES_0 {
    pub fn FID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_FID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn SEL(&self) -> super::ULONG {
        (self._bitfield >> 8) & 7
    }
    pub fn set_SEL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 8)) | ((value & 7) << 8);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 11
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2097151 << 11)) | ((value & 2097151) << 11);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_GET_LOG_PAGE_0 {
    pub fn LID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_LID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn NUMD(&self) -> super::ULONG {
        (self._bitfield >> 16) & 4095
    }
    pub fn set_NUMD(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(4095 << 16)) | ((value & 4095) << 16);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 28
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 28)) | ((value & 15) << 28);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V121 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V121_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_GET_LOG_PAGE_V121 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_V121_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_GET_LOG_PAGE_V121_0 {
    pub fn LID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_LID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn NUMDL(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_NUMDL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V13 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V13_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_GET_LOG_PAGE_V13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_V13_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_GET_LOG_PAGE_V13_0 {
    pub fn LID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_LID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn LSP(&self) -> super::ULONG {
        (self._bitfield >> 8) & 15
    }
    pub fn set_LSP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 12) & 7
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 12)) | ((value & 7) << 12);
    }
    pub fn RAE(&self) -> super::ULONG {
        (self._bitfield >> 15) & 1
    }
    pub fn set_RAE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
    pub fn NUMDL(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_NUMDL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_GET_LOG_PAGE_V20 {
    pub Anonymous: NVME_CDW10_GET_LOG_PAGE_V20_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_GET_LOG_PAGE_V20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_GET_LOG_PAGE_V20_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_GET_LOG_PAGE_V20_0 {
    pub fn LID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_LID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn LSP(&self) -> super::ULONG {
        (self._bitfield >> 8) & 127
    }
    pub fn set_LSP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(127 << 8)) | ((value & 127) << 8);
    }
    pub fn RAE(&self) -> super::ULONG {
        (self._bitfield >> 15) & 1
    }
    pub fn set_RAE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
    pub fn NUMDL(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_NUMDL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_IDENTIFY {
    pub Anonymous: NVME_CDW10_IDENTIFY_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_IDENTIFY_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_IDENTIFY_0 {
    pub fn CNS(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_CNS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn CNTID(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_CNTID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
pub const NVME_CDW10_LSP_ACTION_ESTABLISH_CONTEXT_AND_READ_512_BYTES_OF_HEADER: i32 = 3;
pub const NVME_CDW10_LSP_ACTION_ESTABLISH_CONTEXT_AND_READ_LOG_DATA: i32 = 1;
pub const NVME_CDW10_LSP_ACTION_READ_LOG_DATA: i32 = 0;
pub const NVME_CDW10_LSP_ACTION_RELEASE_CONTEXT: i32 = 2;
pub const NVME_CDW10_LSP_BOOT_PARTITION_0: i32 = 0;
pub const NVME_CDW10_LSP_BOOT_PARTITION_1: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_ACQUIRE {
    pub Anonymous: NVME_CDW10_RESERVATION_ACQUIRE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_RESERVATION_ACQUIRE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_ACQUIRE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_RESERVATION_ACQUIRE_0 {
    pub fn RACQA(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_RACQA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IEKEY(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_IEKEY(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 4) & 15
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn RTYPE(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_RTYPE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_REGISTER {
    pub Anonymous: NVME_CDW10_RESERVATION_REGISTER_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_RESERVATION_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_REGISTER_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_RESERVATION_REGISTER_0 {
    pub fn RREGA(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_RREGA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IEKEY(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_IEKEY(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 4) & 67108863
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(67108863 << 4)) | ((value & 67108863) << 4);
    }
    pub fn CPTPL(&self) -> super::ULONG {
        self._bitfield >> 30
    }
    pub fn set_CPTPL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 30)) | ((value & 3) << 30);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_RELEASE {
    pub Anonymous: NVME_CDW10_RESERVATION_RELEASE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_RESERVATION_RELEASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_RELEASE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_RESERVATION_RELEASE_0 {
    pub fn RRELA(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_RRELA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn IEKEY(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_IEKEY(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 4) & 15
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn RTYPE(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_RTYPE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_RESERVATION_REPORT {
    pub Anonymous: NVME_CDW10_RESERVATION_REPORT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_RESERVATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_RESERVATION_REPORT_0 {
    pub NUMD: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SANITIZE {
    pub Anonymous: NVME_CDW10_SANITIZE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_SANITIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_SANITIZE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_SANITIZE_0 {
    pub fn SANACT(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_SANACT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn AUSE(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_AUSE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn OWPASS(&self) -> super::ULONG {
        (self._bitfield >> 4) & 15
    }
    pub fn set_OWPASS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
    pub fn OIPBP(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_OIPBP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn NDAS(&self) -> super::ULONG {
        (self._bitfield >> 9) & 1
    }
    pub fn set_NDAS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 10
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(4194303 << 10)) | ((value & 4194303) << 10);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SECURITY_SEND_RECEIVE {
    pub Anonymous: NVME_CDW10_SECURITY_SEND_RECEIVE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_SECURITY_SEND_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_SECURITY_SEND_RECEIVE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_SECURITY_SEND_RECEIVE_0 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn SPSP(&self) -> super::ULONG {
        (self._bitfield >> 8) & 65535
    }
    pub fn set_SPSP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 8)) | ((value & 65535) << 8);
    }
    pub fn SECP(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_SECP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW10_SET_FEATURES {
    pub Anonymous: NVME_CDW10_SET_FEATURES_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW10_SET_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_SET_FEATURES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW10_SET_FEATURES_0 {
    pub fn FID(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_FID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 8) & 8388607
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(8388607 << 8)) | ((value & 8388607) << 8);
    }
    pub fn SV(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_SV(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ZONE_APPEND {
    pub SLBA: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_RECEIVE {
    pub SLBA: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW10_ZONE_MANAGEMENT_SEND {
    pub SLBA: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_CREATE_IO_CQ {
    pub Anonymous: NVME_CDW11_CREATE_IO_CQ_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_CREATE_IO_CQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_CREATE_IO_CQ_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_CREATE_IO_CQ_0 {
    pub fn PC(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_PC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn IEN(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_IEN(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 2) & 16383
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
    pub fn IV(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_IV(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_CREATE_IO_SQ {
    pub Anonymous: NVME_CDW11_CREATE_IO_SQ_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_CREATE_IO_SQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_CREATE_IO_SQ_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_CREATE_IO_SQ_0 {
    pub fn PC(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_PC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn QPRIO(&self) -> super::ULONG {
        (self._bitfield >> 1) & 3
    }
    pub fn set_QPRIO(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 1)) | ((value & 3) << 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 3) & 8191
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
    pub fn CQID(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_CQID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DATASET_MANAGEMENT {
    pub Anonymous: NVME_CDW11_DATASET_MANAGEMENT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_DATASET_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_DATASET_MANAGEMENT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_DATASET_MANAGEMENT_0 {
    pub fn IDR(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_IDR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn IDW(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_IDW(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn AD(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_AD(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DIRECTIVE_RECEIVE {
    pub Anonymous: NVME_CDW11_DIRECTIVE_RECEIVE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_DIRECTIVE_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_DIRECTIVE_RECEIVE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_DIRECTIVE_RECEIVE_0 {
    pub fn DOPER(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_DOPER(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn DTYPE(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_DTYPE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn DSPEC(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_DSPEC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_DIRECTIVE_SEND {
    pub Anonymous: NVME_CDW11_DIRECTIVE_SEND_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_DIRECTIVE_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_DIRECTIVE_SEND_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_DIRECTIVE_SEND_0 {
    pub fn DOPER(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_DOPER(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn DTYPE(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_DTYPE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn DSPEC(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_DSPEC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
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
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ARBITRATION {
    pub Anonymous: NVME_CDW11_FEATURE_ARBITRATION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_ARBITRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ARBITRATION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_ARBITRATION_0 {
    pub fn AB(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_AB(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 3) & 31
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
    pub fn LPW(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_LPW(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn MPW(&self) -> super::ULONG {
        (self._bitfield >> 16) & 255
    }
    pub fn set_MPW(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn HPW(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_HPW(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    pub Anonymous: NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG_0 {
    pub fn CriticalWarnings(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_CriticalWarnings(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn NsAttributeNotices(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_NsAttributeNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn FwActivationNotices(&self) -> super::ULONG {
        (self._bitfield >> 9) & 1
    }
    pub fn set_FwActivationNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn TelemetryLogNotices(&self) -> super::ULONG {
        (self._bitfield >> 10) & 1
    }
    pub fn set_TelemetryLogNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn ANAChangeNotices(&self) -> super::ULONG {
        (self._bitfield >> 11) & 1
    }
    pub fn set_ANAChangeNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn PredictableLogChangeNotices(&self) -> super::ULONG {
        (self._bitfield >> 12) & 1
    }
    pub fn set_PredictableLogChangeNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value & 1) << 12);
    }
    pub fn LBAStatusNotices(&self) -> super::ULONG {
        (self._bitfield >> 13) & 1
    }
    pub fn set_LBAStatusNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn EnduranceEventNotices(&self) -> super::ULONG {
        (self._bitfield >> 14) & 1
    }
    pub fn set_EnduranceEventNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn NormalNVMSubsystemShutdown(&self) -> super::ULONG {
        (self._bitfield >> 15) & 1
    }
    pub fn set_NormalNVMSubsystemShutdown(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 16) & 2047
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2047 << 16)) | ((value & 2047) << 16);
    }
    pub fn ZoneDescriptorNotices(&self) -> super::ULONG {
        (self._bitfield >> 27) & 1
    }
    pub fn set_ZoneDescriptorNotices(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value & 1) << 27);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        (self._bitfield >> 28) & 7
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 28)) | ((value & 7) << 28);
    }
    pub fn DiscoveryLogPageChange(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_DiscoveryLogPageChange(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    pub Anonymous: NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION_0 {
    pub fn APSTE(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_APSTE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    pub Anonymous: NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY_0 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 2147483647
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !2147483647) | (value & 2147483647);
    }
    pub fn Clear(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_Clear(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    pub Anonymous: NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS_0 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 2147483647
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !2147483647) | (value & 2147483647);
    }
    pub fn Clear(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_Clear(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    pub Anonymous: NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO_0 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 2147483647
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !2147483647) | (value & 2147483647);
    }
    pub fn Enable(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_Enable(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ERROR_INJECTION {
    pub Anonymous: NVME_CDW11_FEATURE_ERROR_INJECTION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_ERROR_INJECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ERROR_INJECTION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_ERROR_INJECTION_0 {
    pub fn NUM(&self) -> super::ULONG {
        (self._bitfield) & 127
    }
    pub fn set_NUM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 7
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_ERROR_RECOVERY {
    pub Anonymous: NVME_CDW11_FEATURE_ERROR_RECOVERY_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_ERROR_RECOVERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_ERROR_RECOVERY_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_ERROR_RECOVERY_0 {
    pub fn TLER(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_TLER(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn DULBE(&self) -> super::ULONG {
        (self._bitfield >> 16) & 1
    }
    pub fn set_DULBE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value & 1) << 16);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 17
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(32767 << 17)) | ((value & 32767) << 17);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_GET_HOST_METADATA {
    pub Anonymous: NVME_CDW11_FEATURE_GET_HOST_METADATA_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_GET_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_GET_HOST_METADATA_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_GET_HOST_METADATA_0 {
    pub fn GDHM(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_GDHM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_HOST_IDENTIFIER {
    pub fn EXHID(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_EXHID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub fn EHM(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_EHM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn MR(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_MR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    pub Anonymous: NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_INTERRUPT_COALESCING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_INTERRUPT_COALESCING_0 {
    pub fn THR(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_THR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn TIME(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_TIME(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    pub Anonymous: NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG_0 {
    pub fn IV(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_IV(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn CD(&self) -> super::ULONG {
        (self._bitfield >> 16) & 1
    }
    pub fn set_CD(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value & 1) << 16);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 17
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(32767 << 17)) | ((value & 32767) << 17);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    pub Anonymous: NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE_0 {
    pub fn IOCSCI(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_IOCSCI(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    pub Anonymous: NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_LBA_RANGE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_LBA_RANGE_TYPE_0 {
    pub fn NUM(&self) -> super::ULONG {
        (self._bitfield) & 63
    }
    pub fn set_NUM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !63) | (value & 63);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 6
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    pub Anonymous: NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE_0 {
    pub fn NOPPME(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_NOPPME(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    pub Anonymous: NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_NUMBER_OF_QUEUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_NUMBER_OF_QUEUES_0 {
    pub fn NSQ(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NSQ(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn NCQ(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_NCQ(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    pub Anonymous: NVME_CDW11_FEATURE_POWER_MANAGEMENT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_POWER_MANAGEMENT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_POWER_MANAGEMENT_0 {
    pub fn PS(&self) -> super::ULONG {
        (self._bitfield) & 31
    }
    pub fn set_PS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !31) | (value & 31);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 5
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(134217727 << 5)) | ((value & 134217727) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    pub Anonymous: NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE_0 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 1073741823
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1073741823) | (value & 1073741823);
    }
    pub fn EOLBehavior(&self) -> super::ULONG {
        self._bitfield >> 30
    }
    pub fn set_EOLBehavior(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 30)) | ((value & 3) << 30);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK {
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn REGPRE(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_REGPRE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn RESREL(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_RESREL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn RESPRE(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_RESPRE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 4
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE {
    pub fn PTPL(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_PTPL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_SET_HOST_METADATA {
    pub Anonymous: NVME_CDW11_FEATURE_SET_HOST_METADATA_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_SET_HOST_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_SET_HOST_METADATA_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_SET_HOST_METADATA_0 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 8191
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !8191) | (value & 8191);
    }
    pub fn EA(&self) -> super::ULONG {
        (self._bitfield >> 13) & 3
    }
    pub fn set_EA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 13)) | ((value & 3) << 13);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 15
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(131071 << 15)) | ((value & 131071) << 15);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    pub Anonymous: NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY_0 {
    pub fn SAVE(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_SAVE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn NSS(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_NSS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn MOD(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_MOD(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 3
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(536870911 << 3)) | ((value & 536870911) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    pub Anonymous: NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD_0 {
    pub fn TMPTH(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_TMPTH(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn TMPSEL(&self) -> super::ULONG {
        (self._bitfield >> 16) & 15
    }
    pub fn set_TMPSEL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 16)) | ((value & 15) << 16);
    }
    pub fn THSEL(&self) -> super::ULONG {
        (self._bitfield >> 20) & 3
    }
    pub fn set_THSEL(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 20)) | ((value & 3) << 20);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 22
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1023 << 22)) | ((value & 1023) << 22);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    pub Anonymous: NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE_0 {
    pub fn WCE(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_WCE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    pub Anonymous: NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL_0 {
    pub fn DN(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_DN(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_FIRMWARE_DOWNLOAD {
    pub OFST: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW11_GET_LOG_PAGE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_GET_LOG_PAGE_0 {
    pub fn NUMDU(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NUMDU(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn LogSpecificIdentifier(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_LogSpecificIdentifier(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_IDENTIFY {
    pub Anonymous: NVME_CDW11_IDENTIFY_0,
    pub Anonymous2: NVME_CDW11_IDENTIFY_1,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_IDENTIFY_0 {
    pub NVMSETID: super::USHORT,
    pub Reserved: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_IDENTIFY_1 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_IDENTIFY_1 {
    pub fn CNSID(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_CNSID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved2(&self) -> super::ULONG {
        (self._bitfield >> 16) & 255
    }
    pub fn set_Reserved2(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn CSI(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_CSI(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_RESERVATION_REPORT {
    pub Anonymous: NVME_CDW11_RESERVATION_REPORT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_RESERVATION_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_RESERVATION_REPORT_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW11_RESERVATION_REPORT_0 {
    pub fn EDS(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_EDS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2147483647 << 1)) | ((value & 2147483647) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW11_SANITIZE {
    pub Anonymous: NVME_CDW11_SANITIZE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW11_SANITIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_SANITIZE_0 {
    pub OVRPAT: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_SECURITY_RECEIVE {
    pub AL: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW11_SECURITY_SEND {
    pub TL: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE {
    pub AllocateResources: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_DIRECTIVE_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub Anonymous: NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub fn NSR(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NSR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND {
    pub EnableDirective: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_DIRECTIVE_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    pub Anonymous: NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE_0 {
    pub fn ENDIR(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_ENDIR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 1) & 127
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
    pub fn DTYPE(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_DTYPE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURES {
    pub HostMemoryBuffer: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HSIZE: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_GET_LOG_PAGE {
    pub LPOL: super::ULONG,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_READ_WRITE {
    pub Anonymous: NVME_CDW12_READ_WRITE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_READ_WRITE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW12_READ_WRITE_0 {
    pub fn NLB(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NLB(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 16) & 15
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 16)) | ((value & 15) << 16);
    }
    pub fn DTYPE(&self) -> super::ULONG {
        (self._bitfield >> 20) & 15
    }
    pub fn set_DTYPE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 20)) | ((value & 15) << 20);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        (self._bitfield >> 24) & 3
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 24)) | ((value & 3) << 24);
    }
    pub fn PRINFO(&self) -> super::ULONG {
        (self._bitfield >> 26) & 15
    }
    pub fn set_PRINFO(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 26)) | ((value & 15) << 26);
    }
    pub fn FUA(&self) -> super::ULONG {
        (self._bitfield >> 30) & 1
    }
    pub fn set_FUA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value & 1) << 30);
    }
    pub fn LR(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_LR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_VERIFYCOMMAND {
    pub Anonymous: NVME_CDW12_VERIFYCOMMAND_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_VERIFYCOMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_VERIFYCOMMAND_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW12_VERIFYCOMMAND_0 {
    pub fn NLB(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NLB(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 16) & 1023
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1023 << 16)) | ((value & 1023) << 16);
    }
    pub fn PRINFO(&self) -> super::ULONG {
        (self._bitfield >> 26) & 15
    }
    pub fn set_PRINFO(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 26)) | ((value & 15) << 26);
    }
    pub fn FUA(&self) -> super::ULONG {
        (self._bitfield >> 30) & 1
    }
    pub fn set_FUA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value & 1) << 30);
    }
    pub fn LR(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_LR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW12_ZONE_APPEND {
    pub Anonymous: NVME_CDW12_ZONE_APPEND_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW12_ZONE_APPEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW12_ZONE_APPEND_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW12_ZONE_APPEND_0 {
    pub fn NLB(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NLB(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 16) & 511
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(511 << 16)) | ((value & 511) << 16);
    }
    pub fn PIREMAP(&self) -> super::ULONG {
        (self._bitfield >> 25) & 1
    }
    pub fn set_PIREMAP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value & 1) << 25);
    }
    pub fn PRINFO(&self) -> super::ULONG {
        (self._bitfield >> 26) & 15
    }
    pub fn set_PRINFO(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 26)) | ((value & 15) << 26);
    }
    pub fn FUA(&self) -> super::ULONG {
        (self._bitfield >> 30) & 1
    }
    pub fn set_FUA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 30)) | ((value & 1) << 30);
    }
    pub fn LR(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_LR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURES {
    pub HostMemoryBuffer: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW13_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield) & 15
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn HMDLLA(&self) -> super::ULONG {
        self._bitfield >> 4
    }
    pub fn set_HMDLLA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(268435455 << 4)) | ((value & 268435455) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW13_GET_LOG_PAGE {
    pub LPOU: super::ULONG,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW13_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW13_READ_WRITE {
    pub Anonymous: NVME_CDW13_READ_WRITE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW13_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_READ_WRITE_0 {
    pub DSM: NVME_CDW13_READ_WRITE_0_0,
    pub Reserved: super::UCHAR,
    pub DSPEC: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_READ_WRITE_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_CDW13_READ_WRITE_0_0 {
    pub fn AccessFrequency(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_AccessFrequency(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn AccessLatency(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 3
    }
    pub fn set_AccessLatency(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn SequentialRequest(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_SequentialRequest(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Incompressible(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Incompressible(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    pub Anonymous: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW13_ZONE_MANAGEMENT_RECEIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW13_ZONE_MANAGEMENT_RECEIVE_0 {
    pub fn ZRA(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_ZRA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn ZRASpecific(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_ZRASpecific(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn Partial(&self) -> super::ULONG {
        (self._bitfield >> 16) & 1
    }
    pub fn set_Partial(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value & 1) << 16);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 17
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(32767 << 17)) | ((value & 32767) << 17);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW13_ZONE_MANAGEMENT_SEND {
    pub Anonymous: NVME_CDW13_ZONE_MANAGEMENT_SEND_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW13_ZONE_MANAGEMENT_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW13_ZONE_MANAGEMENT_SEND_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW13_ZONE_MANAGEMENT_SEND_0 {
    pub fn ZSA(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_ZSA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn SelectAll(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_SelectAll(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 9
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(8388607 << 9)) | ((value & 8388607) << 9);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURES {
    pub HostMemoryBuffer: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW14_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HMDLUA: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW14_GET_LOG_PAGE {
    pub Anonymous: NVME_CDW14_GET_LOG_PAGE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW14_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_GET_LOG_PAGE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW14_GET_LOG_PAGE_0 {
    pub fn UUIDIndex(&self) -> super::ULONG {
        (self._bitfield) & 127
    }
    pub fn set_UUIDIndex(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 7) & 65535
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 7)) | ((value & 65535) << 7);
    }
    pub fn OT(&self) -> super::ULONG {
        (self._bitfield >> 23) & 1
    }
    pub fn set_OT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value & 1) << 23);
    }
    pub fn CommandSetIdentifier(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_CommandSetIdentifier(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW14_GET_LOG_PAGE_V20 {
    pub Anonymous: NVME_CDW14_GET_LOG_PAGE_V20_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW14_GET_LOG_PAGE_V20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_GET_LOG_PAGE_V20_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW14_GET_LOG_PAGE_V20_0 {
    pub fn UUIDIndex(&self) -> super::ULONG {
        (self._bitfield) & 127
    }
    pub fn set_UUIDIndex(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 7) & 65535
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 7)) | ((value & 65535) << 7);
    }
    pub fn OffsetType(&self) -> super::ULONG {
        (self._bitfield >> 23) & 1
    }
    pub fn set_OffsetType(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value & 1) << 23);
    }
    pub fn CommandSetIdentifier(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_CommandSetIdentifier(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW14_IDENTIFY {
    pub Anonymous: NVME_CDW14_IDENTIFY_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW14_IDENTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW14_IDENTIFY_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW14_IDENTIFY_0 {
    pub fn UUIDIndex(&self) -> super::ULONG {
        (self._bitfield) & 127
    }
    pub fn set_UUIDIndex(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURES {
    pub HostMemoryBuffer: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW15_FEATURES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    pub Anonymous: NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER_0 {
    pub HMDLEC: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW15_READ_WRITE {
    pub Anonymous: NVME_CDW15_READ_WRITE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW15_READ_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_READ_WRITE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW15_READ_WRITE_0 {
    pub fn ELBAT(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_ELBAT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn ELBATM(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_ELBATM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW15_VERIFY_COMMAND {
    pub Anonymous: NVME_CDW15_VERIFY_COMMAND_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW15_VERIFY_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_VERIFY_COMMAND_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW15_VERIFY_COMMAND_0 {
    pub fn ELBAT(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_ELBAT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn ELBATM(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_ELBATM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CDW15_ZONE_APPEND {
    pub Anonymous: NVME_CDW15_ZONE_APPEND_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CDW15_ZONE_APPEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CDW15_ZONE_APPEND_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CDW15_ZONE_APPEND_0 {
    pub fn LBAT(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_LBAT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn LBATM(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_LBATM(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_CHANGED_NAMESPACE_LIST_LOG {
    pub NSID: [super::ULONG; 1024],
}
#[cfg(feature = "shared")]
impl Default for NVME_CHANGED_NAMESPACE_LIST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_CHANGED_ZONE_LIST_LOG {
    pub ZoneIdentifiersCount: super::USHORT,
    pub Reserved: [super::UCHAR; 6],
    pub ZoneIdentifier: [super::ULONGLONG; 511],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND {
    pub CDW0: NVME_COMMAND_DWORD0,
    pub NSID: super::ULONG,
    pub Reserved0: [super::ULONG; 2],
    pub MPTR: super::ULONGLONG,
    pub Anonymous: NVME_COMMAND_0,
    pub u: NVME_COMMAND_1,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_0 {
    pub Anonymous: NVME_COMMAND_0_0,
    pub SGL1: [super::ULONGLONG; 2],
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_0_0 {
    pub PRP1: super::ULONGLONG,
    pub PRP2: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_1_0 {
    pub CDW10: super::ULONG,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_1 {
    pub CDW10: NVME_CDW10_IDENTIFY,
    pub CDW11: NVME_CDW11_IDENTIFY,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub Anonymous: NVME_COMMAND_1_1_0,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1_1_0 {
    pub CDW14: super::ULONG,
    pub CDW14_V20: NVME_CDW14_IDENTIFY,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_10 {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_SEND,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_11 {
    pub CDW10: NVME_CDW10_SECURITY_SEND_RECEIVE,
    pub CDW11: NVME_CDW11_SECURITY_RECEIVE,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_1_12 {
    pub CDW10: NVME_CDW10_FIRMWARE_DOWNLOAD,
    pub CDW11: NVME_CDW11_FIRMWARE_DOWNLOAD,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_13 {
    pub CDW10: NVME_CDW10_FIRMWARE_ACTIVATE,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_14 {
    pub CDW10: NVME_CDW10_FORMAT_NVM,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_15 {
    pub CDW10: NVME_CDW10_DIRECTIVE_RECEIVE,
    pub CDW11: NVME_CDW11_DIRECTIVE_RECEIVE,
    pub CDW12: NVME_CDW12_DIRECTIVE_RECEIVE,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_16 {
    pub CDW10: NVME_CDW10_DIRECTIVE_SEND,
    pub CDW11: NVME_CDW11_DIRECTIVE_SEND,
    pub CDW12: NVME_CDW12_DIRECTIVE_SEND,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_17 {
    pub CDW10: NVME_CDW10_SANITIZE,
    pub CDW11: NVME_CDW11_SANITIZE,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_18 {
    pub LBALOW: super::ULONG,
    pub LBAHIGH: super::ULONG,
    pub CDW12: NVME_CDW12_READ_WRITE,
    pub CDW13: NVME_CDW13_READ_WRITE,
    pub CDW14: super::ULONG,
    pub CDW15: NVME_CDW15_READ_WRITE,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_19 {
    pub CDW10: NVME_CDW10_RESERVATION_ACQUIRE,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_2 {
    pub CDW10: NVME_CDW10_ABORT,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_20 {
    pub CDW10: NVME_CDW10_RESERVATION_REGISTER,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_21 {
    pub CDW10: NVME_CDW10_RESERVATION_RELEASE,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_22 {
    pub CDW10: NVME_CDW10_RESERVATION_REPORT,
    pub CDW11: NVME_CDW11_RESERVATION_REPORT,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_23 {
    pub CDW1011: NVME_CDW10_ZONE_MANAGEMENT_SEND,
    pub CDW12: super::ULONG,
    pub CDW13: NVME_CDW13_ZONE_MANAGEMENT_SEND,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_24 {
    pub CDW1011: NVME_CDW10_ZONE_MANAGEMENT_RECEIVE,
    pub DWORDCOUNT: super::ULONG,
    pub CDW13: NVME_CDW13_ZONE_MANAGEMENT_RECEIVE,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_25 {
    pub CDW1011: NVME_CDW10_ZONE_APPEND,
    pub CDW12: NVME_CDW12_ZONE_APPEND,
    pub CDW13: super::ULONG,
    pub ILBRT: super::ULONG,
    pub CDW15: NVME_CDW15_ZONE_APPEND,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_26 {
    pub CDW10: NVME_CDW10_DEVICE_SELF_TEST,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_26 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_27 {
    pub CDW10: NVME_CDW10_DISCOVERY_INFO_MGMT,
    pub CDW11: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_27 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_1_28 {
    pub NDT: super::ULONG,
    pub NDM: super::ULONG,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_29 {
    pub LBALOW: super::ULONG,
    pub LBAHIGH: super::ULONG,
    pub CDW12: NVME_CDW12_VERIFYCOMMAND,
    pub CDW13: super::ULONG,
    pub EILBRT: super::ULONG,
    pub CDW15: NVME_CDW15_VERIFY_COMMAND,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_29 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_3 {
    pub CDW10: NVME_CDW10_GET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_4 {
    pub CDW10: NVME_CDW10_SET_FEATURES,
    pub CDW11: NVME_CDW11_FEATURES,
    pub CDW12: NVME_CDW12_FEATURES,
    pub CDW13: NVME_CDW13_FEATURES,
    pub CDW14: NVME_CDW14_FEATURES,
    pub CDW15: NVME_CDW15_FEATURES,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_5 {
    pub Anonymous: NVME_COMMAND_1_5_0,
    pub CDW11: NVME_CDW11_GET_LOG_PAGE,
    pub CDW12: NVME_CDW12_GET_LOG_PAGE,
    pub CDW13: NVME_CDW13_GET_LOG_PAGE,
    pub Anonymous2: NVME_COMMAND_1_5_1,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1_5_0 {
    pub CDW10: NVME_CDW10_GET_LOG_PAGE,
    pub CDW10_V121: NVME_CDW10_GET_LOG_PAGE_V121,
    pub CDW10_V13: NVME_CDW10_GET_LOG_PAGE_V13,
    pub CDW10_V20: NVME_CDW10_GET_LOG_PAGE_V20,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_1_5_1 {
    pub CDW14: NVME_CDW14_GET_LOG_PAGE,
    pub CDW14_V20: NVME_CDW14_GET_LOG_PAGE_V20,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_6 {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_CQ,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_7 {
    pub CDW10: NVME_CDW10_CREATE_IO_QUEUE,
    pub CDW11: NVME_CDW11_CREATE_IO_SQ,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_8 {
    pub CDW10: NVME_CDW10_DELETE_IO_QUEUE,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_1_9 {
    pub CDW10: NVME_CDW10_DATASET_MANAGEMENT,
    pub CDW11: NVME_CDW11_DATASET_MANAGEMENT,
    pub CDW12: super::ULONG,
    pub CDW13: super::ULONG,
    pub CDW14: super::ULONG,
    pub CDW15: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_1_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_DWORD0 {
    pub Anonymous: NVME_COMMAND_DWORD0_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_DWORD0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_DWORD0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_COMMAND_DWORD0_0 {
    pub fn OPC(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_OPC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn FUSE(&self) -> super::ULONG {
        (self._bitfield >> 8) & 3
    }
    pub fn set_FUSE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 8)) | ((value & 3) << 8);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 10) & 15
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 10)) | ((value & 15) << 10);
    }
    pub fn PSDT(&self) -> super::ULONG {
        (self._bitfield >> 14) & 3
    }
    pub fn set_PSDT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
    pub fn CID(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_CID(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_EFFECTS_DATA {
    pub Anonymous: NVME_COMMAND_EFFECTS_DATA_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_EFFECTS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_EFFECTS_DATA_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_COMMAND_EFFECTS_DATA_0 {
    pub fn CSUPP(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_CSUPP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn LBCC(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_LBCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn NCC(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_NCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn NIC(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_NIC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn CCC(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_CCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 5) & 2047
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
    pub fn CSE(&self) -> super::ULONG {
        (self._bitfield >> 16) & 7
    }
    pub fn set_CSE(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 16)) | ((value & 7) << 16);
    }
    pub fn UUIDSelectionSupported(&self) -> super::ULONG {
        (self._bitfield >> 19) & 1
    }
    pub fn set_UUIDSelectionSupported(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value & 1) << 19);
    }
    pub fn CSPNamespace(&self) -> super::ULONG {
        (self._bitfield >> 20) & 1
    }
    pub fn set_CSPNamespace(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value & 1) << 20);
    }
    pub fn CSPController(&self) -> super::ULONG {
        (self._bitfield >> 21) & 1
    }
    pub fn set_CSPController(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value & 1) << 21);
    }
    pub fn CSPNVMSet(&self) -> super::ULONG {
        (self._bitfield >> 22) & 1
    }
    pub fn set_CSPNVMSet(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value & 1) << 22);
    }
    pub fn CSPEnduranceGroup(&self) -> super::ULONG {
        (self._bitfield >> 23) & 1
    }
    pub fn set_CSPEnduranceGroup(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value & 1) << 23);
    }
    pub fn CSPDomain(&self) -> super::ULONG {
        (self._bitfield >> 24) & 1
    }
    pub fn set_CSPDomain(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value & 1) << 24);
    }
    pub fn CSPNVMSubsystem(&self) -> super::ULONG {
        (self._bitfield >> 25) & 1
    }
    pub fn set_CSPNVMSubsystem(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value & 1) << 25);
    }
    pub fn CSPReserved(&self) -> super::ULONG {
        self._bitfield >> 26
    }
    pub fn set_CSPReserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(63 << 26)) | ((value & 63) << 26);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMMAND_EFFECTS_LOG {
    pub ACS: [NVME_COMMAND_EFFECTS_DATA; 256],
    pub IOCS: [NVME_COMMAND_EFFECTS_DATA; 256],
    pub Reserved: [super::UCHAR; 2048],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMMAND_STATUS {
    pub Anonymous: NVME_COMMAND_STATUS_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMMAND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMMAND_STATUS_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_COMMAND_STATUS_0 {
    pub fn P(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_P(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SC(&self) -> super::USHORT {
        (self._bitfield >> 1) & 255
    }
    pub fn set_SC(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(255 << 1)) | ((value & 255) << 1);
    }
    pub fn SCT(&self) -> super::USHORT {
        (self._bitfield >> 9) & 7
    }
    pub fn set_SCT(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(7 << 9)) | ((value & 7) << 9);
    }
    pub fn CRD(&self) -> super::USHORT {
        (self._bitfield >> 12) & 3
    }
    pub fn set_CRD(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(3 << 12)) | ((value & 3) << 12);
    }
    pub fn M(&self) -> super::USHORT {
        (self._bitfield >> 14) & 1
    }
    pub fn set_M(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn DNR(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_DNR(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST {
    pub fn AsyncEventType(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_AsyncEventType(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 3) & 31
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
    pub fn AsyncEventInfo(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_AsyncEventInfo(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn LogPage(&self) -> super::ULONG {
        (self._bitfield >> 16) & 255
    }
    pub fn set_LogPage(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES {
    pub Anonymous: NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0,
    pub AsUlong: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES_0 {
    pub fn NSA(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_NSA(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY {
    pub DW0: super::ULONG,
    pub DW1: super::ULONG,
    pub DW2: NVME_COMPLETION_ENTRY_0,
    pub DW3: NVME_COMPLETION_ENTRY_1,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMPLETION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_ENTRY_0 {
    pub Anonymous: NVME_COMPLETION_ENTRY_0_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMPLETION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_ENTRY_0_0 {
    pub SQHD: super::USHORT,
    pub SQID: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_ENTRY_1 {
    pub Anonymous: NVME_COMPLETION_ENTRY_1_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMPLETION_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_COMPLETION_ENTRY_1_0 {
    pub CID: super::USHORT,
    pub Status: NVME_COMMAND_STATUS,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMPLETION_ENTRY_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    pub Anonymous: NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_COMPLETION_QUEUE_HEAD_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_COMPLETION_QUEUE_HEAD_DOORBELL_0 {
    pub fn CQH(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_CQH(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTEXT_ATTRIBUTES {
    pub Anonymous: NVME_CONTEXT_ATTRIBUTES_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTEXT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTEXT_ATTRIBUTES_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CONTEXT_ATTRIBUTES_0 {
    pub fn AccessFrequency(&self) -> super::ULONG {
        (self._bitfield) & 15
    }
    pub fn set_AccessFrequency(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn AccessLatency(&self) -> super::ULONG {
        (self._bitfield >> 4) & 3
    }
    pub fn set_AccessLatency(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 6) & 3
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn SequentialReadRange(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_SequentialReadRange(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn SequentialWriteRange(&self) -> super::ULONG {
        (self._bitfield >> 9) & 1
    }
    pub fn set_SequentialWriteRange(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn WritePrepare(&self) -> super::ULONG {
        (self._bitfield >> 10) & 1
    }
    pub fn set_WritePrepare(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        (self._bitfield >> 11) & 8191
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(8191 << 11)) | ((value & 8191) << 11);
    }
    pub fn CommandAccessSize(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_CommandAccessSize(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_CAPABILITIES {
    pub Anonymous: NVME_CONTROLLER_CAPABILITIES_0,
    pub AsUlonglong: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_CAPABILITIES_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl NVME_CONTROLLER_CAPABILITIES_0 {
    pub fn MQES(&self) -> super::ULONGLONG {
        (self._bitfield) & 65535
    }
    pub fn set_MQES(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn CQR(&self) -> super::ULONGLONG {
        (self._bitfield >> 16) & 1
    }
    pub fn set_CQR(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value & 1) << 16);
    }
    pub fn AMS_WeightedRoundRobinWithUrgent(&self) -> super::ULONGLONG {
        (self._bitfield >> 17) & 1
    }
    pub fn set_AMS_WeightedRoundRobinWithUrgent(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value & 1) << 17);
    }
    pub fn AMS_VendorSpecific(&self) -> super::ULONGLONG {
        (self._bitfield >> 18) & 1
    }
    pub fn set_AMS_VendorSpecific(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value & 1) << 18);
    }
    pub fn Reserved0(&self) -> super::ULONGLONG {
        (self._bitfield >> 19) & 31
    }
    pub fn set_Reserved0(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(31 << 19)) | ((value & 31) << 19);
    }
    pub fn TO(&self) -> super::ULONGLONG {
        (self._bitfield >> 24) & 255
    }
    pub fn set_TO(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
    pub fn DSTRD(&self) -> super::ULONGLONG {
        (self._bitfield >> 32) & 15
    }
    pub fn set_DSTRD(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(15 << 32)) | ((value & 15) << 32);
    }
    pub fn NSSRS(&self) -> super::ULONGLONG {
        (self._bitfield >> 36) & 1
    }
    pub fn set_NSSRS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 36)) | ((value & 1) << 36);
    }
    pub fn CSS_NVM(&self) -> super::ULONGLONG {
        (self._bitfield >> 37) & 1
    }
    pub fn set_CSS_NVM(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 37)) | ((value & 1) << 37);
    }
    pub fn CSS_Reserved0(&self) -> super::ULONGLONG {
        (self._bitfield >> 38) & 1
    }
    pub fn set_CSS_Reserved0(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 38)) | ((value & 1) << 38);
    }
    pub fn CSS_Reserved1(&self) -> super::ULONGLONG {
        (self._bitfield >> 39) & 1
    }
    pub fn set_CSS_Reserved1(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 39)) | ((value & 1) << 39);
    }
    pub fn CSS_Reserved2(&self) -> super::ULONGLONG {
        (self._bitfield >> 40) & 1
    }
    pub fn set_CSS_Reserved2(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 40)) | ((value & 1) << 40);
    }
    pub fn CSS_Reserved3(&self) -> super::ULONGLONG {
        (self._bitfield >> 41) & 1
    }
    pub fn set_CSS_Reserved3(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 41)) | ((value & 1) << 41);
    }
    pub fn CSS_Reserved4(&self) -> super::ULONGLONG {
        (self._bitfield >> 42) & 1
    }
    pub fn set_CSS_Reserved4(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 42)) | ((value & 1) << 42);
    }
    pub fn CSS_MultipleIo(&self) -> super::ULONGLONG {
        (self._bitfield >> 43) & 1
    }
    pub fn set_CSS_MultipleIo(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 43)) | ((value & 1) << 43);
    }
    pub fn CSS_AdminOnly(&self) -> super::ULONGLONG {
        (self._bitfield >> 44) & 1
    }
    pub fn set_CSS_AdminOnly(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 44)) | ((value & 1) << 44);
    }
    pub fn BPS(&self) -> super::ULONGLONG {
        (self._bitfield >> 45) & 1
    }
    pub fn set_BPS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 45)) | ((value & 1) << 45);
    }
    pub fn CPS(&self) -> super::ULONGLONG {
        (self._bitfield >> 46) & 3
    }
    pub fn set_CPS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(3 << 46)) | ((value & 3) << 46);
    }
    pub fn MPSMIN(&self) -> super::ULONGLONG {
        (self._bitfield >> 48) & 15
    }
    pub fn set_MPSMIN(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(15 << 48)) | ((value & 15) << 48);
    }
    pub fn MPSMAX(&self) -> super::ULONGLONG {
        (self._bitfield >> 52) & 15
    }
    pub fn set_MPSMAX(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(15 << 52)) | ((value & 15) << 52);
    }
    pub fn PMRS(&self) -> super::ULONGLONG {
        (self._bitfield >> 56) & 1
    }
    pub fn set_PMRS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 56)) | ((value & 1) << 56);
    }
    pub fn CMBS(&self) -> super::ULONGLONG {
        (self._bitfield >> 57) & 1
    }
    pub fn set_CMBS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 57)) | ((value & 1) << 57);
    }
    pub fn NSSS(&self) -> super::ULONGLONG {
        (self._bitfield >> 58) & 1
    }
    pub fn set_NSSS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 58)) | ((value & 1) << 58);
    }
    pub fn CRWMS(&self) -> super::ULONGLONG {
        (self._bitfield >> 59) & 1
    }
    pub fn set_CRWMS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 59)) | ((value & 1) << 59);
    }
    pub fn CRIMS(&self) -> super::ULONGLONG {
        (self._bitfield >> 60) & 1
    }
    pub fn set_CRIMS(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 60)) | ((value & 1) << 60);
    }
    pub fn Reserved2(&self) -> super::ULONGLONG {
        self._bitfield >> 61
    }
    pub fn set_Reserved2(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(7 << 61)) | ((value & 7) << 61);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_CONFIGURATION {
    pub Anonymous: NVME_CONTROLLER_CONFIGURATION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_CONFIGURATION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CONTROLLER_CONFIGURATION_0 {
    pub fn EN(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_EN(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 1) & 7
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn CSS(&self) -> super::ULONG {
        (self._bitfield >> 4) & 7
    }
    pub fn set_CSS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 4)) | ((value & 7) << 4);
    }
    pub fn MPS(&self) -> super::ULONG {
        (self._bitfield >> 7) & 15
    }
    pub fn set_MPS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 7)) | ((value & 15) << 7);
    }
    pub fn AMS(&self) -> super::ULONG {
        (self._bitfield >> 11) & 7
    }
    pub fn set_AMS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 11)) | ((value & 7) << 11);
    }
    pub fn SHN(&self) -> super::ULONG {
        (self._bitfield >> 14) & 3
    }
    pub fn set_SHN(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 14)) | ((value & 3) << 14);
    }
    pub fn IOSQES(&self) -> super::ULONG {
        (self._bitfield >> 16) & 15
    }
    pub fn set_IOSQES(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 16)) | ((value & 15) << 16);
    }
    pub fn IOCQES(&self) -> super::ULONG {
        (self._bitfield >> 20) & 15
    }
    pub fn set_IOCQES(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 20)) | ((value & 15) << 20);
    }
    pub fn CRIME(&self) -> super::ULONG {
        (self._bitfield >> 24) & 1
    }
    pub fn set_CRIME(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value & 1) << 24);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 25
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(127 << 25)) | ((value & 127) << 25);
    }
}
pub const NVME_CONTROLLER_ID_DYN: i32 = 65535;
pub const NVME_CONTROLLER_ID_MAX: i32 = 65519;
pub const NVME_CONTROLLER_ID_MIN: i32 = 0;
pub const NVME_CONTROLLER_ID_STAT_PERSIST: i32 = 65534;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_CONTROLLER_LIST {
    pub NumberOfIdentifiers: super::USHORT,
    pub ControllerID: [super::USHORT; 2047],
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    pub Anonymous: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_MEMORY_BUFFER_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CONTROLLER_MEMORY_BUFFER_LOCATION_0 {
    pub fn BIR(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_BIR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 3) & 511
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(511 << 3)) | ((value & 511) << 3);
    }
    pub fn OFST(&self) -> super::ULONG {
        self._bitfield >> 12
    }
    pub fn set_OFST(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1048575 << 12)) | ((value & 1048575) << 12);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    pub Anonymous: NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_MEMORY_BUFFER_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CONTROLLER_MEMORY_BUFFER_SIZE_0 {
    pub fn SQS(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_SQS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CQS(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CQS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn LISTS(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_LISTS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn RDS(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_RDS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn WDS(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_WDS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 5) & 7
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn SZU(&self) -> super::ULONG {
        (self._bitfield >> 8) & 15
    }
    pub fn set_SZU(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn SZ(&self) -> super::ULONG {
        self._bitfield >> 12
    }
    pub fn set_SZ(&mut self, value: super::ULONG) {
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_READY_TIMEOUTS {
    pub Anonymous: NVME_CONTROLLER_READY_TIMEOUTS_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_READY_TIMEOUTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_READY_TIMEOUTS_0 {
    pub CRWMT: super::USHORT,
    pub CRIMT: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_CONTROLLER_REGISTERS {
    pub CAP: NVME_CONTROLLER_CAPABILITIES,
    pub VS: NVME_VERSION,
    pub INTMS: super::ULONG,
    pub INTMC: super::ULONG,
    pub CC: NVME_CONTROLLER_CONFIGURATION,
    pub Reserved0: super::ULONG,
    pub CSTS: NVME_CONTROLLER_STATUS,
    pub NSSR: NVME_NVM_SUBSYSTEM_RESET,
    pub AQA: NVME_ADMIN_QUEUE_ATTRIBUTES,
    pub ASQ: NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS,
    pub ACQ: NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS,
    pub CMBLOC: NVME_CONTROLLER_MEMORY_BUFFER_LOCATION,
    pub CMBSZ: NVME_CONTROLLER_MEMORY_BUFFER_SIZE,
    pub BPINFO: NVME_BOOT_PARTITION_INFORMATION,
    pub Reserved1: [super::ULONG; 8],
    pub NSSD: NVME_NVM_SUBSYSTEM_SHUTDOWN,
    pub CRTO: NVME_CONTROLLER_READY_TIMEOUTS,
    pub Reserved2: [super::ULONG; 933],
    pub Reserved3: [super::ULONG; 64],
    pub Doorbells: [super::ULONG; 0],
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_REGISTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_CONTROLLER_STATUS {
    pub Anonymous: NVME_CONTROLLER_STATUS_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_CONTROLLER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_CONTROLLER_STATUS_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_CONTROLLER_STATUS_0 {
    pub fn RDY(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_RDY(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CFS(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CFS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn SHST(&self) -> super::ULONG {
        (self._bitfield >> 2) & 3
    }
    pub fn set_SHST(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 2)) | ((value & 3) << 2);
    }
    pub fn NSSRO(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_NSSRO(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn PP(&self) -> super::ULONG {
        (self._bitfield >> 5) & 1
    }
    pub fn set_PP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn ST(&self) -> super::ULONG {
        (self._bitfield >> 6) & 1
    }
    pub fn set_ST(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 7
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
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
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_DEVICE_SELF_TEST_LOG {
    pub CurrentOperation: NVME_DEVICE_SELF_TEST_LOG_0,
    pub CurrentCompletion: NVME_DEVICE_SELF_TEST_LOG_1,
    pub Reserved: [super::UCHAR; 2],
    pub ResultData: [NVME_DEVICE_SELF_TEST_RESULT_DATA; 20],
}
#[cfg(feature = "shared")]
impl Default for NVME_DEVICE_SELF_TEST_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DEVICE_SELF_TEST_LOG_0 {
    pub fn Status(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_Status(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_LOG_1 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DEVICE_SELF_TEST_LOG_1 {
    pub fn CompletePercent(&self) -> super::UCHAR {
        (self._bitfield) & 127
    }
    pub fn set_CompletePercent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA {
    pub Status: NVME_DEVICE_SELF_TEST_RESULT_DATA_0,
    pub SegmentNumber: super::UCHAR,
    pub ValidDiagnostics: NVME_DEVICE_SELF_TEST_RESULT_DATA_1,
    pub Reserved: super::UCHAR,
    pub POH: super::ULONGLONG,
    pub NSID: super::ULONG,
    pub FailingLBA: super::ULONGLONG,
    pub StatusCodeType: NVME_DEVICE_SELF_TEST_RESULT_DATA_2,
    pub StatusCode: super::UCHAR,
    pub VendorSpecific: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DEVICE_SELF_TEST_RESULT_DATA_0 {
    pub fn Result(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_Result(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn CodeValue(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_CodeValue(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_1 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DEVICE_SELF_TEST_RESULT_DATA_1 {
    pub fn NSIDValid(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_NSIDValid(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn FLBAValid(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_FLBAValid(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn SCTValid(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_SCTValid(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn SCValid(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_SCValid(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_DEVICE_SELF_TEST_RESULT_DATA_2 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DEVICE_SELF_TEST_RESULT_DATA_2 {
    pub fn AdditionalInfo(&self) -> super::UCHAR {
        (self._bitfield) & 7
    }
    pub fn set_AdditionalInfo(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS {
    pub DirectivesSupported: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
    pub DirectivesEnabled: NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    pub _bitfield: u8,
    pub Reserved1: [super::UCHAR; 31],
}
#[cfg(feature = "shared")]
impl NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR {
    pub fn Identify(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Identify(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Streams(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Streams(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    pub OpenStreamCount: super::USHORT,
    pub StreamIdentifiers: [super::USHORT; 65535],
}
#[cfg(feature = "shared")]
impl Default for NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    pub MSL: super::USHORT,
    pub NSSA: super::USHORT,
    pub NSSO: super::USHORT,
    pub Reserved0: [super::UCHAR; 10],
    pub SWS: super::ULONG,
    pub SGS: super::USHORT,
    pub NSA: super::USHORT,
    pub NSO: super::USHORT,
    pub Reserved1: [super::UCHAR; 6],
}
#[cfg(feature = "shared")]
impl Default for NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_DIRECTIVE_TYPES = i32;
pub const NVME_DIRECTIVE_TYPE_IDENTIFY: NVME_DIRECTIVE_TYPES = 0;
pub const NVME_DIRECTIVE_TYPE_STREAMS: NVME_DIRECTIVE_TYPES = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_DISCOVERY_ENTRY {
    pub TRTYPE: super::UCHAR,
    pub ADRFAM: super::UCHAR,
    pub SUBTYPE: super::UCHAR,
    pub TREQ: NVME_DISCOVERY_ENTRY_0,
    pub PORTID: super::USHORT,
    pub CNTLID: super::USHORT,
    pub ASQSZ: super::USHORT,
    pub EFLAGS: NVME_DISCOVERY_ENTRY_1,
    pub Reserved0: [super::UCHAR; 20],
    pub TRSVCID: [super::UCHAR; 32],
    pub Reserved1: [super::UCHAR; 192],
    pub NQN: [super::UCHAR; 256],
    pub TRADDR: [super::UCHAR; 256],
    pub TSAS: [super::UCHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_ENTRY_0 {
    pub Anonymous: NVME_DISCOVERY_ENTRY_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_ENTRY_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DISCOVERY_ENTRY_0_0 {
    pub fn SecureChannel(&self) -> super::UCHAR {
        (self._bitfield) & 3
    }
    pub fn set_SecureChannel(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn SqFlowControlDisable(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_SqFlowControlDisable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ZeroHostIdSupport(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ZeroHostIdSupport(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn AuthAndSecureChannel(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 3
    }
    pub fn set_AuthAndSecureChannel(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_ENTRY_1 {
    pub Anonymous: NVME_DISCOVERY_ENTRY_1_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_ENTRY_1_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_DISCOVERY_ENTRY_1_0 {
    pub fn DuplicateReturnedInfo(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_DuplicateReturnedInfo(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ExplicitPersistentConnectionSupport(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ExplicitPersistentConnectionSupport(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn NoCDCConnectivity(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_NoCDCConnectivity(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_DISCOVERY_HEADER {
    pub GENCTR: super::ULONGLONG,
    pub NUMREC: super::ULONGLONG,
    pub RECFMT: super::USHORT,
    pub DLPF: NVME_DISCOVERY_HEADER_0,
    pub Reserved0: super::UCHAR,
    pub TDLPL: super::ULONG,
    pub Reserved1: [super::UCHAR; 1000],
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_HEADER_0 {
    pub Anonymous: NVME_DISCOVERY_HEADER_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_HEADER_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_DISCOVERY_HEADER_0_0 {
    pub fn Extended(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Extended(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn PortLocal(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_PortLocal(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn AllSubsystems(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_AllSubsystems(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
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
pub const NVME_DISCOVERY_INFO_MGMT_EKTYPE_PORTID: i32 = 63;
pub const NVME_DISCOVERY_INFO_MGMT_EKTYPE_TRADDR: i32 = 95;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_DISCOVERY_INFO_MGMT_HEADER {
    pub TDL: super::ULONG,
    pub Reserved0: super::ULONG,
    pub NUMENT: super::ULONGLONG,
    pub ENTFMT: super::USHORT,
    pub ETYPE: super::USHORT,
    pub PORTLCL: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub EKTYPE: NVME_DISCOVERY_INFO_MGMT_HEADER_0,
    pub EID: [super::UCHAR; 256],
    pub ENAME: [super::UCHAR; 256],
    pub EVER: [super::UCHAR; 64],
    pub Reserved2: [super::UCHAR; 424],
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_INFO_MGMT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_DISCOVERY_INFO_MGMT_HEADER_0 {
    pub Anonymous: NVME_DISCOVERY_INFO_MGMT_HEADER_0_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_DISCOVERY_INFO_MGMT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_DISCOVERY_INFO_MGMT_HEADER_0_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_DISCOVERY_INFO_MGMT_HEADER_0_0 {
    pub fn NQN(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_NQN(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn TSAS(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_TSAS(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn TRSVCID(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_TRSVCID(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ADRFAM(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ADRFAM(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn TRTYPE(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_TRTYPE(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn PORTID(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_PORTID(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn TRADDR(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_TRADDR(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(511 << 7)) | ((value & 511) << 7);
    }
}
pub type NVME_DISCOVERY_INFO_MGMT_TASK = i32;
pub const NVME_DISCOVERY_INFO_MGMT_TASK_DEREGISTER: NVME_DISCOVERY_INFO_MGMT_TASK = 1;
pub const NVME_DISCOVERY_INFO_MGMT_TASK_REGISTER: NVME_DISCOVERY_INFO_MGMT_TASK = 0;
pub const NVME_DISCOVERY_INFO_MGMT_TASK_UPDATE: NVME_DISCOVERY_INFO_MGMT_TASK = 2;
pub type NVME_DISC_CTRL_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_ENDURANCE_GROUP_LOG {
    pub Reserved0: super::ULONG,
    pub AvailableSpareThreshold: super::UCHAR,
    pub PercentageUsed: super::UCHAR,
    pub Reserved1: [super::UCHAR; 26],
    pub EnduranceEstimate: [super::UCHAR; 16],
    pub DataUnitsRead: [super::UCHAR; 16],
    pub DataUnitsWritten: [super::UCHAR; 16],
    pub MediaUnitsWritten: [super::UCHAR; 16],
    pub Reserved2: [super::UCHAR; 416],
}
#[cfg(feature = "shared")]
impl Default for NVME_ENDURANCE_GROUP_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_ERROR_INFO_LOG {
    pub ErrorCount: super::ULONGLONG,
    pub SQID: super::USHORT,
    pub CMDID: super::USHORT,
    pub Status: NVME_COMMAND_STATUS,
    pub ParameterErrorLocation: NVME_ERROR_INFO_LOG_0,
    pub Lba: super::ULONGLONG,
    pub NameSpace: super::ULONG,
    pub VendorInfoAvailable: super::UCHAR,
    pub TRTYPE: super::UCHAR,
    pub Reserved0: [super::UCHAR; 2],
    pub CommandSpecificInfo: super::ULONGLONG,
    pub TransportTypeSpecificInfo: super::USHORT,
    pub Reserved1: [super::UCHAR; 22],
}
#[cfg(feature = "shared")]
impl Default for NVME_ERROR_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ERROR_INFO_LOG_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_ERROR_INFO_LOG_0 {
    pub fn Byte(&self) -> super::USHORT {
        (self._bitfield) & 255
    }
    pub fn set_Byte(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn Bit(&self) -> super::USHORT {
        (self._bitfield >> 8) & 7
    }
    pub fn set_Bit(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(7 << 8)) | ((value & 7) << 8);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 11
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(31 << 11)) | ((value & 31) << 11);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_ERROR_INJECTION_ENTRY {
    pub Flags: NVME_ERROR_INJECTION_ENTRY_0,
    pub Reserved1: super::UCHAR,
    pub ErrorInjectionType: super::USHORT,
    pub ErrorInjectionTypeSpecific: [super::UCHAR; 28],
}
#[cfg(feature = "shared")]
impl Default for NVME_ERROR_INJECTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_ERROR_INJECTION_ENTRY_0 {
    pub Anonymous: NVME_ERROR_INJECTION_ENTRY_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_ERROR_INJECTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ERROR_INJECTION_ENTRY_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_ERROR_INJECTION_ENTRY_0_0 {
    pub fn Enable(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Enable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SingleInstance(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_SingleInstance(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_EXTENDED_ATTR {
    pub EXATTYPE: super::USHORT,
    pub EXATLEN: super::USHORT,
    pub EXATVAL: [super::UCHAR; 0],
}
#[cfg(feature = "shared")]
impl Default for NVME_EXTENDED_ATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_EXTENDED_ATTR_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_EXTENDED_DISCOVERY_ENTRY {
    pub TRTYPE: super::UCHAR,
    pub ADRFAM: super::UCHAR,
    pub SUBTYPE: super::UCHAR,
    pub TREQ: NVME_EXTENDED_DISCOVERY_ENTRY_0,
    pub PORTID: super::USHORT,
    pub CNTLID: super::USHORT,
    pub ASQSZ: super::USHORT,
    pub EFLAGS: NVME_EXTENDED_DISCOVERY_ENTRY_1,
    pub Reserved0: [super::UCHAR; 20],
    pub TRSVCID: [super::UCHAR; 32],
    pub Reserved1: [super::UCHAR; 192],
    pub NQN: [super::UCHAR; 256],
    pub TRADDR: [super::UCHAR; 256],
    pub TSAS: [super::UCHAR; 256],
    pub TEL: super::ULONG,
    pub NUMEXAT: super::USHORT,
    pub Reserved2: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_EXTENDED_DISCOVERY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_EXTENDED_DISCOVERY_ENTRY_0 {
    pub Anonymous: NVME_EXTENDED_DISCOVERY_ENTRY_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_EXTENDED_DISCOVERY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_EXTENDED_DISCOVERY_ENTRY_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_EXTENDED_DISCOVERY_ENTRY_0_0 {
    pub fn SecureChannel(&self) -> super::UCHAR {
        (self._bitfield) & 3
    }
    pub fn set_SecureChannel(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn SqFlowControlDisable(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_SqFlowControlDisable(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ZeroHostIdSupport(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ZeroHostIdSupport(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn AuthAndSecureChannel(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 3
    }
    pub fn set_AuthAndSecureChannel(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_EXTENDED_DISCOVERY_ENTRY_1 {
    pub Anonymous: NVME_EXTENDED_DISCOVERY_ENTRY_1_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_EXTENDED_DISCOVERY_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_EXTENDED_DISCOVERY_ENTRY_1_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_EXTENDED_DISCOVERY_ENTRY_1_0 {
    pub fn DuplicateReturnedInfo(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_DuplicateReturnedInfo(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ExplicitPersistentConnectionSupport(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ExplicitPersistentConnectionSupport(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn NoCDCConnectivity(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_NoCDCConnectivity(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
}
pub const NVME_EXTENDED_HOST_IDENTIFIER_SIZE: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_EXTENDED_REPORT_ZONE_INFO {
    pub ZoneCount: super::ULONGLONG,
    pub Reserved: [super::ULONGLONG; 7],
    pub Desc: [NVME_ZONE_EXTENDED_REPORT_ZONE_DESC; 1],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FEATURE_HOST_IDENTIFIER_DATA {
    pub HOSTID: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for NVME_FEATURE_HOST_IDENTIFIER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_FEATURE_HOST_MEMORY_BUFFER: NVME_FEATURES = 13;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FEATURE_HOST_METADATA_DATA {
    pub NumberOfMetadataElementDescriptors: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub MetadataElementDescriptors: [super::UCHAR; 4094],
}
#[cfg(feature = "shared")]
impl Default for NVME_FEATURE_HOST_METADATA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FEATURE_IDENTIFIERS_EFFECTS_LOG {
    pub FeatureIdentifierSupported: [NVME_FID_SUPPORTED_AND_EFFECTS; 256],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_FID_SUPPORTED_AND_EFFECTS {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_FID_SUPPORTED_AND_EFFECTS {
    pub fn FSUPP(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_FSUPP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn UDCC(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_UDCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn NCC(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_NCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn NIC(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_NIC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn CCC(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_CCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 5) & 16383
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(16383 << 5)) | ((value & 16383) << 5);
    }
    pub fn UUIDSelectionSupported(&self) -> super::ULONG {
        (self._bitfield >> 19) & 1
    }
    pub fn set_UUIDSelectionSupported(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value & 1) << 19);
    }
    pub fn FSPNamespace(&self) -> super::ULONG {
        (self._bitfield >> 20) & 1
    }
    pub fn set_FSPNamespace(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value & 1) << 20);
    }
    pub fn FSPController(&self) -> super::ULONG {
        (self._bitfield >> 21) & 1
    }
    pub fn set_FSPController(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value & 1) << 21);
    }
    pub fn FSPNVMSet(&self) -> super::ULONG {
        (self._bitfield >> 22) & 1
    }
    pub fn set_FSPNVMSet(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value & 1) << 22);
    }
    pub fn FSPEnduranceGroup(&self) -> super::ULONG {
        (self._bitfield >> 23) & 1
    }
    pub fn set_FSPEnduranceGroup(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value & 1) << 23);
    }
    pub fn FSPDomain(&self) -> super::ULONG {
        (self._bitfield >> 24) & 1
    }
    pub fn set_FSPDomain(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value & 1) << 24);
    }
    pub fn FSPNVMSubsystem(&self) -> super::ULONG {
        (self._bitfield >> 25) & 1
    }
    pub fn set_FSPNVMSubsystem(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value & 1) << 25);
    }
    pub fn FSPReserved(&self) -> super::ULONG {
        self._bitfield >> 26
    }
    pub fn set_FSPReserved(&mut self, value: super::ULONG) {
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG {
    pub AFI: NVME_FIRMWARE_SLOT_INFO_LOG_0,
    pub Reserved0: [super::UCHAR; 7],
    pub FRS: [super::ULONGLONG; 7],
    pub Reserved1: [super::UCHAR; 448],
}
#[cfg(feature = "shared")]
impl Default for NVME_FIRMWARE_SLOT_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_FIRMWARE_SLOT_INFO_LOG_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_FIRMWARE_SLOT_INFO_LOG_0 {
    pub fn ActiveSlot(&self) -> super::UCHAR {
        (self._bitfield) & 7
    }
    pub fn set_ActiveSlot(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn PendingActivateSlot(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 7
    }
    pub fn set_PendingActivateSlot(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 4)) | ((value & 7) << 4);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
pub type NVME_FUSED_OPERATION_CODES = i32;
pub const NVME_FUSED_OPERATION_FIRST_CMD: NVME_FUSED_OPERATION_CODES = 1;
pub const NVME_FUSED_OPERATION_NORMAL: NVME_FUSED_OPERATION_CODES = 0;
pub const NVME_FUSED_OPERATION_SECOND_CMD: NVME_FUSED_OPERATION_CODES = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_GET_FEATURE_TIMESTAMP {
    pub Anonymous: NVME_GET_FEATURE_TIMESTAMP_0,
    pub AsUlonglong: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_GET_FEATURE_TIMESTAMP_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl NVME_GET_FEATURE_TIMESTAMP_0 {
    pub fn Timestamp(&self) -> super::ULONGLONG {
        (self._bitfield) & 281474976710655
    }
    pub fn set_Timestamp(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !281474976710655) | (value & 281474976710655);
    }
    pub fn Synch(&self) -> super::ULONGLONG {
        (self._bitfield >> 48) & 1
    }
    pub fn set_Synch(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(1 << 48)) | ((value & 1) << 48);
    }
    pub fn Origin(&self) -> super::ULONGLONG {
        (self._bitfield >> 49) & 7
    }
    pub fn set_Origin(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(7 << 49)) | ((value & 7) << 49);
    }
    pub fn Reserved(&self) -> super::ULONGLONG {
        self._bitfield >> 52
    }
    pub fn set_Reserved(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(4095 << 52)) | ((value & 4095) << 52);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_HEALTH_INFO_LOG {
    pub CriticalWarning: NVME_HEALTH_INFO_LOG_0,
    pub Temperature: [super::UCHAR; 2],
    pub AvailableSpare: super::UCHAR,
    pub AvailableSpareThreshold: super::UCHAR,
    pub PercentageUsed: super::UCHAR,
    pub Reserved0: [super::UCHAR; 26],
    pub DataUnitRead: [super::UCHAR; 16],
    pub DataUnitWritten: [super::UCHAR; 16],
    pub HostReadCommands: [super::UCHAR; 16],
    pub HostWrittenCommands: [super::UCHAR; 16],
    pub ControllerBusyTime: [super::UCHAR; 16],
    pub PowerCycle: [super::UCHAR; 16],
    pub PowerOnHours: [super::UCHAR; 16],
    pub UnsafeShutdowns: [super::UCHAR; 16],
    pub MediaErrors: [super::UCHAR; 16],
    pub ErrorInfoLogEntryCount: [super::UCHAR; 16],
    pub WarningCompositeTemperatureTime: super::ULONG,
    pub CriticalCompositeTemperatureTime: super::ULONG,
    pub TemperatureSensor1: super::USHORT,
    pub TemperatureSensor2: super::USHORT,
    pub TemperatureSensor3: super::USHORT,
    pub TemperatureSensor4: super::USHORT,
    pub TemperatureSensor5: super::USHORT,
    pub TemperatureSensor6: super::USHORT,
    pub TemperatureSensor7: super::USHORT,
    pub TemperatureSensor8: super::USHORT,
    pub Reserved1: [super::UCHAR; 296],
}
#[cfg(feature = "shared")]
impl Default for NVME_HEALTH_INFO_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_HEALTH_INFO_LOG_0 {
    pub Anonymous: NVME_HEALTH_INFO_LOG_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_HEALTH_INFO_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_HEALTH_INFO_LOG_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_HEALTH_INFO_LOG_0_0 {
    pub fn AvailableSpaceLow(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_AvailableSpaceLow(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn TemperatureThreshold(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_TemperatureThreshold(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ReliabilityDegraded(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ReliabilityDegraded(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ReadOnly(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ReadOnly(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn VolatileMemoryBackupDeviceFailed(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_VolatileMemoryBackupDeviceFailed(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_HOST_BEHAVIOR_SUPPORT_DATA {
    pub ACRE: super::UCHAR,
    pub ETDAS: super::UCHAR,
    pub LBAFEE: super::UCHAR,
    pub Reserved: [super::UCHAR; 509],
}
#[cfg(feature = "shared")]
impl Default for NVME_HOST_BEHAVIOR_SUPPORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_HOST_IDENTIFIER_SIZE: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY {
    pub BADD: super::ULONGLONG,
    pub BSIZE: super::ULONG,
    pub Reserved: super::ULONG,
}
pub const NVME_HOST_METADATA_ADD_ENTRY_MULTIPLE: NVME_HOST_METADATA_ELEMENT_ACTIONS = 2;
pub const NVME_HOST_METADATA_ADD_REPLACE_ENTRY: NVME_HOST_METADATA_ELEMENT_ACTIONS = 0;
pub const NVME_HOST_METADATA_DELETE_ENTRY_MULTIPLE: NVME_HOST_METADATA_ELEMENT_ACTIONS = 1;
pub type NVME_HOST_METADATA_ELEMENT_ACTIONS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    pub _bitfield: u32,
    pub EVAL: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl NVME_HOST_METADATA_ELEMENT_DESCRIPTOR {
    pub fn ET(&self) -> super::ULONG {
        (self._bitfield) & 63
    }
    pub fn set_ET(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !63) | (value & 63);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 6) & 3
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn ER(&self) -> super::ULONG {
        (self._bitfield >> 8) & 15
    }
    pub fn set_ER(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 8)) | ((value & 15) << 8);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        (self._bitfield >> 12) & 15
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(15 << 12)) | ((value & 15) << 12);
    }
    pub fn ELEN(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_ELEN(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[cfg(feature = "shared")]
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
pub const NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE_SIZE: i32 = 4096;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA {
    pub VID: super::USHORT,
    pub SSVID: super::USHORT,
    pub SN: [super::UCHAR; 20],
    pub MN: [super::UCHAR; 40],
    pub FR: [super::UCHAR; 8],
    pub RAB: super::UCHAR,
    pub IEEE: [super::UCHAR; 3],
    pub CMIC: NVME_IDENTIFY_CONTROLLER_DATA_0,
    pub MDTS: super::UCHAR,
    pub CNTLID: super::USHORT,
    pub VER: super::ULONG,
    pub RTD3R: super::ULONG,
    pub RTD3E: super::ULONG,
    pub OAES: NVME_IDENTIFY_CONTROLLER_DATA_1,
    pub CTRATT: NVME_IDENTIFY_CONTROLLER_DATA_2,
    pub RRLS: NVME_IDENTIFY_CONTROLLER_DATA_3,
    pub Reserved0: [super::UCHAR; 9],
    pub CNTRLTYPE: super::UCHAR,
    pub FGUID: [super::UCHAR; 16],
    pub CRDT1: super::USHORT,
    pub CRDT2: super::USHORT,
    pub CRDT3: super::USHORT,
    pub Reserved1: [super::UCHAR; 106],
    pub ReservedForManagement: [super::UCHAR; 13],
    pub NVMSR: super::UCHAR,
    pub VWCI: super::UCHAR,
    pub MEC: super::UCHAR,
    pub OACS: NVME_IDENTIFY_CONTROLLER_DATA_4,
    pub ACL: super::UCHAR,
    pub AERL: super::UCHAR,
    pub FRMW: NVME_IDENTIFY_CONTROLLER_DATA_5,
    pub LPA: NVME_IDENTIFY_CONTROLLER_DATA_6,
    pub ELPE: super::UCHAR,
    pub NPSS: super::UCHAR,
    pub AVSCC: NVME_IDENTIFY_CONTROLLER_DATA_7,
    pub APSTA: NVME_IDENTIFY_CONTROLLER_DATA_8,
    pub WCTEMP: super::USHORT,
    pub CCTEMP: super::USHORT,
    pub MTFA: super::USHORT,
    pub HMPRE: super::ULONG,
    pub HMMIN: super::ULONG,
    pub TNVMCAP: [super::UCHAR; 16],
    pub UNVMCAP: [super::UCHAR; 16],
    pub RPMBS: NVME_IDENTIFY_CONTROLLER_DATA_9,
    pub EDSTT: super::USHORT,
    pub DSTO: super::UCHAR,
    pub FWUG: super::UCHAR,
    pub KAS: super::USHORT,
    pub HCTMA: NVME_IDENTIFY_CONTROLLER_DATA_10,
    pub MNTMT: super::USHORT,
    pub MXTMT: super::USHORT,
    pub SANICAP: NVME_IDENTIFY_CONTROLLER_DATA_11,
    pub HMMINDS: super::ULONG,
    pub HMMAXD: super::USHORT,
    pub NSETIDMAX: super::USHORT,
    pub ENDGIDMAX: super::USHORT,
    pub ANATT: super::UCHAR,
    pub ANACAP: NVME_IDENTIFY_CONTROLLER_DATA_12,
    pub ANAGRPMAX: super::ULONG,
    pub NANAGRPID: super::ULONG,
    pub PELS: super::ULONG,
    pub DomainId: super::USHORT,
    pub Reserved2: [super::UCHAR; 10],
    pub MEGCAP: [super::UCHAR; 16],
    pub TMPTHHA: super::UCHAR,
    pub Reserved3: super::UCHAR,
    pub CQT: super::USHORT,
    pub Reserved4: [super::UCHAR; 124],
    pub SQES: NVME_IDENTIFY_CONTROLLER_DATA_13,
    pub CQES: NVME_IDENTIFY_CONTROLLER_DATA_14,
    pub MAXCMD: super::USHORT,
    pub NN: super::ULONG,
    pub ONCS: NVME_IDENTIFY_CONTROLLER_DATA_15,
    pub FUSES: NVME_IDENTIFY_CONTROLLER_DATA_16,
    pub FNA: NVME_IDENTIFY_CONTROLLER_DATA_17,
    pub VWC: NVME_IDENTIFY_CONTROLLER_DATA_18,
    pub AWUN: super::USHORT,
    pub AWUPF: super::USHORT,
    pub NVSCC: NVME_IDENTIFY_CONTROLLER_DATA_19,
    pub NWPC: NVME_IDENTIFY_CONTROLLER_DATA_20,
    pub ACWU: super::USHORT,
    pub CopyDescFormats: super::USHORT,
    pub SGLS: NVME_IDENTIFY_CONTROLLER_DATA_21,
    pub MNAN: super::ULONG,
    pub MAXDNA: [super::UCHAR; 16],
    pub MAXCNA: super::ULONG,
    pub Reserved6: [super::UCHAR; 204],
    pub SUBNQN: [super::UCHAR; 256],
    pub Reserved7: [super::UCHAR; 768],
    pub IOCCSZ: super::ULONG,
    pub IORCSZ: super::ULONG,
    pub ICDOFF: super::USHORT,
    pub FCATT: NVME_IDENTIFY_CONTROLLER_DATA_22,
    pub MSDBD: super::UCHAR,
    pub OFCS: NVME_IDENTIFY_CONTROLLER_DATA_23,
    pub DCTYPE: super::UCHAR,
    pub Reserved8: [super::UCHAR; 241],
    pub PDS: [NVME_POWER_STATE_DESC; 32],
    pub VS: [super::UCHAR; 1024],
}
#[cfg(feature = "shared")]
impl Default for NVME_IDENTIFY_CONTROLLER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_0 {
    pub fn MultiPorts(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_MultiPorts(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn MultiControllers(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_MultiControllers(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn SRIOV(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_SRIOV(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ANAR(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ANAR(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_1 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_1 {
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn NamespaceAttributeChanged(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_NamespaceAttributeChanged(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn FirmwareActivation(&self) -> super::ULONG {
        (self._bitfield >> 9) & 1
    }
    pub fn set_FirmwareActivation(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        (self._bitfield >> 10) & 1
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn AsymmetricAccessChanged(&self) -> super::ULONG {
        (self._bitfield >> 11) & 1
    }
    pub fn set_AsymmetricAccessChanged(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn PredictableLatencyAggregateLogChanged(&self) -> super::ULONG {
        (self._bitfield >> 12) & 1
    }
    pub fn set_PredictableLatencyAggregateLogChanged(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value & 1) << 12);
    }
    pub fn LbaStatusChanged(&self) -> super::ULONG {
        (self._bitfield >> 13) & 1
    }
    pub fn set_LbaStatusChanged(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn EnduranceGroupAggregateLogChanged(&self) -> super::ULONG {
        (self._bitfield >> 14) & 1
    }
    pub fn set_EnduranceGroupAggregateLogChanged(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn NormalNvmSubsystemShutdown(&self) -> super::ULONG {
        (self._bitfield >> 15) & 1
    }
    pub fn set_NormalNvmSubsystemShutdown(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
    pub fn Reserved2(&self) -> super::ULONG {
        (self._bitfield >> 16) & 2047
    }
    pub fn set_Reserved2(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2047 << 16)) | ((value & 2047) << 16);
    }
    pub fn ZoneInformation(&self) -> super::ULONG {
        (self._bitfield >> 27) & 1
    }
    pub fn set_ZoneInformation(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 27)) | ((value & 1) << 27);
    }
    pub fn Reserved3(&self) -> super::ULONG {
        (self._bitfield >> 28) & 7
    }
    pub fn set_Reserved3(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 28)) | ((value & 7) << 28);
    }
    pub fn DiscoveryLogChanged(&self) -> super::ULONG {
        self._bitfield >> 31
    }
    pub fn set_DiscoveryLogChanged(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 31)) | ((value & 1) << 31);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_10 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_10 {
    pub fn Supported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Supported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_11 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_11 {
    pub fn CryptoErase(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_CryptoErase(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn BlockErase(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_BlockErase(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Overwrite(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_Overwrite(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 3) & 67108863
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(67108863 << 3)) | ((value & 67108863) << 3);
    }
    pub fn NDI(&self) -> super::ULONG {
        (self._bitfield >> 29) & 1
    }
    pub fn set_NDI(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 29)) | ((value & 1) << 29);
    }
    pub fn NODMMAS(&self) -> super::ULONG {
        self._bitfield >> 30
    }
    pub fn set_NODMMAS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(3 << 30)) | ((value & 3) << 30);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_12 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_12 {
    pub fn OptimizedState(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_OptimizedState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn NonOptimizedState(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_NonOptimizedState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn InaccessibleState(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_InaccessibleState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn PersistentLossState(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_PersistentLossState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn ChangeState(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_ChangeState(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn StaticANAGRPID(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_StaticANAGRPID(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn SupportNonZeroANAGRPID(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_SupportNonZeroANAGRPID(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_13 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_13 {
    pub fn RequiredEntrySize(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_RequiredEntrySize(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MaxEntrySize(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_MaxEntrySize(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_14 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_14 {
    pub fn RequiredEntrySize(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_RequiredEntrySize(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MaxEntrySize(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_MaxEntrySize(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_15 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_15 {
    pub fn Compare(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Compare(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn WriteUncorrectable(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_WriteUncorrectable(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn DatasetManagement(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_DatasetManagement(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn WriteZeroes(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_WriteZeroes(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn FeatureField(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_FeatureField(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reservations(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Reservations(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Timestamp(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_Timestamp(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Verify(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_Verify(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_16 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_16 {
    pub fn CompareAndWrite(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_CompareAndWrite(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_17 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_17 {
    pub fn FormatApplyToAll(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_FormatApplyToAll(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SecureEraseApplyToAll(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_SecureEraseApplyToAll(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn CryptographicEraseSupported(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_CryptographicEraseSupported(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn FormatSupportNSIDAllF(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_FormatSupportNSIDAllF(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_18 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_18 {
    pub fn Present(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Present(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn FlushBehavior(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 3
    }
    pub fn set_FlushBehavior(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 1)) | ((value & 3) << 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_19 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_19 {
    pub fn CommandFormatInSpec(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_CommandFormatInSpec(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_2 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_2 {
    pub fn HostIdentifier128Bit(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_HostIdentifier128Bit(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn NOPSPMode(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_NOPSPMode(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn NVMSets(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_NVMSets(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ReadRecoveryLevels(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ReadRecoveryLevels(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn EnduranceGroups(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_EnduranceGroups(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn PredictableLatencyMode(&self) -> super::ULONG {
        (self._bitfield >> 5) & 1
    }
    pub fn set_PredictableLatencyMode(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn TBKAS(&self) -> super::ULONG {
        (self._bitfield >> 6) & 1
    }
    pub fn set_TBKAS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn NamespaceGranularity(&self) -> super::ULONG {
        (self._bitfield >> 7) & 1
    }
    pub fn set_NamespaceGranularity(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn SQAssociations(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_SQAssociations(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn UUIDList(&self) -> super::ULONG {
        (self._bitfield >> 9) & 1
    }
    pub fn set_UUIDList(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn MultiDomainSubsystem(&self) -> super::ULONG {
        (self._bitfield >> 10) & 1
    }
    pub fn set_MultiDomainSubsystem(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn FixedCapacityManagement(&self) -> super::ULONG {
        (self._bitfield >> 11) & 1
    }
    pub fn set_FixedCapacityManagement(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn VariableCapacityManagement(&self) -> super::ULONG {
        (self._bitfield >> 12) & 1
    }
    pub fn set_VariableCapacityManagement(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value & 1) << 12);
    }
    pub fn DeleteEnduranceGroup(&self) -> super::ULONG {
        (self._bitfield >> 13) & 1
    }
    pub fn set_DeleteEnduranceGroup(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn DeleteNVMSet(&self) -> super::ULONG {
        (self._bitfield >> 14) & 1
    }
    pub fn set_DeleteNVMSet(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn ELBAS(&self) -> super::ULONG {
        (self._bitfield >> 15) & 1
    }
    pub fn set_ELBAS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_20 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_20 {
    pub fn WriteProtect(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_WriteProtect(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn UntilPowerCycle(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_UntilPowerCycle(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Permanent(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_Permanent(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 3
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(31 << 3)) | ((value & 31) << 3);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_21 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_21 {
    pub fn SGLSupported(&self) -> super::ULONG {
        (self._bitfield) & 3
    }
    pub fn set_SGLSupported(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn KeyedSGLData(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_KeyedSGLData(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 3) & 8191
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(8191 << 3)) | ((value & 8191) << 3);
    }
    pub fn BitBucketDescrSupported(&self) -> super::ULONG {
        (self._bitfield >> 16) & 1
    }
    pub fn set_BitBucketDescrSupported(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 16)) | ((value & 1) << 16);
    }
    pub fn ByteAlignedContiguousPhysicalBuffer(&self) -> super::ULONG {
        (self._bitfield >> 17) & 1
    }
    pub fn set_ByteAlignedContiguousPhysicalBuffer(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 17)) | ((value & 1) << 17);
    }
    pub fn SGLLengthLargerThanDataLength(&self) -> super::ULONG {
        (self._bitfield >> 18) & 1
    }
    pub fn set_SGLLengthLargerThanDataLength(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 18)) | ((value & 1) << 18);
    }
    pub fn MPTRSGLDescriptor(&self) -> super::ULONG {
        (self._bitfield >> 19) & 1
    }
    pub fn set_MPTRSGLDescriptor(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 19)) | ((value & 1) << 19);
    }
    pub fn AddressFieldSGLDataBlock(&self) -> super::ULONG {
        (self._bitfield >> 20) & 1
    }
    pub fn set_AddressFieldSGLDataBlock(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value & 1) << 20);
    }
    pub fn TransportSGLData(&self) -> super::ULONG {
        (self._bitfield >> 21) & 1
    }
    pub fn set_TransportSGLData(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value & 1) << 21);
    }
    pub fn Reserved1(&self) -> super::ULONG {
        self._bitfield >> 22
    }
    pub fn set_Reserved1(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1023 << 22)) | ((value & 1023) << 22);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_22 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_22 {
    pub fn StaticControllerModel(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_StaticControllerModel(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_23 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_23 {
    pub fn IOQueueDeletion(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_IOQueueDeletion(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_3 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_3 {
    pub fn ReadRecoveryLevel0(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_ReadRecoveryLevel0(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ReadRecoveryLevel1(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ReadRecoveryLevel1(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ReadRecoveryLevel2(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ReadRecoveryLevel2(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn ReadRecoveryLevel3(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_ReadRecoveryLevel3(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn ReadRecoveryLevel4(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_ReadRecoveryLevel4(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn ReadRecoveryLevel5(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_ReadRecoveryLevel5(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn ReadRecoveryLevel6(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_ReadRecoveryLevel6(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn ReadRecoveryLevel7(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_ReadRecoveryLevel7(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn ReadRecoveryLevel8(&self) -> super::USHORT {
        (self._bitfield >> 8) & 1
    }
    pub fn set_ReadRecoveryLevel8(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn ReadRecoveryLevel9(&self) -> super::USHORT {
        (self._bitfield >> 9) & 1
    }
    pub fn set_ReadRecoveryLevel9(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn ReadRecoveryLevel10(&self) -> super::USHORT {
        (self._bitfield >> 10) & 1
    }
    pub fn set_ReadRecoveryLevel10(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn ReadRecoveryLevel11(&self) -> super::USHORT {
        (self._bitfield >> 11) & 1
    }
    pub fn set_ReadRecoveryLevel11(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn ReadRecoveryLevel12(&self) -> super::USHORT {
        (self._bitfield >> 12) & 1
    }
    pub fn set_ReadRecoveryLevel12(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 12)) | ((value & 1) << 12);
    }
    pub fn ReadRecoveryLevel13(&self) -> super::USHORT {
        (self._bitfield >> 13) & 1
    }
    pub fn set_ReadRecoveryLevel13(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 13)) | ((value & 1) << 13);
    }
    pub fn ReadRecoveryLevel14(&self) -> super::USHORT {
        (self._bitfield >> 14) & 1
    }
    pub fn set_ReadRecoveryLevel14(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn ReadRecoveryLevel15(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_ReadRecoveryLevel15(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_4 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_4 {
    pub fn SecurityCommands(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_SecurityCommands(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn FormatNVM(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_FormatNVM(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn FirmwareCommands(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_FirmwareCommands(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn NamespaceCommands(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_NamespaceCommands(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn DeviceSelfTest(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_DeviceSelfTest(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Directives(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1
    }
    pub fn set_Directives(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn NVMeMICommands(&self) -> super::USHORT {
        (self._bitfield >> 6) & 1
    }
    pub fn set_NVMeMICommands(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn VirtualizationMgmt(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_VirtualizationMgmt(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn DoorBellBufferConfig(&self) -> super::USHORT {
        (self._bitfield >> 8) & 1
    }
    pub fn set_DoorBellBufferConfig(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn GetLBAStatus(&self) -> super::USHORT {
        (self._bitfield >> 9) & 1
    }
    pub fn set_GetLBAStatus(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn CommandFeatureLockdown(&self) -> super::USHORT {
        (self._bitfield >> 10) & 1
    }
    pub fn set_CommandFeatureLockdown(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 11
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(31 << 11)) | ((value & 31) << 11);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_5 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_5 {
    pub fn Slot1ReadOnly(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Slot1ReadOnly(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SlotCount(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 7
    }
    pub fn set_SlotCount(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 1)) | ((value & 7) << 1);
    }
    pub fn ActivationWithoutReset(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_ActivationWithoutReset(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_6 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_6 {
    pub fn SmartPagePerNamespace(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_SmartPagePerNamespace(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CommandEffectsLog(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CommandEffectsLog(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn LogPageExtendedData(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_LogPageExtendedData(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn TelemetrySupport(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_TelemetrySupport(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn PersistentEventLog(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_PersistentEventLog(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn SupportedLogPages(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_SupportedLogPages(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn TelemetryDataArea4(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_TelemetryDataArea4(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_7 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_7 {
    pub fn CommandFormatInSpec(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_CommandFormatInSpec(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_8 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_8 {
    pub fn Supported(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_Supported(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_CONTROLLER_DATA_9 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_CONTROLLER_DATA_9 {
    pub fn RPMBUnitCount(&self) -> super::ULONG {
        (self._bitfield) & 7
    }
    pub fn set_RPMBUnitCount(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn AuthenticationMethod(&self) -> super::ULONG {
        (self._bitfield >> 3) & 7
    }
    pub fn set_AuthenticationMethod(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        (self._bitfield >> 6) & 1023
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1023 << 6)) | ((value & 1023) << 6);
    }
    pub fn TotalSize(&self) -> super::ULONG {
        (self._bitfield >> 16) & 255
    }
    pub fn set_TotalSize(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 16)) | ((value & 255) << 16);
    }
    pub fn AccessSize(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_AccessSize(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_IO_COMMAND_SET {
    pub IOCommandSetVector: [IO_COMMAND_SET_VECTOR; 512],
}
#[cfg(feature = "shared")]
impl Default for NVME_IDENTIFY_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA {
    pub NSZE: super::ULONGLONG,
    pub NCAP: super::ULONGLONG,
    pub NUSE: super::ULONGLONG,
    pub NSFEAT: NVME_IDENTIFY_NAMESPACE_DATA_0,
    pub NLBAF: super::UCHAR,
    pub FLBAS: NVME_IDENTIFY_NAMESPACE_DATA_1,
    pub MC: NVME_IDENTIFY_NAMESPACE_DATA_2,
    pub DPC: NVME_IDENTIFY_NAMESPACE_DATA_3,
    pub DPS: NVME_IDENTIFY_NAMESPACE_DATA_4,
    pub NMIC: NVME_IDENTIFY_NAMESPACE_DATA_5,
    pub RESCAP: NVM_RESERVATION_CAPABILITIES,
    pub FPI: NVME_IDENTIFY_NAMESPACE_DATA_6,
    pub DLFEAT: NVME_IDENTIFY_NAMESPACE_DATA_7,
    pub NAWUN: super::USHORT,
    pub NAWUPF: super::USHORT,
    pub NACWU: super::USHORT,
    pub NABSN: super::USHORT,
    pub NABO: super::USHORT,
    pub NABSPF: super::USHORT,
    pub NOIOB: super::USHORT,
    pub NVMCAP: [super::UCHAR; 16],
    pub NPWG: super::USHORT,
    pub NPWA: super::USHORT,
    pub NPDG: super::USHORT,
    pub NPDA: super::USHORT,
    pub NOWS: super::USHORT,
    pub MSSRL: super::USHORT,
    pub MCL: super::ULONG,
    pub MSRC: super::UCHAR,
    pub Reserved2: [super::UCHAR; 11],
    pub ANAGRPID: super::ULONG,
    pub Reserved3: [super::UCHAR; 3],
    pub NSATTR: NVME_IDENTIFY_NAMESPACE_DATA_8,
    pub NVMSETID: super::USHORT,
    pub ENDGID: super::USHORT,
    pub NGUID: [super::UCHAR; 16],
    pub EUI64: [super::UCHAR; 8],
    pub LBAF: [NVME_LBA_FORMAT; 64],
    pub VS: [super::UCHAR; 3712],
}
#[cfg(feature = "shared")]
impl Default for NVME_IDENTIFY_NAMESPACE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_0 {
    pub fn ThinProvisioning(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ThinProvisioning(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn NameSpaceAtomicWriteUnit(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_NameSpaceAtomicWriteUnit(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn DeallocatedOrUnwrittenError(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_DeallocatedOrUnwrittenError(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn SkipReuseUI(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_SkipReuseUI(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn NameSpaceIoOptimization(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_NameSpaceIoOptimization(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_1 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_1 {
    pub fn LbaFormatIndex(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_LbaFormatIndex(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn MetadataInExtendedDataLBA(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_MetadataInExtendedDataLBA(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn LbaFormatIndexMS(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 3
    }
    pub fn set_LbaFormatIndexMS(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 5)) | ((value & 3) << 5);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_2 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_2 {
    pub fn MetadataInExtendedDataLBA(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_MetadataInExtendedDataLBA(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn MetadataInSeparateBuffer(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_MetadataInSeparateBuffer(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_3 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_3 {
    pub fn ProtectionInfoType1(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ProtectionInfoType1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ProtectionInfoType2(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ProtectionInfoType2(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ProtectionInfoType3(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ProtectionInfoType3(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn InfoAtBeginningOfMetadata(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_InfoAtBeginningOfMetadata(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn InfoAtEndOfMetadata(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_InfoAtEndOfMetadata(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_4 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_4 {
    pub fn ProtectionInfoTypeEnabled(&self) -> super::UCHAR {
        (self._bitfield) & 7
    }
    pub fn set_ProtectionInfoTypeEnabled(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn InfoAtBeginningOfMetadata(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_InfoAtBeginningOfMetadata(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_5 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_5 {
    pub fn SharedNameSpace(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_SharedNameSpace(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_6 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_6 {
    pub fn PercentageRemained(&self) -> super::UCHAR {
        (self._bitfield) & 127
    }
    pub fn set_PercentageRemained(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !127) | (value & 127);
    }
    pub fn Supported(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Supported(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_7 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_7 {
    pub fn ReadBehavior(&self) -> super::UCHAR {
        (self._bitfield) & 7
    }
    pub fn set_ReadBehavior(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn WriteZeroes(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_WriteZeroes(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn GuardFieldWithCRC(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_GuardFieldWithCRC(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 5
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DATA_8 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_NAMESPACE_DATA_8 {
    pub fn WriteProtected(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_WriteProtected(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    pub NIDT: super::UCHAR,
    pub NIDL: super::UCHAR,
    pub Reserved: [super::UCHAR; 2],
    pub NID: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for NVME_IDENTIFY_NAMESPACE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub VSL: super::UCHAR,
    pub WZSL: super::UCHAR,
    pub WUSL: super::UCHAR,
    pub DMRL: super::UCHAR,
    pub DMRSL: super::ULONG,
    pub DMSL: super::ULONGLONG,
    pub Reserved: [super::UCHAR; 4080],
}
#[cfg(feature = "shared")]
impl Default for NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    pub ZOC: NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0,
    pub OZCS: NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1,
    pub MAR: super::ULONG,
    pub MOR: super::ULONG,
    pub RRL: super::ULONG,
    pub FRL: super::ULONG,
    pub Reserved0: [super::UCHAR; 2796],
    pub LBAEF: [NVME_LBA_ZONE_FORMAT; 16],
    pub Reserved1: [super::UCHAR; 768],
    pub VS: [super::UCHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_0 {
    pub fn VariableZoneCapacity(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_VariableZoneCapacity(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn ZoneExcursions(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_ZoneExcursions(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET_1 {
    pub fn ReadAcrossZoneBoundaries(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_ReadAcrossZoneBoundaries(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET {
    pub ZASL: super::UCHAR,
    pub Reserved: [super::UCHAR; 4095],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_LATENCY_MONITORING_ENTRY {
    pub ActiveBucketTimerThreshold: super::USHORT,
    pub ActiveThresholdA: super::UCHAR,
    pub ActiveThresholdB: super::UCHAR,
    pub ActiveThresholdC: super::UCHAR,
    pub ActiveThresholdD: super::UCHAR,
    pub ActiveLatencyConfig: super::USHORT,
    pub ActiveLatencyMinimumWindow: super::UCHAR,
    pub DebugLogTriggerEnable: super::USHORT,
    pub DiscardDebugLog: super::UCHAR,
    pub LatencyMonitorFeatureEnable: super::UCHAR,
    pub Reserved0: [super::UCHAR; 4083],
}
#[cfg(feature = "shared")]
impl Default for NVME_LATENCY_MONITORING_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_LBA_FORMAT {
    pub Anonymous: NVME_LBA_FORMAT_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_LBA_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LBA_FORMAT_0 {
    pub MS: super::USHORT,
    pub LBADS: super::UCHAR,
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_LBA_FORMAT_0 {
    pub fn RP(&self) -> super::UCHAR {
        (self._bitfield) & 3
    }
    pub fn set_RP(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Reserved0(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved0(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_LBA_RANGE {
    pub Attributes: NVME_CONTEXT_ATTRIBUTES,
    pub LogicalBlockCount: super::ULONG,
    pub StartingLBA: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_LBA_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_LBA_RANGET_TYPE_ENTRY {
    pub Type: super::UCHAR,
    pub Attributes: NVME_LBA_RANGET_TYPE_ENTRY_0,
    pub Reserved0: [super::UCHAR; 14],
    pub SLBA: super::ULONGLONG,
    pub NLB: super::ULONGLONG,
    pub GUID: [super::UCHAR; 16],
    pub Reserved1: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for NVME_LBA_RANGET_TYPE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LBA_RANGET_TYPE_ENTRY_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_LBA_RANGET_TYPE_ENTRY_0 {
    pub fn MayOverwritten(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_MayOverwritten(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Hidden(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_Hidden(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_LBA_ZONE_FORMAT {
    pub ZoneSize: super::ULONGLONG,
    pub ZDES: super::UCHAR,
    pub Reserved: [super::UCHAR; 7],
}
#[cfg(feature = "shared")]
impl Default for NVME_LBA_ZONE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG {
    pub fn EstablishContextAndRead512BytesOfHeaderSupported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_EstablishContextAndRead512BytesOfHeaderSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(32767 << 1)) | ((value & 32767) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_LID_SUPPORTED_AND_EFFECTS {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_LID_SUPPORTED_AND_EFFECTS {
    pub fn LSUPP(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_LSUPP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn IOS(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_IOS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 2) & 16383
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(16383 << 2)) | ((value & 16383) << 2);
    }
    pub fn LIDSpecific(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_LIDSpecific(&mut self, value: super::ULONG) {
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
pub const NVME_LOG_PAGE_WCS_DEVICE_ERROR_RECOVERY: NVME_VENDOR_LOG_PAGES = 193;
pub const NVME_LOG_PAGE_WCS_DEVICE_SMART_ATTRIBUTES: NVME_VENDOR_LOG_PAGES = 192;
pub const NVME_MAX_HOST_IDENTIFIER_SIZE: i32 = 16;
pub const NVME_MAX_LOG_PAGE_IDENTIFIER: i32 = 255;
pub const NVME_MAX_LOG_SIZE: i32 = 4096;
pub const NVME_MAX_UUID_INDEX: i32 = 127;
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
pub const NVME_NQN_MAX_LEN: i32 = 256;
pub const NVME_NQN_NAME_MAX_LEN: i32 = 223;
pub const NVME_NUM_FID_SUPPORTED: i32 = 256;
pub const NVME_NUM_LOG_PAGE_IDENTIFIERS: i32 = 256;
pub const NVME_NUM_NVME_MI_COMMANDS_SUPPORTED: i32 = 256;
pub const NVME_NUM_UUID_LIST_ENTRIES: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS {
    pub fn CSUPP(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_CSUPP(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn UDCC(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_UDCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn NCC(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_NCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn NIC(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_NIC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn CCC(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_CCC(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::ULONG {
        (self._bitfield >> 5) & 32767
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(32767 << 5)) | ((value & 32767) << 5);
    }
    pub fn CSPNamespace(&self) -> super::ULONG {
        (self._bitfield >> 20) & 1
    }
    pub fn set_CSPNamespace(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 20)) | ((value & 1) << 20);
    }
    pub fn CSPController(&self) -> super::ULONG {
        (self._bitfield >> 21) & 1
    }
    pub fn set_CSPController(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 21)) | ((value & 1) << 21);
    }
    pub fn CSPNVMSet(&self) -> super::ULONG {
        (self._bitfield >> 22) & 1
    }
    pub fn set_CSPNVMSet(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 22)) | ((value & 1) << 22);
    }
    pub fn CSPEnduranceGroup(&self) -> super::ULONG {
        (self._bitfield >> 23) & 1
    }
    pub fn set_CSPEnduranceGroup(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 23)) | ((value & 1) << 23);
    }
    pub fn CSPDomain(&self) -> super::ULONG {
        (self._bitfield >> 24) & 1
    }
    pub fn set_CSPDomain(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 24)) | ((value & 1) << 24);
    }
    pub fn CSPNVMSubsystem(&self) -> super::ULONG {
        (self._bitfield >> 25) & 1
    }
    pub fn set_CSPNVMSubsystem(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 25)) | ((value & 1) << 25);
    }
    pub fn CSPReserved(&self) -> super::ULONG {
        self._bitfield >> 26
    }
    pub fn set_CSPReserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(63 << 26)) | ((value & 63) << 26);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG {
    pub ManagementInterfaceCommandSupported: [NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS; 256],
    pub Reserved: [super::UCHAR; 3072],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_NVM_SUBSYSTEM_RESET {
    pub NSSRC: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_NVM_SUBSYSTEM_SHUTDOWN {
    pub NSSC: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG {
    pub PciePorts: super::USHORT,
    pub OobMgmtSupport: NVME_OCP_DEVICE_CAPABILITIES_LOG_0,
    pub WriteZeroesCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_1,
    pub SanitizeCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_2,
    pub DatasetMgmtCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_3,
    pub WriteUncorrectableCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_4,
    pub FusedCommand: NVME_OCP_DEVICE_CAPABILITIES_LOG_5,
    pub MinimumValidDSSDPowerState: super::USHORT,
    pub Reserved0: super::UCHAR,
    pub DssdDescriptors: [DSSD_POWER_STATE_DESCRIPTOR; 127],
    pub Reserved1: [super::UCHAR; 3934],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_0_0 {
    pub fn MctpOverSMBusSupported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_MctpOverSMBusSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn MctpOverPcieVDMSupported(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_MctpOverPcieVDMSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn BasicMgmtCommandSupported(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_BasicMgmtCommandSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::USHORT {
        (self._bitfield >> 3) & 4095
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(4095 << 3)) | ((value & 4095) << 3);
    }
    pub fn CompliesWithSpec(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_CompliesWithSpec(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_1 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_1_0 {
    pub fn Supported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Supported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn DEACBitSupported(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_DEACBitSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn FUABitSupported(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_FUABitSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn NvmeIo5Met(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_NvmeIo5Met(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn NvmeIo6Met(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_NvmeIo6Met(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1023
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1023 << 5)) | ((value & 1023) << 5);
    }
    pub fn CompliesWithSpec(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_CompliesWithSpec(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_2 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_2_0 {
    pub fn Supported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Supported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CryptoEraseSupported(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CryptoEraseSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn BlockEraseSupported(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_BlockEraseSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn OverwriteSupported(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_OverwriteSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn DeallocateLbaSupported(&self) -> super::USHORT {
        (self._bitfield >> 4) & 1
    }
    pub fn set_DeallocateLbaSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn Reserved(&self) -> super::USHORT {
        (self._bitfield >> 5) & 1023
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1023 << 5)) | ((value & 1023) << 5);
    }
    pub fn CompliesWithSpec(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_CompliesWithSpec(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_3 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_3_0 {
    pub fn Supported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Supported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn AttribDeallocateSupported(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_AttribDeallocateSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        (self._bitfield >> 2) & 8191
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(8191 << 2)) | ((value & 8191) << 2);
    }
    pub fn CompliesWithSpec(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_CompliesWithSpec(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_4 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_4_0 {
    pub fn Supported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_Supported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn SingleLBASupported(&self) -> super::USHORT {
        (self._bitfield >> 1) & 1
    }
    pub fn set_SingleLBASupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn MaxLBASupported(&self) -> super::USHORT {
        (self._bitfield >> 2) & 1
    }
    pub fn set_MaxLBASupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn NvmeIo14Met(&self) -> super::USHORT {
        (self._bitfield >> 3) & 1
    }
    pub fn set_NvmeIo14Met(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::USHORT {
        (self._bitfield >> 4) & 2047
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(2047 << 4)) | ((value & 2047) << 4);
    }
    pub fn CompliesWithSpec(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_CompliesWithSpec(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_CAPABILITIES_LOG_5 {
    pub Anonymous: NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0,
    pub AsUshort: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_CAPABILITIES_LOG_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0 {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_CAPABILITIES_LOG_5_0 {
    pub fn CWFusedSupported(&self) -> super::USHORT {
        (self._bitfield) & 1
    }
    pub fn set_CWFusedSupported(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::USHORT {
        (self._bitfield >> 1) & 16383
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(16383 << 1)) | ((value & 16383) << 1);
    }
    pub fn CompliesWithSpec(&self) -> super::USHORT {
        self._bitfield >> 15
    }
    pub fn set_CompliesWithSpec(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value & 1) << 15);
    }
}
pub const NVME_OCP_DEVICE_CAPABILITIES_LOG_VERSION_1: i32 = 1;
pub const NVME_OCP_DEVICE_DSSD_SPEC_MAJOR_VERSION_0: i32 = 0;
pub const NVME_OCP_DEVICE_DSSD_SPEC_MAJOR_VERSION_2: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {
    pub PanicResetWaitTime: super::USHORT,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DeviceRecoveryAction1: super::UCHAR,
    pub PanicId: super::ULONGLONG,
    pub DeviceCapabilities: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub VendorSpecificCommandCDW12: super::ULONG,
    pub VendorSpecificCommandCDW13: super::ULONG,
    pub VendorSpecificCommandTimeout: super::UCHAR,
    pub DeviceRecoveryAction2: super::UCHAR,
    pub DeviceRecoveryAction2Timeout: super::UCHAR,
    pub Reserved1: [super::UCHAR; 463],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_VERSION_2: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {
    pub LID: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub ValidNumberOfEntries: super::ULONG,
    pub Entries: [FIRMWARE_ACTIVATION_HISTORY_ENTRY; 20],
    pub Reserved1: [super::UCHAR; 2790],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {
    pub FeatureStatus: LATENCY_MONITOR_FEATURE_STATUS,
    pub Reserved0: super::UCHAR,
    pub ActiveBucketTimer: super::USHORT,
    pub ActiveBucketTimerThreshold: super::USHORT,
    pub ActiveThresholdA: super::UCHAR,
    pub ActiveThresholdB: super::UCHAR,
    pub ActiveThresholdC: super::UCHAR,
    pub ActiveThresholdD: super::UCHAR,
    pub ActiveLatencyConfig: ACTIVE_LATENCY_CONFIGURATION,
    pub ActiveLatencyMinimumWindow: super::UCHAR,
    pub Reserved1: [super::UCHAR; 19],
    pub ActiveBucketCounter0: BUCKET_COUNTER,
    pub ActiveBucketCounter1: BUCKET_COUNTER,
    pub ActiveBucketCounter2: BUCKET_COUNTER,
    pub ActiveBucketCounter3: BUCKET_COUNTER,
    pub ActiveLatencyStamp: LATENCY_STAMP,
    pub ActiveMeasuredLatency: MEASURED_LATENCY,
    pub ActiveLatencyStampUnits: LATENCY_STAMP_UNITS,
    pub Reserved2: [super::UCHAR; 22],
    pub StaticBucketCounter0: BUCKET_COUNTER,
    pub StaticBucketCounter1: BUCKET_COUNTER,
    pub StaticBucketCounter2: BUCKET_COUNTER,
    pub StaticBucketCounter3: BUCKET_COUNTER,
    pub StaticLatencyStamp: LATENCY_STAMP,
    pub StaticMeasuredLatency: MEASURED_LATENCY,
    pub StaticLatencyStampUnits: LATENCY_STAMP_UNITS,
    pub Reserved3: [super::UCHAR; 22],
    pub DebugLogTriggerEnable: DEBUG_BIT_FIELD,
    pub DebugLogMeasuredLatency: super::USHORT,
    pub DebugLogLatencyStamp: super::ULONGLONG,
    pub DebugLogPointer: super::USHORT,
    pub DebugCounterTriggerSource: DEBUG_BIT_FIELD,
    pub DebugLogStampUnits: NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0,
    pub Reserved4: [super::UCHAR; 29],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_0_0 {
    pub fn BasedOnTimestamp(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_BasedOnTimestamp(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
pub const NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {
    pub MediaUnitsWritten: [super::UCHAR; 16],
    pub MediaUnitsRead: [super::UCHAR; 16],
    pub BadUserNANDBlockCount: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0,
    pub BadSystemNANDBlockCount: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1,
    pub XORRecoveryCount: super::ULONGLONG,
    pub UnrecoverableReadErrorCount: super::ULONGLONG,
    pub SoftECCErrorCount: super::ULONGLONG,
    pub EndToEndCorrectionCounts: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_2,
    pub PercentageSystemDataUsed: super::UCHAR,
    pub RefreshCount: [super::UCHAR; 7],
    pub UserDataEraseCounts: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_3,
    pub ThermalThrottling: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_4,
    pub DSSDSpecVersion: NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_5,
    pub PCIeCorrectableErrorCount: super::ULONGLONG,
    pub IncompleteShutdownCount: super::ULONG,
    pub Reserved1: super::ULONG,
    pub PercentageFreeBlocks: super::UCHAR,
    pub Reserved2: [super::UCHAR; 7],
    pub CapacitorHealth: super::USHORT,
    pub NvmeErrata: super::UCHAR,
    pub Reserved3: [super::UCHAR; 5],
    pub UnalignedIOCount: super::ULONGLONG,
    pub SecurityVersionNumber: super::ULONGLONG,
    pub NUSE: super::ULONGLONG,
    pub PLPStartCount: [super::UCHAR; 16],
    pub EnduranceEstimate: [super::UCHAR; 16],
    pub PCIeLinkRetrainingCount: super::ULONGLONG,
    pub PowerStateChangeCount: super::ULONGLONG,
    pub Reserved4: [super::UCHAR; 286],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0 {
    pub RawCount: [super::UCHAR; 6],
    pub Normalized: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1 {
    pub RawCount: [super::UCHAR; 6],
    pub Normalized: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_2 {
    pub DetectedCounts: super::ULONG,
    pub CorrectedCounts: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_3 {
    pub MaximumCount: super::ULONG,
    pub MinimumCount: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_4 {
    pub EventCount: super::UCHAR,
    pub Status: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3_5 {
    pub Errata: super::UCHAR,
    pub PointVersion: super::USHORT,
    pub MinorVersion: super::USHORT,
    pub MajorVersion: super::UCHAR,
}
pub const NVME_OCP_DEVICE_SMART_INFORMATION_LOG_VERSION_3: i32 = 3;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {
    pub State: NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0,
    pub Reserved0: [super::UCHAR; 3],
    pub LSPActivationCount: super::UCHAR,
    pub TPRevertCount: super::UCHAR,
    pub LSPRevertCount: super::UCHAR,
    pub LOCount: super::UCHAR,
    pub SUMLOCount: super::UCHAR,
    pub RPLOCount: super::UCHAR,
    pub NPLOCount: super::UCHAR,
    pub RLLOCount: super::UCHAR,
    pub WLLOCount: super::UCHAR,
    pub RULOCount: super::UCHAR,
    pub WULOCount: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub SIDAuthTryCount: super::ULONG,
    pub SIDAuthTryLimit: super::ULONG,
    pub ResetCount: super::ULONG,
    pub ResetLockCount: super::ULONG,
    pub Reserved2: [super::UCHAR; 462],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0 {
    pub Anonymous: NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_0_0 {
    pub fn CPINSIDValue(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_CPINSIDValue(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn CPINSIDBlocked(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_CPINSIDBlocked(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn LockingEnabled(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_LockingEnabled(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn SUMOwner(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_SUMOwner(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
pub const NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_TCG_HISTORY_LOG {
    pub LID: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub HistoryEntryCount: super::ULONG,
    pub HistoryEntries: [TCG_HISTORY_ENTRY; 84],
    pub Reserved1: [super::UCHAR; 38],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_TCG_HISTORY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_TCG_HISTORY_LOG_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {
    pub UnsupportedCount: super::USHORT,
    pub Reserved0: [super::UCHAR; 14],
    pub UnsupportedReqList: [UNSUPPORTED_REQUIREMENT; 253],
    pub Reserved1: [super::UCHAR; 14],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    pub EventType: super::UCHAR,
    pub EventTypeRevision: super::UCHAR,
    pub EventHeaderLength: super::UCHAR,
    pub Reserved0: super::UCHAR,
    pub ControllerIdentifier: super::USHORT,
    pub EventTimestamp: super::ULONGLONG,
    pub Reserved1: [super::UCHAR; 6],
    pub VendorSpecificInformationLength: super::USHORT,
    pub EventLength: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_PERSISTENT_EVENT_LOG_HEADER {
    pub LogIdentifier: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub TotalNumberOfEvents: super::ULONG,
    pub TotalLogLength: super::ULONGLONG,
    pub LogRevision: super::UCHAR,
    pub Reserved1: super::UCHAR,
    pub LogHeaderLength: super::USHORT,
    pub Timestamp: super::ULONGLONG,
    pub PowerOnHours: [super::UCHAR; 16],
    pub PowerCycleCount: super::ULONGLONG,
    pub PciVendorId: super::USHORT,
    pub PciSubsystemVendorId: super::USHORT,
    pub SerialNumber: [super::UCHAR; 20],
    pub ModelNumber: [super::UCHAR; 40],
    pub NVMSubsystemNVMeQualifiedName: [super::UCHAR; 256],
    pub Reserved: [super::UCHAR; 108],
    pub SupportedEventsBitmap: [super::UCHAR; 32],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_POWER_STATE_DESC {
    pub MP: super::USHORT,
    pub Reserved0: super::UCHAR,
    pub _bitfield1: u8,
    pub ENLAT: super::ULONG,
    pub EXLAT: super::ULONG,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u8,
    pub _bitfield5: u8,
    pub IDLP: super::USHORT,
    pub _bitfield6: u8,
    pub Reserved7: super::UCHAR,
    pub ACTP: super::USHORT,
    pub _bitfield7: u8,
    pub Reserved9: [super::UCHAR; 9],
}
#[cfg(feature = "shared")]
impl NVME_POWER_STATE_DESC {
    pub fn MPS(&self) -> super::UCHAR {
        (self._bitfield1) & 1
    }
    pub fn set_MPS(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !1) | (value & 1);
    }
    pub fn NOPS(&self) -> super::UCHAR {
        (self._bitfield1 >> 1) & 1
    }
    pub fn set_NOPS(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield1 >> 2
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield1 = (self._bitfield1 & !(63 << 2)) | ((value & 63) << 2);
    }
    pub fn RRT(&self) -> super::UCHAR {
        (self._bitfield2) & 31
    }
    pub fn set_RRT(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !31) | (value & 31);
    }
    pub fn Reserved2(&self) -> super::UCHAR {
        self._bitfield2 >> 5
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield2 = (self._bitfield2 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn RRL(&self) -> super::UCHAR {
        (self._bitfield3) & 31
    }
    pub fn set_RRL(&mut self, value: super::UCHAR) {
        self._bitfield3 = (self._bitfield3 & !31) | (value & 31);
    }
    pub fn Reserved3(&self) -> super::UCHAR {
        self._bitfield3 >> 5
    }
    pub fn set_Reserved3(&mut self, value: super::UCHAR) {
        self._bitfield3 = (self._bitfield3 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn RWT(&self) -> super::UCHAR {
        (self._bitfield4) & 31
    }
    pub fn set_RWT(&mut self, value: super::UCHAR) {
        self._bitfield4 = (self._bitfield4 & !31) | (value & 31);
    }
    pub fn Reserved4(&self) -> super::UCHAR {
        self._bitfield4 >> 5
    }
    pub fn set_Reserved4(&mut self, value: super::UCHAR) {
        self._bitfield4 = (self._bitfield4 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn RWL(&self) -> super::UCHAR {
        (self._bitfield5) & 31
    }
    pub fn set_RWL(&mut self, value: super::UCHAR) {
        self._bitfield5 = (self._bitfield5 & !31) | (value & 31);
    }
    pub fn Reserved5(&self) -> super::UCHAR {
        self._bitfield5 >> 5
    }
    pub fn set_Reserved5(&mut self, value: super::UCHAR) {
        self._bitfield5 = (self._bitfield5 & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn Reserved6(&self) -> super::UCHAR {
        (self._bitfield6) & 63
    }
    pub fn set_Reserved6(&mut self, value: super::UCHAR) {
        self._bitfield6 = (self._bitfield6 & !63) | (value & 63);
    }
    pub fn IPS(&self) -> super::UCHAR {
        self._bitfield6 >> 6
    }
    pub fn set_IPS(&mut self, value: super::UCHAR) {
        self._bitfield6 = (self._bitfield6 & !(3 << 6)) | ((value & 3) << 6);
    }
    pub fn APW(&self) -> super::UCHAR {
        (self._bitfield7) & 7
    }
    pub fn set_APW(&mut self, value: super::UCHAR) {
        self._bitfield7 = (self._bitfield7 & !7) | (value & 7);
    }
    pub fn Reserved8(&self) -> super::UCHAR {
        (self._bitfield7 >> 3) & 7
    }
    pub fn set_Reserved8(&mut self, value: super::UCHAR) {
        self._bitfield7 = (self._bitfield7 & !(7 << 3)) | ((value & 7) << 3);
    }
    pub fn APS(&self) -> super::UCHAR {
        self._bitfield7 >> 6
    }
    pub fn set_APS(&mut self, value: super::UCHAR) {
        self._bitfield7 = (self._bitfield7 & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_PRP_ENTRY {
    pub Anonymous: NVME_PRP_ENTRY_0,
    pub AsUlonglong: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_PRP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_PRP_ENTRY_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "shared")]
impl NVME_PRP_ENTRY_0 {
    pub fn Reserved0(&self) -> super::ULONGLONG {
        (self._bitfield) & 3
    }
    pub fn set_Reserved0(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn PBAO(&self) -> super::ULONGLONG {
        self._bitfield >> 2
    }
    pub fn set_PBAO(&mut self, value: super::ULONGLONG) {
        self._bitfield = (self._bitfield & !(4611686018427387903 << 2)) | ((value & 4611686018427387903) << 2);
    }
}
pub const NVME_PSDT_XFER_PRP: i32 = 0;
pub const NVME_PSDT_XFER_RESERVED: i32 = 3;
pub const NVME_PSDT_XFER_SGL_BYTE: i32 = 1;
pub const NVME_PSDT_XFER_SGL_QWORD: i32 = 2;
pub type NVME_RDMA_KEYED_SGL_DESC_SUBTYPE = i32;
pub const NVME_READ_BEHAVIOR_NOT_REPORTED: NVME_DEALLOCATE_READ_BEHAVIOR = 0;
pub const NVME_READ_BEHAVIOR_RETURN_ONES: NVME_DEALLOCATE_READ_BEHAVIOR = 2;
pub const NVME_READ_BEHAVIOR_RETURN_ZERO: NVME_DEALLOCATE_READ_BEHAVIOR = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_DATA {
    pub CNTLID: super::USHORT,
    pub RCSTS: NVME_REGISTERED_CONTROLLER_DATA_0,
    pub Reserved: [super::UCHAR; 5],
    pub HOSTID: [super::UCHAR; 8],
    pub RKEY: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_REGISTERED_CONTROLLER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_DATA_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_REGISTERED_CONTROLLER_DATA_0 {
    pub fn HoldReservation(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_HoldReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    pub CNTLID: super::USHORT,
    pub RCSTS: NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0,
    pub Reserved: [super::UCHAR; 5],
    pub RKEY: super::ULONGLONG,
    pub HOSTID: [super::UCHAR; 16],
    pub Reserved1: [super::UCHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for NVME_REGISTERED_CONTROLLER_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_REGISTERED_CONTROLLER_EXTENDED_DATA_0 {
    pub fn HoldReservation(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_HoldReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 1
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(127 << 1)) | ((value & 127) << 1);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_REPORT_ZONE_INFO {
    pub ZoneCount: super::ULONGLONG,
    pub Reserved: [super::ULONGLONG; 7],
    pub ZoneDescriptor: [NVME_ZONE_DESCRIPTOR; 1],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE {
    pub CRKEY: super::ULONGLONG,
    pub PRKEY: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_RESERVATION_NOTIFICATION_LOG {
    pub LogPageCount: super::ULONGLONG,
    pub LogPageType: super::UCHAR,
    pub AvailableLogPageCount: super::UCHAR,
    pub Reserved0: [super::UCHAR; 2],
    pub NameSpaceId: super::ULONG,
    pub Reserved1: [super::UCHAR; 48],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_RESERVATION_REGISTER_DATA_STRUCTURE {
    pub CRKEY: super::ULONGLONG,
    pub NRKEY: super::ULONGLONG,
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_RESERVATION_RELEASE_DATA_STRUCTURE {
    pub CRKEY: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub RegisteredControllersData: [NVME_REGISTERED_CONTROLLER_DATA; 1],
}
#[cfg(feature = "shared")]
impl Default for NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    pub Header: NVME_RESERVATION_REPORT_STATUS_HEADER,
    pub Reserved1: [super::UCHAR; 40],
    pub RegisteredControllersExtendedData: [NVME_REGISTERED_CONTROLLER_EXTENDED_DATA; 1],
}
#[cfg(feature = "shared")]
impl Default for NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_RESERVATION_REPORT_STATUS_HEADER {
    pub GEN: super::ULONG,
    pub RTYPE: super::UCHAR,
    pub REGCTL: super::USHORT,
    pub Reserved: [super::UCHAR; 2],
    pub PTPLS: super::UCHAR,
    pub Reserved1: [super::UCHAR; 14],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SANITIZE_STATUS {
    pub _bitfield: u16,
}
#[cfg(feature = "shared")]
impl NVME_SANITIZE_STATUS {
    pub fn MostRecentSanitizeOperationStatus(&self) -> super::USHORT {
        (self._bitfield) & 7
    }
    pub fn set_MostRecentSanitizeOperationStatus(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !7) | (value & 7);
    }
    pub fn NumberCompletedPassesOfOverwrite(&self) -> super::USHORT {
        (self._bitfield >> 3) & 15
    }
    pub fn set_NumberCompletedPassesOfOverwrite(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(15 << 3)) | ((value & 15) << 3);
    }
    pub fn GlobalDataErased(&self) -> super::USHORT {
        (self._bitfield >> 7) & 1
    }
    pub fn set_GlobalDataErased(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn Reserved(&self) -> super::USHORT {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: super::USHORT) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SANITIZE_STATUS_LOG {
    pub SPROG: super::USHORT,
    pub SSTAT: NVME_SANITIZE_STATUS,
    pub SCDW10: super::ULONG,
    pub EstimatedTimeForOverwrite: super::ULONG,
    pub EstimatedTimeForBlockErase: super::ULONG,
    pub EstimatedTimeForCryptoErase: super::ULONG,
    pub EstimatedTimeForOverwriteWithNoDeallocateMediaModification: super::ULONG,
    pub EstimatedTimeForBlockEraseWithNoDeallocateMediaModification: super::ULONG,
    pub EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification: super::ULONG,
    pub Reserved: [super::UCHAR; 480],
}
#[cfg(feature = "shared")]
impl Default for NVME_SANITIZE_STATUS_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SCSI_NAME_STRING {
    pub PCIVendorID: [super::CHAR; 4],
    pub ModelNumber: [super::CHAR; 40],
    pub NamespaceID: [super::CHAR; 4],
    pub SerialNumber: [super::CHAR; 20],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SET_ATTRIBUTES_ENTRY {
    pub Identifier: super::USHORT,
    pub ENDGID: super::USHORT,
    pub Reserved1: super::ULONG,
    pub Random4KBReadTypical: super::ULONG,
    pub OptimalWriteSize: super::ULONG,
    pub TotalCapacity: [super::UCHAR; 16],
    pub UnallocatedCapacity: [super::UCHAR; 16],
    pub Reserved2: [super::UCHAR; 80],
}
#[cfg(feature = "shared")]
impl Default for NVME_SET_ATTRIBUTES_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_SGL_DESC {
    pub Reserved0: [super::UCHAR; 15],
    pub Identifier: NVME_SGL_DESC_0,
}
#[cfg(feature = "shared")]
impl Default for NVME_SGL_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_SGL_DESC_0 {
    pub Anonymous: NVME_SGL_DESC_0_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_SGL_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SGL_DESC_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_SGL_DESC_0_0 {
    pub fn SubType(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_SubType(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Type(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Type(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
pub type NVME_SGL_DESC_SUBTYPE = i32;
pub type NVME_SGL_DESC_TYPE = i32;
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
pub const NVME_STREAMS_GET_STATUS_MAX_IDS: i32 = 65535;
pub const NVME_STREAMS_ID_MAX: i32 = 65535;
pub const NVME_STREAMS_ID_MIN: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    pub Anonymous: NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_SUBMISSION_QUEUE_TAIL_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_SUBMISSION_QUEUE_TAIL_DOORBELL_0 {
    pub fn SQT(&self) -> super::ULONG {
        (self._bitfield) & 65535
    }
    pub fn set_SQT(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !65535) | (value & 65535);
    }
    pub fn Reserved0(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_Reserved0(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_SUPPORTED_LOG_PAGES_LOG {
    pub LogPageIdentifierSupported: [NVME_LID_SUPPORTED_AND_EFFECTS; 256],
}
#[cfg(feature = "shared")]
impl Default for NVME_SUPPORTED_LOG_PAGES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    pub LogIdentifier: super::UCHAR,
    pub Reserved0: [super::UCHAR; 4],
    pub OrganizationID: [super::UCHAR; 3],
    pub Area1LastBlock: super::USHORT,
    pub Area2LastBlock: super::USHORT,
    pub Area3LastBlock: super::USHORT,
    pub Reserved1: [super::UCHAR; 2],
    pub Area4LastBlock: super::ULONG,
    pub Reserved2: [super::UCHAR; 362],
    pub ControllerInitiatedDataAvailable: super::UCHAR,
    pub ControllerInitiatedDataGenerationNumber: super::UCHAR,
    pub ReasonIdentifier: [super::UCHAR; 128],
}
#[cfg(feature = "shared")]
impl Default for NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_TELEMETRY_DATA_BLOCK_SIZE: i32 = 512;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_TELEMETRY_HOST_INITIATED_LOG {
    pub LogIdentifier: super::UCHAR,
    pub Reserved0: [super::UCHAR; 4],
    pub OrganizationID: [super::UCHAR; 3],
    pub Area1LastBlock: super::USHORT,
    pub Area2LastBlock: super::USHORT,
    pub Area3LastBlock: super::USHORT,
    pub Reserved1: [super::UCHAR; 2],
    pub Area4LastBlock: super::ULONG,
    pub Reserved2: [super::UCHAR; 361],
    pub HostInitiatedDataGenerationNumber: super::UCHAR,
    pub ControllerInitiatedDataAvailable: super::UCHAR,
    pub ControllerInitiatedDataGenerationNumber: super::UCHAR,
    pub ReasonIdentifier: [super::UCHAR; 128],
}
#[cfg(feature = "shared")]
impl Default for NVME_TELEMETRY_HOST_INITIATED_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVME_TEMPERATURE_OVER_THRESHOLD: NVME_TEMPERATURE_THRESHOLD_TYPES = 0;
pub type NVME_TEMPERATURE_THRESHOLD_TYPES = i32;
pub const NVME_TEMPERATURE_UNDER_THRESHOLD: NVME_TEMPERATURE_THRESHOLD_TYPES = 1;
pub const NVME_UUID_ASSOCIATION_NONE: i32 = 0;
pub const NVME_UUID_ASSOCIATION_PCI_SUBSYSTEM_VID: i32 = 2;
pub const NVME_UUID_ASSOCIATION_PCI_VID: i32 = 1;
pub const NVME_UUID_ASSOCIATION_RESERVED: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_UUID_LIST {
    pub UUID: [NVME_UUID_LIST_ENTRY; 128],
}
#[cfg(feature = "shared")]
impl Default for NVME_UUID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_UUID_LIST_ENTRY {
    pub _bitfield: u8,
    pub Reserved1: [super::UCHAR; 15],
    pub UUID: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl NVME_UUID_LIST_ENTRY {
    pub fn IdentifierAssociation(&self) -> super::UCHAR {
        (self._bitfield) & 3
    }
    pub fn set_IdentifierAssociation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
}
#[cfg(feature = "shared")]
impl Default for NVME_UUID_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NVME_VENDOR_LOG_PAGES = i32;
pub const NVME_VENDOR_SPECIFIC_ADMIN_COMMAND_MAX_OPCODE: i32 = 255;
pub const NVME_VENDOR_SPECIFIC_ADMIN_COMMAND_MIN_OPCODE: i32 = 192;
pub const NVME_VENDOR_SPECIFIC_FEATURE_MAX_IDENTIFIER: i32 = 255;
pub const NVME_VENDOR_SPECIFIC_FEATURE_MIN_IDENTIFIER: i32 = 192;
pub const NVME_VENDOR_SPECIFIC_LOG_PAGE_MAX_IDENTIFIER: i32 = 255;
pub const NVME_VENDOR_SPECIFIC_LOG_PAGE_MIN_IDENTIFIER: i32 = 192;
pub const NVME_VENDOR_SPECIFIC_NVM_COMMAND_MAX_OPCODE: i32 = 255;
pub const NVME_VENDOR_SPECIFIC_NVM_COMMAND_MIN_OPCODE: i32 = 128;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_VERSION {
    pub Anonymous: NVME_VERSION_0,
    pub AsUlong: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_VERSION_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_VERSION_0 {
    pub fn TER(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_TER(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn MNR(&self) -> super::ULONG {
        (self._bitfield >> 8) & 255
    }
    pub fn set_MNR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 8)) | ((value & 255) << 8);
    }
    pub fn MJR(&self) -> super::ULONG {
        self._bitfield >> 16
    }
    pub fn set_MJR(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(65535 << 16)) | ((value & 65535) << 16);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_CAPABILITIES {
    pub Anonymous: NVME_WCS_DEVICE_CAPABILITIES_0,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_WCS_DEVICE_CAPABILITIES_0 {
    pub Anonymous: NVME_WCS_DEVICE_CAPABILITIES_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_CAPABILITIES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_CAPABILITIES_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl NVME_WCS_DEVICE_CAPABILITIES_0_0 {
    pub fn PanicAEN(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_PanicAEN(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn PanicCFS(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_PanicCFS(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1073741823 << 2)) | ((value & 1073741823) << 2);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    pub PanicResetWaitTime: super::USHORT,
    pub PanicResetAction: NVME_WCS_DEVICE_RESET_ACTION,
    pub DriveRecoveryAction: super::UCHAR,
    pub PanicId: super::ULONGLONG,
    pub DeviceCapabilities: NVME_WCS_DEVICE_CAPABILITIES,
    pub VendorSpecificRecoveryCode: super::UCHAR,
    pub Reserved0: [super::UCHAR; 3],
    pub VendorSpecificCommandCDW12: super::ULONG,
    pub VendorSpecificCommandCDW13: super::ULONG,
    pub Reserved1: [super::UCHAR; 466],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1 = NVME_WCS_DEVICE_ERROR_RECOVERY_LOG;
pub const NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_VERSION_1: i32 = 1;
pub type NVME_WCS_DEVICE_RECOVERY_ACTION1 = i32;
pub type NVME_WCS_DEVICE_RECOVERY_ACTION2 = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_RESET_ACTION {
    pub Anonymous: NVME_WCS_DEVICE_RESET_ACTION_0,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_RESET_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVME_WCS_DEVICE_RESET_ACTION_0 {
    pub Anonymous: NVME_WCS_DEVICE_RESET_ACTION_0_0,
    pub AsUCHAR: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_RESET_ACTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_WCS_DEVICE_RESET_ACTION_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_WCS_DEVICE_RESET_ACTION_0_0 {
    pub fn ControllerReset(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ControllerReset(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn NVMeSubsystemReset(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_NVMeSubsystemReset(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn PCIeFLR(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_PCIeFLR(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn PERST(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_PERST(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn PowerCycle(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_PowerCycle(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn PCIeConventionalHotReset(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_PCIeConventionalHotReset(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(3 << 6)) | ((value & 3) << 6);
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    pub VersionSpecificData: [super::UCHAR; 494],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    pub MediaUnitsWritten: [super::UCHAR; 16],
    pub MediaUnitsRead: [super::UCHAR; 16],
    pub BadUserNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0,
    pub BadSystemNANDBlockCount: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1,
    pub XORRecoveryCount: super::ULONGLONG,
    pub UnrecoverableReadErrorCount: super::ULONGLONG,
    pub SoftECCErrorCount: super::ULONGLONG,
    pub EndToEndCorrectionCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_2,
    pub PercentageSystemDataUsed: super::UCHAR,
    pub RefreshCount: [super::UCHAR; 7],
    pub UserDataEraseCounts: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_3,
    pub ThermalThrottling: NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_4,
    pub Reserved0: [super::UCHAR; 6],
    pub PCIeCorrectableErrorCount: super::ULONGLONG,
    pub IncompleteShutdownCount: super::ULONG,
    pub Reserved1: super::ULONG,
    pub PercentageFreeBlocks: super::UCHAR,
    pub Reserved2: [super::UCHAR; 7],
    pub CapacitorHealth: super::USHORT,
    pub Reserved3: [super::UCHAR; 6],
    pub UnalignedIOCount: super::ULONGLONG,
    pub SecurityVersionNumber: super::ULONGLONG,
    pub NUSE: super::ULONGLONG,
    pub PLPStartCount: [super::UCHAR; 16],
    pub EnduranceEstimate: [super::UCHAR; 16],
    pub Reserved4: [super::UCHAR; 302],
    pub LogPageVersionNumber: super::USHORT,
    pub LogPageGUID: windows_core::GUID,
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0 {
    pub RawCount: [super::UCHAR; 6],
    pub Normalized: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1 {
    pub RawCount: [super::UCHAR; 6],
    pub Normalized: [super::UCHAR; 2],
}
#[cfg(feature = "shared")]
impl Default for NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_2 {
    pub DetectedCounts: super::ULONG,
    pub CorrectedCounts: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_3 {
    pub MaximumCount: super::ULONG,
    pub MinimumCount: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2_4 {
    pub EventCount: super::UCHAR,
    pub Status: super::UCHAR,
}
pub const NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_VERSION_2: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR {
    pub Anonymous: NVME_ZONE_DESCRIPTOR_0,
    pub Anonymous2: NVME_ZONE_DESCRIPTOR_1,
    pub ZA: NVME_ZONE_DESCRIPTOR_2,
    pub Reserved3: [super::UCHAR; 5],
    pub ZCAP: super::ULONGLONG,
    pub ZSLBA: super::ULONGLONG,
    pub WritePointer: super::ULONGLONG,
    pub Reserved4: [super::UCHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for NVME_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_ZONE_DESCRIPTOR_0 {
    pub fn ZT(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_ZT(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn Reserved1(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_Reserved1(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_1 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_ZONE_DESCRIPTOR_1 {
    pub fn Reserved2(&self) -> super::UCHAR {
        (self._bitfield) & 15
    }
    pub fn set_Reserved2(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !15) | (value & 15);
    }
    pub fn ZS(&self) -> super::UCHAR {
        self._bitfield >> 4
    }
    pub fn set_ZS(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 4)) | ((value & 15) << 4);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_2 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVME_ZONE_DESCRIPTOR_2 {
    pub fn ZFC(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_ZFC(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn FZR(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_FZR(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn RZR(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_RZR(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 15
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(15 << 3)) | ((value & 15) << 3);
    }
    pub fn ZDEV(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_ZDEV(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ZONE_DESCRIPTOR_EXTENSION {
    pub ZoneDescriptorExtensionInfo: [super::UCHAR; 64],
}
#[cfg(feature = "shared")]
impl Default for NVME_ZONE_DESCRIPTOR_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVME_ZONE_EXTENDED_REPORT_ZONE_DESC {
    pub ZoneDescriptor: NVME_ZONE_DESCRIPTOR,
    pub ZoneDescriptorExtension: [NVME_ZONE_DESCRIPTOR_EXTENSION; 1],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union NVM_RESERVATION_CAPABILITIES {
    pub Anonymous: NVM_RESERVATION_CAPABILITIES_0,
    pub AsUchar: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for NVM_RESERVATION_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NVM_RESERVATION_CAPABILITIES_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NVM_RESERVATION_CAPABILITIES_0 {
    pub fn PersistThroughPowerLoss(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_PersistThroughPowerLoss(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn WriteExclusiveReservation(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_WriteExclusiveReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn ExclusiveAccessReservation(&self) -> super::UCHAR {
        (self._bitfield >> 2) & 1
    }
    pub fn set_ExclusiveAccessReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn WriteExclusiveRegistrantsOnlyReservation(&self) -> super::UCHAR {
        (self._bitfield >> 3) & 1
    }
    pub fn set_WriteExclusiveRegistrantsOnlyReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn ExclusiveAccessRegistrantsOnlyReservation(&self) -> super::UCHAR {
        (self._bitfield >> 4) & 1
    }
    pub fn set_ExclusiveAccessRegistrantsOnlyReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn WriteExclusiveAllRegistrantsReservation(&self) -> super::UCHAR {
        (self._bitfield >> 5) & 1
    }
    pub fn set_WriteExclusiveAllRegistrantsReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn ExclusiveAccessAllRegistrantsReservation(&self) -> super::UCHAR {
        (self._bitfield >> 6) & 1
    }
    pub fn set_ExclusiveAccessAllRegistrantsReservation(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 7
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NVM_SET_LIST {
    pub IdentifierCount: super::UCHAR,
    pub Reserved: [super::UCHAR; 127],
    pub Entry: [NVME_SET_ATTRIBUTES_ENTRY; 1],
}
#[cfg(feature = "shared")]
impl Default for NVM_SET_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NVM_SUBSYSTEM_SHUTDOWN_ABRUPT: i32 = 1096970356;
pub const NVM_SUBSYSTEM_SHUTDOWN_NORMAL: i32 = 1316121964;
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
#[cfg(feature = "shared")]
pub type PACTIVE_LATENCY_CONFIGURATION = *mut ACTIVE_LATENCY_CONFIGURATION;
#[cfg(feature = "shared")]
pub type PBUCKET_COUNTER = *mut BUCKET_COUNTER;
#[cfg(feature = "shared")]
pub type PDEBUG_BIT_FIELD = *mut DEBUG_BIT_FIELD;
#[cfg(feature = "shared")]
pub type PDSSD_POWER_STATE_DESCRIPTOR = *mut DSSD_POWER_STATE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PFIRMWARE_ACTIVATION_HISTORY_ENTRY = *mut FIRMWARE_ACTIVATION_HISTORY_ENTRY;
#[cfg(feature = "shared")]
pub type PIO_COMMAND_SET_VECTOR = *mut IO_COMMAND_SET_VECTOR;
#[cfg(feature = "shared")]
pub type PLATENCY_MONITOR_FEATURE_STATUS = *mut LATENCY_MONITOR_FEATURE_STATUS;
#[cfg(feature = "shared")]
pub type PLATENCY_STAMP = *mut LATENCY_STAMP;
#[cfg(feature = "shared")]
pub type PLATENCY_STAMP_UNITS = *mut LATENCY_STAMP_UNITS;
#[cfg(feature = "shared")]
pub type PMEASURED_LATENCY = *mut MEASURED_LATENCY;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_DHCHAP_CHALLENGE = *mut NVMEOF_AUTH_DHCHAP_CHALLENGE;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_DHCHAP_DESCRIPTOR = *mut NVMEOF_AUTH_DHCHAP_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_DHCHAP_REPLY = *mut NVMEOF_AUTH_DHCHAP_REPLY;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_DHCHAP_SUCCESS1 = *mut NVMEOF_AUTH_DHCHAP_SUCCESS1;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_DHCHAP_SUCCESS2 = *mut NVMEOF_AUTH_DHCHAP_SUCCESS2;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_FAILURE = *mut NVMEOF_AUTH_FAILURE;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_NEGOTIATE = *mut NVMEOF_AUTH_NEGOTIATE;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_RECEIVE_COMMAND = *mut NVMEOF_AUTH_RECEIVE_COMMAND;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_RECEIVE_RESPONSE = *mut NVMEOF_AUTH_RECEIVE_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_SEND_COMMAND = *mut NVMEOF_AUTH_SEND_COMMAND;
#[cfg(feature = "shared")]
pub type PNVMEOF_AUTH_SEND_RESPONSE = *mut NVMEOF_AUTH_SEND_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVMEOF_CONNECT_COMMAND = *mut NVMEOF_CONNECT_COMMAND;
#[cfg(feature = "shared")]
pub type PNVMEOF_CONNECT_DATA = *mut NVMEOF_CONNECT_DATA;
#[cfg(feature = "shared")]
pub type PNVMEOF_CONNECT_RESPONSE = *mut NVMEOF_CONNECT_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVMEOF_DISCONNECT_RESPONSE = *mut NVMEOF_DISCONNECT_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVMEOF_FABRICS_COMMAND = *mut NVMEOF_FABRICS_COMMAND;
#[cfg(feature = "shared")]
pub type PNVMEOF_FABRICS_RESPONSE = *mut NVMEOF_FABRICS_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVMEOF_PROPERTY_GET_COMMAND = *mut NVMEOF_PROPERTY_GET_COMMAND;
#[cfg(feature = "shared")]
pub type PNVMEOF_PROPERTY_GET_RESPONSE = *mut NVMEOF_PROPERTY_GET_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVMEOF_PROPERTY_SET_COMMAND = *mut NVMEOF_PROPERTY_SET_COMMAND;
#[cfg(feature = "shared")]
pub type PNVMEOF_PROPERTY_SET_RESPONSE = *mut NVMEOF_PROPERTY_SET_RESPONSE;
#[cfg(feature = "shared")]
pub type PNVME_ACTIVE_NAMESPACE_ID_LIST = *mut NVME_ACTIVE_NAMESPACE_ID_LIST;
#[cfg(feature = "shared")]
pub type PNVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS = *mut NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS;
#[cfg(feature = "shared")]
pub type PNVME_ADMIN_QUEUE_ATTRIBUTES = *mut NVME_ADMIN_QUEUE_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PNVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS = *mut NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS;
#[cfg(feature = "shared")]
pub type PNVME_AUTO_POWER_STATE_TRANSITION_ENTRY = *mut NVME_AUTO_POWER_STATE_TRANSITION_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_BOOT_PARTITION_INFORMATION = *mut NVME_BOOT_PARTITION_INFORMATION;
#[cfg(feature = "shared")]
pub type PNVME_BOOT_PARTITION_LOG = *mut NVME_BOOT_PARTITION_LOG;
#[cfg(feature = "shared")]
pub type PNVME_CDW0_FEATURE_DSSD_POWER_STATE = *mut NVME_CDW0_FEATURE_DSSD_POWER_STATE;
#[cfg(feature = "shared")]
pub type PNVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO = *mut NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO;
#[cfg(feature = "shared")]
pub type PNVME_CDW0_FEATURE_ERROR_INJECTION = *mut NVME_CDW11_FEATURE_ERROR_INJECTION;
#[cfg(feature = "shared")]
pub type PNVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE = *mut NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE;
#[cfg(feature = "shared")]
pub type PNVME_CDW0_RESERVATION_PERSISTENCE = *mut NVME_CDW0_RESERVATION_PERSISTENCE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_ABORT = *mut NVME_CDW10_ABORT;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_CREATE_IO_QUEUE = *mut NVME_CDW10_CREATE_IO_QUEUE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_DATASET_MANAGEMENT = *mut NVME_CDW10_DATASET_MANAGEMENT;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_DELETE_IO_QUEUE = *mut NVME_CDW10_DELETE_IO_QUEUE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_DEVICE_SELF_TEST = *mut NVME_CDW10_DEVICE_SELF_TEST;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_DIRECTIVE_RECEIVE = *mut NVME_CDW10_DIRECTIVE_RECEIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_DIRECTIVE_SEND = *mut NVME_CDW10_DIRECTIVE_SEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_DISCOVERY_INFO_MGMT = *mut NVME_CDW10_DISCOVERY_INFO_MGMT;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_FIRMWARE_ACTIVATE = *mut NVME_CDW10_FIRMWARE_ACTIVATE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_FIRMWARE_DOWNLOAD = *mut NVME_CDW10_FIRMWARE_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_FORMAT_NVM = *mut NVME_CDW10_FORMAT_NVM;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_GET_FEATURES = *mut NVME_CDW10_GET_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_GET_LOG_PAGE = *mut NVME_CDW10_GET_LOG_PAGE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_GET_LOG_PAGE_V121 = *mut NVME_CDW10_GET_LOG_PAGE_V121;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_GET_LOG_PAGE_V13 = *mut NVME_CDW10_GET_LOG_PAGE_V13;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_GET_LOG_PAGE_V20 = *mut NVME_CDW10_GET_LOG_PAGE_V20;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_IDENTIFY = *mut NVME_CDW10_IDENTIFY;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_RESERVATION_ACQUIRE = *mut NVME_CDW10_RESERVATION_ACQUIRE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_RESERVATION_REGISTER = *mut NVME_CDW10_RESERVATION_REGISTER;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_RESERVATION_RELEASE = *mut NVME_CDW10_RESERVATION_RELEASE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_RESERVATION_REPORT = *mut NVME_CDW10_RESERVATION_REPORT;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_SANITIZE = *mut NVME_CDW10_SANITIZE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_SECURITY_SEND_RECEIVE = *mut NVME_CDW10_SECURITY_SEND_RECEIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_SET_FEATURES = *mut NVME_CDW10_SET_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_ZONE_APPEND = *mut NVME_CDW10_ZONE_APPEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_ZONE_MANAGEMENT_RECEIVE = *mut NVME_CDW10_ZONE_MANAGEMENT_RECEIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW10_ZONE_MANAGEMENT_SEND = *mut NVME_CDW10_ZONE_MANAGEMENT_SEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_CREATE_IO_CQ = *mut NVME_CDW11_CREATE_IO_CQ;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_CREATE_IO_SQ = *mut NVME_CDW11_CREATE_IO_SQ;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_DATASET_MANAGEMENT = *mut NVME_CDW11_DATASET_MANAGEMENT;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_DIRECTIVE_RECEIVE = *mut NVME_CDW11_DIRECTIVE_RECEIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_DIRECTIVE_SEND = *mut NVME_CDW11_DIRECTIVE_SEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURES = *mut NVME_CDW11_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_ARBITRATION = *mut NVME_CDW11_FEATURE_ARBITRATION;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG = *mut NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION = *mut NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY = *mut NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS = *mut NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO = *mut NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_ERROR_INJECTION = *mut NVME_CDW11_FEATURE_ERROR_INJECTION;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_ERROR_RECOVERY = *mut NVME_CDW11_FEATURE_ERROR_RECOVERY;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_GET_HOST_METADATA = *mut NVME_CDW11_FEATURE_GET_HOST_METADATA;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_HOST_IDENTIFIER = *mut NVME_CDW11_FEATURE_HOST_IDENTIFIER;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_INTERRUPT_COALESCING = *mut NVME_CDW11_FEATURE_INTERRUPT_COALESCING;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG = *mut NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE = *mut NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_LBA_RANGE_TYPE = *mut NVME_CDW11_FEATURE_LBA_RANGE_TYPE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE = *mut NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_NUMBER_OF_QUEUES = *mut NVME_CDW11_FEATURE_NUMBER_OF_QUEUES;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_POWER_MANAGEMENT = *mut NVME_CDW11_FEATURE_POWER_MANAGEMENT;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE = *mut NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK = *mut NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_RESERVATION_PERSISTENCE = *mut NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_SET_HOST_METADATA = *mut NVME_CDW11_FEATURE_SET_HOST_METADATA;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_SUPPORTED_CAPABILITY = *mut NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD = *mut NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE = *mut NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL = *mut NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_FIRMWARE_DOWNLOAD = *mut NVME_CDW11_FIRMWARE_DOWNLOAD;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_GET_LOG_PAGE = *mut NVME_CDW11_GET_LOG_PAGE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_IDENTIFY = *mut NVME_CDW11_IDENTIFY;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_RESERVATION_REPORT = *mut NVME_CDW11_RESERVATION_REPORT;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_SECURITY_RECEIVE = *mut NVME_CDW11_SECURITY_RECEIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW11_SECURITY_SEND = *mut NVME_CDW11_SECURITY_SEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES = *mut NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE = *mut NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_FEATURES = *mut NVME_CDW12_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_GET_LOG_PAGE = *mut NVME_CDW12_GET_LOG_PAGE;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_READ_WRITE = *mut NVME_CDW12_READ_WRITE;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_VERIFYCOMMAND = *mut NVME_CDW12_VERIFYCOMMAND;
#[cfg(feature = "shared")]
pub type PNVME_CDW12_ZONE_APPEND = *mut NVME_CDW12_ZONE_APPEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW13_FEATURES = *mut NVME_CDW13_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW13_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER;
#[cfg(feature = "shared")]
pub type PNVME_CDW13_GET_LOG_PAGE = *mut NVME_CDW13_GET_LOG_PAGE;
#[cfg(feature = "shared")]
pub type PNVME_CDW13_READ_WRITE = *mut NVME_CDW13_READ_WRITE;
#[cfg(feature = "shared")]
pub type PNVME_CDW13_ZONE_MANAGEMENT_RECEIVE = *mut NVME_CDW13_ZONE_MANAGEMENT_RECEIVE;
#[cfg(feature = "shared")]
pub type PNVME_CDW13_ZONE_MANAGEMENT_SEND = *mut NVME_CDW13_ZONE_MANAGEMENT_SEND;
#[cfg(feature = "shared")]
pub type PNVME_CDW14_FEATURES = *mut NVME_CDW14_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW14_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER;
#[cfg(feature = "shared")]
pub type PNVME_CDW14_GET_LOG_PAGE = *mut NVME_CDW14_GET_LOG_PAGE;
#[cfg(feature = "shared")]
pub type PNVME_CDW14_GET_LOG_PAGE_V20 = *mut NVME_CDW14_GET_LOG_PAGE_V20;
#[cfg(feature = "shared")]
pub type PNVME_CDW14_IDENTIFY = *mut NVME_CDW14_IDENTIFY;
#[cfg(feature = "shared")]
pub type PNVME_CDW15_FEATURES = *mut NVME_CDW15_FEATURES;
#[cfg(feature = "shared")]
pub type PNVME_CDW15_FEATURE_HOST_MEMORY_BUFFER = *mut NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER;
#[cfg(feature = "shared")]
pub type PNVME_CDW15_READ_WRITE = *mut NVME_CDW15_READ_WRITE;
#[cfg(feature = "shared")]
pub type PNVME_CDW15_VERIFY_COMMAND = *mut NVME_CDW15_VERIFY_COMMAND;
#[cfg(feature = "shared")]
pub type PNVME_CDW15_ZONE_APPEND = *mut NVME_CDW15_ZONE_APPEND;
#[cfg(feature = "shared")]
pub type PNVME_CHANGED_NAMESPACE_LIST_LOG = *mut NVME_CHANGED_NAMESPACE_LIST_LOG;
#[cfg(feature = "shared")]
pub type PNVME_CHANGED_ZONE_LIST_LOG = *mut NVME_CHANGED_ZONE_LIST_LOG;
#[cfg(feature = "shared")]
pub type PNVME_COMMAND = *mut NVME_COMMAND;
#[cfg(feature = "shared")]
pub type PNVME_COMMAND_DWORD0 = *mut NVME_COMMAND_DWORD0;
#[cfg(feature = "shared")]
pub type PNVME_COMMAND_EFFECTS_DATA = *mut NVME_COMMAND_EFFECTS_DATA;
#[cfg(feature = "shared")]
pub type PNVME_COMMAND_EFFECTS_LOG = *mut NVME_COMMAND_EFFECTS_LOG;
#[cfg(feature = "shared")]
pub type PNVME_COMMAND_STATUS = *mut NVME_COMMAND_STATUS;
#[cfg(feature = "shared")]
pub type PNVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST = *mut NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST;
#[cfg(feature = "shared")]
pub type PNVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES = *mut NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES;
#[cfg(feature = "shared")]
pub type PNVME_COMPLETION_ENTRY = *mut NVME_COMPLETION_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_COMPLETION_QUEUE_HEAD_DOORBELL = *mut NVME_COMPLETION_QUEUE_HEAD_DOORBELL;
#[cfg(feature = "shared")]
pub type PNVME_CONTEXT_ATTRIBUTES = *mut NVME_CONTEXT_ATTRIBUTES;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_CAPABILITIES = *mut NVME_CONTROLLER_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_CONFIGURATION = *mut NVME_CONTROLLER_CONFIGURATION;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_LIST = *mut NVME_CONTROLLER_LIST;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_MEMORY_BUFFER_LOCATION = *mut NVME_CONTROLLER_MEMORY_BUFFER_LOCATION;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_MEMORY_BUFFER_SIZE = *mut NVME_CONTROLLER_MEMORY_BUFFER_SIZE;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_READY_TIMEOUTS = *mut NVME_CONTROLLER_READY_TIMEOUTS;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_REGISTERS = *mut NVME_CONTROLLER_REGISTERS;
#[cfg(feature = "shared")]
pub type PNVME_CONTROLLER_STATUS = *mut NVME_CONTROLLER_STATUS;
#[cfg(feature = "shared")]
pub type PNVME_DEVICE_SELF_TEST_LOG = *mut NVME_DEVICE_SELF_TEST_LOG;
#[cfg(feature = "shared")]
pub type PNVME_DEVICE_SELF_TEST_RESULT_DATA = *mut NVME_DEVICE_SELF_TEST_RESULT_DATA;
#[cfg(feature = "shared")]
pub type PNVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS = *mut NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS;
#[cfg(feature = "shared")]
pub type PNVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR = *mut NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PNVME_DIRECTIVE_STREAMS_GET_STATUS_DATA = *mut NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA;
#[cfg(feature = "shared")]
pub type PNVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS = *mut NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS;
#[cfg(feature = "shared")]
pub type PNVME_DISCOVERY_ENTRY = *mut NVME_DISCOVERY_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_DISCOVERY_HEADER = *mut NVME_DISCOVERY_HEADER;
#[cfg(feature = "shared")]
pub type PNVME_DISCOVERY_INFO_MGMT_HEADER = *mut NVME_DISCOVERY_INFO_MGMT_HEADER;
#[cfg(feature = "shared")]
pub type PNVME_ENDURANCE_GROUP_LOG = *mut NVME_ENDURANCE_GROUP_LOG;
#[cfg(feature = "shared")]
pub type PNVME_ERROR_INFO_LOG = *mut NVME_ERROR_INFO_LOG;
#[cfg(feature = "shared")]
pub type PNVME_ERROR_INJECTION_ENTRY = *mut NVME_ERROR_INJECTION_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_EXTENDED_ATTR = *mut NVME_EXTENDED_ATTR;
#[cfg(feature = "shared")]
pub type PNVME_EXTENDED_DISCOVERY_ENTRY = *mut NVME_EXTENDED_DISCOVERY_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_EXTENDED_REPORT_ZONE_INFO = *mut NVME_EXTENDED_REPORT_ZONE_INFO;
#[cfg(feature = "shared")]
pub type PNVME_FEATURE_HOST_IDENTIFIER_DATA = *mut NVME_FEATURE_HOST_IDENTIFIER_DATA;
#[cfg(feature = "shared")]
pub type PNVME_FEATURE_HOST_METADATA_DATA = *mut NVME_FEATURE_HOST_METADATA_DATA;
#[cfg(feature = "shared")]
pub type PNVME_FEATURE_IDENTIFIERS_EFFECTS_LOG = *mut NVME_FEATURE_IDENTIFIERS_EFFECTS_LOG;
#[cfg(feature = "shared")]
pub type PNVME_FID_SUPPORTED_AND_EFFECTS = *mut NVME_FID_SUPPORTED_AND_EFFECTS;
#[cfg(feature = "shared")]
pub type PNVME_FIRMWARE_SLOT_INFO_LOG = *mut NVME_FIRMWARE_SLOT_INFO_LOG;
#[cfg(feature = "shared")]
pub type PNVME_GET_FEATURE_TIMESTAMP = *mut NVME_GET_FEATURE_TIMESTAMP;
#[cfg(feature = "shared")]
pub type PNVME_HEALTH_INFO_LOG = *mut NVME_HEALTH_INFO_LOG;
#[cfg(feature = "shared")]
pub type PNVME_HOST_BEHAVIOR_SUPPORT_DATA = *mut NVME_HOST_BEHAVIOR_SUPPORT_DATA;
#[cfg(feature = "shared")]
pub type PNVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY = *mut NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_HOST_METADATA_ELEMENT_DESCRIPTOR = *mut NVME_HOST_METADATA_ELEMENT_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_CONTROLLER_DATA = *mut NVME_IDENTIFY_CONTROLLER_DATA;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_IO_COMMAND_SET = *mut NVME_IDENTIFY_IO_COMMAND_SET;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_NAMESPACE_DATA = *mut NVME_IDENTIFY_NAMESPACE_DATA;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_NAMESPACE_DESCRIPTOR = *mut NVME_IDENTIFY_NAMESPACE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET = *mut NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET = *mut NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET;
#[cfg(feature = "shared")]
pub type PNVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET = *mut NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET;
#[cfg(feature = "shared")]
pub type PNVME_LATENCY_MONITORING_ENTRY = *mut NVME_LATENCY_MONITORING_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_LBA_FORMAT = *mut NVME_LBA_FORMAT;
#[cfg(feature = "shared")]
pub type PNVME_LBA_RANGE = *mut NVME_LBA_RANGE;
#[cfg(feature = "shared")]
pub type PNVME_LBA_RANGET_TYPE_ENTRY = *mut NVME_LBA_RANGET_TYPE_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_LBA_ZONE_FORMAT = *mut NVME_LBA_ZONE_FORMAT;
#[cfg(feature = "shared")]
pub type PNVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG = *mut NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG;
#[cfg(feature = "shared")]
pub type PNVME_LID_SUPPORTED_AND_EFFECTS = *mut NVME_LID_SUPPORTED_AND_EFFECTS;
pub type PNVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE = *mut NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE;
#[cfg(feature = "shared")]
pub type PNVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS = *mut NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS;
#[cfg(feature = "shared")]
pub type PNVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG = *mut NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG;
#[cfg(feature = "shared")]
pub type PNVME_NVM_SUBSYSTEM_RESET = *mut NVME_NVM_SUBSYSTEM_RESET;
#[cfg(feature = "shared")]
pub type PNVME_NVM_SUBSYSTEM_SHUTDOWN = *mut NVME_NVM_SUBSYSTEM_SHUTDOWN;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_CAPABILITIES_LOG = *mut NVME_OCP_DEVICE_CAPABILITIES_LOG;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 = *mut NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG = *mut NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_LATENCY_MONITOR_LOG = *mut NVME_OCP_DEVICE_LATENCY_MONITOR_LOG;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 = *mut NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_TCG_CONFIGURATION_LOG = *mut NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_TCG_HISTORY_LOG = *mut NVME_OCP_DEVICE_TCG_HISTORY_LOG;
#[cfg(feature = "shared")]
pub type PNVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG = *mut NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG;
#[cfg(feature = "shared")]
pub type PNVME_PERSISTENT_EVENT_LOG_EVENT_HEADER = *mut NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER;
#[cfg(feature = "shared")]
pub type PNVME_PERSISTENT_EVENT_LOG_HEADER = *mut NVME_PERSISTENT_EVENT_LOG_HEADER;
#[cfg(feature = "shared")]
pub type PNVME_POWER_STATE_DESC = *mut NVME_POWER_STATE_DESC;
#[cfg(feature = "shared")]
pub type PNVME_PRP_ENTRY = *mut NVME_PRP_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_REGISTERED_CONTROLLER_DATA = *mut NVME_REGISTERED_CONTROLLER_DATA;
#[cfg(feature = "shared")]
pub type PNVME_REGISTERED_CONTROLLER_EXTENDED_DATA = *mut NVME_REGISTERED_CONTROLLER_EXTENDED_DATA;
#[cfg(feature = "shared")]
pub type PNVME_REPORT_ZONE_INFO = *mut NVME_REPORT_ZONE_INFO;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_ACQUIRE_DATA_STRUCTURE = *mut NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_CAPABILITIES = *mut NVM_RESERVATION_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_NOTIFICATION_LOG = *mut NVME_RESERVATION_NOTIFICATION_LOG;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_REGISTER_DATA_STRUCTURE = *mut NVME_RESERVATION_REGISTER_DATA_STRUCTURE;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_RELEASE_DATA_STRUCTURE = *mut NVME_RESERVATION_RELEASE_DATA_STRUCTURE;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE = *mut NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE = *mut NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE;
#[cfg(feature = "shared")]
pub type PNVME_RESERVATION_REPORT_STATUS_HEADER = *mut NVME_RESERVATION_REPORT_STATUS_HEADER;
pub type PNVME_SANITIZE_ACTION = *mut NVME_SANITIZE_ACTION;
pub type PNVME_SANITIZE_OPERATION_STATUS = *mut NVME_SANITIZE_OPERATION_STATUS;
#[cfg(feature = "shared")]
pub type PNVME_SANITIZE_STATUS = *mut NVME_SANITIZE_STATUS;
#[cfg(feature = "shared")]
pub type PNVME_SANITIZE_STATUS_LOG = *mut NVME_SANITIZE_STATUS_LOG;
#[cfg(feature = "shared")]
pub type PNVME_SCSI_NAME_STRING = *mut NVME_SCSI_NAME_STRING;
#[cfg(feature = "shared")]
pub type PNVME_SET_ATTRIBUTES_ENTRY = *mut NVME_SET_ATTRIBUTES_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_SGL_DESC = *mut NVME_SGL_DESC;
#[cfg(feature = "shared")]
pub type PNVME_SUBMISSION_QUEUE_TAIL_DOORBELL = *mut NVME_SUBMISSION_QUEUE_TAIL_DOORBELL;
#[cfg(feature = "shared")]
pub type PNVME_SUPPORTED_LOG_PAGES_LOG = *mut NVME_SUPPORTED_LOG_PAGES_LOG;
#[cfg(feature = "shared")]
pub type PNVME_TELEMETRY_CONTROLLER_INITIATED_LOG = *mut NVME_TELEMETRY_CONTROLLER_INITIATED_LOG;
#[cfg(feature = "shared")]
pub type PNVME_TELEMETRY_HOST_INITIATED_LOG = *mut NVME_TELEMETRY_HOST_INITIATED_LOG;
#[cfg(feature = "shared")]
pub type PNVME_UUID_LIST = *mut NVME_UUID_LIST;
#[cfg(feature = "shared")]
pub type PNVME_UUID_LIST_ENTRY = *mut NVME_UUID_LIST_ENTRY;
#[cfg(feature = "shared")]
pub type PNVME_VERSION = *mut NVME_VERSION;
#[cfg(feature = "shared")]
pub type PNVME_WCS_DEVICE_CAPABILITIES = *mut NVME_WCS_DEVICE_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PNVME_WCS_DEVICE_ERROR_RECOVERY_LOG = *mut NVME_WCS_DEVICE_ERROR_RECOVERY_LOG;
#[cfg(feature = "shared")]
pub type PNVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1 = *mut NVME_WCS_DEVICE_ERROR_RECOVERY_LOG;
pub type PNVME_WCS_DEVICE_RECOVERY_ACTION1 = *mut NVME_WCS_DEVICE_RECOVERY_ACTION1;
pub type PNVME_WCS_DEVICE_RECOVERY_ACTION2 = *mut NVME_WCS_DEVICE_RECOVERY_ACTION2;
#[cfg(feature = "shared")]
pub type PNVME_WCS_DEVICE_RESET_ACTION = *mut NVME_WCS_DEVICE_RESET_ACTION;
#[cfg(feature = "shared")]
pub type PNVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG = *mut NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG;
#[cfg(feature = "shared")]
pub type PNVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 = *mut NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2;
#[cfg(feature = "shared")]
pub type PNVME_ZONE_DESCRIPTOR = *mut NVME_ZONE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PNVME_ZONE_DESCRIPTOR_EXTENSION = *mut NVME_ZONE_DESCRIPTOR_EXTENSION;
#[cfg(feature = "shared")]
pub type PNVME_ZONE_EXTENDED_REPORT_ZONE_DESC = *mut NVME_ZONE_EXTENDED_REPORT_ZONE_DESC;
#[cfg(feature = "shared")]
pub type PNVM_SET_LIST = *mut NVM_SET_LIST;
#[cfg(feature = "shared")]
pub type PTCG_ACTIVATE_METHOD_SPECIFIC = *mut TCG_ACTIVATE_METHOD_SPECIFIC;
#[cfg(feature = "shared")]
pub type PTCG_ASSIGN_METHOD_SPECIFIC = *mut TCG_ASSIGN_METHOD_SPECIFIC;
#[cfg(feature = "shared")]
pub type PTCG_AUTH_METHOD_SPECIFIC = *mut TCG_AUTH_METHOD_SPECIFIC;
#[cfg(feature = "shared")]
pub type PTCG_BLOCKSID_METHOD_SPECIFIC = *mut TCG_BLOCKSID_METHOD_SPECIFIC;
#[cfg(feature = "shared")]
pub type PTCG_HISTORY_ENTRY = *mut TCG_HISTORY_ENTRY;
#[cfg(feature = "shared")]
pub type PTCG_REACTIVATE_METHOD_SPECIFIC = *mut TCG_REACTIVATE_METHOD_SPECIFIC;
#[cfg(feature = "shared")]
pub type PUNSUPPORTED_REQUIREMENT = *mut UNSUPPORTED_REQUIREMENT;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TCG_ACTIVATE_METHOD_SPECIFIC {
    pub RangeStartLengthPolicy: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TCG_ASSIGN_METHOD_SPECIFIC {
    pub NamespaceId: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TCG_AUTH_METHOD_SPECIFIC {
    pub AuthorityId: super::ULONGLONG,
    pub TriesCount: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TCG_BLOCKSID_METHOD_SPECIFIC {
    pub ClearEvents: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct TCG_HISTORY_ENTRY {
    pub VersionNumber: super::UCHAR,
    pub EntryLength: super::UCHAR,
    pub PowerCycleCount: super::USHORT,
    pub TcgCommandCount: super::ULONG,
    pub TcgCommandCompletionTS: super::ULONGLONG,
    pub InvokingId: super::ULONGLONG,
    pub MethodId: super::ULONGLONG,
    pub ComId: super::USHORT,
    pub ProtocolId: super::UCHAR,
    pub TcgStatus: super::UCHAR,
    pub ProcessTime: super::USHORT,
    pub CommandSpecific: [super::UCHAR; 10],
}
#[cfg(feature = "shared")]
impl Default for TCG_HISTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TCG_HISTORY_ENTRY_VERSION_1: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TCG_REACTIVATE_METHOD_SPECIFIC {
    pub RangeStartLengthPolicy: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct UNSUPPORTED_REQUIREMENT {
    pub ReqId: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for UNSUPPORTED_REQUIREMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ZDES_SIZE_MULTIPLIER_IN_BYTES: i32 = 64;
pub type ZONE_STATE = i32;
