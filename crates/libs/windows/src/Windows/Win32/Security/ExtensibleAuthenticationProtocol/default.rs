impl ::core::default::Default for EAPHOST_AUTH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAPHOST_AUTH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.dwErrorCode == other.dwErrorCode && self.dwReasonCode == other.dwReasonCode
    }
}
impl ::core::cmp::Eq for EAPHOST_AUTH_INFO {}
impl ::core::fmt::Debug for EAPHOST_AUTH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAPHOST_AUTH_INFO").field("status", &self.status).field("dwErrorCode", &self.dwErrorCode).field("dwReasonCode", &self.dwReasonCode).finish()
    }
}
impl ::core::default::Default for EAPHOST_AUTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAPHOST_AUTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAPHOST_AUTH_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EAPHOST_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAPHOST_IDENTITY_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.eapMethodType == other.eapMethodType && self.dwFlags == other.dwFlags && self.dwSizeofConnectionData == other.dwSizeofConnectionData && self.pConnectionData == other.pConnectionData && self.dwSizeofUserData == other.dwSizeofUserData && self.pUserData == other.pUserData && self.dwSizeofUserDataOut == other.dwSizeofUserDataOut && self.pUserDataOut == other.pUserDataOut && self.pwszIdentity == other.pwszIdentity && self.dwError == other.dwError && self.pEapError == other.pEapError
    }
}
impl ::core::cmp::Eq for EAPHOST_IDENTITY_UI_PARAMS {}
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
impl ::core::default::Default for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeofContextData == other.dwSizeofContextData && self.pContextData == other.pContextData && self.dwSizeofInteractiveUIData == other.dwSizeofInteractiveUIData && self.pInteractiveUIData == other.pInteractiveUIData && self.dwError == other.dwError && self.pEapError == other.pEapError
    }
}
impl ::core::cmp::Eq for EAPHOST_INTERACTIVE_UI_PARAMS {}
impl ::core::fmt::Debug for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAPHOST_INTERACTIVE_UI_PARAMS").field("dwSizeofContextData", &self.dwSizeofContextData).field("pContextData", &self.pContextData).field("dwSizeofInteractiveUIData", &self.dwSizeofInteractiveUIData).field("pInteractiveUIData", &self.pInteractiveUIData).field("dwError", &self.dwError).field("pEapError", &self.pEapError).finish()
    }
}
impl ::core::default::Default for EAP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.eaType == other.eaType && self.dwLength == other.dwLength && self.pValue == other.pValue
    }
}
impl ::core::cmp::Eq for EAP_ATTRIBUTE {}
impl ::core::fmt::Debug for EAP_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_ATTRIBUTE").field("eaType", &self.eaType).field("dwLength", &self.dwLength).field("pValue", &self.pValue).finish()
    }
}
impl ::core::default::Default for EAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfAttributes == other.dwNumberOfAttributes && self.pAttribs == other.pAttribs
    }
}
impl ::core::cmp::Eq for EAP_ATTRIBUTES {}
impl ::core::fmt::Debug for EAP_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_ATTRIBUTES").field("dwNumberOfAttributes", &self.dwNumberOfAttributes).field("pAttribs", &self.pAttribs).finish()
    }
}
impl ::core::default::Default for EAP_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes
            && self.pEapType == other.pEapType
            && self.EapMethodAuthenticatorInitialize == other.EapMethodAuthenticatorInitialize
            && self.EapMethodAuthenticatorBeginSession == other.EapMethodAuthenticatorBeginSession
            && self.EapMethodAuthenticatorUpdateInnerMethodParams == other.EapMethodAuthenticatorUpdateInnerMethodParams
            && self.EapMethodAuthenticatorReceivePacket == other.EapMethodAuthenticatorReceivePacket
            && self.EapMethodAuthenticatorSendPacket == other.EapMethodAuthenticatorSendPacket
            && self.EapMethodAuthenticatorGetAttributes == other.EapMethodAuthenticatorGetAttributes
            && self.EapMethodAuthenticatorSetAttributes == other.EapMethodAuthenticatorSetAttributes
            && self.EapMethodAuthenticatorGetResult == other.EapMethodAuthenticatorGetResult
            && self.EapMethodAuthenticatorEndSession == other.EapMethodAuthenticatorEndSession
            && self.EapMethodAuthenticatorShutdown == other.EapMethodAuthenticatorShutdown
    }
}
impl ::core::cmp::Eq for EAP_AUTHENTICATOR_METHOD_ROUTINES {}
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
impl ::core::default::Default for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_AUTHENTICATOR_SEND_TIMEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_AUTHENTICATOR_SEND_TIMEOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwNumberOfFields == other.dwNumberOfFields && self.pFields == other.pFields
    }
}
impl ::core::cmp::Eq for EAP_CONFIG_INPUT_FIELD_ARRAY {}
impl ::core::fmt::Debug for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_CONFIG_INPUT_FIELD_ARRAY").field("dwVersion", &self.dwVersion).field("dwNumberOfFields", &self.dwNumberOfFields).field("pFields", &self.pFields).finish()
    }
}
impl ::core::default::Default for EAP_CONFIG_INPUT_FIELD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_CONFIG_INPUT_FIELD_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.Type == other.Type && self.dwFlagProps == other.dwFlagProps && self.pwszLabel == other.pwszLabel && self.pwszData == other.pwszData && self.dwMinDataLength == other.dwMinDataLength && self.dwMaxDataLength == other.dwMaxDataLength
    }
}
impl ::core::cmp::Eq for EAP_CONFIG_INPUT_FIELD_DATA {}
impl ::core::fmt::Debug for EAP_CONFIG_INPUT_FIELD_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_CONFIG_INPUT_FIELD_DATA").field("dwSize", &self.dwSize).field("Type", &self.Type).field("dwFlagProps", &self.dwFlagProps).field("pwszLabel", &self.pwszLabel).field("pwszData", &self.pwszData).field("dwMinDataLength", &self.dwMinDataLength).field("dwMaxDataLength", &self.dwMaxDataLength).finish()
    }
}
impl ::core::default::Default for EAP_CONFIG_INPUT_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_CONFIG_INPUT_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_CONFIG_INPUT_FIELD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EAP_CRED_EXPIRY_REQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_CRED_EXPIRY_REQ {
    fn eq(&self, other: &Self) -> bool {
        self.curCreds == other.curCreds && self.newCreds == other.newCreds
    }
}
impl ::core::cmp::Eq for EAP_CRED_EXPIRY_REQ {}
impl ::core::fmt::Debug for EAP_CRED_EXPIRY_REQ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_CRED_EXPIRY_REQ").field("curCreds", &self.curCreds).field("newCreds", &self.newCreds).finish()
    }
}
impl ::core::default::Default for EAP_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.dwWinError == other.dwWinError && self.r#type == other.r#type && self.dwReasonCode == other.dwReasonCode && self.rootCauseGuid == other.rootCauseGuid && self.repairGuid == other.repairGuid && self.helpLinkGuid == other.helpLinkGuid && self.pRootCauseString == other.pRootCauseString && self.pRepairString == other.pRepairString
    }
}
impl ::core::cmp::Eq for EAP_ERROR {}
impl ::core::fmt::Debug for EAP_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_ERROR").field("dwWinError", &self.dwWinError).field("type", &self.r#type).field("dwReasonCode", &self.dwReasonCode).field("rootCauseGuid", &self.rootCauseGuid).field("repairGuid", &self.repairGuid).field("helpLinkGuid", &self.helpLinkGuid).field("pRootCauseString", &self.pRootCauseString).field("pRepairString", &self.pRepairString).finish()
    }
}
impl ::core::default::Default for EAP_INTERACTIVE_UI_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EAP_INTERACTIVE_UI_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_INTERACTIVE_UI_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_INTERACTIVE_UI_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.fIsSuccess == other.fIsSuccess && self.dwFailureReason == other.dwFailureReason && self.pAuthAttribs == other.pAuthAttribs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_AUTHENTICATOR_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_AUTHENTICATOR_RESULT").field("fIsSuccess", &self.fIsSuccess).field("dwFailureReason", &self.dwFailureReason).field("pAuthAttribs", &self.pAuthAttribs).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.eaptype == other.eaptype && self.pwszAuthorName == other.pwszAuthorName && self.pwszFriendlyName == other.pwszFriendlyName && self.eapProperties == other.eapProperties && self.pInnerMethodInfo == other.pInnerMethodInfo
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO {}
impl ::core::fmt::Debug for EAP_METHOD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO").field("eaptype", &self.eaptype).field("pwszAuthorName", &self.pwszAuthorName).field("pwszFriendlyName", &self.pwszFriendlyName).field("eapProperties", &self.eapProperties).field("pInnerMethodInfo", &self.pInnerMethodInfo).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_INFO_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfMethods == other.dwNumberOfMethods && self.pEapMethods == other.pEapMethods
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO_ARRAY {}
impl ::core::fmt::Debug for EAP_METHOD_INFO_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO_ARRAY").field("dwNumberOfMethods", &self.dwNumberOfMethods).field("pEapMethods", &self.pEapMethods).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_INFO_ARRAY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO_ARRAY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfMethods == other.dwNumberOfMethods && self.pEapMethods == other.pEapMethods
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO_ARRAY_EX {}
impl ::core::fmt::Debug for EAP_METHOD_INFO_ARRAY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO_ARRAY_EX").field("dwNumberOfMethods", &self.dwNumberOfMethods).field("pEapMethods", &self.pEapMethods).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.eaptype == other.eaptype && self.pwszAuthorName == other.pwszAuthorName && self.pwszFriendlyName == other.pwszFriendlyName && self.eapProperties == other.eapProperties && self.pInnerMethodInfoArray == other.pInnerMethodInfoArray
    }
}
impl ::core::cmp::Eq for EAP_METHOD_INFO_EX {}
impl ::core::fmt::Debug for EAP_METHOD_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_INFO_EX").field("eaptype", &self.eaptype).field("pwszAuthorName", &self.pwszAuthorName).field("pwszFriendlyName", &self.pwszFriendlyName).field("eapProperties", &self.eapProperties).field("pInnerMethodInfoArray", &self.pInnerMethodInfoArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfProperties == other.dwNumberOfProperties && self.pMethodProperty == other.pMethodProperty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_ARRAY").field("dwNumberOfProperties", &self.dwNumberOfProperties).field("pMethodProperty", &self.pMethodProperty).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_METHOD_PROPERTY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_BOOL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_BOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_VALUE_BOOL").field("length", &self.length).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_DWORD {}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_DWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_VALUE_DWORD").field("length", &self.length).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}
impl ::core::cmp::Eq for EAP_METHOD_PROPERTY_VALUE_STRING {}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_PROPERTY_VALUE_STRING").field("length", &self.length).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_PROPERTY_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EAP_METHOD_PROPERTY_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EAP_METHOD_PROPERTY_VALUE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for EAP_METHOD_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_METHOD_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.eapType == other.eapType && self.dwAuthorId == other.dwAuthorId
    }
}
impl ::core::cmp::Eq for EAP_METHOD_TYPE {}
impl ::core::fmt::Debug for EAP_METHOD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_METHOD_TYPE").field("eapType", &self.eapType).field("dwAuthorId", &self.dwAuthorId).finish()
    }
}
impl ::core::default::Default for EAP_PEER_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_PEER_METHOD_ROUTINES {
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
impl ::core::cmp::Eq for EAP_PEER_METHOD_ROUTINES {}
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
impl ::core::default::Default for EAP_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EAP_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.dwVendorId == other.dwVendorId && self.dwVendorType == other.dwVendorType
    }
}
impl ::core::cmp::Eq for EAP_TYPE {}
impl ::core::fmt::Debug for EAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EAP_TYPE").field("type", &self.r#type).field("dwVendorId", &self.dwVendorId).field("dwVendorType", &self.dwVendorType).finish()
    }
}
impl ::core::default::Default for EAP_UI_DATA_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EapCertificateCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EapCertificateCredential {
    fn eq(&self, other: &Self) -> bool {
        self.certHash == other.certHash && self.password == other.password
    }
}
impl ::core::cmp::Eq for EapCertificateCredential {}
impl ::core::fmt::Debug for EapCertificateCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapCertificateCredential").field("certHash", &self.certHash).field("password", &self.password).finish()
    }
}
impl ::core::default::Default for EapCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for EapCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EapCredentialType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapCredentialType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapCredentialType").field(&self.0).finish()
    }
}
impl ::core::default::Default for EapCredentialTypeData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for EapHostPeerAuthParams {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapHostPeerAuthParams {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapHostPeerAuthParams").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EapHostPeerMethodResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EapHostPeerMethodResult {
    fn eq(&self, other: &Self) -> bool {
        self.fIsSuccess == other.fIsSuccess && self.dwFailureReasonCode == other.dwFailureReasonCode && self.fSaveConnectionData == other.fSaveConnectionData && self.dwSizeofConnectionData == other.dwSizeofConnectionData && self.pConnectionData == other.pConnectionData && self.fSaveUserData == other.fSaveUserData && self.dwSizeofUserData == other.dwSizeofUserData && self.pUserData == other.pUserData && self.pAttribArray == other.pAttribArray && self.isolationState == other.isolationState && self.pEapMethodInfo == other.pEapMethodInfo && self.pEapError == other.pEapError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EapHostPeerMethodResult {}
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
impl ::core::default::Default for EapHostPeerMethodResultReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapHostPeerMethodResultReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapHostPeerMethodResultReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for EapHostPeerResponseAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapHostPeerResponseAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapHostPeerResponseAction").field(&self.0).finish()
    }
}
impl ::core::default::Default for EapPacket {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EapPacket {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code && self.Id == other.Id && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EapPacket {}
impl ::core::fmt::Debug for EapPacket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapPacket").field("Code", &self.Code).field("Id", &self.Id).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EapPeerMethodOutput {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EapPeerMethodOutput {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.fAllowNotifications == other.fAllowNotifications
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EapPeerMethodOutput {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EapPeerMethodOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapPeerMethodOutput").field("action", &self.action).field("fAllowNotifications", &self.fAllowNotifications).finish()
    }
}
impl ::core::default::Default for EapPeerMethodResponseAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapPeerMethodResponseAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapPeerMethodResponseAction").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for EapPeerMethodResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for EapPeerMethodResult {
    fn eq(&self, other: &Self) -> bool {
        self.fIsSuccess == other.fIsSuccess && self.dwFailureReasonCode == other.dwFailureReasonCode && self.fSaveConnectionData == other.fSaveConnectionData && self.dwSizeofConnectionData == other.dwSizeofConnectionData && self.pConnectionData == other.pConnectionData && self.fSaveUserData == other.fSaveUserData && self.dwSizeofUserData == other.dwSizeofUserData && self.pUserData == other.pUserData && self.pAttribArray == other.pAttribArray && self.pEapError == other.pEapError && self.pNgcKerbTicket == other.pNgcKerbTicket && self.fSaveToCredMan == other.fSaveToCredMan
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for EapPeerMethodResult {}
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
impl ::core::default::Default for EapPeerMethodResultReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EapPeerMethodResultReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EapPeerMethodResultReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for EapSimCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EapSimCredential {
    fn eq(&self, other: &Self) -> bool {
        self.iccID == other.iccID
    }
}
impl ::core::cmp::Eq for EapSimCredential {}
impl ::core::fmt::Debug for EapSimCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapSimCredential").field("iccID", &self.iccID).finish()
    }
}
impl ::core::default::Default for EapUsernamePasswordCredential {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EapUsernamePasswordCredential {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for EapUsernamePasswordCredential {}
impl ::core::fmt::Debug for EapUsernamePasswordCredential {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EapUsernamePasswordCredential").field("username", &self.username).field("password", &self.password).finish()
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
impl IEAPProviderConfig2 {
    pub unsafe fn Initialize<P0>(&self, pszmachinename: P0, dweaptypeid: u32) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pszmachinename.into().abi(), dweaptypeid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Uninitialize)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ServerInvokeConfigUI)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ureserved1, ureserved2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RouterInvokeConfigUI)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(pconnectiondatain.as_ptr()), pconnectiondatain.len() as _, ppconnectiondataout, pdwsizeofconnectiondataout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], puserdatain: &[u8], ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RouterInvokeCredentialsUI)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(pconnectiondatain.as_ptr()), pconnectiondatain.len() as _, ::core::mem::transmute(puserdatain.as_ptr()), puserdatain.len() as _, ppuserdataout, pdwsizeofuserdataout).ok()
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
impl IEAPProviderConfig3 {
    pub unsafe fn Initialize<P0>(&self, pszmachinename: P0, dweaptypeid: u32) -> ::windows::core::Result<usize>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), pszmachinename.into().abi(), dweaptypeid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Uninitialize)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ServerInvokeConfigUI)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), ureserved1, ureserved2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeConfigUI<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RouterInvokeConfigUI)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(pconnectiondatain.as_ptr()), pconnectiondatain.len() as _, ppconnectiondataout, pdwsizeofconnectiondataout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RouterInvokeCredentialsUI<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: P0, dwflags: u32, pconnectiondatain: &[u8], puserdatain: &[u8], ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RouterInvokeCredentialsUI)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwndparent.into(), dwflags, ::core::mem::transmute(pconnectiondatain.as_ptr()), pconnectiondatain.len() as _, ::core::mem::transmute(puserdatain.as_ptr()), puserdatain.len() as _, ppuserdataout, pdwsizeofuserdataout).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ServerInvokeConfigUI2<P0>(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: P0, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ServerInvokeConfigUI2)(::windows::core::Vtable::as_raw(self), dweaptypeid, uconnectionparam, hwnd.into(), pconfigdatain, dwsizeofconfigdatain, ppconfigdataout, pdwsizeofconfigdataout).ok()
    }
    pub unsafe fn GetGlobalConfig(&self, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGlobalConfig)(::windows::core::Vtable::as_raw(self), dweaptypeid, ppconfigdataout, pdwsizeofconfigdataout).ok()
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
impl ::core::default::Default for ISOLATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ISOLATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISOLATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for LEGACY_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LEGACY_IDENTITY_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.eapType == other.eapType && self.dwFlags == other.dwFlags && self.dwSizeofConnectionData == other.dwSizeofConnectionData && self.pConnectionData == other.pConnectionData && self.dwSizeofUserData == other.dwSizeofUserData && self.pUserData == other.pUserData && self.dwSizeofUserDataOut == other.dwSizeofUserDataOut && self.pUserDataOut == other.pUserDataOut && self.pwszIdentity == other.pwszIdentity && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for LEGACY_IDENTITY_UI_PARAMS {}
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
impl ::core::default::Default for LEGACY_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LEGACY_INTERACTIVE_UI_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.eapType == other.eapType && self.dwSizeofContextData == other.dwSizeofContextData && self.pContextData == other.pContextData && self.dwSizeofInteractiveUIData == other.dwSizeofInteractiveUIData && self.pInteractiveUIData == other.pInteractiveUIData && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for LEGACY_INTERACTIVE_UI_PARAMS {}
impl ::core::fmt::Debug for LEGACY_INTERACTIVE_UI_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LEGACY_INTERACTIVE_UI_PARAMS").field("eapType", &self.eapType).field("dwSizeofContextData", &self.dwSizeofContextData).field("pContextData", &self.pContextData).field("dwSizeofInteractiveUIData", &self.dwSizeofInteractiveUIData).field("pInteractiveUIData", &self.pInteractiveUIData).field("dwError", &self.dwError).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for NgcTicketContext {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for NgcTicketContext {
    fn eq(&self, other: &Self) -> bool {
        self.wszTicket == other.wszTicket && self.hKey == other.hKey && self.hImpersonateToken == other.hImpersonateToken
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for NgcTicketContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for NgcTicketContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NgcTicketContext").field("wszTicket", &self.wszTicket).field("hKey", &self.hKey).field("hImpersonateToken", &self.hImpersonateToken).finish()
    }
}
impl ::core::default::Default for PPP_EAP_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PPP_EAP_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PPP_EAP_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PPP_EAP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_EAP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes && self.dwEapTypeId == other.dwEapTypeId && self.RasEapInitialize == other.RasEapInitialize && self.RasEapBegin == other.RasEapBegin && self.RasEapEnd == other.RasEapEnd && self.RasEapMakeMessage == other.RasEapMakeMessage
    }
}
impl ::core::cmp::Eq for PPP_EAP_INFO {}
impl ::core::fmt::Debug for PPP_EAP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_EAP_INFO").field("dwSizeInBytes", &self.dwSizeInBytes).field("dwEapTypeId", &self.dwEapTypeId).field("RasEapInitialize", &self.RasEapInitialize).field("RasEapBegin", &self.RasEapBegin).field("RasEapEnd", &self.RasEapEnd).field("RasEapMakeMessage", &self.RasEapMakeMessage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PPP_EAP_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PPP_EAP_INPUT {
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
impl ::core::cmp::Eq for PPP_EAP_INPUT {}
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PPP_EAP_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PPP_EAP_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeInBytes == other.dwSizeInBytes && self.Action == other.Action && self.dwAuthResultCode == other.dwAuthResultCode && self.pUserAttributes == other.pUserAttributes && self.fInvokeInteractiveUI == other.fInvokeInteractiveUI && self.pUIContextData == other.pUIContextData && self.dwSizeOfUIContextData == other.dwSizeOfUIContextData && self.fSaveConnectionData == other.fSaveConnectionData && self.pConnectionData == other.pConnectionData && self.dwSizeOfConnectionData == other.dwSizeOfConnectionData && self.fSaveUserData == other.fSaveUserData && self.pUserData == other.pUserData && self.dwSizeOfUserData == other.dwSizeOfUserData && self.pNgcKerbTicket == other.pNgcKerbTicket && self.fSaveToCredMan == other.fSaveToCredMan
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PPP_EAP_OUTPUT {}
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
impl ::core::default::Default for PPP_EAP_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_EAP_PACKET {
    fn eq(&self, other: &Self) -> bool {
        self.Code == other.Code && self.Id == other.Id && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for PPP_EAP_PACKET {}
impl ::core::fmt::Debug for PPP_EAP_PACKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_EAP_PACKET").field("Code", &self.Code).field("Id", &self.Id).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for RAS_AUTH_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RAS_AUTH_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.raaType == other.raaType && self.dwLength == other.dwLength && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for RAS_AUTH_ATTRIBUTE {}
impl ::core::fmt::Debug for RAS_AUTH_ATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_AUTH_ATTRIBUTE").field("raaType", &self.raaType).field("dwLength", &self.dwLength).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for RAS_AUTH_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAS_AUTH_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_AUTH_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
