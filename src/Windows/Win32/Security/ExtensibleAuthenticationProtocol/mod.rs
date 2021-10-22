#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const CERTIFICATE_HASH_LENGTH: u32 = 20u32;
pub const EAPCODE_Failure: u32 = 4u32;
pub const EAPCODE_Request: u32 = 1u32;
pub const EAPCODE_Response: u32 = 2u32;
pub const EAPCODE_Success: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAPHOST_AUTH_INFO {
    pub status: EAPHOST_AUTH_STATUS,
    pub dwErrorCode: u32,
    pub dwReasonCode: u32,
}
impl EAPHOST_AUTH_INFO {}
impl ::std::default::Default for EAPHOST_AUTH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAPHOST_AUTH_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAPHOST_AUTH_INFO")
            .field("status", &self.status)
            .field("dwErrorCode", &self.dwErrorCode)
            .field("dwReasonCode", &self.dwReasonCode)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAPHOST_AUTH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
            && self.dwErrorCode == other.dwErrorCode
            && self.dwReasonCode == other.dwReasonCode
    }
}
impl ::std::cmp::Eq for EAPHOST_AUTH_INFO {}
unsafe impl ::windows::runtime::Abi for EAPHOST_AUTH_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAPHOST_AUTH_STATUS(pub i32);
pub const EapHostInvalidSession: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(0i32);
pub const EapHostAuthNotStarted: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(1i32);
pub const EapHostAuthIdentityExchange: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(2i32);
pub const EapHostAuthNegotiatingType: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(3i32);
pub const EapHostAuthInProgress: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(4i32);
pub const EapHostAuthSucceeded: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(5i32);
pub const EapHostAuthFailed: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(6i32);
impl ::std::convert::From<i32> for EAPHOST_AUTH_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAPHOST_AUTH_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAPHOST_IDENTITY_UI_PARAMS {
    pub eapMethodType: EAP_METHOD_TYPE,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: super::super::Foundation::PWSTR,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl EAPHOST_IDENTITY_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAPHOST_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAPHOST_IDENTITY_UI_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAPHOST_IDENTITY_UI_PARAMS")
            .field("eapMethodType", &self.eapMethodType)
            .field("dwFlags", &self.dwFlags)
            .field("dwSizeofConnectionData", &self.dwSizeofConnectionData)
            .field("pConnectionData", &self.pConnectionData)
            .field("dwSizeofUserData", &self.dwSizeofUserData)
            .field("pUserData", &self.pUserData)
            .field("dwSizeofUserDataOut", &self.dwSizeofUserDataOut)
            .field("pUserDataOut", &self.pUserDataOut)
            .field("pwszIdentity", &self.pwszIdentity)
            .field("dwError", &self.dwError)
            .field("pEapError", &self.pEapError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAPHOST_IDENTITY_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.eapMethodType == other.eapMethodType
            && self.dwFlags == other.dwFlags
            && self.dwSizeofConnectionData == other.dwSizeofConnectionData
            && self.pConnectionData == other.pConnectionData
            && self.dwSizeofUserData == other.dwSizeofUserData
            && self.pUserData == other.pUserData
            && self.dwSizeofUserDataOut == other.dwSizeofUserDataOut
            && self.pUserDataOut == other.pUserDataOut
            && self.pwszIdentity == other.pwszIdentity
            && self.dwError == other.dwError
            && self.pEapError == other.pEapError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAPHOST_IDENTITY_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAPHOST_IDENTITY_UI_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAPHOST_INTERACTIVE_UI_PARAMS {
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl EAPHOST_INTERACTIVE_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAPHOST_INTERACTIVE_UI_PARAMS")
            .field("dwSizeofContextData", &self.dwSizeofContextData)
            .field("pContextData", &self.pContextData)
            .field("dwSizeofInteractiveUIData", &self.dwSizeofInteractiveUIData)
            .field("pInteractiveUIData", &self.pInteractiveUIData)
            .field("dwError", &self.dwError)
            .field("pEapError", &self.pEapError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeofContextData == other.dwSizeofContextData
            && self.pContextData == other.pContextData
            && self.dwSizeofInteractiveUIData == other.dwSizeofInteractiveUIData
            && self.pInteractiveUIData == other.pInteractiveUIData
            && self.dwError == other.dwError
            && self.pEapError == other.pEapError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAPHOST_INTERACTIVE_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAPHOST_INTERACTIVE_UI_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAPHOST_METHOD_API_VERSION: u32 = 1u32;
pub const EAPHOST_PEER_API_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_ATTRIBUTE {
    pub eaType: EAP_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub pValue: *mut u8,
}
impl EAP_ATTRIBUTE {}
impl ::std::default::Default for EAP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_ATTRIBUTE")
            .field("eaType", &self.eaType)
            .field("dwLength", &self.dwLength)
            .field("pValue", &self.pValue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.eaType == other.eaType
            && self.dwLength == other.dwLength
            && self.pValue == other.pValue
    }
}
impl ::std::cmp::Eq for EAP_ATTRIBUTE {}
unsafe impl ::windows::runtime::Abi for EAP_ATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_ATTRIBUTES {
    pub dwNumberOfAttributes: u32,
    pub pAttribs: *mut EAP_ATTRIBUTE,
}
impl EAP_ATTRIBUTES {}
impl ::std::default::Default for EAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_ATTRIBUTES")
            .field("dwNumberOfAttributes", &self.dwNumberOfAttributes)
            .field("pAttribs", &self.pAttribs)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfAttributes == other.dwNumberOfAttributes && self.pAttribs == other.pAttribs
    }
}
impl ::std::cmp::Eq for EAP_ATTRIBUTES {}
unsafe impl ::windows::runtime::Abi for EAP_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_ATTRIBUTE_TYPE(pub i32);
pub const eatMinimum: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(0i32);
pub const eatUserName: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(1i32);
pub const eatUserPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(2i32);
pub const eatMD5CHAPPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(3i32);
pub const eatNASIPAddress: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(4i32);
pub const eatNASPort: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(5i32);
pub const eatServiceType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(6i32);
pub const eatFramedProtocol: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(7i32);
pub const eatFramedIPAddress: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8i32);
pub const eatFramedIPNetmask: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9i32);
pub const eatFramedRouting: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(10i32);
pub const eatFilterId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(11i32);
pub const eatFramedMTU: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(12i32);
pub const eatFramedCompression: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(13i32);
pub const eatLoginIPHost: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(14i32);
pub const eatLoginService: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(15i32);
pub const eatLoginTCPPort: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(16i32);
pub const eatUnassigned17: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(17i32);
pub const eatReplyMessage: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(18i32);
pub const eatCallbackNumber: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(19i32);
pub const eatCallbackId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(20i32);
pub const eatUnassigned21: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(21i32);
pub const eatFramedRoute: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(22i32);
pub const eatFramedIPXNetwork: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(23i32);
pub const eatState: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(24i32);
pub const eatClass: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(25i32);
pub const eatVendorSpecific: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(26i32);
pub const eatSessionTimeout: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(27i32);
pub const eatIdleTimeout: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(28i32);
pub const eatTerminationAction: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(29i32);
pub const eatCalledStationId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(30i32);
pub const eatCallingStationId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(31i32);
pub const eatNASIdentifier: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(32i32);
pub const eatProxyState: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(33i32);
pub const eatLoginLATService: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(34i32);
pub const eatLoginLATNode: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(35i32);
pub const eatLoginLATGroup: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(36i32);
pub const eatFramedAppleTalkLink: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(37i32);
pub const eatFramedAppleTalkNetwork: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(38i32);
pub const eatFramedAppleTalkZone: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(39i32);
pub const eatAcctStatusType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(40i32);
pub const eatAcctDelayTime: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(41i32);
pub const eatAcctInputOctets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(42i32);
pub const eatAcctOutputOctets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(43i32);
pub const eatAcctSessionId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(44i32);
pub const eatAcctAuthentic: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(45i32);
pub const eatAcctSessionTime: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(46i32);
pub const eatAcctInputPackets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(47i32);
pub const eatAcctOutputPackets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(48i32);
pub const eatAcctTerminateCause: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(49i32);
pub const eatAcctMultiSessionId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(50i32);
pub const eatAcctLinkCount: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(51i32);
pub const eatAcctEventTimeStamp: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(55i32);
pub const eatMD5CHAPChallenge: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(60i32);
pub const eatNASPortType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(61i32);
pub const eatPortLimit: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(62i32);
pub const eatLoginLATPort: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(63i32);
pub const eatTunnelType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(64i32);
pub const eatTunnelMediumType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(65i32);
pub const eatTunnelClientEndpoint: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(66i32);
pub const eatTunnelServerEndpoint: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(67i32);
pub const eatARAPPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(70i32);
pub const eatARAPFeatures: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(71i32);
pub const eatARAPZoneAccess: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(72i32);
pub const eatARAPSecurity: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(73i32);
pub const eatARAPSecurityData: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(74i32);
pub const eatPasswordRetry: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(75i32);
pub const eatPrompt: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(76i32);
pub const eatConnectInfo: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(77i32);
pub const eatConfigurationToken: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(78i32);
pub const eatEAPMessage: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(79i32);
pub const eatSignature: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(80i32);
pub const eatARAPChallengeResponse: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(84i32);
pub const eatAcctInterimInterval: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(85i32);
pub const eatNASIPv6Address: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(95i32);
pub const eatFramedInterfaceId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(96i32);
pub const eatFramedIPv6Prefix: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(97i32);
pub const eatLoginIPv6Host: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(98i32);
pub const eatFramedIPv6Route: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(99i32);
pub const eatFramedIPv6Pool: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(100i32);
pub const eatARAPGuestLogon: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8096i32);
pub const eatCertificateOID: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8097i32);
pub const eatEAPConfiguration: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8098i32);
pub const eatPEAPEmbeddedEAPTypeId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8099i32);
pub const eatPEAPFastRoamedSession: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8100i32);
pub const eatFastRoamedSession: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8100i32);
pub const eatEAPTLV: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8102i32);
pub const eatCredentialsChanged: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8103i32);
pub const eatInnerEapMethodType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8104i32);
pub const eatClearTextPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8107i32);
pub const eatQuarantineSoH: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8150i32);
pub const eatCertificateThumbprint: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8250i32);
pub const eatPeerId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9000i32);
pub const eatServerId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9001i32);
pub const eatMethodId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9002i32);
pub const eatEMSK: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9003i32);
pub const eatSessionId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9004i32);
pub const eatReserved: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(-1i32);
impl ::std::convert::From<i32> for EAP_ATTRIBUTE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_ATTRIBUTE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_AUTHENTICATOR_METHOD_ROUTINES {
    pub dwSizeInBytes: u32,
    pub pEapType: *mut EAP_METHOD_TYPE,
    pub EapMethodAuthenticatorInitialize: isize,
    pub EapMethodAuthenticatorBeginSession: isize,
    pub EapMethodAuthenticatorUpdateInnerMethodParams: isize,
    pub EapMethodAuthenticatorReceivePacket: isize,
    pub EapMethodAuthenticatorSendPacket: isize,
    pub EapMethodAuthenticatorGetAttributes: isize,
    pub EapMethodAuthenticatorSetAttributes: isize,
    pub EapMethodAuthenticatorGetResult: isize,
    pub EapMethodAuthenticatorEndSession: isize,
    pub EapMethodAuthenticatorShutdown: isize,
}
impl EAP_AUTHENTICATOR_METHOD_ROUTINES {}
impl ::std::default::Default for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_AUTHENTICATOR_METHOD_ROUTINES")
            .field("dwSizeInBytes", &self.dwSizeInBytes)
            .field("pEapType", &self.pEapType)
            .field(
                "EapMethodAuthenticatorInitialize",
                &self.EapMethodAuthenticatorInitialize,
            )
            .field(
                "EapMethodAuthenticatorBeginSession",
                &self.EapMethodAuthenticatorBeginSession,
            )
            .field(
                "EapMethodAuthenticatorUpdateInnerMethodParams",
                &self.EapMethodAuthenticatorUpdateInnerMethodParams,
            )
            .field(
                "EapMethodAuthenticatorReceivePacket",
                &self.EapMethodAuthenticatorReceivePacket,
            )
            .field(
                "EapMethodAuthenticatorSendPacket",
                &self.EapMethodAuthenticatorSendPacket,
            )
            .field(
                "EapMethodAuthenticatorGetAttributes",
                &self.EapMethodAuthenticatorGetAttributes,
            )
            .field(
                "EapMethodAuthenticatorSetAttributes",
                &self.EapMethodAuthenticatorSetAttributes,
            )
            .field(
                "EapMethodAuthenticatorGetResult",
                &self.EapMethodAuthenticatorGetResult,
            )
            .field(
                "EapMethodAuthenticatorEndSession",
                &self.EapMethodAuthenticatorEndSession,
            )
            .field(
                "EapMethodAuthenticatorShutdown",
                &self.EapMethodAuthenticatorShutdown,
            )
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes
            && self.pEapType == other.pEapType
            && self.EapMethodAuthenticatorInitialize == other.EapMethodAuthenticatorInitialize
            && self.EapMethodAuthenticatorBeginSession == other.EapMethodAuthenticatorBeginSession
            && self.EapMethodAuthenticatorUpdateInnerMethodParams
                == other.EapMethodAuthenticatorUpdateInnerMethodParams
            && self.EapMethodAuthenticatorReceivePacket == other.EapMethodAuthenticatorReceivePacket
            && self.EapMethodAuthenticatorSendPacket == other.EapMethodAuthenticatorSendPacket
            && self.EapMethodAuthenticatorGetAttributes == other.EapMethodAuthenticatorGetAttributes
            && self.EapMethodAuthenticatorSetAttributes == other.EapMethodAuthenticatorSetAttributes
            && self.EapMethodAuthenticatorGetResult == other.EapMethodAuthenticatorGetResult
            && self.EapMethodAuthenticatorEndSession == other.EapMethodAuthenticatorEndSession
            && self.EapMethodAuthenticatorShutdown == other.EapMethodAuthenticatorShutdown
    }
}
impl ::std::cmp::Eq for EAP_AUTHENTICATOR_METHOD_ROUTINES {}
unsafe impl ::windows::runtime::Abi for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_AUTHENTICATOR_SEND_TIMEOUT(pub i32);
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_NONE: EAP_AUTHENTICATOR_SEND_TIMEOUT =
    EAP_AUTHENTICATOR_SEND_TIMEOUT(0i32);
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_BASIC: EAP_AUTHENTICATOR_SEND_TIMEOUT =
    EAP_AUTHENTICATOR_SEND_TIMEOUT(1i32);
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_INTERACTIVE: EAP_AUTHENTICATOR_SEND_TIMEOUT =
    EAP_AUTHENTICATOR_SEND_TIMEOUT(2i32);
impl ::std::convert::From<i32> for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CONFIG_INPUT_FIELD_ARRAY {
    pub dwVersion: u32,
    pub dwNumberOfFields: u32,
    pub pFields: *mut EAP_CONFIG_INPUT_FIELD_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_CONFIG_INPUT_FIELD_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_CONFIG_INPUT_FIELD_ARRAY")
            .field("dwVersion", &self.dwVersion)
            .field("dwNumberOfFields", &self.dwNumberOfFields)
            .field("pFields", &self.pFields)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwNumberOfFields == other.dwNumberOfFields
            && self.pFields == other.pFields
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_CONFIG_INPUT_FIELD_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_CONFIG_INPUT_FIELD_ARRAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CONFIG_INPUT_FIELD_DATA {
    pub dwSize: u32,
    pub Type: EAP_CONFIG_INPUT_FIELD_TYPE,
    pub dwFlagProps: u32,
    pub pwszLabel: super::super::Foundation::PWSTR,
    pub pwszData: super::super::Foundation::PWSTR,
    pub dwMinDataLength: u32,
    pub dwMaxDataLength: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_CONFIG_INPUT_FIELD_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_CONFIG_INPUT_FIELD_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_CONFIG_INPUT_FIELD_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_CONFIG_INPUT_FIELD_DATA")
            .field("dwSize", &self.dwSize)
            .field("Type", &self.Type)
            .field("dwFlagProps", &self.dwFlagProps)
            .field("pwszLabel", &self.pwszLabel)
            .field("pwszData", &self.pwszData)
            .field("dwMinDataLength", &self.dwMinDataLength)
            .field("dwMaxDataLength", &self.dwMaxDataLength)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_CONFIG_INPUT_FIELD_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.Type == other.Type
            && self.dwFlagProps == other.dwFlagProps
            && self.pwszLabel == other.pwszLabel
            && self.pwszData == other.pwszData
            && self.dwMinDataLength == other.dwMinDataLength
            && self.dwMaxDataLength == other.dwMaxDataLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_CONFIG_INPUT_FIELD_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_CONFIG_INPUT_FIELD_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_CONFIG_INPUT_FIELD_TYPE(pub i32);
pub const EapConfigInputUsername: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(0i32);
pub const EapConfigInputPassword: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(1i32);
pub const EapConfigInputNetworkUsername: EAP_CONFIG_INPUT_FIELD_TYPE =
    EAP_CONFIG_INPUT_FIELD_TYPE(2i32);
pub const EapConfigInputNetworkPassword: EAP_CONFIG_INPUT_FIELD_TYPE =
    EAP_CONFIG_INPUT_FIELD_TYPE(3i32);
pub const EapConfigInputPin: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(4i32);
pub const EapConfigInputPSK: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(5i32);
pub const EapConfigInputEdit: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(6i32);
pub const EapConfigSmartCardUsername: EAP_CONFIG_INPUT_FIELD_TYPE =
    EAP_CONFIG_INPUT_FIELD_TYPE(7i32);
pub const EapConfigSmartCardError: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(8i32);
impl ::std::convert::From<i32> for EAP_CONFIG_INPUT_FIELD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_CONFIG_INPUT_FIELD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_CREDENTIAL_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CRED_EXPIRY_REQ {
    pub curCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub newCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_CRED_EXPIRY_REQ {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_CRED_EXPIRY_REQ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_CRED_EXPIRY_REQ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_CRED_EXPIRY_REQ")
            .field("curCreds", &self.curCreds)
            .field("newCreds", &self.newCreds)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_CRED_EXPIRY_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.curCreds == other.curCreds && self.newCreds == other.newCreds
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_CRED_EXPIRY_REQ {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_CRED_EXPIRY_REQ {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: ::windows::runtime::GUID,
    pub repairGuid: ::windows::runtime::GUID,
    pub helpLinkGuid: ::windows::runtime::GUID,
    pub pRootCauseString: super::super::Foundation::PWSTR,
    pub pRepairString: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_ERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_ERROR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_ERROR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_ERROR")
            .field("dwWinError", &self.dwWinError)
            .field("r#type", &self.r#type)
            .field("dwReasonCode", &self.dwReasonCode)
            .field("rootCauseGuid", &self.rootCauseGuid)
            .field("repairGuid", &self.repairGuid)
            .field("helpLinkGuid", &self.helpLinkGuid)
            .field("pRootCauseString", &self.pRootCauseString)
            .field("pRepairString", &self.pRepairString)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.dwWinError == other.dwWinError
            && self.r#type == other.r#type
            && self.dwReasonCode == other.dwReasonCode
            && self.rootCauseGuid == other.rootCauseGuid
            && self.repairGuid == other.repairGuid
            && self.helpLinkGuid == other.helpLinkGuid
            && self.pRootCauseString == other.pRootCauseString
            && self.pRepairString == other.pRepairString
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_ERROR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_ERROR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_E_AUTHENTICATION_FAILED: u32 = 2151809045u32;
pub const EAP_E_CERT_STORE_INACCESSIBLE: u32 = 2151809040u32;
pub const EAP_E_EAPHOST_EAPQEC_INACCESSIBLE: u32 = 2151809043u32;
pub const EAP_E_EAPHOST_FIRST: i32 = -2143158272i32;
pub const EAP_E_EAPHOST_IDENTITY_UNKNOWN: u32 = 2151809044u32;
pub const EAP_E_EAPHOST_LAST: i32 = -2143158017i32;
pub const EAP_E_EAPHOST_METHOD_INVALID_PACKET: u32 = 2151809047u32;
pub const EAP_E_EAPHOST_METHOD_NOT_INSTALLED: u32 = 2151809041u32;
pub const EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED: u32 = 2151809056u32;
pub const EAP_E_EAPHOST_REMOTE_INVALID_PACKET: u32 = 2151809048u32;
pub const EAP_E_EAPHOST_THIRDPARTY_METHOD_HOST_RESET: u32 = 2151809042u32;
pub const EAP_E_EAPHOST_XML_MALFORMED: u32 = 2151809049u32;
pub const EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO: u32 = 2151809050u32;
pub const EAP_E_NO_SMART_CARD_READER: u32 = 2151809299u32;
pub const EAP_E_SERVER_CERT_EXPIRED: u32 = 2151809538u32;
pub const EAP_E_SERVER_CERT_INVALID: u32 = 2151809537u32;
pub const EAP_E_SERVER_CERT_NOT_FOUND: u32 = 2151809536u32;
pub const EAP_E_SERVER_CERT_OTHER_ERROR: u32 = 2151809540u32;
pub const EAP_E_SERVER_CERT_REVOKED: u32 = 2151809539u32;
pub const EAP_E_SERVER_FIRST: i32 = -2143157760i32;
pub const EAP_E_SERVER_LAST: i32 = -2143157505i32;
pub const EAP_E_SERVER_ROOT_CERT_FIRST: i32 = -2143157248i32;
pub const EAP_E_SERVER_ROOT_CERT_INVALID: u32 = 2151810049u32;
pub const EAP_E_SERVER_ROOT_CERT_LAST: i32 = -2143156993i32;
pub const EAP_E_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 2151810054u32;
pub const EAP_E_SERVER_ROOT_CERT_NOT_FOUND: u32 = 2151810048u32;
pub const EAP_E_SIM_NOT_VALID: u32 = 2151810304u32;
pub const EAP_E_USER_CERT_EXPIRED: u32 = 2151809282u32;
pub const EAP_E_USER_CERT_INVALID: u32 = 2151809281u32;
pub const EAP_E_USER_CERT_NOT_FOUND: u32 = 2151809280u32;
pub const EAP_E_USER_CERT_OTHER_ERROR: u32 = 2151809284u32;
pub const EAP_E_USER_CERT_REJECTED: u32 = 2151809285u32;
pub const EAP_E_USER_CERT_REVOKED: u32 = 2151809283u32;
pub const EAP_E_USER_CREDENTIALS_REJECTED: u32 = 2151809297u32;
pub const EAP_E_USER_FIRST: i32 = -2143158016i32;
pub const EAP_E_USER_LAST: i32 = -2143157761i32;
pub const EAP_E_USER_NAME_PASSWORD_REJECTED: u32 = 2151809298u32;
pub const EAP_E_USER_ROOT_CERT_EXPIRED: u32 = 2151809794u32;
pub const EAP_E_USER_ROOT_CERT_FIRST: i32 = -2143157504i32;
pub const EAP_E_USER_ROOT_CERT_INVALID: u32 = 2151809793u32;
pub const EAP_E_USER_ROOT_CERT_LAST: i32 = -2143157249i32;
pub const EAP_E_USER_ROOT_CERT_NOT_FOUND: u32 = 2151809792u32;
pub const EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
pub const EAP_FLAG_FULL_AUTH: u32 = 4096u32;
pub const EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
pub const EAP_FLAG_LOGON: u32 = 4u32;
pub const EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
pub const EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
pub const EAP_FLAG_ONLY_EAP_TLS: u32 = 16777216u32;
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: u32 = 8192u32;
pub const EAP_FLAG_PREVIEW: u32 = 8u32;
pub const EAP_FLAG_PRE_LOGON: u32 = 131072u32;
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
pub const EAP_FLAG_Reserved1: u32 = 1u32;
pub const EAP_FLAG_Reserved2: u32 = 16u32;
pub const EAP_FLAG_Reserved3: u32 = 128u32;
pub const EAP_FLAG_Reserved4: u32 = 256u32;
pub const EAP_FLAG_Reserved5: u32 = 1024u32;
pub const EAP_FLAG_Reserved6: u32 = 2048u32;
pub const EAP_FLAG_Reserved7: u32 = 16384u32;
pub const EAP_FLAG_Reserved8: u32 = 1048576u32;
pub const EAP_FLAG_Reserved9: u32 = 4194304u32;
pub const EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
pub const EAP_FLAG_SUPRESS_UI: u32 = 65536u32;
pub const EAP_FLAG_USER_AUTH: u32 = 262144u32;
pub const EAP_FLAG_VPN: u32 = 8388608u32;
pub const EAP_GROUP_MASK: i32 = 65280i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_INTERACTIVE_UI_DATA {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwDataType: EAP_INTERACTIVE_UI_DATA_TYPE,
    pub cbUiData: u32,
    pub pbUiData: EAP_UI_DATA_FORMAT,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_INTERACTIVE_UI_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_INTERACTIVE_UI_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_INTERACTIVE_UI_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_INTERACTIVE_UI_DATA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_INTERACTIVE_UI_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_INTERACTIVE_UI_DATA_TYPE(pub i32);
pub const EapCredReq: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(0i32);
pub const EapCredResp: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(1i32);
pub const EapCredExpiryReq: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(2i32);
pub const EapCredExpiryResp: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(3i32);
pub const EapCredLogonReq: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(4i32);
pub const EapCredLogonResp: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(5i32);
impl ::std::convert::From<i32> for EAP_INTERACTIVE_UI_DATA_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_INTERACTIVE_UI_DATA_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_INTERACTIVE_UI_DATA_VERSION: u32 = 1u32;
pub const EAP_INVALID_PACKET: u32 = 2151809048u32;
pub const EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED: u32 = 1078067222u32;
pub const EAP_I_EAPHOST_FIRST: i32 = -2143158272i32;
pub const EAP_I_EAPHOST_LAST: i32 = -2143158017i32;
pub const EAP_I_USER_ACCOUNT_OTHER_ERROR: u32 = 1078067472u32;
pub const EAP_I_USER_FIRST: i32 = 1078067456i32;
pub const EAP_I_USER_LAST: i32 = 1078067711i32;
pub const EAP_METHOD_AUTHENTICATOR_CONFIG_IS_IDENTITY_PRIVACY: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(pub i32);
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_DISCARD: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION =
    EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(0i32);
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_SEND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION =
    EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(1i32);
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESULT: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION =
    EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(2i32);
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESPOND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION =
    EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(3i32);
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_AUTHENTICATE: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION =
    EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(4i32);
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_HANDLE_IDENTITY:
    EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(5i32);
impl ::std::convert::From<i32> for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_AUTHENTICATOR_RESULT {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReason: u32,
    pub pAuthAttribs: *mut EAP_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_AUTHENTICATOR_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_AUTHENTICATOR_RESULT")
            .field("fIsSuccess", &self.fIsSuccess)
            .field("dwFailureReason", &self.dwFailureReason)
            .field("pAuthAttribs", &self.pAuthAttribs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.fIsSuccess == other.fIsSuccess
            && self.dwFailureReason == other.dwFailureReason
            && self.pAuthAttribs == other.pAuthAttribs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_AUTHENTICATOR_RESULT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_AUTHENTICATOR_RESULT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfo: *mut EAP_METHOD_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_INFO")
            .field("eaptype", &self.eaptype)
            .field("pwszAuthorName", &self.pwszAuthorName)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("eapProperties", &self.eapProperties)
            .field("pInnerMethodInfo", &self.pInnerMethodInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.eaptype == other.eaptype
            && self.pwszAuthorName == other.pwszAuthorName
            && self.pwszFriendlyName == other.pwszFriendlyName
            && self.eapProperties == other.eapProperties
            && self.pInnerMethodInfo == other.pInnerMethodInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_ARRAY {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_INFO_ARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_INFO_ARRAY")
            .field("dwNumberOfMethods", &self.dwNumberOfMethods)
            .field("pEapMethods", &self.pEapMethods)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfMethods == other.dwNumberOfMethods && self.pEapMethods == other.pEapMethods
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_INFO_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_INFO_ARRAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_ARRAY_EX {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_INFO_ARRAY_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_INFO_ARRAY_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_INFO_ARRAY_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_INFO_ARRAY_EX")
            .field("dwNumberOfMethods", &self.dwNumberOfMethods)
            .field("pEapMethods", &self.pEapMethods)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_INFO_ARRAY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfMethods == other.dwNumberOfMethods && self.pEapMethods == other.pEapMethods
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_INFO_ARRAY_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_INFO_ARRAY_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_EX {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfoArray: *mut EAP_METHOD_INFO_ARRAY_EX,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_INFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_INFO_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_INFO_EX")
            .field("eaptype", &self.eaptype)
            .field("pwszAuthorName", &self.pwszAuthorName)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("eapProperties", &self.eapProperties)
            .field("pInnerMethodInfoArray", &self.pInnerMethodInfoArray)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.eaptype == other.eaptype
            && self.pwszAuthorName == other.pwszAuthorName
            && self.pwszFriendlyName == other.pwszFriendlyName
            && self.eapProperties == other.eapProperties
            && self.pInnerMethodInfoArray == other.pInnerMethodInfoArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_INFO_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_METHOD_INVALID_PACKET: u32 = 2151809047u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY {
    pub eapMethodPropertyType: EAP_METHOD_PROPERTY_TYPE,
    pub eapMethodPropertyValueType: EAP_METHOD_PROPERTY_VALUE_TYPE,
    pub eapMethodPropertyValue: EAP_METHOD_PROPERTY_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_PROPERTY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_ARRAY {
    pub dwNumberOfProperties: u32,
    pub pMethodProperty: *mut EAP_METHOD_PROPERTY,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_PROPERTY_ARRAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_PROPERTY_ARRAY")
            .field("dwNumberOfProperties", &self.dwNumberOfProperties)
            .field("pMethodProperty", &self.pMethodProperty)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_PROPERTY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfProperties == other.dwNumberOfProperties
            && self.pMethodProperty == other.pMethodProperty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_ARRAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_METHOD_PROPERTY_TYPE(pub i32);
pub const emptPropCipherSuiteNegotiation: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(0i32);
pub const emptPropMutualAuth: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(1i32);
pub const emptPropIntegrity: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(2i32);
pub const emptPropReplayProtection: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(3i32);
pub const emptPropConfidentiality: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(4i32);
pub const emptPropKeyDerivation: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(5i32);
pub const emptPropKeyStrength64: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(6i32);
pub const emptPropKeyStrength128: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(7i32);
pub const emptPropKeyStrength256: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(8i32);
pub const emptPropKeyStrength512: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(9i32);
pub const emptPropKeyStrength1024: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(10i32);
pub const emptPropDictionaryAttackResistance: EAP_METHOD_PROPERTY_TYPE =
    EAP_METHOD_PROPERTY_TYPE(11i32);
pub const emptPropFastReconnect: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(12i32);
pub const emptPropCryptoBinding: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(13i32);
pub const emptPropSessionIndependence: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(14i32);
pub const emptPropFragmentation: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(15i32);
pub const emptPropChannelBinding: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(16i32);
pub const emptPropNap: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(17i32);
pub const emptPropStandalone: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(18i32);
pub const emptPropMppeEncryption: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(19i32);
pub const emptPropTunnelMethod: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(20i32);
pub const emptPropSupportsConfig: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(21i32);
pub const emptPropCertifiedMethod: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(22i32);
pub const emptPropHiddenMethod: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(23i32);
pub const emptPropMachineAuth: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(24i32);
pub const emptPropUserAuth: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(25i32);
pub const emptPropIdentityPrivacy: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(26i32);
pub const emptPropMethodChaining: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(27i32);
pub const emptPropSharedStateEquivalence: EAP_METHOD_PROPERTY_TYPE =
    EAP_METHOD_PROPERTY_TYPE(28i32);
pub const emptLegacyMethodPropertyFlag: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(31i32);
pub const emptPropVendorSpecific: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(255i32);
impl ::std::convert::From<i32> for EAP_METHOD_PROPERTY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EAP_METHOD_PROPERTY_VALUE {
    pub empvBool: EAP_METHOD_PROPERTY_VALUE_BOOL,
    pub empvDword: EAP_METHOD_PROPERTY_VALUE_DWORD,
    pub empvString: EAP_METHOD_PROPERTY_VALUE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_VALUE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_VALUE_BOOL {
    pub length: u32,
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_METHOD_PROPERTY_VALUE_BOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_PROPERTY_VALUE_BOOL")
            .field("length", &self.length)
            .field("value", &self.value)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_BOOL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_VALUE_BOOL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_METHOD_PROPERTY_VALUE_DWORD {
    pub length: u32,
    pub value: u32,
}
impl EAP_METHOD_PROPERTY_VALUE_DWORD {}
impl ::std::default::Default for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_PROPERTY_VALUE_DWORD")
            .field("length", &self.length)
            .field("value", &self.value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
impl ::std::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_DWORD {}
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_VALUE_DWORD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_METHOD_PROPERTY_VALUE_STRING {
    pub length: u32,
    pub value: *mut u8,
}
impl EAP_METHOD_PROPERTY_VALUE_STRING {}
impl ::std::default::Default for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_PROPERTY_VALUE_STRING")
            .field("length", &self.length)
            .field("value", &self.value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
impl ::std::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_STRING {}
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_VALUE_STRING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EAP_METHOD_PROPERTY_VALUE_TYPE(pub i32);
pub const empvtBool: EAP_METHOD_PROPERTY_VALUE_TYPE = EAP_METHOD_PROPERTY_VALUE_TYPE(0i32);
pub const empvtDword: EAP_METHOD_PROPERTY_VALUE_TYPE = EAP_METHOD_PROPERTY_VALUE_TYPE(1i32);
pub const empvtString: EAP_METHOD_PROPERTY_VALUE_TYPE = EAP_METHOD_PROPERTY_VALUE_TYPE(2i32);
impl ::std::convert::From<i32> for EAP_METHOD_PROPERTY_VALUE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EAP_METHOD_PROPERTY_VALUE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_METHOD_TYPE {
    pub eapType: EAP_TYPE,
    pub dwAuthorId: u32,
}
impl EAP_METHOD_TYPE {}
impl ::std::default::Default for EAP_METHOD_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_METHOD_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_METHOD_TYPE")
            .field("eapType", &self.eapType)
            .field("dwAuthorId", &self.dwAuthorId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_METHOD_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.eapType == other.eapType && self.dwAuthorId == other.dwAuthorId
    }
}
impl ::std::cmp::Eq for EAP_METHOD_TYPE {}
unsafe impl ::windows::runtime::Abi for EAP_METHOD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_PEER_FLAG_GUEST_ACCESS: u32 = 64u32;
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: u32 = 32768u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_PEER_METHOD_ROUTINES {
    pub dwVersion: u32,
    pub pEapType: *mut EAP_TYPE,
    pub EapPeerInitialize: isize,
    pub EapPeerGetIdentity: isize,
    pub EapPeerBeginSession: isize,
    pub EapPeerSetCredentials: isize,
    pub EapPeerProcessRequestPacket: isize,
    pub EapPeerGetResponsePacket: isize,
    pub EapPeerGetResult: isize,
    pub EapPeerGetUIContext: isize,
    pub EapPeerSetUIContext: isize,
    pub EapPeerGetResponseAttributes: isize,
    pub EapPeerSetResponseAttributes: isize,
    pub EapPeerEndSession: isize,
    pub EapPeerShutdown: isize,
}
impl EAP_PEER_METHOD_ROUTINES {}
impl ::std::default::Default for EAP_PEER_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_PEER_METHOD_ROUTINES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_PEER_METHOD_ROUTINES")
            .field("dwVersion", &self.dwVersion)
            .field("pEapType", &self.pEapType)
            .field("EapPeerInitialize", &self.EapPeerInitialize)
            .field("EapPeerGetIdentity", &self.EapPeerGetIdentity)
            .field("EapPeerBeginSession", &self.EapPeerBeginSession)
            .field("EapPeerSetCredentials", &self.EapPeerSetCredentials)
            .field(
                "EapPeerProcessRequestPacket",
                &self.EapPeerProcessRequestPacket,
            )
            .field("EapPeerGetResponsePacket", &self.EapPeerGetResponsePacket)
            .field("EapPeerGetResult", &self.EapPeerGetResult)
            .field("EapPeerGetUIContext", &self.EapPeerGetUIContext)
            .field("EapPeerSetUIContext", &self.EapPeerSetUIContext)
            .field(
                "EapPeerGetResponseAttributes",
                &self.EapPeerGetResponseAttributes,
            )
            .field(
                "EapPeerSetResponseAttributes",
                &self.EapPeerSetResponseAttributes,
            )
            .field("EapPeerEndSession", &self.EapPeerEndSession)
            .field("EapPeerShutdown", &self.EapPeerShutdown)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_PEER_METHOD_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.pEapType == other.pEapType
            && self.EapPeerInitialize == other.EapPeerInitialize
            && self.EapPeerGetIdentity == other.EapPeerGetIdentity
            && self.EapPeerBeginSession == other.EapPeerBeginSession
            && self.EapPeerSetCredentials == other.EapPeerSetCredentials
            && self.EapPeerProcessRequestPacket == other.EapPeerProcessRequestPacket
            && self.EapPeerGetResponsePacket == other.EapPeerGetResponsePacket
            && self.EapPeerGetResult == other.EapPeerGetResult
            && self.EapPeerGetUIContext == other.EapPeerGetUIContext
            && self.EapPeerSetUIContext == other.EapPeerSetUIContext
            && self.EapPeerGetResponseAttributes == other.EapPeerGetResponseAttributes
            && self.EapPeerSetResponseAttributes == other.EapPeerSetResponseAttributes
            && self.EapPeerEndSession == other.EapPeerEndSession
            && self.EapPeerShutdown == other.EapPeerShutdown
    }
}
impl ::std::cmp::Eq for EAP_PEER_METHOD_ROUTINES {}
unsafe impl ::windows::runtime::Abi for EAP_PEER_METHOD_ROUTINES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EAP_TYPE {
    pub r#type: u8,
    pub dwVendorId: u32,
    pub dwVendorType: u32,
}
impl EAP_TYPE {}
impl ::std::default::Default for EAP_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EAP_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EAP_TYPE")
            .field("r#type", &self.r#type)
            .field("dwVendorId", &self.dwVendorId)
            .field("dwVendorType", &self.dwVendorType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EAP_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type
            && self.dwVendorId == other.dwVendorId
            && self.dwVendorType == other.dwVendorType
    }
}
impl ::std::cmp::Eq for EAP_TYPE {}
unsafe impl ::windows::runtime::Abi for EAP_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EAP_UI_DATA_FORMAT {
    pub credData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub credExpiryData: *mut EAP_CRED_EXPIRY_REQ,
    pub credLogonData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
}
#[cfg(feature = "Win32_Foundation")]
impl EAP_UI_DATA_FORMAT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EAP_UI_DATA_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EAP_UI_DATA_FORMAT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EAP_UI_DATA_FORMAT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EAP_UI_DATA_FORMAT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapCertificateCredential {
    pub certHash: [u8; 20],
    pub password: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl EapCertificateCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapCertificateCredential {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EapCertificateCredential {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapCertificateCredential")
            .field("certHash", &self.certHash)
            .field("password", &self.password)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapCertificateCredential {
    fn eq(&self, other: &Self) -> bool {
        self.certHash == other.certHash && self.password == other.password
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapCertificateCredential {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapCertificateCredential {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapCode(pub i32);
pub const EapCodeMinimum: EapCode = EapCode(1i32);
pub const EapCodeRequest: EapCode = EapCode(1i32);
pub const EapCodeResponse: EapCode = EapCode(2i32);
pub const EapCodeSuccess: EapCode = EapCode(3i32);
pub const EapCodeFailure: EapCode = EapCode(4i32);
pub const EapCodeMaximum: EapCode = EapCode(4i32);
impl ::std::convert::From<i32> for EapCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapCode {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapCredential {
    pub credType: EapCredentialType,
    pub credData: EapCredentialTypeData,
}
#[cfg(feature = "Win32_Foundation")]
impl EapCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapCredential {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapCredential {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapCredential {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapCredential {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapCredentialType(pub i32);
pub const EAP_EMPTY_CREDENTIAL: EapCredentialType = EapCredentialType(0i32);
pub const EAP_USERNAME_PASSWORD_CREDENTIAL: EapCredentialType = EapCredentialType(1i32);
pub const EAP_WINLOGON_CREDENTIAL: EapCredentialType = EapCredentialType(2i32);
pub const EAP_CERTIFICATE_CREDENTIAL: EapCredentialType = EapCredentialType(3i32);
pub const EAP_SIM_CREDENTIAL: EapCredentialType = EapCredentialType(4i32);
impl ::std::convert::From<i32> for EapCredentialType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapCredentialType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EapCredentialTypeData {
    pub username_password: EapUsernamePasswordCredential,
    pub certificate: EapCertificateCredential,
    pub sim: EapSimCredential,
}
#[cfg(feature = "Win32_Foundation")]
impl EapCredentialTypeData {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapCredentialTypeData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapCredentialTypeData {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapCredentialTypeData {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapCredentialTypeData {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapHostPeerAuthParams(pub i32);
pub const EapHostPeerAuthStatus: EapHostPeerAuthParams = EapHostPeerAuthParams(1i32);
pub const EapHostPeerIdentity: EapHostPeerAuthParams = EapHostPeerAuthParams(2i32);
pub const EapHostPeerIdentityExtendedInfo: EapHostPeerAuthParams = EapHostPeerAuthParams(3i32);
pub const EapHostNapInfo: EapHostPeerAuthParams = EapHostPeerAuthParams(4i32);
impl ::std::convert::From<i32> for EapHostPeerAuthParams {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapHostPeerAuthParams {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerBeginSession<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    dwflags: u32,
    eaptype: Param1,
    pattributearray: *const EAP_ATTRIBUTES,
    htokenimpersonateuser: Param3,
    dwsizeofconnectiondata: u32,
    pconnectiondata: *const u8,
    dwsizeofuserdata: u32,
    puserdata: *const u8,
    dwmaxsendpacketsize: u32,
    pconnectionid: *const ::windows::runtime::GUID,
    func: ::std::option::Option<NotificationHandler>,
    pcontextdata: *mut ::std::ffi::c_void,
    psessionid: *mut u32,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerBeginSession(
                dwflags: u32,
                eaptype: EAP_METHOD_TYPE,
                pattributearray: *const EAP_ATTRIBUTES,
                htokenimpersonateuser: super::super::Foundation::HANDLE,
                dwsizeofconnectiondata: u32,
                pconnectiondata: *const u8,
                dwsizeofuserdata: u32,
                puserdata: *const u8,
                dwmaxsendpacketsize: u32,
                pconnectionid: *const ::windows::runtime::GUID,
                func: ::windows::runtime::RawPtr,
                pcontextdata: *mut ::std::ffi::c_void,
                psessionid: *mut u32,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerBeginSession(
            ::std::mem::transmute(dwflags),
            eaptype.into_param().abi(),
            ::std::mem::transmute(pattributearray),
            htokenimpersonateuser.into_param().abi(),
            ::std::mem::transmute(dwsizeofconnectiondata),
            ::std::mem::transmute(pconnectiondata),
            ::std::mem::transmute(dwsizeofuserdata),
            ::std::mem::transmute(puserdata),
            ::std::mem::transmute(dwmaxsendpacketsize),
            ::std::mem::transmute(pconnectionid),
            ::std::mem::transmute(func),
            ::std::mem::transmute(pcontextdata),
            ::std::mem::transmute(psessionid),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerClearConnection(
    pconnectionid: *mut ::windows::runtime::GUID,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerClearConnection(
                pconnectionid: *mut ::windows::runtime::GUID,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerClearConnection(
            ::std::mem::transmute(pconnectionid),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
pub unsafe fn EapHostPeerConfigBlob2Xml<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
>(
    dwflags: u32,
    eapmethodtype: Param1,
    dwsizeofconfigin: u32,
    pconfigin: *const u8,
    ppconfigdoc: *mut ::std::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerConfigBlob2Xml(
                dwflags: u32,
                eapmethodtype: EAP_METHOD_TYPE,
                dwsizeofconfigin: u32,
                pconfigin: *const u8,
                ppconfigdoc: *mut ::windows::runtime::RawPtr,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerConfigBlob2Xml(
            ::std::mem::transmute(dwflags),
            eapmethodtype.into_param().abi(),
            ::std::mem::transmute(dwsizeofconfigin),
            ::std::mem::transmute(pconfigin),
            ::std::mem::transmute(ppconfigdoc),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
pub unsafe fn EapHostPeerConfigXml2Blob<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::MsXml::IXMLDOMNode>,
>(
    dwflags: u32,
    pconfigdoc: Param1,
    pdwsizeofconfigout: *mut u32,
    ppconfigout: *mut *mut u8,
    peapmethodtype: *mut EAP_METHOD_TYPE,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerConfigXml2Blob(
                dwflags: u32,
                pconfigdoc: ::windows::runtime::RawPtr,
                pdwsizeofconfigout: *mut u32,
                ppconfigout: *mut *mut u8,
                peapmethodtype: *mut EAP_METHOD_TYPE,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerConfigXml2Blob(
            ::std::mem::transmute(dwflags),
            pconfigdoc.into_param().abi(),
            ::std::mem::transmute(pdwsizeofconfigout),
            ::std::mem::transmute(ppconfigout),
            ::std::mem::transmute(peapmethodtype),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
pub unsafe fn EapHostPeerCredentialsXml2Blob<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Data::Xml::MsXml::IXMLDOMNode>,
>(
    dwflags: u32,
    pcredentialsdoc: Param1,
    dwsizeofconfigin: u32,
    pconfigin: *const u8,
    pdwsizeofcredentialsout: *mut u32,
    ppcredentialsout: *mut *mut u8,
    peapmethodtype: *mut EAP_METHOD_TYPE,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerCredentialsXml2Blob(
                dwflags: u32,
                pcredentialsdoc: ::windows::runtime::RawPtr,
                dwsizeofconfigin: u32,
                pconfigin: *const u8,
                pdwsizeofcredentialsout: *mut u32,
                ppcredentialsout: *mut *mut u8,
                peapmethodtype: *mut EAP_METHOD_TYPE,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerCredentialsXml2Blob(
            ::std::mem::transmute(dwflags),
            pcredentialsdoc.into_param().abi(),
            ::std::mem::transmute(dwsizeofconfigin),
            ::std::mem::transmute(pconfigin),
            ::std::mem::transmute(pdwsizeofcredentialsout),
            ::std::mem::transmute(ppcredentialsout),
            ::std::mem::transmute(peapmethodtype),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerEndSession(sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerEndSession(sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
        }
        ::std::mem::transmute(EapHostPeerEndSession(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerFreeEapError(peaperror: *mut EAP_ERROR) {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerFreeEapError(peaperror: *mut EAP_ERROR);
        }
        ::std::mem::transmute(EapHostPeerFreeEapError(::std::mem::transmute(peaperror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerFreeErrorMemory(peaperror: *mut EAP_ERROR) {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerFreeErrorMemory(peaperror: *mut EAP_ERROR);
        }
        ::std::mem::transmute(EapHostPeerFreeErrorMemory(::std::mem::transmute(peaperror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EapHostPeerFreeMemory(pdata: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerFreeMemory(pdata: *mut u8);
        }
        ::std::mem::transmute(EapHostPeerFreeMemory(::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EapHostPeerFreeRuntimeMemory(pdata: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerFreeRuntimeMemory(pdata: *mut u8);
        }
        ::std::mem::transmute(EapHostPeerFreeRuntimeMemory(::std::mem::transmute(pdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetAuthStatus(
    sessionhandle: u32,
    authparam: EapHostPeerAuthParams,
    pcbauthdata: *mut u32,
    ppauthdata: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetAuthStatus(
                sessionhandle: u32,
                authparam: EapHostPeerAuthParams,
                pcbauthdata: *mut u32,
                ppauthdata: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetAuthStatus(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(authparam),
            ::std::mem::transmute(pcbauthdata),
            ::std::mem::transmute(ppauthdata),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetDataToUnplumbCredentials(
    pconnectionidthatlastsavedcreds: *mut ::windows::runtime::GUID,
    phcredentialimpersonationtoken: *mut isize,
    sessionhandle: u32,
    ppeaperror: *mut *mut EAP_ERROR,
    fsavetocredman: *mut super::super::Foundation::BOOL,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetDataToUnplumbCredentials(
                pconnectionidthatlastsavedcreds: *mut ::windows::runtime::GUID,
                phcredentialimpersonationtoken: *mut isize,
                sessionhandle: u32,
                ppeaperror: *mut *mut EAP_ERROR,
                fsavetocredman: *mut super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetDataToUnplumbCredentials(
            ::std::mem::transmute(pconnectionidthatlastsavedcreds),
            ::std::mem::transmute(phcredentialimpersonationtoken),
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(ppeaperror),
            ::std::mem::transmute(fsavetocredman),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetEncryptedPassword<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    dwsizeofpassword: u32,
    szpassword: Param1,
    ppszencpassword: *mut super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetEncryptedPassword(
                dwsizeofpassword: u32,
                szpassword: super::super::Foundation::PWSTR,
                ppszencpassword: *mut super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetEncryptedPassword(
            ::std::mem::transmute(dwsizeofpassword),
            szpassword.into_param().abi(),
            ::std::mem::transmute(ppszencpassword),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetIdentity<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    dwversion: u32,
    dwflags: u32,
    eapmethodtype: Param2,
    dwsizeofconnectiondata: u32,
    pconnectiondata: *const u8,
    dwsizeofuserdata: u32,
    puserdata: *const u8,
    htokenimpersonateuser: Param7,
    pfinvokeui: *mut super::super::Foundation::BOOL,
    pdwsizeofuserdataout: *mut u32,
    ppuserdataout: *mut *mut u8,
    ppwszidentity: *mut super::super::Foundation::PWSTR,
    ppeaperror: *mut *mut EAP_ERROR,
    ppvreserved: *mut *mut u8,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetIdentity(
                dwversion: u32,
                dwflags: u32,
                eapmethodtype: EAP_METHOD_TYPE,
                dwsizeofconnectiondata: u32,
                pconnectiondata: *const u8,
                dwsizeofuserdata: u32,
                puserdata: *const u8,
                htokenimpersonateuser: super::super::Foundation::HANDLE,
                pfinvokeui: *mut super::super::Foundation::BOOL,
                pdwsizeofuserdataout: *mut u32,
                ppuserdataout: *mut *mut u8,
                ppwszidentity: *mut super::super::Foundation::PWSTR,
                ppeaperror: *mut *mut EAP_ERROR,
                ppvreserved: *mut *mut u8,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetIdentity(
            ::std::mem::transmute(dwversion),
            ::std::mem::transmute(dwflags),
            eapmethodtype.into_param().abi(),
            ::std::mem::transmute(dwsizeofconnectiondata),
            ::std::mem::transmute(pconnectiondata),
            ::std::mem::transmute(dwsizeofuserdata),
            ::std::mem::transmute(puserdata),
            htokenimpersonateuser.into_param().abi(),
            ::std::mem::transmute(pfinvokeui),
            ::std::mem::transmute(pdwsizeofuserdataout),
            ::std::mem::transmute(ppuserdataout),
            ::std::mem::transmute(ppwszidentity),
            ::std::mem::transmute(ppeaperror),
            ::std::mem::transmute(ppvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetMethodProperties<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    dwversion: u32,
    dwflags: u32,
    eapmethodtype: Param2,
    huserimpersonationtoken: Param3,
    dweapconndatasize: u32,
    pbeapconndata: *const u8,
    dwuserdatasize: u32,
    pbuserdata: *const u8,
    pmethodpropertyarray: *mut EAP_METHOD_PROPERTY_ARRAY,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerGetMethodProperties(
                dwversion: u32,
                dwflags: u32,
                eapmethodtype: EAP_METHOD_TYPE,
                huserimpersonationtoken: super::super::Foundation::HANDLE,
                dweapconndatasize: u32,
                pbeapconndata: *const u8,
                dwuserdatasize: u32,
                pbuserdata: *const u8,
                pmethodpropertyarray: *mut EAP_METHOD_PROPERTY_ARRAY,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetMethodProperties(
            ::std::mem::transmute(dwversion),
            ::std::mem::transmute(dwflags),
            eapmethodtype.into_param().abi(),
            huserimpersonationtoken.into_param().abi(),
            ::std::mem::transmute(dweapconndatasize),
            ::std::mem::transmute(pbeapconndata),
            ::std::mem::transmute(dwuserdatasize),
            ::std::mem::transmute(pbuserdata),
            ::std::mem::transmute(pmethodpropertyarray),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetMethods(
    peapmethodinfoarray: *mut EAP_METHOD_INFO_ARRAY,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerGetMethods(
                peapmethodinfoarray: *mut EAP_METHOD_INFO_ARRAY,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetMethods(
            ::std::mem::transmute(peapmethodinfoarray),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetResponseAttributes(
    sessionhandle: u32,
    pattribs: *mut EAP_ATTRIBUTES,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetResponseAttributes(
                sessionhandle: u32,
                pattribs: *mut EAP_ATTRIBUTES,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetResponseAttributes(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(pattribs),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetResult(
    sessionhandle: u32,
    reason: EapHostPeerMethodResultReason,
    ppresult: *mut EapHostPeerMethodResult,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetResult(
                sessionhandle: u32,
                reason: EapHostPeerMethodResultReason,
                ppresult: *mut EapHostPeerMethodResult,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetResult(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(reason),
            ::std::mem::transmute(ppresult),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetSendPacket(
    sessionhandle: u32,
    pcbsendpacket: *mut u32,
    ppsendpacket: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetSendPacket(
                sessionhandle: u32,
                pcbsendpacket: *mut u32,
                ppsendpacket: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetSendPacket(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(pcbsendpacket),
            ::std::mem::transmute(ppsendpacket),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerGetUIContext(
    sessionhandle: u32,
    pdwsizeofuicontextdata: *mut u32,
    ppuicontextdata: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerGetUIContext(
                sessionhandle: u32,
                pdwsizeofuicontextdata: *mut u32,
                ppuicontextdata: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerGetUIContext(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(pdwsizeofuicontextdata),
            ::std::mem::transmute(ppuicontextdata),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EapHostPeerInitialize() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerInitialize() -> u32;
        }
        ::std::mem::transmute(EapHostPeerInitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerInvokeConfigUI<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
>(
    hwndparent: Param0,
    dwflags: u32,
    eapmethodtype: Param2,
    dwsizeofconfigin: u32,
    pconfigin: *const u8,
    pdwsizeofconfigout: *mut u32,
    ppconfigout: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerInvokeConfigUI(
                hwndparent: super::super::Foundation::HWND,
                dwflags: u32,
                eapmethodtype: EAP_METHOD_TYPE,
                dwsizeofconfigin: u32,
                pconfigin: *const u8,
                pdwsizeofconfigout: *mut u32,
                ppconfigout: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerInvokeConfigUI(
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            eapmethodtype.into_param().abi(),
            ::std::mem::transmute(dwsizeofconfigin),
            ::std::mem::transmute(pconfigin),
            ::std::mem::transmute(pdwsizeofconfigout),
            ::std::mem::transmute(ppconfigout),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerInvokeIdentityUI<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    dwversion: u32,
    eapmethodtype: Param1,
    dwflags: u32,
    hwndparent: Param3,
    dwsizeofconnectiondata: u32,
    pconnectiondata: *const u8,
    dwsizeofuserdata: u32,
    puserdata: *const u8,
    pdwsizeofuserdataout: *mut u32,
    ppuserdataout: *mut *mut u8,
    ppwszidentity: *mut super::super::Foundation::PWSTR,
    ppeaperror: *mut *mut EAP_ERROR,
    ppvreserved: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerInvokeIdentityUI(
                dwversion: u32,
                eapmethodtype: EAP_METHOD_TYPE,
                dwflags: u32,
                hwndparent: super::super::Foundation::HWND,
                dwsizeofconnectiondata: u32,
                pconnectiondata: *const u8,
                dwsizeofuserdata: u32,
                puserdata: *const u8,
                pdwsizeofuserdataout: *mut u32,
                ppuserdataout: *mut *mut u8,
                ppwszidentity: *mut super::super::Foundation::PWSTR,
                ppeaperror: *mut *mut EAP_ERROR,
                ppvreserved: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerInvokeIdentityUI(
            ::std::mem::transmute(dwversion),
            eapmethodtype.into_param().abi(),
            ::std::mem::transmute(dwflags),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwsizeofconnectiondata),
            ::std::mem::transmute(pconnectiondata),
            ::std::mem::transmute(dwsizeofuserdata),
            ::std::mem::transmute(puserdata),
            ::std::mem::transmute(pdwsizeofuserdataout),
            ::std::mem::transmute(ppuserdataout),
            ::std::mem::transmute(ppwszidentity),
            ::std::mem::transmute(ppeaperror),
            ::std::mem::transmute(ppvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerInvokeInteractiveUI<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndparent: Param0,
    dwsizeofuicontextdata: u32,
    puicontextdata: *const u8,
    pdwsizeofdatafrominteractiveui: *mut u32,
    ppdatafrominteractiveui: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerInvokeInteractiveUI(
                hwndparent: super::super::Foundation::HWND,
                dwsizeofuicontextdata: u32,
                puicontextdata: *const u8,
                pdwsizeofdatafrominteractiveui: *mut u32,
                ppdatafrominteractiveui: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerInvokeInteractiveUI(
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwsizeofuicontextdata),
            ::std::mem::transmute(puicontextdata),
            ::std::mem::transmute(pdwsizeofdatafrominteractiveui),
            ::std::mem::transmute(ppdatafrominteractiveui),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapHostPeerMethodResult {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReasonCode: u32,
    pub fSaveConnectionData: super::super::Foundation::BOOL,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub fSaveUserData: super::super::Foundation::BOOL,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub pAttribArray: *mut EAP_ATTRIBUTES,
    pub isolationState: ISOLATION_STATE,
    pub pEapMethodInfo: *mut EAP_METHOD_INFO,
    pub pEapError: *mut EAP_ERROR,
}
#[cfg(feature = "Win32_Foundation")]
impl EapHostPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapHostPeerMethodResult {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EapHostPeerMethodResult {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapHostPeerMethodResult")
            .field("fIsSuccess", &self.fIsSuccess)
            .field("dwFailureReasonCode", &self.dwFailureReasonCode)
            .field("fSaveConnectionData", &self.fSaveConnectionData)
            .field("dwSizeofConnectionData", &self.dwSizeofConnectionData)
            .field("pConnectionData", &self.pConnectionData)
            .field("fSaveUserData", &self.fSaveUserData)
            .field("dwSizeofUserData", &self.dwSizeofUserData)
            .field("pUserData", &self.pUserData)
            .field("pAttribArray", &self.pAttribArray)
            .field("isolationState", &self.isolationState)
            .field("pEapMethodInfo", &self.pEapMethodInfo)
            .field("pEapError", &self.pEapError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapHostPeerMethodResult {
    fn eq(&self, other: &Self) -> bool {
        self.fIsSuccess == other.fIsSuccess
            && self.dwFailureReasonCode == other.dwFailureReasonCode
            && self.fSaveConnectionData == other.fSaveConnectionData
            && self.dwSizeofConnectionData == other.dwSizeofConnectionData
            && self.pConnectionData == other.pConnectionData
            && self.fSaveUserData == other.fSaveUserData
            && self.dwSizeofUserData == other.dwSizeofUserData
            && self.pUserData == other.pUserData
            && self.pAttribArray == other.pAttribArray
            && self.isolationState == other.isolationState
            && self.pEapMethodInfo == other.pEapMethodInfo
            && self.pEapError == other.pEapError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapHostPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapHostPeerMethodResult {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapHostPeerMethodResultReason(pub i32);
pub const EapHostPeerMethodResultAltSuccessReceived: EapHostPeerMethodResultReason =
    EapHostPeerMethodResultReason(1i32);
pub const EapHostPeerMethodResultTimeout: EapHostPeerMethodResultReason =
    EapHostPeerMethodResultReason(2i32);
pub const EapHostPeerMethodResultFromMethod: EapHostPeerMethodResultReason =
    EapHostPeerMethodResultReason(3i32);
impl ::std::convert::From<i32> for EapHostPeerMethodResultReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapHostPeerMethodResultReason {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerProcessReceivedPacket(
    sessionhandle: u32,
    cbreceivepacket: u32,
    preceivepacket: *const u8,
    peapoutput: *mut EapHostPeerResponseAction,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerProcessReceivedPacket(
                sessionhandle: u32,
                cbreceivepacket: u32,
                preceivepacket: *const u8,
                peapoutput: *mut EapHostPeerResponseAction,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerProcessReceivedPacket(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(cbreceivepacket),
            ::std::mem::transmute(preceivepacket),
            ::std::mem::transmute(peapoutput),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerQueryCredentialInputFields<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
>(
    huserimpersonationtoken: Param0,
    eapmethodtype: Param1,
    dwflags: u32,
    dweapconndatasize: u32,
    pbeapconndata: *const u8,
    peapconfiginputfieldarray: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerQueryCredentialInputFields(
                huserimpersonationtoken: super::super::Foundation::HANDLE,
                eapmethodtype: EAP_METHOD_TYPE,
                dwflags: u32,
                dweapconndatasize: u32,
                pbeapconndata: *const u8,
                peapconfiginputfieldarray: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerQueryCredentialInputFields(
            huserimpersonationtoken.into_param().abi(),
            eapmethodtype.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dweapconndatasize),
            ::std::mem::transmute(pbeapconndata),
            ::std::mem::transmute(peapconfiginputfieldarray),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerQueryInteractiveUIInputFields(
    dwversion: u32,
    dwflags: u32,
    dwsizeofuicontextdata: u32,
    puicontextdata: *const u8,
    peapinteractiveuidata: *mut EAP_INTERACTIVE_UI_DATA,
    ppeaperror: *mut *mut EAP_ERROR,
    ppvreserved: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerQueryInteractiveUIInputFields(
                dwversion: u32,
                dwflags: u32,
                dwsizeofuicontextdata: u32,
                puicontextdata: *const u8,
                peapinteractiveuidata: *mut EAP_INTERACTIVE_UI_DATA,
                ppeaperror: *mut *mut EAP_ERROR,
                ppvreserved: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerQueryInteractiveUIInputFields(
            ::std::mem::transmute(dwversion),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwsizeofuicontextdata),
            ::std::mem::transmute(puicontextdata),
            ::std::mem::transmute(peapinteractiveuidata),
            ::std::mem::transmute(ppeaperror),
            ::std::mem::transmute(ppvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(
    dwversion: u32,
    dwflags: u32,
    dwsizeofuicontextdata: u32,
    puicontextdata: *const u8,
    peapinteractiveuidata: *const EAP_INTERACTIVE_UI_DATA,
    pdwsizeofdatafrominteractiveui: *mut u32,
    ppdatafrominteractiveui: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
    ppvreserved: *mut *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(
                dwversion: u32,
                dwflags: u32,
                dwsizeofuicontextdata: u32,
                puicontextdata: *const u8,
                peapinteractiveuidata: *const EAP_INTERACTIVE_UI_DATA,
                pdwsizeofdatafrominteractiveui: *mut u32,
                ppdatafrominteractiveui: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
                ppvreserved: *mut *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerQueryUIBlobFromInteractiveUIInputFields(
            ::std::mem::transmute(dwversion),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwsizeofuicontextdata),
            ::std::mem::transmute(puicontextdata),
            ::std::mem::transmute(peapinteractiveuidata),
            ::std::mem::transmute(pdwsizeofdatafrominteractiveui),
            ::std::mem::transmute(ppdatafrominteractiveui),
            ::std::mem::transmute(ppeaperror),
            ::std::mem::transmute(ppvreserved),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerQueryUserBlobFromCredentialInputFields<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, EAP_METHOD_TYPE>,
>(
    huserimpersonationtoken: Param0,
    eapmethodtype: Param1,
    dwflags: u32,
    dweapconndatasize: u32,
    pbeapconndata: *const u8,
    peapconfiginputfieldarray: *const EAP_CONFIG_INPUT_FIELD_ARRAY,
    pdwuserblobsize: *mut u32,
    ppbuserblob: *mut *mut u8,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappcfg")]
        extern "system" {
            fn EapHostPeerQueryUserBlobFromCredentialInputFields(
                huserimpersonationtoken: super::super::Foundation::HANDLE,
                eapmethodtype: EAP_METHOD_TYPE,
                dwflags: u32,
                dweapconndatasize: u32,
                pbeapconndata: *const u8,
                peapconfiginputfieldarray: *const EAP_CONFIG_INPUT_FIELD_ARRAY,
                pdwuserblobsize: *mut u32,
                ppbuserblob: *mut *mut u8,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerQueryUserBlobFromCredentialInputFields(
            huserimpersonationtoken.into_param().abi(),
            eapmethodtype.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dweapconndatasize),
            ::std::mem::transmute(pbeapconndata),
            ::std::mem::transmute(peapconfiginputfieldarray),
            ::std::mem::transmute(pdwuserblobsize),
            ::std::mem::transmute(ppbuserblob),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapHostPeerResponseAction(pub i32);
pub const EapHostPeerResponseDiscard: EapHostPeerResponseAction = EapHostPeerResponseAction(0i32);
pub const EapHostPeerResponseSend: EapHostPeerResponseAction = EapHostPeerResponseAction(1i32);
pub const EapHostPeerResponseResult: EapHostPeerResponseAction = EapHostPeerResponseAction(2i32);
pub const EapHostPeerResponseInvokeUi: EapHostPeerResponseAction = EapHostPeerResponseAction(3i32);
pub const EapHostPeerResponseRespond: EapHostPeerResponseAction = EapHostPeerResponseAction(4i32);
pub const EapHostPeerResponseStartAuthentication: EapHostPeerResponseAction =
    EapHostPeerResponseAction(5i32);
pub const EapHostPeerResponseNone: EapHostPeerResponseAction = EapHostPeerResponseAction(6i32);
impl ::std::convert::From<i32> for EapHostPeerResponseAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapHostPeerResponseAction {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerSetResponseAttributes(
    sessionhandle: u32,
    pattribs: *const EAP_ATTRIBUTES,
    peapoutput: *mut EapHostPeerResponseAction,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerSetResponseAttributes(
                sessionhandle: u32,
                pattribs: *const EAP_ATTRIBUTES,
                peapoutput: *mut EapHostPeerResponseAction,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerSetResponseAttributes(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(pattribs),
            ::std::mem::transmute(peapoutput),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn EapHostPeerSetUIContext(
    sessionhandle: u32,
    dwsizeofuicontextdata: u32,
    puicontextdata: *const u8,
    peapoutput: *mut EapHostPeerResponseAction,
    ppeaperror: *mut *mut EAP_ERROR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerSetUIContext(
                sessionhandle: u32,
                dwsizeofuicontextdata: u32,
                puicontextdata: *const u8,
                peapoutput: *mut EapHostPeerResponseAction,
                ppeaperror: *mut *mut EAP_ERROR,
            ) -> u32;
        }
        ::std::mem::transmute(EapHostPeerSetUIContext(
            ::std::mem::transmute(sessionhandle),
            ::std::mem::transmute(dwsizeofuicontextdata),
            ::std::mem::transmute(puicontextdata),
            ::std::mem::transmute(peapoutput),
            ::std::mem::transmute(ppeaperror),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn EapHostPeerUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "eappprxy")]
        extern "system" {
            fn EapHostPeerUninitialize();
        }
        ::std::mem::transmute(EapHostPeerUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EapPacket {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl EapPacket {}
impl ::std::default::Default for EapPacket {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EapPacket {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapPacket")
            .field("Code", &self.Code)
            .field("Id", &self.Id)
            .field("Length", &self.Length)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EapPacket {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code
            && self.Id == other.Id
            && self.Length == other.Length
            && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for EapPacket {}
unsafe impl ::windows::runtime::Abi for EapPacket {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodOutput {
    pub action: EapPeerMethodResponseAction,
    pub fAllowNotifications: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl EapPeerMethodOutput {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapPeerMethodOutput {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EapPeerMethodOutput {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapPeerMethodOutput")
            .field("action", &self.action)
            .field("fAllowNotifications", &self.fAllowNotifications)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapPeerMethodOutput {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.fAllowNotifications == other.fAllowNotifications
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapPeerMethodOutput {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapPeerMethodOutput {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapPeerMethodResponseAction(pub i32);
pub const EapPeerMethodResponseActionDiscard: EapPeerMethodResponseAction =
    EapPeerMethodResponseAction(0i32);
pub const EapPeerMethodResponseActionSend: EapPeerMethodResponseAction =
    EapPeerMethodResponseAction(1i32);
pub const EapPeerMethodResponseActionResult: EapPeerMethodResponseAction =
    EapPeerMethodResponseAction(2i32);
pub const EapPeerMethodResponseActionInvokeUI: EapPeerMethodResponseAction =
    EapPeerMethodResponseAction(3i32);
pub const EapPeerMethodResponseActionRespond: EapPeerMethodResponseAction =
    EapPeerMethodResponseAction(4i32);
pub const EapPeerMethodResponseActionNone: EapPeerMethodResponseAction =
    EapPeerMethodResponseAction(5i32);
impl ::std::convert::From<i32> for EapPeerMethodResponseAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapPeerMethodResponseAction {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodResult {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReasonCode: u32,
    pub fSaveConnectionData: super::super::Foundation::BOOL,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub fSaveUserData: super::super::Foundation::BOOL,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub pAttribArray: *mut EAP_ATTRIBUTES,
    pub pEapError: *mut EAP_ERROR,
    pub pNgcKerbTicket: *mut NgcTicketContext,
    pub fSaveToCredMan: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl EapPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapPeerMethodResult {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EapPeerMethodResult {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapPeerMethodResult")
            .field("fIsSuccess", &self.fIsSuccess)
            .field("dwFailureReasonCode", &self.dwFailureReasonCode)
            .field("fSaveConnectionData", &self.fSaveConnectionData)
            .field("dwSizeofConnectionData", &self.dwSizeofConnectionData)
            .field("pConnectionData", &self.pConnectionData)
            .field("fSaveUserData", &self.fSaveUserData)
            .field("dwSizeofUserData", &self.dwSizeofUserData)
            .field("pUserData", &self.pUserData)
            .field("pAttribArray", &self.pAttribArray)
            .field("pEapError", &self.pEapError)
            .field("pNgcKerbTicket", &self.pNgcKerbTicket)
            .field("fSaveToCredMan", &self.fSaveToCredMan)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapPeerMethodResult {
    fn eq(&self, other: &Self) -> bool {
        self.fIsSuccess == other.fIsSuccess
            && self.dwFailureReasonCode == other.dwFailureReasonCode
            && self.fSaveConnectionData == other.fSaveConnectionData
            && self.dwSizeofConnectionData == other.dwSizeofConnectionData
            && self.pConnectionData == other.pConnectionData
            && self.fSaveUserData == other.fSaveUserData
            && self.dwSizeofUserData == other.dwSizeofUserData
            && self.pUserData == other.pUserData
            && self.pAttribArray == other.pAttribArray
            && self.pEapError == other.pEapError
            && self.pNgcKerbTicket == other.pNgcKerbTicket
            && self.fSaveToCredMan == other.fSaveToCredMan
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapPeerMethodResult {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EapPeerMethodResultReason(pub i32);
pub const EapPeerMethodResultUnknown: EapPeerMethodResultReason = EapPeerMethodResultReason(1i32);
pub const EapPeerMethodResultSuccess: EapPeerMethodResultReason = EapPeerMethodResultReason(2i32);
pub const EapPeerMethodResultFailure: EapPeerMethodResultReason = EapPeerMethodResultReason(3i32);
impl ::std::convert::From<i32> for EapPeerMethodResultReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EapPeerMethodResultReason {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapSimCredential {
    pub iccID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl EapSimCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapSimCredential {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EapSimCredential {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapSimCredential")
            .field("iccID", &self.iccID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapSimCredential {
    fn eq(&self, other: &Self) -> bool {
        self.iccID == other.iccID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapSimCredential {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapSimCredential {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EapUsernamePasswordCredential {
    pub username: super::super::Foundation::PWSTR,
    pub password: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl EapUsernamePasswordCredential {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for EapUsernamePasswordCredential {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for EapUsernamePasswordCredential {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EapUsernamePasswordCredential")
            .field("username", &self.username)
            .field("password", &self.password)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for EapUsernamePasswordCredential {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.password == other.password
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for EapUsernamePasswordCredential {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for EapUsernamePasswordCredential {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FACILITY_EAP_MESSAGE: u32 = 2114u32;
pub const GUID_EapHost_Cause_CertStoreInaccessible: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 4],
    );
pub const GUID_EapHost_Cause_EapNegotiationFailed: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 28],
    );
pub const GUID_EapHost_Cause_EapQecInaccessible: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 3, 18],
    );
pub const GUID_EapHost_Cause_Generic_AuthFailure: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 1, 4],
    );
pub const GUID_EapHost_Cause_IdentityUnknown: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 2, 4],
    );
pub const GUID_EapHost_Cause_MethodDLLNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 1],
    );
pub const GUID_EapHost_Cause_MethodDoesNotSupportOperation: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 30],
    );
pub const GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3659054386,
        79,
        16890,
        [174, 8, 11, 200, 94, 88, 69, 172],
    );
pub const GUID_EapHost_Cause_No_SmartCardReader_Found: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 43],
    );
pub const GUID_EapHost_Cause_Server_CertExpired: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 5],
    );
pub const GUID_EapHost_Cause_Server_CertInvalid: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 6],
    );
pub const GUID_EapHost_Cause_Server_CertNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 7],
    );
pub const GUID_EapHost_Cause_Server_CertOtherError: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 1, 8],
    );
pub const GUID_EapHost_Cause_Server_CertRevoked: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 8],
    );
pub const GUID_EapHost_Cause_Server_Root_CertNameRequired: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 18],
    );
pub const GUID_EapHost_Cause_Server_Root_CertNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 1, 18],
    );
pub const GUID_EapHost_Cause_SimNotValid: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 3, 4],
    );
pub const GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 2, 18],
    );
pub const GUID_EapHost_Cause_User_Account_OtherProblem: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 1, 14],
    );
pub const GUID_EapHost_Cause_User_CertExpired: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 9],
    );
pub const GUID_EapHost_Cause_User_CertInvalid: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 10],
    );
pub const GUID_EapHost_Cause_User_CertNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 11],
    );
pub const GUID_EapHost_Cause_User_CertOtherError: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 12],
    );
pub const GUID_EapHost_Cause_User_CertRejected: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 13],
    );
pub const GUID_EapHost_Cause_User_CertRevoked: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 14],
    );
pub const GUID_EapHost_Cause_User_CredsRejected: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 2, 14],
    );
pub const GUID_EapHost_Cause_User_Root_CertExpired: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 15],
    );
pub const GUID_EapHost_Cause_User_Root_CertInvalid: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 16],
    );
pub const GUID_EapHost_Cause_User_Root_CertNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 17],
    );
pub const GUID_EapHost_Cause_XmlMalformed: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 29],
    );
pub const GUID_EapHost_Default: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
pub const GUID_EapHost_Help_ObtainingCerts: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        4113952419,
        7133,
        18122,
        [162, 252, 166, 101, 89, 57, 183, 232],
    );
pub const GUID_EapHost_Help_Troubleshooting: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        858815183,
        1688,
        16826,
        [176, 20, 234, 10, 46, 184, 208, 168],
    );
pub const GUID_EapHost_Repair_ContactAdmin_AuthFailure: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 31],
    );
pub const GUID_EapHost_Repair_ContactAdmin_CertNameAbsent: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 41],
    );
pub const GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 36],
    );
pub const GUID_EapHost_Repair_ContactAdmin_IdentityUnknown: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 32],
    );
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 37],
    );
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserCert: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 44],
    );
pub const GUID_EapHost_Repair_ContactAdmin_MethodNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 34],
    );
pub const GUID_EapHost_Repair_ContactAdmin_NegotiationFailed: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 33],
    );
pub const GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 42],
    );
pub const GUID_EapHost_Repair_ContactAdmin_RootCertInvalid: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 38],
    );
pub const GUID_EapHost_Repair_ContactAdmin_RootCertNotFound: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 39],
    );
pub const GUID_EapHost_Repair_ContactAdmin_RootExpired: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 40],
    );
pub const GUID_EapHost_Repair_ContactSysadmin: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 2],
    );
pub const GUID_EapHost_Repair_Method_Not_Support_Sso: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 45],
    );
pub const GUID_EapHost_Repair_No_ValidSim_Found: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 46],
    );
pub const GUID_EapHost_Repair_RestartNap: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 35],
    );
pub const GUID_EapHost_Repair_Retry_Authentication: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 1, 27],
    );
pub const GUID_EapHost_Repair_Server_ClientSelectServerCert: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 24],
    );
pub const GUID_EapHost_Repair_User_AuthFailure: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 25],
    );
pub const GUID_EapHost_Repair_User_GetNewCert: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 26],
    );
pub const GUID_EapHost_Repair_User_SelectValidCert: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2517826663,
        24912,
        16905,
        [168, 94, 168, 216, 0, 0, 0, 27],
    );
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAccountingProviderConfig(::windows::runtime::IUnknown);
impl IAccountingProviderConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmachinename: Param0,
    ) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            &mut result__,
        )
        .from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, uconnectionparam: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configure<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        uconnectionparam: usize,
        hwnd: Param1,
        dwflags: u32,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    pub unsafe fn Activate(
        &self,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    pub unsafe fn Deactivate(
        &self,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAccountingProviderConfig {
    type Vtable = IAccountingProviderConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1721948952,
        55046,
        4560,
        [163, 123, 0, 192, 79, 201, 218, 4],
    );
}
impl ::std::convert::From<IAccountingProviderConfig> for ::windows::runtime::IUnknown {
    fn from(value: IAccountingProviderConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAccountingProviderConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IAccountingProviderConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IAccountingProviderConfig
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IAccountingProviderConfig
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountingProviderConfig_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        puconnectionparam: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        dwflags: u32,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IAuthenticationProviderConfig(::windows::runtime::IUnknown);
impl IAuthenticationProviderConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmachinename: Param0,
    ) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            &mut result__,
        )
        .from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, uconnectionparam: usize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configure<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        uconnectionparam: usize,
        hwnd: Param1,
        dwflags: u32,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    pub unsafe fn Activate(
        &self,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    pub unsafe fn Deactivate(
        &self,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(uconnectionparam),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAuthenticationProviderConfig {
    type Vtable = IAuthenticationProviderConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1721948951,
        55046,
        4560,
        [163, 123, 0, 192, 79, 201, 218, 4],
    );
}
impl ::std::convert::From<IAuthenticationProviderConfig> for ::windows::runtime::IUnknown {
    fn from(value: IAuthenticationProviderConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IAuthenticationProviderConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IAuthenticationProviderConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IAuthenticationProviderConfig
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IAuthenticationProviderConfig
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticationProviderConfig_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        puconnectionparam: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        dwflags: u32,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uconnectionparam: usize,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEAPProviderConfig(::windows::runtime::IUnknown);
impl IEAPProviderConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmachinename: Param0,
        dweaptypeid: u32,
    ) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            ::std::mem::transmute(dweaptypeid),
            &mut result__,
        )
        .from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: Param2,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: Param2,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        ppconnectiondataout: *mut *mut u8,
        pdwsizeofconnectiondataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pconnectiondatain),
            ::std::mem::transmute(dwsizeofconnectiondatain),
            ::std::mem::transmute(ppconnectiondataout),
            ::std::mem::transmute(pdwsizeofconnectiondataout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: Param2,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        puserdatain: *const u8,
        dwsizeofuserdatain: u32,
        ppuserdataout: *mut *mut u8,
        pdwsizeofuserdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pconnectiondatain),
            ::std::mem::transmute(dwsizeofconnectiondatain),
            ::std::mem::transmute(puserdatain),
            ::std::mem::transmute(dwsizeofuserdatain),
            ::std::mem::transmute(ppuserdataout),
            ::std::mem::transmute(pdwsizeofuserdataout),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEAPProviderConfig {
    type Vtable = IEAPProviderConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1721948953,
        55046,
        4560,
        [163, 123, 0, 192, 79, 201, 218, 4],
    );
}
impl ::std::convert::From<IEAPProviderConfig> for ::windows::runtime::IUnknown {
    fn from(value: IEAPProviderConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEAPProviderConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IEAPProviderConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEAPProviderConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEAPProviderConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEAPProviderConfig_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        dweaptypeid: u32,
        puconnectionparam: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: super::super::Foundation::HWND,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        ppconnectiondataout: *mut *mut u8,
        pdwsizeofconnectiondataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: super::super::Foundation::HWND,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        puserdatain: *const u8,
        dwsizeofuserdatain: u32,
        ppuserdataout: *mut *mut u8,
        pdwsizeofuserdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEAPProviderConfig2(::windows::runtime::IUnknown);
impl IEAPProviderConfig2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmachinename: Param0,
        dweaptypeid: u32,
    ) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            ::std::mem::transmute(dweaptypeid),
            &mut result__,
        )
        .from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: Param2,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: Param2,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        ppconnectiondataout: *mut *mut u8,
        pdwsizeofconnectiondataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pconnectiondatain),
            ::std::mem::transmute(dwsizeofconnectiondatain),
            ::std::mem::transmute(ppconnectiondataout),
            ::std::mem::transmute(pdwsizeofconnectiondataout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: Param2,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        puserdatain: *const u8,
        dwsizeofuserdatain: u32,
        ppuserdataout: *mut *mut u8,
        pdwsizeofuserdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pconnectiondatain),
            ::std::mem::transmute(dwsizeofconnectiondatain),
            ::std::mem::transmute(puserdatain),
            ::std::mem::transmute(dwsizeofuserdatain),
            ::std::mem::transmute(ppuserdataout),
            ::std::mem::transmute(pdwsizeofuserdataout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI2<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: Param2,
        pconfigdatain: *const u8,
        dwsizeofconfigdatain: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pconfigdatain),
            ::std::mem::transmute(dwsizeofconfigdatain),
            ::std::mem::transmute(ppconfigdataout),
            ::std::mem::transmute(pdwsizeofconfigdataout),
        )
        .ok()
    }
    pub unsafe fn GetGlobalConfig(
        &self,
        dweaptypeid: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(ppconfigdataout),
            ::std::mem::transmute(pdwsizeofconfigdataout),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEAPProviderConfig2 {
    type Vtable = IEAPProviderConfig2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3580203386,
        34244,
        17510,
        [133, 110, 103, 28, 55, 66, 234, 154],
    );
}
impl ::std::convert::From<IEAPProviderConfig2> for ::windows::runtime::IUnknown {
    fn from(value: IEAPProviderConfig2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEAPProviderConfig2> for ::windows::runtime::IUnknown {
    fn from(value: &IEAPProviderConfig2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEAPProviderConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEAPProviderConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IEAPProviderConfig2> for IEAPProviderConfig {
    fn from(value: IEAPProviderConfig2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEAPProviderConfig2> for IEAPProviderConfig {
    fn from(value: &IEAPProviderConfig2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEAPProviderConfig> for IEAPProviderConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEAPProviderConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IEAPProviderConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEAPProviderConfig> for &IEAPProviderConfig2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEAPProviderConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IEAPProviderConfig>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEAPProviderConfig2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        dweaptypeid: u32,
        puconnectionparam: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: super::super::Foundation::HWND,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        ppconnectiondataout: *mut *mut u8,
        pdwsizeofconnectiondataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: super::super::Foundation::HWND,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        puserdatain: *const u8,
        dwsizeofuserdatain: u32,
        ppuserdataout: *mut *mut u8,
        pdwsizeofuserdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        pconfigdatain: *const u8,
        dwsizeofconfigdatain: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IEAPProviderConfig3(::windows::runtime::IUnknown);
impl IEAPProviderConfig3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        pszmachinename: Param0,
        dweaptypeid: u32,
    ) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            ::std::mem::transmute(dweaptypeid),
            &mut result__,
        )
        .from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: Param2,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(ureserved1),
            ::std::mem::transmute(ureserved2),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: Param2,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        ppconnectiondataout: *mut *mut u8,
        pdwsizeofconnectiondataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pconnectiondatain),
            ::std::mem::transmute(dwsizeofconnectiondatain),
            ::std::mem::transmute(ppconnectiondataout),
            ::std::mem::transmute(pdwsizeofconnectiondataout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: Param2,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        puserdatain: *const u8,
        dwsizeofuserdatain: u32,
        ppuserdataout: *mut *mut u8,
        pdwsizeofuserdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwndparent.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(pconnectiondatain),
            ::std::mem::transmute(dwsizeofconnectiondatain),
            ::std::mem::transmute(puserdatain),
            ::std::mem::transmute(dwsizeofuserdatain),
            ::std::mem::transmute(ppuserdataout),
            ::std::mem::transmute(pdwsizeofuserdataout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI2<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: Param2,
        pconfigdatain: *const u8,
        dwsizeofconfigdatain: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pconfigdatain),
            ::std::mem::transmute(dwsizeofconfigdatain),
            ::std::mem::transmute(ppconfigdataout),
            ::std::mem::transmute(pdwsizeofconfigdataout),
        )
        .ok()
    }
    pub unsafe fn GetGlobalConfig(
        &self,
        dweaptypeid: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(ppconfigdataout),
            ::std::mem::transmute(pdwsizeofconfigdataout),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeCertificateConfigUI<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: Param2,
        pconfigdatain: *const u8,
        dwsizeofconfigdatain: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
        ureserved: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(dweaptypeid),
            ::std::mem::transmute(uconnectionparam),
            hwnd.into_param().abi(),
            ::std::mem::transmute(pconfigdatain),
            ::std::mem::transmute(dwsizeofconfigdatain),
            ::std::mem::transmute(ppconfigdataout),
            ::std::mem::transmute(pdwsizeofconfigdataout),
            ::std::mem::transmute(ureserved),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEAPProviderConfig3 {
    type Vtable = IEAPProviderConfig3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3079589138,
        26811,
        20358,
        [155, 240, 132, 56, 221, 59, 233, 130],
    );
}
impl ::std::convert::From<IEAPProviderConfig3> for ::windows::runtime::IUnknown {
    fn from(value: IEAPProviderConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEAPProviderConfig3> for ::windows::runtime::IUnknown {
    fn from(value: &IEAPProviderConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEAPProviderConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEAPProviderConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IEAPProviderConfig3> for IEAPProviderConfig2 {
    fn from(value: IEAPProviderConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEAPProviderConfig3> for IEAPProviderConfig2 {
    fn from(value: &IEAPProviderConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEAPProviderConfig2> for IEAPProviderConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEAPProviderConfig2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IEAPProviderConfig2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEAPProviderConfig2> for &IEAPProviderConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEAPProviderConfig2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IEAPProviderConfig2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IEAPProviderConfig3> for IEAPProviderConfig {
    fn from(value: IEAPProviderConfig3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEAPProviderConfig3> for IEAPProviderConfig {
    fn from(value: &IEAPProviderConfig3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEAPProviderConfig> for IEAPProviderConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEAPProviderConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IEAPProviderConfig>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IEAPProviderConfig> for &IEAPProviderConfig3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IEAPProviderConfig> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IEAPProviderConfig>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEAPProviderConfig3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        dweaptypeid: u32,
        puconnectionparam: *mut usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        ureserved1: usize,
        ureserved2: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: super::super::Foundation::HWND,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        ppconnectiondataout: *mut *mut u8,
        pdwsizeofconnectiondataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwndparent: super::super::Foundation::HWND,
        dwflags: u32,
        pconnectiondatain: *const u8,
        dwsizeofconnectiondatain: u32,
        puserdatain: *const u8,
        dwsizeofuserdatain: u32,
        ppuserdataout: *mut *mut u8,
        pdwsizeofuserdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        pconfigdatain: *const u8,
        dwsizeofconfigdatain: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dweaptypeid: u32,
        uconnectionparam: usize,
        hwnd: super::super::Foundation::HWND,
        pconfigdatain: *const u8,
        dwsizeofconfigdatain: u32,
        ppconfigdataout: *mut *mut u8,
        pdwsizeofconfigdataout: *mut u32,
        ureserved: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRouterProtocolConfig(::windows::runtime::IUnknown);
impl IRouterProtocolConfig {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddProtocol<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pszmachinename: Param0,
        dwtransportid: u32,
        dwprotocolid: u32,
        hwnd: Param3,
        dwflags: u32,
        prouter: Param5,
        ureserved1: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            ::std::mem::transmute(dwtransportid),
            ::std::mem::transmute(dwprotocolid),
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwflags),
            prouter.into_param().abi(),
            ::std::mem::transmute(ureserved1),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveProtocol<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param5: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        pszmachinename: Param0,
        dwtransportid: u32,
        dwprotocolid: u32,
        hwnd: Param3,
        dwflags: u32,
        prouter: Param5,
        ureserved1: usize,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            pszmachinename.into_param().abi(),
            ::std::mem::transmute(dwtransportid),
            ::std::mem::transmute(dwprotocolid),
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwflags),
            prouter.into_param().abi(),
            ::std::mem::transmute(ureserved1),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRouterProtocolConfig {
    type Vtable = IRouterProtocolConfig_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1721948950,
        55046,
        4560,
        [163, 123, 0, 192, 79, 201, 218, 4],
    );
}
impl ::std::convert::From<IRouterProtocolConfig> for ::windows::runtime::IUnknown {
    fn from(value: IRouterProtocolConfig) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRouterProtocolConfig> for ::windows::runtime::IUnknown {
    fn from(value: &IRouterProtocolConfig) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRouterProtocolConfig {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IRouterProtocolConfig
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRouterProtocolConfig_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        dwtransportid: u32,
        dwprotocolid: u32,
        hwnd: super::super::Foundation::HWND,
        dwflags: u32,
        prouter: ::windows::runtime::RawPtr,
        ureserved1: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pszmachinename: super::super::Foundation::PWSTR,
        dwtransportid: u32,
        dwprotocolid: u32,
        hwnd: super::super::Foundation::HWND,
        dwflags: u32,
        prouter: ::windows::runtime::RawPtr,
        ureserved1: usize,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ISOLATION_STATE(pub i32);
pub const ISOLATION_STATE_UNKNOWN: ISOLATION_STATE = ISOLATION_STATE(0i32);
pub const ISOLATION_STATE_NOT_RESTRICTED: ISOLATION_STATE = ISOLATION_STATE(1i32);
pub const ISOLATION_STATE_IN_PROBATION: ISOLATION_STATE = ISOLATION_STATE(2i32);
pub const ISOLATION_STATE_RESTRICTED_ACCESS: ISOLATION_STATE = ISOLATION_STATE(3i32);
impl ::std::convert::From<i32> for ISOLATION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ISOLATION_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LEGACY_IDENTITY_UI_PARAMS {
    pub eapType: u32,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: super::super::Foundation::PWSTR,
    pub dwError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl LEGACY_IDENTITY_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LEGACY_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LEGACY_IDENTITY_UI_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LEGACY_IDENTITY_UI_PARAMS")
            .field("eapType", &self.eapType)
            .field("dwFlags", &self.dwFlags)
            .field("dwSizeofConnectionData", &self.dwSizeofConnectionData)
            .field("pConnectionData", &self.pConnectionData)
            .field("dwSizeofUserData", &self.dwSizeofUserData)
            .field("pUserData", &self.pUserData)
            .field("dwSizeofUserDataOut", &self.dwSizeofUserDataOut)
            .field("pUserDataOut", &self.pUserDataOut)
            .field("pwszIdentity", &self.pwszIdentity)
            .field("dwError", &self.dwError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LEGACY_IDENTITY_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.eapType == other.eapType
            && self.dwFlags == other.dwFlags
            && self.dwSizeofConnectionData == other.dwSizeofConnectionData
            && self.pConnectionData == other.pConnectionData
            && self.dwSizeofUserData == other.dwSizeofUserData
            && self.pUserData == other.pUserData
            && self.dwSizeofUserDataOut == other.dwSizeofUserDataOut
            && self.pUserDataOut == other.pUserDataOut
            && self.pwszIdentity == other.pwszIdentity
            && self.dwError == other.dwError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LEGACY_IDENTITY_UI_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LEGACY_IDENTITY_UI_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LEGACY_INTERACTIVE_UI_PARAMS {
    pub eapType: u32,
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
}
impl LEGACY_INTERACTIVE_UI_PARAMS {}
impl ::std::default::Default for LEGACY_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LEGACY_INTERACTIVE_UI_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LEGACY_INTERACTIVE_UI_PARAMS")
            .field("eapType", &self.eapType)
            .field("dwSizeofContextData", &self.dwSizeofContextData)
            .field("pContextData", &self.pContextData)
            .field("dwSizeofInteractiveUIData", &self.dwSizeofInteractiveUIData)
            .field("pInteractiveUIData", &self.pInteractiveUIData)
            .field("dwError", &self.dwError)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LEGACY_INTERACTIVE_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.eapType == other.eapType
            && self.dwSizeofContextData == other.dwSizeofContextData
            && self.pContextData == other.pContextData
            && self.dwSizeofInteractiveUIData == other.dwSizeofInteractiveUIData
            && self.pInteractiveUIData == other.pInteractiveUIData
            && self.dwError == other.dwError
    }
}
impl ::std::cmp::Eq for LEGACY_INTERACTIVE_UI_PARAMS {}
unsafe impl ::windows::runtime::Abi for LEGACY_INTERACTIVE_UI_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MAXEAPCODE: u32 = 4u32;
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: u32 = 256u32;
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: u32 = 1024u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NgcTicketContext {
    pub wszTicket: [u16; 45],
    pub hKey: usize,
    pub hImpersonateToken: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl NgcTicketContext {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NgcTicketContext {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NgcTicketContext {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NgcTicketContext")
            .field("wszTicket", &self.wszTicket)
            .field("hKey", &self.hKey)
            .field("hImpersonateToken", &self.hImpersonateToken)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NgcTicketContext {
    fn eq(&self, other: &Self) -> bool {
        self.wszTicket == other.wszTicket
            && self.hKey == other.hKey
            && self.hImpersonateToken == other.hImpersonateToken
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NgcTicketContext {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NgcTicketContext {
    type Abi = Self;
    type DefaultType = Self;
}
pub type NotificationHandler = unsafe extern "system" fn(
    connectionid: ::windows::runtime::GUID,
    pcontextdata: *mut ::std::ffi::c_void,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PPP_EAP_ACTION(pub i32);
pub const EAPACTION_NoAction: PPP_EAP_ACTION = PPP_EAP_ACTION(0i32);
pub const EAPACTION_Authenticate: PPP_EAP_ACTION = PPP_EAP_ACTION(1i32);
pub const EAPACTION_Done: PPP_EAP_ACTION = PPP_EAP_ACTION(2i32);
pub const EAPACTION_SendAndDone: PPP_EAP_ACTION = PPP_EAP_ACTION(3i32);
pub const EAPACTION_Send: PPP_EAP_ACTION = PPP_EAP_ACTION(4i32);
pub const EAPACTION_SendWithTimeout: PPP_EAP_ACTION = PPP_EAP_ACTION(5i32);
pub const EAPACTION_SendWithTimeoutInteractive: PPP_EAP_ACTION = PPP_EAP_ACTION(6i32);
pub const EAPACTION_IndicateTLV: PPP_EAP_ACTION = PPP_EAP_ACTION(7i32);
pub const EAPACTION_IndicateIdentity: PPP_EAP_ACTION = PPP_EAP_ACTION(8i32);
impl ::std::convert::From<i32> for PPP_EAP_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PPP_EAP_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_EAP_INFO {
    pub dwSizeInBytes: u32,
    pub dwEapTypeId: u32,
    pub RasEapInitialize: isize,
    pub RasEapBegin: isize,
    pub RasEapEnd: isize,
    pub RasEapMakeMessage: isize,
}
impl PPP_EAP_INFO {}
impl ::std::default::Default for PPP_EAP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_EAP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_EAP_INFO")
            .field("dwSizeInBytes", &self.dwSizeInBytes)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("RasEapInitialize", &self.RasEapInitialize)
            .field("RasEapBegin", &self.RasEapBegin)
            .field("RasEapEnd", &self.RasEapEnd)
            .field("RasEapMakeMessage", &self.RasEapMakeMessage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PPP_EAP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes
            && self.dwEapTypeId == other.dwEapTypeId
            && self.RasEapInitialize == other.RasEapInitialize
            && self.RasEapBegin == other.RasEapBegin
            && self.RasEapEnd == other.RasEapEnd
            && self.RasEapMakeMessage == other.RasEapMakeMessage
    }
}
impl ::std::cmp::Eq for PPP_EAP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_EAP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_INPUT {
    pub dwSizeInBytes: u32,
    pub fFlags: u32,
    pub fAuthenticator: super::super::Foundation::BOOL,
    pub pwszIdentity: super::super::Foundation::PWSTR,
    pub pwszPassword: super::super::Foundation::PWSTR,
    pub bInitialId: u8,
    pub pUserAttributes: *mut RAS_AUTH_ATTRIBUTE,
    pub fAuthenticationComplete: super::super::Foundation::BOOL,
    pub dwAuthResultCode: u32,
    pub hTokenImpersonateUser: super::super::Foundation::HANDLE,
    pub fSuccessPacketReceived: super::super::Foundation::BOOL,
    pub fDataReceivedFromInteractiveUI: super::super::Foundation::BOOL,
    pub pDataFromInteractiveUI: *mut u8,
    pub dwSizeOfDataFromInteractiveUI: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeOfConnectionData: u32,
    pub pUserData: *mut u8,
    pub dwSizeOfUserData: u32,
    pub hReserved: super::super::Foundation::HANDLE,
    pub guidConnectionId: ::windows::runtime::GUID,
    pub isVpn: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl PPP_EAP_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PPP_EAP_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PPP_EAP_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_EAP_INPUT")
            .field("dwSizeInBytes", &self.dwSizeInBytes)
            .field("fFlags", &self.fFlags)
            .field("fAuthenticator", &self.fAuthenticator)
            .field("pwszIdentity", &self.pwszIdentity)
            .field("pwszPassword", &self.pwszPassword)
            .field("bInitialId", &self.bInitialId)
            .field("pUserAttributes", &self.pUserAttributes)
            .field("fAuthenticationComplete", &self.fAuthenticationComplete)
            .field("dwAuthResultCode", &self.dwAuthResultCode)
            .field("hTokenImpersonateUser", &self.hTokenImpersonateUser)
            .field("fSuccessPacketReceived", &self.fSuccessPacketReceived)
            .field(
                "fDataReceivedFromInteractiveUI",
                &self.fDataReceivedFromInteractiveUI,
            )
            .field("pDataFromInteractiveUI", &self.pDataFromInteractiveUI)
            .field(
                "dwSizeOfDataFromInteractiveUI",
                &self.dwSizeOfDataFromInteractiveUI,
            )
            .field("pConnectionData", &self.pConnectionData)
            .field("dwSizeOfConnectionData", &self.dwSizeOfConnectionData)
            .field("pUserData", &self.pUserData)
            .field("dwSizeOfUserData", &self.dwSizeOfUserData)
            .field("hReserved", &self.hReserved)
            .field("guidConnectionId", &self.guidConnectionId)
            .field("isVpn", &self.isVpn)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PPP_EAP_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes
            && self.fFlags == other.fFlags
            && self.fAuthenticator == other.fAuthenticator
            && self.pwszIdentity == other.pwszIdentity
            && self.pwszPassword == other.pwszPassword
            && self.bInitialId == other.bInitialId
            && self.pUserAttributes == other.pUserAttributes
            && self.fAuthenticationComplete == other.fAuthenticationComplete
            && self.dwAuthResultCode == other.dwAuthResultCode
            && self.hTokenImpersonateUser == other.hTokenImpersonateUser
            && self.fSuccessPacketReceived == other.fSuccessPacketReceived
            && self.fDataReceivedFromInteractiveUI == other.fDataReceivedFromInteractiveUI
            && self.pDataFromInteractiveUI == other.pDataFromInteractiveUI
            && self.dwSizeOfDataFromInteractiveUI == other.dwSizeOfDataFromInteractiveUI
            && self.pConnectionData == other.pConnectionData
            && self.dwSizeOfConnectionData == other.dwSizeOfConnectionData
            && self.pUserData == other.pUserData
            && self.dwSizeOfUserData == other.dwSizeOfUserData
            && self.hReserved == other.hReserved
            && self.guidConnectionId == other.guidConnectionId
            && self.isVpn == other.isVpn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PPP_EAP_INPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PPP_EAP_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_OUTPUT {
    pub dwSizeInBytes: u32,
    pub Action: PPP_EAP_ACTION,
    pub dwAuthResultCode: u32,
    pub pUserAttributes: *mut RAS_AUTH_ATTRIBUTE,
    pub fInvokeInteractiveUI: super::super::Foundation::BOOL,
    pub pUIContextData: *mut u8,
    pub dwSizeOfUIContextData: u32,
    pub fSaveConnectionData: super::super::Foundation::BOOL,
    pub pConnectionData: *mut u8,
    pub dwSizeOfConnectionData: u32,
    pub fSaveUserData: super::super::Foundation::BOOL,
    pub pUserData: *mut u8,
    pub dwSizeOfUserData: u32,
    pub pNgcKerbTicket: *mut NgcTicketContext,
    pub fSaveToCredMan: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl PPP_EAP_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PPP_EAP_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PPP_EAP_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_EAP_OUTPUT")
            .field("dwSizeInBytes", &self.dwSizeInBytes)
            .field("Action", &self.Action)
            .field("dwAuthResultCode", &self.dwAuthResultCode)
            .field("pUserAttributes", &self.pUserAttributes)
            .field("fInvokeInteractiveUI", &self.fInvokeInteractiveUI)
            .field("pUIContextData", &self.pUIContextData)
            .field("dwSizeOfUIContextData", &self.dwSizeOfUIContextData)
            .field("fSaveConnectionData", &self.fSaveConnectionData)
            .field("pConnectionData", &self.pConnectionData)
            .field("dwSizeOfConnectionData", &self.dwSizeOfConnectionData)
            .field("fSaveUserData", &self.fSaveUserData)
            .field("pUserData", &self.pUserData)
            .field("dwSizeOfUserData", &self.dwSizeOfUserData)
            .field("pNgcKerbTicket", &self.pNgcKerbTicket)
            .field("fSaveToCredMan", &self.fSaveToCredMan)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PPP_EAP_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes
            && self.Action == other.Action
            && self.dwAuthResultCode == other.dwAuthResultCode
            && self.pUserAttributes == other.pUserAttributes
            && self.fInvokeInteractiveUI == other.fInvokeInteractiveUI
            && self.pUIContextData == other.pUIContextData
            && self.dwSizeOfUIContextData == other.dwSizeOfUIContextData
            && self.fSaveConnectionData == other.fSaveConnectionData
            && self.pConnectionData == other.pConnectionData
            && self.dwSizeOfConnectionData == other.dwSizeOfConnectionData
            && self.fSaveUserData == other.fSaveUserData
            && self.pUserData == other.pUserData
            && self.dwSizeOfUserData == other.dwSizeOfUserData
            && self.pNgcKerbTicket == other.pNgcKerbTicket
            && self.fSaveToCredMan == other.fSaveToCredMan
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PPP_EAP_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PPP_EAP_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_EAP_PACKET {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl PPP_EAP_PACKET {}
impl ::std::default::Default for PPP_EAP_PACKET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_EAP_PACKET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_EAP_PACKET")
            .field("Code", &self.Code)
            .field("Id", &self.Id)
            .field("Length", &self.Length)
            .field("Data", &self.Data)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PPP_EAP_PACKET {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code
            && self.Id == other.Id
            && self.Length == other.Length
            && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for PPP_EAP_PACKET {}
unsafe impl ::windows::runtime::Abi for PPP_EAP_PACKET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAS_AUTH_ATTRIBUTE {
    pub raaType: RAS_AUTH_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub Value: *mut ::std::ffi::c_void,
}
impl RAS_AUTH_ATTRIBUTE {}
impl ::std::default::Default for RAS_AUTH_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAS_AUTH_ATTRIBUTE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_AUTH_ATTRIBUTE")
            .field("raaType", &self.raaType)
            .field("dwLength", &self.dwLength)
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RAS_AUTH_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.raaType == other.raaType
            && self.dwLength == other.dwLength
            && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for RAS_AUTH_ATTRIBUTE {}
unsafe impl ::windows::runtime::Abi for RAS_AUTH_ATTRIBUTE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RAS_AUTH_ATTRIBUTE_TYPE(pub i32);
pub const raatMinimum: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(0i32);
pub const raatUserName: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(1i32);
pub const raatUserPassword: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(2i32);
pub const raatMD5CHAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(3i32);
pub const raatNASIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(4i32);
pub const raatNASPort: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(5i32);
pub const raatServiceType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(6i32);
pub const raatFramedProtocol: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(7i32);
pub const raatFramedIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8i32);
pub const raatFramedIPNetmask: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9i32);
pub const raatFramedRouting: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(10i32);
pub const raatFilterId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(11i32);
pub const raatFramedMTU: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(12i32);
pub const raatFramedCompression: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(13i32);
pub const raatLoginIPHost: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(14i32);
pub const raatLoginService: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(15i32);
pub const raatLoginTCPPort: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(16i32);
pub const raatUnassigned17: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(17i32);
pub const raatReplyMessage: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(18i32);
pub const raatCallbackNumber: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(19i32);
pub const raatCallbackId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(20i32);
pub const raatUnassigned21: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(21i32);
pub const raatFramedRoute: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(22i32);
pub const raatFramedIPXNetwork: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(23i32);
pub const raatState: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(24i32);
pub const raatClass: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(25i32);
pub const raatVendorSpecific: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(26i32);
pub const raatSessionTimeout: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(27i32);
pub const raatIdleTimeout: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(28i32);
pub const raatTerminationAction: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(29i32);
pub const raatCalledStationId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(30i32);
pub const raatCallingStationId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(31i32);
pub const raatNASIdentifier: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(32i32);
pub const raatProxyState: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(33i32);
pub const raatLoginLATService: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(34i32);
pub const raatLoginLATNode: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(35i32);
pub const raatLoginLATGroup: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(36i32);
pub const raatFramedAppleTalkLink: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(37i32);
pub const raatFramedAppleTalkNetwork: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(38i32);
pub const raatFramedAppleTalkZone: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(39i32);
pub const raatAcctStatusType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(40i32);
pub const raatAcctDelayTime: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(41i32);
pub const raatAcctInputOctets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(42i32);
pub const raatAcctOutputOctets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(43i32);
pub const raatAcctSessionId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(44i32);
pub const raatAcctAuthentic: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(45i32);
pub const raatAcctSessionTime: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(46i32);
pub const raatAcctInputPackets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(47i32);
pub const raatAcctOutputPackets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(48i32);
pub const raatAcctTerminateCause: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(49i32);
pub const raatAcctMultiSessionId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(50i32);
pub const raatAcctLinkCount: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(51i32);
pub const raatAcctEventTimeStamp: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(55i32);
pub const raatMD5CHAPChallenge: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(60i32);
pub const raatNASPortType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(61i32);
pub const raatPortLimit: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(62i32);
pub const raatLoginLATPort: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(63i32);
pub const raatTunnelType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(64i32);
pub const raatTunnelMediumType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(65i32);
pub const raatTunnelClientEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(66i32);
pub const raatTunnelServerEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(67i32);
pub const raatARAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(70i32);
pub const raatARAPFeatures: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(71i32);
pub const raatARAPZoneAccess: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(72i32);
pub const raatARAPSecurity: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(73i32);
pub const raatARAPSecurityData: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(74i32);
pub const raatPasswordRetry: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(75i32);
pub const raatPrompt: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(76i32);
pub const raatConnectInfo: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(77i32);
pub const raatConfigurationToken: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(78i32);
pub const raatEAPMessage: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(79i32);
pub const raatSignature: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(80i32);
pub const raatARAPChallengeResponse: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(84i32);
pub const raatAcctInterimInterval: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(85i32);
pub const raatNASIPv6Address: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(95i32);
pub const raatFramedInterfaceId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(96i32);
pub const raatFramedIPv6Prefix: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(97i32);
pub const raatLoginIPv6Host: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(98i32);
pub const raatFramedIPv6Route: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(99i32);
pub const raatFramedIPv6Pool: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(100i32);
pub const raatARAPGuestLogon: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8096i32);
pub const raatCertificateOID: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8097i32);
pub const raatEAPConfiguration: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8098i32);
pub const raatPEAPEmbeddedEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8099i32);
pub const raatInnerEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8099i32);
pub const raatPEAPFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8100i32);
pub const raatFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8100i32);
pub const raatEAPTLV: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8102i32);
pub const raatCredentialsChanged: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8103i32);
pub const raatCertificateThumbprint: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8250i32);
pub const raatPeerId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9000i32);
pub const raatServerId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9001i32);
pub const raatMethodId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9002i32);
pub const raatEMSK: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9003i32);
pub const raatSessionId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9004i32);
pub const raatReserved: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(-1i32);
impl ::std::convert::From<i32> for RAS_AUTH_ATTRIBUTE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAS_AUTH_ATTRIBUTE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RAS_EAP_FLAG_8021X_AUTH: u32 = 128u32;
pub const RAS_EAP_FLAG_ALTERNATIVE_USER_DB: u32 = 2048u32;
pub const RAS_EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
pub const RAS_EAP_FLAG_FIRST_LINK: u32 = 16u32;
pub const RAS_EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
pub const RAS_EAP_FLAG_HOSTED_IN_PEAP: u32 = 256u32;
pub const RAS_EAP_FLAG_LOGON: u32 = 4u32;
pub const RAS_EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
pub const RAS_EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
pub const RAS_EAP_FLAG_PEAP_FORCE_FULL_AUTH: u32 = 4096u32;
pub const RAS_EAP_FLAG_PEAP_UPFRONT: u32 = 1024u32;
pub const RAS_EAP_FLAG_PREVIEW: u32 = 8u32;
pub const RAS_EAP_FLAG_PRE_LOGON: u32 = 131072u32;
pub const RAS_EAP_FLAG_RESERVED: u32 = 1048576u32;
pub const RAS_EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
pub const RAS_EAP_FLAG_ROUTER: u32 = 1u32;
pub const RAS_EAP_FLAG_SAVE_CREDMAN: u32 = 2097152u32;
pub const RAS_EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
pub const RAS_EAP_ROLE_AUTHENTICATEE: u32 = 2u32;
pub const RAS_EAP_ROLE_AUTHENTICATOR: u32 = 1u32;
pub const RAS_EAP_ROLE_EXCLUDE_IN_EAP: u32 = 4u32;
pub const RAS_EAP_ROLE_EXCLUDE_IN_PEAP: u32 = 8u32;
pub const RAS_EAP_ROLE_EXCLUDE_IN_VPN: u32 = 16u32;
