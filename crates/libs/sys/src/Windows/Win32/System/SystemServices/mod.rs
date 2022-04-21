#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterDeviceNotification(handle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_ALLOWED_ACE_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_ALLOWED_CALLBACK_ACE_TYPE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_ALLOWED_COMPOUND_ACE_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_ALLOWED_OBJECT_ACE_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DENIED_ACE_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DENIED_CALLBACK_ACE_TYPE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DENIED_OBJECT_ACE_TYPE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DS_OBJECT_TYPE_NAME_A: &str = "Directory Service Object";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DS_OBJECT_TYPE_NAME_W: &str = "Directory Service Object";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DS_SOURCE_A: &str = "DS";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_DS_SOURCE_W: &str = "DS";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_FILTER_SECURITY_INFORMATION: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_LEVEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_MS_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_MS_OBJECT_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_MS_V2_ACE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_MS_V3_ACE_TYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_MS_V4_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MAX_MS_V5_ACE_TYPE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MIN_MS_ACE_TYPE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_MIN_MS_OBJECT_ACE_TYPE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_OBJECT_GUID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_PROPERTY_GUID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_PROPERTY_SET_GUID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_REASON_DATA_MASK: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_REASON_EXDATA_MASK: u32 = 2130706432u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_REASON_STAGING_MASK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ACCESS_REASON_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNone: ACCESS_REASON_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonAllowedAce: ACCESS_REASON_TYPE = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonDeniedAce: ACCESS_REASON_TYPE = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonAllowedParentAce: ACCESS_REASON_TYPE = 196608i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonDeniedParentAce: ACCESS_REASON_TYPE = 262144i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNotGrantedByCape: ACCESS_REASON_TYPE = 327680i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNotGrantedByParentCape: ACCESS_REASON_TYPE = 393216i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNotGrantedToAppContainer: ACCESS_REASON_TYPE = 458752i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonMissingPrivilege: ACCESS_REASON_TYPE = 1048576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonFromPrivilege: ACCESS_REASON_TYPE = 2097152i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonIntegrityLevel: ACCESS_REASON_TYPE = 3145728i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonOwnership: ACCESS_REASON_TYPE = 4194304i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNullDacl: ACCESS_REASON_TYPE = 5242880i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonEmptyDacl: ACCESS_REASON_TYPE = 6291456i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNoSD: ACCESS_REASON_TYPE = 7340032i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonNoGrant: ACCESS_REASON_TYPE = 8388608i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonTrustLabel: ACCESS_REASON_TYPE = 9437184i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AccessReasonFilterAce: ACCESS_REASON_TYPE = 10485760i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_REASON_TYPE_MASK: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACCESS_SYSTEM_SECURITY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACL_REVISION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACL_REVISION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACL_REVISION3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACL_REVISION4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACPI_PPM_HARDWARE_ALL: u32 = 254u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACPI_PPM_SOFTWARE_ALL: u32 = 252u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACPI_PPM_SOFTWARE_ANY: u32 = 253u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ACTIVATION_CONTEXT_INFO_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ActivationContextBasicInformation: ACTIVATION_CONTEXT_INFO_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ActivationContextDetailedInformation: ACTIVATION_CONTEXT_INFO_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AssemblyDetailedInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FileInformationInAssemblyOfAssemblyInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RunlevelInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CompatibilityInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ActivationContextManifestResourceName: ACTIVATION_CONTEXT_INFO_CLASS = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MaxActivationContextInfoClass: ACTIVATION_CONTEXT_INFO_CLASS = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AssemblyDetailedInformationInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FileInformationInAssemblyOfAssemblyInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_PATH_TYPE_ASSEMBLYREF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_PATH_TYPE_NONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_PATH_TYPE_URL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_PATH_TYPE_WIN32_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_APPLICATION_SETTINGS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_ASSEMBLY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_CLR_SURROGATES: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_COMPATIBILITY_INFO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_COM_INTERFACE_REDIRECTION: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_COM_PROGID_REDIRECTION: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_COM_SERVER_REDIRECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_COM_TYPE_LIBRARY_REDIRECTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_DLL_REDIRECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_GLOBAL_OBJECT_RENAME_TABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_WINDOW_CLASS_REDIRECTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ACTIVATION_CONTEXT_SECTION_WINRT_ACTIVATABLE_CLASSES: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ALERT_SYSTEM_SEV = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALERT_SYSTEM_INFORMATIONAL: ALERT_SYSTEM_SEV = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALERT_SYSTEM_WARNING: ALERT_SYSTEM_SEV = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALERT_SYSTEM_ERROR: ALERT_SYSTEM_SEV = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALERT_SYSTEM_QUERY: ALERT_SYSTEM_SEV = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALERT_SYSTEM_CRITICAL: ALERT_SYSTEM_SEV = 5u32;
pub const ALL_POWERSCHEMES_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1755441502, data2: 5098, data3: 16865, data4: [128, 17, 12, 73, 108, 164, 144, 176] };
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALL_PROCESSOR_GROUPS: u32 = 65535u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct ANON_OBJECT_HEADER {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: ::windows_sys::core::GUID,
    pub SizeOfData: u32,
}
impl ::core::marker::Copy for ANON_OBJECT_HEADER {}
impl ::core::clone::Clone for ANON_OBJECT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct ANON_OBJECT_HEADER_BIGOBJ {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: ::windows_sys::core::GUID,
    pub SizeOfData: u32,
    pub Flags: u32,
    pub MetaDataSize: u32,
    pub MetaDataOffset: u32,
    pub NumberOfSections: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
}
impl ::core::marker::Copy for ANON_OBJECT_HEADER_BIGOBJ {}
impl ::core::clone::Clone for ANON_OBJECT_HEADER_BIGOBJ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct ANON_OBJECT_HEADER_V2 {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: ::windows_sys::core::GUID,
    pub SizeOfData: u32,
    pub Flags: u32,
    pub MetaDataSize: u32,
    pub MetaDataOffset: u32,
}
impl ::core::marker::Copy for ANON_OBJECT_HEADER_V2 {}
impl ::core::clone::Clone for ANON_OBJECT_HEADER_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ANYSIZE_ARRAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type APC_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type APPCOMMAND_ID = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_BACKWARD: APPCOMMAND_ID = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_FORWARD: APPCOMMAND_ID = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_REFRESH: APPCOMMAND_ID = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_STOP: APPCOMMAND_ID = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_SEARCH: APPCOMMAND_ID = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_FAVORITES: APPCOMMAND_ID = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BROWSER_HOME: APPCOMMAND_ID = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_VOLUME_MUTE: APPCOMMAND_ID = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_VOLUME_DOWN: APPCOMMAND_ID = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_VOLUME_UP: APPCOMMAND_ID = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_NEXTTRACK: APPCOMMAND_ID = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_PREVIOUSTRACK: APPCOMMAND_ID = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_STOP: APPCOMMAND_ID = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_PLAY_PAUSE: APPCOMMAND_ID = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_LAUNCH_MAIL: APPCOMMAND_ID = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_LAUNCH_MEDIA_SELECT: APPCOMMAND_ID = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_LAUNCH_APP1: APPCOMMAND_ID = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_LAUNCH_APP2: APPCOMMAND_ID = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BASS_DOWN: APPCOMMAND_ID = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BASS_BOOST: APPCOMMAND_ID = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_BASS_UP: APPCOMMAND_ID = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_TREBLE_DOWN: APPCOMMAND_ID = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_TREBLE_UP: APPCOMMAND_ID = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MICROPHONE_VOLUME_MUTE: APPCOMMAND_ID = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MICROPHONE_VOLUME_DOWN: APPCOMMAND_ID = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MICROPHONE_VOLUME_UP: APPCOMMAND_ID = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_HELP: APPCOMMAND_ID = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_FIND: APPCOMMAND_ID = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_NEW: APPCOMMAND_ID = 29u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_OPEN: APPCOMMAND_ID = 30u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_CLOSE: APPCOMMAND_ID = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_SAVE: APPCOMMAND_ID = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_PRINT: APPCOMMAND_ID = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_UNDO: APPCOMMAND_ID = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_REDO: APPCOMMAND_ID = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_COPY: APPCOMMAND_ID = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_CUT: APPCOMMAND_ID = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_PASTE: APPCOMMAND_ID = 38u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_REPLY_TO_MAIL: APPCOMMAND_ID = 39u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_FORWARD_MAIL: APPCOMMAND_ID = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_SEND_MAIL: APPCOMMAND_ID = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_SPELL_CHECK: APPCOMMAND_ID = 42u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE: APPCOMMAND_ID = 43u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MIC_ON_OFF_TOGGLE: APPCOMMAND_ID = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_CORRECTION_LIST: APPCOMMAND_ID = 45u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_PLAY: APPCOMMAND_ID = 46u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_PAUSE: APPCOMMAND_ID = 47u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_RECORD: APPCOMMAND_ID = 48u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_FAST_FORWARD: APPCOMMAND_ID = 49u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_REWIND: APPCOMMAND_ID = 50u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_CHANNEL_UP: APPCOMMAND_ID = 51u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_MEDIA_CHANNEL_DOWN: APPCOMMAND_ID = 52u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_DELETE: APPCOMMAND_ID = 53u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPCOMMAND_DWM_FLIP3D: APPCOMMAND_ID = 54u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct APPLICATIONLAUNCH_SETTING_VALUE {
    pub ActivationTime: i64,
    pub Flags: u32,
    pub ButtonInstanceID: u32,
}
impl ::core::marker::Copy for APPLICATIONLAUNCH_SETTING_VALUE {}
impl ::core::clone::Clone for APPLICATIONLAUNCH_SETTING_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const APPLICATION_ERROR_MASK: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ARM64_FNPDATA_CR = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataCrUnchained: ARM64_FNPDATA_CR = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataCrUnchainedSavedLr: ARM64_FNPDATA_CR = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataCrChainedWithPac: ARM64_FNPDATA_CR = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataCrChained: ARM64_FNPDATA_CR = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ARM64_FNPDATA_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataRefToFullXdata: ARM64_FNPDATA_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataPackedUnwindFunction: ARM64_FNPDATA_FLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PdataPackedUnwindFragment: ARM64_FNPDATA_FLAGS = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_MAX_BREAKPOINTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_MAX_WATCHPOINTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_MULT_INTRINSICS_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_KEEP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_L1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_L2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_L3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_PLD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_PLI: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_PST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM64_PREFETCH_STRM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM_CACHE_ALIGNMENT_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM_MAX_BREAKPOINTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ARM_MAX_WATCHPOINTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ASSERT_BREAKPOINT: u32 = 524291u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ATF_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ATF_TIMEOUTON: ATF_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ATF_ONOFFFEEDBACK: ATF_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AUDIT_ALLOW_NO_PRIVILEGE: u32 = 1u32;
#[repr(C)]
pub struct AtlThunkData_t(pub u8);
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const BATTERY_DISCHARGE_FLAGS_ENABLE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const BATTERY_DISCHARGE_FLAGS_EVENTCODE_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const BREAK_DEBUG_BASE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const BSF_MSGSRV32ISOK: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const BSF_MSGSRV32ISOK_BIT: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CACHE_FULLY_ASSOCIATIVE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type CFE_UNDERLINE = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_CF1UNDERLINE: CFE_UNDERLINE = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_INVERT: CFE_UNDERLINE = 254u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINETHICKLONGDASH: CFE_UNDERLINE = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINETHICKDOTTED: CFE_UNDERLINE = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINETHICKDASHDOTDOT: CFE_UNDERLINE = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINETHICKDASHDOT: CFE_UNDERLINE = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINETHICKDASH: CFE_UNDERLINE = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINELONGDASH: CFE_UNDERLINE = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEHEAVYWAVE: CFE_UNDERLINE = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEDOUBLEWAVE: CFE_UNDERLINE = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEHAIRLINE: CFE_UNDERLINE = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINETHICK: CFE_UNDERLINE = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEWAVE: CFE_UNDERLINE = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEDASHDOTDOT: CFE_UNDERLINE = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEDASHDOT: CFE_UNDERLINE = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEDASH: CFE_UNDERLINE = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEDOTTED: CFE_UNDERLINE = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEDOUBLE: CFE_UNDERLINE = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINEWORD: CFE_UNDERLINE = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINE: CFE_UNDERLINE = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFU_UNDERLINENONE: CFE_UNDERLINE = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFG_CALL_TARGET_CONVERT_EXPORT_SUPPRESSED_TO_VALID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFG_CALL_TARGET_CONVERT_XFG_TO_CFG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFG_CALL_TARGET_PROCESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFG_CALL_TARGET_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CFG_CALL_TARGET_VALID_XFG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type CHOOSECOLOR_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_RGBINIT: CHOOSECOLOR_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_FULLOPEN: CHOOSECOLOR_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_PREVENTFULLOPEN: CHOOSECOLOR_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_SHOWHELP: CHOOSECOLOR_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_ENABLEHOOK: CHOOSECOLOR_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_ENABLETEMPLATE: CHOOSECOLOR_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_ENABLETEMPLATEHANDLE: CHOOSECOLOR_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_SOLIDCOLOR: CHOOSECOLOR_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CC_ANYCOLOR: CHOOSECOLOR_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type CLIPBOARD_FORMATS = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_TEXT: CLIPBOARD_FORMATS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_BITMAP: CLIPBOARD_FORMATS = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_METAFILEPICT: CLIPBOARD_FORMATS = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_SYLK: CLIPBOARD_FORMATS = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DIF: CLIPBOARD_FORMATS = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_TIFF: CLIPBOARD_FORMATS = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_OEMTEXT: CLIPBOARD_FORMATS = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DIB: CLIPBOARD_FORMATS = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_PALETTE: CLIPBOARD_FORMATS = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_PENDATA: CLIPBOARD_FORMATS = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_RIFF: CLIPBOARD_FORMATS = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_WAVE: CLIPBOARD_FORMATS = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_UNICODETEXT: CLIPBOARD_FORMATS = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_ENHMETAFILE: CLIPBOARD_FORMATS = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_HDROP: CLIPBOARD_FORMATS = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_LOCALE: CLIPBOARD_FORMATS = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DIBV5: CLIPBOARD_FORMATS = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_MAX: CLIPBOARD_FORMATS = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_OWNERDISPLAY: CLIPBOARD_FORMATS = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DSPTEXT: CLIPBOARD_FORMATS = 129u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DSPBITMAP: CLIPBOARD_FORMATS = 130u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DSPMETAFILEPICT: CLIPBOARD_FORMATS = 131u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_DSPENHMETAFILE: CLIPBOARD_FORMATS = 142u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_PRIVATEFIRST: CLIPBOARD_FORMATS = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_PRIVATELAST: CLIPBOARD_FORMATS = 767u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_GDIOBJFIRST: CLIPBOARD_FORMATS = 768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CF_GDIOBJLAST: CLIPBOARD_FORMATS = 1023u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type CM_ERROR_CONTROL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IgnoreError: CM_ERROR_CONTROL_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NormalError: CM_ERROR_CONTROL_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SevereError: CM_ERROR_CONTROL_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CriticalError: CM_ERROR_CONTROL_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type CM_SERVICE_LOAD_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const BootLoad: CM_SERVICE_LOAD_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SystemLoad: CM_SERVICE_LOAD_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AutoLoad: CM_SERVICE_LOAD_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DemandLoad: CM_SERVICE_LOAD_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DisableLoad: CM_SERVICE_LOAD_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_MEASURED_BOOT_LOAD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_NETWORK_BOOT_LOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type CM_SERVICE_NODE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DriverType: CM_SERVICE_NODE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FileSystemType: CM_SERVICE_NODE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const Win32ServiceOwnProcess: CM_SERVICE_NODE_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const Win32ServiceShareProcess: CM_SERVICE_NODE_TYPE = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const AdapterType: CM_SERVICE_NODE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RecognizerType: CM_SERVICE_NODE_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_RAM_DISK_BOOT_LOAD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_SD_DISK_BOOT_LOAD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_USB3_DISK_BOOT_LOAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_USB_DISK_BOOT_LOAD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_VERIFIER_BOOT_LOAD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CM_SERVICE_WINPE_BOOT_LOAD: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct COMPONENT_FILTER {
    pub ComponentFlags: u32,
}
impl ::core::marker::Copy for COMPONENT_FILTER {}
impl ::core::clone::Clone for COMPONENT_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPONENT_KTM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPONENT_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_ENGINE_HIBER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_ENGINE_MAXIMUM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_ENGINE_STANDARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_FORMAT_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_FORMAT_LZNT1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_FORMAT_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_FORMAT_XP10: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_FORMAT_XPRESS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMPRESSION_FORMAT_XPRESS_HUFF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_AMD64: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_ARM: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_ARM64: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_ARM64_RET_TO_GUEST: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_ARM64_UNWOUND_TO_CALL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_EXCEPTION_ACTIVE: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_EXCEPTION_REPORTING: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_EXCEPTION_REQUEST: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_KERNEL_DEBUGGER: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_RET_TO_GUEST: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_SERVICE_ACTIVE: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_UNWOUND_TO_CALL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_i386: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CONTEXT_i486: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CORE_PARKING_POLICY_CHANGE_IDEAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CORE_PARKING_POLICY_CHANGE_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CORE_PARKING_POLICY_CHANGE_MULTISTEP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CORE_PARKING_POLICY_CHANGE_ROCKET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CORE_PARKING_POLICY_CHANGE_SINGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CREATE_BOUNDARY_DESCRIPTOR_ADD_APPCONTAINER_SID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CRITICAL_ACE_FLAG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CTMF_INCLUDE_APPCONTAINER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const CTMF_INCLUDE_LPAC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLEAR_STENCIL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLEAR_TARGET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLEAR_ZBUFFER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPPLANE0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPPLANE1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPPLANE2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPPLANE3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPPLANE4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPPLANE5: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPSTATUS_EXTENTS2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPSTATUS_EXTENTS3: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIPSTATUS_STATUS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_BACK: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_BOTTOM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_FRONT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_GEN0: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_GEN1: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_GEN2: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_GEN3: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_GEN4: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_GEN5: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_LEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_RIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCLIP_TOP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCOLOR_MONO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DCOLOR_RGB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_BCLIPPING: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_COLORMODEL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_DEVCAPS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_DEVICERENDERBITDEPTH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_DEVICEZBUFFERBITDEPTH: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_LIGHTINGCAPS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_LINECAPS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_MAXBUFFERSIZE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_MAXVERTEXCOUNT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_TRANSFORMCAPS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDD_TRICAPS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEBCAPS_SYSTEMMEMORY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEBCAPS_VIDEOMEMORY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEB_BUFSIZE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEB_CAPS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEB_LPDATA: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_CANBLTSYSTONONLOCAL: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_CANRENDERAFTERFLIP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_DRAWPRIMITIVES2: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_DRAWPRIMITIVES2EX: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_DRAWPRIMTLVERTEX: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_EXECUTESYSTEMMEMORY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_EXECUTEVIDEOMEMORY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_FLOATTLVERTEX: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_HWRASTERIZATION: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_HWTRANSFORMANDLIGHT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_SEPARATETEXTUREMEMORIES: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_SORTDECREASINGZ: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_SORTEXACT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_SORTINCREASINGZ: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_TEXTURENONLOCALVIDMEM: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_TEXTURESYSTEMMEMORY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_TEXTUREVIDEOMEMORY: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_TLVERTEXSYSTEMMEMORY: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVCAPS_TLVERTEXVIDEOMEMORY: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVINFOID_D3DTEXTUREMANAGER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVINFOID_TEXTUREMANAGER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDEVINFOID_TEXTURING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DDP_MAXTEXCOORD: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DEXECUTE_CLIPPED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DEXECUTE_UNCLIPPED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_ALPHACMPCAPS: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_COLORMODEL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_DSTBLENDCAPS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_GUID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_HARDWARE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_LINES: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_MISCCAPS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_RASTERCAPS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_SHADECAPS: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_SRCBLENDCAPS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_TEXTUREADDRESSCAPS: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_TEXTUREBLENDCAPS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_TEXTURECAPS: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_TEXTUREFILTERCAPS: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_TRIANGLES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFDS_ZCMPCAPS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVFCAPS_DONOTSTRIPELEMENTS: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVFCAPS_TEXCOORDCOUNTMASK: i32 = 65535i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_DIFFUSE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_NORMAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_POSITION_MASK: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_RESERVED0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_RESERVED1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_RESERVED2: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_SPECULAR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX1: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX2: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX3: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX4: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX5: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX6: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX7: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEX8: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEXCOUNT_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEXCOUNT_SHIFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEXTUREFORMAT1: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEXTUREFORMAT2: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEXTUREFORMAT3: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_TEXTUREFORMAT4: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZB1: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZB2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZB3: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZB4: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZB5: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DFVF_XYZRHW: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_SAMPLER_MAXSAMP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_SAMPLER_MAXVERTEXSAMP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_STATESETBEGIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_STATESETCAPTURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_STATESETDELETE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_STATESETEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DHAL_STATESETEXECUTE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DINFINITEINSTRUCTIONS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTCAPS_DIRECTIONAL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTCAPS_GLSPOT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTCAPS_PARALLELPOINT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTCAPS_POINT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTCAPS_SPOT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTINGMODEL_MONO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHTINGMODEL_RGB: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHT_ACTIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DLIGHT_NO_SPECULAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DMAXUSERCLIPPLANES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTCLEAR_COMPUTERECTS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL2_CB32_SETRENDERTARGET: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL3_CB32_CLEAR2: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL3_CB32_DRAWPRIMITIVES2: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL3_CB32_RESERVED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL3_CB32_VALIDATETEXTURESTAGESTATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_EXECUTEBUFFER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_REQCOMMANDBUFSIZE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_REQVERTEXBUFSIZE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_SWAPCOMMANDBUFFER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_SWAPVERTEXBUFFER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_USERMEMVERTICES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_VIDMEMCOMMANDBUF: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHALDP2_VIDMEMVERTEXBUF: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_COL_WEIGHTS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_CONTEXT_BAD: i64 = 512i64;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_NUMCLIPVERTICES: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_OUTOFCONTEXTS: i64 = 513i64;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_ROW_WEIGHTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_SCENE_CAPTURE_END: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_SCENE_CAPTURE_START: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_STATESETCREATE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_TSS_MAXSTAGES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_TSS_RENDERSTATEBASE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DNTHAL_TSS_STATESPERSTAGE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPAL_FREE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPAL_READONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPAL_RESERVED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_BOTHINVSRCALPHA: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_BOTHSRCALPHA: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_DESTALPHA: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_DESTCOLOR: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_INVDESTALPHA: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_INVDESTCOLOR: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_INVSRCALPHA: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_INVSRCCOLOR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_ONE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_SRCALPHA: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_SRCALPHASAT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_SRCCOLOR: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPBLENDCAPS_ZERO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_ALWAYS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_EQUAL: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_GREATER: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_GREATEREQUAL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_LESS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_LESSEQUAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_NEVER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPCMPCAPS_NOTEQUAL: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_CONFORMANT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_CULLCCW: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_CULLCW: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_CULLNONE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_LINEPATTERNREP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_MASKPLANES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPMISCCAPS_MASKZ: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ANISOTROPY: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ANTIALIASEDGES: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ANTIALIASSORTDEPENDENT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ANTIALIASSORTINDEPENDENT: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_DITHER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_FOGRANGE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_FOGTABLE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_FOGVERTEX: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_MIPMAPLODBIAS: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_PAT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ROP2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_STIPPLE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_STRETCHBLTMULTISAMPLE: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_SUBPIXEL: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_SUBPIXELX: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_TRANSLUCENTSORTINDEPENDENT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_WBUFFER: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_WFOG: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_XOR: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ZBIAS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ZBUFFERLESSHSR: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ZFOG: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPRASTERCAPS_ZTEST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPROCESSVERTICES_COPY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPROCESSVERTICES_NOCOLOR: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPROCESSVERTICES_OPMASK: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPROCESSVERTICES_TRANSFORM: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPROCESSVERTICES_TRANSFORMLIGHT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPROCESSVERTICES_UPDATEEXTENTS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_ALPHAFLATBLEND: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_ALPHAFLATSTIPPLED: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_ALPHAGOURAUDBLEND: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_ALPHAGOURAUDSTIPPLED: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_ALPHAPHONGBLEND: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_ALPHAPHONGSTIPPLED: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_COLORFLATMONO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_COLORFLATRGB: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_COLORGOURAUDMONO: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_COLORGOURAUDRGB: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_COLORPHONGMONO: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_COLORPHONGRGB: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_FOGFLAT: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_FOGGOURAUD: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_FOGPHONG: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_SPECULARFLATMONO: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_SPECULARFLATRGB: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_SPECULARGOURAUDMONO: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_SPECULARGOURAUDRGB: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_SPECULARPHONGMONO: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPSHADECAPS_SPECULARPHONGRGB: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTADDRESSCAPS_BORDER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTADDRESSCAPS_CLAMP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTADDRESSCAPS_INDEPENDENTUV: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTADDRESSCAPS_MIRROR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTADDRESSCAPS_WRAP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_ADD: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_COPY: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_DECAL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_DECALALPHA: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_DECALMASK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_MODULATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_MODULATEALPHA: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTBLENDCAPS_MODULATEMASK: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_ALPHA: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_ALPHAPALETTE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_BORDER: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_COLORKEYBLEND: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_CUBEMAP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_NONPOW2CONDITIONAL: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_PERSPECTIVE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_POW2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_PROJECTED: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_SQUAREONLY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTEXTURECAPS_TRANSPARENCY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_LINEAR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_LINEARMIPLINEAR: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_LINEARMIPNEAREST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MAGFAFLATCUBIC: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MAGFANISOTROPIC: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MAGFGAUSSIANCUBIC: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MAGFLINEAR: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MAGFPOINT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MINFANISOTROPIC: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MINFLINEAR: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MINFPOINT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MIPFLINEAR: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MIPFPOINT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MIPLINEAR: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_MIPNEAREST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPTFILTERCAPS_NEAREST: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DPV_DONOTCOPYDATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DRENDERSTATE_EVICTMANAGEDTEXTURES: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DRENDERSTATE_SCENECAPTURE: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DRENDERSTATE_WRAPBIAS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DRS_MAXPIXELSHADERINST: u32 = 197u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DRS_MAXVERTEXSHADERINST: u32 = 196u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSETSTATUS_EXTENTS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSETSTATUS_STATUS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATE_OVERRIDE_BIAS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONBACK: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONBOTTOM: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONFRONT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONGEN0: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONGEN1: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONGEN2: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONGEN3: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONGEN4: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONGEN5: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONLEFT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONRIGHT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPINTERSECTIONTOP: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONBACK: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONBOTTOM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONFRONT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONGEN0: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONGEN1: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONGEN2: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONGEN3: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONGEN4: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONGEN5: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONLEFT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONRIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_CLIPUNIONTOP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTATUS_ZNOTVISIBLE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_DECR: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_DECRSAT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_INCR: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_INCRSAT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_INVERT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_KEEP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_REPLACE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DSTENCILCAPS_ZERO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_ALPHAREPLICATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_COMPLEMENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_CURRENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_DIFFUSE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_SELECTMASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_SPECULAR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_TEXTURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTA_TFACTOR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_ADD: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_ADDSIGNED: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_ADDSIGNED2X: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_ADDSMOOTH: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BLENDCURRENTALPHA: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BLENDDIFFUSEALPHA: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BLENDFACTORALPHA: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHA: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BLENDTEXTUREALPHAPM: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BUMPENVMAP: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_BUMPENVMAPLUMINANCE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_DISABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_DOTPRODUCT3: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATE2X: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATE4X: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATEALPHA_ADDCOLOR: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATECOLOR_ADDALPHA: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATEINVALPHA_ADDCOLOR: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_MODULATEINVCOLOR_ADDALPHA: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_PREMODULATE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_SELECTARG1: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_SELECTARG2: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTEXOPCAPS_SUBTRACT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRANSFORMCAPS_CLIP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRANSFORM_CLIPPED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRANSFORM_UNCLIPPED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRIFLAG_EDGEENABLE1: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRIFLAG_EDGEENABLE2: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRIFLAG_EDGEENABLE3: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRIFLAG_EVEN: i32 = 31i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRIFLAG_ODD: i32 = 30i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTRIFLAG_START: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTSS_TCI_CAMERASPACENORMAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTSS_TCI_CAMERASPACEPOSITION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTSS_TCI_CAMERASPACEREFLECTIONVECTOR: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTSS_TCI_PASSTHRU: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DTSS_TEXTUREMAP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVBCAPS_DONOTCLIP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVBCAPS_OPTIMIZED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVBCAPS_SYSTEMMEMORY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVBCAPS_WRITEONLY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_BOTTOM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_FAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_FRUSTUM: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_LEFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_NEAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_RIGHT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INSIDE_TOP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_BOTTOM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_FAR: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_FRUSTUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_LEFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_NEAR: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_RIGHT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_INTERSECT_TOP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_BOTTOM: u32 = 768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_FAR: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_FRUSTUM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_LEFT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_NEAR: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_RIGHT: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_MASK_TOP: u32 = 192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_BOTTOM: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_FAR: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_FRUSTUM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_LEFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_NEAR: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_RIGHT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVIS_OUTSIDE_TOP: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVOP_CLIP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVOP_EXTENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVOP_LIGHT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVOP_TRANSFORM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_BLENDINDICES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_BLENDWEIGHT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_DIFFUSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_NORMAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_NORMAL2: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_POSITION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_POSITION2: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_PSIZE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_SPECULAR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD0: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD2: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD3: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD4: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD5: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD6: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDE_TEXCOORD7: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_D3DCOLOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_FLOAT1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_FLOAT2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_FLOAT3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_FLOAT4: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_SHORT2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_SHORT4: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSDT_UBYTE4: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_CONSTADDRESSSHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_CONSTCOUNTSHIFT: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_CONSTRSSHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_DATALOADTYPESHIFT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_DATATYPESHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_EXTCOUNTSHIFT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_EXTINFOSHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_SKIPCOUNTSHIFT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_STREAMNUMBERSHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_STREAMTESSSHIFT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_TOKENTYPESHIFT: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_VERTEXREGINSHIFT: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVSD_VERTEXREGSHIFT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVTXPCAPS_DIRECTIONALLIGHTS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVTXPCAPS_LOCALVIEWER: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVTXPCAPS_MATERIALSOURCE7: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVTXPCAPS_POSITIONALLIGHTS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVTXPCAPS_TEXGEN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DVTXPCAPS_VERTEXFOG: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DWRAPCOORD_0: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DWRAPCOORD_1: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DWRAPCOORD_2: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DWRAPCOORD_3: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DWRAP_U: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const D3DWRAP_V: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBTF_RESOURCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBTF_SLOWNET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBTF_XPORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_APPYBEGIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_APPYEND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_CONFIGCHANGECANCELED: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_CONFIGCHANGED: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_CONFIGMGAPI32: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_CONFIGMGPRIVATE: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_CUSTOMEVENT: u32 = 32774u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVICEARRIVAL: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVICEQUERYREMOVE: u32 = 32769u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVICEQUERYREMOVEFAILED: u32 = 32770u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVICEREMOVECOMPLETE: u32 = 32772u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVICEREMOVEPENDING: u32 = 32771u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVICETYPESPECIFIC: u32 = 32773u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVNODES_CHANGED: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_DEVNODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_NET: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_LOW_DISK_SPACE: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_MONITORCHANGE: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_NO_DISK_SPACE: u32 = 71u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_QUERYCHANGECONFIG: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_SHELLLOGGEDON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_USERDEFINED: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VOLLOCKLOCKFAILED: u32 = 32835u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VOLLOCKLOCKRELEASED: u32 = 32837u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VOLLOCKLOCKTAKEN: u32 = 32834u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VOLLOCKQUERYLOCK: u32 = 32833u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VOLLOCKQUERYUNLOCK: u32 = 32836u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VOLLOCKUNLOCKFAILED: u32 = 32838u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VPOWERDAPI: u32 = 33024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_VXDINITCOMPLETE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DDBLT_EXTENDED_PRESENTATION_STRETCHFACTOR: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DEDICATED_MEMORY_CACHE_ELIGIBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DELETE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DEVICEFAMILYDEVICEFORM_KEY: &str = "\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion\\OEM";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DEVICEFAMILYDEVICEFORM_VALUE: &str = "DeviceForm";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEVICE_EVENT_BECOMING_READY {
    pub Version: u32,
    pub Reason: u32,
    pub Estimated100msToReady: u32,
}
impl ::core::marker::Copy for DEVICE_EVENT_BECOMING_READY {}
impl ::core::clone::Clone for DEVICE_EVENT_BECOMING_READY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEVICE_EVENT_EXTERNAL_REQUEST {
    pub Version: u32,
    pub DeviceClass: u32,
    pub ButtonStatus: u16,
    pub Request: u16,
    pub SystemTime: i64,
}
impl ::core::marker::Copy for DEVICE_EVENT_EXTERNAL_REQUEST {}
impl ::core::clone::Clone for DEVICE_EVENT_EXTERNAL_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEVICE_EVENT_GENERIC_DATA {
    pub EventNumber: u32,
}
impl ::core::marker::Copy for DEVICE_EVENT_GENERIC_DATA {}
impl ::core::clone::Clone for DEVICE_EVENT_GENERIC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEVICE_EVENT_MOUNT {
    pub Version: u32,
    pub Flags: u32,
    pub FileSystemNameLength: u32,
    pub FileSystemNameOffset: u32,
}
impl ::core::marker::Copy for DEVICE_EVENT_MOUNT {}
impl ::core::clone::Clone for DEVICE_EVENT_MOUNT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEVICE_EVENT_RBC_DATA {
    pub EventNumber: u32,
    pub SenseQualifier: u8,
    pub SenseCode: u8,
    pub SenseKey: u8,
    pub Reserved: u8,
    pub Information: u32,
}
impl ::core::marker::Copy for DEVICE_EVENT_RBC_DATA {}
impl ::core::clone::Clone for DEVICE_EVENT_RBC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEV_BROADCAST_DEVICEINTERFACE_A {
    pub dbcc_size: u32,
    pub dbcc_devicetype: u32,
    pub dbcc_reserved: u32,
    pub dbcc_classguid: ::windows_sys::core::GUID,
    pub dbcc_name: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEV_BROADCAST_DEVICEINTERFACE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEV_BROADCAST_DEVICEINTERFACE_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_DEVICEINTERFACE_W {
    pub dbcc_size: u32,
    pub dbcc_devicetype: u32,
    pub dbcc_reserved: u32,
    pub dbcc_classguid: ::windows_sys::core::GUID,
    pub dbcc_name: [u16; 1],
}
impl ::core::marker::Copy for DEV_BROADCAST_DEVICEINTERFACE_W {}
impl ::core::clone::Clone for DEV_BROADCAST_DEVICEINTERFACE_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_DEVNODE {
    pub dbcd_size: u32,
    pub dbcd_devicetype: u32,
    pub dbcd_reserved: u32,
    pub dbcd_devnode: u32,
}
impl ::core::marker::Copy for DEV_BROADCAST_DEVNODE {}
impl ::core::clone::Clone for DEV_BROADCAST_DEVNODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEV_BROADCAST_HANDLE {
    pub dbch_size: u32,
    pub dbch_devicetype: u32,
    pub dbch_reserved: u32,
    pub dbch_handle: super::super::Foundation::HANDLE,
    pub dbch_hdevnotify: *mut ::core::ffi::c_void,
    pub dbch_eventguid: ::windows_sys::core::GUID,
    pub dbch_nameoffset: i32,
    pub dbch_data: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEV_BROADCAST_HANDLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEV_BROADCAST_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_HANDLE32 {
    pub dbch_size: u32,
    pub dbch_devicetype: u32,
    pub dbch_reserved: u32,
    pub dbch_handle: u32,
    pub dbch_hdevnotify: u32,
    pub dbch_eventguid: ::windows_sys::core::GUID,
    pub dbch_nameoffset: i32,
    pub dbch_data: [u8; 1],
}
impl ::core::marker::Copy for DEV_BROADCAST_HANDLE32 {}
impl ::core::clone::Clone for DEV_BROADCAST_HANDLE32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_HANDLE64 {
    pub dbch_size: u32,
    pub dbch_devicetype: u32,
    pub dbch_reserved: u32,
    pub dbch_handle: u64,
    pub dbch_hdevnotify: u64,
    pub dbch_eventguid: ::windows_sys::core::GUID,
    pub dbch_nameoffset: i32,
    pub dbch_data: [u8; 1],
}
impl ::core::marker::Copy for DEV_BROADCAST_HANDLE64 {}
impl ::core::clone::Clone for DEV_BROADCAST_HANDLE64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_HDR {
    pub dbch_size: u32,
    pub dbch_devicetype: DEV_BROADCAST_HDR_DEVICE_TYPE,
    pub dbch_reserved: u32,
}
impl ::core::marker::Copy for DEV_BROADCAST_HDR {}
impl ::core::clone::Clone for DEV_BROADCAST_HDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type DEV_BROADCAST_HDR_DEVICE_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_DEVICEINTERFACE: DEV_BROADCAST_HDR_DEVICE_TYPE = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_HANDLE: DEV_BROADCAST_HDR_DEVICE_TYPE = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_OEM: DEV_BROADCAST_HDR_DEVICE_TYPE = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_PORT: DEV_BROADCAST_HDR_DEVICE_TYPE = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBT_DEVTYP_VOLUME: DEV_BROADCAST_HDR_DEVICE_TYPE = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_NET {
    pub dbcn_size: u32,
    pub dbcn_devicetype: u32,
    pub dbcn_reserved: u32,
    pub dbcn_resource: u32,
    pub dbcn_flags: u32,
}
impl ::core::marker::Copy for DEV_BROADCAST_NET {}
impl ::core::clone::Clone for DEV_BROADCAST_NET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_OEM {
    pub dbco_size: u32,
    pub dbco_devicetype: u32,
    pub dbco_reserved: u32,
    pub dbco_identifier: u32,
    pub dbco_suppfunc: u32,
}
impl ::core::marker::Copy for DEV_BROADCAST_OEM {}
impl ::core::clone::Clone for DEV_BROADCAST_OEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DEV_BROADCAST_PORT_A {
    pub dbcp_size: u32,
    pub dbcp_devicetype: u32,
    pub dbcp_reserved: u32,
    pub dbcp_name: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DEV_BROADCAST_PORT_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DEV_BROADCAST_PORT_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_PORT_W {
    pub dbcp_size: u32,
    pub dbcp_devicetype: u32,
    pub dbcp_reserved: u32,
    pub dbcp_name: [u16; 1],
}
impl ::core::marker::Copy for DEV_BROADCAST_PORT_W {}
impl ::core::clone::Clone for DEV_BROADCAST_PORT_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DEV_BROADCAST_VOLUME {
    pub dbcv_size: u32,
    pub dbcv_devicetype: u32,
    pub dbcv_reserved: u32,
    pub dbcv_unitmask: u32,
    pub dbcv_flags: DEV_BROADCAST_VOLUME_FLAGS,
}
impl ::core::marker::Copy for DEV_BROADCAST_VOLUME {}
impl ::core::clone::Clone for DEV_BROADCAST_VOLUME {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type DEV_BROADCAST_VOLUME_FLAGS = u16;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBTF_MEDIA: DEV_BROADCAST_VOLUME_FLAGS = 1u16;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DBTF_NET: DEV_BROADCAST_VOLUME_FLAGS = 2u16;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DIAGNOSTIC_REASON_DETAILED_STRING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DIAGNOSTIC_REASON_NOT_SPECIFIED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DIAGNOSTIC_REASON_SIMPLE_STRING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DIAGNOSTIC_REASON_VERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DIRECT3D_VERSION: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DISCHARGE_POLICY_CRITICAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DISCHARGE_POLICY_LOW: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DISK_HEALTH_NOTIFICATION_DATA {
    pub DeviceGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for DISK_HEALTH_NOTIFICATION_DATA {}
impl ::core::clone::Clone for DISK_HEALTH_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    pub Buffer: [u8; 152],
    pub Anonymous: DISPATCHER_CONTEXT_NONVOLREG_ARM64_0,
}
impl ::core::marker::Copy for DISPATCHER_CONTEXT_NONVOLREG_ARM64 {}
impl ::core::clone::Clone for DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {
    pub GpNvRegs: [u64; 11],
    pub FpNvRegs: [f64; 8],
}
impl ::core::marker::Copy for DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {}
impl ::core::clone::Clone for DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DLL_PROCESS_ATTACH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DLL_PROCESS_DETACH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DLL_THREAD_ATTACH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DLL_THREAD_DETACH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_ACCESS_CONTROL_ASSISTANCE_OPS: i32 = 579i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_ACCOUNT_OPS: i32 = 548i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_ADMINS: i32 = 544i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_AUTHORIZATIONACCESS: i32 = 560i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_BACKUP_OPS: i32 = 551i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_CACHEABLE_PRINCIPALS_GROUP: i32 = 571i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_CERTSVC_DCOM_ACCESS_GROUP: i32 = 574i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_CRYPTO_OPERATORS: i32 = 569i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_DCOM_USERS: i32 = 562i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT: i32 = 581i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_DEVICE_OWNERS: i32 = 583i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_EVENT_LOG_READERS_GROUP: i32 = 573i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_GUESTS: i32 = 546i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_HYPER_V_ADMINS: i32 = 578i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_INCOMING_FOREST_TRUST_BUILDERS: i32 = 557i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_IUSERS: i32 = 568i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_LOGGING_USERS: i32 = 559i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_MONITORING_USERS: i32 = 558i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_NETWORK_CONFIGURATION_OPS: i32 = 556i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_NON_CACHEABLE_PRINCIPALS_GROUP: i32 = 572i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_POWER_USERS: i32 = 547i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_PREW2KCOMPACCESS: i32 = 554i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_PRINT_OPS: i32 = 550i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_RAS_SERVERS: i32 = 553i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_RDS_ENDPOINT_SERVERS: i32 = 576i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_RDS_MANAGEMENT_SERVERS: i32 = 577i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_RDS_REMOTE_ACCESS_SERVERS: i32 = 575i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_REMOTE_DESKTOP_USERS: i32 = 555i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_REMOTE_MANAGEMENT_USERS: i32 = 580i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_REPLICATOR: i32 = 552i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_STORAGE_REPLICA_ADMINS: i32 = 582i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_SYSTEM_OPS: i32 = 549i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_TS_LICENSE_SERVERS: i32 = 561i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_ALIAS_RID_USERS: i32 = 545i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_ADMINS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_CONTAINS_CLAIMS: i32 = 497i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_IS_COMPOUNDED: i32 = 496i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_CDC_RESERVED: i32 = 524i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_CERT_ADMINS: i32 = 517i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_CLONEABLE_CONTROLLERS: i32 = 522i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_COMPUTERS: i32 = 515i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_CONTROLLERS: i32 = 516i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_ENTERPRISE_ADMINS: i32 = 519i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_ENTERPRISE_KEY_ADMINS: i32 = 527i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS: i32 = 498i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_GUESTS: i32 = 514i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_KEY_ADMINS: i32 = 526i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_POLICY_ADMINS: i32 = 520i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_PROTECTED_USERS: i32 = 525i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_READONLY_CONTROLLERS: i32 = 521i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_SCHEMA_ADMINS: i32 = 518i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_GROUP_RID_USERS: i32 = 513i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_USER_RID_ADMIN: i32 = 500i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_USER_RID_DEFAULT_ACCOUNT: i32 = 503i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_USER_RID_GUEST: i32 = 501i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_USER_RID_KRBTGT: i32 = 502i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_USER_RID_MAX: i32 = 999i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DOMAIN_USER_RID_WDAG_ACCOUNT: i32 = 504i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DP2BLT_LINEAR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DP2BLT_POINT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DX9_DDI_VERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DYNAMIC_EH_CONTINUATION_TARGET_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DYNAMIC_EH_CONTINUATION_TARGET_PROCESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DYNAMIC_ENFORCED_ADDRESS_RANGE_ADD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const DYNAMIC_ENFORCED_ADDRESS_RANGE_PROCESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IC_INST_WORD_POS_X: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IC_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IC_SIZE_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IC_VAL_POS_X: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_POS_X: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41a_SIZE_X: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41a_VAL_POS_X: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_POS_X: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41b_SIZE_X: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41b_VAL_POS_X: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_X: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41c_SIZE_X: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM41c_VAL_POS_X: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM5C_SIZE_X: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM5C_VAL_POS_X: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM7B_SIZE_X: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM7B_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM9D_SIZE_X: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_IMM9D_VAL_POS_X: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_SIGN_INST_WORD_POS_X: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_SIGN_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_SIGN_SIZE_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EMARCH_ENC_I17_SIGN_VAL_POS_X: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_LONG_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_SHORT_ID_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_TYPE_SGX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_TYPE_SGX2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_TYPE_VBS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_TYPE_VBS_BASIC: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENCLAVE_VBS_FLAG_DEBUG: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct ENLISTMENT_BASIC_INFORMATION {
    pub EnlistmentId: ::windows_sys::core::GUID,
    pub TransactionId: ::windows_sys::core::GUID,
    pub ResourceManagerId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for ENLISTMENT_BASIC_INFORMATION {}
impl ::core::clone::Clone for ENLISTMENT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct ENLISTMENT_CRM_INFORMATION {
    pub CrmTransactionManagerId: ::windows_sys::core::GUID,
    pub CrmResourceManagerId: ::windows_sys::core::GUID,
    pub CrmEnlistmentId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for ENLISTMENT_CRM_INFORMATION {}
impl ::core::clone::Clone for ENLISTMENT_CRM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ENLISTMENT_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EnlistmentBasicInformation: ENLISTMENT_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EnlistmentRecoveryInformation: ENLISTMENT_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EnlistmentCrmInformation: ENLISTMENT_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENLISTMENT_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENLISTMENT_RECOVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENLISTMENT_SET_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENLISTMENT_SUBORDINATE_RIGHTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ENLISTMENT_SUPERIOR_RIGHTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ERROR_SEVERITY_ERROR: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ERROR_SEVERITY_INFORMATIONAL: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ERROR_SEVERITY_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ERROR_SEVERITY_WARNING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_BACKWARDS_READ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_END_ALL_PAIRED_EVENTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_END_PAIRED_EVENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_FORWARDS_READ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_PAIRED_EVENT_ACTIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_PAIRED_EVENT_INACTIVE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENTLOG_START_PAIRED_EVENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EVENT_MODIFY_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_COLLIDED_UNWIND: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_EXECUTE_FAULT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_EXIT_UNWIND: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_MAXIMUM_PARAMETERS: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_NESTED_CALL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_NONCONTINUABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_READ_FAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_SOFTWARE_ORIGINATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_STACK_INVALID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_TARGET_UNWIND: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_UNWINDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const EXCEPTION_WRITE_FAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FACILITY_MCA_ERROR_CODE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_ADMINLESS_ACCESS_DENIED: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_APCS_DISABLED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_CAST_GUARD: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_CERTIFICATION_FAILURE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_CONTROL_INVALID_RETURN_ADDRESS: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_CORRUPT_LIST_ENTRY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_CRYPTO_LIBRARY: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_DEPRECATED_SERVICE_INVOKED: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_DLOAD_PROTECTION_FAILURE: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_ENCLAVE_CALL_FAILURE: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_ETW_CORRUPTION: u32 = 61u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_FATAL_APP_EXIT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_FLAGS_CORRUPTION: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GS_COOKIE_INIT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_EXPORT_SUPPRESSION_FAILURE: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE_XFG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_ICALL_CHECK_SUPPRESSED: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_JUMPTABLE: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_SS_FAILURE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_GUARD_WRITE_CHECK_FAILURE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_HEAP_METADATA_CORRUPTION: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_HOST_VISIBILITY_CHANGE: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INCORRECT_STACK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_ARG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_BALANCED_TREE: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_BUFFER_ACCESS: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_CONTROL_STACK: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_DISPATCH_CONTEXT: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_EXCEPTION_CHAIN: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_FAST_FAIL_CODE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_FIBER_SWITCH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_FILE_OPERATION: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_FLS_DATA: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_IAT: u32 = 49u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_IDLE_STATE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_IMAGE_BASE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_JUMP_BUFFER: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_LOCK_STATE: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_LONGJUMP_TARGET: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_NEXT_THREAD: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_PFN: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_REFERENCE_COUNT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_SET_OF_CONTEXT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_SYSCALL_NUMBER: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_INVALID_THREAD: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_KERNEL_CET_SHADOW_STACK_ASSIST: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_LEGACY_GS_VIOLATION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_LOADER_CONTINUITY_FAILURE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_LOW_LABEL_ACCESS_DENIED: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_LPAC_ACCESS_DENIED: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_MRDATA_MODIFIED: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_MRDATA_PROTECTION_FAILURE: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_NTDLL_PATCH_FAILED: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_PATCH_CALLBACK_FAILED: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_PAYLOAD_RESTRICTION_VIOLATION: u32 = 51u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_RANGE_CHECK_FAILURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_RIO_ABORT: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_SET_CONTEXT_DENIED: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_STACK_COOKIE_CHECK_FAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_UNEXPECTED_CALL: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_UNEXPECTED_HEAP_EXCEPTION: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_UNEXPECTED_HOST_BEHAVIOR: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_UNHANDLED_LSS_EXCEPTON: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_UNSAFE_EXTENSION_CALL: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_UNSAFE_REGISTRY_ACCESS: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_VEH_CORRUPTION: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FAST_FAIL_VTGUARD_CHECK_FAILURE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_ATTRIBUTE_STRICTLY_SEQUENTIAL: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_CASE_PRESERVED_NAMES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_CASE_SENSITIVE_SEARCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_CS_FLAG_CASE_SENSITIVE_DIR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_DAX_VOLUME: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_FILE_COMPRESSION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_NAMED_STREAMS: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_PERSISTENT_ACLS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_READ_ONLY_VOLUME: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_RETURNS_CLEANUP_RESULT_INFO: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SEQUENTIAL_WRITE_ONCE: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_BLOCK_REFCOUNTING: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_BYPASS_IO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_ENCRYPTION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_GHOSTING: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_HARD_LINKS: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_INTEGRITY_STREAMS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_OBJECT_IDS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_POSIX_UNLINK_RENAME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_REMOTE_STORAGE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_REPARSE_POINTS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_SPARSE_FILES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_SPARSE_VDL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_TRANSACTIONS: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_SUPPORTS_USN_JOURNAL: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_UNICODE_ON_DISK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_VOLUME_IS_COMPRESSED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILE_VOLUME_QUOTAS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILL_NV_MEMORY_FLAG_FLUSH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILL_NV_MEMORY_FLAG_NON_TEMPORAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FILL_NV_MEMORY_FLAG_NO_DRAIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FLS_MAXIMUM_AVAILABLE: u32 = 4080u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FLUSH_FLAGS_FILE_DATA_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FLUSH_FLAGS_FILE_DATA_SYNC_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FLUSH_FLAGS_NO_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FLUSH_NV_MEMORY_IN_FLAG_NO_DRAIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FOREST_USER_RID_MAX: i32 = 499i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FRAME_FPO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FRAME_NONFPO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FRAME_TRAP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const FRAME_TSS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct GDI_NONREMOTE {
    pub fContext: i32,
    pub u: GDI_NONREMOTE_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for GDI_NONREMOTE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for GDI_NONREMOTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union GDI_NONREMOTE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::DWORD_BLOB,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for GDI_NONREMOTE_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for GDI_NONREMOTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GENERIC_ALL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GENERIC_EXECUTE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GENERIC_READ: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GENERIC_WRITE: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type GESTURECONFIG_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_ALLGESTURES: GESTURECONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_ZOOM: GESTURECONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_PAN: GESTURECONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_PAN_WITH_SINGLE_FINGER_VERTICALLY: GESTURECONFIG_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_PAN_WITH_SINGLE_FINGER_HORIZONTALLY: GESTURECONFIG_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_PAN_WITH_GUTTER: GESTURECONFIG_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_PAN_WITH_INERTIA: GESTURECONFIG_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_ROTATE: GESTURECONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_TWOFINGERTAP: GESTURECONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_PRESSANDTAP: GESTURECONFIG_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const GC_ROLLOVER: GESTURECONFIG_FLAGS = 1u32;
pub const GUID_ACDC_POWER_SOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1564383833, data2: 59861, data3: 19200, data4: [166, 189, 255, 52, 255, 81, 101, 72] };
pub const GUID_ACTIVE_POWERSCHEME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 838464134, data2: 20612, data3: 17150, data4: [183, 32, 43, 2, 100, 153, 55, 99] };
pub const GUID_ADAPTIVE_INPUT_CONTROLLER_STATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 244906729, data2: 62554, data3: 19937, data4: [167, 87, 96, 49, 241, 151, 246, 234] };
pub const GUID_ADAPTIVE_POWER_BEHAVIOR_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2249832726, data2: 57348, data3: 19928, data4: [155, 102, 218, 232, 111, 128, 102, 152] };
pub const GUID_ADVANCED_COLOR_QUALITY_BIAS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1749827177, data2: 42231, data3: 16404, data4: [135, 84, 212, 81, 121, 165, 97, 103] };
pub const GUID_ALLOW_AWAYMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 635412809, data2: 24017, data3: 18230, data4: [181, 171, 232, 163, 123, 91, 129, 135] };
pub const GUID_ALLOW_DISPLAY_REQUIRED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2848897242, data2: 52550, data3: 17659, data4: [169, 139, 2, 175, 105, 222, 70, 35] };
pub const GUID_ALLOW_RTC_WAKE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3174789514, data2: 1664, data3: 19869, data4: [138, 178, 225, 210, 180, 172, 128, 109] };
pub const GUID_ALLOW_STANDBY_STATES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2885428505, data2: 13832, data3: 19498, data4: [148, 234, 23, 27, 14, 213, 70, 171] };
pub const GUID_ALLOW_SYSTEM_REQUIRED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2763101685, data2: 33317, data3: 18392, data4: [128, 18, 157, 65, 54, 151, 134, 226] };
pub const GUID_APPLAUNCH_BUTTON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 443060785, data2: 29593, data3: 20122, data4: [143, 153, 183, 31, 153, 157, 179, 250] };
pub const GUID_BACKGROUND_TASK_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3475239488, data2: 10836, data3: 18648, data4: [177, 20, 222, 21, 24, 255, 5, 46] };
pub const GUID_BATTERY_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099658517, data2: 64676, data3: 18917, data4: [133, 75, 169, 242, 191, 189, 92, 36] };
pub const GUID_BATTERY_DISCHARGE_ACTION_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1669242927, data2: 48075, data3: 16405, data4: [142, 44, 161, 199, 185, 192, 181, 70] };
pub const GUID_BATTERY_DISCHARGE_ACTION_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3631492555, data2: 15978, data3: 19260, data4: [179, 254, 55, 70, 35, 205, 207, 6] };
pub const GUID_BATTERY_DISCHARGE_ACTION_2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1109178936, data2: 6798, data3: 18561, data4: [172, 137, 227, 58, 139, 4, 236, 228] };
pub const GUID_BATTERY_DISCHARGE_ACTION_3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2152146451, data2: 38784, data3: 17758, data4: [179, 8, 114, 211, 0, 60, 242, 248] };
pub const GUID_BATTERY_DISCHARGE_FLAGS_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1572568223, data2: 14569, data3: 16594, data4: [151, 73, 79, 138, 14, 159, 100, 15] };
pub const GUID_BATTERY_DISCHARGE_FLAGS_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3168721233, data2: 6267, data3: 19717, data4: [188, 204, 247, 229, 25, 96, 194, 88] };
pub const GUID_BATTERY_DISCHARGE_FLAGS_2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2144530628, data2: 65207, data3: 19875, data4: [129, 23, 227, 251, 237, 196, 101, 130] };
pub const GUID_BATTERY_DISCHARGE_FLAGS_3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935752399, data2: 56314, data3: 17017, data4: [131, 86, 73, 53, 246, 191, 98, 243] };
pub const GUID_BATTERY_DISCHARGE_LEVEL_0: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2590431447, data2: 20471, data3: 20217, data4: [181, 162, 90, 50, 108, 162, 164, 105] };
pub const GUID_BATTERY_DISCHARGE_LEVEL_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2172893850, data2: 59664, data3: 18650, data4: [135, 105, 20, 174, 109, 193, 23, 10] };
pub const GUID_BATTERY_DISCHARGE_LEVEL_2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 127958178, data2: 44463, data3: 16599, data4: [176, 119, 83, 58, 173, 237, 27, 250] };
pub const GUID_BATTERY_DISCHARGE_LEVEL_3: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1487918502, data2: 49885, data3: 18386, data4: [159, 191, 239, 112, 204, 92, 89, 101] };
pub const GUID_BATTERY_PERCENTAGE_REMAINING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2813165633, data2: 46170, data3: 19630, data4: [135, 163, 238, 203, 180, 104, 169, 225] };
pub const GUID_BATTERY_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3879339149, data2: 48935, data3: 20242, data4: [151, 49, 139, 32, 118, 232, 137, 31] };
pub const GUID_CONNECTIVITY_IN_STANDBY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4048910056, data2: 39095, data3: 16774, data4: [185, 68, 234, 250, 102, 68, 2, 217] };
pub const GUID_CONSOLE_DISPLAY_STATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1877382486, data2: 28746, data3: 18336, data4: [143, 36, 194, 141, 147, 111, 218, 71] };
pub const GUID_CRITICAL_POWER_TRANSITION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3080876069, data2: 58729, data3: 18114, data4: [165, 4, 43, 150, 202, 210, 37, 161] };
pub const GUID_DEEP_SLEEP_ENABLED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3573741550, data2: 7623, data3: 20221, data4: [165, 93, 240, 75, 111, 92, 5, 69] };
pub const GUID_DEEP_SLEEP_PLATFORM_STATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3527356344, data2: 38198, data3: 16440, data4: [156, 148, 28, 224, 46, 92, 33, 82] };
pub const GUID_DEVICE_EVENT_RBC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497281426, data2: 43406, data3: 4562, data4: [145, 122, 0, 160, 201, 6, 143, 243] };
pub const GUID_DEVICE_IDLE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1336588058, data2: 37605, data3: 18214, data4: [181, 49, 34, 69, 89, 103, 45, 25] };
pub const GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2918014594, data2: 47369, data3: 17945, data4: [153, 73, 245, 215, 29, 172, 11, 203] };
pub const GUID_DEVICE_POWER_POLICY_VIDEO_DIM_BRIGHTNESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4059823586, data2: 43360, data3: 16741, data4: [159, 136, 80, 102, 121, 17, 206, 150] };
pub const GUID_DEVINTERFACE_DMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 632611432, data2: 10757, data3: 18798, data4: [128, 59, 38, 104, 55, 251, 218, 75] };
pub const GUID_DEVINTERFACE_DMR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3498532788, data2: 8598, data3: 19578, data4: [166, 61, 228, 22, 173, 221, 96, 161] };
pub const GUID_DEVINTERFACE_DMS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3378526126, data2: 42328, data3: 17520, data4: [180, 50, 17, 90, 49, 184, 85, 83] };
pub const GUID_DISCONNECTED_STANDBY_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1756345049, data2: 61077, data3: 18344, data4: [143, 80, 65, 21, 8, 128, 115, 177] };
pub const GUID_DISK_ADAPTIVE_POWERDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 963261153, data2: 18842, data3: 16562, data4: [145, 36, 169, 106, 254, 112, 118, 103] };
pub const GUID_DISK_BURST_IGNORE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2162411022, data2: 48020, data3: 19160, data4: [187, 224, 13, 49, 149, 239, 198, 99] };
pub const GUID_DISK_COALESCING_POWERDOWN_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3278835380, data2: 10632, data3: 19056, data4: [142, 238, 8, 132, 252, 44, 36, 51] };
pub const GUID_DISK_IDLE_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1491311528, data2: 47334, data3: 20214, data4: [144, 208, 137, 174, 50, 178, 88, 214] };
pub const GUID_DISK_MAX_POWER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1373545808, data2: 47928, data3: 19396, data4: [153, 27, 234, 207, 55, 190, 94, 200] };
pub const GUID_DISK_NVME_NOPPME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4235424438, data2: 43821, data3: 17390, data4: [135, 151, 21, 233, 132, 31, 44, 202] };
pub const GUID_DISK_POWERDOWN_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1731781316, data2: 59557, data3: 19010, data4: [177, 106, 224, 64, 231, 105, 117, 110] };
pub const GUID_DISK_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1240647, data2: 36929, data3: 19293, data4: [155, 119, 83, 95, 186, 139, 20, 66] };
pub const GUID_ENABLE_SWITCH_FORCED_SHUTDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2201643874, data2: 57252, data3: 18129, data4: [130, 248, 224, 158, 52, 208, 41, 214] };
pub const GUID_ENERGY_SAVER_BATTERY_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3868611530, data2: 53119, data3: 20229, data4: [170, 115, 203, 131, 63, 169, 10, 212] };
pub const GUID_ENERGY_SAVER_BRIGHTNESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 332437636, data2: 63310, data3: 18250, data4: [168, 82, 182, 189, 232, 173, 3, 168] };
pub const GUID_ENERGY_SAVER_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1549513545, data2: 44329, data3: 20194, data4: [157, 11, 43, 37, 39, 15, 122, 129] };
pub const GUID_ENERGY_SAVER_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3733129507, data2: 42338, data3: 16815, data4: [160, 134, 227, 162, 198, 186, 210, 218] };
pub const GUID_EXECUTION_REQUIRED_REQUEST_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 828816449, data2: 32408, data3: 19971, data4: [179, 78, 236, 15, 95, 43, 33, 142] };
pub const GUID_GLOBAL_USER_PRESENCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2020510237, data2: 46119, data3: 17220, data4: [146, 7, 9, 231, 11, 220, 190, 169] };
pub const GUID_GPU_PREFERENCE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3716451114, data2: 35421, data3: 17489, data4: [154, 226, 57, 205, 65, 101, 143, 108] };
pub const GUID_GRAPHICS_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1605669773, data2: 7912, data3: 19215, data4: [154, 60, 80, 54, 176, 171, 153, 92] };
pub const GUID_HIBERNATE_FASTS4_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2494328105, data2: 29646, data3: 16806, data4: [128, 159, 99, 99, 186, 33, 180, 126] };
pub const GUID_HIBERNATE_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2641892774, data2: 32484, data3: 18814, data4: [136, 136, 81, 90, 5, 240, 35, 100] };
pub const GUID_HUPR_ADAPTIVE_DISPLAY_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 175991478, data2: 44163, data3: 19153, data4: [130, 130, 236, 165, 181, 131, 8, 243] };
pub const GUID_IDLE_BACKGROUND_TASK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1364996568, data2: 63284, data3: 5693, data4: [160, 253, 17, 160, 140, 145, 232, 241] };
pub const GUID_IDLE_RESILIENCY_PERIOD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3291183530, data2: 43578, data3: 18507, data4: [169, 143, 44, 243, 42, 169, 10, 40] };
pub const GUID_IDLE_RESILIENCY_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 778047792, data2: 21329, data3: 19869, data4: [142, 4, 37, 41, 102, 186, 208, 84] };
pub const GUID_INTSTEER_LOAD_PER_PROC_TRIGGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1942873677, data2: 55072, data3: 19378, data4: [168, 96, 199, 85, 175, 231, 126, 242] };
pub const GUID_INTSTEER_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 737944825, data2: 24226, data3: 18433, data4: [130, 19, 61, 186, 224, 26, 163, 157] };
pub const GUID_INTSTEER_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1214721848, data2: 31386, data3: 19378, data4: [139, 248, 61, 133, 190, 25, 222, 78] };
pub const GUID_INTSTEER_TIME_UNPARK_TRIGGER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3602532611, data2: 14447, data3: 19500, data4: [138, 219, 92, 33, 179, 50, 141, 37] };
pub const GUID_IO_CDROM_EXCLUSIVE_LOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159802169, data2: 31248, data3: 18414, data4: [162, 148, 76, 106, 56, 240, 20, 154] };
pub const GUID_IO_CDROM_EXCLUSIVE_UNLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2746667645, data2: 24117, data3: 18565, data4: [129, 229, 238, 24, 192, 14, 215, 121] };
pub const GUID_IO_DEVICE_BECOMING_READY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497276400, data2: 43406, data3: 4562, data4: [145, 122, 0, 160, 201, 6, 143, 243] };
pub const GUID_IO_DEVICE_EXTERNAL_REQUEST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497276368, data2: 43406, data3: 4562, data4: [145, 122, 0, 160, 201, 6, 143, 243] };
pub const GUID_IO_DISK_CLONE_ARRIVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1784776795, data2: 31801, data3: 17373, data4: [155, 86, 184, 172, 34, 165, 73, 170] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    pub DiskNumber: u32,
}
impl ::core::marker::Copy for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {}
impl ::core::clone::Clone for GUID_IO_DISK_CLONE_ARRIVAL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GUID_IO_DISK_HEALTH_NOTIFICATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 253482564, data2: 14614, data3: 18885, data4: [176, 99, 153, 25, 64, 17, 143, 178] };
pub const GUID_IO_DISK_LAYOUT_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 299889996, data2: 33897, data3: 16889, data4: [179, 222, 239, 131, 100, 135, 197, 74] };
pub const GUID_IO_DRIVE_REQUIRES_CLEANING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1913096060, data2: 37101, data3: 17637, data4: [160, 0, 129, 66, 141, 76, 121, 187] };
pub const GUID_IO_MEDIA_ARRIVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497276352, data2: 43406, data3: 4562, data4: [145, 122, 0, 160, 201, 6, 143, 243] };
pub const GUID_IO_MEDIA_EJECT_REQUEST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497276369, data2: 43406, data3: 4562, data4: [145, 122, 0, 160, 201, 6, 143, 243] };
pub const GUID_IO_MEDIA_REMOVAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3497276353, data2: 43406, data3: 4562, data4: [145, 122, 0, 160, 201, 6, 143, 243] };
pub const GUID_IO_TAPE_ERASE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2234323435, data2: 19384, data3: 17671, data4: [157, 155, 65, 124, 194, 177, 180, 56] };
pub const GUID_IO_VOLUME_BACKGROUND_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2732981382, data2: 54733, data3: 16440, data4: [178, 227, 68, 69, 6, 92, 35, 119] };
pub const GUID_IO_VOLUME_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1936942410, data2: 33066, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] };
pub const GUID_IO_VOLUME_CHANGE_SIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 974529982, data2: 44291, data3: 18929, data4: [142, 248, 107, 186, 193, 130, 209, 253] };
pub const GUID_IO_VOLUME_DEVICE_INTERFACE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1408590605, data2: 46783, data3: 4560, data4: [148, 242, 0, 160, 201, 30, 251, 139] };
pub const GUID_IO_VOLUME_DISMOUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3513406952, data2: 4185, data3: 4562, data4: [143, 253, 0, 160, 201, 160, 109, 50] };
pub const GUID_IO_VOLUME_DISMOUNT_FAILED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3821384056, data2: 4189, data3: 4562, data4: [143, 253, 0, 160, 201, 160, 109, 50] };
pub const GUID_IO_VOLUME_FORCE_CLOSED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1092278351, data2: 17214, data3: 19906, data4: [165, 174, 74, 45, 26, 45, 230, 84] };
pub const GUID_IO_VOLUME_FVE_STATUS_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 103389362, data2: 60959, data3: 19306, data4: [184, 87, 231, 108, 187, 233, 166, 218] };
pub const GUID_IO_VOLUME_INFO_MAKE_COMPAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 985243858, data2: 61312, data3: 17871, data4: [140, 220, 203, 224, 42, 33, 41, 6] };
pub const GUID_IO_VOLUME_LOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1349552244, data2: 51631, data3: 4561, data4: [143, 239, 0, 160, 201, 160, 109, 50] };
pub const GUID_IO_VOLUME_LOCK_FAILED: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2922310928, data2: 2984, data3: 4562, data4: [143, 251, 0, 160, 201, 160, 109, 50] };
pub const GUID_IO_VOLUME_MOUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3045083256, data2: 6806, data3: 4562, data4: [143, 253, 0, 160, 201, 160, 109, 50] };
pub const GUID_IO_VOLUME_NAME_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 770277251, data2: 19462, data3: 4562, data4: [165, 50, 0, 96, 151, 19, 5, 90] };
pub const GUID_IO_VOLUME_NEED_CHKDSK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2040138080, data2: 2571, data3: 19971, data4: [173, 136, 47, 167, 198, 206, 116, 138] };
pub const GUID_IO_VOLUME_PHYSICAL_CONFIGURATION_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 770277252, data2: 19462, data3: 4562, data4: [165, 50, 0, 96, 151, 19, 5, 90] };
pub const GUID_IO_VOLUME_PREPARING_EJECT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3349066094, data2: 3500, data3: 20090, data4: [168, 108, 178, 92, 238, 170, 136, 246] };
pub const GUID_IO_VOLUME_UNIQUE_ID_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2939804226, data2: 26146, data3: 16885, data4: [151, 11, 19, 157, 9, 47, 163, 217] };
pub const GUID_IO_VOLUME_UNLOCK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2592882024, data2: 53451, data3: 4561, data4: [143, 239, 0, 160, 201, 160, 109, 50] };
pub const GUID_IO_VOLUME_WEARING_OUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2268140490, data2: 5254, data3: 17672, data4: [130, 172, 195, 178, 229, 41, 122, 170] };
pub const GUID_IO_VOLUME_WORM_NEAR_FULL: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4089446274, data2: 62430, data3: 18642, data4: [175, 149, 69, 127, 128, 183, 99, 242] };
pub const GUID_LEGACY_RTC_MITIGATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 439664067, data2: 32363, data3: 17454, data4: [169, 208, 100, 182, 239, 55, 142, 132] };
pub const GUID_LIDCLOSE_ACTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1554527079, data2: 28229, data3: 17823, data4: [162, 123, 71, 107, 29, 1, 201, 54] };
pub const GUID_LIDOPEN_POWERSTATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2583630055, data2: 9137, data3: 19463, data4: [169, 209, 92, 50, 6, 215, 65, 180] };
pub const GUID_LIDSWITCH_STATE_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3124629325, data2: 47127, data3: 16532, data4: [162, 209, 213, 99, 121, 230, 160, 243] };
pub const GUID_LIDSWITCH_STATE_RELIABILITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2924236785, data2: 54113, data3: 17396, data4: [128, 170, 187, 182, 235, 3, 222, 148] };
pub const GUID_LOCK_CONSOLE_ON_WAKE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 242838491, data2: 4109, data3: 18390, data4: [162, 213, 247, 210, 218, 165, 31, 81] };
pub const GUID_MAX_POWER_SAVINGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2709787400, data2: 13633, data3: 20395, data4: [188, 129, 247, 21, 86, 242, 11, 74] };
pub const GUID_MIN_POWER_SAVINGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2355003354, data2: 59583, data3: 19094, data4: [154, 133, 166, 226, 58, 140, 99, 92] };
pub const GUID_MIXED_REALITY_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 509766478, data2: 52996, data3: 20365, data4: [156, 199, 201, 124, 91, 15, 35, 145] };
pub const GUID_MONITOR_POWER_ON: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 41095189, data2: 17680, data3: 17702, data4: [153, 230, 229, 161, 126, 189, 26, 234] };
pub const GUID_NON_ADAPTIVE_INPUT_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1524350908, data2: 1870, data3: 19873, data4: [186, 56, 219, 139, 54, 178, 200, 243] };
pub const GUID_PCIEXPRESS_ASPM_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3994220806, data2: 53879, data3: 16459, data4: [182, 218, 229, 250, 26, 87, 109, 245] };
pub const GUID_PCIEXPRESS_SETTINGS_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1343900947, data2: 17071, data3: 17449, data4: [159, 209, 168, 33, 140, 38, 142, 32] };
pub const GUID_POWERBUTTON_ACTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1984491427, data2: 56732, data3: 20030, data4: [181, 102, 80, 249, 41, 56, 98, 128] };
pub const GUID_POWERSCHEME_PERSONALITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 610108737, data2: 14659, data3: 17442, data4: [176, 37, 19, 167, 132, 246, 121, 183] };
pub const GUID_POWER_SAVING_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3758708928, data2: 49683, data3: 19150, data4: [172, 119, 254, 204, 237, 46, 238, 165] };
pub const GUID_PROCESSOR_ALLOW_THROTTLING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 990172413, data2: 7367, data3: 20259, data4: [171, 28, 209, 51, 120, 25, 196, 187] };
pub const GUID_PROCESSOR_CLASS0_FLOOR_PERF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4259087403, data2: 33636, data3: 20188, data4: [148, 207, 193, 127, 96, 222, 28, 128] };
pub const GUID_PROCESSOR_CLASS1_INITIAL_PERF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 531430501, data2: 43312, data3: 19397, data4: [159, 56, 80, 78, 192, 151, 187, 192] };
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_DECREASE_FACTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2407220707, data2: 50067, data3: 18442, data4: [135, 140, 246, 122, 195, 208, 112, 130] };
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1530095995, data2: 59549, data3: 19768, data4: [170, 70, 158, 125, 251, 124, 210, 249] };
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_WEIGHTING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3876087793, data2: 64047, data3: 20302, data4: [174, 161, 77, 138, 11, 162, 59, 32] };
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1895963457, data2: 51017, data3: 19745, data4: [190, 116, 160, 15, 51, 93, 88, 43] };
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1759325991, data2: 42190, data3: 19985, data4: [132, 135, 55, 148, 228, 19, 93, 250] };
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3755019543, data2: 54763, data3: 17885, data4: [135, 122, 154, 52, 221, 209, 92, 130] };
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3351119481, data2: 10263, data3: 19817, data4: [157, 2, 81, 154, 83, 126, 208, 198] };
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3742640449, data2: 8435, data3: 20191, data4: [154, 74, 156, 131, 211, 215, 23, 209] };
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 769481348, data2: 23153, data3: 17278, data4: [145, 42, 219, 11, 140, 120, 135, 50] };
pub const GUID_PROCESSOR_CORE_PARKING_MAX_CORES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3926270001, data2: 3636, data3: 20465, data4: [155, 109, 235, 16, 89, 51, 64, 40] };
pub const GUID_PROCESSOR_CORE_PARKING_MAX_CORES_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3926270001, data2: 3636, data3: 20465, data4: [155, 109, 235, 16, 89, 51, 64, 41] };
pub const GUID_PROCESSOR_CORE_PARKING_MIN_CORES: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 214283847, data2: 49631, data3: 17975, data4: [137, 26, 222, 195, 92, 49, 133, 131] };
pub const GUID_PROCESSOR_CORE_PARKING_MIN_CORES_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 214283847, data2: 49631, data3: 17975, data4: [137, 26, 222, 195, 92, 49, 133, 132] };
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_DECREASE_FACTOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 312017468, data2: 48168, data3: 20234, data4: [129, 236, 211, 41, 90, 141, 129, 93] };
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2596376210, data2: 43580, data3: 20007, data4: [179, 7, 1, 174, 55, 48, 113, 41] };
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2486996150, data2: 28563, data3: 16935, data4: [173, 135, 233, 163, 254, 236, 8, 209] };
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_WEIGHTING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2282341080, data2: 45397, data3: 17108, data4: [188, 218, 13, 52, 86, 81, 177, 219] };
pub const GUID_PROCESSOR_DISTRIBUTE_UTILITY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3758125872, data2: 62857, data3: 17133, data4: [164, 1, 93, 219, 16, 231, 133, 211] };
pub const GUID_PROCESSOR_DUTY_CYCLING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1313099955, data2: 24953, data3: 20113, data4: [184, 241, 91, 185, 147, 143, 129, 161] };
pub const GUID_PROCESSOR_FREQUENCY_LIMIT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1974513215, data2: 48352, data3: 17831, data4: [140, 137, 201, 97, 28, 37, 225, 0] };
pub const GUID_PROCESSOR_FREQUENCY_LIMIT_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1974513215, data2: 48352, data3: 17831, data4: [140, 137, 201, 97, 28, 37, 225, 1] };
pub const GUID_PROCESSOR_HETEROGENEOUS_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2133810426, data2: 61708, data3: 18467, data4: [181, 225, 233, 58, 232, 95, 70, 181] };
pub const GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4169538599, data2: 38375, data3: 18268, data4: [134, 91, 19, 192, 203, 63, 157, 107] };
pub const GUID_PROCESSOR_HETERO_DECREASE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2133103286, data2: 24753, data3: 17893, data4: [174, 85, 119, 63, 140, 213, 202, 236] };
pub const GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2952804733, data2: 39691, data3: 18493, data4: [152, 201, 105, 42, 96, 96, 207, 191] };
pub const GUID_PROCESSOR_HETERO_INCREASE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1074392999, data2: 59181, data3: 19642, data4: [158, 223, 145, 8, 78, 168, 203, 195] };
pub const GUID_PROCESSOR_IDLESTATE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1760715431, data2: 63009, data3: 16489, data4: [185, 165, 72, 116, 22, 155, 226, 60] };
pub const GUID_PROCESSOR_IDLE_ALLOW_SCALING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1814664112, data2: 36680, data3: 18463, data4: [188, 198, 0, 221, 39, 66, 170, 6] };
pub const GUID_PROCESSOR_IDLE_DEMOTE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1267914584, data2: 23076, data3: 18513, data4: [164, 112, 129, 93, 120, 174, 225, 25] };
pub const GUID_PROCESSOR_IDLE_DISABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1568056010, data2: 59584, data3: 16431, data4: [161, 51, 33, 88, 73, 45, 88, 173] };
pub const GUID_PROCESSOR_IDLE_PROMOTE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2065844355, data2: 46028, data3: 19833, data4: [129, 159, 131, 116, 21, 44, 190, 124] };
pub const GUID_PROCESSOR_IDLE_STATE_MAXIMUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2571364613, data2: 39472, data3: 20161, data4: [155, 153, 68, 221, 59, 118, 247, 162] };
pub const GUID_PROCESSOR_IDLE_TIME_CHECK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3294108721, data2: 35243, data3: 17815, data4: [142, 43, 156, 156, 171, 68, 14, 107] };
pub const GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1634523813, data2: 26974, data3: 17733, data4: [151, 173, 151, 220, 45, 27, 221, 136] };
pub const GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1634523813, data2: 26974, data3: 17733, data4: [151, 173, 151, 220, 45, 27, 221, 137] };
pub const GUID_PROCESSOR_PARKING_CONCURRENCY_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 607169391, data2: 42272, data3: 17570, data4: [150, 1, 247, 242, 59, 81, 52, 177] };
pub const GUID_PROCESSOR_PARKING_CORE_OVERRIDE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2773881514, data2: 63012, data3: 17094, data4: [164, 67, 115, 151, 208, 100, 192, 79] };
pub const GUID_PROCESSOR_PARKING_DISTRIBUTION_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1272640745, data2: 53507, data3: 18135, data4: [165, 240, 98, 128, 18, 22, 22, 239] };
pub const GUID_PROCESSOR_PARKING_HEADROOM_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4147488371, data2: 8294, data3: 20352, data4: [160, 197, 221, 238, 12, 241, 191, 93] };
pub const GUID_PROCESSOR_PARKING_PERF_STATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1148335559, data2: 27277, data3: 19648, data4: [142, 36, 158, 175, 112, 185, 110, 43] };
pub const GUID_PROCESSOR_PARKING_PERF_STATE_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1148335559, data2: 27277, data3: 19648, data4: [142, 36, 158, 175, 112, 185, 110, 44] };
pub const GUID_PROCESSOR_PERFSTATE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3151771668, data2: 6377, data3: 17507, data4: [138, 85, 209, 151, 50, 124, 69, 192] };
pub const GUID_PROCESSOR_PERF_AUTONOMOUS_ACTIVITY_WINDOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3488457680, data2: 30359, data3: 17766, data4: [169, 34, 169, 8, 108, 212, 157, 250] };
pub const GUID_PROCESSOR_PERF_AUTONOMOUS_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2343193226, data2: 5318, data3: 17489, data4: [142, 139, 20, 189, 189, 25, 117, 55] };
pub const GUID_PROCESSOR_PERF_BOOST_MODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3191042616, data2: 3458, data3: 16710, data4: [169, 96, 79, 55, 73, 212, 112, 199] };
pub const GUID_PROCESSOR_PERF_BOOST_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1169997892, data2: 55429, data3: 17378, data4: [134, 5, 238, 14, 198, 233, 107, 89] };
pub const GUID_PROCESSOR_PERF_CORE_PARKING_HISTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2010641026, data2: 36634, data3: 17101, data4: [133, 55, 69, 69, 10, 131, 155, 232] };
pub const GUID_PROCESSOR_PERF_DECREASE_HISTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 50394872, data2: 43990, data3: 17833, data4: [183, 79, 73, 8, 105, 26, 64, 181] };
pub const GUID_PROCESSOR_PERF_DECREASE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1090252743, data2: 11933, data3: 19749, data4: [161, 133, 12, 253, 133, 116, 186, 198] };
pub const GUID_PROCESSOR_PERF_DECREASE_POLICY_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1090252743, data2: 11933, data3: 19749, data4: [161, 133, 12, 253, 133, 116, 186, 199] };
pub const GUID_PROCESSOR_PERF_DECREASE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 312519492, data2: 65064, data3: 20393, data4: [179, 189, 75, 100, 244, 73, 96, 166] };
pub const GUID_PROCESSOR_PERF_DECREASE_THRESHOLD_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 312519492, data2: 65064, data3: 20393, data4: [179, 189, 75, 100, 244, 73, 96, 167] };
pub const GUID_PROCESSOR_PERF_DECREASE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3639471003, data2: 38351, data3: 20373, data4: [167, 60, 176, 97, 151, 54, 147, 200] };
pub const GUID_PROCESSOR_PERF_DECREASE_TIME_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3639471003, data2: 38351, data3: 20373, data4: [167, 60, 176, 97, 151, 54, 147, 201] };
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 912818078, data2: 58277, data3: 19903, data4: [177, 220, 21, 235, 56, 28, 104, 99] };
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 912818078, data2: 58277, data3: 19903, data4: [177, 220, 21, 235, 56, 28, 104, 100] };
pub const GUID_PROCESSOR_PERF_HISTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099559079, data2: 2948, data3: 18447, data4: [132, 12, 27, 7, 67, 192, 15, 95] };
pub const GUID_PROCESSOR_PERF_HISTORY_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2099559079, data2: 2948, data3: 18447, data4: [132, 12, 27, 7, 67, 192, 15, 96] };
pub const GUID_PROCESSOR_PERF_INCREASE_HISTORY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2578706177, data2: 29999, data3: 18081, data4: [128, 251, 119, 48, 1, 31, 35, 84] };
pub const GUID_PROCESSOR_PERF_INCREASE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180573520, data2: 46608, data3: 18234, data4: [171, 88, 0, 209, 7, 125, 196, 24] };
pub const GUID_PROCESSOR_PERF_INCREASE_POLICY_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1180573520, data2: 46608, data3: 18234, data4: [171, 88, 0, 209, 7, 125, 196, 25] };
pub const GUID_PROCESSOR_PERF_INCREASE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 113958670, data2: 25837, data3: 17546, data4: [137, 39, 206, 123, 249, 14, 179, 93] };
pub const GUID_PROCESSOR_PERF_INCREASE_THRESHOLD_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 113958670, data2: 25837, data3: 17546, data4: [137, 39, 206, 123, 249, 14, 179, 94] };
pub const GUID_PROCESSOR_PERF_INCREASE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2555180178, data2: 15341, data3: 17544, data4: [168, 249, 66, 134, 201, 123, 245, 170] };
pub const GUID_PROCESSOR_PERF_INCREASE_TIME_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2555180178, data2: 15341, data3: 17544, data4: [168, 249, 66, 134, 201, 123, 245, 171] };
pub const GUID_PROCESSOR_PERF_LATENCY_HINT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 136503089, data2: 40067, data3: 17436, data4: [160, 121, 13, 228, 207, 0, 156, 123] };
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1637577989, data2: 59, data3: 20098, data4: [183, 166, 77, 210, 156, 48, 9, 113] };
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1637577989, data2: 59, data3: 20098, data4: [183, 166, 77, 210, 156, 48, 9, 114] };
pub const GUID_PROCESSOR_PERF_TIME_CHECK: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1294664018, data2: 32092, data3: 18827, data4: [136, 226, 52, 52, 83, 146, 162, 197] };
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 951597117, data2: 52448, data3: 19577, data4: [158, 62, 86, 164, 241, 124, 196, 128] };
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 951597117, data2: 52448, data3: 19577, data4: [158, 62, 86, 164, 241, 124, 196, 129] };
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4117076383, data2: 16304, data3: 16666, data4: [162, 38, 63, 1, 152, 222, 193, 48] };
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4117076383, data2: 16304, data3: 16666, data4: [162, 38, 63, 1, 152, 222, 193, 49] };
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1027924566, data2: 29218, data3: 17429, data4: [169, 237, 156, 69, 250, 61, 216, 48] };
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1027924566, data2: 29218, data3: 17429, data4: [169, 237, 156, 69, 250, 61, 216, 49] };
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1032933768, data2: 30768, data3: 18862, data4: [167, 154, 15, 176, 161, 229, 162, 0] };
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1032933768, data2: 30768, data3: 18862, data4: [167, 154, 15, 176, 161, 229, 162, 1] };
pub const GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1143457595, data2: 38742, data3: 19036, data4: [184, 75, 199, 189, 167, 156, 115, 32] };
pub const GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1143457595, data2: 38742, data3: 19036, data4: [184, 75, 199, 189, 167, 156, 115, 33] };
pub const GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3465450222, data2: 27270, data3: 17778, data4: [191, 224, 32, 194, 29, 3, 205, 64] };
pub const GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3465450222, data2: 27270, data3: 17778, data4: [191, 224, 32, 194, 29, 3, 205, 65] };
pub const GUID_PROCESSOR_SETTINGS_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1414738513, data2: 33470, data3: 18468, data4: [150, 193, 71, 182, 11, 116, 13, 0] };
pub const GUID_PROCESSOR_SHORT_THREAD_RUNTIME_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3643381954, data2: 27208, data3: 18890, data4: [133, 212, 140, 206, 236, 41, 69, 112] };
pub const GUID_PROCESSOR_SHORT_THREAD_SCHEDULING_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3135277953, data2: 11614, data3: 18056, data4: [173, 106, 19, 36, 51, 86, 101, 75] };
pub const GUID_PROCESSOR_SOFT_PARKING_LATENCY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2546969665, data2: 8727, data3: 18411, data4: [153, 45, 97, 139, 25, 119, 201, 7] };
pub const GUID_PROCESSOR_THREAD_SCHEDULING_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2478356188, data2: 1688, data3: 19740, data4: [158, 228, 6, 68, 233, 0, 200, 93] };
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159374071, data2: 9184, data3: 18784, data4: [150, 218, 51, 171, 175, 89, 53, 236] };
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3159374071, data2: 9184, data3: 18784, data4: [150, 218, 51, 171, 175, 89, 53, 237] };
pub const GUID_PROCESSOR_THROTTLE_MINIMUM: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2302537358, data2: 11247, data3: 16864, data4: [137, 198, 181, 93, 9, 41, 150, 76] };
pub const GUID_PROCESSOR_THROTTLE_MINIMUM_1: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2302537358, data2: 11247, data3: 16864, data4: [137, 198, 181, 93, 9, 41, 150, 77] };
pub const GUID_PROCESSOR_THROTTLE_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1459778308, data2: 19190, data3: 16644, data4: [146, 96, 227, 217, 82, 72, 252, 54] };
pub const GUID_SESSION_DISPLAY_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 730120718, data2: 44323, data3: 19935, data4: [147, 219, 5, 255, 189, 126, 252, 165] };
pub const GUID_SESSION_USER_PRESENCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1007633736, data2: 49215, data3: 19533, data4: [185, 242, 35, 126, 222, 104, 99, 118] };
pub const GUID_SLEEPBUTTON_ACTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2526636992, data2: 44368, data3: 18412, data4: [146, 59, 111, 65, 135, 77, 217, 235] };
pub const GUID_SLEEP_IDLE_THRESHOLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2177708768, data2: 30771, data3: 17651, data4: [135, 55, 112, 129, 243, 141, 31, 112] };
pub const GUID_SLEEP_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 596418472, data2: 2733, data3: 16877, data4: [131, 244, 151, 190, 36, 44, 143, 32] };
pub const GUID_SPR_ACTIVE_SESSION_CHANGE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 237293112, data2: 50067, data3: 18242, data4: [189, 177, 116, 79, 75, 158, 224, 142] };
pub const GUID_STANDBY_BUDGET_GRACE_PERIOD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1623228385, data2: 1366, data3: 17871, data4: [153, 3, 213, 110, 50, 33, 2, 66] };
pub const GUID_STANDBY_BUDGET_PERCENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2682595262, data2: 7024, data3: 18650, data4: [147, 13, 123, 207, 23, 180, 73, 144] };
pub const GUID_STANDBY_RESERVE_GRACE_PERIOD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3345215122, data2: 29160, data3: 16679, data4: [132, 235, 246, 237, 4, 58, 62, 61] };
pub const GUID_STANDBY_RESERVE_TIME: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1183836133, data2: 4440, data3: 18156, data4: [136, 188, 91, 150, 201, 228, 79, 208] };
pub const GUID_STANDBY_RESET_PERCENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1238045093, data2: 22242, data3: 19195, data4: [157, 56, 61, 244, 120, 114, 226, 27] };
pub const GUID_STANDBY_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 704037339, data2: 34522, data3: 18629, data4: [159, 219, 242, 182, 123, 31, 68, 218] };
pub const GUID_SYSTEM_AWAYMODE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2561144192, data2: 503, data3: 18602, data4: [156, 15, 68, 53, 44, 41, 229, 192] };
pub const GUID_SYSTEM_BUTTON_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1335303817, data2: 61117, data3: 17493, data4: [168, 222, 158, 89, 4, 14, 115, 71] };
pub const GUID_SYSTEM_COOLING_POLICY: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2496898581, data2: 43161, data3: 19141, data4: [174, 43, 228, 216, 246, 52, 54, 127] };
pub const GUID_TYPICAL_POWER_SAVINGS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 941310498, data2: 63124, data3: 16880, data4: [150, 133, 255, 91, 178, 96, 223, 46] };
pub const GUID_UNATTEND_SLEEP_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2076484345, data2: 55548, data3: 17513, data4: [176, 123, 51, 235, 120, 90, 172, 160] };
pub const GUID_USERINTERFACEBUTTON_ACTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2802214483, data2: 36204, data3: 16552, data4: [145, 14, 161, 245, 75, 132, 199, 229] };
pub const GUID_USER_PRESENCE_PREDICTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2181109509, data2: 64405, data3: 19782, data4: [141, 53, 64, 66, 177, 210, 13, 239] };
pub const GUID_VIDEO_ADAPTIVE_DISPLAY_BRIGHTNESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4225346150, data2: 38227, data3: 16535, data4: [186, 68, 237, 110, 157, 101, 234, 184] };
pub const GUID_VIDEO_ADAPTIVE_PERCENT_INCREASE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4007199967, data2: 45378, data3: 16771, data4: [177, 11, 90, 17, 151, 163, 120, 100] };
pub const GUID_VIDEO_ADAPTIVE_POWERDOWN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2425724194, data2: 54945, data3: 18873, data4: [175, 147, 188, 232, 133, 173, 51, 91] };
pub const GUID_VIDEO_ANNOYANCE_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2195443501, data2: 52583, data3: 16581, data4: [191, 220, 159, 26, 92, 205, 70, 99] };
pub const GUID_VIDEO_CONSOLE_LOCK_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2395255717, data2: 26728, data3: 18626, data4: [190, 117, 79, 48, 68, 190, 136, 167] };
pub const GUID_VIDEO_CURRENT_MONITOR_BRIGHTNESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2415846086, data2: 11521, data3: 18110, data4: [173, 185, 57, 138, 221, 197, 180, 255] };
pub const GUID_VIDEO_DIM_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 397058715, data2: 35651, data3: 19348, data4: [170, 254, 53, 246, 77, 170, 241, 238] };
pub const GUID_VIDEO_POWERDOWN_TIMEOUT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1007403041, data2: 51368, data3: 19975, data4: [169, 115, 107, 20, 203, 203, 43, 126] };
pub const GUID_VIDEO_SUBGROUP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1964423519, data2: 63350, data3: 17508, data4: [140, 83, 6, 22, 127, 64, 204, 153] };
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HEAP_OPTIMIZE_RESOURCES_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    pub Version: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for HEAP_OPTIMIZE_RESOURCES_INFORMATION {}
impl ::core::clone::Clone for HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct HIBERFILE_BUCKET {
    pub MaxPhysicalMemory: u64,
    pub PhysicalMemoryPercent: [u32; 3],
}
impl ::core::marker::Copy for HIBERFILE_BUCKET {}
impl ::core::clone::Clone for HIBERFILE_BUCKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type HIBERFILE_BUCKET_SIZE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucket1GB: HIBERFILE_BUCKET_SIZE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucket2GB: HIBERFILE_BUCKET_SIZE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucket4GB: HIBERFILE_BUCKET_SIZE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucket8GB: HIBERFILE_BUCKET_SIZE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucket16GB: HIBERFILE_BUCKET_SIZE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucket32GB: HIBERFILE_BUCKET_SIZE = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucketUnlimited: HIBERFILE_BUCKET_SIZE = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HiberFileBucketMax: HIBERFILE_BUCKET_SIZE = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HIBERFILE_TYPE_FULL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HIBERFILE_TYPE_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HIBERFILE_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const HIBERFILE_TYPE_REDUCED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type IGP_ID = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_GETIMEVERSION: IGP_ID = 4294967292u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_PROPERTY: IGP_ID = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_CONVERSION: IGP_ID = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_SENTENCE: IGP_ID = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_UI: IGP_ID = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_SETCOMPSTR: IGP_ID = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IGP_SELECT: IGP_ID = 24u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u64,
    pub EndAddress: u64,
    pub ExceptionHandler: u64,
    pub HandlerData: u64,
    pub PrologEndAddress: u64,
}
impl ::core::marker::Copy for IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub ExceptionHandler: u32,
    pub HandlerData: u32,
    pub PrologEndAddress: u32,
}
impl ::core::marker::Copy for IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ARCHITECTURE_ENTRY {
    pub FixupInstRVA: u32,
    pub NewInst: u32,
}
impl ::core::marker::Copy for IMAGE_ARCHITECTURE_ENTRY {}
impl ::core::clone::Clone for IMAGE_ARCHITECTURE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ARCHITECTURE_HEADER {
    pub _bitfield: u32,
    pub FirstEntryRVA: u32,
}
impl ::core::marker::Copy for IMAGE_ARCHITECTURE_HEADER {}
impl ::core::clone::Clone for IMAGE_ARCHITECTURE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_END: &str = "`\n";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_HYBRIDMAP_MEMBER: &str = "/<HYBRIDMAP>/   ";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_LINKER_MEMBER: &str = "/               ";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_LONGNAMES_MEMBER: &str = "//              ";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ARCHIVE_MEMBER_HEADER {
    pub Name: [u8; 16],
    pub Date: [u8; 12],
    pub UserID: [u8; 6],
    pub GroupID: [u8; 6],
    pub Mode: [u8; 8],
    pub Size: [u8; 10],
    pub EndHeader: [u8; 2],
}
impl ::core::marker::Copy for IMAGE_ARCHIVE_MEMBER_HEADER {}
impl ::core::clone::Clone for IMAGE_ARCHIVE_MEMBER_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_PAD: &str = "\n";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_START: &str = "!<arch>\n";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ARCHIVE_START_SIZE: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    pub HeaderData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0,
}
impl ::core::marker::Copy for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {}
impl ::core::clone::Clone for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0 {}
impl ::core::clone::Clone for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub Anonymous: IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0,
}
impl ::core::marker::Copy for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindData: u32,
    pub Anonymous: IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0,
}
impl ::core::marker::Copy for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {}
impl ::core::clone::Clone for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0 {}
impl ::core::clone::Clone for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_AUX_SYMBOL {
    pub Sym: IMAGE_AUX_SYMBOL_3,
    pub File: IMAGE_AUX_SYMBOL_1,
    pub Section: IMAGE_AUX_SYMBOL_2,
    pub TokenDef: IMAGE_AUX_SYMBOL_TOKEN_DEF,
    pub CRC: IMAGE_AUX_SYMBOL_0,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_0 {
    pub crc: u32,
    pub rgbReserved: [u8; 14],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_0 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_1 {
    pub Name: [u8; 18],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_1 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_2 {
    pub Length: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub CheckSum: u32,
    pub Number: i16,
    pub Selection: u8,
    pub bReserved: u8,
    pub HighNumber: i16,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_2 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_3 {
    pub TagIndex: u32,
    pub Misc: IMAGE_AUX_SYMBOL_3_1,
    pub FcnAry: IMAGE_AUX_SYMBOL_3_0,
    pub TvIndex: u16,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_3 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_AUX_SYMBOL_3_0 {
    pub Function: IMAGE_AUX_SYMBOL_3_0_1,
    pub Array: IMAGE_AUX_SYMBOL_3_0_0,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_3_0 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_3_0_0 {
    pub Dimension: [u16; 4],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_3_0_0 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_3_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_3_0_1 {
    pub PointerToLinenumber: u32,
    pub PointerToNextFunction: u32,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_3_0_1 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_3_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_AUX_SYMBOL_3_1 {
    pub LnSz: IMAGE_AUX_SYMBOL_3_1_0,
    pub TotalSize: u32,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_3_1 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_3_1_0 {
    pub Linenumber: u16,
    pub Size: u16,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_3_1_0 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_AUX_SYMBOL_EX {
    pub Sym: IMAGE_AUX_SYMBOL_EX_4,
    pub File: IMAGE_AUX_SYMBOL_EX_2,
    pub Section: IMAGE_AUX_SYMBOL_EX_3,
    pub Anonymous: IMAGE_AUX_SYMBOL_EX_0,
    pub CRC: IMAGE_AUX_SYMBOL_EX_1,
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_EX {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_EX_0 {
    pub TokenDef: IMAGE_AUX_SYMBOL_TOKEN_DEF,
    pub rgbReserved: [u8; 2],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_EX_0 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_EX_1 {
    pub crc: u32,
    pub rgbReserved: [u8; 16],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_EX_1 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_EX_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_EX_2 {
    pub Name: [u8; 20],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_EX_2 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_EX_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_EX_3 {
    pub Length: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub CheckSum: u32,
    pub Number: i16,
    pub Selection: u8,
    pub bReserved: u8,
    pub HighNumber: i16,
    pub rgbReserved: [u8; 2],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_EX_3 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_EX_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_EX_4 {
    pub WeakDefaultSymIndex: u32,
    pub WeakSearchType: u32,
    pub rgbReserved: [u8; 12],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_EX_4 {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_EX_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_AUX_SYMBOL_TOKEN_DEF {
    pub bAuxType: u8,
    pub bReserved: u8,
    pub SymbolTableIndex: u32,
    pub rgbReserved: [u8; 12],
}
impl ::core::marker::Copy for IMAGE_AUX_SYMBOL_TOKEN_DEF {}
impl ::core::clone::Clone for IMAGE_AUX_SYMBOL_TOKEN_DEF {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type IMAGE_AUX_SYMBOL_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF: IMAGE_AUX_SYMBOL_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_BASE_RELOCATION {
    pub VirtualAddress: u32,
    pub SizeOfBlock: u32,
}
impl ::core::marker::Copy for IMAGE_BASE_RELOCATION {}
impl ::core::clone::Clone for IMAGE_BASE_RELOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_BOUND_FORWARDER_REF {
    pub TimeDateStamp: u32,
    pub OffsetModuleName: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for IMAGE_BOUND_FORWARDER_REF {}
impl ::core::clone::Clone for IMAGE_BOUND_FORWARDER_REF {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_BOUND_IMPORT_DESCRIPTOR {
    pub TimeDateStamp: u32,
    pub OffsetModuleName: u16,
    pub NumberOfModuleForwarderRefs: u16,
}
impl ::core::marker::Copy for IMAGE_BOUND_IMPORT_DESCRIPTOR {}
impl ::core::clone::Clone for IMAGE_BOUND_IMPORT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    pub FuncStart: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_CE_RUNTIME_FUNCTION_ENTRY {}
impl ::core::clone::Clone for IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_ANY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_EXACT_MATCH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_LARGEST: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_NEWEST: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_NODUPLICATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COMDAT_SELECT_SAME_SIZE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGE_DEBUG_MISC {
    pub DataType: u32,
    pub Length: u32,
    pub Unicode: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
    pub Data: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_DEBUG_MISC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_DEBUG_MISC {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_MISC_EXENAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_CLSID: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_EX_DLLCHARACTERISTICS: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_ILTCG: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_MPX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_POGO: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_REPRO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_RESERVED10: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DEBUG_TYPE_VC_FEATURE: u32 = 12u32;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}
impl ::core::marker::Copy for IMAGE_DOS_HEADER {}
impl ::core::clone::Clone for IMAGE_DOS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DOS_SIGNATURE: u16 = 23117u16;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_DYNAMIC_RELOCATION32 {
    pub Symbol: u32,
    pub BaseRelocSize: u32,
}
impl ::core::marker::Copy for IMAGE_DYNAMIC_RELOCATION32 {}
impl ::core::clone::Clone for IMAGE_DYNAMIC_RELOCATION32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_DYNAMIC_RELOCATION32_V2 {
    pub HeaderSize: u32,
    pub FixupInfoSize: u32,
    pub Symbol: u32,
    pub SymbolGroup: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for IMAGE_DYNAMIC_RELOCATION32_V2 {}
impl ::core::clone::Clone for IMAGE_DYNAMIC_RELOCATION32_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_DYNAMIC_RELOCATION64 {
    pub Symbol: u64,
    pub BaseRelocSize: u32,
}
impl ::core::marker::Copy for IMAGE_DYNAMIC_RELOCATION64 {}
impl ::core::clone::Clone for IMAGE_DYNAMIC_RELOCATION64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_DYNAMIC_RELOCATION64_V2 {
    pub HeaderSize: u32,
    pub FixupInfoSize: u32,
    pub Symbol: u64,
    pub SymbolGroup: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for IMAGE_DYNAMIC_RELOCATION64_V2 {}
impl ::core::clone::Clone for IMAGE_DYNAMIC_RELOCATION64_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_DYNAMIC_RELOCATION_TABLE {
    pub Version: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for IMAGE_DYNAMIC_RELOCATION_TABLE {}
impl ::core::clone::Clone for IMAGE_DYNAMIC_RELOCATION_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_IMPORT_MATCH_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_LONG_ID_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_POLICY_DEBUGGABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ENCLAVE_SHORT_ID_LENGTH: u32 = 16u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    pub EpilogueCount: u32,
    pub EpilogueByteCount: u8,
    pub BranchDescriptorElementSize: u8,
    pub BranchDescriptorCount: u16,
}
impl ::core::marker::Copy for IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {}
impl ::core::clone::Clone for IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Name: u32,
    pub Base: u32,
    pub NumberOfFunctions: u32,
    pub NumberOfNames: u32,
    pub AddressOfFunctions: u32,
    pub AddressOfNames: u32,
    pub AddressOfNameOrdinals: u32,
}
impl ::core::marker::Copy for IMAGE_EXPORT_DIRECTORY {}
impl ::core::clone::Clone for IMAGE_EXPORT_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CFW_INSTRUMENTED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_INSTRUMENTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_FLAG_FID_LANGEXCPTHANDLER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_FLAG_FID_SUPPRESSED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_FLAG_FID_XFG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_RETPOLINE_PRESENT: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_RF_ENABLE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_RF_INSTRUMENTED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_RF_STRICT: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_GUARD_XFG_ENABLED: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_ABSOLUTE: u32 = 180224u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_HOT_PATCH_BASE {
    pub SequenceNumber: u32,
    pub Flags: u32,
    pub OriginalTimeDateStamp: u32,
    pub OriginalCheckSum: u32,
    pub CodeIntegrityInfo: u32,
    pub CodeIntegritySize: u32,
    pub PatchTable: u32,
    pub BufferOffset: u32,
}
impl ::core::marker::Copy for IMAGE_HOT_PATCH_BASE {}
impl ::core::clone::Clone for IMAGE_HOT_PATCH_BASE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_BASE_OBLIGATORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CALL_TARGET: u32 = 278528u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_INVERSE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_OBLIGATORY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_RESERVED: u32 = 1072705536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_SIZE: u32 = 4095u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_TARGET_RVA: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_CHUNK_TYPE: u32 = 1032192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_DYNAMIC_VALUE: u32 = 491520u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_FUNCTION: u32 = 114688u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_HOT_PATCH_HASHES {
    pub SHA256: [u8; 32],
    pub SHA1: [u8; 20],
}
impl ::core::marker::Copy for IMAGE_HOT_PATCH_HASHES {}
impl ::core::clone::Clone for IMAGE_HOT_PATCH_HASHES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_INDIRECT: u32 = 376832u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_HOT_PATCH_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SequenceNumber: u32,
    pub BaseImageList: u32,
    pub BaseImageCount: u32,
    pub BufferOffset: u32,
    pub ExtraPatchSize: u32,
}
impl ::core::marker::Copy for IMAGE_HOT_PATCH_INFO {}
impl ::core::clone::Clone for IMAGE_HOT_PATCH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_NO_CALL_TARGET: u32 = 409600u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_HOT_PATCH_REL32: u32 = 245760u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGE_IMPORT_BY_NAME {
    pub Hint: u16,
    pub Name: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_IMPORT_BY_NAME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_IMPORT_BY_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {}
impl ::core::clone::Clone for IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub Anonymous: IMAGE_IMPORT_DESCRIPTOR_0,
    pub TimeDateStamp: u32,
    pub ForwarderChain: u32,
    pub Name: u32,
    pub FirstThunk: u32,
}
impl ::core::marker::Copy for IMAGE_IMPORT_DESCRIPTOR {}
impl ::core::clone::Clone for IMAGE_IMPORT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_IMPORT_DESCRIPTOR_0 {
    pub Characteristics: u32,
    pub OriginalFirstThunk: u32,
}
impl ::core::marker::Copy for IMAGE_IMPORT_DESCRIPTOR_0 {}
impl ::core::clone::Clone for IMAGE_IMPORT_DESCRIPTOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {}
impl ::core::clone::Clone for IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_LINENUMBER {
    pub Type: IMAGE_LINENUMBER_0,
    pub Linenumber: u16,
}
impl ::core::marker::Copy for IMAGE_LINENUMBER {}
impl ::core::clone::Clone for IMAGE_LINENUMBER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_LINENUMBER_0 {
    pub SymbolTableIndex: u32,
    pub VirtualAddress: u32,
}
impl ::core::marker::Copy for IMAGE_LINENUMBER_0 {}
impl ::core::clone::Clone for IMAGE_LINENUMBER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_NT_SIGNATURE: u16 = 17744u16;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ORDINAL_FLAG: u64 = 9223372036854775808u64;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ORDINAL_FLAG32: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_ORDINAL_FLAG64: u64 = 9223372036854775808u64;
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGE_OS2_HEADER {
    pub ne_magic: u16,
    pub ne_ver: super::super::Foundation::CHAR,
    pub ne_rev: super::super::Foundation::CHAR,
    pub ne_enttab: u16,
    pub ne_cbenttab: u16,
    pub ne_crc: i32,
    pub ne_flags: u16,
    pub ne_autodata: u16,
    pub ne_heap: u16,
    pub ne_stack: u16,
    pub ne_csip: i32,
    pub ne_sssp: i32,
    pub ne_cseg: u16,
    pub ne_cmod: u16,
    pub ne_cbnrestab: u16,
    pub ne_segtab: u16,
    pub ne_rsrctab: u16,
    pub ne_restab: u16,
    pub ne_modtab: u16,
    pub ne_imptab: u16,
    pub ne_nrestab: i32,
    pub ne_cmovent: u16,
    pub ne_align: u16,
    pub ne_cres: u16,
    pub ne_exetyp: u8,
    pub ne_flagsothers: u8,
    pub ne_pretthunks: u16,
    pub ne_psegrefbytes: u16,
    pub ne_swaparea: u16,
    pub ne_expver: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_OS2_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_OS2_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_OS2_SIGNATURE: u16 = 17742u16;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_OS2_SIGNATURE_LE: u16 = 17740u16;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGE_POLICY_ENTRY {
    pub Type: IMAGE_POLICY_ENTRY_TYPE,
    pub PolicyId: IMAGE_POLICY_ID,
    pub u: IMAGE_POLICY_ENTRY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_POLICY_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_POLICY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union IMAGE_POLICY_ENTRY_0 {
    pub None: *const ::core::ffi::c_void,
    pub BoolValue: super::super::Foundation::BOOLEAN,
    pub Int8Value: i8,
    pub UInt8Value: u8,
    pub Int16Value: i16,
    pub UInt16Value: u16,
    pub Int32Value: i32,
    pub UInt32Value: u32,
    pub Int64Value: i64,
    pub UInt64Value: u64,
    pub AnsiStringValue: ::windows_sys::core::PCSTR,
    pub UnicodeStringValue: ::windows_sys::core::PCWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_POLICY_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_POLICY_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type IMAGE_POLICY_ENTRY_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeNone: IMAGE_POLICY_ENTRY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeBool: IMAGE_POLICY_ENTRY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeInt8: IMAGE_POLICY_ENTRY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeUInt8: IMAGE_POLICY_ENTRY_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeInt16: IMAGE_POLICY_ENTRY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeUInt16: IMAGE_POLICY_ENTRY_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeInt32: IMAGE_POLICY_ENTRY_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeUInt32: IMAGE_POLICY_ENTRY_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeInt64: IMAGE_POLICY_ENTRY_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeUInt64: IMAGE_POLICY_ENTRY_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeAnsiString: IMAGE_POLICY_ENTRY_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeUnicodeString: IMAGE_POLICY_ENTRY_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeOverride: IMAGE_POLICY_ENTRY_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyEntryTypeMaximum: IMAGE_POLICY_ENTRY_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type IMAGE_POLICY_ID = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdNone: IMAGE_POLICY_ID = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdEtw: IMAGE_POLICY_ID = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdDebug: IMAGE_POLICY_ID = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdCrashDump: IMAGE_POLICY_ID = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdCrashDumpKey: IMAGE_POLICY_ID = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdCrashDumpKeyGuid: IMAGE_POLICY_ID = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdParentSd: IMAGE_POLICY_ID = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdParentSdRev: IMAGE_POLICY_ID = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdSvn: IMAGE_POLICY_ID = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdDeviceId: IMAGE_POLICY_ID = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdCapability: IMAGE_POLICY_ID = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdScenarioId: IMAGE_POLICY_ID = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ImagePolicyIdMaximum: IMAGE_POLICY_ID = 12i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGE_POLICY_METADATA {
    pub Version: u8,
    pub Reserved0: [u8; 7],
    pub ApplicationId: u64,
    pub Policies: [IMAGE_POLICY_ENTRY; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_POLICY_METADATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_POLICY_METADATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_POLICY_METADATA_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_POLICY_SECTION_NAME: &str = ".tPolicy";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    pub PrologueByteCount: u8,
}
impl ::core::marker::Copy for IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {}
impl ::core::clone::Clone for IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RELOCATION {
    pub Anonymous: IMAGE_RELOCATION_0,
    pub SymbolTableIndex: u32,
    pub Type: u16,
}
impl ::core::marker::Copy for IMAGE_RELOCATION {}
impl ::core::clone::Clone for IMAGE_RELOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_RELOCATION_0 {
    pub VirtualAddress: u32,
    pub RelocCount: u32,
}
impl ::core::marker::Copy for IMAGE_RELOCATION_0 {}
impl ::core::clone::Clone for IMAGE_RELOCATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_BRADDR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_GPDISP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_GPREL32: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_GPRELHI: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_GPRELLO: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_HINT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_INLINE_REFLONG: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_LITERAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_LITUSE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_MATCH: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_PAIR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFHI: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFLO: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFLONG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFLONGNB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFQ1: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFQ2: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFQ3: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_REFQUAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_SECREL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_SECRELHI: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_SECRELLO: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ALPHA_SECTION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_ADDR32: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_ADDR32NB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_ADDR64: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_CFG_BR: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_CFG_BR_REX: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_CFG_CALL: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_EHANDLER: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_IMPORT_BR: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_IMPORT_CALL: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_INDIR_BR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_INDIR_BR_REX: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_INDIR_CALL: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_PAIR: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_REL32: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_REL32_1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_REL32_2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_REL32_3: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_REL32_4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_REL32_5: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_SECREL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_SECREL7: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_SECTION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_SREL32: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_SSPAN32: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AMD64_TOKEN: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_ADDR32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_ADDR32NB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_CALL32: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_FUNCINFO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_REL32_1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_REL32_2: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_SECREL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_SECTION: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_AM_TOKEN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_ADDR32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_ADDR32NB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_ADDR64: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_BRANCH19: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_BRANCH26: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_PAGEBASE_REL21: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_PAGEOFFSET_12A: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_PAGEOFFSET_12L: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_REL21: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_SECREL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_SECREL_HIGH12A: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_SECREL_LOW12A: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_SECREL_LOW12L: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_SECTION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM64_TOKEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_ADDR32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_ADDR32NB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BLX11: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BLX23T: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BLX24: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BRANCH11: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BRANCH20T: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BRANCH24: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_BRANCH24T: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_GPREL12: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_GPREL7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_MOV32: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_MOV32A: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_MOV32T: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_SECREL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_SECTION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_ARM_TOKEN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_ARM_MOV32: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_DIR64: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_HIGH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_HIGHADJ: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_HIGHLOW: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_IA64_IMM64: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_LOW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_MIPS_JMPADDR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_MIPS_JMPADDR16: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_RESERVED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_BASED_THUMB_MOV32: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_ADDR32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_ADDR32NB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_ADDR64: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_SECREL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_SECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEE_TOKEN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_ADDR32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_ADDR32NB: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_ADDR64: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_SECREL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_SECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_CEF_TOKEN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_EBC_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_EBC_ADDR32NB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_EBC_REL32: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_EBC_SECREL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_EBC_SECTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_DIR16: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_DIR32: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_DIR32NB: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_REL16: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_REL32: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_SECREL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_SECREL7: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_SECTION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_SEG12: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_I386_TOKEN: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_ADDEND: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_DIR32: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_DIR32NB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_DIR64: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_GPREL22: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_GPREL32: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_IMM14: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_IMM22: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_IMM64: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_IMMGPREL64: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_LTOFF22: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL21B: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL21F: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL21M: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL60B: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL60F: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL60I: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL60M: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_PCREL60X: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SECREL22: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SECREL32: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SECREL64I: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SECTION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SREL14: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SREL22: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_SREL32: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_TOKEN: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_IA64_UREL32: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_ADDR24: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_ADDR32: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_ADDR32NB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_GPREL16: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_PAIR: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_PCREL16: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_PCREL24: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_PCREL8: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_REFHALF: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_REFHI: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_REFLO: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_SECREL32: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_SECTION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_M32R_TOKEN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_GPREL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_JMPADDR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_JMPADDR16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_LITERAL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_PAIR: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_REFHALF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_REFHI: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_REFLO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_REFWORD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_REFWORDNB: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_SECREL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_SECRELHI: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_SECRELLO: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_SECTION: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_MIPS_TOKEN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ADDR14: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ADDR16: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ADDR24: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ADDR32: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ADDR32NB: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_ADDR64: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_BRNTAKEN: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_BRTAKEN: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_GPREL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_IFGLUE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_IMGLUE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_NEG: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_PAIR: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_REFHI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_REFLO: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_REL14: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_REL24: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_SECREL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_SECREL16: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_SECRELHI: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_SECRELLO: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_SECTION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_TOCDEFN: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_TOCREL14: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_TOCREL16: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_TOKEN: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_PPC_TYPEMASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_ABSOLUTE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT16: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT32: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT32_NB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT4: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT4_LONG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT4_WORD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT8: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT8_LONG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_DIRECT8_WORD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_GPREL4_LONG: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_PCREL12_WORD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_PCREL8_LONG: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_PCREL8_WORD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_SECREL: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_SECTION: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_SIZEOF_SECTION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_STARTOF_SECTION: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH3_TOKEN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SHM_PAIR: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SHM_PCRELPT: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SHM_REFHALF: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SHM_REFLO: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SHM_RELHALF: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SHM_RELLO: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_SH_NOMODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_THUMB_BLX23: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_THUMB_BRANCH20: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_THUMB_BRANCH24: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_REL_THUMB_MOV32: u32 = 17u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RESOURCE_DATA_ENTRY {
    pub OffsetToData: u32,
    pub Size: u32,
    pub CodePage: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DATA_ENTRY {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DATA_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_RESOURCE_DATA_IS_DIRECTORY: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RESOURCE_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub NumberOfNamedEntries: u16,
    pub NumberOfIdEntries: u16,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    pub Anonymous1: IMAGE_RESOURCE_DIRECTORY_ENTRY_0,
    pub Anonymous2: IMAGE_RESOURCE_DIRECTORY_ENTRY_1,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY_ENTRY {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0,
    pub Name: u32,
    pub Id: u16,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0 {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {
    pub OffsetToData: u32,
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0 {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IMAGE_RESOURCE_DIRECTORY_STRING {
    pub Length: u16,
    pub NameString: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IMAGE_RESOURCE_DIRECTORY_STRING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IMAGE_RESOURCE_DIRECTORY_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_RESOURCE_DIR_STRING_U {
    pub Length: u16,
    pub NameString: [u16; 1],
}
impl ::core::marker::Copy for IMAGE_RESOURCE_DIR_STRING_U {}
impl ::core::clone::Clone for IMAGE_RESOURCE_DIR_STRING_U {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_RESOURCE_NAME_IS_STRING: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SEPARATE_DEBUG_FLAGS_MASK: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_SEPARATE_DEBUG_HEADER {
    pub Signature: u16,
    pub Flags: u16,
    pub Machine: u16,
    pub Characteristics: u16,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub ImageBase: u32,
    pub SizeOfImage: u32,
    pub NumberOfSections: u32,
    pub ExportedNamesSize: u32,
    pub DebugDirectorySize: u32,
    pub SectionAlignment: u32,
    pub Reserved: [u32; 2],
}
impl ::core::marker::Copy for IMAGE_SEPARATE_DEBUG_HEADER {}
impl ::core::clone::Clone for IMAGE_SEPARATE_DEBUG_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SEPARATE_DEBUG_MISMATCH: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SEPARATE_DEBUG_SIGNATURE: u32 = 18756u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SIZEOF_FILE_HEADER: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SIZEOF_SECTION_HEADER: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SIZEOF_SHORT_NAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SIZEOF_SYMBOL: u32 = 18u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {}
impl ::core::clone::Clone for IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_SYMBOL {
    pub N: IMAGE_SYMBOL_0,
    pub Value: u32,
    pub SectionNumber: i16,
    pub Type: u16,
    pub StorageClass: u8,
    pub NumberOfAuxSymbols: u8,
}
impl ::core::marker::Copy for IMAGE_SYMBOL {}
impl ::core::clone::Clone for IMAGE_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_SYMBOL_0 {
    pub ShortName: [u8; 8],
    pub Name: IMAGE_SYMBOL_0_0,
    pub LongName: [u32; 2],
}
impl ::core::marker::Copy for IMAGE_SYMBOL_0 {}
impl ::core::clone::Clone for IMAGE_SYMBOL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_SYMBOL_0_0 {
    pub Short: u32,
    pub Long: u32,
}
impl ::core::marker::Copy for IMAGE_SYMBOL_0_0 {}
impl ::core::clone::Clone for IMAGE_SYMBOL_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_SYMBOL_EX {
    pub N: IMAGE_SYMBOL_EX_0,
    pub Value: u32,
    pub SectionNumber: i32,
    pub Type: u16,
    pub StorageClass: u8,
    pub NumberOfAuxSymbols: u8,
}
impl ::core::marker::Copy for IMAGE_SYMBOL_EX {}
impl ::core::clone::Clone for IMAGE_SYMBOL_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_SYMBOL_EX_0 {
    pub ShortName: [u8; 8],
    pub Name: IMAGE_SYMBOL_EX_0_0,
    pub LongName: [u32; 2],
}
impl ::core::marker::Copy for IMAGE_SYMBOL_EX_0 {}
impl ::core::clone::Clone for IMAGE_SYMBOL_EX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_SYMBOL_EX_0_0 {
    pub Short: u32,
    pub Long: u32,
}
impl ::core::marker::Copy for IMAGE_SYMBOL_EX_0_0 {}
impl ::core::clone::Clone for IMAGE_SYMBOL_EX_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_ARGUMENT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_AUTOMATIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_BIT_FIELD: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_BLOCK: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_CLR_TOKEN: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_END_OF_STRUCT: u32 = 102u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_ENUM_TAG: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_EXTERNAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_EXTERNAL_DEF: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_FAR_EXTERNAL: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_FILE: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_FUNCTION: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_LABEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_REGISTER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_REGISTER_PARAM: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_SECTION: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_STATIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_STRUCT_TAG: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_TYPE_DEFINITION: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_UNION_TAG: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_DTYPE_ARRAY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_DTYPE_FUNCTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_DTYPE_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_DTYPE_POINTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_SECTION_MAX: u32 = 65279u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_SECTION_MAX_EX: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_BYTE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_CHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_DOUBLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_DWORD: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_ENUM: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_FLOAT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_INT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_LONG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_MOE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_NULL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_PCODE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_SHORT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_STRUCT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_UINT: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_UNION: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_VOID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_SYM_TYPE_WORD: u32 = 13u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_TLS_DIRECTORY32 {
    pub StartAddressOfRawData: u32,
    pub EndAddressOfRawData: u32,
    pub AddressOfIndex: u32,
    pub AddressOfCallBacks: u32,
    pub SizeOfZeroFill: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY32_0,
}
impl ::core::marker::Copy for IMAGE_TLS_DIRECTORY32 {}
impl ::core::clone::Clone for IMAGE_TLS_DIRECTORY32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_TLS_DIRECTORY32_0 {
    pub Characteristics: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY32_0_0,
}
impl ::core::marker::Copy for IMAGE_TLS_DIRECTORY32_0 {}
impl ::core::clone::Clone for IMAGE_TLS_DIRECTORY32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_TLS_DIRECTORY32_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_TLS_DIRECTORY32_0_0 {}
impl ::core::clone::Clone for IMAGE_TLS_DIRECTORY32_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_TLS_DIRECTORY64 {
    pub StartAddressOfRawData: u64,
    pub EndAddressOfRawData: u64,
    pub AddressOfIndex: u64,
    pub AddressOfCallBacks: u64,
    pub SizeOfZeroFill: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY64_0,
}
impl ::core::marker::Copy for IMAGE_TLS_DIRECTORY64 {}
impl ::core::clone::Clone for IMAGE_TLS_DIRECTORY64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMAGE_TLS_DIRECTORY64_0 {
    pub Characteristics: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY64_0_0,
}
impl ::core::marker::Copy for IMAGE_TLS_DIRECTORY64_0 {}
impl ::core::clone::Clone for IMAGE_TLS_DIRECTORY64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_TLS_DIRECTORY64_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for IMAGE_TLS_DIRECTORY64_0_0 {}
impl ::core::clone::Clone for IMAGE_TLS_DIRECTORY64_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(2))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMAGE_VXD_HEADER {
    pub e32_magic: u16,
    pub e32_border: u8,
    pub e32_worder: u8,
    pub e32_level: u32,
    pub e32_cpu: u16,
    pub e32_os: u16,
    pub e32_ver: u32,
    pub e32_mflags: u32,
    pub e32_mpages: u32,
    pub e32_startobj: u32,
    pub e32_eip: u32,
    pub e32_stackobj: u32,
    pub e32_esp: u32,
    pub e32_pagesize: u32,
    pub e32_lastpagesize: u32,
    pub e32_fixupsize: u32,
    pub e32_fixupsum: u32,
    pub e32_ldrsize: u32,
    pub e32_ldrsum: u32,
    pub e32_objtab: u32,
    pub e32_objcnt: u32,
    pub e32_objmap: u32,
    pub e32_itermap: u32,
    pub e32_rsrctab: u32,
    pub e32_rsrccnt: u32,
    pub e32_restab: u32,
    pub e32_enttab: u32,
    pub e32_dirtab: u32,
    pub e32_dircnt: u32,
    pub e32_fpagetab: u32,
    pub e32_frectab: u32,
    pub e32_impmod: u32,
    pub e32_impmodcnt: u32,
    pub e32_impproc: u32,
    pub e32_pagesum: u32,
    pub e32_datapage: u32,
    pub e32_preload: u32,
    pub e32_nrestab: u32,
    pub e32_cbnrestab: u32,
    pub e32_nressum: u32,
    pub e32_autodata: u32,
    pub e32_debuginfo: u32,
    pub e32_debuglen: u32,
    pub e32_instpreload: u32,
    pub e32_instdemand: u32,
    pub e32_heapsize: u32,
    pub e32_res3: [u8; 12],
    pub e32_winresoff: u32,
    pub e32_winreslen: u32,
    pub e32_devid: u16,
    pub e32_ddkver: u16,
}
impl ::core::marker::Copy for IMAGE_VXD_HEADER {}
impl ::core::clone::Clone for IMAGE_VXD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_VXD_SIGNATURE: u16 = 17740u16;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_HDR_SIG2: u32 = 65535u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct IMPORT_OBJECT_HEADER {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub SizeOfData: u32,
    pub Anonymous: IMPORT_OBJECT_HEADER_0,
    pub _bitfield: u16,
}
impl ::core::marker::Copy for IMPORT_OBJECT_HEADER {}
impl ::core::clone::Clone for IMPORT_OBJECT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union IMPORT_OBJECT_HEADER_0 {
    pub Ordinal: u16,
    pub Hint: u16,
}
impl ::core::marker::Copy for IMPORT_OBJECT_HEADER_0 {}
impl ::core::clone::Clone for IMPORT_OBJECT_HEADER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type IMPORT_OBJECT_NAME_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_ORDINAL: IMPORT_OBJECT_NAME_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_NAME: IMPORT_OBJECT_NAME_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_NAME_NO_PREFIX: IMPORT_OBJECT_NAME_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_NAME_UNDECORATE: IMPORT_OBJECT_NAME_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_NAME_EXPORTAS: IMPORT_OBJECT_NAME_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type IMPORT_OBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_CODE: IMPORT_OBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_DATA: IMPORT_OBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMPORT_OBJECT_CONST: IMPORT_OBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const INITIAL_CPSR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const INITIAL_FPCSR: u32 = 639u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const INITIAL_FPSCR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const INITIAL_MXCSR: u32 = 8064u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_BAD_BLOCK_WITH_NAME: super::super::Foundation::NTSTATUS = -1073479649i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_CDROM_EXCLUSIVE_LOCK: super::super::Foundation::NTSTATUS = 1074004101i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_COMPLETION_MODIFY_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DRIVER_CANCEL_TIMEOUT: super::super::Foundation::NTSTATUS = -2147221450i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_CALLBACK_EXCEPTION: super::super::Foundation::NTSTATUS = -1073479517i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_CREATION_SUCCESS: super::super::Foundation::NTSTATUS = 262306i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_DIRECT_CONFIG_FAILED: super::super::Foundation::NTSTATUS = -1073479632i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_DRIVER_LOAD_FAILURE: super::super::Foundation::NTSTATUS = -1073479635i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_DUMPFILE_CONFLICT: super::super::Foundation::NTSTATUS = -1073479633i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_INITIALIZATION_FAILURE: super::super::Foundation::NTSTATUS = -1073479634i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_PAGE_CONFIG_FAILED: super::super::Foundation::NTSTATUS = -1073479631i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_DUMP_POINTER_FAILURE: super::super::Foundation::NTSTATUS = -1073479636i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERROR_DISK_RESOURCES_EXHAUSTED: super::super::Foundation::NTSTATUS = -1073479530i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERROR_DUMP_CREATION_ERROR: super::super::Foundation::NTSTATUS = -1073479519i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERROR_IO_HARDWARE_ERROR: super::super::Foundation::NTSTATUS = -1073479526i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_BAD_BLOCK: super::super::Foundation::NTSTATUS = -1073479673i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_BAD_FIRMWARE: super::super::Foundation::NTSTATUS = -1073479655i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_CONFIGURATION_ERROR: super::super::Foundation::NTSTATUS = -1073479677i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_CONTROLLER_ERROR: super::super::Foundation::NTSTATUS = -1073479669i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_DMA_CONFLICT_DETECTED: super::super::Foundation::NTSTATUS = -1073479657i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_DMA_RESOURCE_CONFLICT: super::super::Foundation::NTSTATUS = -1073479653i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_DRIVER_ERROR: super::super::Foundation::NTSTATUS = -1073479676i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_INCORRECT_IRQL: super::super::Foundation::NTSTATUS = -1073479667i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_INSUFFICIENT_RESOURCES: super::super::Foundation::NTSTATUS = -1073479678i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_INTERNAL_ERROR: super::super::Foundation::NTSTATUS = -1073479668i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_INTERRUPT_RESOURCE_CONFLICT: super::super::Foundation::NTSTATUS = -1073479652i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_INVALID_IOBASE: super::super::Foundation::NTSTATUS = -1073479666i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_INVALID_REQUEST: super::super::Foundation::NTSTATUS = -1073479664i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_IRQ_CONFLICT_DETECTED: super::super::Foundation::NTSTATUS = -1073479656i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_LAYERED_FAILURE: super::super::Foundation::NTSTATUS = -1073479662i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_MEMORY_CONFLICT_DETECTED: super::super::Foundation::NTSTATUS = -1073479659i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_MEMORY_RESOURCE_CONFLICT: super::super::Foundation::NTSTATUS = -1073479651i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_NOT_READY: super::super::Foundation::NTSTATUS = -1073479665i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_OVERRUN_ERROR: super::super::Foundation::NTSTATUS = -1073479672i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_PARITY: super::super::Foundation::NTSTATUS = -1073479675i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_PORT_CONFLICT_DETECTED: super::super::Foundation::NTSTATUS = -1073479658i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_PORT_RESOURCE_CONFLICT: super::super::Foundation::NTSTATUS = -1073479650i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_PORT_TIMEOUT: super::super::Foundation::NTSTATUS = -1073479563i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_PROTOCOL: super::super::Foundation::NTSTATUS = -1073479660i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_RESET: super::super::Foundation::NTSTATUS = -1073479661i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_RETRY_SUCCEEDED: super::super::Foundation::NTSTATUS = 262145i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_SEEK_ERROR: super::super::Foundation::NTSTATUS = -1073479674i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_SEQUENCE: super::super::Foundation::NTSTATUS = -1073479670i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_THREAD_STUCK_IN_DEVICE_DRIVER: super::super::Foundation::NTSTATUS = -1073479572i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_TIMEOUT: super::super::Foundation::NTSTATUS = -1073479671i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_ERR_VERSION: super::super::Foundation::NTSTATUS = -1073479663i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_QUOTA_CORRUPT: super::super::Foundation::NTSTATUS = -1073479638i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_QUOTA_FAILED: super::super::Foundation::NTSTATUS = -2147221464i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_QUOTA_LIMIT: super::super::Foundation::NTSTATUS = 1074004005i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_QUOTA_STARTED: super::super::Foundation::NTSTATUS = 1074004006i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_QUOTA_SUCCEEDED: super::super::Foundation::NTSTATUS = 1074004007i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_QUOTA_THRESHOLD: super::super::Foundation::NTSTATUS = 1074004004i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_SYSTEM_CORRUPT: super::super::Foundation::NTSTATUS = -1073479639i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_FILE_SYSTEM_CORRUPT_WITH_NAME: super::super::Foundation::NTSTATUS = -1073479625i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_INFO_THROTTLE_COMPLETE: super::super::Foundation::NTSTATUS = 1074004087i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_LOST_DELAYED_WRITE: super::super::Foundation::NTSTATUS = -2147221454i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_LOST_DELAYED_WRITE_NETWORK_DISCONNECTED: super::super::Foundation::NTSTATUS = -2147221365i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_LOST_DELAYED_WRITE_NETWORK_LOCAL_DISK_ERROR: super::super::Foundation::NTSTATUS = -2147221363i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_LOST_DELAYED_WRITE_NETWORK_SERVER_ERROR: super::super::Foundation::NTSTATUS = -2147221364i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_RECOVERED_VIA_ECC: super::super::Foundation::NTSTATUS = -2147221471i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_AF_UNIX: u32 = 2147483683u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_APPEXECLINK: u32 = 2147483675u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD: u32 = 2415919130u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_1: u32 = 2415923226u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_2: u32 = 2415927322u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_3: u32 = 2415931418u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_4: u32 = 2415935514u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_5: u32 = 2415939610u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_6: u32 = 2415943706u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_7: u32 = 2415947802u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_8: u32 = 2415951898u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_9: u32 = 2415955994u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_A: u32 = 2415960090u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_B: u32 = 2415964186u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_C: u32 = 2415968282u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_D: u32 = 2415972378u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_E: u32 = 2415976474u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_F: u32 = 2415980570u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CLOUD_MASK: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_CSV: u32 = 2147483657u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_DATALESS_CIM: u32 = 2684354600u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_DEDUP: u32 = 2147483667u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_DFS: u32 = 2147483658u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_DFSR: u32 = 2147483666u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_FILE_PLACEHOLDER: u32 = 2147483669u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_GLOBAL_REPARSE: u32 = 2684354585u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_HSM: u32 = 3221225476u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_HSM2: u32 = 2147483654u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_MOUNT_POINT: u32 = 2684354563u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_NFS: u32 = 2147483668u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_ONEDRIVE: u32 = 2147483681u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_PROJFS: u32 = 2415919132u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_PROJFS_TOMBSTONE: u32 = 2684354594u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_RESERVED_ONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_RESERVED_RANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_RESERVED_TWO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_RESERVED_ZERO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_SIS: u32 = 2147483655u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_STORAGE_SYNC: u32 = 2147483678u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_SYMLINK: u32 = 2684354572u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_UNHANDLED: u32 = 2147483680u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WCI: u32 = 2147483672u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WCI_1: u32 = 2415923224u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WCI_LINK: u32 = 2684354599u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WCI_LINK_1: u32 = 2684358695u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WCI_TOMBSTONE: u32 = 2684354591u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WIM: u32 = 2147483656u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IO_REPARSE_TAG_WOF: u32 = 2147483671u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_SYSTEM_SLEEP_FAILED: super::super::Foundation::NTSTATUS = -1073479637i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_ADAPTER_FIRMWARE_UPDATED: super::super::Foundation::NTSTATUS = 1074004128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_ALLOCATION_FAILED: super::super::Foundation::NTSTATUS = -2147221448i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_BUS_RESET: super::super::Foundation::NTSTATUS = -2147221386i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_COMPLETION_TIME: super::super::Foundation::NTSTATUS = -2147221349i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DEVICE_HAS_INTERNAL_DUMP: super::super::Foundation::NTSTATUS = -2147221361i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DISK_CAPACITY_CHANGED: super::super::Foundation::NTSTATUS = -2147221353i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DISK_FIRMWARE_UPDATED: super::super::Foundation::NTSTATUS = 1074004127i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DISK_PROVISIONING_TYPE_CHANGED: super::super::Foundation::NTSTATUS = -2147221352i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DISK_SURPRISE_REMOVED: super::super::Foundation::NTSTATUS = -2147221347i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DUMP_DISABLED_DEVICE_GONE: super::super::Foundation::NTSTATUS = -2147221348i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DUPLICATE_PATH: super::super::Foundation::NTSTATUS = -2147221445i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_DUPLICATE_SIGNATURE: super::super::Foundation::NTSTATUS = -2147221446i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_INTERRUPT_STILL_PENDING: super::super::Foundation::NTSTATUS = -2147221451i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_IO_OPERATION_RETRIED: super::super::Foundation::NTSTATUS = -2147221351i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_LOG_FLUSH_FAILED: super::super::Foundation::NTSTATUS = -2147221447i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_PAGING_FAILURE: super::super::Foundation::NTSTATUS = -2147221453i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_REPEATED_DISK_GUID: super::super::Foundation::NTSTATUS = -2147221346i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_RESET: super::super::Foundation::NTSTATUS = -2147221375i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_SOFT_THRESHOLD_REACHED: super::super::Foundation::NTSTATUS = -2147221360i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX: super::super::Foundation::NTSTATUS = -2147221359i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_LUN: super::super::Foundation::NTSTATUS = -2147221358i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_LUN_POOL: super::super::Foundation::NTSTATUS = -2147221357i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_LUN: super::super::Foundation::NTSTATUS = -2147221356i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_SOFT_THRESHOLD_REACHED_EX_POOL_POOL: super::super::Foundation::NTSTATUS = -2147221355i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_VOLUME_LOST_DISK_EXTENT: super::super::Foundation::NTSTATUS = -2147221362i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WARNING_WRITE_FUA_PROBLEM: super::super::Foundation::NTSTATUS = -2147221372i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WRITE_CACHE_DISABLED: super::super::Foundation::NTSTATUS = -2147221470i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WRITE_CACHE_ENABLED: super::super::Foundation::NTSTATUS = -2147221472i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WRN_BAD_FIRMWARE: super::super::Foundation::NTSTATUS = -2147221478i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const IO_WRN_FAILURE_PREDICTED: super::super::Foundation::NTSTATUS = -2147221452i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IS_TEXT_UNICODE_DBCS_LEADBYTE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IS_TEXT_UNICODE_UTF8: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_ASSIGN_PROCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_IMPERSONATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_ABNORMAL_EXIT_PROCESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_END_OF_JOB_TIME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_END_OF_PROCESS_TIME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_EXIT_PROCESS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_JOB_CYCLE_TIME_LIMIT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_JOB_MEMORY_LIMIT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_MAXIMUM: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_MINIMUM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_NEW_PROCESS: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_NOTIFICATION_LIMIT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_MSG_SILO_TERMINATED: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_DSCP_TAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_QUERY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_SET_ATTRIBUTES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_SET_SECURITY_ATTRIBUTES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_TERMINATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_UILIMIT_ALL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const JOB_OBJECT_UI_VALID_FLAGS: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct KERNEL_CET_CONTEXT {
    pub Ssp: u64,
    pub Rip: u64,
    pub SegCs: u16,
    pub Anonymous: KERNEL_CET_CONTEXT_0,
    pub Fill: [u16; 2],
}
impl ::core::marker::Copy for KERNEL_CET_CONTEXT {}
impl ::core::clone::Clone for KERNEL_CET_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union KERNEL_CET_CONTEXT_0 {
    pub AllFlags: u16,
    pub Anonymous: KERNEL_CET_CONTEXT_0_0,
}
impl ::core::marker::Copy for KERNEL_CET_CONTEXT_0 {}
impl ::core::clone::Clone for KERNEL_CET_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct KERNEL_CET_CONTEXT_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for KERNEL_CET_CONTEXT_0_0 {}
impl ::core::clone::Clone for KERNEL_CET_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct KTMOBJECT_CURSOR {
    pub LastQuery: ::windows_sys::core::GUID,
    pub ObjectIdCount: u32,
    pub ObjectIds: [::windows_sys::core::GUID; 1],
}
impl ::core::marker::Copy for KTMOBJECT_CURSOR {}
impl ::core::clone::Clone for KTMOBJECT_CURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type KTMOBJECT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const KTMOBJECT_TRANSACTION: KTMOBJECT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const KTMOBJECT_TRANSACTION_MANAGER: KTMOBJECT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const KTMOBJECT_RESOURCE_MANAGER: KTMOBJECT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const KTMOBJECT_ENLISTMENT: KTMOBJECT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const KTMOBJECT_INVALID: KTMOBJECT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_AFRIKAANS: u32 = 54u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ALBANIAN: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ALSATIAN: u32 = 132u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_AMHARIC: u32 = 94u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ARABIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ARMENIAN: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ASSAMESE: u32 = 77u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_AZERBAIJANI: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_AZERI: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BANGLA: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BASHKIR: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BASQUE: u32 = 45u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BELARUSIAN: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BENGALI: u32 = 69u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BOSNIAN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BOSNIAN_NEUTRAL: u32 = 30746u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BRETON: u32 = 126u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_BULGARIAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CATALAN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CENTRAL_KURDISH: u32 = 146u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CHEROKEE: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CHINESE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CHINESE_SIMPLIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CHINESE_TRADITIONAL: u32 = 31748u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CORSICAN: u32 = 131u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CROATIAN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_CZECH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_DANISH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_DARI: u32 = 140u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_DIVEHI: u32 = 101u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_DUTCH: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ENGLISH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ESTONIAN: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FAEROESE: u32 = 56u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FARSI: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FILIPINO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FINNISH: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FRENCH: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FRISIAN: u32 = 98u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_FULAH: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_GALICIAN: u32 = 86u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_GEORGIAN: u32 = 55u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_GERMAN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_GREEK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_GREENLANDIC: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_GUJARATI: u32 = 71u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_HAUSA: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_HAWAIIAN: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_HEBREW: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_HINDI: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_HUNGARIAN: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ICELANDIC: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_IGBO: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_INDONESIAN: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_INUKTITUT: u32 = 93u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_INVARIANT: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_IRISH: u32 = 60u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ITALIAN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_JAPANESE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KANNADA: u32 = 75u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KASHMIRI: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KAZAK: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KHMER: u32 = 83u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KICHE: u32 = 134u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KINYARWANDA: u32 = 135u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KONKANI: u32 = 87u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KOREAN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_KYRGYZ: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_LAO: u32 = 84u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_LATVIAN: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_LITHUANIAN: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_LOWER_SORBIAN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_LUXEMBOURGISH: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MACEDONIAN: u32 = 47u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MALAY: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MALAYALAM: u32 = 76u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MALTESE: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MANIPURI: u32 = 88u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MAORI: u32 = 129u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MAPUDUNGUN: u32 = 122u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MARATHI: u32 = 78u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MOHAWK: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_MONGOLIAN: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_NEPALI: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_NEUTRAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_NORWEGIAN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_OCCITAN: u32 = 130u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ODIA: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ORIYA: u32 = 72u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_PASHTO: u32 = 99u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_PERSIAN: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_POLISH: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_PORTUGUESE: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_PULAR: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_PUNJABI: u32 = 70u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_QUECHUA: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ROMANIAN: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ROMANSH: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_RUSSIAN: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SAKHA: u32 = 133u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SAMI: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SANSKRIT: u32 = 79u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SCOTTISH_GAELIC: u32 = 145u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SERBIAN: u32 = 26u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SERBIAN_NEUTRAL: u32 = 31770u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SINDHI: u32 = 89u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SINHALESE: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SLOVAK: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SLOVENIAN: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SOTHO: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SPANISH: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SWAHILI: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SWEDISH: u32 = 29u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_SYRIAC: u32 = 90u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TAJIK: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TAMAZIGHT: u32 = 95u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TAMIL: u32 = 73u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TATAR: u32 = 68u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TELUGU: u32 = 74u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_THAI: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TIBETAN: u32 = 81u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TIGRIGNA: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TIGRINYA: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TSWANA: u32 = 50u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TURKISH: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_TURKMEN: u32 = 66u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_UIGHUR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_UKRAINIAN: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_UPPER_SORBIAN: u32 = 46u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_URDU: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_UZBEK: u32 = 67u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_VALENCIAN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_VIETNAMESE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_WELSH: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_WOLOF: u32 = 136u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_XHOSA: u32 = 52u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_YAKUT: u32 = 133u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_YI: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_YORUBA: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LANG_ZULU: u32 = 53u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_DISCARDABLE: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_DISCARDED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_INVALID_HANDLE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_LOCKCOUNT: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_MODIFY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_NOCOMPACT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_NODISCARD: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LMEM_VALID_FLAGS: u32 = 3954u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCALE_NAME_MAX_LENGTH: u32 = 85u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCALE_TRANSIENT_KEYBOARD1: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCALE_TRANSIENT_KEYBOARD2: u32 = 9216u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCALE_TRANSIENT_KEYBOARD3: u32 = 10240u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCALE_TRANSIENT_KEYBOARD4: u32 = 11264u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKF_LOGICAL_LOCK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKF_PHYSICAL_LOCK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKP_ALLOW_MEM_MAPPING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKP_ALLOW_WRITES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKP_FAIL_MEM_MAPPING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKP_FAIL_WRITES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKP_LOCK_FOR_FORMAT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LOCKP_USER_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const LTP_PC_SMT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAILSLOT_NO_MESSAGE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAILSLOT_WAIT_FOREVER: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXBYTE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXCHAR: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXDWORD: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_ALLOWED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_PROCESSORS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_PROC_PER_GROUP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_SUSPEND_COUNT: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_WAIT_OBJECTS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXIMUM_XSTATE_FEATURES: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXLOGICALLOGNAMESIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXLONG: u32 = 2147483647u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXLONGLONG: u64 = 9223372036854775807u64;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXSHORT: u32 = 32767u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct MAXVERSIONTESTED_INFO {
    pub MaxVersionTested: u64,
}
impl ::core::marker::Copy for MAXVERSIONTESTED_INFO {}
impl ::core::clone::Clone for MAXVERSIONTESTED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAXWORD: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAX_ACL_REVISION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAX_HW_COUNTERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAX_UCSCHAR: u32 = 1114111u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_BUS_ERROR: super::super::Foundation::NTSTATUS = -1073414022i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_BUS_TIMEOUT_ERROR: super::super::Foundation::NTSTATUS = -1073414021i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_CACHE: super::super::Foundation::NTSTATUS = -1073414083i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_CPU: super::super::Foundation::NTSTATUS = -1073414030i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_CPU_BUS: super::super::Foundation::NTSTATUS = -1073414079i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_MAS: super::super::Foundation::NTSTATUS = -1073414075i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_MEM_1_2: super::super::Foundation::NTSTATUS = -1073414071i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_MEM_1_2_5: super::super::Foundation::NTSTATUS = -1073414069i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_MEM_1_2_5_4: super::super::Foundation::NTSTATUS = -1073414067i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_MEM_UNKNOWN: super::super::Foundation::NTSTATUS = -1073414073i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_MASTER_ABORT: super::super::Foundation::NTSTATUS = -1073414055i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_MASTER_ABORT_NO_INFO: super::super::Foundation::NTSTATUS = -1073414053i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_PARITY: super::super::Foundation::NTSTATUS = -1073414063i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_PARITY_NO_INFO: super::super::Foundation::NTSTATUS = -1073414061i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_SERR: super::super::Foundation::NTSTATUS = -1073414059i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_SERR_NO_INFO: super::super::Foundation::NTSTATUS = -1073414057i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_TIMEOUT: super::super::Foundation::NTSTATUS = -1073414051i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_TIMEOUT_NO_INFO: super::super::Foundation::NTSTATUS = -1073414049i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_BUS_UNKNOWN: super::super::Foundation::NTSTATUS = -1073414047i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PCI_DEVICE: super::super::Foundation::NTSTATUS = -1073414045i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_PLATFORM_SPECIFIC: super::super::Foundation::NTSTATUS = -1073414041i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_REGISTER_FILE: super::super::Foundation::NTSTATUS = -1073414077i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_SMBIOS: super::super::Foundation::NTSTATUS = -1073414043i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_SYSTEM_EVENT: super::super::Foundation::NTSTATUS = -1073414065i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_TLB: super::super::Foundation::NTSTATUS = -1073414081i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_UNKNOWN: super::super::Foundation::NTSTATUS = -1073414039i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_ERROR_UNKNOWN_NO_CPU: super::super::Foundation::NTSTATUS = -1073414037i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_EXTERNAL_ERROR: super::super::Foundation::NTSTATUS = -1073414017i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_FRC_ERROR: super::super::Foundation::NTSTATUS = -1073414016i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_INFO_CPU_THERMAL_THROTTLING_REMOVED: super::super::Foundation::NTSTATUS = 1074069616i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_INFO_MEMORY_PAGE_MARKED_BAD: super::super::Foundation::NTSTATUS = 1074069620i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_INFO_NO_MORE_CORRECTED_ERROR_LOGS: super::super::Foundation::NTSTATUS = 1074069619i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_INTERNALTIMER_ERROR: super::super::Foundation::NTSTATUS = -1073414020i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_MEMORYHIERARCHY_ERROR: super::super::Foundation::NTSTATUS = -1073414024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_MICROCODE_ROM_PARITY_ERROR: super::super::Foundation::NTSTATUS = -1073414018i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_TLB_ERROR: super::super::Foundation::NTSTATUS = -1073414023i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_CACHE: super::super::Foundation::NTSTATUS = -2147155908i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_CMC_THRESHOLD_EXCEEDED: super::super::Foundation::NTSTATUS = -2147155859i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_CPE_THRESHOLD_EXCEEDED: super::super::Foundation::NTSTATUS = -2147155858i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_CPU: super::super::Foundation::NTSTATUS = -2147155855i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_CPU_BUS: super::super::Foundation::NTSTATUS = -2147155904i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_CPU_THERMAL_THROTTLED: super::super::Foundation::NTSTATUS = -2147155857i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_MAS: super::super::Foundation::NTSTATUS = -2147155900i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_MEM_1_2: super::super::Foundation::NTSTATUS = -2147155896i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_MEM_1_2_5: super::super::Foundation::NTSTATUS = -2147155894i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_MEM_1_2_5_4: super::super::Foundation::NTSTATUS = -2147155892i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_MEM_UNKNOWN: super::super::Foundation::NTSTATUS = -2147155898i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_MASTER_ABORT: super::super::Foundation::NTSTATUS = -2147155880i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_MASTER_ABORT_NO_INFO: super::super::Foundation::NTSTATUS = -2147155878i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_PARITY: super::super::Foundation::NTSTATUS = -2147155888i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_PARITY_NO_INFO: super::super::Foundation::NTSTATUS = -2147155886i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_SERR: super::super::Foundation::NTSTATUS = -2147155884i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_SERR_NO_INFO: super::super::Foundation::NTSTATUS = -2147155882i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_TIMEOUT: super::super::Foundation::NTSTATUS = -2147155876i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_TIMEOUT_NO_INFO: super::super::Foundation::NTSTATUS = -2147155874i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_BUS_UNKNOWN: super::super::Foundation::NTSTATUS = -2147155872i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PCI_DEVICE: super::super::Foundation::NTSTATUS = -2147155870i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_PLATFORM_SPECIFIC: super::super::Foundation::NTSTATUS = -2147155866i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_REGISTER_FILE: super::super::Foundation::NTSTATUS = -2147155902i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_SMBIOS: super::super::Foundation::NTSTATUS = -2147155868i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_SYSTEM_EVENT: super::super::Foundation::NTSTATUS = -2147155890i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_TLB: super::super::Foundation::NTSTATUS = -2147155906i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_UNKNOWN: super::super::Foundation::NTSTATUS = -2147155864i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const MCA_WARNING_UNKNOWN_NO_CPU: super::super::Foundation::NTSTATUS = -2147155862i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEMORY_ALLOCATION_ALIGNMENT: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub Type: MEM_DEDICATED_ATTRIBUTE_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl ::core::marker::Copy for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {}
impl ::core::clone::Clone for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub SizeOfInformation: u32,
    pub Flags: u32,
    pub AttributesOffset: u32,
    pub AttributeCount: u32,
    pub Reserved: u32,
    pub TypeId: u64,
}
impl ::core::marker::Copy for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEMORY_PARTITION_MODIFY_ACCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEMORY_PARTITION_QUERY_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEMORY_PRIORITY_LOWEST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_4MB_PAGES: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_COALESCE_PLACEHOLDERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type MEM_DEDICATED_ATTRIBUTE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_DIFFERENT_IMAGE_BASE_OK: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_EC_CODE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_GRAPHICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_NONPAGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_NONPAGED_HUGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_NONPAGED_LARGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_SOFT_FAULT_PAGES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_EXTENDED_PARAMETER_ZERO_PAGES_OPTIONAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_PHYSICAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_ROTATE: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type MEM_SECTION_EXTENDED_PARAMETER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_TOP_DOWN: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MEM_WRITE_WATCH: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MESSAGE_RESOURCE_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MESSAGE_RESOURCE_UTF8: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MINCHAR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MINLONG: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MINSHORT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MIN_UCSCHAR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type MONITOR_DISPLAY_STATE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerMonitorOff: MONITOR_DISPLAY_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerMonitorOn: MONITOR_DISPLAY_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerMonitorDim: MONITOR_DISPLAY_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MS_PPM_SOFTWARE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MUTANT_QUERY_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct NETWORK_APP_INSTANCE_EA {
    pub AppInstanceID: ::windows_sys::core::GUID,
    pub CsvFlags: u32,
}
impl ::core::marker::Copy for NETWORK_APP_INSTANCE_EA {}
impl ::core::clone::Clone for NETWORK_APP_INSTANCE_EA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NLS_VALID_LOCALE_MASK: u32 = 1048575u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NONVOL_FP_NUMREG_ARM64: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NONVOL_INT_NUMREG_ARM64: u32 = 11u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct NON_PAGED_DEBUG_INFO {
    pub Signature: u16,
    pub Flags: u16,
    pub Size: u32,
    pub Machine: u16,
    pub Characteristics: u16,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub SizeOfImage: u32,
    pub ImageBase: u64,
}
impl ::core::marker::Copy for NON_PAGED_DEBUG_INFO {}
impl ::core::clone::Clone for NON_PAGED_DEBUG_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NON_PAGED_DEBUG_SIGNATURE: u32 = 18766u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct NOTIFY_USER_POWER_SETTING {
    pub Guid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for NOTIFY_USER_POWER_SETTING {}
impl ::core::clone::Clone for NOTIFY_USER_POWER_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NO_SUBGROUP_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4272111934, data2: 32261, data3: 18705, data4: [154, 113, 112, 3, 49, 241, 194, 148] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct NT_TIB32 {
    pub ExceptionList: u32,
    pub StackBase: u32,
    pub StackLimit: u32,
    pub SubSystemTib: u32,
    pub Anonymous: NT_TIB32_0,
    pub ArbitraryUserPointer: u32,
    pub Self_: u32,
}
impl ::core::marker::Copy for NT_TIB32 {}
impl ::core::clone::Clone for NT_TIB32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union NT_TIB32_0 {
    pub FiberData: u32,
    pub Version: u32,
}
impl ::core::marker::Copy for NT_TIB32_0 {}
impl ::core::clone::Clone for NT_TIB32_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct NT_TIB64 {
    pub ExceptionList: u64,
    pub StackBase: u64,
    pub StackLimit: u64,
    pub SubSystemTib: u64,
    pub Anonymous: NT_TIB64_0,
    pub ArbitraryUserPointer: u64,
    pub Self_: u64,
}
impl ::core::marker::Copy for NT_TIB64 {}
impl ::core::clone::Clone for NT_TIB64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union NT_TIB64_0 {
    pub FiberData: u64,
    pub Version: u32,
}
impl ::core::marker::Copy for NT_TIB64_0 {}
impl ::core::clone::Clone for NT_TIB64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NUMA_NO_PREFERRED_NODE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NUM_DISCHARGE_POLICIES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const N_BTMASK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const N_BTSHFT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const N_TMASK: u32 = 48u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const N_TMASK1: u32 = 192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const N_TMASK2: u32 = 240u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const N_TSHIFT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME: &str = "OutOfProcessFunctionTableCallback";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PACKEDEVENTINFO {
    pub ulSize: u32,
    pub ulNumEventsForLogFile: u32,
    pub ulOffsets: [u32; 1],
}
impl ::core::marker::Copy for PACKEDEVENTINFO {}
impl ::core::clone::Clone for PACKEDEVENTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_D0_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_D1_SUPPORTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_D2_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_D3_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_WAKE_FROM_D0_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_WAKE_FROM_D1_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_WAKE_FROM_D2_SUPPORTED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_WAKE_FROM_D3_SUPPORTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PDCAP_WARM_EJECT_SUPPORTED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFORMANCE_DATA_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFSTATE_POLICY_CHANGE_DECREASE_MAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFSTATE_POLICY_CHANGE_IDEAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFSTATE_POLICY_CHANGE_INCREASE_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFSTATE_POLICY_CHANGE_ROCKET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PERFSTATE_POLICY_CHANGE_SINGLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
pub type PEXCEPTION_FILTER = ::core::option::Option<unsafe extern "system" fn(exceptionpointers: *mut super::Diagnostics::Debug::EXCEPTION_POINTERS, establisherframe: *const ::core::ffi::c_void) -> i32>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_ALPHA_BYTE_INSTRUCTIONS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_ARM_NEON_INSTRUCTIONS_AVAILABLE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE: u32 = 43u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE: u32 = 44u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_AVX2_INSTRUCTIONS_AVAILABLE: u32 = 40u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_AVX512F_INSTRUCTIONS_AVAILABLE: u32 = 41u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_AVX_INSTRUCTIONS_AVAILABLE: u32 = 39u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_ERMS_AVAILABLE: u32 = 42u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_MONITORX_INSTRUCTION_AVAILABLE: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_NON_TEMPORAL_LEVEL_ALL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_PPC_MOVEMEM_64BIT_OK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_RDPID_INSTRUCTION_AVAILABLE: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_RDRAND_INSTRUCTION_AVAILABLE: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_RDTSCP_INSTRUCTION_AVAILABLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_SSE4_1_INSTRUCTIONS_AVAILABLE: u32 = 37u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_SSE4_2_INSTRUCTIONS_AVAILABLE: u32 = 38u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_SSE_DAZ_MODE_AVAILABLE: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_SSSE3_INSTRUCTIONS_AVAILABLE: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_TEMPORAL_LEVEL_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_TEMPORAL_LEVEL_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PF_TEMPORAL_LEVEL_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type PIMAGE_TLS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dllhandle: *mut ::core::ffi::c_void, reason: u32, reserved: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POLICY_AUDIT_SUBCATEGORY_COUNT: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(process: super::super::Foundation::HANDLE, tableaddress: *const ::core::ffi::c_void, entries: *mut u32, functions: *mut *mut super::Diagnostics::Debug::IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> u32>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Diagnostics_Debug\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug"))]
pub type POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(process: super::super::Foundation::HANDLE, tableaddress: *const ::core::ffi::c_void, entries: *mut u32, functions: *mut *mut super::Diagnostics::Debug::IMAGE_RUNTIME_FUNCTION_ENTRY) -> u32>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_INDEX_HIBERNATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_INDEX_NOTHING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_INDEX_SHUTDOWN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_INDEX_SLEEP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_INDEX_TURN_OFF_THE_DISPLAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_VALUE_HIBERNATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_VALUE_NOTHING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_VALUE_SHUTDOWN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_VALUE_SLEEP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWERBUTTON_ACTION_VALUE_TURN_OFF_THE_DISPLAY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_ACPI_CRITICAL: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_ACPI_USER_NOTIFY: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_CRITICAL: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_DIRECTED_DRIPS: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_DISABLE_WAKES: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_DOZE_TO_HIBERNATE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_HIBERBOOT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_LIGHTEST_FIRST: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_LOCK_CONSOLE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_OVERRIDE_APPS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_PSEUDO_TRANSITION: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_QUERY_ALLOWED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_UI_ALLOWED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_ACTION_USER_NOTIFY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_CONNECTIVITY_IN_STANDBY_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_CONNECTIVITY_IN_STANDBY_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_CONNECTIVITY_IN_STANDBY_SYSTEM_MANAGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_DEVICE_IDLE_POLICY_CONSERVATIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_DEVICE_IDLE_POLICY_PERFORMANCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_DISCONNECTED_STANDBY_MODE_AGGRESSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_DISCONNECTED_STANDBY_MODE_NORMAL: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct POWER_IDLE_RESILIENCY {
    pub CoalescingTimeout: u32,
    pub IdleResiliencyPeriod: u32,
}
impl ::core::marker::Copy for POWER_IDLE_RESILIENCY {}
impl ::core::clone::Clone for POWER_IDLE_RESILIENCY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_MONITOR_INVOCATION {
    pub Console: super::super::Foundation::BOOLEAN,
    pub RequestReason: POWER_MONITOR_REQUEST_REASON,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POWER_MONITOR_INVOCATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POWER_MONITOR_INVOCATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type POWER_MONITOR_REQUEST_REASON = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUnknown: POWER_MONITOR_REQUEST_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPowerButton: POWER_MONITOR_REQUEST_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonRemoteConnection: POWER_MONITOR_REQUEST_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonScMonitorpower: POWER_MONITOR_REQUEST_REASON = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInput: POWER_MONITOR_REQUEST_REASON = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonAcDcDisplayBurst: POWER_MONITOR_REQUEST_REASON = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserDisplayBurst: POWER_MONITOR_REQUEST_REASON = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPoSetSystemState: POWER_MONITOR_REQUEST_REASON = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonSetThreadExecutionState: POWER_MONITOR_REQUEST_REASON = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonFullWake: POWER_MONITOR_REQUEST_REASON = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonSessionUnlock: POWER_MONITOR_REQUEST_REASON = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonScreenOffRequest: POWER_MONITOR_REQUEST_REASON = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonIdleTimeout: POWER_MONITOR_REQUEST_REASON = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPolicyChange: POWER_MONITOR_REQUEST_REASON = 13i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonSleepButton: POWER_MONITOR_REQUEST_REASON = 14i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonLid: POWER_MONITOR_REQUEST_REASON = 15i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonBatteryCountChange: POWER_MONITOR_REQUEST_REASON = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonGracePeriod: POWER_MONITOR_REQUEST_REASON = 17i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPnP: POWER_MONITOR_REQUEST_REASON = 18i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonDP: POWER_MONITOR_REQUEST_REASON = 19i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonSxTransition: POWER_MONITOR_REQUEST_REASON = 20i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonSystemIdle: POWER_MONITOR_REQUEST_REASON = 21i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonNearProximity: POWER_MONITOR_REQUEST_REASON = 22i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonThermalStandby: POWER_MONITOR_REQUEST_REASON = 23i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonResumePdc: POWER_MONITOR_REQUEST_REASON = 24i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonResumeS4: POWER_MONITOR_REQUEST_REASON = 25i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonTerminal: POWER_MONITOR_REQUEST_REASON = 26i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignal: POWER_MONITOR_REQUEST_REASON = 27i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonAcDcDisplayBurstSuppressed: POWER_MONITOR_REQUEST_REASON = 28i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonSystemStateEntered: POWER_MONITOR_REQUEST_REASON = 29i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonWinrt: POWER_MONITOR_REQUEST_REASON = 30i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputKeyboard: POWER_MONITOR_REQUEST_REASON = 31i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputMouse: POWER_MONITOR_REQUEST_REASON = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputTouchpad: POWER_MONITOR_REQUEST_REASON = 33i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputPen: POWER_MONITOR_REQUEST_REASON = 34i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputAccelerometer: POWER_MONITOR_REQUEST_REASON = 35i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputHid: POWER_MONITOR_REQUEST_REASON = 36i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputPoUserPresent: POWER_MONITOR_REQUEST_REASON = 37i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputSessionSwitch: POWER_MONITOR_REQUEST_REASON = 38i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputInitialization: POWER_MONITOR_REQUEST_REASON = 39i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignalWindowsMobilePwrNotif: POWER_MONITOR_REQUEST_REASON = 40i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignalWindowsMobileShell: POWER_MONITOR_REQUEST_REASON = 41i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignalHeyCortana: POWER_MONITOR_REQUEST_REASON = 42i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignalHolographicShell: POWER_MONITOR_REQUEST_REASON = 43i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignalFingerprint: POWER_MONITOR_REQUEST_REASON = 44i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonDirectedDrips: POWER_MONITOR_REQUEST_REASON = 45i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonDim: POWER_MONITOR_REQUEST_REASON = 46i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonBuiltinPanel: POWER_MONITOR_REQUEST_REASON = 47i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonDisplayRequiredUnDim: POWER_MONITOR_REQUEST_REASON = 48i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonBatteryCountChangeSuppressed: POWER_MONITOR_REQUEST_REASON = 49i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonResumeModernStandby: POWER_MONITOR_REQUEST_REASON = 50i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonTerminalInit: POWER_MONITOR_REQUEST_REASON = 51i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonPdcSignalSensorsHumanPresence: POWER_MONITOR_REQUEST_REASON = 52i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonBatteryPreCritical: POWER_MONITOR_REQUEST_REASON = 53i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonUserInputTouch: POWER_MONITOR_REQUEST_REASON = 54i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestReasonMax: POWER_MONITOR_REQUEST_REASON = 55i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type POWER_MONITOR_REQUEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestTypeOff: POWER_MONITOR_REQUEST_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestTypeOnAndPresent: POWER_MONITOR_REQUEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MonitorRequestTypeToggleOn: POWER_MONITOR_REQUEST_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_PLATFORM_INFORMATION {
    pub AoAc: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POWER_PLATFORM_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POWER_PLATFORM_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_REQUEST_CONTEXT_VERSION: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    pub IsAllowed: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_SESSION_CONNECT {
    pub Connected: super::super::Foundation::BOOLEAN,
    pub Console: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POWER_SESSION_CONNECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POWER_SESSION_CONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_SESSION_RIT_STATE {
    pub Active: super::super::Foundation::BOOLEAN,
    pub LastInputTime: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POWER_SESSION_RIT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POWER_SESSION_RIT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct POWER_SESSION_TIMEOUTS {
    pub InputTimeout: u32,
    pub DisplayTimeout: u32,
}
impl ::core::marker::Copy for POWER_SESSION_TIMEOUTS {}
impl ::core::clone::Clone for POWER_SESSION_TIMEOUTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct POWER_SESSION_WINLOGON {
    pub SessionId: u32,
    pub Console: super::super::Foundation::BOOLEAN,
    pub Locked: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POWER_SESSION_WINLOGON {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POWER_SESSION_WINLOGON {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type POWER_SETTING_ALTITUDE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_GROUP_POLICY: POWER_SETTING_ALTITUDE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_USER: POWER_SETTING_ALTITUDE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_RUNTIME_OVERRIDE: POWER_SETTING_ALTITUDE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_PROVISIONING: POWER_SETTING_ALTITUDE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_OEM_CUSTOMIZATION: POWER_SETTING_ALTITUDE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_INTERNAL_OVERRIDE: POWER_SETTING_ALTITUDE = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ALTITUDE_OS_DEFAULT: POWER_SETTING_ALTITUDE = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_SETTING_VALUE_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_SYSTEM_MAXIMUM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const POWER_USER_NOTIFY_FORCED_SHUTDOWN: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct POWER_USER_PRESENCE {
    pub UserPresence: POWER_USER_PRESENCE_TYPE,
}
impl ::core::marker::Copy for POWER_USER_PRESENCE {}
impl ::core::clone::Clone for POWER_USER_PRESENCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type POWER_USER_PRESENCE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UserNotPresent: POWER_USER_PRESENCE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UserPresent: POWER_USER_PRESENCE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UserUnknown: POWER_USER_PRESENCE_TYPE = 255i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PO_THROTTLE_ADAPTIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PO_THROTTLE_CONSTANT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PO_THROTTLE_DEGRADE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PO_THROTTLE_MAXIMUM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PO_THROTTLE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_ACPI1C2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_ACPI1C3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_ACPI1TSTATES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_CPC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_CSD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_CST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_LPI: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_OSC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PCCH: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PCCP: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PCT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PDC: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PPC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PSD: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PSS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_PTC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_TPC: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_TSD: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_TSS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_FIRMWARE_XPSS: u32 = 128u32;
pub const PPM_IDLESTATES_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3121843728, data2: 57936, data3: 19159, data4: [134, 22, 207, 26, 122, 212, 16, 231] };
pub const PPM_IDLESTATE_CHANGE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1211694671, data2: 63260, data3: 20049, data4: [158, 204, 132, 48, 167, 172, 76, 108] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_IDLESTATE_EVENT {
    pub NewState: u32,
    pub OldState: u32,
    pub Processors: u64,
}
impl ::core::marker::Copy for PPM_IDLESTATE_EVENT {}
impl ::core::clone::Clone for PPM_IDLESTATE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_IDLE_ACCOUNTING {
    pub StateCount: u32,
    pub TotalTransitions: u32,
    pub ResetCount: u32,
    pub StartTime: u64,
    pub State: [PPM_IDLE_STATE_ACCOUNTING; 1],
}
impl ::core::marker::Copy for PPM_IDLE_ACCOUNTING {}
impl ::core::clone::Clone for PPM_IDLE_ACCOUNTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_IDLE_ACCOUNTING_EX {
    pub StateCount: u32,
    pub TotalTransitions: u32,
    pub ResetCount: u32,
    pub AbortCount: u32,
    pub StartTime: u64,
    pub State: [PPM_IDLE_STATE_ACCOUNTING_EX; 1],
}
impl ::core::marker::Copy for PPM_IDLE_ACCOUNTING_EX {}
impl ::core::clone::Clone for PPM_IDLE_ACCOUNTING_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PPM_IDLE_ACCOUNTING_EX_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3598368057, data2: 33272, data3: 19038, data4: [129, 82, 114, 227, 30, 201, 18, 238] };
pub const PPM_IDLE_ACCOUNTING_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3802296184, data2: 44551, data3: 20192, data4: [163, 15, 206, 84, 245, 90, 148, 205] };
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_IDLE_IMPLEMENTATION_CSTATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_IDLE_IMPLEMENTATION_LPISTATES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_IDLE_IMPLEMENTATION_MICROPEP: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_IDLE_IMPLEMENTATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_IDLE_IMPLEMENTATION_PEP: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_IDLE_STATE_ACCOUNTING {
    pub IdleTransitions: u32,
    pub FailedTransitions: u32,
    pub InvalidBucketIndex: u32,
    pub TotalTime: u64,
    pub IdleTimeBuckets: [u32; 6],
}
impl ::core::marker::Copy for PPM_IDLE_STATE_ACCOUNTING {}
impl ::core::clone::Clone for PPM_IDLE_STATE_ACCOUNTING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_IDLE_STATE_ACCOUNTING_EX {
    pub TotalTime: u64,
    pub IdleTransitions: u32,
    pub FailedTransitions: u32,
    pub InvalidBucketIndex: u32,
    pub MinTimeUs: u32,
    pub MaxTimeUs: u32,
    pub CancelledTransitions: u32,
    pub IdleTimeBuckets: [PPM_IDLE_STATE_BUCKET_EX; 16],
}
impl ::core::marker::Copy for PPM_IDLE_STATE_ACCOUNTING_EX {}
impl ::core::clone::Clone for PPM_IDLE_STATE_ACCOUNTING_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_IDLE_STATE_BUCKET_EX {
    pub TotalTimeUs: u64,
    pub MinTimeUs: u32,
    pub MaxTimeUs: u32,
    pub Count: u32,
}
impl ::core::marker::Copy for PPM_IDLE_STATE_BUCKET_EX {}
impl ::core::clone::Clone for PPM_IDLE_STATE_BUCKET_EX {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PPM_PERFMON_PERFSTATE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2144437842, data2: 3326, data3: 16594, data4: [176, 161, 11, 6, 106, 135, 117, 158] };
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_PERFORMANCE_IMPLEMENTATION_CPPC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_PERFORMANCE_IMPLEMENTATION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_PERFORMANCE_IMPLEMENTATION_PCCV1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_PERFORMANCE_IMPLEMENTATION_PEP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PPM_PERFORMANCE_IMPLEMENTATION_PSTATES: u32 = 1u32;
pub const PPM_PERFSTATES_DATA_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1460194336, data2: 32064, data3: 19444, data4: [180, 170, 43, 1, 51, 141, 1, 38] };
pub const PPM_PERFSTATE_CHANGE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2779983325, data2: 32569, data3: 19132, data4: [184, 146, 144, 14, 67, 181, 158, 187] };
pub const PPM_PERFSTATE_DOMAIN_CHANGE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2573101951, data2: 54867, data3: 18810, data4: [185, 120, 54, 163, 12, 41, 191, 1] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_PERFSTATE_DOMAIN_EVENT {
    pub State: u32,
    pub Latency: u32,
    pub Speed: u32,
    pub Processors: u64,
}
impl ::core::marker::Copy for PPM_PERFSTATE_DOMAIN_EVENT {}
impl ::core::clone::Clone for PPM_PERFSTATE_DOMAIN_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_PERFSTATE_EVENT {
    pub State: u32,
    pub Status: u32,
    pub Latency: u32,
    pub Speed: u32,
    pub Processor: u32,
}
impl ::core::marker::Copy for PPM_PERFSTATE_EVENT {}
impl ::core::clone::Clone for PPM_PERFSTATE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_THERMALCHANGE_EVENT {
    pub ThermalConstraint: u32,
    pub Processors: u64,
}
impl ::core::marker::Copy for PPM_THERMALCHANGE_EVENT {}
impl ::core::clone::Clone for PPM_THERMALCHANGE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PPM_THERMALCONSTRAINT_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2823996104, data2: 6732, data3: 16955, data4: [140, 44, 243, 13, 130, 147, 26, 136] };
pub const PPM_THERMAL_POLICY_CHANGE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1223915448, data2: 26752, data3: 19579, data4: [139, 220, 56, 1, 118, 198, 101, 77] };
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_THERMAL_POLICY_EVENT {
    pub Mode: u8,
    pub Processors: u64,
}
impl ::core::marker::Copy for PPM_THERMAL_POLICY_EVENT {}
impl ::core::clone::Clone for PPM_THERMAL_POLICY_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_IDLE_STATE {
    pub Latency: u32,
    pub Power: u32,
    pub TimeCheck: u32,
    pub PromotePercent: u8,
    pub DemotePercent: u8,
    pub StateType: u8,
    pub Reserved: u8,
    pub StateFlags: u32,
    pub Context: u32,
    pub IdleHandler: u32,
    pub Reserved1: u32,
}
impl ::core::marker::Copy for PPM_WMI_IDLE_STATE {}
impl ::core::clone::Clone for PPM_WMI_IDLE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_IDLE_STATES {
    pub Type: u32,
    pub Count: u32,
    pub TargetState: u32,
    pub OldState: u32,
    pub TargetProcessors: u64,
    pub State: [PPM_WMI_IDLE_STATE; 1],
}
impl ::core::marker::Copy for PPM_WMI_IDLE_STATES {}
impl ::core::clone::Clone for PPM_WMI_IDLE_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_IDLE_STATES_EX {
    pub Type: u32,
    pub Count: u32,
    pub TargetState: u32,
    pub OldState: u32,
    pub TargetProcessors: *mut ::core::ffi::c_void,
    pub State: [PPM_WMI_IDLE_STATE; 1],
}
impl ::core::marker::Copy for PPM_WMI_IDLE_STATES_EX {}
impl ::core::clone::Clone for PPM_WMI_IDLE_STATES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_LEGACY_PERFSTATE {
    pub Frequency: u32,
    pub Flags: u32,
    pub PercentFrequency: u32,
}
impl ::core::marker::Copy for PPM_WMI_LEGACY_PERFSTATE {}
impl ::core::clone::Clone for PPM_WMI_LEGACY_PERFSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_PERF_STATE {
    pub Frequency: u32,
    pub Power: u32,
    pub PercentFrequency: u8,
    pub IncreaseLevel: u8,
    pub DecreaseLevel: u8,
    pub Type: u8,
    pub IncreaseTime: u32,
    pub DecreaseTime: u32,
    pub Control: u64,
    pub Status: u64,
    pub HitCount: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub Reserved3: u64,
}
impl ::core::marker::Copy for PPM_WMI_PERF_STATE {}
impl ::core::clone::Clone for PPM_WMI_PERF_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_PERF_STATES {
    pub Count: u32,
    pub MaxFrequency: u32,
    pub CurrentState: u32,
    pub MaxPerfState: u32,
    pub MinPerfState: u32,
    pub LowestPerfState: u32,
    pub ThermalConstraint: u32,
    pub BusyAdjThreshold: u8,
    pub PolicyType: u8,
    pub Type: u8,
    pub Reserved: u8,
    pub TimerInterval: u32,
    pub TargetProcessors: u64,
    pub PStateHandler: u32,
    pub PStateContext: u32,
    pub TStateHandler: u32,
    pub TStateContext: u32,
    pub FeedbackHandler: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub State: [PPM_WMI_PERF_STATE; 1],
}
impl ::core::marker::Copy for PPM_WMI_PERF_STATES {}
impl ::core::clone::Clone for PPM_WMI_PERF_STATES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PPM_WMI_PERF_STATES_EX {
    pub Count: u32,
    pub MaxFrequency: u32,
    pub CurrentState: u32,
    pub MaxPerfState: u32,
    pub MinPerfState: u32,
    pub LowestPerfState: u32,
    pub ThermalConstraint: u32,
    pub BusyAdjThreshold: u8,
    pub PolicyType: u8,
    pub Type: u8,
    pub Reserved: u8,
    pub TimerInterval: u32,
    pub TargetProcessors: *mut ::core::ffi::c_void,
    pub PStateHandler: u32,
    pub PStateContext: u32,
    pub TStateHandler: u32,
    pub TStateContext: u32,
    pub FeedbackHandler: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub State: [PPM_WMI_PERF_STATE; 1],
}
impl ::core::marker::Copy for PPM_WMI_PERF_STATES_EX {}
impl ::core::clone::Clone for PPM_WMI_PERF_STATES_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRAGMA_DEPRECATED_DDK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRIVILEGE_SET_ALL_NECESSARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ALPHA_21064: u32 = 21064u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_AMD_X8664: u32 = 8664u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_ALPHA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_ALPHA64: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_ARM64: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_IA32_ON_ARM64: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_IA32_ON_WIN64: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_MIPS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_MSIL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_NEUTRAL: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_PPC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARCHITECTURE_SHX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARM720: u32 = 1824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARM820: u32 = 2080u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARM920: u32 = 2336u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_ARM_7TDMI: u32 = 70001u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_DUTY_CYCLING_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_DUTY_CYCLING_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_HITACHI_SH3: u32 = 10003u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_HITACHI_SH3E: u32 = 10004u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_HITACHI_SH4: u32 = 10005u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESSOR_IDLESTATE_INFO {
    pub TimeCheck: u32,
    pub DemotePercent: u8,
    pub PromotePercent: u8,
    pub Spare: [u8; 2],
}
impl ::core::marker::Copy for PROCESSOR_IDLESTATE_INFO {}
impl ::core::clone::Clone for PROCESSOR_IDLESTATE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESSOR_IDLESTATE_POLICY {
    pub Revision: u16,
    pub Flags: PROCESSOR_IDLESTATE_POLICY_0,
    pub PolicyCount: u32,
    pub Policy: [PROCESSOR_IDLESTATE_INFO; 3],
}
impl ::core::marker::Copy for PROCESSOR_IDLESTATE_POLICY {}
impl ::core::clone::Clone for PROCESSOR_IDLESTATE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESSOR_IDLESTATE_POLICY_0 {
    pub AsWORD: u16,
    pub Anonymous: PROCESSOR_IDLESTATE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESSOR_IDLESTATE_POLICY_0 {}
impl ::core::clone::Clone for PROCESSOR_IDLESTATE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESSOR_IDLESTATE_POLICY_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for PROCESSOR_IDLESTATE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESSOR_IDLESTATE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_IDLESTATE_POLICY_COUNT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_INTEL_386: u32 = 386u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_INTEL_486: u32 = 486u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_INTEL_IA64: u32 = 2200u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_INTEL_PENTIUM: u32 = 586u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_MIPS_R4000: u32 = 4000u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_MOTOROLA_821: u32 = 821u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_OPTIL: u32 = 18767u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESSOR_PERFSTATE_POLICY {
    pub Revision: u32,
    pub MaxThrottle: u8,
    pub MinThrottle: u8,
    pub BusyAdjThreshold: u8,
    pub Anonymous: PROCESSOR_PERFSTATE_POLICY_0,
    pub TimeCheck: u32,
    pub IncreaseTime: u32,
    pub DecreaseTime: u32,
    pub IncreasePercent: u32,
    pub DecreasePercent: u32,
}
impl ::core::marker::Copy for PROCESSOR_PERFSTATE_POLICY {}
impl ::core::clone::Clone for PROCESSOR_PERFSTATE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESSOR_PERFSTATE_POLICY_0 {
    pub Spare: u8,
    pub Flags: PROCESSOR_PERFSTATE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESSOR_PERFSTATE_POLICY_0 {}
impl ::core::clone::Clone for PROCESSOR_PERFSTATE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESSOR_PERFSTATE_POLICY_0_0 {
    pub AsBYTE: u8,
    pub Anonymous: PROCESSOR_PERFSTATE_POLICY_0_0_0,
}
impl ::core::marker::Copy for PROCESSOR_PERFSTATE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESSOR_PERFSTATE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESSOR_PERFSTATE_POLICY_0_0_0 {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for PROCESSOR_PERFSTATE_POLICY_0_0_0 {}
impl ::core::clone::Clone for PROCESSOR_PERFSTATE_POLICY_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_AUTONOMOUS_MODE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_AUTONOMOUS_MODE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE_AT_GUARANTEED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE_AT_GUARANTEED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_ENABLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_MODE_MAX: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_POLICY_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_BOOST_POLICY_MAX: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_ENERGY_PREFERENCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_MAXIMUM_ACTIVITY_WINDOW: u32 = 1270000000u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_MINIMUM_ACTIVITY_WINDOW: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PERF_PERFORMANCE_PREFERENCE: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PPC_601: u32 = 601u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PPC_603: u32 = 603u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PPC_604: u32 = 604u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_PPC_620: u32 = 620u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_SHx_SH3: u32 = 103u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_SHx_SH4: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_STRONGARM: u32 = 2577u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_THROTTLE_AUTOMATIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_THROTTLE_DISABLED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESSOR_THROTTLE_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_HEAP_ENTRY_BUSY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_HEAP_ENTRY_DDESHARE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_HEAP_ENTRY_MOVEABLE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_HEAP_REGION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_HEAP_SEG_ALLOC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_ASLR_POLICY {
    pub Anonymous: PROCESS_MITIGATION_ASLR_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_ASLR_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_ASLR_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_ASLR_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_ASLR_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_ASLR_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_ASLR_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_ASLR_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_ASLR_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_ASLR_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    pub Anonymous: PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    pub Anonymous: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_MITIGATION_DEP_POLICY {
    pub Anonymous: PROCESS_MITIGATION_DEP_POLICY_0,
    pub Permanent: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_MITIGATION_DEP_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_MITIGATION_DEP_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROCESS_MITIGATION_DEP_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_DEP_POLICY_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_MITIGATION_DEP_POLICY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_MITIGATION_DEP_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_MITIGATION_DEP_POLICY_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_MITIGATION_DEP_POLICY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_MITIGATION_DEP_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_FONT_DISABLE_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_FONT_DISABLE_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    pub Anonymous: PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    pub Anonymous: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    pub Anonymous: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    pub Anonymous: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    pub Anonymous: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0 {}
impl ::core::clone::Clone for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROCESS_TRUST_LABEL_SECURITY_INFORMATION: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROC_IDLE_BUCKET_COUNT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PROC_IDLE_BUCKET_COUNT_EX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_ARM64_SERVER: u32 = 120u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_AZURESTACKHCI_SERVER_CORE: u32 = 406u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_AZURE_NANO_SERVER: u32 = 169u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_AZURE_SERVER_CLOUDHOST: u32 = 199u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_AZURE_SERVER_CLOUDMOS: u32 = 200u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_AZURE_SERVER_CORE: u32 = 168u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUD: u32 = 178u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUDE: u32 = 183u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUDEDITION: u32 = 203u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUDEDITIONN: u32 = 202u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUDEN: u32 = 186u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUDN: u32 = 179u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUD_HOST_INFRASTRUCTURE_SERVER: u32 = 124u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CLOUD_STORAGE_SERVER: u32 = 110u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CONNECTED_CAR: u32 = 117u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CORE_ARM: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CORE_CONNECTED: u32 = 111u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CORE_CONNECTED_COUNTRYSPECIFIC: u32 = 116u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CORE_CONNECTED_N: u32 = 113u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_CORE_CONNECTED_SINGLELANGUAGE: u32 = 115u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_DATACENTER_EVALUATION_SERVER_CORE: u32 = 159u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_DATACENTER_NANO_SERVER: u32 = 143u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_DATACENTER_SERVER_AZURE_EDITION: u32 = 407u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_DATACENTER_SERVER_CORE_AZURE_EDITION: u32 = 408u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_DATACENTER_WS_SERVER_CORE: u32 = 147u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED: u32 = 65u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_A: u32 = 88u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_AUTOMOTIVE: u32 = 85u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_E: u32 = 90u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_EVAL: u32 = 107u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_E_EVAL: u32 = 108u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_INDUSTRY: u32 = 89u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_INDUSTRY_A: u32 = 86u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_INDUSTRY_A_E: u32 = 92u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_INDUSTRY_E: u32 = 91u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_INDUSTRY_EVAL: u32 = 105u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_EMBEDDED_INDUSTRY_E_EVAL: u32 = 106u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_ENTERPRISEG: u32 = 171u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_ENTERPRISEGN: u32 = 172u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION: u32 = 140u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION_N: u32 = 141u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_HOLOGRAPHIC: u32 = 135u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_HOLOGRAPHIC_BUSINESS: u32 = 136u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_HUBOS: u32 = 180u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_INDUSTRY_HANDHELD: u32 = 118u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_IOTEDGEOS: u32 = 187u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_IOTENTERPRISE: u32 = 188u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_IOTENTERPRISES: u32 = 191u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_IOTOS: u32 = 185u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_LITE: u32 = 189u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_NANO_SERVER: u32 = 109u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_ONECOREUPDATEOS: u32 = 182u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PPI_PRO: u32 = 119u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PROFESSIONAL_EMBEDDED: u32 = 58u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PROFESSIONAL_S: u32 = 127u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PROFESSIONAL_STUDENT: u32 = 112u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PROFESSIONAL_STUDENT_N: u32 = 114u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PROFESSIONAL_S_N: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PRO_CHINA: u32 = 139u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PRO_FOR_EDUCATION: u32 = 164u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PRO_FOR_EDUCATION_N: u32 = 165u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_PRO_SINGLE_LANGUAGE: u32 = 138u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_SERVERRDSH: u32 = 175u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER_CORE: u32 = 57u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_STANDARD_EVALUATION_SERVER_CORE: u32 = 160u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_STANDARD_NANO_SERVER: u32 = 144u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_STANDARD_SERVER_CORE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_STANDARD_WS_SERVER_CORE: u32 = 148u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_THINPC: u32 = 87u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_UNLICENSED: u32 = 2882382797u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_UTILITY_VM: u32 = 149u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_XBOX_DURANGOHOSTOS: u32 = 196u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_XBOX_ERAOS: u32 = 195u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_XBOX_GAMEOS: u32 = 194u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_XBOX_NATIVEOS: u32 = 193u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_XBOX_SCARLETTHOSTOS: u32 = 197u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PRODUCT_XBOX_SYSTEMOS: u32 = 192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "Win32_Foundation")]
pub type PTERMINATION_HANDLER = ::core::option::Option<unsafe extern "system" fn(_abnormal_termination: super::super::Foundation::BOOLEAN, establisherframe: u64)>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86_64")]
#[cfg(feature = "Win32_Foundation")]
pub type PTERMINATION_HANDLER = ::core::option::Option<unsafe extern "system" fn(_abnormal_termination: super::super::Foundation::BOOLEAN, establisherframe: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type PUMS_SCHEDULER_ENTRY_POINT = ::core::option::Option<unsafe extern "system" fn(reason: RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PcTeb: u32 = 24u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct QUOTA_LIMITS_EX {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
    pub WorkingSetLimit: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
    pub Flags: u32,
    pub CpuRateLimit: RATE_QUOTA_LIMIT,
}
impl ::core::marker::Copy for QUOTA_LIMITS_EX {}
impl ::core::clone::Clone for QUOTA_LIMITS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const QUOTA_LIMITS_HARDWS_MAX_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const QUOTA_LIMITS_HARDWS_MIN_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const QUOTA_LIMITS_USE_DEFAULT_LIMITS: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union RATE_QUOTA_LIMIT {
    pub RateData: u32,
    pub Anonymous: RATE_QUOTA_LIMIT_0,
}
impl ::core::marker::Copy for RATE_QUOTA_LIMIT {}
impl ::core::clone::Clone for RATE_QUOTA_LIMIT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RATE_QUOTA_LIMIT_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for RATE_QUOTA_LIMIT_0 {}
impl ::core::clone::Clone for RATE_QUOTA_LIMIT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const READ_THREAD_PROFILING_FLAG_DISPATCHING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const READ_THREAD_PROFILING_FLAG_HARDWARE_COUNTERS: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REARRANGE_FILE_DATA {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: super::super::Foundation::HANDLE,
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REARRANGE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REARRANGE_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct REARRANGE_FILE_DATA32 {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: u32,
    pub Length: u32,
    pub Flags: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for REARRANGE_FILE_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for REARRANGE_FILE_DATA32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {
    pub Version: u32,
    pub Accurate: u32,
    pub Supported: u32,
    pub AccurateMask0: u32,
}
impl ::core::marker::Copy for REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {}
impl ::core::clone::Clone for REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REDBOOK_DIGITAL_AUDIO_EXTRACTION_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_APP_HIVE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_APP_HIVE_OPEN_READ_ONLY: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_BOOT_HIVE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_FLUSH_HIVE_FILE_GROWTH: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_FORCE_UNLOAD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_HIVE_EXACT_FILE_GROWTH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_HIVE_NO_RM: i32 = 256i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_HIVE_SINGLE_LOG: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_IMMUTABLE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_LOAD_HIVE_OPEN_HANDLE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_NO_IMPERSONATION_FALLBACK: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_NO_LAZY_FLUSH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_OPEN_READ_ONLY: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_PROCESS_PRIVATE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_REFRESH_HIVE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_START_JOURNAL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const REG_UNLOAD_LEGAL_FLAGS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RESOURCEMANAGER_BASIC_INFORMATION {
    pub ResourceManagerId: ::windows_sys::core::GUID,
    pub DescriptionLength: u32,
    pub Description: [u16; 1],
}
impl ::core::marker::Copy for RESOURCEMANAGER_BASIC_INFORMATION {}
impl ::core::clone::Clone for RESOURCEMANAGER_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_COMPLETE_PROPAGATION: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESOURCEMANAGER_COMPLETION_INFORMATION {
    pub IoCompletionPortHandle: super::super::Foundation::HANDLE,
    pub CompletionKey: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESOURCEMANAGER_COMPLETION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESOURCEMANAGER_COMPLETION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_ENLIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_GET_NOTIFICATION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type RESOURCEMANAGER_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ResourceManagerBasicInformation: RESOURCEMANAGER_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ResourceManagerCompletionInformation: RESOURCEMANAGER_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_RECOVER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_REGISTER_PROTOCOL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RESOURCEMANAGER_SET_INFORMATION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RESUME_PERFORMANCE {
    pub PostTimeMs: u32,
    pub TotalResumeTimeMs: u64,
    pub ResumeCompleteTimestamp: u64,
}
impl ::core::marker::Copy for RESUME_PERFORMANCE {}
impl ::core::clone::Clone for RESUME_PERFORMANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ROTFLAGS_ALLOWANYCLIENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ROTFLAGS_REGISTRATIONKEEPSALIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const ROT_COMPARE_MAX: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_RUN_ONCE_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_RUN_ONCE_CHECK_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_RUN_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_RUN_ONCE_INIT_FAILED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type RTL_UMS_SCHEDULER_REASON = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UmsSchedulerStartup: RTL_UMS_SCHEDULER_REASON = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UmsSchedulerThreadBlocked: RTL_UMS_SCHEDULER_REASON = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UmsSchedulerThreadYield: RTL_UMS_SCHEDULER_REASON = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_UMS_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RTL_VIRTUAL_UNWIND2_VALIDATE_PAC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const RUNTIME_FUNCTION_INDIRECT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemBRUSH {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemBRUSH {}
impl ::core::clone::Clone for RemBRUSH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemHBITMAP {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemHBITMAP {}
impl ::core::clone::Clone for RemHBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemHENHMETAFILE {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemHENHMETAFILE {}
impl ::core::clone::Clone for RemHENHMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemHGLOBAL {
    pub fNullHGlobal: i32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemHGLOBAL {}
impl ::core::clone::Clone for RemHGLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemHMETAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemHMETAFILEPICT {}
impl ::core::clone::Clone for RemHMETAFILEPICT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemHPALETTE {
    pub cbData: u32,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for RemHPALETTE {}
impl ::core::clone::Clone for RemHPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct RemotableHandle {
    pub fContext: i32,
    pub u: RemotableHandle_0,
}
impl ::core::marker::Copy for RemotableHandle {}
impl ::core::clone::Clone for RemotableHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union RemotableHandle_0 {
    pub hInproc: i32,
    pub hRemote: i32,
}
impl ::core::marker::Copy for RemotableHandle_0 {}
impl ::core::clone::Clone for RemotableHandle_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type ReplacesCorHdrNumericDefines = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_ILONLY: ReplacesCorHdrNumericDefines = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_32BITREQUIRED: ReplacesCorHdrNumericDefines = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_IL_LIBRARY: ReplacesCorHdrNumericDefines = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_STRONGNAMESIGNED: ReplacesCorHdrNumericDefines = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: ReplacesCorHdrNumericDefines = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_TRACKDEBUGDATA: ReplacesCorHdrNumericDefines = 65536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COMIMAGE_FLAGS_32BITPREFERRED: ReplacesCorHdrNumericDefines = 131072i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VERSION_MAJOR_V2: ReplacesCorHdrNumericDefines = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VERSION_MAJOR: ReplacesCorHdrNumericDefines = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VERSION_MINOR: ReplacesCorHdrNumericDefines = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_DELETED_NAME_LENGTH: ReplacesCorHdrNumericDefines = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VTABLEGAP_NAME_LENGTH: ReplacesCorHdrNumericDefines = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const NATIVE_TYPE_MAX_CB: ReplacesCorHdrNumericDefines = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE: ReplacesCorHdrNumericDefines = 255i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COR_MIH_METHODRVA: ReplacesCorHdrNumericDefines = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COR_MIH_EHRVA: ReplacesCorHdrNumericDefines = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COR_MIH_BASICBLOCK: ReplacesCorHdrNumericDefines = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VTABLE_32BIT: ReplacesCorHdrNumericDefines = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VTABLE_64BIT: ReplacesCorHdrNumericDefines = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VTABLE_FROM_UNMANAGED: ReplacesCorHdrNumericDefines = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN: ReplacesCorHdrNumericDefines = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const COR_VTABLE_CALL_MOST_DERIVED: ReplacesCorHdrNumericDefines = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const IMAGE_COR_EATJ_THUNK_SIZE: ReplacesCorHdrNumericDefines = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAX_CLASS_NAME: ReplacesCorHdrNumericDefines = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const MAX_PACKAGE_NAME: ReplacesCorHdrNumericDefines = 1024i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCOPE_TABLE_AMD64 {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_AMD64_0; 1],
}
impl ::core::marker::Copy for SCOPE_TABLE_AMD64 {}
impl ::core::clone::Clone for SCOPE_TABLE_AMD64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCOPE_TABLE_AMD64_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
impl ::core::marker::Copy for SCOPE_TABLE_AMD64_0 {}
impl ::core::clone::Clone for SCOPE_TABLE_AMD64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCOPE_TABLE_ARM {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_ARM_0; 1],
}
impl ::core::marker::Copy for SCOPE_TABLE_ARM {}
impl ::core::clone::Clone for SCOPE_TABLE_ARM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCOPE_TABLE_ARM_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
impl ::core::marker::Copy for SCOPE_TABLE_ARM_0 {}
impl ::core::clone::Clone for SCOPE_TABLE_ARM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCOPE_TABLE_ARM64 {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_ARM64_0; 1],
}
impl ::core::marker::Copy for SCOPE_TABLE_ARM64 {}
impl ::core::clone::Clone for SCOPE_TABLE_ARM64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCOPE_TABLE_ARM64_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
impl ::core::marker::Copy for SCOPE_TABLE_ARM64_0 {}
impl ::core::clone::Clone for SCOPE_TABLE_ARM64_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCRUB_DATA_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub MaximumIos: u32,
    pub ObjectId: [u32; 4],
    pub Reserved: [u32; 41],
    pub ResumeContext: [u8; 1040],
}
impl ::core::marker::Copy for SCRUB_DATA_INPUT {}
impl ::core::clone::Clone for SCRUB_DATA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_IGNORE_REDUNDANCY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_OPLOCK_NOT_ACQUIRED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_RESUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SCRUB_BY_OBJECT_ID: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SKIP_DATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SKIP_IN_SYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_INPUT_FLAG_SKIP_NON_INTEGRITY_DATA: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCRUB_DATA_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub Status: u32,
    pub ErrorFileOffset: u64,
    pub ErrorLength: u64,
    pub NumberOfBytesRepaired: u64,
    pub NumberOfBytesFailed: u64,
    pub InternalFileReference: u64,
    pub ResumeContextLength: u16,
    pub ParityExtentDataOffset: u16,
    pub Reserved: [u32; 9],
    pub NumberOfMetadataBytesProcessed: u64,
    pub NumberOfDataBytesProcessed: u64,
    pub TotalNumberOfMetadataBytesInUse: u64,
    pub TotalNumberOfDataBytesInUse: u64,
    pub DataBytesSkippedDueToNoAllocation: u64,
    pub DataBytesSkippedDueToInvalidRun: u64,
    pub DataBytesSkippedDueToIntegrityStream: u64,
    pub DataBytesSkippedDueToRegionBeingClean: u64,
    pub DataBytesSkippedDueToLockConflict: u64,
    pub DataBytesSkippedDueToNoScrubDataFlag: u64,
    pub DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag: u64,
    pub DataBytesScrubbed: u64,
    pub ResumeContext: [u8; 1040],
}
impl ::core::marker::Copy for SCRUB_DATA_OUTPUT {}
impl ::core::clone::Clone for SCRUB_DATA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_NON_USER_DATA_RANGE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_PARITY_EXTENT_DATA_RETURNED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SCRUB_DATA_OUTPUT_FLAG_RESUME_CONTEXT_LENGTH_SPECIFIED: u32 = 262144u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCRUB_PARITY_EXTENT {
    pub Offset: i64,
    pub Length: u64,
}
impl ::core::marker::Copy for SCRUB_PARITY_EXTENT {}
impl ::core::clone::Clone for SCRUB_PARITY_EXTENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SCRUB_PARITY_EXTENT_DATA {
    pub Size: u16,
    pub Flags: u16,
    pub NumberOfParityExtents: u16,
    pub MaximumNumberOfParityExtents: u16,
    pub ParityExtents: [SCRUB_PARITY_EXTENT; 1],
}
impl ::core::marker::Copy for SCRUB_PARITY_EXTENT_DATA {}
impl ::core::clone::Clone for SCRUB_PARITY_EXTENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type SECTION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_ALL_ACCESS: SECTION_FLAGS = 983071u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_QUERY: SECTION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_MAP_WRITE: SECTION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_MAP_READ: SECTION_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_MAP_EXECUTE: SECTION_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_EXTEND_SIZE: SECTION_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECTION_MAP_EXECUTE_EXPLICIT: SECTION_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_ANONYMOUS_LOGON_RID: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_APPPOOL_ID_BASE_RID: i32 = 82i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_APPPOOL_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_APP_PACKAGE_BASE_RID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_APP_PACKAGE_RID_COUNT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATED_USER_RID: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED_RID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_AUTHORITY_RID_COUNT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_FRESH_KEY_AUTH_RID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION_RID: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA_RID: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_KEY_TRUST_RID: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_AUTHENTICATION_SERVICE_ASSERTED_RID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_BATCH_RID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_BUILTIN_APP_PACKAGE_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_BUILTIN_CAPABILITY_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_BUILTIN_DOMAIN_RID: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_APPOINTMENTS: i32 = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_APP_RID: u64 = 1024u64;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_BASE_RID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_CONTACTS: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_DOCUMENTS_LIBRARY: i32 = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_INTERNET_CLIENT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_INTERNET_EXPLORER: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_MUSIC_LIBRARY: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_PICTURES_LIBRARY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_REMOVABLE_STORAGE: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_RID_COUNT: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CAPABILITY_VIDEOS_LIBRARY: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CCG_ID_BASE_RID: i32 = 95i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CHILD_PACKAGE_RID_COUNT: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE_RID: i32 = 85i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_COM_ID_BASE_RID: i32 = 89i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CREATOR_GROUP_RID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CREATOR_GROUP_SERVER_RID: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CREATOR_OWNER_RID: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CREATOR_OWNER_RIGHTS_RID: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CREATOR_OWNER_SERVER_RID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CRED_TYPE_BASE_RID: i32 = 65i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CRED_TYPE_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_CRED_TYPE_THIS_ORG_CERT_RID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_DASHOST_ID_BASE_RID: i32 = 92i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_DASHOST_ID_RID_COUNT: i32 = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SECURITY_DESCRIPTOR_RELATIVE {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: u16,
    pub Owner: u32,
    pub Group: u32,
    pub Sacl: u32,
    pub Dacl: u32,
}
impl ::core::marker::Copy for SECURITY_DESCRIPTOR_RELATIVE {}
impl ::core::clone::Clone for SECURITY_DESCRIPTOR_RELATIVE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_DESCRIPTOR_REVISION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_DESCRIPTOR_REVISION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_DIALUP_RID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_ENTERPRISE_CONTROLLERS_RID: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_ENTERPRISE_READONLY_CONTROLLERS_RID: i32 = 22i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_INSTALLER_CAPABILITY_RID_COUNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_BASE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_RID_COUNT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_INTERACTIVE_RID: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_IUSER_RID: i32 = 17i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOCAL_ACCOUNT_AND_ADMIN_RID: i32 = 114i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOCAL_ACCOUNT_RID: i32 = 113i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOCAL_LOGON_RID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOCAL_RID: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOCAL_SERVICE_RID: i32 = 19i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOCAL_SYSTEM_RID: i32 = 18i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOGON_IDS_RID: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_LOGON_IDS_RID_COUNT: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_HIGH_RID: i32 = 12288i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_LOW_RID: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_MAXIMUM_USER_RID: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_MEDIUM_PLUS_RID: u32 = 8448u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_MEDIUM_RID: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_PROTECTED_PROCESS_RID: i32 = 20480i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_SYSTEM_RID: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MANDATORY_UNTRUSTED_RID: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MAX_ALWAYS_FILTERED: i32 = 999i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MAX_BASE_RID: i32 = 111i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MIN_BASE_RID: i32 = 80i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_MIN_NEVER_FILTERED: i32 = 1000i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_NETWORK_RID: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_NETWORK_SERVICE_RID: i32 = 20i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_NFS_ID_BASE_RID: i32 = 88i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_NT_NON_UNIQUE: i32 = 21i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_NT_NON_UNIQUE_SUB_AUTH_COUNT: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_NULL_RID: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SECURITY_OBJECT_AI_PARAMS {
    pub Size: u32,
    pub ConstraintMask: u32,
}
impl ::core::marker::Copy for SECURITY_OBJECT_AI_PARAMS {}
impl ::core::clone::Clone for SECURITY_OBJECT_AI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_OTHER_ORGANIZATION_RID: i32 = 1000i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PACKAGE_BASE_RID: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PACKAGE_DIGEST_RID: i32 = 21i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PACKAGE_NTLM_RID: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PACKAGE_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PACKAGE_SCHANNEL_RID: i32 = 14i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PARENT_PACKAGE_RID_COUNT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PRINCIPAL_SELF_RID: i32 = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE_RID: i32 = 1536i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_APP_RID: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE_RID: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_NONE_RID: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS_RID: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB_RID: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_TYPE_FULL_RID: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_TYPE_LITE_RID: i32 = 512i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_PROTECTION_TYPE_NONE_RID: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROCESS_TRUST_AUTHORITY_RID_COUNT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_PROXY_RID: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_RDV_GFX_BASE_RID: i32 = 91i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_REMOTE_LOGON_RID: i32 = 14i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_RESERVED_ID_BASE_RID: i32 = 81i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_RESTRICTED_CODE_RID: i32 = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_SERVER_LOGON_RID: i32 = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_SERVICE_ID_BASE_RID: i32 = 80i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_SERVICE_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_SERVICE_RID: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TASK_ID_BASE_RID: i32 = 87i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TERMINAL_SERVER_RID: i32 = 13i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_THIS_ORGANIZATION_RID: i32 = 15i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID1: u32 = 956008885u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID2: u32 = 3418522649u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID3: u32 = 1831038044u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID4: u32 = 1853292631u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_TRUSTED_INSTALLER_RID5: u32 = 2271478464u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_UMFD_BASE_RID: i32 = 96i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_USERMANAGER_ID_BASE_RID: i32 = 93i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_USERMANAGER_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_USERMODEDRIVERHOST_ID_BASE_RID: i32 = 84i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_USERMODEDRIVERHOST_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_VIRTUALACCOUNT_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_VIRTUALSERVER_ID_BASE_RID: i32 = 83i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_VIRTUALSERVER_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WINDOWSMOBILE_ID_BASE_RID: i32 = 112i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WINDOW_MANAGER_BASE_RID: i32 = 90i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WINRM_ID_BASE_RID: i32 = 94i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WINRM_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WMIHOST_ID_BASE_RID: i32 = 86i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WMIHOST_ID_RID_COUNT: i32 = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WORLD_RID: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SECURITY_WRITE_RESTRICTED_CODE_RID: i32 = 33i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SEC_HUGE_PAGES: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SEF_AI_USE_EXTRA_PARAMS: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SEF_FORCE_USER_MODE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SEMAPHORE_MODIFY_STATE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVERSILO_BASIC_INFORMATION {
    pub ServiceSessionId: u32,
    pub State: SERVERSILO_STATE,
    pub ExitStatus: u32,
    pub IsDownlevelContainer: super::super::Foundation::BOOLEAN,
    pub ApiSetSchema: *mut ::core::ffi::c_void,
    pub HostApiSetSchema: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVERSILO_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVERSILO_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type SERVERSILO_STATE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVERSILO_INITING: SERVERSILO_STATE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVERSILO_STARTED: SERVERSILO_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVERSILO_SHUTTING_DOWN: SERVERSILO_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVERSILO_TERMINATING: SERVERSILO_STATE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVERSILO_TERMINATED: SERVERSILO_STATE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVICE_INTERACTIVE_PROCESS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVICE_PKG_SERVICE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVICE_USERSERVICE_INSTANCE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SERVICE_USER_SERVICE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SESSION_MODIFY_ACCESS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SESSION_QUERY_ACCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_ACCESS_CHECK_FLAG_NO_LEARNING_MODE_LOGGING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_ACCESS_CHECK_VALID_FLAGS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_ACTIVATE_AS_USER_CAPABILITY: &str = "activateAsUser";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_ASSIGNPRIMARYTOKEN_NAME: &str = "SeAssignPrimaryTokenPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_AUDIT_NAME: &str = "SeAuditPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_BACKUP_NAME: &str = "SeBackupPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CHANGE_NOTIFY_NAME: &str = "SeChangeNotifyPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CONSTRAINED_IMPERSONATION_CAPABILITY: &str = "constrainedImpersonation";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CREATE_GLOBAL_NAME: &str = "SeCreateGlobalPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CREATE_PAGEFILE_NAME: &str = "SeCreatePagefilePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CREATE_PERMANENT_NAME: &str = "SeCreatePermanentPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CREATE_SYMBOLIC_LINK_NAME: &str = "SeCreateSymbolicLinkPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_CREATE_TOKEN_NAME: &str = "SeCreateTokenPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DACL_AUTO_INHERITED: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DACL_AUTO_INHERIT_REQ: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DACL_DEFAULTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DACL_PRESENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DACL_PROTECTED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DEBUG_NAME: &str = "SeDebugPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME: &str = "SeDelegateSessionUserImpersonatePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_DEVELOPMENT_MODE_NETWORK_CAPABILITY: &str = "developmentModeNetwork";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_ENABLE_DELEGATION_NAME: &str = "SeEnableDelegationPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_DEFAULTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_ENABLED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_ENABLED_BY_DEFAULT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_INTEGRITY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_INTEGRITY_ENABLED: i32 = 64i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_LOGON_ID: i32 = -1073741824i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_MANDATORY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_OWNER: i32 = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_RESOURCE: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_GROUP_USE_FOR_DENY_ONLY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type SE_IMAGE_SIGNATURE_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignatureNone: SE_IMAGE_SIGNATURE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignatureEmbedded: SE_IMAGE_SIGNATURE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignatureCache: SE_IMAGE_SIGNATURE_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignatureCatalogCached: SE_IMAGE_SIGNATURE_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignatureCatalogNotCached: SE_IMAGE_SIGNATURE_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignatureCatalogHint: SE_IMAGE_SIGNATURE_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignaturePackageCatalog: SE_IMAGE_SIGNATURE_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeImageSignaturePplMitigated: SE_IMAGE_SIGNATURE_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_IMPERSONATE_NAME: &str = "SeImpersonatePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_INCREASE_QUOTA_NAME: &str = "SeIncreaseQuotaPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_INC_BASE_PRIORITY_NAME: &str = "SeIncreaseBasePriorityPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_INC_WORKING_SET_NAME: &str = "SeIncreaseWorkingSetPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type SE_LEARNING_MODE_DATA_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeLearningModeInvalidType: SE_LEARNING_MODE_DATA_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeLearningModeSettings: SE_LEARNING_MODE_DATA_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SeLearningModeMax: SE_LEARNING_MODE_DATA_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_LEARNING_MODE_FLAG_PERMISSIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_LOAD_DRIVER_NAME: &str = "SeLoadDriverPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_LOCK_MEMORY_NAME: &str = "SeLockMemoryPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_MACHINE_ACCOUNT_NAME: &str = "SeMachineAccountPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_MANAGE_VOLUME_NAME: &str = "SeManageVolumePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_MUMA_CAPABILITY: &str = "muma";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_OWNER_DEFAULTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_PERMISSIVE_LEARNING_MODE_CAPABILITY: &str = "permissiveLearningMode";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_PROF_SINGLE_PROCESS_NAME: &str = "SeProfileSingleProcessPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_RELABEL_NAME: &str = "SeRelabelPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_REMOTE_SHUTDOWN_NAME: &str = "SeRemoteShutdownPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_RESTORE_NAME: &str = "SeRestorePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_RM_CONTROL_VALID: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SACL_AUTO_INHERITED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SACL_AUTO_INHERIT_REQ: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SACL_DEFAULTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SACL_PRESENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SACL_PROTECTED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_ACCESS_FILTER_ACE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_LABEL_ACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_OWNER_ACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SECURITY_DESCRIPTOR_VALID_FLAGS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SECURITY_NAME: &str = "SeSecurityPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SELF_RELATIVE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SESSION_IMPERSONATION_CAPABILITY: &str = "sessionImpersonation";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SHUTDOWN_NAME: &str = "SeShutdownPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_ANTIMALWARE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_AUTHENTICODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_1: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_2: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_3: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_4: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_5: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_6: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_CUSTOM_7: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_DEVELOPER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_DYNAMIC_CODEGEN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_MICROSOFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_STORE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_UNCHECKED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_UNSIGNED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_WINDOWS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SIGNING_LEVEL_WINDOWS_TCB: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SYNC_AGENT_NAME: &str = "SeSyncAgentPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SYSTEMTIME_NAME: &str = "SeSystemtimePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SYSTEM_ENVIRONMENT_NAME: &str = "SeSystemEnvironmentPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_SYSTEM_PROFILE_NAME: &str = "SeSystemProfilePrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_TAKE_OWNERSHIP_NAME: &str = "SeTakeOwnershipPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_TCB_NAME: &str = "SeTcbPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_TIME_ZONE_NAME: &str = "SeTimeZonePrivilege";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct SE_TOKEN_USER {
    pub Anonymous1: SE_TOKEN_USER_0,
    pub Anonymous2: SE_TOKEN_USER_1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_TOKEN_USER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_TOKEN_USER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union SE_TOKEN_USER_0 {
    pub TokenUser: super::super::Security::TOKEN_USER,
    pub User: super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_TOKEN_USER_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_TOKEN_USER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union SE_TOKEN_USER_1 {
    pub Sid: super::super::Security::SID,
    pub Buffer: [u8; 68],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for SE_TOKEN_USER_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for SE_TOKEN_USER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_TRUSTED_CREDMAN_ACCESS_NAME: &str = "SeTrustedCredManAccessPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_UNDOCK_NAME: &str = "SeUndockPrivilege";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SE_UNSOLICITED_INPUT_NAME: &str = "SeUnsolicitedInputPrivilege";
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SHARED_VIRTUAL_DISK_SUPPORT {
    pub SharedVirtualDiskSupport: SharedVirtualDiskSupportType,
    pub HandleState: SharedVirtualDiskHandleState,
}
impl ::core::marker::Copy for SHARED_VIRTUAL_DISK_SUPPORT {}
impl ::core::clone::Clone for SHARED_VIRTUAL_DISK_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SHUFFLE_FILE_DATA {
    pub StartingOffset: i64,
    pub Length: i64,
    pub Flags: u32,
}
impl ::core::marker::Copy for SHUFFLE_FILE_DATA {}
impl ::core::clone::Clone for SHUFFLE_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SHUFFLE_FILE_FLAG_SKIP_INITIALIZING_NEW_CLUSTERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SID_HASH_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SID_MAX_SUB_AUTHORITIES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SID_RECOMMENDED_SUB_AUTHORITIES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SID_REVISION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SILOOBJECT_BASIC_INFORMATION {
    pub SiloId: u32,
    pub SiloParentId: u32,
    pub NumberOfProcesses: u32,
    pub IsInServerSilo: super::super::Foundation::BOOLEAN,
    pub Reserved: [u8; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SILOOBJECT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SILOOBJECT_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SIZEOF_RFPO_DATA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SIZE_OF_80387_REGISTERS: u32 = 80u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SMB_CCF_APP_INSTANCE_EA_NAME: &str = "ClusteredApplicationInstance";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_CHINESE_BIG5: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_CHINESE_BOPOMOFO: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_CHINESE_PRC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_CHINESE_PRCP: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_CHINESE_RADICALSTROKE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_CHINESE_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_GEORGIAN_MODERN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_GEORGIAN_TRADITIONAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_GERMAN_PHONE_BOOK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_HUNGARIAN_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_HUNGARIAN_TECHNICAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_INVARIANT_MATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_JAPANESE_RADICALSTROKE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_JAPANESE_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_JAPANESE_XJIS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_KOREAN_KSC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SORT_KOREAN_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_AFRIKAANS_SOUTH_AFRICA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ALBANIAN_ALBANIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ALSATIAN_FRANCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_AMHARIC_ETHIOPIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_ALGERIA: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_BAHRAIN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_EGYPT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_IRAQ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_JORDAN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_KUWAIT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_LEBANON: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_LIBYA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_MOROCCO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_OMAN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_QATAR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_SAUDI_ARABIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_SYRIA: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_TUNISIA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_UAE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARABIC_YEMEN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ARMENIAN_ARMENIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ASSAMESE_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_AZERI_CYRILLIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_AZERI_LATIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BANGLA_BANGLADESH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BANGLA_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BASHKIR_RUSSIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BASQUE_BASQUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BELARUSIAN_BELARUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BENGALI_BANGLADESH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BENGALI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BRETON_FRANCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_BULGARIAN_BULGARIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CATALAN_CATALAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CENTRAL_KURDISH_IRAQ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CHEROKEE_CHEROKEE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CHINESE_HONGKONG: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CHINESE_MACAU: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CHINESE_SIMPLIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CHINESE_SINGAPORE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CHINESE_TRADITIONAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CORSICAN_FRANCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CROATIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CROATIAN_CROATIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CUSTOM_DEFAULT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CUSTOM_UNSPECIFIED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_CZECH_CZECH_REPUBLIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_DANISH_DENMARK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_DARI_AFGHANISTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_DIVEHI_MALDIVES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_DUTCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_DUTCH_BELGIAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_AUS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_BELIZE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_CAN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_CARIBBEAN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_EIRE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_INDIA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_JAMAICA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_MALAYSIA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_NZ: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_PHILIPPINES: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_SINGAPORE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_SOUTH_AFRICA: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_TRINIDAD: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_UK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_US: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ENGLISH_ZIMBABWE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ESTONIAN_ESTONIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FAEROESE_FAROE_ISLANDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FILIPINO_PHILIPPINES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FINNISH_FINLAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRENCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRENCH_BELGIAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRENCH_CANADIAN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRENCH_LUXEMBOURG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRENCH_MONACO: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRENCH_SWISS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FRISIAN_NETHERLANDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_FULAH_SENEGAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GALICIAN_GALICIAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GEORGIAN_GEORGIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GERMAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GERMAN_AUSTRIAN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GERMAN_LIECHTENSTEIN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GERMAN_LUXEMBOURG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GERMAN_SWISS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GREEK_GREECE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GREENLANDIC_GREENLAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_GUJARATI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_HAUSA_NIGERIA_LATIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_HAWAIIAN_US: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_HEBREW_ISRAEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_HINDI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_HUNGARIAN_HUNGARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ICELANDIC_ICELAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_IGBO_NIGERIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_INDONESIAN_INDONESIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_INUKTITUT_CANADA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_INUKTITUT_CANADA_LATIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_IRISH_IRELAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ITALIAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ITALIAN_SWISS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_JAPANESE_JAPAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KANNADA_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KASHMIRI_INDIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KASHMIRI_SASIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KAZAK_KAZAKHSTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KHMER_CAMBODIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KICHE_GUATEMALA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KINYARWANDA_RWANDA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KONKANI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KOREAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_KYRGYZ_KYRGYZSTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_LAO_LAO: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_LATVIAN_LATVIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_LITHUANIAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_LOWER_SORBIAN_GERMANY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_LUXEMBOURGISH_LUXEMBOURG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MACEDONIAN_MACEDONIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MALAYALAM_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MALAY_BRUNEI_DARUSSALAM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MALAY_MALAYSIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MALTESE_MALTA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MAORI_NEW_ZEALAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MAPUDUNGUN_CHILE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MARATHI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MOHAWK_MOHAWK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MONGOLIAN_CYRILLIC_MONGOLIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_MONGOLIAN_PRC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_NEPALI_INDIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_NEPALI_NEPAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_NEUTRAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_NORWEGIAN_BOKMAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_NORWEGIAN_NYNORSK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_OCCITAN_FRANCE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ODIA_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ORIYA_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PASHTO_AFGHANISTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PERSIAN_IRAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_POLISH_POLAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PORTUGUESE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PORTUGUESE_BRAZILIAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PULAR_SENEGAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PUNJABI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_PUNJABI_PAKISTAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_QUECHUA_BOLIVIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_QUECHUA_ECUADOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_QUECHUA_PERU: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ROMANIAN_ROMANIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ROMANSH_SWITZERLAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_RUSSIAN_RUSSIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAKHA_RUSSIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_INARI_FINLAND: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_LULE_NORWAY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_LULE_SWEDEN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_NORTHERN_FINLAND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_NORTHERN_NORWAY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_NORTHERN_SWEDEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_SKOLT_FINLAND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_SOUTHERN_NORWAY: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SAMI_SOUTHERN_SWEDEN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SANSKRIT_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SCOTTISH_GAELIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_CROATIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_CYRILLIC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_LATIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_MONTENEGRO_CYRILLIC: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_MONTENEGRO_LATIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_SERBIA_CYRILLIC: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SERBIAN_SERBIA_LATIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SINDHI_AFGHANISTAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SINDHI_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SINDHI_PAKISTAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SINHALESE_SRI_LANKA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SLOVAK_SLOVAKIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SLOVENIAN_SLOVENIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SOTHO_NORTHERN_SOUTH_AFRICA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_ARGENTINA: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_BOLIVIA: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_CHILE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_COLOMBIA: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_COSTA_RICA: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_DOMINICAN_REPUBLIC: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_ECUADOR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_EL_SALVADOR: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_GUATEMALA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_HONDURAS: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_MEXICAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_MODERN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_NICARAGUA: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_PANAMA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_PARAGUAY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_PERU: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_PUERTO_RICO: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_URUGUAY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_US: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SPANISH_VENEZUELA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SWAHILI_KENYA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SWEDISH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SWEDISH_FINLAND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SYRIAC_SYRIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_SYS_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TAJIK_TAJIKISTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TAMAZIGHT_ALGERIA_LATIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TAMAZIGHT_MOROCCO_TIFINAGH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TAMIL_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TAMIL_SRI_LANKA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TATAR_RUSSIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TELUGU_INDIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_THAI_THAILAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TIBETAN_PRC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TIGRIGNA_ERITREA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TIGRINYA_ERITREA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TIGRINYA_ETHIOPIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TSWANA_BOTSWANA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TSWANA_SOUTH_AFRICA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TURKISH_TURKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_TURKMEN_TURKMENISTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_UIGHUR_PRC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_UI_CUSTOM_DEFAULT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_UKRAINIAN_UKRAINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_UPPER_SORBIAN_GERMANY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_URDU_INDIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_URDU_PAKISTAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_UZBEK_CYRILLIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_UZBEK_LATIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_VALENCIAN_VALENCIA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_VIETNAMESE_VIETNAM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_WELSH_UNITED_KINGDOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_WOLOF_SENEGAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_XHOSA_SOUTH_AFRICA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_YAKUT_RUSSIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_YI_PRC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_YORUBA_NIGERIA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SUBLANG_ZULU_SOUTH_AFRICA: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct SUPPORTED_OS_INFO {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for SUPPORTED_OS_INFO {}
impl ::core::clone::Clone for SUPPORTED_OS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ACCESS_FILTER_ACE_TYPE: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ACCESS_FILTER_NOCONSTRAINT_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ACCESS_FILTER_VALID_MASK: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ALARM_ACE_TYPE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ALARM_CALLBACK_ACE_TYPE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_ALARM_OBJECT_ACE_TYPE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_AUDIT_ACE_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_AUDIT_CALLBACK_ACE_TYPE: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_AUDIT_OBJECT_ACE_TYPE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_CACHE_ALIGNMENT_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_MANDATORY_LABEL_ACE_TYPE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_MANDATORY_LABEL_NO_READ_UP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_MANDATORY_LABEL_NO_WRITE_UP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_PROCESS_TRUST_LABEL_ACE_TYPE: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_PROCESS_TRUST_LABEL_VALID_MASK: u32 = 16777215u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_PROCESS_TRUST_NOCONSTRAINT_MASK: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SYSTEM_SCOPED_POLICY_ID_ACE_TYPE: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type SharedVirtualDiskHandleState = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDiskHandleStateNone: SharedVirtualDiskHandleState = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDiskHandleStateFileShared: SharedVirtualDiskHandleState = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDiskHandleStateHandleShared: SharedVirtualDiskHandleState = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type SharedVirtualDiskSupportType = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDisksUnsupported: SharedVirtualDiskSupportType = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDisksSupported: SharedVirtualDiskSupportType = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDiskSnapshotsSupported: SharedVirtualDiskSupportType = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const SharedVirtualDiskCDPSnapshotsSupported: SharedVirtualDiskSupportType = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_CHECK_FOR_DRIVE_PROBLEM: i32 = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TAPE_CREATE_PARTITION {
    pub Method: u32,
    pub Count: u32,
    pub Size: u32,
}
impl ::core::marker::Copy for TAPE_CREATE_PARTITION {}
impl ::core::clone::Clone for TAPE_CREATE_PARTITION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_CLEAN_REQUESTS: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_COMPRESSION: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ECC: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_EJECT_MEDIA: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_EOT_WZ_SIZE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ERASE_BOP_ONLY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ERASE_IMMEDIATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ERASE_LONG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ERASE_SHORT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_FIXED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_FIXED_BLOCK: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_FORMAT: u32 = 2684354560u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_FORMAT_IMMEDIATE: u32 = 3221225472u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_GET_ABSOLUTE_BLK: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_GET_LOGICAL_BLK: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_HIGH_FEATURES: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_INITIATOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_PADDING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type TAPE_DRIVE_PROBLEM_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveProblemNone: TAPE_DRIVE_PROBLEM_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveReadWriteWarning: TAPE_DRIVE_PROBLEM_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveReadWriteError: TAPE_DRIVE_PROBLEM_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveReadWarning: TAPE_DRIVE_PROBLEM_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveWriteWarning: TAPE_DRIVE_PROBLEM_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveReadError: TAPE_DRIVE_PROBLEM_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveWriteError: TAPE_DRIVE_PROBLEM_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveHardwareError: TAPE_DRIVE_PROBLEM_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveUnsupportedMedia: TAPE_DRIVE_PROBLEM_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveScsiConnectionError: TAPE_DRIVE_PROBLEM_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveTimetoClean: TAPE_DRIVE_PROBLEM_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveCleanDriveNow: TAPE_DRIVE_PROBLEM_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveMediaLifeExpired: TAPE_DRIVE_PROBLEM_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TapeDriveSnappedTape: TAPE_DRIVE_PROBLEM_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_REPORT_SMKS: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_RESERVED_BIT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_CMP_BOP_ONLY: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_EOT_WZ_SIZE: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_TAPE_CAPACITY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_TAPE_REMAINING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_VARIABLE_BLOCK: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_WRITE_PROTECT: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_GET_DRIVE_PARAMETERS {
    pub ECC: super::super::Foundation::BOOLEAN,
    pub Compression: super::super::Foundation::BOOLEAN,
    pub DataPadding: super::super::Foundation::BOOLEAN,
    pub ReportSetmarks: super::super::Foundation::BOOLEAN,
    pub DefaultBlockSize: u32,
    pub MaximumBlockSize: u32,
    pub MinimumBlockSize: u32,
    pub MaximumPartitionCount: u32,
    pub FeaturesLow: u32,
    pub FeaturesHigh: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH,
    pub EOTWarningZoneSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_GET_DRIVE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_GET_DRIVE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ABS_BLK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147491840u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_ABSOLUTE_BLK: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147487744u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_END_OF_DATA: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147549184u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_FILEMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147745792u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_LOAD_UNLOAD: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483649u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_LOAD_UNLD_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483680u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_LOCK_UNLOCK: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483652u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_LOCK_UNLK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483776u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_LOG_BLK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147516416u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_LOGICAL_BLK: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147500032u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_RELATIVE_BLKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147614720u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_REVERSE_POSITION: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2151677952u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_REWIND_IMMEDIATE: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483656u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SEQUENTIAL_FMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2148007936u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SEQUENTIAL_SMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2149580800u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_BLOCK_SIZE: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483664u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_COMPRESSION: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147484160u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_ECC: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483904u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_PADDING: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147484672u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SET_REPORT_SMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147485696u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SETMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2148532224u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_SPACE_IMMEDIATE: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2155872256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_TENSION: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483650u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_TENSION_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2147483712u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_WRITE_FILEMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2181038080u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_WRITE_LONG_FMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2281701376u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_WRITE_MARK_IMMED: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2415919104u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_WRITE_SETMARKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2164260864u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_DRIVE_WRITE_SHORT_FMKS: TAPE_GET_DRIVE_PARAMETERS_FEATURES_HIGH = 2214592512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_GET_MEDIA_PARAMETERS {
    pub Capacity: i64,
    pub Remaining: i64,
    pub BlockSize: u32,
    pub PartitionCount: u32,
    pub WriteProtected: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_GET_MEDIA_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_GET_MEDIA_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_PSEUDO_LOGICAL_BLOCK: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_PSEUDO_LOGICAL_POSITION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_QUERY_DEVICE_ERROR_DATA: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_QUERY_DRIVE_PARAMETERS: i32 = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_QUERY_IO_ERROR_DATA: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TAPE_QUERY_MEDIA_CAPACITY: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TAPE_SET_DRIVE_PARAMETERS {
    pub ECC: super::super::Foundation::BOOLEAN,
    pub Compression: super::super::Foundation::BOOLEAN,
    pub DataPadding: super::super::Foundation::BOOLEAN,
    pub ReportSetmarks: super::super::Foundation::BOOLEAN,
    pub EOTWarningZoneSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TAPE_SET_DRIVE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TAPE_SET_DRIVE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TAPE_SET_MEDIA_PARAMETERS {
    pub BlockSize: u32,
}
impl ::core::marker::Copy for TAPE_SET_MEDIA_PARAMETERS {}
impl ::core::clone::Clone for TAPE_SET_MEDIA_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TAPE_WMI_OPERATIONS {
    pub Method: u32,
    pub DataBufferSize: u32,
    pub DataBuffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for TAPE_WMI_OPERATIONS {}
impl ::core::clone::Clone for TAPE_WMI_OPERATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct TEB(pub u8);
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const THREAD_BASE_PRIORITY_IDLE: i32 = -15i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const THREAD_BASE_PRIORITY_LOWRT: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const THREAD_BASE_PRIORITY_MAX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const THREAD_BASE_PRIORITY_MIN: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const THREAD_DYNAMIC_CODE_ALLOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const THREAD_PROFILING_FLAG_DISPATCH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TIMER_MODIFY_STATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TIMER_QUERY_STATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TIME_ZONE_ID_DAYLIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TIME_ZONE_ID_STANDARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TIME_ZONE_ID_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TLS_MINIMUM_AVAILABLE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_BNO_ISOLATION_INFORMATION {
    pub IsolationPrefix: ::windows_sys::core::PWSTR,
    pub IsolationEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_BNO_ISOLATION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_BNO_ISOLATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TOKEN_SID_INFORMATION {
    pub Sid: super::super::Foundation::PSID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TOKEN_SID_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TOKEN_SID_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TOKEN_SOURCE_LENGTH: u32 = 8u32;
#[repr(C)]
pub struct TP_CLEANUP_GROUP(pub u8);
#[repr(C)]
pub struct TP_POOL(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTIONMANAGER_BASIC_INFORMATION {
    pub TmIdentity: ::windows_sys::core::GUID,
    pub VirtualClock: i64,
}
impl ::core::marker::Copy for TRANSACTIONMANAGER_BASIC_INFORMATION {}
impl ::core::clone::Clone for TRANSACTIONMANAGER_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTIONMANAGER_BIND_TRANSACTION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTIONMANAGER_CREATE_RM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type TRANSACTIONMANAGER_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionManagerBasicInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionManagerLogInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionManagerLogPathInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionManagerRecoveryInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionManagerOnlineProbeInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionManagerOldestTransactionInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    pub LogPathLength: u32,
    pub LogPath: [u16; 1],
}
impl ::core::marker::Copy for TRANSACTIONMANAGER_LOGPATH_INFORMATION {}
impl ::core::clone::Clone for TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTIONMANAGER_LOG_INFORMATION {
    pub LogIdentity: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TRANSACTIONMANAGER_LOG_INFORMATION {}
impl ::core::clone::Clone for TRANSACTIONMANAGER_LOG_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTIONMANAGER_OLDEST_INFORMATION {
    pub OldestTransactionGuid: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TRANSACTIONMANAGER_OLDEST_INFORMATION {}
impl ::core::clone::Clone for TRANSACTIONMANAGER_OLDEST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTIONMANAGER_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTIONMANAGER_RECOVER: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    pub LastRecoveredLsn: u64,
}
impl ::core::marker::Copy for TRANSACTIONMANAGER_RECOVERY_INFORMATION {}
impl ::core::clone::Clone for TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTIONMANAGER_RENAME: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTIONMANAGER_SET_INFORMATION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_BASIC_INFORMATION {
    pub TransactionId: ::windows_sys::core::GUID,
    pub State: u32,
    pub Outcome: u32,
}
impl ::core::marker::Copy for TRANSACTION_BASIC_INFORMATION {}
impl ::core::clone::Clone for TRANSACTION_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct TRANSACTION_BIND_INFORMATION {
    pub TmHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for TRANSACTION_BIND_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for TRANSACTION_BIND_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_COMMIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_ENLIST: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_ENLISTMENTS_INFORMATION {
    pub NumberOfEnlistments: u32,
    pub EnlistmentPair: [TRANSACTION_ENLISTMENT_PAIR; 1],
}
impl ::core::marker::Copy for TRANSACTION_ENLISTMENTS_INFORMATION {}
impl ::core::clone::Clone for TRANSACTION_ENLISTMENTS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_ENLISTMENT_PAIR {
    pub EnlistmentId: ::windows_sys::core::GUID,
    pub ResourceManagerId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_ENLISTMENT_PAIR {}
impl ::core::clone::Clone for TRANSACTION_ENLISTMENT_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type TRANSACTION_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionBasicInformation: TRANSACTION_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionPropertiesInformation: TRANSACTION_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionEnlistmentInformation: TRANSACTION_INFORMATION_CLASS = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionSuperiorEnlistmentInformation: TRANSACTION_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionBindInformation: TRANSACTION_INFORMATION_CLASS = 4i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionDTCPrivateInformation: TRANSACTION_INFORMATION_CLASS = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_LIST_ENTRY {
    pub UOW: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for TRANSACTION_LIST_ENTRY {}
impl ::core::clone::Clone for TRANSACTION_LIST_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_LIST_INFORMATION {
    pub NumberOfTransactions: u32,
    pub TransactionInformation: [TRANSACTION_LIST_ENTRY; 1],
}
impl ::core::marker::Copy for TRANSACTION_LIST_INFORMATION {}
impl ::core::clone::Clone for TRANSACTION_LIST_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_PROPAGATE: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_PROPERTIES_INFORMATION {
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: i64,
    pub Outcome: u32,
    pub DescriptionLength: u32,
    pub Description: [u16; 1],
}
impl ::core::marker::Copy for TRANSACTION_PROPERTIES_INFORMATION {}
impl ::core::clone::Clone for TRANSACTION_PROPERTIES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_QUERY_INFORMATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_RIGHT_RESERVED1: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_ROLLBACK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRANSACTION_SET_INFORMATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type TRANSACTION_STATE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionStateNormal: TRANSACTION_STATE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionStateIndoubt: TRANSACTION_STATE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TransactionStateCommittedNotify: TRANSACTION_STATE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    pub SuperiorEnlistmentPair: TRANSACTION_ENLISTMENT_PAIR,
}
impl ::core::marker::Copy for TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {}
impl ::core::clone::Clone for TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TREE_CONNECT_ATTRIBUTE_GLOBAL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TREE_CONNECT_ATTRIBUTE_INTEGRITY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TREE_CONNECT_ATTRIBUTE_PINNED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TREE_CONNECT_ATTRIBUTE_PRIVACY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const TRUST_PROTECTED_FILTER_ACE_FLAG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UCSCHAR_INVALID_CHARACTER: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct UMS_CREATE_THREAD_ATTRIBUTES {
    pub UmsVersion: u32,
    pub UmsContext: *mut ::core::ffi::c_void,
    pub UmsCompletionList: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for UMS_CREATE_THREAD_ATTRIBUTES {}
impl ::core::clone::Clone for UMS_CREATE_THREAD_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNICODE_STRING_MAX_CHARS: u32 = 32767u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNIFIEDBUILDREVISION_KEY: &str = "\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNIFIEDBUILDREVISION_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNIFIEDBUILDREVISION_VALUE: &str = "UBR";
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNWIND_CHAIN_LIMIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNWIND_HISTORY_TABLE_SIZE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const UNW_FLAG_NO_EPILOGUE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type USER_ACTIVITY_PRESENCE = i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerUserPresent: USER_ACTIVITY_PRESENCE = 0i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerUserNotPresent: USER_ACTIVITY_PRESENCE = 1i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerUserInactive: USER_ACTIVITY_PRESENCE = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerUserMaximum: USER_ACTIVITY_PRESENCE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const PowerUserInvalid: USER_ACTIVITY_PRESENCE = 3i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VALID_INHERIT_FLAGS: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VBS_BASIC_PAGE_MEASURED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VBS_BASIC_PAGE_SYSTEM_CALL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VBS_BASIC_PAGE_THREAD_DESCRIPTOR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VBS_BASIC_PAGE_UNMEASURED_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VBS_BASIC_PAGE_ZERO_FILL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_AND: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_CONDITION_MASK: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_EQUAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_GREATER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_GREATER_EQUAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_LESS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_LESS_EQUAL: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_NT_DOMAIN_CONTROLLER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_NT_SERVER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_NT_WORKSTATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_NUM_BITS_PER_CONDITION_MASK: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_OR: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SERVER_NT: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_BACKOFFICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_BLADE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_COMMUNICATIONS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_COMPUTE_SERVER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_DATACENTER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_EMBEDDEDNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_EMBEDDED_RESTRICTED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_MULTIUSERTS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_PERSONAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_SECURITY_APPLIANCE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_SINGLEUSERTS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_SMALLBUSINESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_SMALLBUSINESS_RESTRICTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_STORAGE_SERVER: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_TERMINAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_SUITE_WH_SERVER: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VER_WORKSTATION_NT: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const VOLMGR_KSR_BYPASS: super::super::Foundation::NTSTATUS = -2143813629i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const VOLMGR_KSR_ERROR: super::super::Foundation::NTSTATUS = -2143813631i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub const VOLMGR_KSR_READ_ERROR: super::super::Foundation::NTSTATUS = -2143813630i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VRL_CUSTOM_CLASS_BEGIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VRL_ENABLE_KERNEL_BREAKS: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const VRL_PREDEFINED_CLASS_BEGIN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct VolLockBroadcast {
    pub vlb_dbh: DEV_BROADCAST_HDR,
    pub vlb_owner: u32,
    pub vlb_perms: u8,
    pub vlb_lockType: u8,
    pub vlb_drive: u8,
    pub vlb_flags: u8,
}
impl ::core::marker::Copy for VolLockBroadcast {}
impl ::core::clone::Clone for VolLockBroadcast {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WDT_INPROC64_CALL: u32 = 1349805143u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WDT_INPROC_CALL: u32 = 1215587415u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WDT_REMOTE_CALL: u32 = 1383359575u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub type WORKERCALLBACKFUNC = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WRITE_DAC: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WRITE_NV_MEMORY_FLAG_FLUSH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WRITE_NV_MEMORY_FLAG_NON_TEMPORAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WRITE_NV_MEMORY_FLAG_NO_DRAIN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WRITE_OWNER: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WRITE_WATCH_FLAG_RESET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WT_EXECUTEDELETEWAIT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WT_EXECUTEINLONGTHREAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WT_EXECUTEINPERSISTENTIOTHREAD: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const WT_EXECUTEINUITHREAD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_BTYPE_QP_INST_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_BTYPE_QP_INST_WORD_POS_X: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_BTYPE_QP_INST_WORD_X: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_BTYPE_QP_SIZE_X: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_D_WH_INST_WORD_POS_X: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_D_WH_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_D_WH_SIGN_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_D_WH_SIZE_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_EMPTY_INST_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_EMPTY_INST_WORD_POS_X: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_EMPTY_INST_WORD_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_EMPTY_SIZE_X: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM20_INST_WORD_POS_X: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM20_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM20_SIGN_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM20_SIZE_X: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_1_INST_WORD_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_1_INST_WORD_X: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_1_SIGN_VAL_POS_X: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_1_SIZE_X: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_2_INST_WORD_POS_X: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_2_INST_WORD_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_2_SIGN_VAL_POS_X: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_IMM39_2_SIZE_X: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_I_INST_WORD_POS_X: u32 = 27u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_I_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_I_SIGN_VAL_POS_X: u32 = 59u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_I_SIZE_X: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_OPCODE_INST_WORD_POS_X: u32 = 28u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_OPCODE_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_OPCODE_SIGN_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_OPCODE_SIZE_X: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_P_INST_WORD_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_P_INST_WORD_X: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_P_SIGN_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_P_SIZE_X: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_TMPLT_INST_WORD_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_TMPLT_INST_WORD_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_TMPLT_SIGN_VAL_POS_X: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X3_TMPLT_SIZE_X: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const X86_CACHE_ALIGNMENT_SIZE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct XSAVE_CET_U_FORMAT {
    pub Ia32CetUMsr: u64,
    pub Ia32Pl3SspMsr: u64,
}
impl ::core::marker::Copy for XSAVE_CET_U_FORMAT {}
impl ::core::clone::Clone for XSAVE_CET_U_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_ALIGN_BIT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_AMX_TILE_CONFIG: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_AMX_TILE_DATA: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_AVX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_AVX512_KMASK: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_AVX512_ZMM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_AVX512_ZMM_H: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_CET_S: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_CET_U: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_COMPACTION_ENABLE: u32 = 63u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_CONTROLFLAG_XFD_MASK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_CONTROLFLAG_XSAVEC_MASK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_CONTROLFLAG_XSAVEOPT_MASK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_GSSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_IPT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_LEGACY_FLOATING_POINT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_LEGACY_SSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_LWP: u32 = 62u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_MPX_BNDCSR: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_MPX_BNDREGS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_PASID: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const XSTATE_XFD_BIT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct _DEV_BROADCAST_HEADER {
    pub dbcd_size: u32,
    pub dbcd_devicetype: u32,
    pub dbcd_reserved: u32,
}
impl ::core::marker::Copy for _DEV_BROADCAST_HEADER {}
impl ::core::clone::Clone for _DEV_BROADCAST_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _DEV_BROADCAST_USERDEFINED {
    pub dbud_dbh: DEV_BROADCAST_HDR,
    pub dbud_szName: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DEV_BROADCAST_USERDEFINED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DEV_BROADCAST_USERDEFINED {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _MM_HINT_NTA: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _MM_HINT_T0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _MM_HINT_T1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _MM_HINT_T2: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DDEVCAPS_HWINDEXBUFFER: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DDEVCAPS_HWVERTEXBUFFER: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DDEVCAPS_SUBVOLUMELOCK: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DFVF_FOG: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_MAGIC: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_DEFERRED_AGP_AWARE: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_DEFER_AGP_FREES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_DXVERSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_FREE_DEFERRED_AGP: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETADAPTERGROUP: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETD3DCAPS8: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETD3DCAPS9: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETD3DQUERY: u32 = 34u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETD3DQUERYCOUNT: u32 = 33u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETDDIVERSION: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETEXTENDEDMODE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETEXTENDEDMODECOUNT: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETFORMAT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETFORMATCOUNT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DGDI2_TYPE_GETMULTISAMPLEQUALITYLEVELS: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPMISCCAPS_FOGINFVF: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_COLOROUT_MAX_V2_0: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_COLOROUT_MAX_V2_1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_COLOROUT_MAX_V3_0: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTBOOLREG_MAX_SW_DX9: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTBOOLREG_MAX_V2_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTBOOLREG_MAX_V3_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTINTREG_MAX_SW_DX9: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTINTREG_MAX_V2_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTINTREG_MAX_V3_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_DX8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_SW_DX9: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V1_1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V1_2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V1_3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V1_4: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V2_0: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V2_1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_CONSTREG_MAX_V3_0: u32 = 224u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_DX8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V1_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V1_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V1_3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V1_4: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V2_0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V2_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_INPUTREG_MAX_V3_0: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_MAXLOOPINITVALUE_V2_1: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_MAXLOOPINITVALUE_V3_0: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_MAXLOOPITERATIONCOUNT_V2_1: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_MAXLOOPITERATIONCOUNT_V3_0: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_MAXLOOPSTEP_V2_1: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_MAXLOOPSTEP_V3_0: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_PREDICATE_MAX_V2_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_PREDICATE_MAX_V3_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_DX8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V1_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V1_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V1_3: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V1_4: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V2_0: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V2_1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEMPREG_MAX_V3_0: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_DX8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_1: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_3: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V1_4: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V2_0: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V2_1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DPS_TEXTUREREG_MAX_V3_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DRS_DELETERTPATCH: u32 = 169u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ADDRREG_MAX_V1_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ADDRREG_MAX_V2_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ADDRREG_MAX_V2_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ADDRREG_MAX_V3_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ATTROUTREG_MAX_V1_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ATTROUTREG_MAX_V2_0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_ATTROUTREG_MAX_V2_1: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTBOOLREG_MAX_SW_DX9: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTBOOLREG_MAX_V2_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTBOOLREG_MAX_V2_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTBOOLREG_MAX_V3_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTINTREG_MAX_SW_DX9: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTINTREG_MAX_V2_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTINTREG_MAX_V2_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTINTREG_MAX_V3_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTREG_MAX_V1_1: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTREG_MAX_V2_0: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTREG_MAX_V2_1: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_CONSTREG_MAX_V3_0: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_INPUTREG_MAX_V1_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_INPUTREG_MAX_V2_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_INPUTREG_MAX_V2_1: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_INPUTREG_MAX_V3_0: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_LABEL_MAX_V3_0: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXINSTRUCTIONCOUNT_V1_1: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPINITVALUE_V2_0: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPINITVALUE_V2_1: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPINITVALUE_V3_0: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPITERATIONCOUNT_V2_0: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPITERATIONCOUNT_V2_1: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPITERATIONCOUNT_V3_0: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPSTEP_V2_0: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPSTEP_V2_1: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_MAXLOOPSTEP_V3_0: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_OUTPUTREG_MAX_SW_DX9: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_OUTPUTREG_MAX_V3_0: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_PREDICATE_MAX_V2_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_PREDICATE_MAX_V3_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TCRDOUTREG_MAX_V1_1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TCRDOUTREG_MAX_V2_0: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TCRDOUTREG_MAX_V2_1: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TEMPREG_MAX_V1_1: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TEMPREG_MAX_V2_0: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TEMPREG_MAX_V2_1: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_D3DVS_TEMPREG_MAX_V3_0: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_RTPATCHFLAG_HASINFO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub const _NT_RTPATCHFLAG_HASSEGS: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct remoteMETAFILEPICT {
    pub mm: i32,
    pub xExt: i32,
    pub yExt: i32,
    pub hMF: *mut userHMETAFILE,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for remoteMETAFILEPICT {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for remoteMETAFILEPICT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct userBITMAP {
    pub bmType: i32,
    pub bmWidth: i32,
    pub bmHeight: i32,
    pub bmWidthBytes: i32,
    pub bmPlanes: u16,
    pub bmBitsPixel: u16,
    pub cbSize: u32,
    pub pBuffer: [u8; 1],
}
impl ::core::marker::Copy for userBITMAP {}
impl ::core::clone::Clone for userBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct userCLIPFORMAT {
    pub fContext: i32,
    pub u: userCLIPFORMAT_0,
}
impl ::core::marker::Copy for userCLIPFORMAT {}
impl ::core::clone::Clone for userCLIPFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union userCLIPFORMAT_0 {
    pub dwValue: u32,
    pub pwszName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for userCLIPFORMAT_0 {}
impl ::core::clone::Clone for userCLIPFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub struct userHBITMAP {
    pub fContext: i32,
    pub u: userHBITMAP_0,
}
impl ::core::marker::Copy for userHBITMAP {}
impl ::core::clone::Clone for userHBITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`*"]
pub union userHBITMAP_0 {
    pub hInproc: i32,
    pub hRemote: *mut userBITMAP,
    pub hInproc64: i64,
}
impl ::core::marker::Copy for userHBITMAP_0 {}
impl ::core::clone::Clone for userHBITMAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct userHENHMETAFILE {
    pub fContext: i32,
    pub u: userHENHMETAFILE_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHENHMETAFILE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHENHMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union userHENHMETAFILE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHENHMETAFILE_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHENHMETAFILE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct userHGLOBAL {
    pub fContext: i32,
    pub u: userHGLOBAL_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHGLOBAL {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHGLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union userHGLOBAL_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::FLAGGED_BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHGLOBAL_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHGLOBAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct userHMETAFILE {
    pub fContext: i32,
    pub u: userHMETAFILE_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHMETAFILE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHMETAFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union userHMETAFILE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::Com::BYTE_BLOB,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHMETAFILE_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHMETAFILE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct userHMETAFILEPICT {
    pub fContext: i32,
    pub u: userHMETAFILEPICT_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHMETAFILEPICT {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHMETAFILEPICT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union userHMETAFILEPICT_0 {
    pub hInproc: i32,
    pub hRemote: *mut remoteMETAFILEPICT,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for userHMETAFILEPICT_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for userHMETAFILEPICT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct userHPALETTE {
    pub fContext: i32,
    pub u: userHPALETTE_0,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for userHPALETTE {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for userHPALETTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SystemServices\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub union userHPALETTE_0 {
    pub hInproc: i32,
    pub hRemote: *mut super::super::Graphics::Gdi::LOGPALETTE,
    pub hInproc64: i64,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for userHPALETTE_0 {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for userHPALETTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
