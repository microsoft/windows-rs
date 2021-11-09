#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub struct ACT_AUTHORIZATION_STATE {
    pub ulState: u32,
}
impl ACT_AUTHORIZATION_STATE {}
impl ::core::default::Default for ACT_AUTHORIZATION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ACT_AUTHORIZATION_STATE {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ACT_AUTHORIZATION_STATE").field("ulState", &self.ulState).finish()
    }
}
impl ::core::cmp::PartialEq for ACT_AUTHORIZATION_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.ulState == other.ulState
    }
}
impl ::core::cmp::Eq for ACT_AUTHORIZATION_STATE {}
unsafe impl ::windows::runtime::Abi for ACT_AUTHORIZATION_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACT_AUTHORIZATION_STATE_VALUE(pub i32);
pub const ACT_UNAUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = ACT_AUTHORIZATION_STATE_VALUE(0i32);
pub const ACT_AUTHORIZED: ACT_AUTHORIZATION_STATE_VALUE = ACT_AUTHORIZATION_STATE_VALUE(1i32);
impl ::core::convert::From<i32> for ACT_AUTHORIZATION_STATE_VALUE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACT_AUTHORIZATION_STATE_VALUE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_AUTHORIZE_ON_RESUME: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_AUTHORIZE_ON_SESSION_UNLOCK: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_UNAUTHORIZE_ON_SESSION_LOCK: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ACT_UNAUTHORIZE_ON_SUSPEND: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const APPUSERMODEL_STARTPINOPTION_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const APPUSERMODEL_STARTPINOPTION_NOPINONINSTALL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const APPUSERMODEL_STARTPINOPTION_USERPINNED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const AUDIO_CHANNELCOUNT_MONO: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const AUDIO_CHANNELCOUNT_STEREO: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_ADDRESS_TYPE_PUBLIC: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_ADDRESS_TYPE_RANDOM: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_CACHED_MODE_UNCACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const BLUETOOTH_CACHE_MODE_CACHED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_CERTIFICATE_SUPPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_HASH_ALG: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_OPTIONAL_FEATURES: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_CAPABILITY_SIGNATURE_ALG: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_MAX_CAPABILITY: u32 = 255u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_ASCh: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_ASCm: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_EMPTY: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_HCh: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_PCp: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_TYPE_SIGNER: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_BASIC: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_EXTENDED: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_NONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CERT_VALIDATION_POLICY_RESERVED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CREATOROPENWITHUIOPTION_HIDDEN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const CREATOROPENWITHUIOPTION_VISIBLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATED: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_AUTHENTICATION_DENIED: u32 = 2147483649u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_DEVICE_ERROR: u32 = 2147483650u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_NOT_AUTHENTICATED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_NO_AUTHENTICATION_REQUIRED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ENHANCED_STORAGE_AUTHN_STATE_UNKNOWN: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_CAPABILITY_ASYMMETRIC_KEY_CRYPTOGRAPHY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 4002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_CAPABILITY_CERTIFICATE_EXTENSION_PARSING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 4005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_CAPABILITY_HASH_ALGS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 4001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_CAPABILITY_RENDER_USER_DATA_UNUSABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 4004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_CAPABILITY_SIGNING_ALGS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 4003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_ADMIN_CERTIFICATE_AUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_CREATE_CERTIFICATE_REQUEST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_DEVICE_CERTIFICATE_AUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_ACT_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_CERTIFICATE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 111u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_GET_SILO_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_HOST_CERTIFICATE_AUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_INITIALIZE_TO_MANUFACTURER_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_SET_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_CERT_UNAUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_AUTHORIZE_ACT_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 203u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CHANGE_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 209u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CONFIG_ADMINISTRATOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 206u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_CREATE_USER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 207u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_DELETE_USER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 208u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_INITIALIZE_USER_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 210u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_QUERY_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 205u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_START_INITIALIZE_TO_MANUFACTURER_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 211u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_PASSWORD_UNAUTHORIZE_ACT_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 204u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_SILO_ENUMERATE_SILOS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_SILO_GET_AUTHENTICATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_COMMAND_SILO_IS_AUTHENTICATION_SILO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 6u32,
};
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
pub struct ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    pub CurrentAdminFailures: u8,
    pub CurrentUserFailures: u8,
    pub TotalUserAuthenticationCount: u32,
    pub TotalAdminAuthenticationCount: u32,
    pub FipsCompliant: super::super::Foundation::BOOL,
    pub SecurityIDAvailable: super::super::Foundation::BOOL,
    pub InitializeInProgress: super::super::Foundation::BOOL,
    pub ITMSArmed: super::super::Foundation::BOOL,
    pub ITMSArmable: super::super::Foundation::BOOL,
    pub UserCreated: super::super::Foundation::BOOL,
    pub ResetOnPORDefault: super::super::Foundation::BOOL,
    pub ResetOnPORCurrent: super::super::Foundation::BOOL,
    pub MaxAdminFailures: u8,
    pub MaxUserFailures: u8,
    pub TimeToCompleteInitialization: u32,
    pub TimeRemainingToCompleteInitialization: u32,
    pub MinTimeToAuthenticate: u32,
    pub MaxAdminPasswordSize: u8,
    pub MinAdminPasswordSize: u8,
    pub MaxAdminHintSize: u8,
    pub MaxUserPasswordSize: u8,
    pub MinUserPasswordSize: u8,
    pub MaxUserHintSize: u8,
    pub MaxUserNameSize: u8,
    pub MaxSiloNameSize: u8,
    pub MaxChallengeSize: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION")
            .field("CurrentAdminFailures", &self.CurrentAdminFailures)
            .field("CurrentUserFailures", &self.CurrentUserFailures)
            .field("TotalUserAuthenticationCount", &self.TotalUserAuthenticationCount)
            .field("TotalAdminAuthenticationCount", &self.TotalAdminAuthenticationCount)
            .field("FipsCompliant", &self.FipsCompliant)
            .field("SecurityIDAvailable", &self.SecurityIDAvailable)
            .field("InitializeInProgress", &self.InitializeInProgress)
            .field("ITMSArmed", &self.ITMSArmed)
            .field("ITMSArmable", &self.ITMSArmable)
            .field("UserCreated", &self.UserCreated)
            .field("ResetOnPORDefault", &self.ResetOnPORDefault)
            .field("ResetOnPORCurrent", &self.ResetOnPORCurrent)
            .field("MaxAdminFailures", &self.MaxAdminFailures)
            .field("MaxUserFailures", &self.MaxUserFailures)
            .field("TimeToCompleteInitialization", &self.TimeToCompleteInitialization)
            .field("TimeRemainingToCompleteInitialization", &self.TimeRemainingToCompleteInitialization)
            .field("MinTimeToAuthenticate", &self.MinTimeToAuthenticate)
            .field("MaxAdminPasswordSize", &self.MaxAdminPasswordSize)
            .field("MinAdminPasswordSize", &self.MinAdminPasswordSize)
            .field("MaxAdminHintSize", &self.MaxAdminHintSize)
            .field("MaxUserPasswordSize", &self.MaxUserPasswordSize)
            .field("MinUserPasswordSize", &self.MinUserPasswordSize)
            .field("MaxUserHintSize", &self.MaxUserHintSize)
            .field("MaxUserNameSize", &self.MaxUserNameSize)
            .field("MaxSiloNameSize", &self.MaxSiloNameSize)
            .field("MaxChallengeSize", &self.MaxChallengeSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentAdminFailures == other.CurrentAdminFailures
            && self.CurrentUserFailures == other.CurrentUserFailures
            && self.TotalUserAuthenticationCount == other.TotalUserAuthenticationCount
            && self.TotalAdminAuthenticationCount == other.TotalAdminAuthenticationCount
            && self.FipsCompliant == other.FipsCompliant
            && self.SecurityIDAvailable == other.SecurityIDAvailable
            && self.InitializeInProgress == other.InitializeInProgress
            && self.ITMSArmed == other.ITMSArmed
            && self.ITMSArmable == other.ITMSArmable
            && self.UserCreated == other.UserCreated
            && self.ResetOnPORDefault == other.ResetOnPORDefault
            && self.ResetOnPORCurrent == other.ResetOnPORCurrent
            && self.MaxAdminFailures == other.MaxAdminFailures
            && self.MaxUserFailures == other.MaxUserFailures
            && self.TimeToCompleteInitialization == other.TimeToCompleteInitialization
            && self.TimeRemainingToCompleteInitialization == other.TimeRemainingToCompleteInitialization
            && self.MinTimeToAuthenticate == other.MinTimeToAuthenticate
            && self.MaxAdminPasswordSize == other.MaxAdminPasswordSize
            && self.MinAdminPasswordSize == other.MinAdminPasswordSize
            && self.MaxAdminHintSize == other.MaxAdminHintSize
            && self.MaxUserPasswordSize == other.MaxUserPasswordSize
            && self.MinUserPasswordSize == other.MinUserPasswordSize
            && self.MaxUserHintSize == other.MaxUserHintSize
            && self.MaxUserNameSize == other.MaxUserNameSize
            && self.MaxSiloNameSize == other.MaxSiloNameSize
            && self.MaxChallengeSize == other.MaxChallengeSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENHANCED_STORAGE_PASSWORD_SILO_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_ADMIN_HINT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_AUTHENTICATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_ACT_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_CAPABILITY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_REQUEST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_SILO_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_CERTIFICATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_IS_AUTHENTICATION_SILO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_MAX_AUTH_FAILURES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_MAX_CERTIFICATE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_NEW_PASSWORD_INDICATOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_NEXT_CERTIFICATE_OF_TYPE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_OLD_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD_INDICATOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_PASSWORD_SILO_INFO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_QUERY_SILO_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2017u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_QUERY_SILO_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_SECURITY_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_SIGNER_CERTIFICATE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_SILO_FRIENDLYNAME_SPECIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_SILO_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_STORED_CERTIFICATE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_TEMPORARY_UNAUTHENTICATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_USER_HINT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_USER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 2010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const ENHANCED_STORAGE_PROPERTY_VALIDATION_POLICY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]),
    pid: 3005u32,
};
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_AUTHN_ERROR_END: u32 = 1279u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_AUTHN_ERROR_START: u32 = 1024u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_GENERAL_ERROR_END: u32 = 1023u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_GENERAL_ERROR_START: u32 = 512u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_PW_SILO_ERROR_END: u32 = 4607u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_PW_SILO_ERROR_START: u32 = 4352u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_COM_ERROR_END: u32 = 511u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_COM_ERROR_START: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_ERROR_END: u32 = 4095u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_ERROR_START: u32 = 1280u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_END: u32 = 49151u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_RESERVED_SILO_SPECIFIC_ERROR_START: u32 = 4608u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_VENDOR_ERROR_END: u32 = 65535u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ES_VENDOR_ERROR_START: u32 = 49152u32;
pub const EnhancedStorageACT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2936498709, 11982, 19156, [187, 33, 41, 240, 64, 225, 118, 216]);
pub const EnhancedStorageSilo: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3408208396, 30407, 20462, [132, 43, 243, 56, 60, 208, 34, 188]);
pub const EnhancedStorageSiloAction: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2288855517, 46342, 18027, [159, 191, 180, 79, 243, 131, 251, 63]);
pub const EnumEnhancedStorageACT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4270068883, 33628, 20387, [182, 204, 180, 178, 212, 113, 152, 72]);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FACILITY_ENHANCED_STORAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_COMPLETE_PINNED: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_EXCLUDED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_FOLDER_EMPTY: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_NOTAVAILABLEOFFLINE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FILEOFFLINEAVAILABILITYSTATUS_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FLAGSTATUS_COMPLETED: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FLAGSTATUS_FOLLOWUP: i32 = 2i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const FLAGSTATUS_NOTFLAGGED: i32 = 0i32;
pub const GUID_DEVINTERFACE_ENHANCED_STORAGE_SILO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(949483172, 64821, 19400, [160, 183, 93, 187, 163, 106, 218, 250]);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageACT(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageACT {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Unauthorize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::runtime::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__: <ACT_AUTHORIZATION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageACT {
    type Vtable = IEnhancedStorageACT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1853325812, 57586, 16953, [185, 118, 160, 26, 186, 181, 41, 48]);
}
impl ::core::convert::From<IEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageACT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageACT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesilos: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageACT2(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageACT2 {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Unauthorize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::runtime::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__: <ACT_AUTHORIZATION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageACT2 {
    type Vtable = IEnhancedStorageACT2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1302691118, 36531, 16886, [160, 126, 152, 181, 43, 136, 36, 43]);
}
impl ::core::convert::From<IEnhancedStorageACT2> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageACT2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageACT2> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageACT2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IEnhancedStorageACT2> for IEnhancedStorageACT {
    fn from(value: IEnhancedStorageACT2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT2> for IEnhancedStorageACT {
    fn from(value: &IEnhancedStorageACT2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for &IEnhancedStorageACT2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesilos: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageACT3(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageACT3 {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hwndparent), ::core::mem::transmute(dwflags)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Unauthorize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetAuthorizationState(&self) -> ::windows::runtime::Result<ACT_AUTHORIZATION_STATE> {
        let mut result__: <ACT_AUTHORIZATION_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<ACT_AUTHORIZATION_STATE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingVolume(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetUniqueIdentity(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesilos), ::core::mem::transmute(pcenhancedstoragesilos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn IsDeviceRemovable(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn UnauthorizeEx(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn IsQueueFrozen(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetShellExtSupport(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageACT3 {
    type Vtable = IEnhancedStorageACT3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(35737761, 4413, 4575, [187, 97, 0, 26, 160, 27, 188, 88]);
}
impl ::core::convert::From<IEnhancedStorageACT3> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageACT3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for IEnhancedStorageACT2 {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for IEnhancedStorageACT2 {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT2> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT2> for &IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEnhancedStorageACT3> for IEnhancedStorageACT {
    fn from(value: IEnhancedStorageACT3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnhancedStorageACT3> for IEnhancedStorageACT {
    fn from(value: &IEnhancedStorageACT3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEnhancedStorageACT> for &IEnhancedStorageACT3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEnhancedStorageACT> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageACT3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesilos: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageSilo(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageSilo {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetInfo(&self) -> ::windows::runtime::Result<SILO_INFO> {
        let mut result__: <SILO_INFO as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<SILO_INFO>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetActions(&self, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstoragesiloactions), ::core::mem::transmute(pcenhancedstoragesiloactions)).ok()
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn SendCommand(&self, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(command), ::core::mem::transmute(pbcommandbuffer), ::core::mem::transmute(cbcommandbuffer), ::core::mem::transmute(pbresponsebuffer), ::core::mem::transmute(pcbresponsebuffer)).ok()
    }
    #[cfg(feature = "Win32_Devices_PortableDevices")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetPortableDevice(&self) -> ::windows::runtime::Result<super::super::Devices::PortableDevices::IPortableDevice> {
        let mut result__: <super::super::Devices::PortableDevices::IPortableDevice as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Devices::PortableDevices::IPortableDevice>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDevicePath(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageSilo {
    type Vtable = IEnhancedStorageSilo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1525643462, 8770, 18179, [191, 73, 68, 178, 147, 87, 163, 89]);
}
impl ::core::convert::From<IEnhancedStorageSilo> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageSilo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageSilo> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageSilo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageSilo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageSilo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageSilo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psiloinfo: *mut SILO_INFO) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstoragesiloactions: *mut *mut ::windows::runtime::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Devices_PortableDevices")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiportabledevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_PortableDevices"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszsilodevicepath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnhancedStorageSiloAction(pub ::windows::runtime::IUnknown);
impl IEnhancedStorageSiloAction {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetName(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetDescription(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn Invoke(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnhancedStorageSiloAction {
    type Vtable = IEnhancedStorageSiloAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3069702929, 8303, 20472, [156, 75, 39, 239, 238, 119, 168, 111]);
}
impl ::core::convert::From<IEnhancedStorageSiloAction> for ::windows::runtime::IUnknown {
    fn from(value: IEnhancedStorageSiloAction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnhancedStorageSiloAction> for ::windows::runtime::IUnknown {
    fn from(value: &IEnhancedStorageSiloAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnhancedStorageSiloAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnhancedStorageSiloAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnhancedStorageSiloAction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszactionname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszactiondescription: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumEnhancedStorageACT(pub ::windows::runtime::IUnknown);
impl IEnumEnhancedStorageACT {
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
    pub unsafe fn GetACTs(&self, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppienhancedstorageacts), ::core::mem::transmute(pcenhancedstorageacts)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_Foundation`*"]
    pub unsafe fn GetMatchingACT<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, szvolume: Param0) -> ::windows::runtime::Result<IEnhancedStorageACT> {
        let mut result__: <IEnhancedStorageACT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), szvolume.into_param().abi(), &mut result__).from_abi::<IEnhancedStorageACT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumEnhancedStorageACT {
    type Vtable = IEnumEnhancedStorageACT_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(162669757, 4917, 17969, [167, 255, 207, 211, 169, 38, 70, 215]);
}
impl ::core::convert::From<IEnumEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: IEnumEnhancedStorageACT) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumEnhancedStorageACT> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumEnhancedStorageACT) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumEnhancedStorageACT {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEnhancedStorageACT_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppienhancedstorageacts: *mut *mut ::windows::runtime::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, szvolume: super::super::Foundation::PWSTR, ppienhancedstorageact: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_HIGH_MAX: i32 = 5i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_HIGH_MIN: i32 = 5i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_HIGH_SET: i32 = 5i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_LOW_MAX: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_LOW_MIN: i32 = 0i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_LOW_SET: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_NORMAL_MAX: i32 = 4i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_NORMAL_MIN: i32 = 2i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const IMPORTANCE_NORMAL_SET: i32 = 3i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_NONOWNER: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const ISDEFAULTSAVE_OWNER: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const LINK_STATUS_BROKEN: i32 = 2i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const LINK_STATUS_RESOLVED: i32 = 1i32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINEAVAILABILITY_ALWAYS_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINEAVAILABILITY_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINEAVAILABILITY_NOT_AVAILABLE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_ERROR: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_FORCED: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_ITEM_VERSION_CONFLICT: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_SLOW: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_OFFLINE_SUSPENDED: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const OFFLINESTATUS_ONLINE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_CONTRAST_HARD: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_CONTRAST_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_CONTRAST_SOFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_ACTION: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_APERTURE: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_CREATIVE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_LANDSCAPE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_MANUAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_NORMAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_PORTRAIT: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_SHUTTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_EXPOSUREPROGRAM_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO: u32 = 25u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_NORETURNLIGHT: u32 = 29u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE: u32 = 89u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_NORETURNLIGHT: u32 = 93u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_REDEYE_RETURNLIGHT: u32 = 95u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_AUTO_RETURNLIGHT: u32 = 31u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY: u32 = 9u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_NORETURNLIGHT: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE: u32 = 73u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_NORETURNLIGHT: u32 = 77u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_REDEYE_RETURNLIGHT: u32 = 79u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_COMPULSORY_RETURNLIGHT: u32 = 15u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_REDEYE: u32 = 65u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_REDEYE_NORETURNLIGHT: u32 = 69u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_FLASH_REDEYE_RETURNLIGHT: u32 = 71u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NOFUNCTION: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NONE_AUTO: u32 = 24u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_NONE_COMPULSORY: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_WITHOUTSTROBE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_FLASH_WITHSTROBE: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_HIGHGAINDOWN: f64 = 4f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_HIGHGAINUP: f64 = 2f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_LOWGAINDOWN: f64 = 3f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_LOWGAINUP: f64 = 1f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_GAINCONTROL_NONE: f64 = 0f64;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_D55: u32 = 20u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_D65: u32 = 21u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_D75: u32 = 22u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_DAYLIGHT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_FLUORESCENT: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_STANDARD_A: u32 = 17u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_STANDARD_B: u32 = 18u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_STANDARD_C: u32 = 19u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_TUNGSTEN: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_LIGHTSOURCE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_ACTION: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_APERTURE: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_CREATIVE: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_LANDSCAPE: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_MANUAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_NORMAL: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_NOTDEFINED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_PORTRAIT: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_PROGRAMMODE_SHUTTER: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SATURATION_HIGH: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SATURATION_LOW: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SATURATION_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SHARPNESS_HARD: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SHARPNESS_NORMAL: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_SHARPNESS_SOFT: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_WHITEBALANCE_AUTO: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PHOTO_WHITEBALANCE_MANUAL: u32 = 1u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AcquisitionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1705609333, 15488, 16555, [171, 188, 239, 218, 247, 125, 190, 226]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Address_Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3229303193, 57823, 17555, [177, 225, 222, 89, 70, 251, 88, 248]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Address_CountryCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3229303193, 57823, 17555, [177, 225, 222, 89, 70, 251, 88, 248]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Address_Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3229303193, 57823, 17555, [177, 225, 222, 89, 70, 251, 88, 248]),
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Address_RegionCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3229303193, 57823, 17555, [177, 225, 222, 89, 70, 251, 88, 248]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Address_Town: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3229303193, 57823, 17555, [177, 225, 222, 89, 70, 251, 88, 248]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_ExcludeFromShowInNewInstall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_IsDestListSeparator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_IsDualMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_PreventPinning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_RelaunchCommand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_RelaunchDisplayNameResource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_RelaunchIconResource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_SettingsCommand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_StartPinOption: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_ToastActivatorCLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_UninstallCommand: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppUserModel_VisualElementsManifestHintPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2672568405, 40825, 19257, [168, 208, 225, 212, 45, 225, 213, 243]),
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_AppZoneIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1345126059, 18411, 17820, [185, 96, 230, 216, 114, 143, 119, 1]),
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ApplicationDefinedProperties: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3451896167, 13182, 16856, [175, 124, 140, 9, 32, 84, 41, 199]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ApplicationName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_ChannelCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_Compression: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_EncodingBitrate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_Format: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_IsVariableBitRate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867291630, 35863, 19810, [130, 60, 142, 156, 252, 189, 29, 92]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_PeakValue: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(628745680, 4374, 16516, [189, 154, 155, 79, 124, 180, 223, 94]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_SampleRate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_SampleSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_StreamName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Audio_StreamNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Author: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CachedFileUpdaterContentIdForConflictResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CachedFileUpdaterContentIdForStream: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_Duration: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(691839834, 2474, 19922, [177, 128, 31, 226, 69, 114, 138, 82]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_IsOnline: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3220083017, 58338, 18855, [168, 98, 192, 89, 136, 20, 92, 236]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_IsRecurring: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(828087437, 32937, 20217, [174, 22, 142, 116, 109, 165, 29, 112]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_Location: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4129762584, 52940, 16561, [178, 106, 57, 17, 113, 122, 167, 189]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_OptionalAttendeeAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3579555418, 14482, 16762, [166, 73, 198, 172, 90, 170, 234, 179]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_OptionalAttendeeNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(155358727, 22573, 17279, [132, 195, 222, 147, 162, 178, 76, 60]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_OrganizerAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1951171138, 19957, 17772, [171, 158, 1, 78, 251, 144, 33, 227]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_OrganizerName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2863030521, 39013, 17806, [180, 132, 1, 188, 127, 227, 151, 62]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_ReminderTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1929141156, 9465, 16401, [159, 63, 173, 210, 122, 250, 216, 24]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_RequiredAttendeeAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(195548867, 22157, 16729, [171, 145, 120, 26, 145, 251, 113, 229]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_RequiredAttendeeNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3006984971, 62802, 17796, [147, 108, 203, 147, 229, 205, 162, 159]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_Resources: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16091704, 50507, 19520, [134, 150, 151, 35, 89, 128, 234, 225]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_ResponseStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(411836305, 15424, 16690, [158, 197, 216, 176, 59, 114, 168, 162]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_ShowTimeAs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1542690516, 24242, 18031, [189, 233, 47, 179, 242, 54, 29, 110]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Calendar_ShowTimeAsText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1406818255, 25280, 17860, [129, 222, 118, 16, 188, 239, 215, 245]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Capacity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995061, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_AccountName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_DateItemExpires: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1115701420, 41335, 19594, [151, 96, 246, 247, 97, 34, 127, 154]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_Direction: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2387808304, 47456, 17222, [174, 13, 102, 188, 154, 134, 251, 148]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_FollowupIconIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2208707710, 28644, 20288, [186, 156, 196, 134, 82, 64, 209, 244]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_HeaderItem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3385020292, 8769, 17409, [182, 7, 189, 32, 237, 117, 174, 127]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_PolicyTag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3960160657, 43787, 19558, [144, 182, 198, 99, 124, 222, 187, 171]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_SecurityFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2249827510, 40781, 17449, [140, 15, 185, 150, 202, 89, 227, 53]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_Suffix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2155570490, 40593, 17391, [143, 151, 17, 206, 4, 238, 32, 197]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_TaskStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3189404358, 39453, 18103, [175, 231, 175, 175, 140, 239, 73, 153]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Communication_TaskStatusText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2792637559, 49719, 18267, [160, 117, 84, 243, 68, 152, 41, 42]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Company: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ComputerName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Computer_DecoratedFreeSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995061, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_AccountPictureDynamicVideo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(193703960, 10021, 19268, [146, 186, 121, 51, 174, 178, 221, 231]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_AccountPictureLarge: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(193703960, 10021, 19268, [146, 186, 121, 51, 174, 178, 221, 231]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_AccountPictureSmall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(193703960, 10021, 19268, [146, 186, 121, 51, 174, 178, 221, 231]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Anniversary: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2597698267, 52903, 17520, [160, 61, 184, 78, 81, 185, 148, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_AssistantName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3440389276, 21824, 19080, [166, 246, 100, 228, 152, 28, 140, 209]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_AssistantTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2593334349, 42925, 20472, [155, 153, 69, 238, 76, 192, 154, 246]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Birthday: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 47u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1930409693, 53116, 17003, [160, 63, 189, 22, 108, 201, 238, 36]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress1Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 119u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress1Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 117u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress1PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 120u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress1Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 118u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress1Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 116u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress2Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 124u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress2Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 122u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress2PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 125u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress2Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 123u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress2Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 121u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress3Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 129u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress3Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 127u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress3PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 130u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress3Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 128u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddress3Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 126u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1076582708, 60506, 18627, [147, 230, 133, 232, 106, 45, 147, 78]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2964878100, 64758, 20459, [141, 255, 165, 13, 166, 175, 86, 28]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3159257550, 6137, 18645, [190, 233, 2, 29, 240, 234, 84, 9]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3788808350, 55128, 19665, [182, 236, 52, 168, 181, 167, 63, 128]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1148156031, 4292, 16843, [166, 196, 77, 3, 67, 85, 21, 151]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3721479695, 49343, 17747, [140, 228, 16, 67, 60, 144, 143, 176]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessEmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4067542617, 32350, 18207, [186, 37, 127, 119, 178, 134, 248, 54]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessFaxNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2448422643, 11815, 17098, [147, 62, 124, 153, 159, 190, 49, 11]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessHomePage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1446054176, 9361, 18713, [153, 206, 234, 219, 6, 250, 253, 178]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_BusinessTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1779819936, 2590, 19671, [187, 140, 210, 241, 176, 201, 41, 188]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_CallbackTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3209941443, 18912, 20351, [133, 103, 90, 130, 29, 138, 197, 66]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_CarTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2413587946, 47401, 16683, [186, 144, 57, 122, 37, 116, 101, 254]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3564279556, 36593, 17391, [144, 36, 43, 211, 129, 24, 127, 213]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_CompanyMainTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2240406657, 24640, 18237, [177, 113, 127, 168, 156, 39, 8, 237]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_ConnectedServiceDisplayName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(968326991, 41220, 18531, [179, 149, 45, 178, 173, 143, 123, 193]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_ConnectedServiceIdentities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2163482296, 44996, 16904, [170, 95, 204, 226, 26, 98, 114, 129]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_ConnectedServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3049802910, 22823, 18101, [163, 204, 147, 60, 33, 183, 132, 105]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_ConnectedServiceSupportedActions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2711599017, 587, 17265, [168, 191, 77, 41, 195, 228, 233, 201]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_DataSuppliers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2522923651, 64570, 18952, [160, 150, 238, 211, 170, 196, 109, 162]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4238308102, 65423, 19785, [159, 182, 63, 254, 92, 9, 81, 236]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_DisplayBusinessPhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(910174426, 55445, 16894, [165, 132, 48, 43, 27, 183, 10, 118]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_DisplayHomePhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1349041375, 54935, 19845, [140, 83, 31, 28, 218, 176, 23, 99]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_DisplayMobilePhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2628830040, 40314, 18097, [180, 102, 220, 198, 241, 163, 217, 61]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_DisplayOtherPhoneNumbers: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(50894963, 36584, 16785, [189, 96, 211, 31, 114, 183, 144, 11]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_EmailAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4177166243, 53547, 18309, [138, 78, 105, 26, 148, 247, 163, 231]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_EmailAddress2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(949375075, 60872, 17000, [132, 145, 183, 114, 49, 114, 207, 41]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_EmailAddress3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682782132, 57779, 19373, [176, 153, 126, 124, 4, 150, 106, 202]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_EmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2228810551, 38941, 17587, [150, 21, 199, 89, 109, 186, 23, 227]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_EmailName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3429846820, 24707, 19412, [135, 84, 103, 77, 13, 232, 122, 184]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_FileAsName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4053944999, 40103, 16630, [137, 236, 151, 222, 249, 255, 232, 219]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_FirstName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(345471044, 27465, 19117, [167, 20, 164, 81, 59, 246, 4, 96]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_FullName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1667141713, 20645, 19362, [185, 219, 78, 208, 86, 199, 114, 150]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Gender: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1015869016, 54512, 19705, [183, 86, 78, 93, 36, 68, 123, 205]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_GenderValue: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1015869016, 54512, 19705, [183, 86, 78, 93, 36, 68, 123, 205]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Hobbies: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1573004607, 24081, 19167, [156, 254, 145, 13, 208, 30, 62, 112]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2566488916, 24954, 18104, [133, 96, 91, 27, 100, 191, 31, 137]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress1Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress1Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress1PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress1Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress1Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress2Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 109u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress2Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress2PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress2Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress2Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress3Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress3Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress3PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 115u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress3Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddress3Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 111u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 65u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(145119905, 62665, 17373, [157, 223, 163, 61, 142, 126, 173, 133]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2074043289, 2623, 19218, [137, 189, 74, 220, 81, 201, 24, 175]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2331820400, 35398, 19283, [158, 238, 144, 186, 231, 21, 30, 98]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3365544912, 32109, 20152, [135, 212, 119, 106, 130, 212, 147, 229]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(182382944, 56127, 17160, [154, 33, 6, 35, 123, 22, 250, 42]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeEmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1456017053, 40262, 18787, [136, 111, 46, 28, 217, 166, 148, 239]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeFaxNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1712194774, 33195, 18807, [160, 159, 130, 49, 49, 19, 171, 38]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_HomeTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_IMAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3599613322, 13172, 19329, [153, 114, 62, 195, 6, 130, 219, 61]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Initials: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4091081741, 20683, 17570, [151, 24, 64, 203, 145, 25, 73, 93]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JA_CompanyNamePhonetic: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2306553492, 65182, 17382, [128, 102, 38, 15, 89, 12, 1, 0]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JA_FirstNamePhonetic: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2306553492, 65182, 17382, [128, 102, 38, 15, 89, 12, 1, 0]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JA_LastNamePhonetic: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2306553492, 65182, 17382, [128, 102, 38, 15, 89, 12, 1, 0]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1CompanyAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 120u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1CompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo1YomiCompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2CompanyAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 121u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2CompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 113u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 109u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo2YomiCompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3CompanyAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 123u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3CompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 115u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3Department: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 119u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 118u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 117u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 116u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobInfo3YomiCompanyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 114u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_JobTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Label: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2544938377, 57161, 18892, [131, 78, 102, 9, 116, 253, 117, 91]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_LastName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2402710016, 49776, 17788, [177, 212, 224, 124, 91, 205, 144, 199]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_MailingAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3232505962, 33406, 18000, [149, 174, 119, 226, 187, 116, 252, 201]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_MiddleName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 71u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_MobileTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_NickName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 74u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OfficeLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1350656506, 12603, 17365, [131, 161, 193, 172, 207, 104, 98, 44]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress1Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 134u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress1Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 132u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress1PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 135u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress1Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 133u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress1Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 131u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress2Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 139u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress2Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 137u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress2PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 140u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress2Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 138u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress2Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 136u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress3Country: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 144u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress3Locality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 142u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress3PostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 145u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress3Region: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 143u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddress3Street: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2813785494, 54904, 19393, [176, 95, 2, 3, 210, 126, 138, 161]),
    pid: 141u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1852320035, 32635, 20236, [163, 55, 207, 202, 41, 102, 135, 191]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2400613736, 2734, 17186, [142, 217, 96, 85, 183, 176, 227, 152]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2334583361, 1423, 17398, [174, 204, 64, 53, 104, 28, 233, 119]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2512803521, 10943, 16712, [158, 211, 158, 198, 2, 227, 183, 205]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1907587030, 58736, 16991, [161, 112, 128, 159, 174, 115, 229, 78]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4288030217, 47062, 18841, [134, 45, 149, 24, 13, 82, 154, 234]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_OtherEmailAddresses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(299250539, 14532, 20169, [132, 214, 235, 56, 208, 177, 80, 175]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PagerTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3593489921, 63733, 20293, [139, 21, 208, 36, 166, 41, 103, 137]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PersonalTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 69u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PhoneNumbersCanonical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3494040225, 37502, 16565, [165, 3, 110, 219, 212, 42, 81, 126]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Prefix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 75u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryAddressCity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3370816752, 43491, 18793, [169, 75, 156, 98, 169, 83, 36, 224]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryAddressCountry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3846011293, 3903, 18030, [178, 255, 116, 99, 74, 60, 183, 164]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryAddressPostOfficeBox: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3730764743, 18145, 18510, [153, 153, 98, 197, 48, 131, 148, 193]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryAddressPostalCode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(414962725, 60669, 18159, [182, 18, 123, 74, 96, 52, 237, 160]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryAddressState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4044844542, 28984, 17984, [139, 76, 174, 55, 93, 199, 10, 109]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryAddressStreet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1673681696, 38590, 18575, [135, 136, 192, 156, 64, 122, 216, 18]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryEmailAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 48u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_PrimaryTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Profession: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1919463253, 7396, 20334, [164, 31, 182, 228, 239, 16, 228, 169]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_SpouseName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2636384438, 12647, 16939, [130, 176, 245, 131, 183, 167, 207, 227]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Suffix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(393070140, 9864, 20105, [129, 67, 163, 71, 128, 15, 37, 233]),
    pid: 73u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_TTYTDDTelephone: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2867948460, 11093, 17894, [159, 109, 65, 94, 185, 73, 16, 223]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_TelexNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3310635324, 49655, 16577, [167, 108, 239, 140, 6, 20, 0, 62]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_WebPage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Webpage2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 124u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Contact_Webpage3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(16137688, 8893, 19037, [186, 52, 92, 176, 185, 189, 203, 3]),
    pid: 125u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ContainedItems: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ContentId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 132u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ContentStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ContentType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ContentUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 131u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Copyright: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CreatorAppId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3270116462, 828, 20113, [189, 91, 212, 148, 47, 107, 190, 73]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_CreatorOpenWithUIOptions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3270116462, 828, 20113, [189, 91, 212, 148, 47, 107, 190, 73]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DRM_DatePlayExpires: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2930514404, 35246, 17672, [185, 183, 187, 134, 122, 190, 226, 237]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DRM_DatePlayStarts: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2930514404, 35246, 17672, [185, 183, 187, 134, 122, 190, 226, 237]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DRM_Description: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2930514404, 35246, 17672, [185, 183, 187, 134, 122, 190, 226, 237]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DRM_IsDisabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2930514404, 35246, 17672, [185, 183, 187, 134, 122, 190, 226, 237]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DRM_IsProtected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2930514404, 35246, 17672, [185, 183, 187, 134, 122, 190, 226, 237]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DRM_PlayCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2930514404, 35246, 17672, [185, 183, 187, 134, 122, 190, 226, 237]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DataObjectFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(511812600, 41743, 16967, [185, 238, 29, 3, 104, 169, 66, 92]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateAccessed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateAcquired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(750430453, 55327, 18378, [177, 122, 248, 216, 34, 48, 1, 49]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateArchived: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1140381623, 42052, 20359, [147, 131, 82, 39, 28, 155, 145, 92]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateCompleted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1929033601, 44250, 17381, [177, 85, 178, 67, 79, 133, 230, 120]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateCreated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateImported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 18258u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DateModified: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DefaultSaveLocationDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DescriptionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_Flags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_LastConnectedTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_ProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_ProductVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_ServiceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_VendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Bluetooth_VendorIdSource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Hid_IsReadOnly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3421733648, 18967, 17168, [161, 235, 36, 127, 11, 103, 89, 59]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Hid_ProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3421733648, 18967, 17168, [161, 235, 36, 127, 11, 103, 89, 59]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Hid_UsageId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3421733648, 18967, 17168, [161, 235, 36, 127, 11, 103, 89, 59]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Hid_UsagePage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3421733648, 18967, 17168, [161, 235, 36, 127, 11, 103, 89, 59]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Hid_VendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3421733648, 18967, 17168, [161, 235, 36, 127, 11, 103, 89, 59]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Hid_VersionNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3421733648, 18967, 17168, [161, 235, 36, 127, 11, 103, 89, 59]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_PrinterDriverDirectory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2222745310, 47318, 19193, [171, 195, 111, 79, 146, 107, 192, 57]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_PrinterDriverName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948886896, 5365, 18828, [143, 48, 176, 209, 155, 228, 73, 198]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_PrinterEnumerationFlag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2684830369, 52620, 19255, [149, 171, 112, 117, 85, 135, 118, 122]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_PrinterName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(175867119, 3111, 17983, [132, 239, 6, 197, 7, 0, 1, 190]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_PrinterPortName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4006066017, 28564, 16817, [148, 159, 199, 41, 114, 13, 209, 60]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Proximity_SupportsNfc: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4214768333, 40490, 20355, [143, 204, 75, 7, 97, 19, 154, 233]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Serial_PortName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1282142556, 19459, 19116, [145, 245, 100, 192, 248, 82, 188, 244]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Serial_UsbProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1282142556, 19459, 19116, [145, 245, 100, 192, 248, 82, 188, 244]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_Serial_UsbVendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1282142556, 19459, 19116, [145, 245, 100, 192, 248, 82, 188, 244]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_WinUsb_DeviceInterfaceClasses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2514560949, 31180, 20099, [156, 158, 132, 34, 24, 123, 62, 14]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_WinUsb_UsbClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2514560949, 31180, 20099, [156, 158, 132, 34, 24, 123, 62, 14]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_WinUsb_UsbProductId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2514560949, 31180, 20099, [156, 158, 132, 34, 24, 123, 62, 14]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_WinUsb_UsbProtocol: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2514560949, 31180, 20099, [156, 158, 132, 34, 24, 123, 62, 14]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_WinUsb_UsbSubClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2514560949, 31180, 20099, [156, 158, 132, 34, 24, 123, 62, 14]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DeviceInterface_WinUsb_UsbVendorId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2514560949, 31180, 20099, [156, 158, 132, 34, 24, 123, 62, 14]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Device_PrinterURL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(189330266, 48750, 20247, [177, 8, 60, 64, 115, 209, 102, 154]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_CanPair: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_Categories: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_DialProtocol_InstalledApplications: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_IsPresent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_ModelIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_ProtocolIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(196746974, 30054, 20295, [144, 236, 37, 252, 86, 124, 237, 42]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportedUriSchemes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsAudio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsCapturing: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsImages: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsLimitedDiscovery: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsNetworking: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsObjectTransfer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsPositioning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsRendering: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsTelephony: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepContainer_SupportsVideo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1794465093, 14555, 17557, [172, 176, 212, 114, 138, 59, 131, 20]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_AepId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3384885673, 6988, 20247, [169, 209, 242, 152, 83, 140, 173, 184]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_Bluetooth_CacheMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2537828638, 31057, 19246, [182, 240, 236, 178, 147, 202, 193, 25]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_Bluetooth_ServiceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2744756935, 49765, 18254, [176, 115, 255, 206, 87, 114, 23, 22]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_Bluetooth_TargetDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2537828638, 31057, 19246, [182, 240, 236, 178, 147, 202, 193, 25]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1903314774, 15988, 17458, [155, 89, 231, 178, 246, 104, 165, 147]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1903314774, 15988, 17458, [155, 89, 231, 178, 246, 104, 165, 147]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_IoT_ServiceInterfaces: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2044284546, 19833, 17834, [130, 26, 116, 133, 139, 78, 76, 166]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_ParentAepIsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3384885673, 6988, 20247, [169, 209, 242, 152, 83, 140, 173, 184]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_ProtocolId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3384885673, 6988, 20247, [169, 209, 242, 152, 83, 140, 173, 184]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_ServiceClassId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1903314774, 15988, 17458, [155, 89, 231, 178, 246, 104, 165, 147]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AepService_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3384885673, 6988, 20247, [169, 209, 242, 152, 83, 140, 173, 184]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_AepId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(992796678, 24161, 20446, [186, 184, 155, 138, 172, 155, 38, 223]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Major: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Minor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Audio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Capturing: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Information: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Networking: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_ObjectXfer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Positioning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Rendering: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Cod_Services_Telephony: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1606235341, 22042, 16686, [186, 152, 71, 138, 107, 15, 239, 29]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_LastSeenTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735477131, 35819, 18645, [135, 224, 108, 218, 52, 40, 4, 10]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Le_AddressType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2573136048, 32435, 19083, [185, 206, 6, 139, 179, 244, 175, 105]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2573136048, 32435, 19083, [185, 206, 6, 139, 179, 244, 175, 105]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2573136048, 32435, 19083, [185, 206, 6, 139, 179, 244, 175, 105]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Le_Appearance_Subcategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2573136048, 32435, 19083, [185, 206, 6, 139, 179, 244, 175, 105]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Bluetooth_Le_IsConnectable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2573136048, 32435, 19083, [185, 206, 6, 139, 179, 244, 175, 105]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_CanPair: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3888380713, 51879, 20295, [140, 139, 190, 89, 179, 48, 212, 197]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3888380713, 51879, 20295, [140, 139, 190, 89, 179, 48, 212, 197]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_IsPaired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_IsPresent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_PointOfService_ConnectionTypes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3569312179, 17454, 19162, [136, 45, 250, 123, 112, 200, 50, 217]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_ProtocolId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(992796678, 24161, 20446, [186, 184, 155, 138, 172, 155, 38, 223]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Aep_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2740557483, 4559, 18741, [139, 97, 166, 118, 16, 129, 236, 223]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AppPackageFamilyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1361274243, 3146, 20456, [184, 31, 22, 106, 236, 19, 245, 16]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AudioDevice_Microphone_IsFarField: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2302915443, 14476, 17301, [181, 87, 188, 109, 186, 255, 175, 219]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2302915443, 14476, 17301, [181, 87, 188, 109, 186, 255, 175, 219]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AudioDevice_Microphone_SensitivityInDbfs2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2302915443, 14476, 17301, [181, 87, 188, 109, 186, 255, 175, 219]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2302915443, 14476, 17301, [181, 87, 188, 109, 186, 255, 175, 219]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AudioDevice_RawProcessingSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2302915443, 14476, 17301, [181, 87, 188, 109, 186, 255, 175, 219]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_AudioDevice_SpeechProcessingSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4213041252, 57453, 18420, [130, 166, 138, 10, 239, 68, 73, 60]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_BatteryLife: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_BatteryPlusCharging: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_BatteryPlusChargingText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Category: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 91u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_CategoryGroup: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 94u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_CategoryIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 90u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_CategoryPlural: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 92u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ChallengeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(125055326, 46868, 18668, [141, 232, 129, 37, 192, 119, 172, 17]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ChargingState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Children: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1128310469, 37882, 18182, [151, 44, 123, 100, 128, 8, 165, 167]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_CompatibleIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 55u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ContainerId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2357121542, 16266, 18471, [179, 171, 174, 158, 31, 174, 252, 108]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DefaultTooltip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2282713250, 24706, 18348, [138, 171, 167, 57, 209, 163, 0, 195]),
    pid: 153u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DevObjectType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(325533506, 41942, 18934, [180, 218, 174, 70, 224, 197, 35, 124]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceCapabilities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceCharacteristics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceDescription1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 81u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceDescription2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 82u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceHasProblem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1410045054, 35648, 17852, [168, 162, 106, 11, 137, 76, 189, 162]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceInstanceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 256u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DeviceManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DialProtocol_InstalledApplications: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1749404786, 7025, 18627, [175, 134, 176, 145, 113, 161, 155, 20]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_DiscoveryMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 52u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_Domain: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_FullName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_HostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_InstanceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_NetworkAdapterId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_PortNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_Priority: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_ServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_TextAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_Ttl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Dnssd_Weight: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3212427435, 47988, 19694, [176, 112, 71, 11, 90, 226, 2, 234]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_FriendlyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 12288u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_FunctionPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3498955968, 15006, 17966, [130, 144, 123, 99, 107, 37, 118, 185]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_GlyphIcon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1361274243, 3146, 20456, [184, 31, 22, 106, 236, 19, 245, 16]),
    pid: 123u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_HardwareIds: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Icon: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 57u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_InLocalMachineContainer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2357121542, 16266, 18471, [179, 171, 174, 158, 31, 174, 252, 108]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_InterfaceClassGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(40784238, 47124, 16715, [131, 205, 133, 109, 111, 239, 72, 34]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_InterfaceEnabled: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(40784238, 47124, 16715, [131, 205, 133, 109, 111, 239, 72, 34]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_InterfacePaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3498955968, 15006, 17966, [130, 144, 123, 99, 107, 37, 118, 185]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_IpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 12297u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_IsDefault: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 86u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_IsNetworkConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 85u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_IsShared: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 84u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_IsSoftwareInstalling: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2212127526, 38822, 16520, [148, 83, 161, 146, 63, 87, 59, 41]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_LaunchDeviceStageFromExplorer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 77u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_LocalMachine: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 70u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_LocationPaths: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2757502286, 57116, 20221, [128, 32, 103, 209, 70, 168, 80, 224]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Manufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 8192u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_MetadataPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 71u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_MicrophoneArray_Geometry: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2709692066, 10219, 17822, [147, 93, 178, 250, 215, 176, 119, 98]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_MissedCalls: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ModelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2161647270, 29811, 19212, [130, 22, 239, 193, 26, 44, 76, 139]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ModelName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 8194u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ModelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 8195u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_NetworkName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_NetworkType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_NetworkedTooltip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2282713250, 24706, 18348, [138, 171, 167, 57, 209, 163, 0, 195]),
    pid: 152u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_NewPictures: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_NotWorkingProperly: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 83u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notification: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(108022540, 59440, 19585, [145, 120, 145, 228, 233, 90, 128, 160]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_NotificationStore: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(108022540, 59440, 19585, [145, 120, 145, 228, 233, 90, 128, 160]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notifications_LowBattery: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3300949803, 34084, 20070, [174, 58, 166, 35, 95, 16, 59, 235]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notifications_MissedCall: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1712648008, 20222, 17444, [158, 218, 199, 159, 64, 78, 223, 62]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notifications_NewMessage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(736699914, 8210, 18242, [165, 85, 244, 27, 99, 139, 125, 203]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notifications_NewVoicemail: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1498846550, 2568, 16914, [149, 185, 250, 226, 173, 100, 19, 219]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notifications_StorageFull: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2699038433, 61639, 19777, [184, 231, 38, 167, 189, 141, 56, 176]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Notifications_StorageFullLinkText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2699038433, 61639, 19777, [184, 231, 38, 167, 189, 141, 56, 176]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Paired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2026065864, 4170, 19146, [158, 164, 82, 77, 82, 153, 110, 87]),
    pid: 56u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Panel_PanelGroup: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2377948294, 38825, 19455, [155, 198, 191, 233, 93, 62, 109, 173]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Panel_PanelId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2377948294, 38825, 19455, [155, 198, 191, 233, 93, 62, 109, 173]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Parent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1128310469, 37882, 18182, [151, 44, 123, 100, 128, 8, 165, 167]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PhoneLineTransportDevice_Connected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2932813800, 7424, 20462, [138, 109, 167, 13, 113, 155, 119, 43]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PhysicalDeviceLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1410045054, 35648, 17852, [168, 162, 106, 11, 137, 76, 189, 162]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PlaybackPositionPercent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(909368921, 26661, 17281, [164, 155, 159, 107, 161, 58, 20, 113]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PlaybackState: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(909368921, 26661, 17281, [164, 155, 159, 107, 161, 58, 20, 113]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PlaybackTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(909368921, 26661, 17281, [164, 155, 159, 107, 161, 58, 20, 113]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Present: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1410045054, 35648, 17852, [168, 162, 106, 11, 137, 76, 189, 162]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PresentationUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 8198u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_PrimaryCategory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3498955968, 15006, 17966, [130, 144, 123, 99, 107, 37, 118, 185]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_RemainingDuration: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(909368921, 26661, 17281, [164, 155, 159, 107, 161, 58, 20, 113]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_RestrictedInterface: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(40784238, 47124, 16715, [131, 205, 133, 109, 111, 239, 72, 34]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Roaming: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_SafeRemovalRequired: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2950264384, 34467, 16912, [182, 124, 40, 156, 65, 170, 190, 85]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_SchematicName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(40784238, 47124, 16715, [131, 205, 133, 109, 111, 239, 72, 34]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 16384u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_ServiceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1701460915, 60608, 17405, [132, 119, 74, 224, 64, 74, 150, 205]),
    pid: 16385u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_SharedTooltip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2282713250, 24706, 18348, [138, 171, 167, 57, 209, 163, 0, 195]),
    pid: 151u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_SignalStrength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_SmartCards_ReaderKind: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3602233475, 6333, 19277, [178, 236, 158, 56, 175, 254, 218, 130]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3498955968, 15006, 17966, [130, 144, 123, 99, 107, 37, 118, 185]),
    pid: 259u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Status1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3498955968, 15006, 17966, [130, 144, 123, 99, 107, 37, 118, 185]),
    pid: 257u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Status2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3498955968, 15006, 17966, [130, 144, 123, 99, 107, 37, 118, 185]),
    pid: 258u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_StorageCapacity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_StorageFreeSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_StorageFreeSpacePercent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_TextMessages: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Voicemail: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238179702, 22054, 19223, [164, 232, 24, 180, 170, 26, 34, 19]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirectServices_AdvertisementId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(833845059, 31838, 16389, [147, 230, 233, 83, 249, 43, 130, 233]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirectServices_RequestServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(833845059, 31838, 16389, [147, 230, 233, 83, 249, 43, 130, 233]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirectServices_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(833845059, 31838, 16389, [147, 230, 233, 83, 249, 43, 130, 233]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirectServices_ServiceConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(833845059, 31838, 16389, [147, 230, 233, 83, 249, 43, 130, 233]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirectServices_ServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(833845059, 31838, 16389, [147, 230, 233, 83, 249, 43, 130, 233]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirectServices_ServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(833845059, 31838, 16389, [147, 230, 233, 83, 249, 43, 130, 233]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_GroupId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_InformationElements: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_InterfaceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_IsLegacyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_IsMiracastLcpSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_IsVisible: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_MiracastVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_Services: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFiDirect_SupportedChannelList: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(352752477, 58343, 17679, [134, 55, 130, 35, 62, 190, 95, 110]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiFi_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4010895339, 52220, 17217, [165, 104, 167, 201, 26, 104, 152, 44]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WiaDeviceType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1809653702, 33039, 4560, [190, 199, 8, 0, 43, 226, 9, 47]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_WinPhone8CameraFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3082081820, 23140, 16775, [165, 46, 177, 83, 159, 53, 144, 153]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Devices_Wwan_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4279330795, 52220, 17217, [165, 104, 167, 201, 26, 104, 152, 44]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_ByteCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_CharacterCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_ClientID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(661486512, 23348, 20400, [170, 75, 21, 142, 209, 42, 24, 9]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_Contributor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4080275806, 55835, 17673, [155, 61, 17, 149, 4, 220, 122, 187]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_DateCreated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_DatePrinted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_DateSaved: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_Division: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(503340774, 48935, 17035, [176, 28, 121, 103, 106, 205, 40, 112]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_DocumentID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3767010760, 58261, 16607, [128, 210, 84, 240, 214, 196, 49, 84]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_HiddenSlideCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_LastAuthor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_LineCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_Manager: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_MultimediaClipCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_NoteCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_PageCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_ParagraphCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_PresentationFormat: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_RevisionNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_Security: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_SlideCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_Template: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_TotalEditingTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_Version: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Document_WordCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_DueDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1065644725, 57519, 19890, [128, 113, 197, 63, 231, 106, 231, 206]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_EdgeGesture_DisableTouchWhenFullscreen: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(852375730, 11418, 16817, [155, 197, 179, 120, 67, 148, 170, 68]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_EndDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3344935429, 38653, 18919, [156, 180, 159, 96, 16, 130, 213, 83]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ExpandoProperties: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1872891366, 53532, 19869, [161, 84, 100, 49, 118, 40, 193, 45]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileAllocationSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileAttributes: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217021779, 64100, 4561, [162, 3, 0, 0, 248, 31, 237, 238]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3841002044, 18918, 16477, [130, 136, 162, 59, 212, 238, 170, 108]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileFRN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1104108256, 63322, 18438, [189, 135, 89, 199, 217, 36, 142, 185]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileOfflineAvailabilityStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileOwner: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995060, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FilePlaceholderStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FileVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217021779, 64100, 4561, [162, 3, 0, 0, 248, 31, 237, 238]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FindData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FlagColor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1742705886, 3239, 19823, [183, 146, 5, 58, 62, 79, 3, 207]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FlagColorText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1173022535, 36394, 16558, [140, 191, 202, 82, 171, 166, 21, 42]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FlagStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FlagStatusText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3696557358, 6301, 18545, [170, 1, 8, 194, 245, 122, 74, 188]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FolderKind: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FolderNameDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FreeSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995061, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_FullText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(507439168, 48171, 18284, [130, 55, 42, 205, 26, 131, 155, 34]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Altitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2189351759, 23411, 17575, [137, 29, 253, 255, 171, 234, 53, 202]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_AltitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2016685515, 58200, 16709, [174, 154, 107, 254, 78, 15, 159, 81]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_AltitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(766320311, 33133, 16595, [158, 195, 201, 119, 59, 226, 170, 222]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_AltitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1185702557, 30186, 17685, [134, 127, 109, 196, 50, 28, 88, 68]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_AreaInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2536387390, 44158, 18929, [138, 223, 167, 13, 7, 169, 188, 171]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217643778, 6199, 17137, [166, 151, 167, 1, 122, 162, 137, 185]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DOPDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2696844485, 20666, 18555, [189, 53, 6, 84, 190, 136, 129, 237]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DOPNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1192651542, 13903, 19104, [159, 49, 226, 171, 61, 244, 73, 195]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Date: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(906151954, 3899, 17904, [133, 173, 96, 52, 104, 214, 148, 35]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestBearing: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3329051452, 59528, 18380, [185, 159, 157, 202, 62, 227, 77, 234]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestBearingDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2059203832, 31807, 18824, [172, 145, 141, 44, 46, 151, 236, 165]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestBearingNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3124436393, 34542, 19293, [162, 164, 162, 113, 164, 41, 240, 207]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestBearingRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2595767187, 10767, 19317, [187, 34, 114, 121, 120, 105, 119, 203]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestDistance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2839457284, 26628, 20260, [172, 129, 9, 178, 102, 69, 33, 24]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestDistanceDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2613234075, 44145, 16679, [157, 28, 37, 150, 208, 215, 220, 183]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestDistanceNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(735725530, 2246, 20449, [128, 188, 167, 47, 197, 23, 197, 208]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestDistanceRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3981308627, 34453, 17675, [133, 111, 245, 193, 197, 58, 203, 102]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLatitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2635955397, 23609, 17692, [134, 179, 146, 142, 45, 24, 204, 71]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLatitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(976691858, 32714, 18855, [153, 213, 228, 123, 178, 212, 231, 171]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLatitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3975460598, 54694, 17212, [187, 146, 64, 118, 101, 15, 200, 144]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLatitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3467124921, 52833, 18565, [161, 40, 0, 93, 144, 135, 193, 146]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLongitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1202283105, 52044, 18439, [138, 211, 64, 185, 217, 219, 198, 188]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLongitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1113418213, 18605, 18688, [141, 128, 110, 182, 184, 208, 172, 134]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLongitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2737111682, 64365, 18645, [154, 137, 219, 202, 206, 117, 204, 207]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_DestLongitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(405544614, 31772, 16515, [171, 75, 172, 108, 159, 78, 209, 40]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Differential: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2868178469, 48443, 19927, [191, 196, 71, 247, 123, 176, 15, 109]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_ImgDirection: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(373767313, 53271, 20185, [186, 77, 182, 186, 165, 93, 188, 248]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_ImgDirectionDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(280118677, 16802, 20000, [147, 194, 87, 97, 193, 57, 95, 50]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_ImgDirectionNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3696785351, 8799, 17911, [186, 199, 232, 19, 52, 182, 19, 10]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_ImgDirectionRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2762646967, 6864, 17503, [129, 26, 15, 143, 110, 103, 246, 181]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Latitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2267533311, 18536, 20166, [173, 91, 129, 185, 133, 33, 209, 171]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LatitudeDecimal: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(257281506, 20297, 17677, [146, 193, 220, 209, 99, 1, 177, 183]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LatitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(384185582, 11263, 18811, [189, 138, 67, 65, 173, 57, 238, 185]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LatitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2111482577, 52424, 16814, [183, 80, 178, 203, 128, 49, 174, 162]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LatitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(43778642, 23430, 18119, [172, 160, 39, 105, 255, 200, 227, 212]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Longitude: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3301235634, 46483, 18027, [187, 218, 208, 61, 39, 213, 228, 58]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LongitudeDecimal: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1182384565, 33869, 17808, [186, 245, 243, 34, 35, 31, 27, 129]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LongitudeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3194885996, 17716, 19756, [172, 229, 49, 222, 218, 193, 96, 107]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LongitudeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(45151881, 43284, 20037, [130, 29, 29, 218, 69, 46, 210, 196]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_LongitudeRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(870117931, 10453, 17996, [128, 53, 30, 233, 239, 210, 82, 120]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_MapDatum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(748870374, 60892, 16509, [190, 241, 119, 57, 66, 171, 250, 149]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_MeasureMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2685791581, 43754, 19800, [138, 134, 60, 88, 105, 32, 234, 11]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_ProcessingMethod: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1507106401, 33807, 19113, [169, 57, 226, 9, 155, 127, 99, 153]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Satellites: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1182721397, 7973, 17751, [173, 78, 184, 181, 139, 13, 156, 21]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Speed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3663530082, 28278, 19995, [186, 189, 112, 2, 27, 210, 84, 148]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_SpeedDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2098343258, 44638, 17205, [136, 65, 215, 30, 124, 231, 47, 83]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_SpeedNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2898906685, 49683, 18754, [139, 72, 109, 8, 32, 242, 28, 109]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_SpeedRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3975673033, 21583, 19821, [157, 152, 138, 215, 154, 218, 244, 83]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(307532276, 33167, 18098, [145, 181, 213, 55, 117, 54, 23, 178]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_Track: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1992333635, 31795, 18915, [158, 126, 205, 186, 135, 44, 250, 218]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_TrackDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3369177612, 502, 16576, [172, 134, 47, 58, 74, 208, 7, 112]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_TrackNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1881745140, 17574, 17377, [174, 113, 69, 98, 113, 22, 137, 59]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_TrackRef: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(903603966, 17603, 17408, [170, 174, 210, 199, 153, 196, 7, 232]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_GPS_VersionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(577785252, 50866, 19097, [142, 86, 241, 109, 248, 201, 37, 153]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_HighKeywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_History_SelectionCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(484497084, 21356, 17920, [176, 221, 126, 12, 102, 179, 80, 213]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_History_TargetUrlHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(484497084, 21356, 17920, [176, 221, 126, 12, 102, 179, 80, 213]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_History_VisitCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1556031367, 18639, 16904, [185, 14, 238, 94, 93, 66, 2, 148]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2725202684, 29510, 17049, [190, 71, 235, 26, 230, 19, 19, 159]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IdentityProvider_Name: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3111059323, 13770, 18997, [134, 7, 41, 227, 165, 76, 70, 234]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IdentityProvider_Picture: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(606410351, 22082, 18532, [153, 47, 152, 253, 152, 242, 148, 195]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_Blob: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2352714660, 47853, 6787, [154, 50, 16, 46, 227, 19, 246, 235]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_DisplayName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2103984073, 53589, 17832, [187, 31, 137, 209, 155, 203, 121, 47]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_InternetSid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1835883849, 9821, 18056, [159, 78, 31, 221, 51, 231, 204, 131]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_IsMeIdentity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2752546568, 2527, 17271, [157, 252, 109, 153, 152, 109, 90, 103]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_KeyProviderContext: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2725202684, 29510, 17049, [190, 71, 235, 26, 230, 19, 19, 159]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_KeyProviderName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2725202684, 29510, 17049, [190, 71, 235, 26, 230, 19, 19, 159]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_LogonStatusString: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4052610547, 13183, 17088, [158, 3, 206, 224, 135, 8, 168, 195]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_PrimaryEmailAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4240533539, 47853, 20260, [155, 50, 160, 152, 33, 23, 247, 250]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_PrimarySid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(723222558, 49345, 18823, [158, 197, 114, 250, 137, 129, 71, 135]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_ProviderData: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2829536146, 13851, 20122, [183, 34, 124, 74, 115, 48, 163, 18]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_ProviderID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1957158473, 64017, 19773, [160, 6, 219, 126, 8, 103, 89, 22]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_QualifiedUserName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3662810705, 62697, 18233, [172, 130, 2, 224, 169, 92, 144, 48]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_UniqueID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3848258480, 11104, 16928, [145, 142, 178, 30, 139, 241, 96, 22]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Identity_UserName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3291620611, 30922, 18886, [154, 204, 166, 142, 42, 253, 123, 107]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ImageParsingName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3614772960, 50852, 18668, [181, 62, 184, 123, 82, 230, 208, 115]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_BitDepth: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_ColorSpace: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 40961u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_CompressedBitsPerPixel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(910913449, 14251, 18474, [190, 43, 174, 2, 246, 13, 67, 24]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_CompressedBitsPerPixelDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(529024225, 9389, 17672, [157, 253, 83, 38, 164, 21, 206, 2]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_CompressedBitsPerPixelNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3524948296, 54060, 17956, [137, 0, 39, 114, 16, 247, 156, 15]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_Compression: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 259u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_CompressionText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1057547887, 12100, 19385, [166, 130, 172, 53, 210, 86, 35, 34]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_Dimensions: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_HorizontalResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_HorizontalSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_ImageID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(282770949, 12970, 19497, [191, 26, 99, 226, 210, 32, 88, 127]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_ResolutionUnit: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(431300518, 8082, 19036, [171, 72, 125, 240, 171, 214, 116, 68]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_VerticalResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Image_VerticalSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Importance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ImportanceText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2746390417, 30483, 19997, [187, 64, 23, 219, 133, 240, 24, 49]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_InfoTipText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_InternalName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217021779, 64100, 4561, [162, 3, 0, 0, 248, 31, 237, 238]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsAttachment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4064232028, 29089, 20392, [146, 47, 103, 142, 164, 166, 4, 8]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsDefaultNonOwnerSaveLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsDefaultSaveLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsDeleted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1557815240, 13294, 20467, [144, 148, 174, 123, 216, 134, 140, 77]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsEncrypted: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2430984526, 25739, 18470, [178, 170, 172, 175, 121, 14, 53, 19]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsFlagged: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1571309413, 58367, 17016, [134, 176, 162, 121, 103, 251, 221, 3]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsFlaggedComplete: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2800967890, 22009, 18654, [185, 9, 98, 14, 9, 10, 100, 124]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsIncomplete: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879528913, 11882, 19525, [137, 164, 97, 183, 142, 142, 112, 15]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsLocationSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsPinnedToNameSpaceTree: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsRead: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsSearchOnlyItem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsSendToTarget: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_IsShared: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4018687067, 11262, 16827, [170, 229, 118, 238, 223, 79, 153, 2]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemAuthors: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3500166922, 17962, 18596, [187, 47, 55, 6, 232, 141, 189, 125]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemClassType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(75913389, 11704, 16804, [187, 182, 172, 30, 241, 32, 126, 177]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4158354612, 17031, 16643, [175, 186, 241, 177, 61, 205, 117, 207]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemFolderNameDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemFolderPathDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemFolderPathDisplayNarrow: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3669831917, 67, 18313, [167, 248, 208, 19, 164, 115, 102, 34]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1804443764, 15196, 17340, [136, 111, 10, 44, 220, 224, 11, 111]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemNameDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemNameDisplayWithoutExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemNamePrefix: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3610329073, 42874, 16412, [140, 153, 61, 189, 214, 138, 221, 54]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemNameSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemParticipants: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3570444822, 39240, 16804, [170, 133, 217, 127, 249, 100, 105, 147]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemPathDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemPathDisplayNarrow: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemSubType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemTypeText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ItemUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1231625360, 32279, 4122, [169, 28, 8, 0, 43, 46, 205, 169]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Journal_Contacts: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3735537708, 7561, 19046, [148, 39, 164, 227, 222, 186, 188, 177]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Journal_EntryType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2512302588, 12909, 17988, [179, 150, 205, 62, 217, 14, 109, 223]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Keywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Kind: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(507439168, 48171, 18284, [130, 55, 42, 205, 26, 131, 155, 34]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_KindText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4031508373, 50565, 16791, [162, 183, 223, 70, 253, 201, 238, 109]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Language: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3587036418, 11932, 4123, [147, 151, 8, 0, 43, 44, 249, 174]),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LastSyncError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 107u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LastSyncWarning: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 128u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LastWriterPackageFamilyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1345126059, 18411, 17820, [185, 96, 230, 216, 114, 143, 119, 1]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LayoutPattern_ContentViewModeForBrowse: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 500u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LayoutPattern_ContentViewModeForSearch: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 501u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LibraryLocationsCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2424739527, 36743, 17650, [128, 237, 168, 193, 198, 137, 69, 117]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_Arguments: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1131357799, 5346, 20459, [179, 10, 20, 108, 83, 181, 182, 116]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_Comment: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3115627516, 11089, 19010, [181, 216, 50, 65, 70, 175, 207, 37]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_DateVisited: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1556031367, 18639, 16904, [185, 14, 238, 94, 93, 66, 2, 148]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_Description: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1556031367, 18639, 16904, [185, 14, 238, 94, 93, 66, 2, 148]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_FeedItemLocalId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2318375417, 15415, 18013, [168, 215, 105, 119, 122, 36, 109, 12]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3115627516, 11089, 19010, [181, 216, 50, 65, 70, 175, 207, 37]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetExtension: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2055042804, 46640, 19415, [149, 255, 55, 204, 81, 169, 117, 201]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetParsingPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3115627516, 11089, 19010, [181, 216, 50, 65, 70, 175, 207, 37]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetSFGAOFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3115627516, 11089, 19010, [181, 216, 50, 65, 70, 175, 207, 37]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetSFGAOFlagsStrings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3600031873, 54587, 17469, [173, 71, 94, 5, 157, 156, 210, 122]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1556031367, 18639, 16904, [185, 14, 238, 94, 93, 66, 2, 148]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetUrlHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2318375417, 15415, 18013, [168, 215, 105, 119, 122, 36, 109, 12]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Link_TargetUrlPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2318375417, 15415, 18013, [168, 215, 105, 119, 122, 36, 109, 12]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_LowKeywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_MIMEType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(191095632, 40140, 4560, [188, 219, 0, 128, 95, 204, 206, 4]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_AuthorUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_AverageLevel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(166581686, 45825, 17349, [153, 144, 208, 3, 2, 239, 253, 70]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ClassPrimaryID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ClassSecondaryID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_CollectionGroupID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_CollectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ContentDistributor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ContentID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_CreatorApplication: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_CreatorApplicationVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_DVDID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_DateEncoded: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(776692749, 20505, 18136, [136, 129, 85, 65, 76, 197, 202, 160]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_DateReleased: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3728854057, 26993, 17040, [180, 114, 245, 159, 46, 47, 49, 226]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_DlnaProfileID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3483573061, 21085, 18840, [187, 68, 63, 125, 129, 84, 47, 164]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_Duration: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179216, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_EncodedBy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_EncodingSettings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_EpisodeNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_FrameCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179215, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_MCDI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_MetadataContentProvider: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_Producer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_PromotionUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ProtectionType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ProviderRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ProviderStyle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 40u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_Publisher: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_SeasonNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_SeriesName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 42u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_SubTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_SubscriptionContentId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2599136890, 38468, 18557, [169, 44, 101, 117, 133, 237, 117, 26]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ThumbnailLargePath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 47u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ThumbnailLargeUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 48u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ThumbnailSmallPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 49u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_ThumbnailSmallUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 50u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_UniqueFileIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_UserNoAutoInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 41u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_UserWebUrl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_Writer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Media_Year: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_MediumKeywords: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_AttachmentContents: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(826523516, 32936, 18516, [136, 128, 226, 228, 1, 137, 189, 208]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_AttachmentNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_BccAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_BccName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_CcAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_CcName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ConversationID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3700392125, 44830, 17033, [133, 182, 61, 252, 27, 73, 57, 146]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ConversationIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3700392125, 44830, 17033, [133, 182, 61, 252, 27, 73, 57, 146]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_DateReceived: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_DateSent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_Flags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2821562087, 51815, 17170, [150, 94, 34, 107, 206, 168, 80, 35]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_FromAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_FromName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_HasAttachments: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2619330420, 11671, 16826, [180, 174, 203, 46, 54, 97, 166, 228]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_IsFwdOrReply: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2593898632, 20333, 18078, [153, 25, 231, 5, 65, 32, 64, 249]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_MessageClass: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3449738328, 2254, 16783, [167, 14, 249, 18, 199, 187, 156, 92]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_Participants: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(446408197, 36476, 19729, [173, 125, 165, 10, 218, 24, 186, 27]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ProofInProgress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2425942844, 39549, 18600, [141, 229, 46, 18, 39, 166, 78, 145]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_SenderAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(199346407, 6529, 18038, [174, 20, 253, 215, 143, 5, 166, 231]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_SenderName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(228859130, 53796, 18968, [174, 47, 89, 97, 88, 219, 75, 58]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_Store: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ToAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ToDoFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(528837279, 26880, 19130, [149, 5, 45, 95, 27, 77, 102, 203]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ToDoTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3167521340, 36079, 17125, [155, 28, 198, 144, 121, 57, 139, 199]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Message_ToName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3823130700, 46984, 19034, [187, 32, 127, 90, 68, 201, 172, 221]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_MileageInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4260905840, 794, 19165, [158, 145, 13, 119, 95, 28, 102, 5]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_AlbumArtist: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_AlbumArtistSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4059935919, 63372, 18028, [187, 5, 86, 233, 45, 176, 184, 236]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_AlbumID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_AlbumTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_AlbumTitleSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(334200828, 60553, 17222, [177, 157, 204, 198, 241, 120, 66, 35]),
    pid: 101u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Artist: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_ArtistSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3739954613, 1686, 19680, [148, 254, 160, 31, 119, 164, 95, 181]),
    pid: 102u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_BeatsPerMinute: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Composer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_ComposerSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(12329123, 48456, 16517, [135, 44, 168, 141, 119, 245, 9, 126]),
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Conductor: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_ContentGroupDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_DiscNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1795060791, 39885, 18887, [128, 254, 74, 92, 101, 250, 88, 116]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_DisplayArtist: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4245825875, 64147, 20215, [146, 195, 4, 201, 70, 178, 247, 200]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Genre: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_InitialKey: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_IsCompilation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3293173195, 40612, 18441, [130, 232, 175, 157, 89, 222, 214, 209]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Lyrics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Mood: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_PartOfSet: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_Period: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_SynchronizedLyrics: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1797405546, 5678, 19113, [179, 159, 5, 214, 120, 252, 109, 119]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Music_TrackNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1453537070, 52892, 4562, [159, 14, 0, 96, 151, 198, 134, 246]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_NamespaceCLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Note_Color: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1198967546, 48356, 19633, [162, 62, 38, 94, 118, 216, 235, 17]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Note_ColorText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1186261214, 52658, 17421, [136, 92, 22, 88, 235, 101, 185, 20]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Null: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_OfflineAvailability: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2839972022, 32159, 17776, [166, 72, 227, 223, 192, 171, 43, 63]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_OfflineStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1831110799, 18200, 19418, [175, 237, 234, 15, 180, 56, 108, 216]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_OriginalFileName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217021779, 64100, 4561, [162, 3, 0, 0, 248, 31, 237, 238]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_OwnerSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1568061055, 39741, 17595, [182, 174, 37, 218, 79, 99, 138, 103]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ParentalRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ParentalRatingReason: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(278416906, 63986, 17185, [183, 239, 186, 241, 149, 175, 67, 25]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ParentalRatingsOrganization: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2818443328, 4932, 18160, [141, 55, 82, 237, 113, 42, 75, 249]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ParsingBindContext: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3753484365, 13871, 19619, [179, 11, 2, 84, 177, 123, 91, 132]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ParsingName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ParsingPath: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PerceivedType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PercentFull: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995061, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Aperture: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37378u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ApertureDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3785991051, 26245, 18109, [135, 94, 87, 13, 199, 173, 115, 32]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ApertureNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(53996780, 14843, 17793, [160, 189, 76, 76, 197, 30, 153, 20]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Brightness: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(443554806, 18316, 17249, [131, 171, 55, 1, 187, 5, 60, 88]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_BrightnessDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1857972550, 8993, 17418, [144, 240, 192, 67, 239, 211, 36, 118]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_BrightnessNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2658996623, 45844, 17824, [140, 251, 214, 84, 185, 23, 201, 233]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_CameraManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 271u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_CameraModel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 272u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_CameraSerialNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 273u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Contrast: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(712530857, 36131, 19949, [130, 230, 96, 163, 80, 200, 106, 16]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ContrastText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1507715570, 21075, 16618, [154, 139, 71, 158, 150, 198, 36, 154]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_DateTaken: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 36867u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_DigitalZoom: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4166776896, 43301, 19394, [176, 196, 142, 54, 181, 152, 103, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_DigitalZoomDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1952165646, 58817, 19707, [138, 27, 208, 49, 160, 165, 35, 147]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_DigitalZoomNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(382449956, 25856, 18235, [165, 190, 241, 89, 155, 203, 228, 19]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_EXIFVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3546248250, 60206, 18418, [162, 134, 132, 65, 50, 203, 20, 39]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Event: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 18248u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureBias: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37380u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureBiasDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2871025232, 1207, 17948, [161, 140, 47, 35, 56, 54, 230, 39]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureBiasNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1938551428, 7559, 16907, [146, 207, 88, 52, 191, 110, 249, 237]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2524666616, 39258, 18157, [158, 17, 53, 179, 197, 185, 120, 45]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureIndexDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2467377033, 49803, 18735, [138, 157, 75, 226, 6, 44, 238, 138]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureIndexNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3454914352, 35097, 17631, [143, 76, 78, 178, 255, 219, 141, 137]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureProgram: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 34850u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureProgramText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4274426039, 24368, 17990, [174, 71, 76, 170, 251, 168, 132, 163]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 33434u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureTimeDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1441367447, 44310, 17120, [182, 36, 33, 89, 154, 25, 152, 56]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ExposureTimeNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(629032162, 36913, 17187, [172, 56, 133, 197, 82, 135, 27, 46]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 33437u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FNumberDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3911853206, 8763, 17507, [164, 227, 48, 234, 187, 167, 157, 128]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FNumberNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(462910346, 65020, 17967, [157, 147, 25, 87, 224, 139, 233, 12]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Flash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37385u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FlashEnergy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 41483u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FlashEnergyDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3619036272, 25379, 18893, [165, 252, 200, 66, 119, 22, 44, 151]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FlashEnergyNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4239211837, 2136, 16399, [170, 163, 47, 102, 204, 226, 166, 188]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FlashManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2864379593, 57541, 18201, [133, 133, 87, 177, 3, 229, 132, 254]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FlashModel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4270046005, 19738, 17122, [145, 107, 6, 243, 225, 175, 113, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FlashText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1804298486, 8203, 18410, [141, 37, 216, 5, 15, 87, 51, 159]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalLength: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37386u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalLengthDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(811320853, 56481, 17573, [159, 212, 16, 192, 186, 121, 65, 46]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalLengthInFilm: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2699511305, 47181, 20297, [184, 96, 70, 43, 217, 151, 31, 152]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalLengthNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2003528507, 7741, 19212, [154, 14, 143, 186, 242, 168, 73, 42]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalPlaneXResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3485502871, 50935, 17540, [137, 221, 235, 239, 67, 86, 254, 118]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalPlaneXResolutionDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(154399733, 18310, 20294, [168, 232, 214, 77, 211, 127, 165, 33]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalPlaneXResolutionNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3704295599, 46306, 19336, [149, 249, 3, 27, 77, 90, 180, 144]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalPlaneYResolution: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1342170320, 37199, 19140, [141, 111, 201, 198, 29, 225, 105, 177]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalPlaneYResolutionDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(492927398, 43126, 16433, [176, 19, 51, 71, 178, 182, 77, 200]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_FocalPlaneYResolutionNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2732933573, 17472, 19368, [134, 126, 117, 207, 192, 104, 40, 205]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_GainControl: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4197468041, 199, 19840, [144, 74, 30, 77, 204, 114, 101, 170]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_GainControlDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1116098045, 40356, 20343, [189, 237, 74, 173, 123, 37, 103, 53]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_GainControlNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2391723900, 47032, 20152, [166, 63, 14, 231, 21, 201, 111, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_GainControlText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3227662514, 3065, 17017, [167, 35, 37, 133, 103, 21, 203, 157]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ISOSpeed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 34855u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_LensManufacturer: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3873295095, 10693, 20234, [154, 104, 209, 148, 18, 236, 112, 144]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_LensModel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3777459478, 11103, 18537, [137, 177, 46, 88, 91, 211, 139, 122]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_LightSource: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37384u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MakerNote: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4197462867, 46681, 16466, [133, 233, 188, 172, 121, 84, 155, 132]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MakerNoteOffset: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2168406308, 13542, 19735, [171, 62, 107, 31, 60, 34, 71, 161]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MaxAperture: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(150394818, 58354, 17660, [175, 30, 90, 165, 200, 26, 45, 62]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MaxApertureDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3346474196, 24607, 18117, [155, 137, 197, 63, 147, 188, 235, 119]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MaxApertureNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3238519185, 42073, 17605, [154, 230, 185, 82, 173, 75, 144, 109]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MeteringMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37383u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_MeteringModeText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4129881484, 31656, 18010, [166, 91, 197, 170, 121, 38, 58, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Orientation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 274u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_OrientationText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2850691388, 50449, 18826, [160, 107, 88, 226, 119, 109, 204, 40]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_PeopleNames: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3895499630, 2124, 18868, [177, 252, 144, 168, 3, 49, 182, 56]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_PhotometricInterpretation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(873961201, 7673, 19228, [165, 100, 145, 189, 239, 164, 56, 119]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_PhotometricInterpretationText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2182363094, 40619, 18277, [165, 137, 59, 28, 187, 210, 42, 97]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ProgramMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1830911853, 16234, 18469, [180, 112, 95, 3, 202, 47, 190, 155]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ProgramModeText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2145626663, 9800, 17139, [137, 176, 69, 78, 92, 177, 80, 195]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_RelatedSoundFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(831155013, 2175, 19906, [184, 204, 5, 53, 149, 81, 252, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Saturation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1227060005, 43354, 20327, [178, 17, 129, 107, 45, 69, 210, 224]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_SaturationText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1632078856, 46592, 19076, [187, 228, 233, 156, 69, 240, 160, 114]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_Sharpness: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4234770139, 33609, 18800, [174, 151, 179, 197, 49, 106, 8, 240]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_SharpnessText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1374437191, 56656, 16925, [135, 105, 51, 79, 80, 66, 75, 30]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ShutterSpeed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37377u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ShutterSpeedDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3778906485, 33223, 18760, [174, 63, 55, 202, 225, 30, 143, 247]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_ShutterSpeedNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(384450626, 55028, 19402, [131, 73, 124, 120, 211, 15, 179, 51]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_SubjectDistance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 37382u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_SubjectDistanceDenominator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(209980040, 45123, 18029, [151, 102, 212, 178, 109, 163, 250, 119]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_SubjectDistanceNumerator: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2331285020, 62758, 17381, [170, 129, 219, 118, 130, 25, 23, 141]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_TagViewAggregate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3088249181, 49880, 19391, [186, 205, 121, 116, 67, 70, 17, 63]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_TranscodedForSync: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2593045365, 25688, 20098, [186, 203, 53, 192, 9, 91, 3, 187]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_WhiteBalance: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3996990858, 21377, 19706, [177, 59, 170, 246, 107, 95, 78, 201]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Photo_WhiteBalanceText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1664530782, 51111, 17005, [134, 253, 122, 227, 211, 156, 132, 180]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Priority: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2619330420, 11671, 16826, [180, 174, 203, 46, 54, 97, 166, 228]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PriorityText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3649825163, 47211, 16533, [191, 82, 157, 35, 178, 224, 167, 82]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Project: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(967309602, 18300, 18654, [139, 200, 178, 132, 65, 227, 66, 227]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Advanced: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2416590907, 2427, 19349, [138, 226, 7, 31, 218, 238, 177, 24]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Audio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(671405161, 30863, 18602, [133, 112, 113, 185, 193, 135, 225, 56]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Calendar: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2574504629, 49112, 17290, [186, 148, 83, 73, 178, 147, 24, 26]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Camera: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3724598834, 21630, 18817, [173, 75, 84, 47, 46, 144, 7, 216]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Contact: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3751239635, 9482, 16388, [133, 143, 52, 226, 154, 62, 55, 170]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Content: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3503993018, 13962, 16464, [168, 130, 108, 1, 15, 209, 154, 79]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Description: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2305405557, 38005, 19968, [168, 135, 255, 147, 184, 180, 30, 68]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_FileSystem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3819426497, 33020, 19264, [143, 52, 48, 234, 17, 27, 220, 46]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_GPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4084284122, 37091, 19985, [170, 229, 253, 193, 118, 133, 185, 190]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_General: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3425703472, 45458, 19490, [179, 114, 159, 76, 109, 51, 142, 7]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Image: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3815312007, 4008, 18986, [154, 159, 252, 232, 130, 112, 85, 172]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Media: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1636248823, 27486, 19275, [172, 45, 89, 218, 132, 69, 146, 72]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_MediaAdvanced: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2287575684, 56958, 17986, [153, 186, 212, 49, 208, 68, 177, 236]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Message: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2144806301, 5812, 16693, [159, 151, 124, 150, 236, 210, 250, 158]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Music: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1759338644, 29206, 16625, [160, 41, 67, 254, 113, 39, 4, 63]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Origin: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(630772475, 21865, 17255, [149, 223, 92, 211, 161, 119, 225, 165]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_PhotoAdvanced: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(213040986, 40679, 19078, [130, 34, 240, 30, 7, 253, 173, 175]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_RecordedTV: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3887280696, 25988, 16752, [165, 192, 172, 37, 239, 217, 218, 86]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropGroup_Video: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3200125216, 30321, 19540, [163, 235, 73, 253, 223, 193, 145, 238]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_ConflictPrompt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_ContentViewModeForBrowse: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_ContentViewModeForSearch: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_ExtendedTileInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_FileOperationPrompt: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_FullDetails: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_InfoTip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_NonPersonal: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1238436127, 2094, 18751, [178, 63, 210, 48, 138, 169, 102, 140]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_PreviewDetails: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_PreviewTitle: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_QuickTip: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_TileInfo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3381938721, 41990, 18686, [130, 37, 174, 199, 226, 76, 33, 27]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_PropList_XPDetailsPanel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4062663808, 63362, 17041, [189, 148, 241, 54, 147, 81, 58, 236]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ProviderItemID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4062026049, 33264, 18202, [173, 238, 78, 116, 180, 146, 23, 237]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Rating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RatingText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2417589415, 64911, 20108, [157, 163, 181, 126, 30, 96, 146, 149]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_ChannelNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_Credits: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_DateContentExpires: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_EpisodeName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_IsATSCContent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_IsClosedCaptioningAvailable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_IsDTVContent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_IsHDContent: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_IsRepeatBroadcast: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_IsSAP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_NetworkAffiliation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(743688211, 64355, 20002, [161, 171, 11, 51, 28, 161, 226, 115]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_OriginalBroadcastDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1183121047, 34661, 18498, [156, 19, 240, 6, 68, 123, 23, 140]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_ProgramDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_RecordingTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2772926305, 31362, 20170, [157, 222, 152, 182, 155, 36, 121, 179]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_StationCallSign: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1836355042, 36152, 19651, [172, 96, 240, 9, 176, 87, 197, 87]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RecordedTV_StationName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(458504679, 60321, 19192, [189, 215, 122, 241, 212, 84, 148, 147]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_RemoteConflictingFile: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 115u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SFGAOFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_AutoSummary: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1443641024, 20538, 4559, [186, 161, 0, 0, 76, 117, 42, 154]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_ContainerHash: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3169772163, 13791, 19795, [130, 106, 243, 106, 62, 239, 198, 190]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_Contents: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_EntryID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1231625360, 32279, 4122, [169, 28, 8, 0, 43, 46, 205, 169]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_ExtendedProperties: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2063840582, 64079, 19026, [162, 254, 3, 213, 49, 30, 88, 101]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_GatherTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(191095632, 40140, 4560, [188, 219, 0, 128, 95, 204, 206, 4]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_HitCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1231625360, 32279, 4122, [169, 28, 8, 0, 43, 46, 205, 169]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_IsClosedDirectory: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(191095619, 40140, 4560, [188, 219, 0, 128, 95, 204, 206, 4]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_IsFullyContained: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(191095619, 40140, 4560, [188, 219, 0, 128, 95, 204, 206, 4]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_QueryFocusedSummary: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1443641024, 20538, 4559, [186, 161, 0, 0, 76, 117, 42, 154]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_QueryFocusedSummaryWithFallback: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1443641024, 20538, 4559, [186, 161, 0, 0, 76, 117, 42, 154]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_QueryPropertyHits: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1231625360, 32279, 4122, [169, 28, 8, 0, 43, 46, 205, 169]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_Rank: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1231625360, 32279, 4122, [169, 28, 8, 0, 43, 46, 205, 169]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_Store: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2691273395, 36015, 20183, [165, 71, 178, 89, 227, 42, 201, 252]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_UrlToIndex: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(191095619, 40140, 4560, [188, 219, 0, 128, 95, 204, 206, 4]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Search_UrlToIndexWithModificationTime: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(191095619, 40140, 4560, [188, 219, 0, 128, 95, 204, 206, 4]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Security_AllowedEnterpriseDataProtectionIdentities: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(953430912, 54296, 18480, [132, 213, 70, 147, 90, 129, 197, 198]),
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Security_EncryptionOwners: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1599799146, 14309, 18304, [151, 234, 128, 199, 86, 92, 245, 53]),
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Security_EncryptionOwnersDisplay: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3730971535, 57637, 17315, [163, 45, 86, 101, 68, 109, 99, 42]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sensitivity: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4174640812, 18548, 17099, [190, 89, 171, 69, 75, 48, 113, 106]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SensitivityText: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3502764116, 16242, 18213, [133, 39, 18, 154, 87, 124, 178, 105]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ShareUserRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SharedWith: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4018687067, 11262, 16827, [170, 229, 118, 238, 223, 79, 153, 2]),
    pid: 200u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SharingStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4018687067, 11262, 16827, [170, 229, 118, 238, 223, 79, 153, 2]),
    pid: 300u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Shell_OmitFromView: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3728024972, 50837, 19644, [185, 130, 56, 176, 173, 36, 206, 208]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Shell_SFGAOFlagsStrings: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3600031873, 54587, 17469, [173, 71, 94, 5, 157, 156, 210, 122]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SimpleRating: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2694776910, 44353, 18591, [128, 118, 170, 91, 227, 8, 43, 202]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Size: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3072717104, 18415, 4122, [165, 241, 2, 96, 140, 158, 235, 172]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SoftwareUsed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(347610529, 309, 19761, [150, 217, 108, 191, 201, 103, 26, 153]),
    pid: 305u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Software_DateLastUsed: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2216578960, 65369, 19734, [137, 71, 232, 27, 191, 250, 179, 109]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Software_ProductName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217021779, 64100, 4561, [162, 3, 0, 0, 248, 31, 237, 238]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SourceItem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1720508325, 31259, 17187, [174, 75, 229, 39, 57, 58, 29, 129]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SourcePackageFamilyName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4289633719, 7309, 17407, [129, 140, 132, 64, 58, 163, 115, 45]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StartDate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1224568520, 35346, 19679, [160, 62, 78, 197, 165, 17, 237, 222]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(136353, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StatusBarSelectedItemCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651962492, 28221, 19411, [178, 176, 106, 38, 186, 46, 52, 109]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StatusBarViewItemCount: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651962492, 28221, 19411, [178, 176, 106, 38, 186, 46, 52, 109]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderCallerVersionInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderError: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 109u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileChecksum: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileFlags: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileHasConflict: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileRemoteUri: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 112u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderFileVersionWaterline: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3002710486, 65220, 19925, [148, 215, 137, 87, 72, 140, 128, 123]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 108u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderShareStatuses: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 111u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderSharingStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 117u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_StorageProviderStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 110u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Storage_Portable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1293860584, 2051, 18292, [152, 66, 183, 125, 181, 2, 101, 233]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Storage_RemovableMedia: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1293860584, 2051, 18292, [152, 66, 183, 125, 181, 2, 101, 233]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Storage_SystemCritical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1293860584, 2051, 18292, [152, 66, 183, 125, 181, 2, 101, 233]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Subject: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Supplemental_Album: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(208908609, 14806, 18003, [166, 131, 202, 178, 145, 234, 249, 91]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Supplemental_AlbumID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(208908609, 14806, 18003, [166, 131, 202, 178, 145, 234, 249, 91]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Supplemental_Location: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(208908609, 14806, 18003, [166, 131, 202, 178, 145, 234, 249, 91]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Supplemental_Person: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(208908609, 14806, 18003, [166, 131, 202, 178, 145, 234, 249, 91]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Supplemental_ResourceId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(208908609, 14806, 18003, [166, 131, 202, 178, 145, 234, 249, 91]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Supplemental_Tag: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(208908609, 14806, 18003, [166, 131, 202, 178, 145, 234, 249, 91]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_SyncTransferStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 103u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_Comments: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_ConflictDescription: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3461398873, 12216, 16893, [190, 104, 211, 224, 66, 226, 116, 188]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_ConflictFirstLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3461398873, 12216, 16893, [190, 104, 211, 224, 66, 226, 116, 188]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_ConflictSecondLocation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3461398873, 12216, 16893, [190, 104, 211, 224, 66, 226, 116, 188]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_HandlerCollectionID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_HandlerID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_HandlerName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3461398873, 12216, 16893, [190, 104, 211, 224, 66, 226, 116, 188]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_HandlerType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_HandlerTypeLabel: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_ItemID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_ItemName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3461398873, 12216, 16893, [190, 104, 211, 224, 66, 226, 116, 188]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_ProgressPercentage: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_State: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Sync_Status: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2077578046, 44821, 17627, [184, 200, 189, 102, 36, 225, 208, 50]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Task_BillingInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3548205766, 9756, 17155, [130, 179, 8, 185, 38, 172, 111, 18]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Task_CompletionStatus: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(139299338, 59093, 16606, [191, 31, 200, 130, 14, 124, 135, 124]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Task_Owner: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(147311711, 24818, 17556, [173, 117, 85, 227, 224, 181, 173, 208]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Thumbnail: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ThumbnailCacheId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1147999921, 36269, 18544, [167, 72, 64, 46, 164, 61, 120, 140]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ThumbnailStream: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Title: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4070540768, 20473, 4200, [171, 145, 8, 0, 43, 39, 179, 217]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_TitleSortOverride: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4042758221, 8750, 19154, [130, 171, 29, 216, 234, 64, 229, 126]),
    pid: 300u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_TotalFileSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(677604006, 38205, 4562, [181, 214, 0, 192, 79, 217, 24, 208]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Trademarks: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(217021779, 64100, 4561, [162, 3, 0, 0, 248, 31, 237, 238]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_TransferOrder: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 106u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_TransferPosition: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_TransferSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4243583315, 59449, 19699, [169, 231, 234, 34, 131, 32, 148, 184]),
    pid: 105u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_Compression: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_Director: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179218, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_EncodingBitrate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_FourCC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 44u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_FrameHeight: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_FrameRate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_FrameWidth: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_HorizontalAspectRatio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 42u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_IsSpherical: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 100u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_IsStereo: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 98u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_Orientation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 99u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_SampleSize: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_StreamName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_StreamNumber: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_TotalBitrate: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 43u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_TranscodedForSync: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 46u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Video_VerticalAspectRatio: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1682179217, 19595, 4561, [139, 112, 8, 0, 54, 177, 26, 3]),
    pid: 45u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_VolumeId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1147999921, 36269, 18544, [167, 72, 64, 46, 164, 61, 120, 140]),
    pid: 104u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Volume_FileSystem: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995061, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Volume_IsMappedDrive: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(345770857, 11309, 18684, [128, 143, 211, 24, 215, 140, 70, 54]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_Volume_IsRoot: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2601995061, 16639, 4562, [162, 126, 0, 192, 79, 195, 8, 113]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const PKEY_ZoneIdentifier: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1345126059, 18411, 17820, [185, 96, 230, 216, 114, 143, 119, 1]),
    pid: 100u32,
};
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_NOMEDIA: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_PAUSED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_PLAYING: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_RECORDING: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_RECORDINGPAUSED: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_STOPPED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_TRANSITIONING: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const PLAYBACKSTATE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FIVE_STARS_MAX: u32 = 99u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FIVE_STARS_MIN: u32 = 88u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FIVE_STARS_SET: u32 = 99u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FOUR_STARS_MAX: u32 = 87u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FOUR_STARS_MIN: u32 = 63u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_FOUR_STARS_SET: u32 = 75u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_ONE_STAR_MAX: u32 = 12u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_ONE_STAR_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_ONE_STAR_SET: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_THREE_STARS_MAX: u32 = 62u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_THREE_STARS_MIN: u32 = 38u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_THREE_STARS_SET: u32 = 50u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_TWO_STARS_MAX: u32 = 37u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_TWO_STARS_MIN: u32 = 13u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const RATING_TWO_STARS_SET: u32 = 25u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SHARINGSTATUS_NOTSHARED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SHARINGSTATUS_PRIVATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SHARINGSTATUS_SHARED: u32 = 1u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub struct SILO_INFO {
    pub ulSTID: u32,
    pub SpecificationMajor: u8,
    pub SpecificationMinor: u8,
    pub ImplementationMajor: u8,
    pub ImplementationMinor: u8,
    pub r#type: u8,
    pub capabilities: u8,
}
impl SILO_INFO {}
impl ::core::default::Default for SILO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SILO_INFO {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SILO_INFO")
            .field("ulSTID", &self.ulSTID)
            .field("SpecificationMajor", &self.SpecificationMajor)
            .field("SpecificationMinor", &self.SpecificationMinor)
            .field("ImplementationMajor", &self.ImplementationMajor)
            .field("ImplementationMinor", &self.ImplementationMinor)
            .field("r#type", &self.r#type)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}
impl ::core::cmp::PartialEq for SILO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ulSTID == other.ulSTID && self.SpecificationMajor == other.SpecificationMajor && self.SpecificationMinor == other.SpecificationMinor && self.ImplementationMajor == other.ImplementationMajor && self.ImplementationMinor == other.ImplementationMinor && self.r#type == other.r#type && self.capabilities == other.capabilities
    }
}
impl ::core::cmp::Eq for SILO_INFO {}
unsafe impl ::windows::runtime::Abi for SILO_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_NOTSHARED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PRIVATE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_COOWNED: u32 = 7u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_PUBLIC_OWNED: u32 = 6u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_COOWNED: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const STORAGE_PROVIDER_SHARINGSTATUS_SHARED_OWNED: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_COMPUTERS: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_DEVICES: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_FOLDERS: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_OTHER: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_PROGRAMS: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_HANDLERTYPE_WEBSERVICES: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_ERROR: u32 = 3u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_IDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_NOTSETUP: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_PENDING: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_SYNCING: u32 = 5u32;
#[doc = "*Required features: `Win32_Storage_EnhancedStorage`*"]
pub const SYNC_STATE_SYNCNOTRUN: u32 = 1u32;
pub const WPD_CATEGORY_ENHANCED_STORAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2435088742, 47154, 19156, [186, 164, 124, 160, 182, 178, 121, 140]);
