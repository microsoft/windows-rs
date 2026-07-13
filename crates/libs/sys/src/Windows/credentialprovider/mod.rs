pub const CPAO_EMPTY_CONNECTED: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 2;
pub const CPAO_EMPTY_LOCAL: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 1;
pub const CPAO_NONE: CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = 0;
pub const CPCFO_ENABLE_PASSWORD_REVEAL: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 1;
pub const CPCFO_ENABLE_TOUCH_KEYBOARD_AUTO_INVOKE: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 4;
pub const CPCFO_IS_EMAIL_ADDRESS: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 2;
pub const CPCFO_NONE: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 0;
pub const CPCFO_NUMBERS_ONLY: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 8;
pub const CPCFO_SHOW_ENGLISH_KEYBOARD: CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = 16;
pub const CPFIS_DISABLED: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 2;
pub const CPFIS_FOCUSED: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 3;
pub const CPFIS_NONE: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 0;
pub const CPFIS_READONLY: CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = 1;
pub const CPFS_DISPLAY_IN_BOTH: CREDENTIAL_PROVIDER_FIELD_STATE = 3;
pub const CPFS_DISPLAY_IN_DESELECTED_TILE: CREDENTIAL_PROVIDER_FIELD_STATE = 2;
pub const CPFS_DISPLAY_IN_SELECTED_TILE: CREDENTIAL_PROVIDER_FIELD_STATE = 1;
pub const CPFS_HIDDEN: CREDENTIAL_PROVIDER_FIELD_STATE = 0;
pub const CPFT_CHECKBOX: CREDENTIAL_PROVIDER_FIELD_TYPE = 7;
pub const CPFT_COMBOBOX: CREDENTIAL_PROVIDER_FIELD_TYPE = 8;
pub const CPFT_COMMAND_LINK: CREDENTIAL_PROVIDER_FIELD_TYPE = 3;
pub const CPFT_EDIT_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 4;
pub const CPFT_INVALID: CREDENTIAL_PROVIDER_FIELD_TYPE = 0;
pub const CPFT_LARGE_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 1;
pub const CPFT_PASSWORD_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 5;
pub const CPFT_SMALL_TEXT: CREDENTIAL_PROVIDER_FIELD_TYPE = 2;
pub const CPFT_SUBMIT_BUTTON: CREDENTIAL_PROVIDER_FIELD_TYPE = 9;
pub const CPFT_TILE_IMAGE: CREDENTIAL_PROVIDER_FIELD_TYPE = 6;
pub const CPGSR_NO_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 1;
pub const CPGSR_NO_CREDENTIAL_NOT_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 0;
pub const CPGSR_RETURN_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 2;
pub const CPGSR_RETURN_NO_CREDENTIAL_FINISHED: CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = 3;
pub const CPSI_ERROR: CREDENTIAL_PROVIDER_STATUS_ICON = 1;
pub const CPSI_NONE: CREDENTIAL_PROVIDER_STATUS_ICON = 0;
pub const CPSI_SUCCESS: CREDENTIAL_PROVIDER_STATUS_ICON = 3;
pub const CPSI_WARNING: CREDENTIAL_PROVIDER_STATUS_ICON = 2;
pub const CPUS_CHANGE_PASSWORD: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 3;
pub const CPUS_CREDUI: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 4;
pub const CPUS_INVALID: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 0;
pub const CPUS_LOGON: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 1;
pub const CPUS_PLAP: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 5;
pub const CPUS_UNLOCK_WORKSTATION: CREDENTIAL_PROVIDER_USAGE_SCENARIO = 2;
pub type CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS = u32;
pub type CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS = u32;
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy)]
pub struct CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    pub ulAuthenticationPackage: u32,
    pub clsidCredentialProvider: windows_sys::core::GUID,
    pub cbSerialization: u32,
    pub rgbSerialization: *mut super::rpc::byte,
}
#[cfg(feature = "rpc")]
impl Default for CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    pub dwFieldID: u32,
    pub cpft: CREDENTIAL_PROVIDER_FIELD_TYPE,
    pub pszLabel: windows_sys::core::PWSTR,
    pub guidFieldType: windows_sys::core::GUID,
}
impl Default for CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE = i32;
pub type CREDENTIAL_PROVIDER_FIELD_STATE = i32;
pub type CREDENTIAL_PROVIDER_FIELD_TYPE = i32;
pub type CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE = i32;
pub const CREDENTIAL_PROVIDER_NO_DEFAULT: u32 = 4294967295;
pub type CREDENTIAL_PROVIDER_STATUS_ICON = i32;
pub type CREDENTIAL_PROVIDER_USAGE_SCENARIO = i32;
pub const GenericCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x25cbb996_92ed_457e_b28c_4774084bd562);
pub const Identity_LocalUserProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa198529b_730f_4089_b646_a12557f5665e);
pub const NPCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3dd6bec0_8193_4ffe_ae25_e08e39ea4063);
pub const OnexCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x07aa0886_cc8d_4e19_a410_1c75af686e62);
pub const OnexPlapSmartcardCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33c86cd6_705f_4ba1_9adb_67070b837775);
pub const PINLogonCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcb82ea12_9f71_446d_89e1_8d0924e1256e);
pub const PasswordCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x60b78e88_ead8_445c_9cfd_0b87f74ea6cd);
pub const RASProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5537e283_b1e7_4ef8_9c6e_7ab0afe5056d);
pub const SmartcardCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fd7e19c_3bf7_489b_a72c_846ab3678c96);
pub const SmartcardPinProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94596c7e_3744_41ce_893e_bbf09122f76a);
pub const SmartcardReaderSelectionProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1b283861_754f_4022_ad47_a5eaaa618894);
pub const SmartcardWinRTProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ee7337f_85ac_45e2_a23c_37c753209769);
pub const V1PasswordCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f45dc1e_5384_457a_bc13_2cd81b0d28ed);
pub const V1SmartcardCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8bf9a910_a8ff_457f_999f_a5ca10b4a885);
pub const V1WinBioCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac3ac249_e820_4343_a65b_377ac634dc09);
pub const VaultProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x503739d0_4c5e_4cfd_b3ba_d881334f0df2);
pub const WinBioCredentialProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbec09223_b018_416d_a0ac_523971b639f5);
