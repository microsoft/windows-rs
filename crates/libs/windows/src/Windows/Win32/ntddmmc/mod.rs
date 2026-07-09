#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BD_CLASS_SUPPORT_BITMAP {
    pub _bitfield1: u8,
    pub _bitfield2: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_BD_READ {
    pub Header: FEATURE_HEADER,
    pub Reserved: [u8; 4],
    pub Class0BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDREReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class0BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDRReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class0BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDROMReadSupport: BD_CLASS_SUPPORT_BITMAP,
}
impl Default for FEATURE_BD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_BD_R_PSEUDO_OVERWRITE {
    pub Header: FEATURE_HEADER,
    pub Reserved: [u8; 4],
}
impl Default for FEATURE_BD_R_PSEUDO_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_BD_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
    pub Class0BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDREWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class0BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class1BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class2BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
    pub Class3BitmapBDRWriteSupport: BD_CLASS_SUPPORT_BITMAP,
}
impl Default for FEATURE_BD_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    pub Header: FEATURE_HEADER,
    pub Reserved1: u8,
    pub CDRWMediaSubtypeSupport: FEATURE_CD_RW_MEDIA_WRITE_SUPPORT_0,
    pub Reserved2: [u8; 2],
}
impl Default for FEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_CD_RW_MEDIA_WRITE_SUPPORT_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_AACS {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub BindingNonceBlockCount: u8,
    pub _bitfield2: u8,
    pub AACSVersion: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_CDRW_CAV_WRITE {
    pub Header: FEATURE_HEADER,
    pub Reserved1: [u8; 4],
}
impl Default for FEATURE_DATA_CDRW_CAV_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: u8,
    pub NumerOfVolumeLevels: [u8; 2],
}
impl Default for FEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_CD_MASTERING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub MaximumCueSheetLength: [u8; 3],
}
impl Default for FEATURE_DATA_CD_MASTERING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_CD_READ {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_CD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_CD_TRACK_AT_ONCE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: u8,
    pub DataTypeSupported: [u8; 2],
}
impl Default for FEATURE_DATA_CD_TRACK_AT_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_CORE {
    pub Header: FEATURE_HEADER,
    pub PhysicalInterface: [u8; 4],
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_CORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_DDCD_READ {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DDCD_RW_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_DDCD_RW_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DDCD_R_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved3: [u8; 3],
}
impl Default for FEATURE_DATA_DDCD_R_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DEFECT_MANAGEMENT {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_DEFECT_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DISC_CONTROL_BLOCKS {
    pub Header: FEATURE_HEADER,
    pub Data: [FEATURE_DATA_DISC_CONTROL_BLOCKS_EX; 0],
}
impl Default for FEATURE_DATA_DISC_CONTROL_BLOCKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    pub ContentDescriptor: [u8; 4],
}
impl Default for FEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_CPRM {
    pub Header: FEATURE_HEADER,
    pub Reserved0: [u8; 3],
    pub CPRMVersion: u8,
}
impl Default for FEATURE_DATA_DVD_CPRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_CSS {
    pub Header: FEATURE_HEADER,
    pub Reserved1: [u8; 3],
    pub CssVersion: u8,
}
impl Default for FEATURE_DATA_DVD_CSS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_R {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_DVD_PLUS_R {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_RW {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub Reserved03: [u8; 2],
}
impl Default for FEATURE_DATA_DVD_PLUS_RW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub Reserved3: [u8; 2],
}
impl Default for FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_DVD_READ {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved2: u8,
    pub _bitfield2: u8,
    pub Reserved4: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_RECORDABLE_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved4: [u8; 3],
}
impl Default for FEATURE_DATA_DVD_RECORDABLE_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved1: [u8; 3],
}
impl Default for FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_EMBEDDED_CHANGER {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved4: [u8; 2],
    pub _bitfield2: u8,
}
impl Default for FEATURE_DATA_EMBEDDED_CHANGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_FIRMWARE_DATE {
    pub Header: FEATURE_HEADER,
    pub Year: [u8; 4],
    pub Month: [u8; 2],
    pub Day: [u8; 2],
    pub Hour: [u8; 2],
    pub Minute: [u8; 2],
    pub Seconds: [u8; 2],
    pub Reserved: [u8; 2],
}
impl Default for FEATURE_DATA_FIRMWARE_DATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_FORMATTABLE {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved2: [u8; 3],
    pub _bitfield2: u8,
    pub Reserved4: [u8; 3],
}
impl Default for FEATURE_DATA_FORMATTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_HDDVD_READ {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved1: u8,
    pub _bitfield2: u8,
    pub Reserved3: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_HDDVD_WRITE {
    pub Header: FEATURE_HEADER,
    pub _bitfield1: u8,
    pub Reserved1: u8,
    pub _bitfield2: u8,
    pub Reserved3: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    pub Header: FEATURE_HEADER,
    pub DataTypeSupported: [u8; 2],
    pub _bitfield: u8,
    pub NumberOfLinkSizes: u8,
    pub LinkSize: [u8; 0],
}
impl Default for FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_LAYER_JUMP_RECORDING {
    pub Header: FEATURE_HEADER,
    pub Reserved0: [u8; 3],
    pub NumberOfLinkSizes: u8,
    pub LinkSizes: [u8; 0],
}
impl Default for FEATURE_DATA_LAYER_JUMP_RECORDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    pub Header: FEATURE_HEADER,
    pub SerialNumber: [u8; 0],
}
impl Default for FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_MICROCODE_UPDATE {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_MICROCODE_UPDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_MORPHING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_MORPHING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_MRW {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_MRW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_MULTI_READ {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_POWER_MANAGEMENT {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_PROFILE_LIST {
    pub Header: FEATURE_HEADER,
    pub Profiles: [FEATURE_DATA_PROFILE_LIST_EX; 0],
}
impl Default for FEATURE_DATA_PROFILE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_PROFILE_LIST_EX {
    pub ProfileNumber: [u8; 2],
    pub _bitfield: u8,
    pub Reserved2: u8,
}
impl Default for FEATURE_DATA_PROFILE_LIST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_RANDOM_READABLE {
    pub Header: FEATURE_HEADER,
    pub LogicalBlockSize: [u8; 4],
    pub Blocking: [u8; 2],
    pub _bitfield: u8,
    pub Reserved2: u8,
}
impl Default for FEATURE_DATA_RANDOM_READABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_RANDOM_WRITABLE {
    pub Header: FEATURE_HEADER,
    pub LastLBA: [u8; 4],
    pub LogicalBlockSize: [u8; 4],
    pub Blocking: [u8; 2],
    pub _bitfield: u8,
    pub Reserved2: u8,
}
impl Default for FEATURE_DATA_RANDOM_WRITABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_REAL_TIME_STREAMING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_REAL_TIME_STREAMING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_REMOVABLE_MEDIUM {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved3: [u8; 3],
}
impl Default for FEATURE_DATA_REMOVABLE_MEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_RESERVED {
    pub Header: FEATURE_HEADER,
    pub Data: [u8; 0],
}
impl Default for FEATURE_DATA_RESERVED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_RESTRICTED_OVERWRITE {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_DATA_SECTOR_ERASABLE {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_SMART {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved02: [u8; 3],
}
impl Default for FEATURE_DATA_SMART {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_TIMEOUT {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: u8,
    pub UnitLength: [u8; 2],
}
impl Default for FEATURE_DATA_TIMEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_VENDOR_SPECIFIC {
    pub Header: FEATURE_HEADER,
    pub VendorSpecificData: [u8; 0],
}
impl Default for FEATURE_DATA_VENDOR_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_WRITE_ONCE {
    pub Header: FEATURE_HEADER,
    pub LogicalBlockSize: [u8; 4],
    pub Blocking: [u8; 2],
    pub _bitfield: u8,
    pub Reserved2: u8,
}
impl Default for FEATURE_DATA_WRITE_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_DATA_WRITE_PROTECT {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_DATA_WRITE_PROTECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_ENHANCED_DEFECT_REPORTING {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub NumberOfDBICacheZones: u8,
    pub NumberOfEntries: [u8; 2],
}
impl Default for FEATURE_ENHANCED_DEFECT_REPORTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_HEADER {
    pub FeatureCode: [u8; 2],
    pub _bitfield: u8,
    pub AdditionalLength: u8,
}
impl Default for FEATURE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_HYBRID_DISC {
    pub Header: FEATURE_HEADER,
    pub _bitfield: u8,
    pub Reserved2: [u8; 3],
}
impl Default for FEATURE_HYBRID_DISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_MEDIA_SERIAL_NUMBER {
    pub Header: FEATURE_HEADER,
}
pub type FEATURE_NUMBER = i32;
pub type FEATURE_PROFILE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FEATURE_TSR {
    pub Header: FEATURE_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FEATURE_VCPS {
    pub Header: FEATURE_HEADER,
    pub Reserved: [u8; 4],
}
impl Default for FEATURE_VCPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FeatureAACS: FEATURE_NUMBER = 269;
pub const FeatureBDRPseudoOverwrite: FEATURE_NUMBER = 56;
pub const FeatureBDRead: FEATURE_NUMBER = 64;
pub const FeatureBDWrite: FEATURE_NUMBER = 65;
pub const FeatureCDAudioAnalogPlay: FEATURE_NUMBER = 259;
pub const FeatureCDRWMediaWriteSupport: FEATURE_NUMBER = 55;
pub const FeatureCdMastering: FEATURE_NUMBER = 46;
pub const FeatureCdRead: FEATURE_NUMBER = 30;
pub const FeatureCdTrackAtOnce: FEATURE_NUMBER = 45;
pub const FeatureCdrwCAVWrite: FEATURE_NUMBER = 39;
pub const FeatureCore: FEATURE_NUMBER = 1;
pub const FeatureDDCDRWWrite: FEATURE_NUMBER = 50;
pub const FeatureDDCDRWrite: FEATURE_NUMBER = 49;
pub const FeatureDDCDRead: FEATURE_NUMBER = 48;
pub const FeatureDefectManagement: FEATURE_NUMBER = 36;
pub const FeatureDiscControlBlocks: FEATURE_NUMBER = 266;
pub const FeatureDvdCPRM: FEATURE_NUMBER = 267;
pub const FeatureDvdCSS: FEATURE_NUMBER = 262;
pub const FeatureDvdPlusR: FEATURE_NUMBER = 43;
pub const FeatureDvdPlusRDualLayer: FEATURE_NUMBER = 59;
pub const FeatureDvdPlusRW: FEATURE_NUMBER = 42;
pub const FeatureDvdPlusRWDualLayer: FEATURE_NUMBER = 58;
pub const FeatureDvdRead: FEATURE_NUMBER = 31;
pub const FeatureDvdRecordableWrite: FEATURE_NUMBER = 47;
pub const FeatureEmbeddedChanger: FEATURE_NUMBER = 258;
pub const FeatureEnhancedDefectReporting: FEATURE_NUMBER = 41;
pub const FeatureFirmwareDate: FEATURE_NUMBER = 268;
pub const FeatureFormattable: FEATURE_NUMBER = 35;
pub const FeatureHDDVDRead: FEATURE_NUMBER = 80;
pub const FeatureHDDVDWrite: FEATURE_NUMBER = 81;
pub const FeatureHybridDisc: FEATURE_NUMBER = 128;
pub const FeatureIncrementalStreamingWritable: FEATURE_NUMBER = 33;
pub const FeatureLayerJumpRecording: FEATURE_NUMBER = 51;
pub const FeatureLogicalUnitSerialNumber: FEATURE_NUMBER = 264;
pub const FeatureMediaSerialNumber: FEATURE_NUMBER = 265;
pub const FeatureMicrocodeUpgrade: FEATURE_NUMBER = 260;
pub const FeatureMorphing: FEATURE_NUMBER = 2;
pub const FeatureMrw: FEATURE_NUMBER = 40;
pub const FeatureMultiRead: FEATURE_NUMBER = 29;
pub const FeaturePowerManagement: FEATURE_NUMBER = 256;
pub const FeatureProfileList: FEATURE_NUMBER = 0;
pub const FeatureRandomReadable: FEATURE_NUMBER = 16;
pub const FeatureRandomWritable: FEATURE_NUMBER = 32;
pub const FeatureRealTimeStreaming: FEATURE_NUMBER = 263;
pub const FeatureRemovableMedium: FEATURE_NUMBER = 3;
pub const FeatureRestrictedOverwrite: FEATURE_NUMBER = 38;
pub const FeatureRigidRestrictedOverwrite: FEATURE_NUMBER = 44;
pub const FeatureSMART: FEATURE_NUMBER = 257;
pub const FeatureSectorErasable: FEATURE_NUMBER = 34;
pub const FeatureTSR: FEATURE_NUMBER = 66;
pub const FeatureTimeout: FEATURE_NUMBER = 261;
pub const FeatureVCPS: FEATURE_NUMBER = 272;
pub const FeatureWriteOnce: FEATURE_NUMBER = 37;
pub const FeatureWriteProtect: FEATURE_NUMBER = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GET_CONFIGURATION_HEADER {
    pub DataLength: [u8; 4],
    pub Reserved: [u8; 2],
    pub CurrentProfile: [u8; 2],
    pub Data: [u8; 0],
}
impl Default for GET_CONFIGURATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GET_CONFIGURATION_IOCTL_INPUT {
    pub Feature: FEATURE_NUMBER,
    pub RequestType: u32,
    pub Reserved: [*mut core::ffi::c_void; 2],
}
impl Default for GET_CONFIGURATION_IOCTL_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GET_CONFIGURATION_IOCTL_INPUT32 {
    pub Feature: FEATURE_NUMBER,
    pub RequestType: u32,
    pub Reserved: [*mut core::ffi::c_void; 2],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for GET_CONFIGURATION_IOCTL_INPUT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBD_CLASS_SUPPORT_BITMAP(pub *mut BD_CLASS_SUPPORT_BITMAP);
impl PBD_CLASS_SUPPORT_BITMAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBD_CLASS_SUPPORT_BITMAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_BD_READ(pub *mut FEATURE_BD_READ);
impl PFEATURE_BD_READ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_BD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_BD_R_PSEUDO_OVERWRITE(pub *mut FEATURE_BD_R_PSEUDO_OVERWRITE);
impl PFEATURE_BD_R_PSEUDO_OVERWRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_BD_R_PSEUDO_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_BD_WRITE(pub *mut FEATURE_BD_WRITE);
impl PFEATURE_BD_WRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_BD_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_CD_RW_MEDIA_WRITE_SUPPORT(pub *mut FEATURE_CD_RW_MEDIA_WRITE_SUPPORT);
impl PFEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_CD_RW_MEDIA_WRITE_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_AACS(pub *mut FEATURE_DATA_AACS);
impl PFEATURE_DATA_AACS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_AACS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_CDRW_CAV_WRITE(pub *mut FEATURE_DATA_CDRW_CAV_WRITE);
impl PFEATURE_DATA_CDRW_CAV_WRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_CDRW_CAV_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_CD_AUDIO_ANALOG_PLAY(pub *mut FEATURE_DATA_CD_AUDIO_ANALOG_PLAY);
impl PFEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_CD_AUDIO_ANALOG_PLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_CD_MASTERING(pub *mut FEATURE_DATA_CD_MASTERING);
impl PFEATURE_DATA_CD_MASTERING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_CD_MASTERING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_CD_READ(pub *mut FEATURE_DATA_CD_READ);
impl PFEATURE_DATA_CD_READ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_CD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_CD_TRACK_AT_ONCE(pub *mut FEATURE_DATA_CD_TRACK_AT_ONCE);
impl PFEATURE_DATA_CD_TRACK_AT_ONCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_CD_TRACK_AT_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_CORE(pub *mut FEATURE_DATA_CORE);
impl PFEATURE_DATA_CORE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_CORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DDCD_READ(pub *mut FEATURE_DATA_DDCD_READ);
impl PFEATURE_DATA_DDCD_READ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DDCD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DDCD_RW_WRITE(pub *mut FEATURE_DATA_DDCD_RW_WRITE);
impl PFEATURE_DATA_DDCD_RW_WRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DDCD_RW_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DDCD_R_WRITE(pub *mut FEATURE_DATA_DDCD_R_WRITE);
impl PFEATURE_DATA_DDCD_R_WRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DDCD_R_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DEFECT_MANAGEMENT(pub *mut FEATURE_DATA_DEFECT_MANAGEMENT);
impl PFEATURE_DATA_DEFECT_MANAGEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DEFECT_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DISC_CONTROL_BLOCKS(pub *mut FEATURE_DATA_DISC_CONTROL_BLOCKS);
impl PFEATURE_DATA_DISC_CONTROL_BLOCKS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DISC_CONTROL_BLOCKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DISC_CONTROL_BLOCKS_EX(pub *mut FEATURE_DATA_DISC_CONTROL_BLOCKS_EX);
impl PFEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DISC_CONTROL_BLOCKS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_CPRM(pub *mut FEATURE_DATA_DVD_CPRM);
impl PFEATURE_DATA_DVD_CPRM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_CPRM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_CSS(pub *mut FEATURE_DATA_DVD_CSS);
impl PFEATURE_DATA_DVD_CSS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_CSS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_PLUS_R(pub *mut FEATURE_DATA_DVD_PLUS_R);
impl PFEATURE_DATA_DVD_PLUS_R {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_PLUS_R {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_PLUS_RW(pub *mut FEATURE_DATA_DVD_PLUS_RW);
impl PFEATURE_DATA_DVD_PLUS_RW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_PLUS_RW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER(pub *mut FEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER);
impl PFEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_PLUS_RW_DUAL_LAYER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_PLUS_R_DUAL_LAYER(pub *mut FEATURE_DATA_DVD_PLUS_R_DUAL_LAYER);
impl PFEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_PLUS_R_DUAL_LAYER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_READ(pub *mut FEATURE_DATA_DVD_READ);
impl PFEATURE_DATA_DVD_READ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_RECORDABLE_WRITE(pub *mut FEATURE_DATA_DVD_RECORDABLE_WRITE);
impl PFEATURE_DATA_DVD_RECORDABLE_WRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_RECORDABLE_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE(pub *mut FEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE);
impl PFEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_DVD_RW_RESTRICTED_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_EMBEDDED_CHANGER(pub *mut FEATURE_DATA_EMBEDDED_CHANGER);
impl PFEATURE_DATA_EMBEDDED_CHANGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_EMBEDDED_CHANGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_FIRMWARE_DATE(pub *mut FEATURE_DATA_FIRMWARE_DATE);
impl PFEATURE_DATA_FIRMWARE_DATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_FIRMWARE_DATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_FORMATTABLE(pub *mut FEATURE_DATA_FORMATTABLE);
impl PFEATURE_DATA_FORMATTABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_FORMATTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_HDDVD_READ(pub *mut FEATURE_DATA_HDDVD_READ);
impl PFEATURE_DATA_HDDVD_READ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_HDDVD_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_HDDVD_WRITE(pub *mut FEATURE_DATA_HDDVD_WRITE);
impl PFEATURE_DATA_HDDVD_WRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_HDDVD_WRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE(pub *mut FEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE);
impl PFEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_INCREMENTAL_STREAMING_WRITABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_LAYER_JUMP_RECORDING(pub *mut FEATURE_DATA_LAYER_JUMP_RECORDING);
impl PFEATURE_DATA_LAYER_JUMP_RECORDING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_LAYER_JUMP_RECORDING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER(pub *mut FEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER);
impl PFEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_LOGICAL_UNIT_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_MICROCODE_UPDATE(pub *mut FEATURE_DATA_MICROCODE_UPDATE);
impl PFEATURE_DATA_MICROCODE_UPDATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_MICROCODE_UPDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_MORPHING(pub *mut FEATURE_DATA_MORPHING);
impl PFEATURE_DATA_MORPHING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_MORPHING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_MRW(pub *mut FEATURE_DATA_MRW);
impl PFEATURE_DATA_MRW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_MRW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_MULTI_READ(pub *mut FEATURE_DATA_MULTI_READ);
impl PFEATURE_DATA_MULTI_READ {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_MULTI_READ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_POWER_MANAGEMENT(pub *mut FEATURE_DATA_POWER_MANAGEMENT);
impl PFEATURE_DATA_POWER_MANAGEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_POWER_MANAGEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_PROFILE_LIST(pub *mut FEATURE_DATA_PROFILE_LIST);
impl PFEATURE_DATA_PROFILE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_PROFILE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_PROFILE_LIST_EX(pub *mut FEATURE_DATA_PROFILE_LIST_EX);
impl PFEATURE_DATA_PROFILE_LIST_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_PROFILE_LIST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_RANDOM_READABLE(pub *mut FEATURE_DATA_RANDOM_READABLE);
impl PFEATURE_DATA_RANDOM_READABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_RANDOM_READABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_RANDOM_WRITABLE(pub *mut FEATURE_DATA_RANDOM_WRITABLE);
impl PFEATURE_DATA_RANDOM_WRITABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_RANDOM_WRITABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_REAL_TIME_STREAMING(pub *mut FEATURE_DATA_REAL_TIME_STREAMING);
impl PFEATURE_DATA_REAL_TIME_STREAMING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_REAL_TIME_STREAMING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_REMOVABLE_MEDIUM(pub *mut FEATURE_DATA_REMOVABLE_MEDIUM);
impl PFEATURE_DATA_REMOVABLE_MEDIUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_REMOVABLE_MEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_RESERVED(pub *mut FEATURE_DATA_RESERVED);
impl PFEATURE_DATA_RESERVED {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_RESERVED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_RESTRICTED_OVERWRITE(pub *mut FEATURE_DATA_RESTRICTED_OVERWRITE);
impl PFEATURE_DATA_RESTRICTED_OVERWRITE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_RESTRICTED_OVERWRITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_SECTOR_ERASABLE(pub *mut FEATURE_DATA_SECTOR_ERASABLE);
impl PFEATURE_DATA_SECTOR_ERASABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_SECTOR_ERASABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_SMART(pub *mut FEATURE_DATA_SMART);
impl PFEATURE_DATA_SMART {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_SMART {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_TIMEOUT(pub *mut FEATURE_DATA_TIMEOUT);
impl PFEATURE_DATA_TIMEOUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_TIMEOUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_VENDOR_SPECIFIC(pub *mut FEATURE_DATA_VENDOR_SPECIFIC);
impl PFEATURE_DATA_VENDOR_SPECIFIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_VENDOR_SPECIFIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_WRITE_ONCE(pub *mut FEATURE_DATA_WRITE_ONCE);
impl PFEATURE_DATA_WRITE_ONCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_WRITE_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_DATA_WRITE_PROTECT(pub *mut FEATURE_DATA_WRITE_PROTECT);
impl PFEATURE_DATA_WRITE_PROTECT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_DATA_WRITE_PROTECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_ENHANCED_DEFECT_REPORTING(pub *mut FEATURE_ENHANCED_DEFECT_REPORTING);
impl PFEATURE_ENHANCED_DEFECT_REPORTING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_ENHANCED_DEFECT_REPORTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_HEADER(pub *mut FEATURE_HEADER);
impl PFEATURE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_HYBRID_DISC(pub *mut FEATURE_HYBRID_DISC);
impl PFEATURE_HYBRID_DISC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_HYBRID_DISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_MEDIA_SERIAL_NUMBER(pub *mut FEATURE_MEDIA_SERIAL_NUMBER);
impl PFEATURE_MEDIA_SERIAL_NUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_MEDIA_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_NUMBER(pub *mut FEATURE_NUMBER);
impl PFEATURE_NUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_PROFILE_TYPE(pub *mut FEATURE_PROFILE_TYPE);
impl PFEATURE_PROFILE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_PROFILE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_TSR(pub *mut FEATURE_TSR);
impl PFEATURE_TSR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_TSR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFEATURE_VCPS(pub *mut FEATURE_VCPS);
impl PFEATURE_VCPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFEATURE_VCPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGET_CONFIGURATION_HEADER(pub *mut GET_CONFIGURATION_HEADER);
impl PGET_CONFIGURATION_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGET_CONFIGURATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGET_CONFIGURATION_IOCTL_INPUT(pub *mut GET_CONFIGURATION_IOCTL_INPUT);
impl PGET_CONFIGURATION_IOCTL_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGET_CONFIGURATION_IOCTL_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGET_CONFIGURATION_IOCTL_INPUT32(pub *mut GET_CONFIGURATION_IOCTL_INPUT32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PGET_CONFIGURATION_IOCTL_INPUT32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PGET_CONFIGURATION_IOCTL_INPUT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ProfileAS_MO: FEATURE_PROFILE_TYPE = 5;
pub const ProfileBDRRandomWritable: FEATURE_PROFILE_TYPE = 66;
pub const ProfileBDRSequentialWritable: FEATURE_PROFILE_TYPE = 65;
pub const ProfileBDRewritable: FEATURE_PROFILE_TYPE = 67;
pub const ProfileBDRom: FEATURE_PROFILE_TYPE = 64;
pub const ProfileCdRecordable: FEATURE_PROFILE_TYPE = 9;
pub const ProfileCdRewritable: FEATURE_PROFILE_TYPE = 10;
pub const ProfileCdrom: FEATURE_PROFILE_TYPE = 8;
pub const ProfileDDCdRecordable: FEATURE_PROFILE_TYPE = 33;
pub const ProfileDDCdRewritable: FEATURE_PROFILE_TYPE = 34;
pub const ProfileDDCdrom: FEATURE_PROFILE_TYPE = 32;
pub const ProfileDvdDashRDualLayer: FEATURE_PROFILE_TYPE = 21;
pub const ProfileDvdDashRLayerJump: FEATURE_PROFILE_TYPE = 22;
pub const ProfileDvdPlusR: FEATURE_PROFILE_TYPE = 27;
pub const ProfileDvdPlusRDualLayer: FEATURE_PROFILE_TYPE = 43;
pub const ProfileDvdPlusRW: FEATURE_PROFILE_TYPE = 26;
pub const ProfileDvdPlusRWDualLayer: FEATURE_PROFILE_TYPE = 42;
pub const ProfileDvdRWSequential: FEATURE_PROFILE_TYPE = 20;
pub const ProfileDvdRam: FEATURE_PROFILE_TYPE = 18;
pub const ProfileDvdRecordable: FEATURE_PROFILE_TYPE = 17;
pub const ProfileDvdRewritable: FEATURE_PROFILE_TYPE = 19;
pub const ProfileDvdRom: FEATURE_PROFILE_TYPE = 16;
pub const ProfileHDDVDRDualLayer: FEATURE_PROFILE_TYPE = 88;
pub const ProfileHDDVDRWDualLayer: FEATURE_PROFILE_TYPE = 90;
pub const ProfileHDDVDRam: FEATURE_PROFILE_TYPE = 82;
pub const ProfileHDDVDRecordable: FEATURE_PROFILE_TYPE = 81;
pub const ProfileHDDVDRewritable: FEATURE_PROFILE_TYPE = 83;
pub const ProfileHDDVDRom: FEATURE_PROFILE_TYPE = 80;
pub const ProfileInvalid: FEATURE_PROFILE_TYPE = 0;
pub const ProfileMOErasable: FEATURE_PROFILE_TYPE = 3;
pub const ProfileMOWriteOnce: FEATURE_PROFILE_TYPE = 4;
pub const ProfileNonRemovableDisk: FEATURE_PROFILE_TYPE = 1;
pub const ProfileNonStandard: FEATURE_PROFILE_TYPE = 65535;
pub const ProfileRemovableDisk: FEATURE_PROFILE_TYPE = 2;
pub const SCSI_GET_CONFIGURATION_REQUEST_TYPE_ALL: u32 = 0;
pub const SCSI_GET_CONFIGURATION_REQUEST_TYPE_CURRENT: u32 = 1;
pub const SCSI_GET_CONFIGURATION_REQUEST_TYPE_ONE: u32 = 2;
