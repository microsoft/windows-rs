#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const CERTIFICATE_HASH_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPCODE_Failure: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPCODE_Request: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPCODE_Response: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPCODE_Success: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAPHOST_AUTH_INFO {
    pub status: EAPHOST_AUTH_STATUS,
    pub dwErrorCode: u32,
    pub dwReasonCode: u32,
}
impl ::core::marker::Copy for EAPHOST_AUTH_INFO {}
impl ::core::clone::Clone for EAPHOST_AUTH_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAPHOST_AUTH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAPHOST_AUTH_INFO").field("status", &self.status).field("dwErrorCode", &self.dwErrorCode).field("dwReasonCode", &self.dwReasonCode).finish()
    }
}
unsafe impl ::windows::core::Abi for EAPHOST_AUTH_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAPHOST_AUTH_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAPHOST_AUTH_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAPHOST_AUTH_INFO {}
impl ::core::default::Default for EAPHOST_AUTH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAPHOST_AUTH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostInvalidSession: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostAuthNotStarted: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostAuthIdentityExchange: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostAuthNegotiatingType: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostAuthInProgress: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostAuthSucceeded: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostAuthFailed: EAPHOST_AUTH_STATUS = EAPHOST_AUTH_STATUS(6i32);
impl ::core::marker::Copy for EAPHOST_AUTH_STATUS {}
impl ::core::clone::Clone for EAPHOST_AUTH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAPHOST_AUTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAPHOST_AUTH_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAPHOST_AUTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAPHOST_AUTH_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAPHOST_IDENTITY_UI_PARAMS {
    pub eapMethodType: EAP_METHOD_TYPE,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: ::windows::core::PWSTR,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
impl ::core::marker::Copy for EAPHOST_IDENTITY_UI_PARAMS {}
impl ::core::clone::Clone for EAPHOST_IDENTITY_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAPHOST_IDENTITY_UI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAPHOST_IDENTITY_UI_PARAMS")
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
unsafe impl ::windows::core::Abi for EAPHOST_IDENTITY_UI_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAPHOST_IDENTITY_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAPHOST_IDENTITY_UI_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAPHOST_IDENTITY_UI_PARAMS {}
impl ::core::default::Default for EAPHOST_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAPHOST_INTERACTIVE_UI_PARAMS {
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
impl ::core::marker::Copy for EAPHOST_INTERACTIVE_UI_PARAMS {}
impl ::core::clone::Clone for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAPHOST_INTERACTIVE_UI_PARAMS").field("dwSizeofContextData", &self.dwSizeofContextData).field("pContextData", &self.pContextData).field("dwSizeofInteractiveUIData", &self.dwSizeofInteractiveUIData).field("pInteractiveUIData", &self.pInteractiveUIData).field("dwError", &self.dwError).field("pEapError", &self.pEapError).finish()
    }
}
unsafe impl ::windows::core::Abi for EAPHOST_INTERACTIVE_UI_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAPHOST_INTERACTIVE_UI_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAPHOST_INTERACTIVE_UI_PARAMS {}
impl ::core::default::Default for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPHOST_METHOD_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPHOST_PEER_API_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_ATTRIBUTE {
    pub eaType: EAP_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub pValue: *mut u8,
}
impl ::core::marker::Copy for EAP_ATTRIBUTE {}
impl ::core::clone::Clone for EAP_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_ATTRIBUTE").field("eaType", &self.eaType).field("dwLength", &self.dwLength).field("pValue", &self.pValue).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_ATTRIBUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_ATTRIBUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_ATTRIBUTE {}
impl ::core::default::Default for EAP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_ATTRIBUTES {
    pub dwNumberOfAttributes: u32,
    pub pAttribs: *mut EAP_ATTRIBUTE,
}
impl ::core::marker::Copy for EAP_ATTRIBUTES {}
impl ::core::clone::Clone for EAP_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_ATTRIBUTES").field("dwNumberOfAttributes", &self.dwNumberOfAttributes).field("pAttribs", &self.pAttribs).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_ATTRIBUTES {}
impl ::core::default::Default for EAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_ATTRIBUTE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatMinimum: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatUserName: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatUserPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatMD5CHAPPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatNASIPAddress: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatNASPort: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatServiceType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedProtocol: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedIPAddress: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedIPNetmask: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedRouting: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFilterId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedMTU: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedCompression: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginIPHost: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginService: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginTCPPort: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatUnassigned17: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatReplyMessage: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCallbackNumber: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCallbackId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatUnassigned21: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedRoute: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedIPXNetwork: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatState: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatClass: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatVendorSpecific: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatSessionTimeout: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatIdleTimeout: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatTerminationAction: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCalledStationId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCallingStationId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatNASIdentifier: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatProxyState: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginLATService: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginLATNode: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginLATGroup: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedAppleTalkLink: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedAppleTalkNetwork: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedAppleTalkZone: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctStatusType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctDelayTime: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctInputOctets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctOutputOctets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctSessionId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctAuthentic: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctSessionTime: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctInputPackets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctOutputPackets: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctTerminateCause: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctMultiSessionId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctLinkCount: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctEventTimeStamp: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(55i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatMD5CHAPChallenge: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(60i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatNASPortType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(61i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatPortLimit: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(62i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginLATPort: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(63i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatTunnelType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatTunnelMediumType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatTunnelClientEndpoint: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(66i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatTunnelServerEndpoint: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(67i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(70i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPFeatures: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(71i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPZoneAccess: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(72i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPSecurity: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(73i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPSecurityData: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(74i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatPasswordRetry: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(75i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatPrompt: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(76i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatConnectInfo: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(77i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatConfigurationToken: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(78i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatEAPMessage: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(79i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatSignature: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(80i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPChallengeResponse: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(84i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatAcctInterimInterval: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(85i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatNASIPv6Address: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(95i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedInterfaceId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(96i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedIPv6Prefix: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(97i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatLoginIPv6Host: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(98i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedIPv6Route: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(99i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFramedIPv6Pool: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(100i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatARAPGuestLogon: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8096i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCertificateOID: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8097i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatEAPConfiguration: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8098i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatPEAPEmbeddedEAPTypeId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8099i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatPEAPFastRoamedSession: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8100i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatFastRoamedSession: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8100i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatEAPTLV: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8102i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCredentialsChanged: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8103i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatInnerEapMethodType: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8104i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatClearTextPassword: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8107i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatQuarantineSoH: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8150i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatCertificateThumbprint: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(8250i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatPeerId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9000i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatServerId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9001i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatMethodId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9002i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatEMSK: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9003i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatSessionId: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(9004i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eatReserved: EAP_ATTRIBUTE_TYPE = EAP_ATTRIBUTE_TYPE(-1i32);
impl ::core::marker::Copy for EAP_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for EAP_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_ATTRIBUTE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
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
impl ::core::marker::Copy for EAP_AUTHENTICATOR_METHOD_ROUTINES {}
impl ::core::clone::Clone for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_AUTHENTICATOR_METHOD_ROUTINES")
            .field("dwSizeInBytes", &self.dwSizeInBytes)
            .field("pEapType", &self.pEapType)
            .field("EapMethodAuthenticatorInitialize", &self.EapMethodAuthenticatorInitialize)
            .field("EapMethodAuthenticatorBeginSession", &self.EapMethodAuthenticatorBeginSession)
            .field("EapMethodAuthenticatorUpdateInnerMethodParams", &self.EapMethodAuthenticatorUpdateInnerMethodParams)
            .field("EapMethodAuthenticatorReceivePacket", &self.EapMethodAuthenticatorReceivePacket)
            .field("EapMethodAuthenticatorSendPacket", &self.EapMethodAuthenticatorSendPacket)
            .field("EapMethodAuthenticatorGetAttributes", &self.EapMethodAuthenticatorGetAttributes)
            .field("EapMethodAuthenticatorSetAttributes", &self.EapMethodAuthenticatorSetAttributes)
            .field("EapMethodAuthenticatorGetResult", &self.EapMethodAuthenticatorGetResult)
            .field("EapMethodAuthenticatorEndSession", &self.EapMethodAuthenticatorEndSession)
            .field("EapMethodAuthenticatorShutdown", &self.EapMethodAuthenticatorShutdown)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_AUTHENTICATOR_METHOD_ROUTINES>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_AUTHENTICATOR_METHOD_ROUTINES {}
impl ::core::default::Default for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_AUTHENTICATOR_SEND_TIMEOUT(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_NONE: EAP_AUTHENTICATOR_SEND_TIMEOUT = EAP_AUTHENTICATOR_SEND_TIMEOUT(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_BASIC: EAP_AUTHENTICATOR_SEND_TIMEOUT = EAP_AUTHENTICATOR_SEND_TIMEOUT(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_INTERACTIVE: EAP_AUTHENTICATOR_SEND_TIMEOUT = EAP_AUTHENTICATOR_SEND_TIMEOUT(2i32);
impl ::core::marker::Copy for EAP_AUTHENTICATOR_SEND_TIMEOUT {}
impl ::core::clone::Clone for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_AUTHENTICATOR_SEND_TIMEOUT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_VALUENAME_CONFIGUI: &str = "AuthenticatorConfigUIPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_VALUENAME_DLL_PATH: &str = "AuthenticatorDllPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_VALUENAME_FRIENDLY_NAME: &str = "AuthenticatorFriendlyName";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_AUTHENTICATOR_VALUENAME_PROPERTIES: &str = "Properties";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_CONFIG_INPUT_FIELD_ARRAY {
    pub dwVersion: u32,
    pub dwNumberOfFields: u32,
    pub pFields: *mut EAP_CONFIG_INPUT_FIELD_DATA,
}
impl ::core::marker::Copy for EAP_CONFIG_INPUT_FIELD_ARRAY {}
impl ::core::clone::Clone for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_CONFIG_INPUT_FIELD_ARRAY").field("dwVersion", &self.dwVersion).field("dwNumberOfFields", &self.dwNumberOfFields).field("pFields", &self.pFields).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_CONFIG_INPUT_FIELD_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_CONFIG_INPUT_FIELD_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_CONFIG_INPUT_FIELD_ARRAY {}
impl ::core::default::Default for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_CONFIG_INPUT_FIELD_DATA {
    pub dwSize: u32,
    pub Type: EAP_CONFIG_INPUT_FIELD_TYPE,
    pub dwFlagProps: u32,
    pub pwszLabel: ::windows::core::PWSTR,
    pub pwszData: ::windows::core::PWSTR,
    pub dwMinDataLength: u32,
    pub dwMaxDataLength: u32,
}
impl ::core::marker::Copy for EAP_CONFIG_INPUT_FIELD_DATA {}
impl ::core::clone::Clone for EAP_CONFIG_INPUT_FIELD_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_CONFIG_INPUT_FIELD_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_CONFIG_INPUT_FIELD_DATA").field("dwSize", &self.dwSize).field("Type", &self.Type).field("dwFlagProps", &self.dwFlagProps).field("pwszLabel", &self.pwszLabel).field("pwszData", &self.pwszData).field("dwMinDataLength", &self.dwMinDataLength).field("dwMaxDataLength", &self.dwMaxDataLength).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_CONFIG_INPUT_FIELD_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_CONFIG_INPUT_FIELD_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_CONFIG_INPUT_FIELD_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_CONFIG_INPUT_FIELD_DATA {}
impl ::core::default::Default for EAP_CONFIG_INPUT_FIELD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_CONFIG_INPUT_FIELD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputUsername: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputPassword: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputNetworkUsername: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputNetworkPassword: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputPin: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputPSK: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigInputEdit: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigSmartCardUsername: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapConfigSmartCardError: EAP_CONFIG_INPUT_FIELD_TYPE = EAP_CONFIG_INPUT_FIELD_TYPE(8i32);
impl ::core::marker::Copy for EAP_CONFIG_INPUT_FIELD_TYPE {}
impl ::core::clone::Clone for EAP_CONFIG_INPUT_FIELD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_CONFIG_INPUT_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_CONFIG_INPUT_FIELD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_CONFIG_INPUT_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_CONFIG_INPUT_FIELD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_CREDENTIAL_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_CRED_EXPIRY_REQ {
    pub curCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub newCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
}
impl ::core::marker::Copy for EAP_CRED_EXPIRY_REQ {}
impl ::core::clone::Clone for EAP_CRED_EXPIRY_REQ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_CRED_EXPIRY_REQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_CRED_EXPIRY_REQ").field("curCreds", &self.curCreds).field("newCreds", &self.newCreds).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_CRED_EXPIRY_REQ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_CRED_EXPIRY_REQ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_CRED_EXPIRY_REQ>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_CRED_EXPIRY_REQ {}
impl ::core::default::Default for EAP_CRED_EXPIRY_REQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: ::windows::core::GUID,
    pub repairGuid: ::windows::core::GUID,
    pub helpLinkGuid: ::windows::core::GUID,
    pub pRootCauseString: ::windows::core::PWSTR,
    pub pRepairString: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for EAP_ERROR {}
impl ::core::clone::Clone for EAP_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_ERROR").field("dwWinError", &self.dwWinError).field("type", &self.r#type).field("dwReasonCode", &self.dwReasonCode).field("rootCauseGuid", &self.rootCauseGuid).field("repairGuid", &self.repairGuid).field("helpLinkGuid", &self.helpLinkGuid).field("pRootCauseString", &self.pRootCauseString).field("pRepairString", &self.pRepairString).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_ERROR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_ERROR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_ERROR>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_ERROR {}
impl ::core::default::Default for EAP_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_AUTHENTICATION_FAILED: u32 = 2151809045u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_CERT_STORE_INACCESSIBLE: u32 = 2151809040u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_EAPQEC_INACCESSIBLE: u32 = 2151809043u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_FIRST: i32 = -2143158272i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_IDENTITY_UNKNOWN: u32 = 2151809044u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_LAST: i32 = -2143158017i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_METHOD_INVALID_PACKET: u32 = 2151809047u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_METHOD_NOT_INSTALLED: u32 = 2151809041u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED: u32 = 2151809056u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_REMOTE_INVALID_PACKET: u32 = 2151809048u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_THIRDPARTY_METHOD_HOST_RESET: u32 = 2151809042u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_EAPHOST_XML_MALFORMED: u32 = 2151809049u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO: u32 = 2151809050u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_NO_SMART_CARD_READER: u32 = 2151809299u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_CERT_EXPIRED: u32 = 2151809538u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_CERT_INVALID: u32 = 2151809537u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_CERT_NOT_FOUND: u32 = 2151809536u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_CERT_OTHER_ERROR: u32 = 2151809540u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_CERT_REVOKED: u32 = 2151809539u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_FIRST: i32 = -2143157760i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_LAST: i32 = -2143157505i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_ROOT_CERT_FIRST: i32 = -2143157248i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_ROOT_CERT_INVALID: u32 = 2151810049u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_ROOT_CERT_LAST: i32 = -2143156993i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 2151810054u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SERVER_ROOT_CERT_NOT_FOUND: u32 = 2151810048u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_SIM_NOT_VALID: u32 = 2151810304u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CERT_EXPIRED: u32 = 2151809282u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CERT_INVALID: u32 = 2151809281u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CERT_NOT_FOUND: u32 = 2151809280u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CERT_OTHER_ERROR: u32 = 2151809284u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CERT_REJECTED: u32 = 2151809285u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CERT_REVOKED: u32 = 2151809283u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_CREDENTIALS_REJECTED: u32 = 2151809297u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_FIRST: i32 = -2143158016i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_LAST: i32 = -2143157761i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_NAME_PASSWORD_REJECTED: u32 = 2151809298u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_ROOT_CERT_EXPIRED: u32 = 2151809794u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_ROOT_CERT_FIRST: i32 = -2143157504i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_ROOT_CERT_INVALID: u32 = 2151809793u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_ROOT_CERT_LAST: i32 = -2143157249i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_E_USER_ROOT_CERT_NOT_FOUND: u32 = 2151809792u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_FULL_AUTH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_LOGON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_ONLY_EAP_TLS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_PREVIEW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_PRE_LOGON: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved2: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved3: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved4: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved5: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved6: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved7: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved8: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_Reserved9: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_SUPRESS_UI: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_USER_AUTH: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_FLAG_VPN: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_GROUP_MASK: i32 = 65280i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_INTERACTIVE_UI_DATA {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwDataType: EAP_INTERACTIVE_UI_DATA_TYPE,
    pub cbUiData: u32,
    pub pbUiData: EAP_UI_DATA_FORMAT,
}
impl ::core::marker::Copy for EAP_INTERACTIVE_UI_DATA {}
impl ::core::clone::Clone for EAP_INTERACTIVE_UI_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EAP_INTERACTIVE_UI_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_INTERACTIVE_UI_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_INTERACTIVE_UI_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_INTERACTIVE_UI_DATA {}
impl ::core::default::Default for EAP_INTERACTIVE_UI_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_INTERACTIVE_UI_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCredReq: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCredResp: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCredExpiryReq: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCredExpiryResp: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCredLogonReq: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCredLogonResp: EAP_INTERACTIVE_UI_DATA_TYPE = EAP_INTERACTIVE_UI_DATA_TYPE(5i32);
impl ::core::marker::Copy for EAP_INTERACTIVE_UI_DATA_TYPE {}
impl ::core::clone::Clone for EAP_INTERACTIVE_UI_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_INTERACTIVE_UI_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_INTERACTIVE_UI_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_INTERACTIVE_UI_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_INTERACTIVE_UI_DATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_INTERACTIVE_UI_DATA_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_INVALID_PACKET: u32 = 2151809048u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED: u32 = 1078067222u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_I_EAPHOST_FIRST: i32 = -2143158272i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_I_EAPHOST_LAST: i32 = -2143158017i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_I_USER_ACCOUNT_OTHER_ERROR: u32 = 1078067472u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_I_USER_FIRST: i32 = 1078067456i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_I_USER_LAST: i32 = 1078067711i32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_CONFIG_IS_IDENTITY_PRIVACY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_DISCARD: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_SEND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESULT: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESPOND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_AUTHENTICATE: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_HANDLE_IDENTITY: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(5i32);
impl ::core::marker::Copy for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {}
impl ::core::clone::Clone for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_AUTHENTICATOR_RESULT {
    pub fIsSuccess: super::super::Foundation::BOOL,
    pub dwFailureReason: u32,
    pub pAuthAttribs: *mut EAP_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_AUTHENTICATOR_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_AUTHENTICATOR_RESULT").field("fIsSuccess", &self.fIsSuccess).field("dwFailureReason", &self.dwFailureReason).field("pAuthAttribs", &self.pAuthAttribs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EAP_METHOD_AUTHENTICATOR_RESULT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_AUTHENTICATOR_RESULT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_AUTHENTICATOR_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_INFO {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: ::windows::core::PWSTR,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfo: *mut EAP_METHOD_INFO,
}
impl ::core::marker::Copy for EAP_METHOD_INFO {}
impl ::core::clone::Clone for EAP_METHOD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO").field("eaptype", &self.eaptype).field("pwszAuthorName", &self.pwszAuthorName).field("pwszFriendlyName", &self.pwszFriendlyName).field("eapProperties", &self.eapProperties).field("pInnerMethodInfo", &self.pInnerMethodInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO {}
impl ::core::default::Default for EAP_METHOD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_INFO_ARRAY {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO,
}
impl ::core::marker::Copy for EAP_METHOD_INFO_ARRAY {}
impl ::core::clone::Clone for EAP_METHOD_INFO_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO_ARRAY").field("dwNumberOfMethods", &self.dwNumberOfMethods).field("pEapMethods", &self.pEapMethods).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_INFO_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_INFO_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO_ARRAY {}
impl ::core::default::Default for EAP_METHOD_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_INFO_ARRAY_EX {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO_EX,
}
impl ::core::marker::Copy for EAP_METHOD_INFO_ARRAY_EX {}
impl ::core::clone::Clone for EAP_METHOD_INFO_ARRAY_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_INFO_ARRAY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO_ARRAY_EX").field("dwNumberOfMethods", &self.dwNumberOfMethods).field("pEapMethods", &self.pEapMethods).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_INFO_ARRAY_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO_ARRAY_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_INFO_ARRAY_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO_ARRAY_EX {}
impl ::core::default::Default for EAP_METHOD_INFO_ARRAY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_INFO_EX {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: ::windows::core::PWSTR,
    pub pwszFriendlyName: ::windows::core::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfoArray: *mut EAP_METHOD_INFO_ARRAY_EX,
}
impl ::core::marker::Copy for EAP_METHOD_INFO_EX {}
impl ::core::clone::Clone for EAP_METHOD_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO_EX").field("eaptype", &self.eaptype).field("pwszAuthorName", &self.pwszAuthorName).field("pwszFriendlyName", &self.pwszFriendlyName).field("eapProperties", &self.eapProperties).field("pInnerMethodInfoArray", &self.pInnerMethodInfoArray).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_INFO_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_INFO_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO_EX {}
impl ::core::default::Default for EAP_METHOD_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_METHOD_INVALID_PACKET: u32 = 2151809047u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY {
    pub eapMethodPropertyType: EAP_METHOD_PROPERTY_TYPE,
    pub eapMethodPropertyValueType: EAP_METHOD_PROPERTY_VALUE_TYPE,
    pub eapMethodPropertyValue: EAP_METHOD_PROPERTY_VALUE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_PROPERTY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_ARRAY {
    pub dwNumberOfProperties: u32,
    pub pMethodProperty: *mut EAP_METHOD_PROPERTY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_ARRAY").field("dwNumberOfProperties", &self.dwNumberOfProperties).field("pMethodProperty", &self.pMethodProperty).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_PROPERTY_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_METHOD_PROPERTY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropCipherSuiteNegotiation: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropMutualAuth: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropIntegrity: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropReplayProtection: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropConfidentiality: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropKeyDerivation: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropKeyStrength64: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropKeyStrength128: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropKeyStrength256: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropKeyStrength512: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropKeyStrength1024: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropDictionaryAttackResistance: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropFastReconnect: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropCryptoBinding: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropSessionIndependence: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropFragmentation: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropChannelBinding: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropNap: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropStandalone: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropMppeEncryption: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropTunnelMethod: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropSupportsConfig: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropCertifiedMethod: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropHiddenMethod: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropMachineAuth: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropUserAuth: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropIdentityPrivacy: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropMethodChaining: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropSharedStateEquivalence: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptLegacyMethodPropertyFlag: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const emptPropVendorSpecific: EAP_METHOD_PROPERTY_TYPE = EAP_METHOD_PROPERTY_TYPE(255i32);
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_TYPE {}
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_METHOD_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_METHOD_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union EAP_METHOD_PROPERTY_VALUE {
    pub empvBool: EAP_METHOD_PROPERTY_VALUE_BOOL,
    pub empvDword: EAP_METHOD_PROPERTY_VALUE_DWORD,
    pub empvString: EAP_METHOD_PROPERTY_VALUE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_VALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_PROPERTY_VALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_VALUE_BOOL {
    pub length: u32,
    pub value: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_BOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_VALUE_BOOL").field("length", &self.length).field("value", &self.value).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_VALUE_BOOL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_PROPERTY_VALUE_BOOL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_BOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_PROPERTY_VALUE_DWORD {
    pub length: u32,
    pub value: u32,
}
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_DWORD {}
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_VALUE_DWORD").field("length", &self.length).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_VALUE_DWORD {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_PROPERTY_VALUE_DWORD>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_DWORD {}
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_PROPERTY_VALUE_STRING {
    pub length: u32,
    pub value: *mut u8,
}
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_STRING {}
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_VALUE_STRING").field("length", &self.length).field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_VALUE_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_PROPERTY_VALUE_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_STRING {}
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EAP_METHOD_PROPERTY_VALUE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const empvtBool: EAP_METHOD_PROPERTY_VALUE_TYPE = EAP_METHOD_PROPERTY_VALUE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const empvtDword: EAP_METHOD_PROPERTY_VALUE_TYPE = EAP_METHOD_PROPERTY_VALUE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const empvtString: EAP_METHOD_PROPERTY_VALUE_TYPE = EAP_METHOD_PROPERTY_VALUE_TYPE(2i32);
impl ::core::marker::Copy for EAP_METHOD_PROPERTY_VALUE_TYPE {}
impl ::core::clone::Clone for EAP_METHOD_PROPERTY_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_PROPERTY_VALUE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_METHOD_PROPERTY_VALUE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_METHOD_TYPE {
    pub eapType: EAP_TYPE,
    pub dwAuthorId: u32,
}
impl ::core::marker::Copy for EAP_METHOD_TYPE {}
impl ::core::clone::Clone for EAP_METHOD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_METHOD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_TYPE").field("eapType", &self.eapType).field("dwAuthorId", &self.dwAuthorId).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_METHOD_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_METHOD_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_METHOD_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_METHOD_TYPE {}
impl ::core::default::Default for EAP_METHOD_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_FLAG_GUEST_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: u32 = 32768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
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
impl ::core::marker::Copy for EAP_PEER_METHOD_ROUTINES {}
impl ::core::clone::Clone for EAP_PEER_METHOD_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_PEER_METHOD_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_PEER_METHOD_ROUTINES")
            .field("dwVersion", &self.dwVersion)
            .field("pEapType", &self.pEapType)
            .field("EapPeerInitialize", &self.EapPeerInitialize)
            .field("EapPeerGetIdentity", &self.EapPeerGetIdentity)
            .field("EapPeerBeginSession", &self.EapPeerBeginSession)
            .field("EapPeerSetCredentials", &self.EapPeerSetCredentials)
            .field("EapPeerProcessRequestPacket", &self.EapPeerProcessRequestPacket)
            .field("EapPeerGetResponsePacket", &self.EapPeerGetResponsePacket)
            .field("EapPeerGetResult", &self.EapPeerGetResult)
            .field("EapPeerGetUIContext", &self.EapPeerGetUIContext)
            .field("EapPeerSetUIContext", &self.EapPeerSetUIContext)
            .field("EapPeerGetResponseAttributes", &self.EapPeerGetResponseAttributes)
            .field("EapPeerSetResponseAttributes", &self.EapPeerSetResponseAttributes)
            .field("EapPeerEndSession", &self.EapPeerEndSession)
            .field("EapPeerShutdown", &self.EapPeerShutdown)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_PEER_METHOD_ROUTINES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_PEER_METHOD_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_PEER_METHOD_ROUTINES>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_PEER_METHOD_ROUTINES {}
impl ::core::default::Default for EAP_PEER_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_CONFIGUI: &str = "PeerConfigUIPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_DLL_PATH: &str = "PeerDllPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_FRIENDLY_NAME: &str = "PeerFriendlyName";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_IDENTITY: &str = "PeerIdentityPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_INTERACTIVEUI: &str = "PeerInteractiveUIPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_INVOKE_NAMEDLG: &str = "PeerInvokeUsernameDialog";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_INVOKE_PWDDLG: &str = "PeerInvokePasswordDialog";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_PROPERTIES: &str = "Properties";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_PEER_VALUENAME_REQUIRE_CONFIGUI: &str = "PeerRequireConfigUI";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_REGISTRY_LOCATION: &str = "System\\CurrentControlSet\\Services\\EapHost\\Methods";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EAP_TYPE {
    pub r#type: u8,
    pub dwVendorId: u32,
    pub dwVendorType: u32,
}
impl ::core::marker::Copy for EAP_TYPE {}
impl ::core::clone::Clone for EAP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_TYPE").field("type", &self.r#type).field("dwVendorId", &self.dwVendorId).field("dwVendorType", &self.dwVendorType).finish()
    }
}
unsafe impl ::windows::core::Abi for EAP_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_TYPE {}
impl ::core::default::Default for EAP_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub union EAP_UI_DATA_FORMAT {
    pub credData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub credExpiryData: *mut EAP_CRED_EXPIRY_REQ,
    pub credLogonData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
}
impl ::core::marker::Copy for EAP_UI_DATA_FORMAT {}
impl ::core::clone::Clone for EAP_UI_DATA_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EAP_UI_DATA_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EAP_UI_DATA_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EAP_UI_DATA_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for EAP_UI_DATA_FORMAT {}
impl ::core::default::Default for EAP_UI_DATA_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_VALUENAME_PROPERTIES: &str = "Properties";
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EapCertificateCredential {
    pub certHash: [u8; 20],
    pub password: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for EapCertificateCredential {}
impl ::core::clone::Clone for EapCertificateCredential {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EapCertificateCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapCertificateCredential").field("certHash", &self.certHash).field("password", &self.password).finish()
    }
}
unsafe impl ::windows::core::Abi for EapCertificateCredential {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EapCertificateCredential {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapCertificateCredential>()) == 0 }
    }
}
impl ::core::cmp::Eq for EapCertificateCredential {}
impl ::core::default::Default for EapCertificateCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapCode(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCodeMinimum: EapCode = EapCode(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCodeRequest: EapCode = EapCode(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCodeResponse: EapCode = EapCode(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCodeSuccess: EapCode = EapCode(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCodeFailure: EapCode = EapCode(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapCodeMaximum: EapCode = EapCode(4i32);
impl ::core::marker::Copy for EapCode {}
impl ::core::clone::Clone for EapCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapCode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapCode {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapCode").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EapCredential {
    pub credType: EapCredentialType,
    pub credData: EapCredentialTypeData,
}
impl ::core::marker::Copy for EapCredential {}
impl ::core::clone::Clone for EapCredential {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EapCredential {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EapCredential {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapCredential>()) == 0 }
    }
}
impl ::core::cmp::Eq for EapCredential {}
impl ::core::default::Default for EapCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapCredentialType(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_EMPTY_CREDENTIAL: EapCredentialType = EapCredentialType(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_USERNAME_PASSWORD_CREDENTIAL: EapCredentialType = EapCredentialType(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_WINLOGON_CREDENTIAL: EapCredentialType = EapCredentialType(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_CERTIFICATE_CREDENTIAL: EapCredentialType = EapCredentialType(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAP_SIM_CREDENTIAL: EapCredentialType = EapCredentialType(4i32);
impl ::core::marker::Copy for EapCredentialType {}
impl ::core::clone::Clone for EapCredentialType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapCredentialType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapCredentialType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapCredentialType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapCredentialType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub union EapCredentialTypeData {
    pub username_password: EapUsernamePasswordCredential,
    pub certificate: EapCertificateCredential,
    pub sim: EapSimCredential,
}
impl ::core::marker::Copy for EapCredentialTypeData {}
impl ::core::clone::Clone for EapCredentialTypeData {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for EapCredentialTypeData {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EapCredentialTypeData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapCredentialTypeData>()) == 0 }
    }
}
impl ::core::cmp::Eq for EapCredentialTypeData {}
impl ::core::default::Default for EapCredentialTypeData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapHostPeerAuthParams(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerAuthStatus: EapHostPeerAuthParams = EapHostPeerAuthParams(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerIdentity: EapHostPeerAuthParams = EapHostPeerAuthParams(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerIdentityExtendedInfo: EapHostPeerAuthParams = EapHostPeerAuthParams(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostNapInfo: EapHostPeerAuthParams = EapHostPeerAuthParams(4i32);
impl ::core::marker::Copy for EapHostPeerAuthParams {}
impl ::core::clone::Clone for EapHostPeerAuthParams {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapHostPeerAuthParams {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapHostPeerAuthParams {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapHostPeerAuthParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapHostPeerAuthParams").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerBeginSession<'a, P0>(dwflags: u32, eaptype: EAP_METHOD_TYPE, pattributearray: *const EAP_ATTRIBUTES, htokenimpersonateuser: P0, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, dwmaxsendpacketsize: u32, pconnectionid: *const ::windows::core::GUID, func: NotificationHandler, pcontextdata: *mut ::core::ffi::c_void, psessionid: *mut u32, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerBeginSession(dwflags: u32, eaptype: EAP_METHOD_TYPE, pattributearray: *const EAP_ATTRIBUTES, htokenimpersonateuser: super::super::Foundation::HANDLE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, dwmaxsendpacketsize: u32, pconnectionid: *const ::windows::core::GUID, func: *mut ::core::ffi::c_void, pcontextdata: *mut ::core::ffi::c_void, psessionid: *mut u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerBeginSession(dwflags, ::core::mem::transmute(eaptype), ::core::mem::transmute(pattributearray), htokenimpersonateuser.into(), dwsizeofconnectiondata, ::core::mem::transmute(pconnectiondata), dwsizeofuserdata, ::core::mem::transmute(puserdata), dwmaxsendpacketsize, ::core::mem::transmute(pconnectionid), ::core::mem::transmute(func), ::core::mem::transmute(pcontextdata), ::core::mem::transmute(psessionid), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerClearConnection(pconnectionid: *mut ::windows::core::GUID, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerClearConnection(pconnectionid: *mut ::windows::core::GUID, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerClearConnection(::core::mem::transmute(pconnectionid), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn EapHostPeerConfigBlob2Xml(dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, pconfigin: &[u8], ppconfigdoc: *mut ::core::option::Option<super::super::Data::Xml::MsXml::IXMLDOMDocument2>, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerConfigBlob2Xml(dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, ppconfigdoc: *mut *mut ::core::ffi::c_void, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerConfigBlob2Xml(dwflags, ::core::mem::transmute(eapmethodtype), pconfigin.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigin)), ::core::mem::transmute(ppconfigdoc), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn EapHostPeerConfigXml2Blob<'a, P0>(dwflags: u32, pconfigdoc: P0, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Data::Xml::MsXml::IXMLDOMNode>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerConfigXml2Blob(dwflags: u32, pconfigdoc: *mut ::core::ffi::c_void, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerConfigXml2Blob(dwflags, pconfigdoc.into().abi(), ::core::mem::transmute(pdwsizeofconfigout), ::core::mem::transmute(ppconfigout), ::core::mem::transmute(peapmethodtype), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn EapHostPeerCredentialsXml2Blob<'a, P0>(dwflags: u32, pcredentialsdoc: P0, pconfigin: &[u8], pdwsizeofcredentialsout: *mut u32, ppcredentialsout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Data::Xml::MsXml::IXMLDOMNode>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerCredentialsXml2Blob(dwflags: u32, pcredentialsdoc: *mut ::core::ffi::c_void, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofcredentialsout: *mut u32, ppcredentialsout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerCredentialsXml2Blob(dwflags, pcredentialsdoc.into().abi(), pconfigin.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigin)), ::core::mem::transmute(pdwsizeofcredentialsout), ::core::mem::transmute(ppcredentialsout), ::core::mem::transmute(peapmethodtype), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerEndSession(sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerEndSession(sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerEndSession(sessionhandle, ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerFreeEapError(peaperror: *mut EAP_ERROR) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerFreeEapError(peaperror: *mut EAP_ERROR);
    }
    EapHostPeerFreeEapError(::core::mem::transmute(peaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerFreeErrorMemory(peaperror: *mut EAP_ERROR) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerFreeErrorMemory(peaperror: *mut EAP_ERROR);
    }
    EapHostPeerFreeErrorMemory(::core::mem::transmute(peaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerFreeMemory(pdata: *mut u8) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerFreeMemory(pdata: *mut u8);
    }
    EapHostPeerFreeMemory(::core::mem::transmute(pdata))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerFreeRuntimeMemory(pdata: *mut u8) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerFreeRuntimeMemory(pdata: *mut u8);
    }
    EapHostPeerFreeRuntimeMemory(::core::mem::transmute(pdata))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerGetAuthStatus(sessionhandle: u32, authparam: EapHostPeerAuthParams, pcbauthdata: *mut u32, ppauthdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetAuthStatus(sessionhandle: u32, authparam: EapHostPeerAuthParams, pcbauthdata: *mut u32, ppauthdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetAuthStatus(sessionhandle, authparam, ::core::mem::transmute(pcbauthdata), ::core::mem::transmute(ppauthdata), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerGetDataToUnplumbCredentials(pconnectionidthatlastsavedcreds: *mut ::windows::core::GUID, phcredentialimpersonationtoken: *mut isize, sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR, fsavetocredman: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetDataToUnplumbCredentials(pconnectionidthatlastsavedcreds: *mut ::windows::core::GUID, phcredentialimpersonationtoken: *mut isize, sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR, fsavetocredman: *mut super::super::Foundation::BOOL) -> u32;
    }
    EapHostPeerGetDataToUnplumbCredentials(::core::mem::transmute(pconnectionidthatlastsavedcreds), ::core::mem::transmute(phcredentialimpersonationtoken), sessionhandle, ::core::mem::transmute(ppeaperror), ::core::mem::transmute(fsavetocredman))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerGetEncryptedPassword<'a, P0>(dwsizeofpassword: u32, szpassword: P0, ppszencpassword: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetEncryptedPassword(dwsizeofpassword: u32, szpassword: ::windows::core::PCWSTR, ppszencpassword: *mut ::windows::core::PWSTR) -> u32;
    }
    EapHostPeerGetEncryptedPassword(dwsizeofpassword, szpassword.into(), ::core::mem::transmute(ppszencpassword))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerGetIdentity<'a, P0>(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, pconnectiondata: &[u8], puserdata: &[u8], htokenimpersonateuser: P0, pfinvokeui: *mut super::super::Foundation::BOOL, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut ::windows::core::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetIdentity(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, htokenimpersonateuser: super::super::Foundation::HANDLE, pfinvokeui: *mut super::super::Foundation::BOOL, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut ::windows::core::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut u8) -> u32;
    }
    EapHostPeerGetIdentity(
        dwversion,
        dwflags,
        ::core::mem::transmute(eapmethodtype),
        pconnectiondata.len() as _,
        ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondata)),
        puserdata.len() as _,
        ::core::mem::transmute(::windows::core::as_ptr_or_null(puserdata)),
        htokenimpersonateuser.into(),
        ::core::mem::transmute(pfinvokeui),
        ::core::mem::transmute(pdwsizeofuserdataout),
        ::core::mem::transmute(ppuserdataout),
        ::core::mem::transmute(ppwszidentity),
        ::core::mem::transmute(ppeaperror),
        ::core::mem::transmute(ppvreserved),
    )
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerGetMethodProperties<'a, P0>(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, huserimpersonationtoken: P0, pbeapconndata: &[u8], pbuserdata: &[u8], pmethodpropertyarray: *mut EAP_METHOD_PROPERTY_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetMethodProperties(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, huserimpersonationtoken: super::super::Foundation::HANDLE, dweapconndatasize: u32, pbeapconndata: *const u8, dwuserdatasize: u32, pbuserdata: *const u8, pmethodpropertyarray: *mut EAP_METHOD_PROPERTY_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetMethodProperties(dwversion, dwflags, ::core::mem::transmute(eapmethodtype), huserimpersonationtoken.into(), pbeapconndata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbeapconndata)), pbuserdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbuserdata)), ::core::mem::transmute(pmethodpropertyarray), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerGetMethods(peapmethodinfoarray: *mut EAP_METHOD_INFO_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetMethods(peapmethodinfoarray: *mut EAP_METHOD_INFO_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetMethods(::core::mem::transmute(peapmethodinfoarray), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerGetResponseAttributes(sessionhandle: u32, pattribs: *mut EAP_ATTRIBUTES, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetResponseAttributes(sessionhandle: u32, pattribs: *mut EAP_ATTRIBUTES, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetResponseAttributes(sessionhandle, ::core::mem::transmute(pattribs), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerGetResult(sessionhandle: u32, reason: EapHostPeerMethodResultReason, ppresult: *mut EapHostPeerMethodResult, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetResult(sessionhandle: u32, reason: EapHostPeerMethodResultReason, ppresult: *mut EapHostPeerMethodResult, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetResult(sessionhandle, reason, ::core::mem::transmute(ppresult), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerGetSendPacket(sessionhandle: u32, pcbsendpacket: *mut u32, ppsendpacket: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetSendPacket(sessionhandle: u32, pcbsendpacket: *mut u32, ppsendpacket: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetSendPacket(sessionhandle, ::core::mem::transmute(pcbsendpacket), ::core::mem::transmute(ppsendpacket), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerGetUIContext(sessionhandle: u32, pdwsizeofuicontextdata: *mut u32, ppuicontextdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerGetUIContext(sessionhandle: u32, pdwsizeofuicontextdata: *mut u32, ppuicontextdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerGetUIContext(sessionhandle, ::core::mem::transmute(pdwsizeofuicontextdata), ::core::mem::transmute(ppuicontextdata), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerInitialize() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerInitialize() -> u32;
    }
    EapHostPeerInitialize()
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerInvokeConfigUI<'a, P0>(hwndparent: P0, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, pconfigin: &[u8], pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerInvokeConfigUI(hwndparent: super::super::Foundation::HWND, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerInvokeConfigUI(hwndparent.into(), dwflags, ::core::mem::transmute(eapmethodtype), pconfigin.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconfigin)), ::core::mem::transmute(pdwsizeofconfigout), ::core::mem::transmute(ppconfigout), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerInvokeIdentityUI<'a, P0>(dwversion: u32, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, hwndparent: P0, pconnectiondata: &[u8], puserdata: &[u8], pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut ::windows::core::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerInvokeIdentityUI(dwversion: u32, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, hwndparent: super::super::Foundation::HWND, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut ::windows::core::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    }
    EapHostPeerInvokeIdentityUI(dwversion, ::core::mem::transmute(eapmethodtype), dwflags, hwndparent.into(), pconnectiondata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondata)), puserdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puserdata)), ::core::mem::transmute(pdwsizeofuserdataout), ::core::mem::transmute(ppuserdataout), ::core::mem::transmute(ppwszidentity), ::core::mem::transmute(ppeaperror), ::core::mem::transmute(ppvreserved))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerInvokeInteractiveUI<'a, P0>(hwndparent: P0, puicontextdata: &[u8], pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerInvokeInteractiveUI(hwndparent: super::super::Foundation::HWND, dwsizeofuicontextdata: u32, puicontextdata: *const u8, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerInvokeInteractiveUI(hwndparent.into(), puicontextdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puicontextdata)), ::core::mem::transmute(pdwsizeofdatafrominteractiveui), ::core::mem::transmute(ppdatafrominteractiveui), ::core::mem::transmute(ppeaperror))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
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
impl ::core::marker::Copy for EapHostPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapHostPeerMethodResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EapHostPeerMethodResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapHostPeerMethodResult")
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
unsafe impl ::windows::core::Abi for EapHostPeerMethodResult {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EapHostPeerMethodResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapHostPeerMethodResult>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EapHostPeerMethodResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EapHostPeerMethodResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapHostPeerMethodResultReason(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerMethodResultAltSuccessReceived: EapHostPeerMethodResultReason = EapHostPeerMethodResultReason(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerMethodResultTimeout: EapHostPeerMethodResultReason = EapHostPeerMethodResultReason(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerMethodResultFromMethod: EapHostPeerMethodResultReason = EapHostPeerMethodResultReason(3i32);
impl ::core::marker::Copy for EapHostPeerMethodResultReason {}
impl ::core::clone::Clone for EapHostPeerMethodResultReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapHostPeerMethodResultReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapHostPeerMethodResultReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapHostPeerMethodResultReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapHostPeerMethodResultReason").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerProcessReceivedPacket(sessionhandle: u32, cbreceivepacket: u32, preceivepacket: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerProcessReceivedPacket(sessionhandle: u32, cbreceivepacket: u32, preceivepacket: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerProcessReceivedPacket(sessionhandle, cbreceivepacket, ::core::mem::transmute(preceivepacket), ::core::mem::transmute(peapoutput), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerQueryCredentialInputFields<'a, P0>(huserimpersonationtoken: P0, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, pbeapconndata: &[u8], peapconfiginputfieldarray: *mut EAP_CONFIG_INPUT_FIELD_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerQueryCredentialInputFields(huserimpersonationtoken: super::super::Foundation::HANDLE, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, dweapconndatasize: u32, pbeapconndata: *const u8, peapconfiginputfieldarray: *mut EAP_CONFIG_INPUT_FIELD_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerQueryCredentialInputFields(huserimpersonationtoken.into(), ::core::mem::transmute(eapmethodtype), dwflags, pbeapconndata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbeapconndata)), ::core::mem::transmute(peapconfiginputfieldarray), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerQueryInteractiveUIInputFields(dwversion: u32, dwflags: u32, puicontextdata: &[u8], peapinteractiveuidata: *mut EAP_INTERACTIVE_UI_DATA, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerQueryInteractiveUIInputFields(dwversion: u32, dwflags: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapinteractiveuidata: *mut EAP_INTERACTIVE_UI_DATA, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    }
    EapHostPeerQueryInteractiveUIInputFields(dwversion, dwflags, puicontextdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puicontextdata)), ::core::mem::transmute(peapinteractiveuidata), ::core::mem::transmute(ppeaperror), ::core::mem::transmute(ppvreserved))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(dwversion: u32, dwflags: u32, puicontextdata: &[u8], peapinteractiveuidata: *const EAP_INTERACTIVE_UI_DATA, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(dwversion: u32, dwflags: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapinteractiveuidata: *const EAP_INTERACTIVE_UI_DATA, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    }
    EapHostPeerQueryUIBlobFromInteractiveUIInputFields(dwversion, dwflags, puicontextdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puicontextdata)), ::core::mem::transmute(peapinteractiveuidata), ::core::mem::transmute(pdwsizeofdatafrominteractiveui), ::core::mem::transmute(ppdatafrominteractiveui), ::core::mem::transmute(ppeaperror), ::core::mem::transmute(ppvreserved))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EapHostPeerQueryUserBlobFromCredentialInputFields<'a, P0>(huserimpersonationtoken: P0, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, pbeapconndata: &[u8], peapconfiginputfieldarray: *const EAP_CONFIG_INPUT_FIELD_ARRAY, pdwuserblobsize: *mut u32, ppbuserblob: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerQueryUserBlobFromCredentialInputFields(huserimpersonationtoken: super::super::Foundation::HANDLE, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, dweapconndatasize: u32, pbeapconndata: *const u8, peapconfiginputfieldarray: *const EAP_CONFIG_INPUT_FIELD_ARRAY, pdwuserblobsize: *mut u32, ppbuserblob: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerQueryUserBlobFromCredentialInputFields(huserimpersonationtoken.into(), ::core::mem::transmute(eapmethodtype), dwflags, pbeapconndata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbeapconndata)), ::core::mem::transmute(peapconfiginputfieldarray), ::core::mem::transmute(pdwuserblobsize), ::core::mem::transmute(ppbuserblob), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapHostPeerResponseAction(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseDiscard: EapHostPeerResponseAction = EapHostPeerResponseAction(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseSend: EapHostPeerResponseAction = EapHostPeerResponseAction(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseResult: EapHostPeerResponseAction = EapHostPeerResponseAction(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseInvokeUi: EapHostPeerResponseAction = EapHostPeerResponseAction(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseRespond: EapHostPeerResponseAction = EapHostPeerResponseAction(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseStartAuthentication: EapHostPeerResponseAction = EapHostPeerResponseAction(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapHostPeerResponseNone: EapHostPeerResponseAction = EapHostPeerResponseAction(6i32);
impl ::core::marker::Copy for EapHostPeerResponseAction {}
impl ::core::clone::Clone for EapHostPeerResponseAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapHostPeerResponseAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapHostPeerResponseAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapHostPeerResponseAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapHostPeerResponseAction").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerSetResponseAttributes(sessionhandle: u32, pattribs: *const EAP_ATTRIBUTES, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerSetResponseAttributes(sessionhandle: u32, pattribs: *const EAP_ATTRIBUTES, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerSetResponseAttributes(sessionhandle, ::core::mem::transmute(pattribs), ::core::mem::transmute(peapoutput), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerSetUIContext(sessionhandle: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerSetUIContext(sessionhandle: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    }
    EapHostPeerSetUIContext(sessionhandle, dwsizeofuicontextdata, ::core::mem::transmute(puicontextdata), ::core::mem::transmute(peapoutput), ::core::mem::transmute(ppeaperror))
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[inline]
pub unsafe fn EapHostPeerUninitialize() {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EapHostPeerUninitialize();
    }
    EapHostPeerUninitialize()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EapPacket {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for EapPacket {}
impl ::core::clone::Clone for EapPacket {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EapPacket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapPacket").field("Code", &self.Code).field("Id", &self.Id).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for EapPacket {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EapPacket {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapPacket>()) == 0 }
    }
}
impl ::core::cmp::Eq for EapPacket {}
impl ::core::default::Default for EapPacket {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodOutput {
    pub action: EapPeerMethodResponseAction,
    pub fAllowNotifications: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EapPeerMethodOutput {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EapPeerMethodOutput {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EapPeerMethodOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapPeerMethodOutput").field("action", &self.action).field("fAllowNotifications", &self.fAllowNotifications).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EapPeerMethodOutput {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EapPeerMethodOutput {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapPeerMethodOutput>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EapPeerMethodOutput {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EapPeerMethodOutput {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapPeerMethodResponseAction(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResponseActionDiscard: EapPeerMethodResponseAction = EapPeerMethodResponseAction(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResponseActionSend: EapPeerMethodResponseAction = EapPeerMethodResponseAction(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResponseActionResult: EapPeerMethodResponseAction = EapPeerMethodResponseAction(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResponseActionInvokeUI: EapPeerMethodResponseAction = EapPeerMethodResponseAction(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResponseActionRespond: EapPeerMethodResponseAction = EapPeerMethodResponseAction(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResponseActionNone: EapPeerMethodResponseAction = EapPeerMethodResponseAction(5i32);
impl ::core::marker::Copy for EapPeerMethodResponseAction {}
impl ::core::clone::Clone for EapPeerMethodResponseAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapPeerMethodResponseAction {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapPeerMethodResponseAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapPeerMethodResponseAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapPeerMethodResponseAction").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for EapPeerMethodResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for EapPeerMethodResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for EapPeerMethodResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapPeerMethodResult")
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for EapPeerMethodResult {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for EapPeerMethodResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapPeerMethodResult>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for EapPeerMethodResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for EapPeerMethodResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EapPeerMethodResultReason(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResultUnknown: EapPeerMethodResultReason = EapPeerMethodResultReason(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResultSuccess: EapPeerMethodResultReason = EapPeerMethodResultReason(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EapPeerMethodResultFailure: EapPeerMethodResultReason = EapPeerMethodResultReason(3i32);
impl ::core::marker::Copy for EapPeerMethodResultReason {}
impl ::core::clone::Clone for EapPeerMethodResultReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EapPeerMethodResultReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EapPeerMethodResultReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for EapPeerMethodResultReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapPeerMethodResultReason").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EapSimCredential {
    pub iccID: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for EapSimCredential {}
impl ::core::clone::Clone for EapSimCredential {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EapSimCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapSimCredential").field("iccID", &self.iccID).finish()
    }
}
unsafe impl ::windows::core::Abi for EapSimCredential {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EapSimCredential {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapSimCredential>()) == 0 }
    }
}
impl ::core::cmp::Eq for EapSimCredential {}
impl ::core::default::Default for EapSimCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct EapUsernamePasswordCredential {
    pub username: ::windows::core::PWSTR,
    pub password: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for EapUsernamePasswordCredential {}
impl ::core::clone::Clone for EapUsernamePasswordCredential {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EapUsernamePasswordCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapUsernamePasswordCredential").field("username", &self.username).field("password", &self.password).finish()
    }
}
unsafe impl ::windows::core::Abi for EapUsernamePasswordCredential {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for EapUsernamePasswordCredential {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EapUsernamePasswordCredential>()) == 0 }
    }
}
impl ::core::cmp::Eq for EapUsernamePasswordCredential {}
impl ::core::default::Default for EapUsernamePasswordCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const FACILITY_EAP_MESSAGE: u32 = 2114u32;
pub const GUID_EapHost_Cause_CertStoreInaccessible: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000004);
pub const GUID_EapHost_Cause_EapNegotiationFailed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001c);
pub const GUID_EapHost_Cause_EapQecInaccessible: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000312);
pub const GUID_EapHost_Cause_Generic_AuthFailure: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000104);
pub const GUID_EapHost_Cause_IdentityUnknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000204);
pub const GUID_EapHost_Cause_MethodDLLNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000001);
pub const GUID_EapHost_Cause_MethodDoesNotSupportOperation: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001e);
pub const GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda18bd32_004f_41fa_ae08_0bc85e5845ac);
pub const GUID_EapHost_Cause_No_SmartCardReader_Found: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002b);
pub const GUID_EapHost_Cause_Server_CertExpired: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000005);
pub const GUID_EapHost_Cause_Server_CertInvalid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000006);
pub const GUID_EapHost_Cause_Server_CertNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000007);
pub const GUID_EapHost_Cause_Server_CertOtherError: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000108);
pub const GUID_EapHost_Cause_Server_CertRevoked: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000008);
pub const GUID_EapHost_Cause_Server_Root_CertNameRequired: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000012);
pub const GUID_EapHost_Cause_Server_Root_CertNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000112);
pub const GUID_EapHost_Cause_SimNotValid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000304);
pub const GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000212);
pub const GUID_EapHost_Cause_User_Account_OtherProblem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000010e);
pub const GUID_EapHost_Cause_User_CertExpired: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000009);
pub const GUID_EapHost_Cause_User_CertInvalid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000a);
pub const GUID_EapHost_Cause_User_CertNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000b);
pub const GUID_EapHost_Cause_User_CertOtherError: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000c);
pub const GUID_EapHost_Cause_User_CertRejected: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000d);
pub const GUID_EapHost_Cause_User_CertRevoked: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000e);
pub const GUID_EapHost_Cause_User_CredsRejected: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000020e);
pub const GUID_EapHost_Cause_User_Root_CertExpired: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000f);
pub const GUID_EapHost_Cause_User_Root_CertInvalid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000010);
pub const GUID_EapHost_Cause_User_Root_CertNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000011);
pub const GUID_EapHost_Cause_XmlMalformed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001d);
pub const GUID_EapHost_Default: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_EapHost_Help_ObtainingCerts: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf535eea3_1bdd_46ca_a2fc_a6655939b7e8);
pub const GUID_EapHost_Help_Troubleshooting: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33307acf_0698_41ba_b014_ea0a2eb8d0a8);
pub const GUID_EapHost_Repair_ContactAdmin_AuthFailure: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001f);
pub const GUID_EapHost_Repair_ContactAdmin_CertNameAbsent: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000029);
pub const GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000024);
pub const GUID_EapHost_Repair_ContactAdmin_IdentityUnknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000020);
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000025);
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserCert: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002c);
pub const GUID_EapHost_Repair_ContactAdmin_MethodNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000022);
pub const GUID_EapHost_Repair_ContactAdmin_NegotiationFailed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000021);
pub const GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002a);
pub const GUID_EapHost_Repair_ContactAdmin_RootCertInvalid: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000026);
pub const GUID_EapHost_Repair_ContactAdmin_RootCertNotFound: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000027);
pub const GUID_EapHost_Repair_ContactAdmin_RootExpired: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000028);
pub const GUID_EapHost_Repair_ContactSysadmin: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000002);
pub const GUID_EapHost_Repair_Method_Not_Support_Sso: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002d);
pub const GUID_EapHost_Repair_No_ValidSim_Found: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002e);
pub const GUID_EapHost_Repair_RestartNap: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000023);
pub const GUID_EapHost_Repair_Retry_Authentication: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000011b);
pub const GUID_EapHost_Repair_Server_ClientSelectServerCert: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000018);
pub const GUID_EapHost_Repair_User_AuthFailure: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000019);
pub const GUID_EapHost_Repair_User_GetNewCert: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001a);
pub const GUID_EapHost_Repair_User_SelectValidCert: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001b);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
pub struct IAccountingProviderConfig(::windows::core::IUnknown);
impl IAccountingProviderConfig {
    pub unsafe fn Initialize<'a, P0>(&self, pszmachinename: P0) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pszmachinename.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Uninitialize)(::windows::core::Interface::as_raw(self), uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configure<'a, P0>(&self, uconnectionparam: usize, hwnd: P0, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Configure)(::windows::core::Interface::as_raw(self), uconnectionparam, hwnd.into(), dwflags, ureserved1, ureserved2).ok()
    }
    pub unsafe fn Activate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), uconnectionparam, ureserved1, ureserved2).ok()
    }
    pub unsafe fn Deactivate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self), uconnectionparam, ureserved1, ureserved2).ok()
    }
}
impl ::core::convert::From<IAccountingProviderConfig> for ::windows::core::IUnknown {
    fn from(value: IAccountingProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAccountingProviderConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAccountingProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAccountingProviderConfig> for ::windows::core::IUnknown {
    fn from(value: &IAccountingProviderConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAccountingProviderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAccountingProviderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccountingProviderConfig {}
impl ::core::fmt::Debug for IAccountingProviderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccountingProviderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAccountingProviderConfig {
    type Vtable = IAccountingProviderConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66a2db18_d706_11d0_a37b_00c04fc9da04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountingProviderConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Configure: usize,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
pub struct IAuthenticationProviderConfig(::windows::core::IUnknown);
impl IAuthenticationProviderConfig {
    pub unsafe fn Initialize<'a, P0>(&self, pszmachinename: P0) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pszmachinename.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Uninitialize)(::windows::core::Interface::as_raw(self), uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Configure<'a, P0>(&self, uconnectionparam: usize, hwnd: P0, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).Configure)(::windows::core::Interface::as_raw(self), uconnectionparam, hwnd.into(), dwflags, ureserved1, ureserved2).ok()
    }
    pub unsafe fn Activate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Activate)(::windows::core::Interface::as_raw(self), uconnectionparam, ureserved1, ureserved2).ok()
    }
    pub unsafe fn Deactivate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Deactivate)(::windows::core::Interface::as_raw(self), uconnectionparam, ureserved1, ureserved2).ok()
    }
}
impl ::core::convert::From<IAuthenticationProviderConfig> for ::windows::core::IUnknown {
    fn from(value: IAuthenticationProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAuthenticationProviderConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAuthenticationProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAuthenticationProviderConfig> for ::windows::core::IUnknown {
    fn from(value: &IAuthenticationProviderConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAuthenticationProviderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAuthenticationProviderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAuthenticationProviderConfig {}
impl ::core::fmt::Debug for IAuthenticationProviderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAuthenticationProviderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAuthenticationProviderConfig {
    type Vtable = IAuthenticationProviderConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66a2db17_d706_11d0_a37b_00c04fc9da04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAuthenticationProviderConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Configure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Configure: usize,
    pub Activate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
    pub Deactivate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
pub struct IEAPProviderConfig(::windows::core::IUnknown);
impl IEAPProviderConfig {
    pub unsafe fn Initialize<'a, P0>(&self, pszmachinename: P0, dweaptypeid: u32) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pszmachinename.into(), dweaptypeid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Uninitialize)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).ServerInvokeConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ureserved1, ureserved2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).RouterInvokeConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondatain)), pconnectiondatain.len() as _, ::core::mem::transmute(ppconnectiondataout), ::core::mem::transmute(pdwsizeofconnectiondataout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], puserdatain: &[u8], ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).RouterInvokeCredentialsUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondatain)), pconnectiondatain.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puserdatain)), puserdatain.len() as _, ::core::mem::transmute(ppuserdataout), ::core::mem::transmute(pdwsizeofuserdataout)).ok()
    }
}
impl ::core::convert::From<IEAPProviderConfig> for ::windows::core::IUnknown {
    fn from(value: IEAPProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEAPProviderConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEAPProviderConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEAPProviderConfig> for ::windows::core::IUnknown {
    fn from(value: &IEAPProviderConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEAPProviderConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEAPProviderConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEAPProviderConfig {}
impl ::core::fmt::Debug for IEAPProviderConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEAPProviderConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEAPProviderConfig {
    type Vtable = IEAPProviderConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66a2db19_d706_11d0_a37b_00c04fc9da04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEAPProviderConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, dweaptypeid: u32, puconnectionparam: *mut usize) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ServerInvokeConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServerInvokeConfigUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RouterInvokeConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RouterInvokeConfigUI: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RouterInvokeCredentialsUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RouterInvokeCredentialsUI: usize,
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
pub struct IEAPProviderConfig2(::windows::core::IUnknown);
impl IEAPProviderConfig2 {
    pub unsafe fn Initialize<'a, P0>(&self, pszmachinename: P0, dweaptypeid: u32) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), pszmachinename.into(), dweaptypeid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Uninitialize)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.ServerInvokeConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ureserved1, ureserved2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.RouterInvokeConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondatain)), pconnectiondatain.len() as _, ::core::mem::transmute(ppconnectiondataout), ::core::mem::transmute(pdwsizeofconnectiondataout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], puserdatain: &[u8], ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.RouterInvokeCredentialsUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondatain)), pconnectiondatain.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puserdatain)), puserdatain.len() as _, ::core::mem::transmute(ppuserdataout), ::core::mem::transmute(pdwsizeofuserdataout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI2<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).ServerInvokeConfigUI2)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ::core::mem::transmute(pconfigdatain), dwsizeofconfigdatain, ::core::mem::transmute(ppconfigdataout), ::core::mem::transmute(pdwsizeofconfigdataout)).ok()
    }
    pub unsafe fn GetGlobalConfig(&self, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGlobalConfig)(::windows::core::Interface::as_raw(self), dweaptypeid, ::core::mem::transmute(ppconfigdataout), ::core::mem::transmute(pdwsizeofconfigdataout)).ok()
    }
}
impl ::core::convert::From<IEAPProviderConfig2> for ::windows::core::IUnknown {
    fn from(value: IEAPProviderConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEAPProviderConfig2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEAPProviderConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEAPProviderConfig2> for ::windows::core::IUnknown {
    fn from(value: &IEAPProviderConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEAPProviderConfig2> for IEAPProviderConfig {
    fn from(value: IEAPProviderConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEAPProviderConfig2> for &'a IEAPProviderConfig {
    fn from(value: &'a IEAPProviderConfig2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEAPProviderConfig2> for IEAPProviderConfig {
    fn from(value: &IEAPProviderConfig2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEAPProviderConfig2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEAPProviderConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEAPProviderConfig2 {}
impl ::core::fmt::Debug for IEAPProviderConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEAPProviderConfig2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEAPProviderConfig2 {
    type Vtable = IEAPProviderConfig2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd565917a_85c4_4466_856e_671c3742ea9a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEAPProviderConfig2_Vtbl {
    pub base__: IEAPProviderConfig_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ServerInvokeConfigUI2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServerInvokeConfigUI2: usize,
    pub GetGlobalConfig: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
pub struct IEAPProviderConfig3(::windows::core::IUnknown);
impl IEAPProviderConfig3 {
    pub unsafe fn Initialize<'a, P0>(&self, pszmachinename: P0, dweaptypeid: u32) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.Initialize)(::windows::core::Interface::as_raw(self), pszmachinename.into(), dweaptypeid, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<usize>(result__)
    }
    pub unsafe fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Uninitialize)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.ServerInvokeConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ureserved1, ureserved2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RouterInvokeConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondatain)), pconnectiondatain.len() as _, ::core::mem::transmute(ppconnectiondataout), ::core::mem::transmute(pdwsizeofconnectiondataout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], puserdatain: &[u8], ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.RouterInvokeCredentialsUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(::windows::core::as_ptr_or_null(pconnectiondatain)), pconnectiondatain.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(puserdatain)), puserdatain.len() as _, ::core::mem::transmute(ppuserdataout), ::core::mem::transmute(pdwsizeofuserdataout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI2<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).base__.ServerInvokeConfigUI2)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ::core::mem::transmute(pconfigdatain), dwsizeofconfigdatain, ::core::mem::transmute(ppconfigdataout), ::core::mem::transmute(pdwsizeofconfigdataout)).ok()
    }
    pub unsafe fn GetGlobalConfig(&self, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetGlobalConfig)(::windows::core::Interface::as_raw(self), dweaptypeid, ::core::mem::transmute(ppconfigdataout), ::core::mem::transmute(pdwsizeofconfigdataout)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeCertificateConfigUI<'a, P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).ServerInvokeCertificateConfigUI)(::windows::core::Interface::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ::core::mem::transmute(pconfigdatain), dwsizeofconfigdatain, ::core::mem::transmute(ppconfigdataout), ::core::mem::transmute(pdwsizeofconfigdataout), ureserved).ok()
    }
}
impl ::core::convert::From<IEAPProviderConfig3> for ::windows::core::IUnknown {
    fn from(value: IEAPProviderConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEAPProviderConfig3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEAPProviderConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEAPProviderConfig3> for ::windows::core::IUnknown {
    fn from(value: &IEAPProviderConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEAPProviderConfig3> for IEAPProviderConfig {
    fn from(value: IEAPProviderConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEAPProviderConfig3> for &'a IEAPProviderConfig {
    fn from(value: &'a IEAPProviderConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEAPProviderConfig3> for IEAPProviderConfig {
    fn from(value: &IEAPProviderConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IEAPProviderConfig3> for IEAPProviderConfig2 {
    fn from(value: IEAPProviderConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEAPProviderConfig3> for &'a IEAPProviderConfig2 {
    fn from(value: &'a IEAPProviderConfig3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEAPProviderConfig3> for IEAPProviderConfig2 {
    fn from(value: &IEAPProviderConfig3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEAPProviderConfig3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEAPProviderConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEAPProviderConfig3 {}
impl ::core::fmt::Debug for IEAPProviderConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEAPProviderConfig3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEAPProviderConfig3 {
    type Vtable = IEAPProviderConfig3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb78ecd12_68bb_4f86_9bf0_8438dd3be982);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEAPProviderConfig3_Vtbl {
    pub base__: IEAPProviderConfig2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ServerInvokeCertificateConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ServerInvokeCertificateConfigUI: usize,
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
pub struct IRouterProtocolConfig(::windows::core::IUnknown);
impl IRouterProtocolConfig {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddProtocol<'a, P0, P1, P2>(&self, pszmachinename: P0, dwtransportid: u32, dwprotocolid: u32, hwnd: P1, dwflags: u32, prouter: P2, ureserved1: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).AddProtocol)(::windows::core::Interface::as_raw(self), pszmachinename.into(), dwtransportid, dwprotocolid, hwnd.into(), dwflags, prouter.into().abi(), ureserved1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveProtocol<'a, P0, P1, P2>(&self, pszmachinename: P0, dwtransportid: u32, dwprotocolid: u32, hwnd: P1, dwflags: u32, prouter: P2, ureserved1: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).RemoveProtocol)(::windows::core::Interface::as_raw(self), pszmachinename.into(), dwtransportid, dwprotocolid, hwnd.into(), dwflags, prouter.into().abi(), ureserved1).ok()
    }
}
impl ::core::convert::From<IRouterProtocolConfig> for ::windows::core::IUnknown {
    fn from(value: IRouterProtocolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRouterProtocolConfig> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRouterProtocolConfig) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRouterProtocolConfig> for ::windows::core::IUnknown {
    fn from(value: &IRouterProtocolConfig) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRouterProtocolConfig {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRouterProtocolConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRouterProtocolConfig {}
impl ::core::fmt::Debug for IRouterProtocolConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRouterProtocolConfig").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRouterProtocolConfig {
    type Vtable = IRouterProtocolConfig_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66a2db16_d706_11d0_a37b_00c04fc9da04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRouterProtocolConfig_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddProtocol: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveProtocol: usize,
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ISOLATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const ISOLATION_STATE_UNKNOWN: ISOLATION_STATE = ISOLATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const ISOLATION_STATE_NOT_RESTRICTED: ISOLATION_STATE = ISOLATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const ISOLATION_STATE_IN_PROBATION: ISOLATION_STATE = ISOLATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const ISOLATION_STATE_RESTRICTED_ACCESS: ISOLATION_STATE = ISOLATION_STATE(3i32);
impl ::core::marker::Copy for ISOLATION_STATE {}
impl ::core::clone::Clone for ISOLATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ISOLATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ISOLATION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ISOLATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOLATION_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct LEGACY_IDENTITY_UI_PARAMS {
    pub eapType: u32,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: ::windows::core::PWSTR,
    pub dwError: u32,
}
impl ::core::marker::Copy for LEGACY_IDENTITY_UI_PARAMS {}
impl ::core::clone::Clone for LEGACY_IDENTITY_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LEGACY_IDENTITY_UI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LEGACY_IDENTITY_UI_PARAMS")
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
unsafe impl ::windows::core::Abi for LEGACY_IDENTITY_UI_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LEGACY_IDENTITY_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LEGACY_IDENTITY_UI_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for LEGACY_IDENTITY_UI_PARAMS {}
impl ::core::default::Default for LEGACY_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct LEGACY_INTERACTIVE_UI_PARAMS {
    pub eapType: u32,
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
}
impl ::core::marker::Copy for LEGACY_INTERACTIVE_UI_PARAMS {}
impl ::core::clone::Clone for LEGACY_INTERACTIVE_UI_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LEGACY_INTERACTIVE_UI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LEGACY_INTERACTIVE_UI_PARAMS").field("eapType", &self.eapType).field("dwSizeofContextData", &self.dwSizeofContextData).field("pContextData", &self.pContextData).field("dwSizeofInteractiveUIData", &self.dwSizeofInteractiveUIData).field("pInteractiveUIData", &self.pInteractiveUIData).field("dwError", &self.dwError).finish()
    }
}
unsafe impl ::windows::core::Abi for LEGACY_INTERACTIVE_UI_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LEGACY_INTERACTIVE_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LEGACY_INTERACTIVE_UI_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for LEGACY_INTERACTIVE_UI_PARAMS {}
impl ::core::default::Default for LEGACY_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const MAXEAPCODE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH: u32 = 90u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct NgcTicketContext {
    pub wszTicket: [u16; 45],
    pub hKey: super::Cryptography::NCRYPT_KEY_HANDLE,
    pub hImpersonateToken: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for NgcTicketContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for NgcTicketContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for NgcTicketContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NgcTicketContext").field("wszTicket", &self.wszTicket).field("hKey", &self.hKey).field("hImpersonateToken", &self.hImpersonateToken).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for NgcTicketContext {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for NgcTicketContext {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NgcTicketContext>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for NgcTicketContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for NgcTicketContext {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub type NotificationHandler = ::core::option::Option<unsafe extern "system" fn(connectionid: ::windows::core::GUID, pcontextdata: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PPP_EAP_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_NoAction: PPP_EAP_ACTION = PPP_EAP_ACTION(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_Authenticate: PPP_EAP_ACTION = PPP_EAP_ACTION(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_Done: PPP_EAP_ACTION = PPP_EAP_ACTION(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_SendAndDone: PPP_EAP_ACTION = PPP_EAP_ACTION(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_Send: PPP_EAP_ACTION = PPP_EAP_ACTION(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_SendWithTimeout: PPP_EAP_ACTION = PPP_EAP_ACTION(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_SendWithTimeoutInteractive: PPP_EAP_ACTION = PPP_EAP_ACTION(6i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_IndicateTLV: PPP_EAP_ACTION = PPP_EAP_ACTION(7i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const EAPACTION_IndicateIdentity: PPP_EAP_ACTION = PPP_EAP_ACTION(8i32);
impl ::core::marker::Copy for PPP_EAP_ACTION {}
impl ::core::clone::Clone for PPP_EAP_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PPP_EAP_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PPP_EAP_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for PPP_EAP_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PPP_EAP_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct PPP_EAP_INFO {
    pub dwSizeInBytes: u32,
    pub dwEapTypeId: u32,
    pub RasEapInitialize: isize,
    pub RasEapBegin: isize,
    pub RasEapEnd: isize,
    pub RasEapMakeMessage: isize,
}
impl ::core::marker::Copy for PPP_EAP_INFO {}
impl ::core::clone::Clone for PPP_EAP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_EAP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_EAP_INFO").field("dwSizeInBytes", &self.dwSizeInBytes).field("dwEapTypeId", &self.dwEapTypeId).field("RasEapInitialize", &self.RasEapInitialize).field("RasEapBegin", &self.RasEapBegin).field("RasEapEnd", &self.RasEapEnd).field("RasEapMakeMessage", &self.RasEapMakeMessage).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_EAP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_EAP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_EAP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_EAP_INFO {}
impl ::core::default::Default for PPP_EAP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_INPUT {
    pub dwSizeInBytes: u32,
    pub fFlags: u32,
    pub fAuthenticator: super::super::Foundation::BOOL,
    pub pwszIdentity: ::windows::core::PWSTR,
    pub pwszPassword: ::windows::core::PWSTR,
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
    pub guidConnectionId: ::windows::core::GUID,
    pub isVpn: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PPP_EAP_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PPP_EAP_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PPP_EAP_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_EAP_INPUT")
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
            .field("fDataReceivedFromInteractiveUI", &self.fDataReceivedFromInteractiveUI)
            .field("pDataFromInteractiveUI", &self.pDataFromInteractiveUI)
            .field("dwSizeOfDataFromInteractiveUI", &self.dwSizeOfDataFromInteractiveUI)
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
unsafe impl ::windows::core::Abi for PPP_EAP_INPUT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PPP_EAP_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_EAP_INPUT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PPP_EAP_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PPP_EAP_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for PPP_EAP_OUTPUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for PPP_EAP_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PPP_EAP_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_EAP_OUTPUT")
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for PPP_EAP_OUTPUT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PPP_EAP_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_EAP_OUTPUT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PPP_EAP_OUTPUT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PPP_EAP_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct PPP_EAP_PACKET {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for PPP_EAP_PACKET {}
impl ::core::clone::Clone for PPP_EAP_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_EAP_PACKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_EAP_PACKET").field("Code", &self.Code).field("Id", &self.Id).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_EAP_PACKET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_EAP_PACKET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_EAP_PACKET>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_EAP_PACKET {}
impl ::core::default::Default for PPP_EAP_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub struct RAS_AUTH_ATTRIBUTE {
    pub raaType: RAS_AUTH_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub Value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RAS_AUTH_ATTRIBUTE {}
impl ::core::clone::Clone for RAS_AUTH_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAS_AUTH_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_AUTH_ATTRIBUTE").field("raaType", &self.raaType).field("dwLength", &self.dwLength).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for RAS_AUTH_ATTRIBUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAS_AUTH_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_AUTH_ATTRIBUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAS_AUTH_ATTRIBUTE {}
impl ::core::default::Default for RAS_AUTH_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RAS_AUTH_ATTRIBUTE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatMinimum: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatUserName: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatUserPassword: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatMD5CHAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatNASIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatNASPort: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatServiceType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedProtocol: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedIPNetmask: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedRouting: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFilterId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedMTU: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedCompression: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginIPHost: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginService: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginTCPPort: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatUnassigned17: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatReplyMessage: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCallbackNumber: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCallbackId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatUnassigned21: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedRoute: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedIPXNetwork: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatState: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatClass: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatVendorSpecific: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatSessionTimeout: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatIdleTimeout: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatTerminationAction: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCalledStationId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCallingStationId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatNASIdentifier: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatProxyState: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginLATService: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginLATNode: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginLATGroup: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedAppleTalkLink: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedAppleTalkNetwork: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedAppleTalkZone: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctStatusType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctDelayTime: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctInputOctets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctOutputOctets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctSessionId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctAuthentic: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctSessionTime: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctInputPackets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctOutputPackets: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctTerminateCause: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctMultiSessionId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(50i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctLinkCount: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(51i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctEventTimeStamp: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(55i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatMD5CHAPChallenge: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(60i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatNASPortType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(61i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatPortLimit: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(62i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginLATPort: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(63i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatTunnelType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatTunnelMediumType: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(65i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatTunnelClientEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(66i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatTunnelServerEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(67i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(70i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPFeatures: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(71i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPZoneAccess: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(72i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPSecurity: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(73i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPSecurityData: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(74i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatPasswordRetry: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(75i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatPrompt: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(76i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatConnectInfo: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(77i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatConfigurationToken: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(78i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatEAPMessage: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(79i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatSignature: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(80i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPChallengeResponse: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(84i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatAcctInterimInterval: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(85i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatNASIPv6Address: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(95i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedInterfaceId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(96i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedIPv6Prefix: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(97i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatLoginIPv6Host: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(98i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedIPv6Route: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(99i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFramedIPv6Pool: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(100i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPGuestLogon: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8096i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCertificateOID: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8097i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatEAPConfiguration: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8098i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatPEAPEmbeddedEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8099i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatInnerEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8099i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatPEAPFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8100i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8100i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatEAPTLV: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8102i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCredentialsChanged: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8103i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatCertificateThumbprint: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(8250i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatPeerId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9000i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatServerId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9001i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatMethodId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9002i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatEMSK: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9003i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatSessionId: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(9004i32);
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatReserved: RAS_AUTH_ATTRIBUTE_TYPE = RAS_AUTH_ATTRIBUTE_TYPE(-1i32);
impl ::core::marker::Copy for RAS_AUTH_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for RAS_AUTH_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAS_AUTH_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAS_AUTH_ATTRIBUTE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAS_AUTH_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_AUTH_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_8021X_AUTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_ALTERNATIVE_USER_DB: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_FIRST_LINK: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_HOSTED_IN_PEAP: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_LOGON: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_PEAP_FORCE_FULL_AUTH: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_PEAP_UPFRONT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_PREVIEW: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_PRE_LOGON: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_RESERVED: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_ROUTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_SAVE_CREDMAN: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_REGISTRY_LOCATION: &str = "System\\CurrentControlSet\\Services\\Rasman\\PPP\\EAP";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_ROLE_AUTHENTICATEE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_ROLE_AUTHENTICATOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_ROLE_EXCLUDE_IN_EAP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_ROLE_EXCLUDE_IN_PEAP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_ROLE_EXCLUDE_IN_VPN: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_CONFIGUI: &str = "ConfigUIPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_CONFIG_CLSID: &str = "ConfigCLSID";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_DEFAULT_DATA: &str = "ConfigData";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_ENCRYPTION: &str = "MPPEEncryptionSupported";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_FILTER_INNERMETHODS: &str = "FilterInnerMethods";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_FRIENDLY_NAME: &str = "FriendlyName";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_IDENTITY: &str = "IdentityPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_INTERACTIVEUI: &str = "InteractiveUIPath";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_INVOKE_NAMEDLG: &str = "InvokeUsernameDialog";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_INVOKE_PWDDLG: &str = "InvokePasswordDialog";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_ISTUNNEL_METHOD: &str = "IsTunnelMethod";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_PATH: &str = "Path";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_PER_POLICY_CONFIG: &str = "PerPolicyConfig";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_REQUIRE_CONFIGUI: &str = "RequireConfigUI";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_ROLES_SUPPORTED: &str = "RolesSupported";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const RAS_EAP_VALUENAME_STANDALONE_SUPPORTED: &str = "StandaloneSupported";
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropCertifiedMethod: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropChannelBinding: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropCipherSuiteNegotiation: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropConfidentiality: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropCryptoBinding: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropDictionaryAttackResistance: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropFastReconnect: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropFragmentation: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropHiddenMethod: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropIdentityPrivacy: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropIntegrity: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropKeyDerivation: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropKeyStrength1024: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropKeyStrength128: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropKeyStrength256: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropKeyStrength512: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropKeyStrength64: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropMachineAuth: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropMethodChaining: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropMppeEncryption: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropMutualAuth: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropNap: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropReplayProtection: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropReserved: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropSessionIndependence: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropSharedStateEquivalence: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropStandalone: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropSupportsConfig: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropTunnelMethod: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const eapPropUserAuth: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPChallenge: u32 = 33u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPNewPassword: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPOldPassword: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
pub const raatARAPPasswordChangeReason: u32 = 21u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
