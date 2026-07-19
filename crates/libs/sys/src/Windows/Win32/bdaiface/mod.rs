pub const Associated: SmartCardAssociationType = 1;
pub const AssociationUnknown: SmartCardAssociationType = 2;
pub type BDA_DrmPairingError = i32;
pub const BDA_DrmPairing_Aborted: BDA_DrmPairingError = 8;
pub const BDA_DrmPairing_DrmInitFailed: BDA_DrmPairingError = 5;
pub const BDA_DrmPairing_DrmNotPaired: BDA_DrmPairingError = 6;
pub const BDA_DrmPairing_DrmRePairSoon: BDA_DrmPairingError = 7;
pub const BDA_DrmPairing_HardwareFailure: BDA_DrmPairingError = 1;
pub const BDA_DrmPairing_NeedIndiv: BDA_DrmPairingError = 3;
pub const BDA_DrmPairing_NeedRevocationData: BDA_DrmPairingError = 2;
pub const BDA_DrmPairing_NeedSDKUpdate: BDA_DrmPairingError = 9;
pub const BDA_DrmPairing_Other: BDA_DrmPairingError = 4;
pub const BDA_DrmPairing_Succeeded: BDA_DrmPairingError = 0;
pub const CLSID_BroadcastEventService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0b3ffb92_0919_4934_9d5b_619c719d0202);
pub const CLSID_PBDA_AUX_DATA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfd456373_3323_4090_adca_8ed45f55cf10);
pub const CLSID_PBDA_Encoder_DATA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x728fd6bc_5546_4716_b103_f899f5a1fa68);
pub const CLSID_PBDA_FDC_DATA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe7dbf9a0_22ab_4047_8e67_ef9ad504e729);
pub const CLSID_PBDA_GDDS_DATA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc80c0df3_6052_4c16_9f56_c44c21f73c45);
pub const CardDataChanged: SmartCardStatusType = 3;
pub const CardError: SmartCardStatusType = 2;
pub const CardFirmwareUpgrade: SmartCardStatusType = 4;
pub const CardInserted: SmartCardStatusType = 0;
pub const CardRemoved: SmartCardStatusType = 1;
pub const DeviceClosed: UICloseReasonType = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EALocationCodeType {
    pub LocationCodeScheme: LocationCodeSchemeType,
    pub state_code: u8,
    pub county_subdivision: u8,
    pub county_code: u16,
}
pub const Entitled: EntitlementType = 0;
pub type EntitlementType = i32;
pub const ErrorClosed: UICloseReasonType = 4;
pub type KSPROPERTY_IPSINK = i32;
pub const KSPROPERTY_IPSINK_ADAPTER_ADDRESS: KSPROPERTY_IPSINK = 2;
pub const KSPROPERTY_IPSINK_ADAPTER_DESCRIPTION: KSPROPERTY_IPSINK = 1;
pub const KSPROPERTY_IPSINK_MULTICASTLIST: KSPROPERTY_IPSINK = 0;
pub type LocationCodeSchemeType = i32;
pub const NotAssociated: SmartCardAssociationType = 0;
pub const NotEntitled: EntitlementType = 1;
pub const NotReady: UICloseReasonType = 0;
pub const PBDA_AUX_CONNECTOR_TYPE_Composite: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf6298b4c_c725_4d42_849b_410bbb14ea62);
pub const PBDA_AUX_CONNECTOR_TYPE_SVideo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa0e905f4_24c9_4a54_b761_213355efc13a);
pub const PBDA_Encoder_Audio_AlgorithmType_AC3: u32 = 1;
pub const PBDA_Encoder_Audio_AlgorithmType_MPEG1LayerII: u32 = 0;
pub const PBDA_Encoder_BitrateMode_Average: u32 = 3;
pub const PBDA_Encoder_BitrateMode_Constant: u32 = 1;
pub const PBDA_Encoder_BitrateMode_Variable: u32 = 2;
pub const PBDA_Encoder_Video_AVC: u32 = 1;
pub const PBDA_Encoder_Video_H264: u32 = 1;
pub const PBDA_Encoder_Video_MPEG2PartII: u32 = 0;
pub const PBDA_Encoder_Video_MPEG4Part10: u32 = 1;
pub const SCTE_18: LocationCodeSchemeType = 0;
#[repr(C)]
#[cfg(feature = "bdatypes")]
#[derive(Clone, Copy)]
pub struct SmartCardApplication {
    pub ApplicationType: super::ApplicationTypeType,
    pub ApplicationVersion: u16,
    pub pbstrApplicationName: windows_sys::core::BSTR,
    pub pbstrApplicationURL: windows_sys::core::BSTR,
}
#[cfg(feature = "bdatypes")]
impl Default for SmartCardApplication {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SmartCardAssociationType = i32;
pub type SmartCardStatusType = i32;
pub const SystemClosed: UICloseReasonType = 2;
pub const TechnicalFailure: EntitlementType = 2;
pub type UICloseReasonType = i32;
pub const UserClosed: UICloseReasonType = 1;
