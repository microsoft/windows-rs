pub const ANNEX_A_DSM_CC: MPEG2StreamType = 8;
pub const ATSCCT_AC3: ATSCComponentTypeFlags = 1;
pub type ATSCComponentTypeFlags = i32;
pub type ApplicationTypeType = i32;
pub const BDACOMP_EXCLUDE_TS_FROM_TR: BDA_Comp_Flags = 1;
pub const BDACOMP_INCLUDE_COMPONENTS_IN_TR: BDA_Comp_Flags = 4;
pub const BDACOMP_INCLUDE_LOCATOR_IN_TR: BDA_Comp_Flags = 2;
pub const BDACOMP_NOT_DEFINED: BDA_Comp_Flags = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDANODE_DESCRIPTOR {
    pub ulBdaNodeType: u32,
    pub guidFunction: windows_core::GUID,
    pub guidName: windows_core::GUID,
}
pub const BDA_BCC_RATE_1_2: BinaryConvolutionCodeRate = 1;
pub const BDA_BCC_RATE_1_3: BinaryConvolutionCodeRate = 10;
pub const BDA_BCC_RATE_1_4: BinaryConvolutionCodeRate = 9;
pub const BDA_BCC_RATE_2_3: BinaryConvolutionCodeRate = 2;
pub const BDA_BCC_RATE_2_5: BinaryConvolutionCodeRate = 11;
pub const BDA_BCC_RATE_3_4: BinaryConvolutionCodeRate = 3;
pub const BDA_BCC_RATE_3_5: BinaryConvolutionCodeRate = 4;
pub const BDA_BCC_RATE_4_5: BinaryConvolutionCodeRate = 5;
pub const BDA_BCC_RATE_5_11: BinaryConvolutionCodeRate = 7;
pub const BDA_BCC_RATE_5_6: BinaryConvolutionCodeRate = 6;
pub const BDA_BCC_RATE_6_7: BinaryConvolutionCodeRate = 12;
pub const BDA_BCC_RATE_7_8: BinaryConvolutionCodeRate = 8;
pub const BDA_BCC_RATE_8_9: BinaryConvolutionCodeRate = 13;
pub const BDA_BCC_RATE_9_10: BinaryConvolutionCodeRate = 14;
pub const BDA_BCC_RATE_MAX: BinaryConvolutionCodeRate = 15;
pub const BDA_BCC_RATE_NOT_DEFINED: BinaryConvolutionCodeRate = 0;
pub const BDA_BCC_RATE_NOT_SET: BinaryConvolutionCodeRate = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_BUFFER {
    pub lResult: PBDARESULT,
    pub ulBufferSize: u32,
    pub argbBuffer: [u8; 1],
}
impl Default for BDA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_CAS_CHECK_ENTITLEMENTTOKEN {
    pub lResult: PBDARESULT,
    pub ulDescrambleStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_CAS_CLOSEMMIDATA {
    pub ulDialogNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_CAS_CLOSE_MMIDIALOG {
    pub lResult: PBDARESULT,
    pub SessionResult: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_CAS_OPENMMIDATA {
    pub ulDialogNumber: u32,
    pub ulDialogRequest: u32,
    pub uuidDialogType: windows_core::GUID,
    pub usDialogDataLength: u16,
    pub argbDialogData: [u8; 1],
}
impl Default for BDA_CAS_OPENMMIDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_CAS_REQUESTTUNERDATA {
    pub ucRequestPriority: u8,
    pub ucRequestReason: u8,
    pub ucRequestConsequences: u8,
    pub ulEstimatedTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_CA_MODULE_UI {
    pub ulFormat: u32,
    pub ulbcDesc: u32,
    pub ulDesc: [u32; 1],
}
impl Default for BDA_CA_MODULE_UI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_CHANGES_COMPLETE: BDA_CHANGE_STATE = 0;
pub const BDA_CHANGES_PENDING: BDA_CHANGE_STATE = 1;
pub type BDA_CHANGE_STATE = i32;
pub const BDA_CHAN_BANDWITH_NOT_DEFINED: BDA_Channel_Bandwidth = 0;
pub const BDA_CHAN_BANDWITH_NOT_SET: BDA_Channel_Bandwidth = -1;
pub type BDA_CONDITIONALACCESS_MMICLOSEREASON = i32;
pub type BDA_CONDITIONALACCESS_REQUESTTYPE = i32;
pub type BDA_CONDITIONALACCESS_SESSION_RESULT = i32;
pub type BDA_Channel = i32;
pub type BDA_Channel_Bandwidth = i32;
pub type BDA_Comp_Flags = i32;
pub const BDA_DISCOVERY_COMPLETE: BDA_DISCOVERY_STATE = 2;
pub const BDA_DISCOVERY_REQUIRED: BDA_DISCOVERY_STATE = 1;
pub type BDA_DISCOVERY_STATE = i32;
pub const BDA_DISCOVERY_UNSPECIFIED: BDA_DISCOVERY_STATE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_DISEQC_RESPONSE {
    pub ulRequestId: u32,
    pub ulPacketLength: u32,
    pub argbPacketData: [u8; 8],
}
impl Default for BDA_DISEQC_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_DISEQC_SEND {
    pub ulRequestId: u32,
    pub ulPacketLength: u32,
    pub argbPacketData: [u8; 8],
}
impl Default for BDA_DISEQC_SEND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_DRM_DRMSTATUS {
    pub lResult: PBDARESULT,
    pub DRMuuid: windows_core::GUID,
    pub ulDrmUuidListStringSize: u32,
    pub argbDrmUuidListString: [windows_core::GUID; 1],
}
impl Default for BDA_DRM_DRMSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_DVBT2_L1_SIGNALLING_DATA {
    pub L1Pre_TYPE: u8,
    pub L1Pre_BWT_S1_S2: u8,
    pub L1Pre_REPETITION_GUARD_PAPR: u8,
    pub L1Pre_MOD_COD_FEC: u8,
    pub L1Pre_POSTSIZE_INFO_PILOT: [u8; 5],
    pub L1Pre_TX_ID_AVAIL: u8,
    pub L1Pre_CELL_ID: [u8; 2],
    pub L1Pre_NETWORK_ID: [u8; 2],
    pub L1Pre_T2SYSTEM_ID: [u8; 2],
    pub L1Pre_NUM_T2_FRAMES: u8,
    pub L1Pre_NUM_DATA_REGENFLAG_L1POSTEXT: [u8; 2],
    pub L1Pre_NUMRF_CURRENTRF_RESERVED: [u8; 2],
    pub L1Pre_CRC32: [u8; 4],
    pub L1PostData: [u8; 1],
}
impl Default for BDA_DVBT2_L1_SIGNALLING_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_ETHERNET_ADDRESS {
    pub rgbAddress: [u8; 6],
}
impl Default for BDA_ETHERNET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_ETHERNET_ADDRESS_LIST {
    pub ulcAddresses: u32,
    pub rgAddressl: [BDA_ETHERNET_ADDRESS; 1],
}
impl Default for BDA_ETHERNET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_EVENT_ACCESS_DENIED: BDA_EVENT_ID = 15;
pub const BDA_EVENT_ACCESS_GRANTED: BDA_EVENT_ID = 14;
pub const BDA_EVENT_CHANNEL_ACQUIRED: BDA_EVENT_ID = 4;
pub const BDA_EVENT_CHANNEL_ACTIVATED: BDA_EVENT_ID = 7;
pub const BDA_EVENT_CHANNEL_DEACTIVATED: BDA_EVENT_ID = 8;
pub const BDA_EVENT_CHANNEL_LOST: BDA_EVENT_ID = 5;
pub const BDA_EVENT_CHANNEL_SOURCE_CHANGED: BDA_EVENT_ID = 6;
pub const BDA_EVENT_DATA_START: BDA_EVENT_ID = 2;
pub const BDA_EVENT_DATA_STOP: BDA_EVENT_ID = 3;
pub type BDA_EVENT_ID = i32;
pub const BDA_EVENT_OFFER_EXTENDED: BDA_EVENT_ID = 16;
pub const BDA_EVENT_PURCHASE_COMPLETED: BDA_EVENT_ID = 17;
pub const BDA_EVENT_SIGNAL_LOCK: BDA_EVENT_ID = 1;
pub const BDA_EVENT_SIGNAL_LOSS: BDA_EVENT_ID = 0;
pub const BDA_EVENT_SMART_CARD_INSERTED: BDA_EVENT_ID = 18;
pub const BDA_EVENT_SMART_CARD_REMOVED: BDA_EVENT_ID = 19;
pub const BDA_EVENT_SUBCHANNEL_ACQUIRED: BDA_EVENT_ID = 9;
pub const BDA_EVENT_SUBCHANNEL_ACTIVATED: BDA_EVENT_ID = 12;
pub const BDA_EVENT_SUBCHANNEL_DEACTIVATED: BDA_EVENT_ID = 13;
pub const BDA_EVENT_SUBCHANNEL_LOST: BDA_EVENT_ID = 10;
pub const BDA_EVENT_SUBCHANNEL_SOURCE_CHANGED: BDA_EVENT_ID = 11;
pub const BDA_FEC_BCH: FECMethod = 4;
pub const BDA_FEC_LDPC: FECMethod = 3;
pub const BDA_FEC_MAX: FECMethod = 6;
pub const BDA_FEC_METHOD_NOT_DEFINED: FECMethod = 0;
pub const BDA_FEC_METHOD_NOT_SET: FECMethod = -1;
pub const BDA_FEC_RS_147_130: FECMethod = 5;
pub const BDA_FEC_RS_204_188: FECMethod = 2;
pub const BDA_FEC_VITERBI: FECMethod = 1;
pub const BDA_FILTERED_MULTICAST: BDA_MULTICAST_MODE = 1;
pub const BDA_FREQUENCY_MULTIPLIER_NOT_DEFINED: BDA_Frequency_Multiplier = 0;
pub const BDA_FREQUENCY_MULTIPLIER_NOT_SET: BDA_Frequency_Multiplier = -1;
pub const BDA_FREQUENCY_NOT_DEFINED: BDA_Frequency = 0;
pub const BDA_FREQUENCY_NOT_SET: BDA_Frequency = -1;
pub type BDA_Frequency = i32;
pub type BDA_Frequency_Multiplier = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_GDDS_DATA {
    pub lResult: PBDARESULT,
    pub ulDataLength: u32,
    pub ulPercentageProgress: u32,
    pub argbData: [u8; 1],
}
impl Default for BDA_GDDS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_GDDS_DATATYPE {
    pub lResult: PBDARESULT,
    pub uuidDataType: windows_core::GUID,
}
pub const BDA_GUARD_19_128: GuardInterval = 6;
pub const BDA_GUARD_19_256: GuardInterval = 7;
pub const BDA_GUARD_1_128: GuardInterval = 5;
pub const BDA_GUARD_1_16: GuardInterval = 2;
pub const BDA_GUARD_1_32: GuardInterval = 1;
pub const BDA_GUARD_1_4: GuardInterval = 4;
pub const BDA_GUARD_1_8: GuardInterval = 3;
pub const BDA_GUARD_MAX: GuardInterval = 8;
pub const BDA_GUARD_NOT_DEFINED: GuardInterval = 0;
pub const BDA_GUARD_NOT_SET: GuardInterval = -1;
pub const BDA_HALPHA_1: HierarchyAlpha = 1;
pub const BDA_HALPHA_2: HierarchyAlpha = 2;
pub const BDA_HALPHA_4: HierarchyAlpha = 3;
pub const BDA_HALPHA_MAX: HierarchyAlpha = 4;
pub const BDA_HALPHA_NOT_DEFINED: HierarchyAlpha = 0;
pub const BDA_HALPHA_NOT_SET: HierarchyAlpha = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_IPv4_ADDRESS {
    pub rgbAddress: [u8; 4],
}
impl Default for BDA_IPv4_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_IPv4_ADDRESS_LIST {
    pub ulcAddresses: u32,
    pub rgAddressl: [BDA_IPv4_ADDRESS; 1],
}
impl Default for BDA_IPv4_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_IPv6_ADDRESS {
    pub rgbAddress: [u8; 6],
}
impl Default for BDA_IPv6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_IPv6_ADDRESS_LIST {
    pub ulcAddresses: u32,
    pub rgAddressl: [BDA_IPv6_ADDRESS; 1],
}
impl Default for BDA_IPv6_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_ISDBCAS_EMG_REQ {
    pub bCLA: u8,
    pub bINS: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bLC: u8,
    pub bCardId: [u8; 6],
    pub bProtocol: u8,
    pub bCABroadcasterGroupId: u8,
    pub bMessageControl: u8,
    pub bMessageCode: [u8; 1],
}
impl Default for BDA_ISDBCAS_EMG_REQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct BDA_ISDBCAS_REQUESTHEADER {
    pub bInstruction: u8,
    pub bReserved: [u8; 3],
    pub ulDataLength: u32,
    pub argbIsdbCommand: [u8; 1],
}
impl Default for BDA_ISDBCAS_REQUESTHEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct BDA_ISDBCAS_RESPONSEDATA {
    pub lResult: PBDARESULT,
    pub ulRequestID: u32,
    pub ulIsdbStatus: u32,
    pub ulIsdbDataSize: u32,
    pub argbIsdbCommandData: [u8; 1],
}
impl Default for BDA_ISDBCAS_RESPONSEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_LNB_SOURCE_A: LNB_Source = 1;
pub const BDA_LNB_SOURCE_B: LNB_Source = 2;
pub const BDA_LNB_SOURCE_C: LNB_Source = 3;
pub const BDA_LNB_SOURCE_D: LNB_Source = 4;
pub const BDA_LNB_SOURCE_MAX: LNB_Source = 5;
pub const BDA_LNB_SOURCE_NOT_DEFINED: LNB_Source = 0;
pub const BDA_LNB_SOURCE_NOT_SET: LNB_Source = -1;
pub const BDA_MOD_1024QAM: ModulationType = 19;
pub const BDA_MOD_112QAM: ModulationType = 6;
pub const BDA_MOD_128QAM: ModulationType = 7;
pub const BDA_MOD_160QAM: ModulationType = 8;
pub const BDA_MOD_16APSK: ModulationType = 29;
pub const BDA_MOD_16QAM: ModulationType = 1;
pub const BDA_MOD_16VSB: ModulationType = 24;
pub const BDA_MOD_192QAM: ModulationType = 9;
pub const BDA_MOD_224QAM: ModulationType = 10;
pub const BDA_MOD_256QAM: ModulationType = 11;
pub const BDA_MOD_320QAM: ModulationType = 12;
pub const BDA_MOD_32APSK: ModulationType = 30;
pub const BDA_MOD_32QAM: ModulationType = 2;
pub const BDA_MOD_384QAM: ModulationType = 13;
pub const BDA_MOD_448QAM: ModulationType = 14;
pub const BDA_MOD_512QAM: ModulationType = 15;
pub const BDA_MOD_640QAM: ModulationType = 16;
pub const BDA_MOD_64QAM: ModulationType = 3;
pub const BDA_MOD_768QAM: ModulationType = 17;
pub const BDA_MOD_80QAM: ModulationType = 4;
pub const BDA_MOD_896QAM: ModulationType = 18;
pub const BDA_MOD_8PSK: ModulationType = 27;
pub const BDA_MOD_8VSB: ModulationType = 23;
pub const BDA_MOD_96QAM: ModulationType = 5;
pub const BDA_MOD_ANALOG_AMPLITUDE: ModulationType = 25;
pub const BDA_MOD_ANALOG_FREQUENCY: ModulationType = 26;
pub const BDA_MOD_BPSK: ModulationType = 21;
pub const BDA_MOD_DIRECTV: ModulationType = 33;
pub const BDA_MOD_ISDB_S_TMCC: ModulationType = 35;
pub const BDA_MOD_ISDB_T_TMCC: ModulationType = 34;
pub const BDA_MOD_MAX: ModulationType = 36;
pub const BDA_MOD_NBC_8PSK: ModulationType = 32;
pub const BDA_MOD_NBC_QPSK: ModulationType = 31;
pub const BDA_MOD_NOT_DEFINED: ModulationType = 0;
pub const BDA_MOD_NOT_SET: ModulationType = -1;
pub const BDA_MOD_OQPSK: ModulationType = 22;
pub const BDA_MOD_QPSK: ModulationType = 20;
pub const BDA_MOD_RF: ModulationType = 28;
pub type BDA_MULTICAST_MODE = i32;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct BDA_MUX_PIDLISTITEM {
    pub usPIDNumber: u16,
    pub usProgramNumber: u16,
    pub ePIDType: MUX_PID_TYPE,
}
pub const BDA_NO_MULTICAST: BDA_MULTICAST_MODE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_PID_MAP {
    pub MediaSampleContent: MEDIA_SAMPLE_CONTENT,
    pub ulcPIDs: u32,
    pub aulPIDs: [u32; 1],
}
impl Default for BDA_PID_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_PID_UNMAP {
    pub ulcPIDs: u32,
    pub aulPIDs: [u32; 1],
}
impl Default for BDA_PID_UNMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_PILOT_MAX: Pilot = 3;
pub const BDA_PILOT_NOT_DEFINED: Pilot = 0;
pub const BDA_PILOT_NOT_SET: Pilot = -1;
pub const BDA_PILOT_OFF: Pilot = 1;
pub const BDA_PILOT_ON: Pilot = 2;
pub const BDA_PLP_ID_NOT_SET: i32 = -1;
pub const BDA_POLARISATION_CIRCULAR_L: Polarisation = 3;
pub const BDA_POLARISATION_CIRCULAR_R: Polarisation = 4;
pub const BDA_POLARISATION_LINEAR_H: Polarisation = 1;
pub const BDA_POLARISATION_LINEAR_V: Polarisation = 2;
pub const BDA_POLARISATION_MAX: Polarisation = 5;
pub const BDA_POLARISATION_NOT_DEFINED: Polarisation = 0;
pub const BDA_POLARISATION_NOT_SET: Polarisation = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_PROGRAM_PID_LIST {
    pub ulProgramNumber: u32,
    pub ulcPIDs: u32,
    pub ulPID: [u32; 1],
}
impl Default for BDA_PROGRAM_PID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_PROMISCUOUS_MULTICAST: BDA_MULTICAST_MODE = 0;
pub const BDA_RANGE_NOT_DEFINED: BDA_Range = 0;
pub const BDA_RANGE_NOT_SET: BDA_Range = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_RATING_PINRESET {
    pub bPinLength: u8,
    pub argbNewPin: [u8; 1],
}
impl Default for BDA_RATING_PINRESET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_ROLL_OFF_20: RollOff = 1;
pub const BDA_ROLL_OFF_25: RollOff = 2;
pub const BDA_ROLL_OFF_35: RollOff = 3;
pub const BDA_ROLL_OFF_MAX: RollOff = 4;
pub const BDA_ROLL_OFF_NOT_DEFINED: RollOff = 0;
pub const BDA_ROLL_OFF_NOT_SET: RollOff = -1;
pub type BDA_Range = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_SCAN_CAPABILTIES {
    pub lResult: PBDARESULT,
    pub ul64AnalogStandardsSupported: u64,
}
pub const BDA_SCAN_MOD_1024QAM: ScanModulationTypes = 262144;
pub const BDA_SCAN_MOD_112QAM: ScanModulationTypes = 32;
pub const BDA_SCAN_MOD_128QAM: ScanModulationTypes = 64;
pub const BDA_SCAN_MOD_160QAM: ScanModulationTypes = 128;
pub const BDA_SCAN_MOD_16APSK: ScanModulationTypes = 268435456;
pub const BDA_SCAN_MOD_16QAM: ScanModulationTypes = 1;
pub const BDA_SCAN_MOD_16VSB: ScanModulationTypes = 8388608;
pub const BDA_SCAN_MOD_192QAM: ScanModulationTypes = 256;
pub const BDA_SCAN_MOD_224QAM: ScanModulationTypes = 512;
pub const BDA_SCAN_MOD_256QAM: ScanModulationTypes = 1024;
pub const BDA_SCAN_MOD_320QAM: ScanModulationTypes = 2048;
pub const BDA_SCAN_MOD_32APSK: ScanModulationTypes = 536870912;
pub const BDA_SCAN_MOD_32QAM: ScanModulationTypes = 2;
pub const BDA_SCAN_MOD_384QAM: ScanModulationTypes = 4096;
pub const BDA_SCAN_MOD_448QAM: ScanModulationTypes = 8192;
pub const BDA_SCAN_MOD_512QAM: ScanModulationTypes = 16384;
pub const BDA_SCAN_MOD_640QAM: ScanModulationTypes = 32768;
pub const BDA_SCAN_MOD_64QAM: ScanModulationTypes = 4;
pub const BDA_SCAN_MOD_768QAM: ScanModulationTypes = 65536;
pub const BDA_SCAN_MOD_80QAM: ScanModulationTypes = 8;
pub const BDA_SCAN_MOD_896QAM: ScanModulationTypes = 131072;
pub const BDA_SCAN_MOD_8PSK: ScanModulationTypes = 67108864;
pub const BDA_SCAN_MOD_8VSB: ScanModulationTypes = 4194304;
pub const BDA_SCAN_MOD_96QAM: ScanModulationTypes = 16;
pub const BDA_SCAN_MOD_AM_RADIO: ScanModulationTypes = 16777216;
pub const BDA_SCAN_MOD_BPSK: ScanModulationTypes = 1048576;
pub const BDA_SCAN_MOD_FM_RADIO: ScanModulationTypes = 33554432;
pub const BDA_SCAN_MOD_OQPSK: ScanModulationTypes = 2097152;
pub const BDA_SCAN_MOD_QPSK: ScanModulationTypes = 524288;
pub const BDA_SCAN_MOD_RF: ScanModulationTypes = 134217728;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_SCAN_START {
    pub lResult: PBDARESULT,
    pub LowerFrequency: u32,
    pub HigerFrequency: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_SCAN_STATE {
    pub lResult: PBDARESULT,
    pub ulSignalLock: u32,
    pub ulSecondsLeft: u32,
    pub ulCurrentFrequency: u32,
}
pub const BDA_SIGNAL_ACTIVE: BDA_SIGNAL_STATE = 2;
pub const BDA_SIGNAL_INACTIVE: BDA_SIGNAL_STATE = 1;
pub type BDA_SIGNAL_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_SIGNAL_TIMEOUTS {
    pub ulCarrierTimeoutMs: u32,
    pub ulScanningTimeoutMs: u32,
    pub ulTuningTimeoutMs: u32,
}
pub const BDA_SIGNAL_UNAVAILABLE: BDA_SIGNAL_STATE = 0;
pub const BDA_SPECTRAL_INVERSION_AUTOMATIC: SpectralInversion = 1;
pub const BDA_SPECTRAL_INVERSION_INVERTED: SpectralInversion = 3;
pub const BDA_SPECTRAL_INVERSION_MAX: SpectralInversion = 4;
pub const BDA_SPECTRAL_INVERSION_NORMAL: SpectralInversion = 2;
pub const BDA_SPECTRAL_INVERSION_NOT_DEFINED: SpectralInversion = 0;
pub const BDA_SPECTRAL_INVERSION_NOT_SET: SpectralInversion = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_STRING {
    pub lResult: PBDARESULT,
    pub ulStringSize: u32,
    pub argbString: [u8; 1],
}
impl Default for BDA_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_TABLE_SECTION {
    pub ulPrimarySectionId: u32,
    pub ulSecondarySectionId: u32,
    pub ulcbSectionLength: u32,
    pub argbSectionData: [u32; 1],
}
impl Default for BDA_TABLE_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_TEMPLATE_CONNECTION {
    pub FromNodeType: u32,
    pub FromNodePinType: u32,
    pub ToNodeType: u32,
    pub ToNodePinType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_TEMPLATE_PIN_JOINT {
    pub uliTemplateConnection: u32,
    pub ulcInstancesMax: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct BDA_TS_SELECTORINFO {
    pub bTSInfolength: u8,
    pub bReserved: [u8; 2],
    pub guidNetworkType: windows_core::GUID,
    pub bTSIDCount: u8,
    pub usTSID: [u16; 1],
}
impl Default for BDA_TS_SELECTORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_TS_SELECTORINFO_ISDBS_EXT {
    pub bTMCC: [u8; 48],
}
impl Default for BDA_TS_SELECTORINFO_ISDBS_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_TUNER_DIAGNOSTICS {
    pub lResult: PBDARESULT,
    pub ulSignalLevel: u32,
    pub ulSignalLevelQuality: u32,
    pub ulSignalNoiseRatio: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_TUNER_TUNERSTATE {
    pub lResult: PBDARESULT,
    pub ulTuneLength: u32,
    pub argbTuneData: [u8; 1],
}
impl Default for BDA_TUNER_TUNERSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BDA_UNDEFINED_CHANNEL: BDA_Channel = -1;
pub const BDA_UNITIALIZED_MPEG2STREAMTYPE: MPEG2StreamType = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_USERACTIVITY_INTERVAL {
    pub lResult: PBDARESULT,
    pub ulActivityInterval: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_WMDRMTUNER_PIDPROTECTION {
    pub lResult: PBDARESULT,
    pub uuidKeyID: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_WMDRMTUNER_PURCHASEENTITLEMENT {
    pub lResult: PBDARESULT,
    pub ulDescrambleStatus: u32,
    pub ulCaptureTokenLength: u32,
    pub argbCaptureTokenBuffer: [u8; 1],
}
impl Default for BDA_WMDRMTUNER_PURCHASEENTITLEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_WMDRM_KEYINFOLIST {
    pub lResult: PBDARESULT,
    pub ulKeyuuidBufferLen: u32,
    pub argKeyuuidBuffer: [windows_core::GUID; 1],
}
impl Default for BDA_WMDRM_KEYINFOLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BDA_WMDRM_RENEWLICENSE {
    pub lResult: PBDARESULT,
    pub ulDescrambleStatus: u32,
    pub ulXmrLicenseOutputLength: u32,
    pub argbXmrLicenceOutputBuffer: [u8; 1],
}
impl Default for BDA_WMDRM_RENEWLICENSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BDA_WMDRM_STATUS {
    pub lResult: PBDARESULT,
    pub ulMaxCaptureTokenSize: u32,
    pub uMaxStreamingPid: u32,
    pub ulMaxLicense: u32,
    pub ulMinSecurityLevel: u32,
    pub ulRevInfoSequenceNumber: u32,
    pub ulRevInfoIssuedTime: u64,
    pub ulRevListVersion: u32,
    pub ulRevInfoTTL: u32,
    pub ulState: u32,
}
pub const BDA_XMIT_MODE_16K: TransmissionMode = 7;
pub const BDA_XMIT_MODE_1K: TransmissionMode = 6;
pub const BDA_XMIT_MODE_2K: TransmissionMode = 1;
pub const BDA_XMIT_MODE_2K_INTERLEAVED: TransmissionMode = 4;
pub const BDA_XMIT_MODE_32K: TransmissionMode = 8;
pub const BDA_XMIT_MODE_4K: TransmissionMode = 3;
pub const BDA_XMIT_MODE_4K_INTERLEAVED: TransmissionMode = 5;
pub const BDA_XMIT_MODE_8K: TransmissionMode = 2;
pub const BDA_XMIT_MODE_MAX: TransmissionMode = 9;
pub const BDA_XMIT_MODE_NOT_DEFINED: TransmissionMode = 0;
pub const BDA_XMIT_MODE_NOT_SET: TransmissionMode = -1;
pub type BinaryConvolutionCodeRate = i32;
pub const CATEGORY_COUNT: ComponentCategory = 8;
pub const CONDITIONALACCESS_ABORTED: BDA_CONDITIONALACCESS_SESSION_RESULT = 2;
pub const CONDITIONALACCESS_ACCESS_NOT_POSSIBLE: BDA_CONDITIONALACCESS_REQUESTTYPE = 1;
pub const CONDITIONALACCESS_ACCESS_POSSIBLE: BDA_CONDITIONALACCESS_REQUESTTYPE = 2;
pub const CONDITIONALACCESS_ACCESS_POSSIBLE_NO_STREAMING_DISRUPTION: BDA_CONDITIONALACCESS_REQUESTTYPE = 3;
pub const CONDITIONALACCESS_ACCESS_UNSPECIFIED: BDA_CONDITIONALACCESS_REQUESTTYPE = 0;
pub const CONDITIONALACCESS_CLOSED_ITSELF: BDA_CONDITIONALACCESS_MMICLOSEREASON = 1;
pub const CONDITIONALACCESS_DIALOG_FOCUS_CHANGE: BDA_CONDITIONALACCESS_MMICLOSEREASON = 4;
pub const CONDITIONALACCESS_DIALOG_TIMEOUT: BDA_CONDITIONALACCESS_MMICLOSEREASON = 3;
pub const CONDITIONALACCESS_DIALOG_USER_DISMISSED: BDA_CONDITIONALACCESS_MMICLOSEREASON = 5;
pub const CONDITIONALACCESS_DIALOG_USER_NOT_AVAILABLE: BDA_CONDITIONALACCESS_MMICLOSEREASON = 6;
pub const CONDITIONALACCESS_ENDED_NOCHANGE: BDA_CONDITIONALACCESS_SESSION_RESULT = 1;
pub const CONDITIONALACCESS_SUCCESSFULL: BDA_CONDITIONALACCESS_SESSION_RESULT = 0;
pub const CONDITIONALACCESS_TUNER_REQUESTED_CLOSE: BDA_CONDITIONALACCESS_MMICLOSEREASON = 2;
pub const CONDITIONALACCESS_UNSPECIFIED: BDA_CONDITIONALACCESS_MMICLOSEREASON = 0;
pub const CategoryAudio: ComponentCategory = 2;
pub const CategoryCaptions: ComponentCategory = 5;
pub const CategoryData: ComponentCategory = 7;
pub const CategoryNotSet: ComponentCategory = -1;
pub const CategoryOther: ComponentCategory = 0;
pub const CategorySubtitles: ComponentCategory = 4;
pub const CategorySuperimpose: ComponentCategory = 6;
pub const CategoryText: ComponentCategory = 3;
pub const CategoryVideo: ComponentCategory = 1;
pub type ComponentCategory = i32;
pub type ComponentStatus = i32;
pub const DOLBY_AC3_AUDIO: MPEG2StreamType = 129;
pub const DOLBY_DIGITAL_PLUS_AUDIO_ATSC: MPEG2StreamType = 135;
pub type DVBSystemType = i32;
pub const DVB_Cable: DVBSystemType = 0;
pub const DVB_Satellite: DVBSystemType = 2;
pub const DVB_Terrestrial: DVBSystemType = 1;
pub type FECMethod = i32;
pub type GuardInterval = i32;
pub const HEVC_TEMPORAL_VIDEO_SUBSET: MPEG2StreamType = 37;
pub const HEVC_VIDEO_OR_TEMPORAL_VIDEO: MPEG2StreamType = 36;
pub type HierarchyAlpha = i32;
pub const IRPM_STREAMM: MPEG2StreamType = 26;
pub type ISDBCAS_REQUEST_ID = i32;
pub const ISDBCAS_REQUEST_ID_EMD: ISDBCAS_REQUEST_ID = 58;
pub const ISDBCAS_REQUEST_ID_EMG: ISDBCAS_REQUEST_ID = 56;
pub const ISDB_Satellite: DVBSystemType = 4;
pub const ISDB_Terrestrial: DVBSystemType = 3;
pub const ISO_IEC_11172_2_VIDEO: MPEG2StreamType = 1;
pub const ISO_IEC_11172_3_AUDIO: MPEG2StreamType = 3;
pub const ISO_IEC_13522_MHEG: MPEG2StreamType = 7;
pub const ISO_IEC_13818_1_AUXILIARY: MPEG2StreamType = 14;
pub const ISO_IEC_13818_1_PES: MPEG2StreamType = 6;
pub const ISO_IEC_13818_1_PRIVATE_SECTION: MPEG2StreamType = 5;
pub const ISO_IEC_13818_1_RESERVED: MPEG2StreamType = 28;
pub const ISO_IEC_13818_2_VIDEO: MPEG2StreamType = 2;
pub const ISO_IEC_13818_3_AUDIO: MPEG2StreamType = 4;
pub const ISO_IEC_13818_6_DOWNLOAD: MPEG2StreamType = 20;
pub const ISO_IEC_13818_6_TYPE_A: MPEG2StreamType = 10;
pub const ISO_IEC_13818_6_TYPE_B: MPEG2StreamType = 11;
pub const ISO_IEC_13818_6_TYPE_C: MPEG2StreamType = 12;
pub const ISO_IEC_13818_6_TYPE_D: MPEG2StreamType = 13;
pub const ISO_IEC_13818_7_AUDIO: MPEG2StreamType = 15;
pub const ISO_IEC_14496_1_IN_PES: MPEG2StreamType = 18;
pub const ISO_IEC_14496_1_IN_SECTION: MPEG2StreamType = 19;
pub const ISO_IEC_14496_2_VISUAL: MPEG2StreamType = 16;
pub const ISO_IEC_14496_3_AUDIO: MPEG2StreamType = 17;
pub const ISO_IEC_USER_PRIVATE: MPEG2StreamType = 128;
pub const ITU_T_H264: MPEG2StreamType = 27;
pub const ITU_T_REC_H_222_1: MPEG2StreamType = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KS_BDA_FRAME_INFO {
    pub ExtendedHeaderSize: u32,
    pub dwFrameFlags: u32,
    pub ulEvent: u32,
    pub ulChannelNumber: u32,
    pub ulSubchannelNumber: u32,
    pub ulReason: u32,
}
pub type LNB_Source = i32;
pub const MEDIA_ELEMENTARY_STREAM: MEDIA_SAMPLE_CONTENT = 1;
pub const MEDIA_MPEG2_PSI: MEDIA_SAMPLE_CONTENT = 2;
pub type MEDIA_SAMPLE_CONTENT = i32;
pub const MEDIA_TRANSPORT_PACKET: MEDIA_SAMPLE_CONTENT = 0;
pub const MEDIA_TRANSPORT_PAYLOAD: MEDIA_SAMPLE_CONTENT = 3;
pub const METADATA_IN_DATA_CAROUSEL: MPEG2StreamType = 23;
pub const METADATA_IN_DOWNLOAD_PROTOCOL: MPEG2StreamType = 25;
pub const METADATA_IN_OBJECT_CAROUSEL: MPEG2StreamType = 24;
pub const METADATA_IN_PES: MPEG2StreamType = 21;
pub const METADATA_IN_SECTION: MPEG2StreamType = 22;
pub const MIN_DIMENSION: u32 = 1;
pub type MPEG2StreamType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MPEG2_TRANSPORT_STRIDE {
    pub dwOffset: u32,
    pub dwPacketLength: u32,
    pub dwStride: u32,
}
pub const MPEG_H_AUDIO: MPEG2StreamType = 45;
pub const MPEG_H_AUDIO_MS: MPEG2StreamType = 46;
pub type MUX_PID_TYPE = i32;
pub type ModulationType = i32;
pub type PBDANODE_DESCRIPTOR = *mut BDANODE_DESCRIPTOR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PBDARESULT(pub i32);
pub type PBDA_BUFFER = *mut BDA_BUFFER;
pub type PBDA_CAS_CHECK_ENTITLEMENTTOKEN = *mut BDA_CAS_CHECK_ENTITLEMENTTOKEN;
pub type PBDA_CAS_CLOSEMMIDATA = *mut BDA_CAS_CLOSEMMIDATA;
pub type PBDA_CAS_CLOSE_MMIDIALOG = *mut BDA_CAS_CLOSE_MMIDIALOG;
pub type PBDA_CAS_OPENMMIDATA = *mut BDA_CAS_OPENMMIDATA;
pub type PBDA_CAS_REQUESTTUNERDATA = *mut BDA_CAS_REQUESTTUNERDATA;
pub type PBDA_CA_MODULE_UI = *mut BDA_CA_MODULE_UI;
pub type PBDA_CHANGE_STATE = *mut BDA_CHANGE_STATE;
pub type PBDA_DISEQC_RESPONSE = *mut BDA_DISEQC_RESPONSE;
pub type PBDA_DISEQC_SEND = *mut BDA_DISEQC_SEND;
pub type PBDA_DRM_DRMSTATUS = *mut BDA_DRM_DRMSTATUS;
pub type PBDA_DVBT2_L1_SIGNALLING_DATA = *mut BDA_DVBT2_L1_SIGNALLING_DATA;
pub type PBDA_ETHERNET_ADDRESS = *mut BDA_ETHERNET_ADDRESS;
pub type PBDA_ETHERNET_ADDRESS_LIST = *mut BDA_ETHERNET_ADDRESS_LIST;
pub type PBDA_EVENT_ID = *mut BDA_EVENT_ID;
pub type PBDA_IPv4_ADDRESS = *mut BDA_IPv4_ADDRESS;
pub type PBDA_IPv4_ADDRESS_LIST = *mut BDA_IPv4_ADDRESS_LIST;
pub type PBDA_IPv6_ADDRESS = *mut BDA_IPv6_ADDRESS;
pub type PBDA_IPv6_ADDRESS_LIST = *mut BDA_IPv6_ADDRESS_LIST;
pub type PBDA_ISDBCAS_EMG_REQ = *mut BDA_ISDBCAS_EMG_REQ;
pub type PBDA_ISDBCAS_REQUESTHEADER = *mut BDA_ISDBCAS_REQUESTHEADER;
pub type PBDA_ISDBCAS_RESPONSEDATA = *mut BDA_ISDBCAS_RESPONSEDATA;
pub type PBDA_MULTICAST_MODE = *mut BDA_MULTICAST_MODE;
pub type PBDA_MUX_PIDLISTITEM = *mut BDA_MUX_PIDLISTITEM;
pub type PBDA_PID_MAP = *mut BDA_PID_MAP;
pub type PBDA_PID_UNMAP = *mut BDA_PID_UNMAP;
pub type PBDA_PROGRAM_PID_LIST = *mut BDA_PROGRAM_PID_LIST;
pub type PBDA_RATING_PINRESET = *mut BDA_RATING_PINRESET;
pub type PBDA_SCAN_CAPABILTIES = *mut BDA_SCAN_CAPABILTIES;
pub type PBDA_SCAN_START = *mut BDA_SCAN_START;
pub type PBDA_SCAN_STATE = *mut BDA_SCAN_STATE;
pub type PBDA_SIGNAL_STATE = *mut BDA_SIGNAL_STATE;
pub type PBDA_SIGNAL_TIMEOUTS = *mut BDA_SIGNAL_TIMEOUTS;
pub type PBDA_STRING = *mut BDA_STRING;
pub type PBDA_TABLE_SECTION = *mut BDA_TABLE_SECTION;
pub type PBDA_TEMPLATE_CONNECTION = *mut BDA_TEMPLATE_CONNECTION;
pub type PBDA_TEMPLATE_PIN_JOINT = *mut BDA_TEMPLATE_PIN_JOINT;
pub type PBDA_TS_SELECTORINFO = *mut BDA_TS_SELECTORINFO;
pub type PBDA_TS_SELECTORINFO_ISDBS_EXT = *mut BDA_TS_SELECTORINFO_ISDBS_EXT;
pub type PBDA_TUNER_DIAGNOSTICS = *mut BDA_TUNER_DIAGNOSTICS;
pub type PBDA_TUNER_TUNERSTATE = *mut BDA_TUNER_TUNERSTATE;
pub type PBDA_WMDRMTUNER_PIDPROTECTION = *mut BDA_WMDRMTUNER_PIDPROTECTION;
pub type PBDA_WMDRMTUNER_PURCHASEENTITLEMENT = *mut BDA_WMDRMTUNER_PURCHASEENTITLEMENT;
pub type PBDA_WMDRM_KEYINFOLIST = *mut BDA_WMDRM_KEYINFOLIST;
pub type PBDA_WMDRM_RENEWLICENSE = *mut BDA_WMDRM_RENEWLICENSE;
pub type PBDA_WMDRM_STATUS = *mut BDA_WMDRM_STATUS;
pub const PID_ELEMENTARY_STREAM: MUX_PID_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PID_MAP {
    pub ulPID: u32,
    pub MediaSampleContent: MEDIA_SAMPLE_CONTENT,
}
pub const PID_MPEG2_SECTION_PSI_SI: MUX_PID_TYPE = 1;
pub const PID_OTHER: MUX_PID_TYPE = -1;
pub type PKS_BDA_FRAME_INFO = *mut KS_BDA_FRAME_INFO;
pub type PMPEG2_TRANSPORT_STRIDE = *mut MPEG2_TRANSPORT_STRIDE;
pub type P_BDA_GDDS_DATA = *mut BDA_GDDS_DATA;
pub type P_BDA_GDDS_DATATYPE = *mut BDA_GDDS_DATATYPE;
pub type P_BDA_USERACTIVITY_INTERVAL = *mut BDA_USERACTIVITY_INTERVAL;
pub type Pilot = i32;
pub type Polarisation = i32;
pub const Reserved1: MPEG2StreamType = 0;
pub type RollOff = i32;
pub const SCTE28_ConditionalAccess: ApplicationTypeType = 0;
pub const SCTE28_CopyProtection: ApplicationTypeType = 5;
pub const SCTE28_Diagnostic: ApplicationTypeType = 6;
pub const SCTE28_IPService: ApplicationTypeType = 2;
pub const SCTE28_NetworkInterface_SCTE55_1: ApplicationTypeType = 4;
pub const SCTE28_NetworkInterface_SCTE55_2: ApplicationTypeType = 3;
pub const SCTE28_POD_Host_Binding_Information: ApplicationTypeType = 1;
pub const SCTE28_Reserved: ApplicationTypeType = 8;
pub const SCTE28_Undesignated: ApplicationTypeType = 7;
pub type ScanModulationTypes = i32;
pub const ScanModulationTypesMask_DVBC: ScanModulationTypes = 75;
pub const ScanModulationTypesMask_MCE_All_TV: ScanModulationTypes = -1;
pub const ScanModulationTypesMask_MCE_AnalogTv: ScanModulationTypes = 28;
pub const ScanModulationTypesMask_MCE_DigitalCable: ScanModulationTypes = 11;
pub const ScanModulationTypesMask_MCE_TerrestrialATSC: ScanModulationTypes = 23;
pub type SpectralInversion = i32;
pub const StatusActive: ComponentStatus = 0;
pub const StatusInactive: ComponentStatus = 1;
pub const StatusUnavailable: ComponentStatus = 2;
pub type TransmissionMode = i32;
pub const USER_PRIVATE: MPEG2StreamType = 16;
