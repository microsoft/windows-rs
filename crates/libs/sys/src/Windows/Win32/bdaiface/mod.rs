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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct EALocationCodeType {
    pub LocationCodeScheme: LocationCodeSchemeType,
    pub state_code: super::BYTE,
    pub county_subdivision: super::BYTE,
    pub county_code: super::WORD,
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
pub const PBDA_Encoder_Audio_AlgorithmType_AC3: i32 = 1;
pub const PBDA_Encoder_Audio_AlgorithmType_MPEG1LayerII: i32 = 0;
pub const PBDA_Encoder_BitrateMode_Average: i32 = 3;
pub const PBDA_Encoder_BitrateMode_Constant: i32 = 1;
pub const PBDA_Encoder_BitrateMode_Variable: i32 = 2;
pub const PBDA_Encoder_Video_AVC: i32 = 1;
pub const PBDA_Encoder_Video_H264: i32 = 1;
pub const PBDA_Encoder_Video_MPEG2PartII: i32 = 0;
pub const PBDA_Encoder_Video_MPEG4Part10: i32 = 1;
pub const SCTE_18: LocationCodeSchemeType = 0;
pub const SID_BDA_AUX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7def4c09_6e66_4567_a819_f0e17f4a81ab);
pub const SID_BDA_ConditionalAccess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcd51f1e0_7be9_4123_8482_a2a796c0a6b0);
pub const SID_BDA_ConditionalAccessEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x497c3418_23cb_44ba_bb62_769f506fcea7);
pub const SID_BDA_DRIDRMService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1f9bc2a5_44a3_4c52_aab1_0bbce5a1381d);
pub const SID_BDA_DRIWMDRMSession: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05c690f8_56db_4bb2_b053_79c12098bb26);
pub const SID_BDA_DRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf98d88b0_1992_4cd6_a6d9_b9afab99330d);
pub const SID_BDA_DRMService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbff6b5bb_b0ae_484c_9dca_73528fb0b46e);
pub const SID_BDA_DiagnosticProperties: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x20e80cb5_c543_4c1b_8eb3_49e719eee7d4);
pub const SID_BDA_EasMessage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd806973d_3ebe_46de_8fbb_6358fe784208);
pub const SID_BDA_Encoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3a8bad59_59fe_4559_a0ba_396cfaa98ae3);
pub const SID_BDA_EventingService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x207c413f_00dc_4c61_bad6_6fee1ff07064);
pub const SID_BDA_GuideDataDeliveryService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc0afcb73_23e7_4bc6_bafa_fdc167b4719f);
pub const SID_BDA_ISDBConditionalAccess: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5e68c627_16c2_4e6c_b1e2_d00170cdaa0f);
pub const SID_BDA_MUX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x942aafec_4c05_4c74_b8eb_8706c2a4943f);
pub const SID_BDA_NameValueService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f0b3150_7b81_4ad4_98e3_7e9097094301);
pub const SID_BDA_TransportStreamInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8e882535_5f86_47ab_86cf_c281a72a0549);
pub const SID_BDA_TransportStreamSelector: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1dcfafe9_b45e_41b3_bb2a_561eb129ae98);
pub const SID_BDA_UserActivityService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x53b14189_e478_4b7a_a1ff_506db4b99dfe);
pub const SID_BDA_WMDRMSession: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4be6fa3d_07cd_4139_8b80_8c18ba3aec88);
pub const SID_BDA_WMDRMTuner: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x86d979cf_a8a7_4f94_b5fb_14c0aca68fe6);
#[repr(C)]
#[cfg(all(feature = "bdatypes", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct SmartCardApplication {
    pub ApplicationType: super::ApplicationTypeType,
    pub ApplicationVersion: super::USHORT,
    pub pbstrApplicationName: windows_sys::core::BSTR,
    pub pbstrApplicationURL: windows_sys::core::BSTR,
}
pub type SmartCardAssociationType = i32;
pub type SmartCardStatusType = i32;
pub const SystemClosed: UICloseReasonType = 2;
pub const TechnicalFailure: EntitlementType = 2;
pub type UICloseReasonType = i32;
pub const UserClosed: UICloseReasonType = 1;
