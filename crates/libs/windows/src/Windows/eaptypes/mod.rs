pub const CERTIFICATE_HASH_LENGTH: u32 = 20;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAPHOST_IDENTITY_UI_PARAMS {
    pub eapMethodType: EAP_METHOD_TYPE,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: windows_core::PWSTR,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
impl Default for EAPHOST_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAPHOST_INTERACTIVE_UI_PARAMS {
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
impl Default for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAPHOST_METHOD_API_VERSION: u32 = 1;
pub const EAPHOST_PEER_API_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_ATTRIBUTE {
    pub eaType: EAP_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub pValue: *mut u8,
}
impl Default for EAP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_ATTRIBUTES {
    pub dwNumberOfAttributes: u32,
    pub pAttribs: *mut EAP_ATTRIBUTE,
}
impl Default for EAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_ATTRIBUTE_TYPE = i32;
pub const EAP_CERTIFICATE_CREDENTIAL: EapCredentialType = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_CONFIG_INPUT_FIELD_ARRAY {
    pub dwVersion: u32,
    pub dwNumberOfFields: u32,
    pub pFields: *mut EAP_CONFIG_INPUT_FIELD_DATA,
}
impl Default for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_CONFIG_INPUT_FIELD_DATA {
    pub dwSize: u32,
    pub Type: EAP_CONFIG_INPUT_FIELD_TYPE,
    pub dwFlagProps: u32,
    pub pwszLabel: windows_core::PWSTR,
    pub pwszData: windows_core::PWSTR,
    pub dwMinDataLength: u32,
    pub dwMaxDataLength: u32,
}
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: u32 = 0;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2;
pub type EAP_CONFIG_INPUT_FIELD_TYPE = i32;
pub const EAP_CREDENTIAL_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_CRED_EXPIRY_REQ {
    pub curCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub newCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
}
pub type EAP_CRED_EXPIRY_RESP = EAP_CRED_EXPIRY_REQ;
pub type EAP_CRED_LOGON_REQ = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type EAP_CRED_LOGON_RESP = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type EAP_CRED_REQ = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type EAP_CRED_RESP = EAP_CONFIG_INPUT_FIELD_ARRAY;
pub const EAP_EMPTY_CREDENTIAL: EapCredentialType = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: windows_core::GUID,
    pub repairGuid: windows_core::GUID,
    pub helpLinkGuid: windows_core::GUID,
    pub pRootCauseString: windows_core::PWSTR,
    pub pRepairString: windows_core::PWSTR,
}
pub const EAP_FLAG_CONFG_READONLY: u32 = 524288;
pub const EAP_FLAG_DISABLE_SESSION_RESUMPTION: u32 = 67108864;
pub const EAP_FLAG_FULL_AUTH: u32 = 4096;
pub const EAP_FLAG_GUEST_ACCESS: u32 = 64;
pub const EAP_FLAG_LOGON: u32 = 4;
pub const EAP_FLAG_MACHINE_AUTH: u32 = 32;
pub const EAP_FLAG_NON_INTERACTIVE: u32 = 2;
pub const EAP_FLAG_ONLY_EAP_TLS: u32 = 16777216;
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: u32 = 8192;
pub const EAP_FLAG_PREVIEW: u32 = 8;
pub const EAP_FLAG_PRE_LOGON: u32 = 131072;
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512;
pub const EAP_FLAG_Reserved1: u32 = 1;
pub const EAP_FLAG_Reserved2: u32 = 16;
pub const EAP_FLAG_Reserved3: u32 = 128;
pub const EAP_FLAG_Reserved4: u32 = 256;
pub const EAP_FLAG_Reserved5: u32 = 1024;
pub const EAP_FLAG_Reserved6: u32 = 2048;
pub const EAP_FLAG_Reserved7: u32 = 16384;
pub const EAP_FLAG_Reserved8: u32 = 1048576;
pub const EAP_FLAG_Reserved9: u32 = 4194304;
pub const EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432;
pub const EAP_FLAG_SUPRESS_UI: u32 = 65536;
pub const EAP_FLAG_USER_AUTH: u32 = 262144;
pub const EAP_FLAG_VPN: u32 = 8388608;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EAP_INTERACTIVE_UI_DATA {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwDataType: EAP_INTERACTIVE_UI_DATA_TYPE,
    pub cbUiData: u32,
    pub pbUiData: EAP_UI_DATA_FORMAT,
}
impl Default for EAP_INTERACTIVE_UI_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_INTERACTIVE_UI_DATA_TYPE = i32;
pub const EAP_INTERACTIVE_UI_DATA_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: windows_core::PWSTR,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfo: *mut Self,
}
impl Default for EAP_METHOD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO_ARRAY {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO,
}
impl Default for EAP_METHOD_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO_ARRAY_EX {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO_EX,
}
impl Default for EAP_METHOD_INFO_ARRAY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO_EX {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: windows_core::PWSTR,
    pub pwszFriendlyName: windows_core::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfoArray: *mut EAP_METHOD_INFO_ARRAY_EX,
}
impl Default for EAP_METHOD_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EAP_METHOD_PROPERTY {
    pub eapMethodPropertyType: EAP_METHOD_PROPERTY_TYPE,
    pub eapMethodPropertyValueType: EAP_METHOD_PROPERTY_VALUE_TYPE,
    pub eapMethodPropertyValue: EAP_METHOD_PROPERTY_VALUE,
}
impl Default for EAP_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_PROPERTY_ARRAY {
    pub dwNumberOfProperties: u32,
    pub pMethodProperty: *mut EAP_METHOD_PROPERTY,
}
impl Default for EAP_METHOD_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_METHOD_PROPERTY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union EAP_METHOD_PROPERTY_VALUE {
    pub empvBool: EAP_METHOD_PROPERTY_VALUE_BOOL,
    pub empvDword: EAP_METHOD_PROPERTY_VALUE_DWORD,
    pub empvString: EAP_METHOD_PROPERTY_VALUE_STRING,
}
impl Default for EAP_METHOD_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_METHOD_PROPERTY_VALUE_BOOL {
    pub length: u32,
    pub value: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_METHOD_PROPERTY_VALUE_DWORD {
    pub length: u32,
    pub value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_PROPERTY_VALUE_STRING {
    pub length: u32,
    pub value: *mut u8,
}
impl Default for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_METHOD_PROPERTY_VALUE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_METHOD_TYPE {
    pub eapType: EAP_TYPE,
    pub dwAuthorId: u32,
}
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: u32 = 32768;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EAP_SESSIONID(pub u32);
pub const EAP_SIM_CREDENTIAL: EapCredentialType = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_TYPE {
    pub r#type: u8,
    pub dwVendorId: u32,
    pub dwVendorType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EAP_UI_DATA_FORMAT {
    pub credData: *mut EAP_CRED_REQ,
    pub credExpiryData: *mut EAP_CRED_EXPIRY_REQ,
    pub credLogonData: *mut EAP_CRED_LOGON_REQ,
}
impl Default for EAP_UI_DATA_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: u32 = 0;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2;
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: u32 = 4;
pub const EAP_USERNAME_PASSWORD_CREDENTIAL: EapCredentialType = 1;
pub const EAP_VALUENAME_PROPERTIES: windows_core::PCWSTR = windows_core::w!("Properties");
pub const EAP_WINLOGON_CREDENTIAL: EapCredentialType = 2;
pub type EapAttribute = EAP_ATTRIBUTE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EapAttributeType(pub EAP_ATTRIBUTE_TYPE);
pub type EapAttributes = EAP_ATTRIBUTES;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapCertificateCredential {
    pub certHash: [u8; 20],
    pub password: windows_core::PWSTR,
}
impl Default for EapCertificateCredential {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EapConfigInputEdit: EAP_CONFIG_INPUT_FIELD_TYPE = 6;
pub const EapConfigInputNetworkPassword: EAP_CONFIG_INPUT_FIELD_TYPE = 3;
pub const EapConfigInputNetworkUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 2;
pub const EapConfigInputPSK: EAP_CONFIG_INPUT_FIELD_TYPE = 5;
pub const EapConfigInputPassword: EAP_CONFIG_INPUT_FIELD_TYPE = 1;
pub const EapConfigInputPin: EAP_CONFIG_INPUT_FIELD_TYPE = 4;
pub const EapConfigInputUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 0;
pub const EapConfigSmartCardError: EAP_CONFIG_INPUT_FIELD_TYPE = 8;
pub const EapConfigSmartCardUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 7;
pub const EapCredExpiryReq: EAP_INTERACTIVE_UI_DATA_TYPE = 2;
pub const EapCredExpiryResp: EAP_INTERACTIVE_UI_DATA_TYPE = 3;
pub const EapCredLogonReq: EAP_INTERACTIVE_UI_DATA_TYPE = 4;
pub const EapCredLogonResp: EAP_INTERACTIVE_UI_DATA_TYPE = 5;
pub const EapCredReq: EAP_INTERACTIVE_UI_DATA_TYPE = 0;
pub const EapCredResp: EAP_INTERACTIVE_UI_DATA_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EapCredential {
    pub credType: EapCredentialType,
    pub credData: EapCredentialTypeData,
}
impl Default for EapCredential {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EapCredentialType = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union EapCredentialTypeData {
    pub username_password: EapUsernamePasswordCredential,
    pub certificate: EapCertificateCredential,
    pub sim: EapSimCredential,
}
impl Default for EapCredentialTypeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EapSimCredential {
    pub iccID: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EapUsernamePasswordCredential {
    pub username: windows_core::PWSTR,
    pub password: windows_core::PWSTR,
}
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: u32 = 256;
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: u32 = 1024;
pub type PEAP_CONFIG_INPUT_FIELD_ARRAY = *mut EAP_CONFIG_INPUT_FIELD_ARRAY;
pub type PEAP_CONFIG_INPUT_FIELD_DATA = *mut EAP_CONFIG_INPUT_FIELD_DATA;
pub type PEAP_CONFIG_INPUT_FIELD_TYPE = *mut EAP_CONFIG_INPUT_FIELD_TYPE;
pub const eapPropCertifiedMethod: u32 = 4194304;
pub const eapPropChannelBinding: u32 = 65536;
pub const eapPropCipherSuiteNegotiation: u32 = 1;
pub const eapPropConfidentiality: u32 = 16;
pub const eapPropCryptoBinding: u32 = 8192;
pub const eapPropDictionaryAttackResistance: u32 = 2048;
pub const eapPropFastReconnect: u32 = 4096;
pub const eapPropFragmentation: u32 = 32768;
pub const eapPropHiddenMethod: u32 = 8388608;
pub const eapPropIdentityPrivacy: u32 = 67108864;
pub const eapPropIntegrity: u32 = 4;
pub const eapPropKeyDerivation: u32 = 32;
pub const eapPropKeyStrength1024: u32 = 1024;
pub const eapPropKeyStrength128: u32 = 128;
pub const eapPropKeyStrength256: u32 = 256;
pub const eapPropKeyStrength512: u32 = 512;
pub const eapPropKeyStrength64: u32 = 64;
pub const eapPropMachineAuth: u32 = 16777216;
pub const eapPropMethodChaining: u32 = 134217728;
pub const eapPropMppeEncryption: u32 = 524288;
pub const eapPropMutualAuth: u32 = 2;
pub const eapPropNap: u32 = 131072;
pub const eapPropReplayProtection: u32 = 8;
pub const eapPropReserved: u32 = 2147483648;
pub const eapPropSessionIndependence: u32 = 16384;
pub const eapPropSharedStateEquivalence: u32 = 268435456;
pub const eapPropStandalone: u32 = 262144;
pub const eapPropSupportsConfig: u32 = 2097152;
pub const eapPropTunnelMethod: u32 = 1048576;
pub const eapPropUserAuth: u32 = 33554432;
pub const eatARAPChallengeResponse: EAP_ATTRIBUTE_TYPE = 84;
pub const eatARAPFeatures: EAP_ATTRIBUTE_TYPE = 71;
pub const eatARAPGuestLogon: EAP_ATTRIBUTE_TYPE = 8096;
pub const eatARAPPassword: EAP_ATTRIBUTE_TYPE = 70;
pub const eatARAPSecurity: EAP_ATTRIBUTE_TYPE = 73;
pub const eatARAPSecurityData: EAP_ATTRIBUTE_TYPE = 74;
pub const eatARAPZoneAccess: EAP_ATTRIBUTE_TYPE = 72;
pub const eatAcctAuthentic: EAP_ATTRIBUTE_TYPE = 45;
pub const eatAcctDelayTime: EAP_ATTRIBUTE_TYPE = 41;
pub const eatAcctEventTimeStamp: EAP_ATTRIBUTE_TYPE = 55;
pub const eatAcctInputOctets: EAP_ATTRIBUTE_TYPE = 42;
pub const eatAcctInputPackets: EAP_ATTRIBUTE_TYPE = 47;
pub const eatAcctInterimInterval: EAP_ATTRIBUTE_TYPE = 85;
pub const eatAcctLinkCount: EAP_ATTRIBUTE_TYPE = 51;
pub const eatAcctMultiSessionId: EAP_ATTRIBUTE_TYPE = 50;
pub const eatAcctOutputOctets: EAP_ATTRIBUTE_TYPE = 43;
pub const eatAcctOutputPackets: EAP_ATTRIBUTE_TYPE = 48;
pub const eatAcctSessionId: EAP_ATTRIBUTE_TYPE = 44;
pub const eatAcctSessionTime: EAP_ATTRIBUTE_TYPE = 46;
pub const eatAcctStatusType: EAP_ATTRIBUTE_TYPE = 40;
pub const eatAcctTerminateCause: EAP_ATTRIBUTE_TYPE = 49;
pub const eatCallbackId: EAP_ATTRIBUTE_TYPE = 20;
pub const eatCallbackNumber: EAP_ATTRIBUTE_TYPE = 19;
pub const eatCalledStationId: EAP_ATTRIBUTE_TYPE = 30;
pub const eatCallingStationId: EAP_ATTRIBUTE_TYPE = 31;
pub const eatCertificateOID: EAP_ATTRIBUTE_TYPE = 8097;
pub const eatCertificateThumbprint: EAP_ATTRIBUTE_TYPE = 8250;
pub const eatClass: EAP_ATTRIBUTE_TYPE = 25;
pub const eatClearTextPassword: EAP_ATTRIBUTE_TYPE = 8107;
pub const eatConfigurationToken: EAP_ATTRIBUTE_TYPE = 78;
pub const eatConnectInfo: EAP_ATTRIBUTE_TYPE = 77;
pub const eatCredentialsChanged: EAP_ATTRIBUTE_TYPE = 8103;
pub const eatEAPConfiguration: EAP_ATTRIBUTE_TYPE = 8098;
pub const eatEAPMessage: EAP_ATTRIBUTE_TYPE = 79;
pub const eatEAPTLV: EAP_ATTRIBUTE_TYPE = 8102;
pub const eatEMSK: EAP_ATTRIBUTE_TYPE = 9003;
pub const eatFastRoamedSession: EAP_ATTRIBUTE_TYPE = 8100;
pub const eatFilterId: EAP_ATTRIBUTE_TYPE = 11;
pub const eatFramedAppleTalkLink: EAP_ATTRIBUTE_TYPE = 37;
pub const eatFramedAppleTalkNetwork: EAP_ATTRIBUTE_TYPE = 38;
pub const eatFramedAppleTalkZone: EAP_ATTRIBUTE_TYPE = 39;
pub const eatFramedCompression: EAP_ATTRIBUTE_TYPE = 13;
pub const eatFramedIPAddress: EAP_ATTRIBUTE_TYPE = 8;
pub const eatFramedIPNetmask: EAP_ATTRIBUTE_TYPE = 9;
pub const eatFramedIPXNetwork: EAP_ATTRIBUTE_TYPE = 23;
pub const eatFramedIPv6Pool: EAP_ATTRIBUTE_TYPE = 100;
pub const eatFramedIPv6Prefix: EAP_ATTRIBUTE_TYPE = 97;
pub const eatFramedIPv6Route: EAP_ATTRIBUTE_TYPE = 99;
pub const eatFramedInterfaceId: EAP_ATTRIBUTE_TYPE = 96;
pub const eatFramedMTU: EAP_ATTRIBUTE_TYPE = 12;
pub const eatFramedProtocol: EAP_ATTRIBUTE_TYPE = 7;
pub const eatFramedRoute: EAP_ATTRIBUTE_TYPE = 22;
pub const eatFramedRouting: EAP_ATTRIBUTE_TYPE = 10;
pub const eatIdleTimeout: EAP_ATTRIBUTE_TYPE = 28;
pub const eatInnerEapMethodType: EAP_ATTRIBUTE_TYPE = 8104;
pub const eatLoginIPHost: EAP_ATTRIBUTE_TYPE = 14;
pub const eatLoginIPv6Host: EAP_ATTRIBUTE_TYPE = 98;
pub const eatLoginLATGroup: EAP_ATTRIBUTE_TYPE = 36;
pub const eatLoginLATNode: EAP_ATTRIBUTE_TYPE = 35;
pub const eatLoginLATPort: EAP_ATTRIBUTE_TYPE = 63;
pub const eatLoginLATService: EAP_ATTRIBUTE_TYPE = 34;
pub const eatLoginService: EAP_ATTRIBUTE_TYPE = 15;
pub const eatLoginTCPPort: EAP_ATTRIBUTE_TYPE = 16;
pub const eatMD5CHAPChallenge: EAP_ATTRIBUTE_TYPE = 60;
pub const eatMD5CHAPPassword: EAP_ATTRIBUTE_TYPE = 3;
pub const eatMethodId: EAP_ATTRIBUTE_TYPE = 9002;
pub const eatMinimum: EAP_ATTRIBUTE_TYPE = 0;
pub const eatNASIPAddress: EAP_ATTRIBUTE_TYPE = 4;
pub const eatNASIPv6Address: EAP_ATTRIBUTE_TYPE = 95;
pub const eatNASIdentifier: EAP_ATTRIBUTE_TYPE = 32;
pub const eatNASPort: EAP_ATTRIBUTE_TYPE = 5;
pub const eatNASPortType: EAP_ATTRIBUTE_TYPE = 61;
pub const eatPEAPEmbeddedEAPTypeId: EAP_ATTRIBUTE_TYPE = 8099;
pub const eatPEAPFastRoamedSession: EAP_ATTRIBUTE_TYPE = 8100;
pub const eatPasswordRetry: EAP_ATTRIBUTE_TYPE = 75;
pub const eatPeerId: EAP_ATTRIBUTE_TYPE = 9000;
pub const eatPortLimit: EAP_ATTRIBUTE_TYPE = 62;
pub const eatPrompt: EAP_ATTRIBUTE_TYPE = 76;
pub const eatProxyState: EAP_ATTRIBUTE_TYPE = 33;
pub const eatQuarantineSoH: EAP_ATTRIBUTE_TYPE = 8150;
pub const eatReplyMessage: EAP_ATTRIBUTE_TYPE = 18;
pub const eatReserved: EAP_ATTRIBUTE_TYPE = -1;
pub const eatServerId: EAP_ATTRIBUTE_TYPE = 9001;
pub const eatServiceType: EAP_ATTRIBUTE_TYPE = 6;
pub const eatSessionId: EAP_ATTRIBUTE_TYPE = 9004;
pub const eatSessionTimeout: EAP_ATTRIBUTE_TYPE = 27;
pub const eatSignature: EAP_ATTRIBUTE_TYPE = 80;
pub const eatState: EAP_ATTRIBUTE_TYPE = 24;
pub const eatTerminationAction: EAP_ATTRIBUTE_TYPE = 29;
pub const eatTunnelClientEndpoint: EAP_ATTRIBUTE_TYPE = 66;
pub const eatTunnelMediumType: EAP_ATTRIBUTE_TYPE = 65;
pub const eatTunnelServerEndpoint: EAP_ATTRIBUTE_TYPE = 67;
pub const eatTunnelType: EAP_ATTRIBUTE_TYPE = 64;
pub const eatUnassigned17: EAP_ATTRIBUTE_TYPE = 17;
pub const eatUnassigned21: EAP_ATTRIBUTE_TYPE = 21;
pub const eatUserName: EAP_ATTRIBUTE_TYPE = 1;
pub const eatUserPassword: EAP_ATTRIBUTE_TYPE = 2;
pub const eatVendorSpecific: EAP_ATTRIBUTE_TYPE = 26;
pub const emptLegacyMethodPropertyFlag: EAP_METHOD_PROPERTY_TYPE = 31;
pub const emptPropCertifiedMethod: EAP_METHOD_PROPERTY_TYPE = 22;
pub const emptPropChannelBinding: EAP_METHOD_PROPERTY_TYPE = 16;
pub const emptPropCipherSuiteNegotiation: EAP_METHOD_PROPERTY_TYPE = 0;
pub const emptPropConfidentiality: EAP_METHOD_PROPERTY_TYPE = 4;
pub const emptPropCryptoBinding: EAP_METHOD_PROPERTY_TYPE = 13;
pub const emptPropDictionaryAttackResistance: EAP_METHOD_PROPERTY_TYPE = 11;
pub const emptPropFastReconnect: EAP_METHOD_PROPERTY_TYPE = 12;
pub const emptPropFragmentation: EAP_METHOD_PROPERTY_TYPE = 15;
pub const emptPropHiddenMethod: EAP_METHOD_PROPERTY_TYPE = 23;
pub const emptPropIdentityPrivacy: EAP_METHOD_PROPERTY_TYPE = 26;
pub const emptPropIntegrity: EAP_METHOD_PROPERTY_TYPE = 2;
pub const emptPropKeyDerivation: EAP_METHOD_PROPERTY_TYPE = 5;
pub const emptPropKeyStrength1024: EAP_METHOD_PROPERTY_TYPE = 10;
pub const emptPropKeyStrength128: EAP_METHOD_PROPERTY_TYPE = 7;
pub const emptPropKeyStrength256: EAP_METHOD_PROPERTY_TYPE = 8;
pub const emptPropKeyStrength512: EAP_METHOD_PROPERTY_TYPE = 9;
pub const emptPropKeyStrength64: EAP_METHOD_PROPERTY_TYPE = 6;
pub const emptPropMachineAuth: EAP_METHOD_PROPERTY_TYPE = 24;
pub const emptPropMethodChaining: EAP_METHOD_PROPERTY_TYPE = 27;
pub const emptPropMppeEncryption: EAP_METHOD_PROPERTY_TYPE = 19;
pub const emptPropMutualAuth: EAP_METHOD_PROPERTY_TYPE = 1;
pub const emptPropNap: EAP_METHOD_PROPERTY_TYPE = 17;
pub const emptPropReplayProtection: EAP_METHOD_PROPERTY_TYPE = 3;
pub const emptPropSessionIndependence: EAP_METHOD_PROPERTY_TYPE = 14;
pub const emptPropSharedStateEquivalence: EAP_METHOD_PROPERTY_TYPE = 28;
pub const emptPropStandalone: EAP_METHOD_PROPERTY_TYPE = 18;
pub const emptPropSupportsConfig: EAP_METHOD_PROPERTY_TYPE = 21;
pub const emptPropTunnelMethod: EAP_METHOD_PROPERTY_TYPE = 20;
pub const emptPropUserAuth: EAP_METHOD_PROPERTY_TYPE = 25;
pub const emptPropVendorSpecific: EAP_METHOD_PROPERTY_TYPE = 255;
pub const empvtBool: EAP_METHOD_PROPERTY_VALUE_TYPE = 0;
pub const empvtDword: EAP_METHOD_PROPERTY_VALUE_TYPE = 1;
pub const empvtString: EAP_METHOD_PROPERTY_VALUE_TYPE = 2;
