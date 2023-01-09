#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.socketAddress == other.socketAddress && self.flags == other.flags && self.nearness == other.nearness && self.latency == other.latency
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_ADDRESS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for DRT_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_ADDRESS").field("socketAddress", &self.socketAddress).field("flags", &self.flags).field("nearness", &self.nearness).field("latency", &self.latency).finish()
    }
}
impl ::core::default::Default for DRT_ADDRESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_ADDRESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_ADDRESS_FLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for DRT_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AddressCount == other.AddressCount && self.AddressList == other.AddressList
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for DRT_ADDRESS_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for DRT_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_ADDRESS_LIST").field("AddressCount", &self.AddressCount).field("AddressList", &self.AddressList).finish()
    }
}
impl ::core::default::Default for DRT_BOOTSTRAP_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRT_BOOTSTRAP_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.pvContext == other.pvContext && self.Attach == other.Attach && self.Detach == other.Detach && self.InitResolve == other.InitResolve && self.IssueResolve == other.IssueResolve && self.EndResolve == other.EndResolve && self.Register == other.Register && self.Unregister == other.Unregister
    }
}
impl ::core::cmp::Eq for DRT_BOOTSTRAP_PROVIDER {}
impl ::core::fmt::Debug for DRT_BOOTSTRAP_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_BOOTSTRAP_PROVIDER").field("pvContext", &self.pvContext).field("Attach", &self.Attach).field("Detach", &self.Detach).field("InitResolve", &self.InitResolve).field("IssueResolve", &self.IssueResolve).field("EndResolve", &self.EndResolve).field("Register", &self.Register).field("Unregister", &self.Unregister).finish()
    }
}
impl ::core::default::Default for DRT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.pb == other.pb
    }
}
impl ::core::cmp::Eq for DRT_DATA {}
impl ::core::fmt::Debug for DRT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_DATA").field("cb", &self.cb).field("pb", &self.pb).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for DRT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRT_LEAFSET_KEY_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_LEAFSET_KEY_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_LEAFSET_KEY_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRT_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_MATCH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.appData == other.appData
    }
}
impl ::core::cmp::Eq for DRT_REGISTRATION {}
impl ::core::fmt::Debug for DRT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_REGISTRATION").field("key", &self.key).field("appData", &self.appData).finish()
    }
}
impl ::core::default::Default for DRT_REGISTRATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_REGISTRATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_REGISTRATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRT_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_SCOPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRT_SEARCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRT_SEARCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fIterative == other.fIterative && self.fAllowCurrentInstanceMatch == other.fAllowCurrentInstanceMatch && self.fAnyMatchInRange == other.fAnyMatchInRange && self.cMaxEndpoints == other.cMaxEndpoints && self.pMaximumKey == other.pMaximumKey && self.pMinimumKey == other.pMinimumKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRT_SEARCH_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRT_SEARCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SEARCH_INFO").field("dwSize", &self.dwSize).field("fIterative", &self.fIterative).field("fAllowCurrentInstanceMatch", &self.fAllowCurrentInstanceMatch).field("fAnyMatchInRange", &self.fAnyMatchInRange).field("cMaxEndpoints", &self.cMaxEndpoints).field("pMaximumKey", &self.pMaximumKey).field("pMinimumKey", &self.pMinimumKey).finish()
    }
}
impl ::core::default::Default for DRT_SEARCH_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRT_SEARCH_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.r#type == other.r#type && self.pvContext == other.pvContext && self.registration == other.registration
    }
}
impl ::core::cmp::Eq for DRT_SEARCH_RESULT {}
impl ::core::fmt::Debug for DRT_SEARCH_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SEARCH_RESULT").field("dwSize", &self.dwSize).field("type", &self.r#type).field("pvContext", &self.pvContext).field("registration", &self.registration).finish()
    }
}
impl ::core::default::Default for DRT_SECURITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_SECURITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_SECURITY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRT_SECURITY_PROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRT_SECURITY_PROVIDER {
    fn eq(&self, other: &Self) -> bool {
        self.pvContext == other.pvContext && self.Attach == other.Attach && self.Detach == other.Detach && self.RegisterKey == other.RegisterKey && self.UnregisterKey == other.UnregisterKey && self.ValidateAndUnpackPayload == other.ValidateAndUnpackPayload && self.SecureAndPackPayload == other.SecureAndPackPayload && self.FreeData == other.FreeData && self.EncryptData == other.EncryptData && self.DecryptData == other.DecryptData && self.GetSerializedCredential == other.GetSerializedCredential && self.ValidateRemoteCredential == other.ValidateRemoteCredential && self.SignData == other.SignData && self.VerifyData == other.VerifyData
    }
}
impl ::core::cmp::Eq for DRT_SECURITY_PROVIDER {}
impl ::core::fmt::Debug for DRT_SECURITY_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SECURITY_PROVIDER")
            .field("pvContext", &self.pvContext)
            .field("Attach", &self.Attach)
            .field("Detach", &self.Detach)
            .field("RegisterKey", &self.RegisterKey)
            .field("UnregisterKey", &self.UnregisterKey)
            .field("ValidateAndUnpackPayload", &self.ValidateAndUnpackPayload)
            .field("SecureAndPackPayload", &self.SecureAndPackPayload)
            .field("FreeData", &self.FreeData)
            .field("EncryptData", &self.EncryptData)
            .field("DecryptData", &self.DecryptData)
            .field("GetSerializedCredential", &self.GetSerializedCredential)
            .field("ValidateRemoteCredential", &self.ValidateRemoteCredential)
            .field("SignData", &self.SignData)
            .field("VerifyData", &self.VerifyData)
            .finish()
    }
}
impl ::core::default::Default for DRT_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRT_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cbKey == other.cbKey && self.bProtocolMajorVersion == other.bProtocolMajorVersion && self.bProtocolMinorVersion == other.bProtocolMinorVersion && self.ulMaxRoutingAddresses == other.ulMaxRoutingAddresses && self.pwzDrtInstancePrefix == other.pwzDrtInstancePrefix && self.hTransport == other.hTransport && self.pSecurityProvider == other.pSecurityProvider && self.pBootstrapProvider == other.pBootstrapProvider && self.eSecurityMode == other.eSecurityMode
    }
}
impl ::core::cmp::Eq for DRT_SETTINGS {}
impl ::core::fmt::Debug for DRT_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRT_SETTINGS")
            .field("dwSize", &self.dwSize)
            .field("cbKey", &self.cbKey)
            .field("bProtocolMajorVersion", &self.bProtocolMajorVersion)
            .field("bProtocolMinorVersion", &self.bProtocolMinorVersion)
            .field("ulMaxRoutingAddresses", &self.ulMaxRoutingAddresses)
            .field("pwzDrtInstancePrefix", &self.pwzDrtInstancePrefix)
            .field("hTransport", &self.hTransport)
            .field("pSecurityProvider", &self.pSecurityProvider)
            .field("pBootstrapProvider", &self.pBootstrapProvider)
            .field("eSecurityMode", &self.eSecurityMode)
            .finish()
    }
}
impl ::core::default::Default for DRT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRT_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEERDIST_CLIENT_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEERDIST_CLIENT_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fFlashCrowd == other.fFlashCrowd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEERDIST_CLIENT_BASIC_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEERDIST_CLIENT_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_CLIENT_BASIC_INFO").field("fFlashCrowd", &self.fFlashCrowd).finish()
    }
}
impl ::core::default::Default for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEERDIST_CLIENT_INFO_BY_HANDLE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEERDIST_CONTENT_TAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEERDIST_CONTENT_TAG {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for PEERDIST_CONTENT_TAG {}
impl ::core::fmt::Debug for PEERDIST_CONTENT_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_CONTENT_TAG").field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for PEERDIST_PUBLICATION_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEERDIST_PUBLICATION_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for PEERDIST_PUBLICATION_OPTIONS {}
impl ::core::fmt::Debug for PEERDIST_PUBLICATION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_PUBLICATION_OPTIONS").field("dwVersion", &self.dwVersion).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for PEERDIST_RETRIEVAL_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEERDIST_RETRIEVAL_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwContentInfoMinVersion == other.dwContentInfoMinVersion && self.dwContentInfoMaxVersion == other.dwContentInfoMaxVersion && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for PEERDIST_RETRIEVAL_OPTIONS {}
impl ::core::fmt::Debug for PEERDIST_RETRIEVAL_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_RETRIEVAL_OPTIONS").field("cbSize", &self.cbSize).field("dwContentInfoMinVersion", &self.dwContentInfoMinVersion).field("dwContentInfoMaxVersion", &self.dwContentInfoMaxVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEERDIST_RETRIEVAL_OPTIONS_CONTENTINFO_VERSION_VALUE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEERDIST_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEERDIST_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEERDIST_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEERDIST_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEERDIST_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.status == other.status && self.dwMinVer == other.dwMinVer && self.dwMaxVer == other.dwMaxVer
    }
}
impl ::core::cmp::Eq for PEERDIST_STATUS_INFO {}
impl ::core::fmt::Debug for PEERDIST_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEERDIST_STATUS_INFO").field("cbSize", &self.cbSize).field("status", &self.status).field("dwMinVer", &self.dwMinVer).field("dwMaxVer", &self.dwMaxVer).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_APPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_APPLICATION {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data && self.pwzDescription == other.pwzDescription
    }
}
impl ::core::cmp::Eq for PEER_APPLICATION {}
impl ::core::fmt::Debug for PEER_APPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_APPLICATION").field("id", &self.id).field("data", &self.data).field("pwzDescription", &self.pwzDescription).finish()
    }
}
impl ::core::default::Default for PEER_APPLICATION_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_APPLICATION_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.application == other.application && self.pwzApplicationToLaunch == other.pwzApplicationToLaunch && self.pwzApplicationArguments == other.pwzApplicationArguments && self.dwPublicationScope == other.dwPublicationScope
    }
}
impl ::core::cmp::Eq for PEER_APPLICATION_REGISTRATION_INFO {}
impl ::core::fmt::Debug for PEER_APPLICATION_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_APPLICATION_REGISTRATION_INFO").field("application", &self.application).field("pwzApplicationToLaunch", &self.pwzApplicationToLaunch).field("pwzApplicationArguments", &self.pwzApplicationArguments).field("dwPublicationScope", &self.dwPublicationScope).finish()
    }
}
impl ::core::default::Default for PEER_APPLICATION_REGISTRATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_APPLICATION_REGISTRATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_APPLICATION_REGISTRATION_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_APP_LAUNCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_APP_LAUNCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.pInvitation == other.pInvitation
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_APP_LAUNCH_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_APP_LAUNCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_APP_LAUNCH_INFO").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("pInvitation", &self.pInvitation).finish()
    }
}
impl ::core::default::Default for PEER_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_COLLAB_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_COLLAB_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_COLLAB_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.eventType == other.eventType && self.pInstance == other.pInstance
    }
}
impl ::core::cmp::Eq for PEER_COLLAB_EVENT_REGISTRATION {}
impl ::core::fmt::Debug for PEER_COLLAB_EVENT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_COLLAB_EVENT_REGISTRATION").field("eventType", &self.eventType).field("pInstance", &self.pInstance).finish()
    }
}
impl ::core::default::Default for PEER_COLLAB_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_COLLAB_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_COLLAB_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_CONNECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_CONNECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_CONNECTION_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_CONNECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_CONNECTION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_CONNECTION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_CONNECTION_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_CONTACT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_CONTACT {
    fn eq(&self, other: &Self) -> bool {
        self.pwzPeerName == other.pwzPeerName && self.pwzNickName == other.pwzNickName && self.pwzDisplayName == other.pwzDisplayName && self.pwzEmailAddress == other.pwzEmailAddress && self.fWatch == other.fWatch && self.WatcherPermissions == other.WatcherPermissions && self.credentials == other.credentials
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_CONTACT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_CONTACT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_CONTACT").field("pwzPeerName", &self.pwzPeerName).field("pwzNickName", &self.pwzNickName).field("pwzDisplayName", &self.pwzDisplayName).field("pwzEmailAddress", &self.pwzEmailAddress).field("fWatch", &self.fWatch).field("WatcherPermissions", &self.WatcherPermissions).field("credentials", &self.credentials).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_CREDENTIAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.pwzFriendlyName == other.pwzFriendlyName && self.pPublicKey == other.pPublicKey && self.pwzIssuerPeerName == other.pwzIssuerPeerName && self.pwzIssuerFriendlyName == other.pwzIssuerFriendlyName && self.ftValidityStart == other.ftValidityStart && self.ftValidityEnd == other.ftValidityEnd && self.cRoles == other.cRoles && self.pRoles == other.pRoles
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_CREDENTIAL_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PEER_CREDENTIAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_CREDENTIAL_INFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("pwzFriendlyName", &self.pwzFriendlyName).field("pPublicKey", &self.pPublicKey).field("pwzIssuerPeerName", &self.pwzIssuerPeerName).field("pwzIssuerFriendlyName", &self.pwzIssuerFriendlyName).field("ftValidityStart", &self.ftValidityStart).field("ftValidityEnd", &self.ftValidityEnd).field("cRoles", &self.cRoles).field("pRoles", &self.pRoles).finish()
    }
}
impl ::core::default::Default for PEER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbData == other.cbData && self.pbData == other.pbData
    }
}
impl ::core::cmp::Eq for PEER_DATA {}
impl ::core::fmt::Debug for PEER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_DATA").field("cbData", &self.cbData).field("pbData", &self.pbData).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.changeType == other.changeType && self.pApplication == other.pApplication
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_APPLICATION_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_APPLICATION_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_APPLICATION_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("changeType", &self.changeType).field("pApplication", &self.pApplication).finish()
    }
}
impl ::core::default::Default for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.status == other.status && self.ullConnectionId == other.ullConnectionId && self.ullNodeId == other.ullNodeId && self.ullNextConnectionId == other.ullNextConnectionId && self.hrConnectionFailedReason == other.hrConnectionFailedReason
    }
}
impl ::core::cmp::Eq for PEER_EVENT_CONNECTION_CHANGE_DATA {}
impl ::core::fmt::Debug for PEER_EVENT_CONNECTION_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_CONNECTION_CHANGE_DATA").field("dwSize", &self.dwSize).field("status", &self.status).field("ullConnectionId", &self.ullConnectionId).field("ullNodeId", &self.ullNodeId).field("ullNextConnectionId", &self.ullNextConnectionId).field("hrConnectionFailedReason", &self.hrConnectionFailedReason).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_ENDPOINT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_ENDPOINT_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_ENDPOINT_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).finish()
    }
}
impl ::core::default::Default for PEER_EVENT_INCOMING_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_EVENT_INCOMING_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ullConnectionId == other.ullConnectionId && self.r#type == other.r#type && self.data == other.data
    }
}
impl ::core::cmp::Eq for PEER_EVENT_INCOMING_DATA {}
impl ::core::fmt::Debug for PEER_EVENT_INCOMING_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_INCOMING_DATA").field("dwSize", &self.dwSize).field("ullConnectionId", &self.ullConnectionId).field("type", &self.r#type).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.changeType == other.changeType && self.pwzIdentity == other.pwzIdentity
    }
}
impl ::core::cmp::Eq for PEER_EVENT_MEMBER_CHANGE_DATA {}
impl ::core::fmt::Debug for PEER_EVENT_MEMBER_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_MEMBER_CHANGE_DATA").field("dwSize", &self.dwSize).field("changeType", &self.changeType).field("pwzIdentity", &self.pwzIdentity).finish()
    }
}
impl ::core::default::Default for PEER_EVENT_NODE_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_EVENT_NODE_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.changeType == other.changeType && self.ullNodeId == other.ullNodeId && self.pwzPeerId == other.pwzPeerId
    }
}
impl ::core::cmp::Eq for PEER_EVENT_NODE_CHANGE_DATA {}
impl ::core::fmt::Debug for PEER_EVENT_NODE_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_NODE_CHANGE_DATA").field("dwSize", &self.dwSize).field("changeType", &self.changeType).field("ullNodeId", &self.ullNodeId).field("pwzPeerId", &self.pwzPeerId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.changeType == other.changeType && self.pObject == other.pObject
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_OBJECT_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_OBJECT_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_OBJECT_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("changeType", &self.changeType).field("pObject", &self.pObject).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.pPeopleNearMe == other.pPeopleNearMe
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_PEOPLE_NEAR_ME_CHANGED_DATA").field("changeType", &self.changeType).field("pPeopleNearMe", &self.pPeopleNearMe).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.pEndpoint == other.pEndpoint && self.changeType == other.changeType && self.pPresenceInfo == other.pPresenceInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_EVENT_PRESENCE_CHANGED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_EVENT_PRESENCE_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_PRESENCE_CHANGED_DATA").field("pContact", &self.pContact).field("pEndpoint", &self.pEndpoint).field("changeType", &self.changeType).field("pPresenceInfo", &self.pPresenceInfo).finish()
    }
}
impl ::core::default::Default for PEER_EVENT_RECORD_CHANGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_EVENT_RECORD_CHANGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.changeType == other.changeType && self.recordId == other.recordId && self.recordType == other.recordType
    }
}
impl ::core::cmp::Eq for PEER_EVENT_RECORD_CHANGE_DATA {}
impl ::core::fmt::Debug for PEER_EVENT_RECORD_CHANGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_RECORD_CHANGE_DATA").field("dwSize", &self.dwSize).field("changeType", &self.changeType).field("recordId", &self.recordId).field("recordType", &self.recordType).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pEndpoint == other.pEndpoint && self.hrChange == other.hrChange
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_EVENT_REQUEST_STATUS_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_REQUEST_STATUS_CHANGED_DATA").field("pEndpoint", &self.pEndpoint).field("hrChange", &self.hrChange).finish()
    }
}
impl ::core::default::Default for PEER_EVENT_SYNCHRONIZED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_EVENT_SYNCHRONIZED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.recordType == other.recordType
    }
}
impl ::core::cmp::Eq for PEER_EVENT_SYNCHRONIZED_DATA {}
impl ::core::fmt::Debug for PEER_EVENT_SYNCHRONIZED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_SYNCHRONIZED_DATA").field("dwSize", &self.dwSize).field("recordType", &self.recordType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pContact == other.pContact && self.changeType == other.changeType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_EVENT_WATCHLIST_CHANGED_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_EVENT_WATCHLIST_CHANGED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_EVENT_WATCHLIST_CHANGED_DATA").field("pContact", &self.pContact).field("changeType", &self.changeType).finish()
    }
}
impl ::core::default::Default for PEER_GRAPH_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_GRAPH_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_GRAPH_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.eventType == other.eventType && self.pType == other.pType
    }
}
impl ::core::cmp::Eq for PEER_GRAPH_EVENT_REGISTRATION {}
impl ::core::fmt::Debug for PEER_GRAPH_EVENT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GRAPH_EVENT_REGISTRATION").field("eventType", &self.eventType).field("pType", &self.pType).finish()
    }
}
impl ::core::default::Default for PEER_GRAPH_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GRAPH_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GRAPH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_GRAPH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwScope == other.dwScope && self.dwMaxRecordSize == other.dwMaxRecordSize && self.pwzGraphId == other.pwzGraphId && self.pwzCreatorId == other.pwzCreatorId && self.pwzFriendlyName == other.pwzFriendlyName && self.pwzComment == other.pwzComment && self.ulPresenceLifetime == other.ulPresenceLifetime && self.cPresenceMax == other.cPresenceMax
    }
}
impl ::core::cmp::Eq for PEER_GRAPH_PROPERTIES {}
impl ::core::fmt::Debug for PEER_GRAPH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GRAPH_PROPERTIES").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwScope", &self.dwScope).field("dwMaxRecordSize", &self.dwMaxRecordSize).field("pwzGraphId", &self.pwzGraphId).field("pwzCreatorId", &self.pwzCreatorId).field("pwzFriendlyName", &self.pwzFriendlyName).field("pwzComment", &self.pwzComment).field("ulPresenceLifetime", &self.ulPresenceLifetime).field("cPresenceMax", &self.cPresenceMax).finish()
    }
}
impl ::core::default::Default for PEER_GRAPH_PROPERTY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GRAPH_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_PROPERTY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GRAPH_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GRAPH_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GRAPH_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GRAPH_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GRAPH_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GROUP_AUTHENTICATION_SCHEME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GROUP_AUTHENTICATION_SCHEME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_AUTHENTICATION_SCHEME").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GROUP_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_GROUP_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_GROUP_EVENT_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.eventType == other.eventType && self.pType == other.pType
    }
}
impl ::core::cmp::Eq for PEER_GROUP_EVENT_REGISTRATION {}
impl ::core::fmt::Debug for PEER_GROUP_EVENT_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GROUP_EVENT_REGISTRATION").field("eventType", &self.eventType).field("pType", &self.pType).finish()
    }
}
impl ::core::default::Default for PEER_GROUP_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GROUP_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GROUP_ISSUE_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_ISSUE_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GROUP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_GROUP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.pwzCloud == other.pwzCloud && self.pwzClassifier == other.pwzClassifier && self.pwzGroupPeerName == other.pwzGroupPeerName && self.pwzCreatorPeerName == other.pwzCreatorPeerName && self.pwzFriendlyName == other.pwzFriendlyName && self.pwzComment == other.pwzComment && self.ulMemberDataLifetime == other.ulMemberDataLifetime && self.ulPresenceLifetime == other.ulPresenceLifetime && self.dwAuthenticationSchemes == other.dwAuthenticationSchemes && self.pwzGroupPassword == other.pwzGroupPassword && self.groupPasswordRole == other.groupPasswordRole
    }
}
impl ::core::cmp::Eq for PEER_GROUP_PROPERTIES {}
impl ::core::fmt::Debug for PEER_GROUP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_GROUP_PROPERTIES")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("pwzCloud", &self.pwzCloud)
            .field("pwzClassifier", &self.pwzClassifier)
            .field("pwzGroupPeerName", &self.pwzGroupPeerName)
            .field("pwzCreatorPeerName", &self.pwzCreatorPeerName)
            .field("pwzFriendlyName", &self.pwzFriendlyName)
            .field("pwzComment", &self.pwzComment)
            .field("ulMemberDataLifetime", &self.ulMemberDataLifetime)
            .field("ulPresenceLifetime", &self.ulPresenceLifetime)
            .field("dwAuthenticationSchemes", &self.dwAuthenticationSchemes)
            .field("pwzGroupPassword", &self.pwzGroupPassword)
            .field("groupPasswordRole", &self.groupPasswordRole)
            .finish()
    }
}
impl ::core::default::Default for PEER_GROUP_PROPERTY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GROUP_PROPERTY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_PROPERTY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_GROUP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_GROUP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_GROUP_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_INVITATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_INVITATION {
    fn eq(&self, other: &Self) -> bool {
        self.applicationId == other.applicationId && self.applicationData == other.applicationData && self.pwzMessage == other.pwzMessage
    }
}
impl ::core::cmp::Eq for PEER_INVITATION {}
impl ::core::fmt::Debug for PEER_INVITATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_INVITATION").field("applicationId", &self.applicationId).field("applicationData", &self.applicationData).field("pwzMessage", &self.pwzMessage).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_INVITATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_INVITATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.pwzCloudName == other.pwzCloudName
            && self.dwScope == other.dwScope
            && self.dwCloudFlags == other.dwCloudFlags
            && self.pwzGroupPeerName == other.pwzGroupPeerName
            && self.pwzIssuerPeerName == other.pwzIssuerPeerName
            && self.pwzSubjectPeerName == other.pwzSubjectPeerName
            && self.pwzGroupFriendlyName == other.pwzGroupFriendlyName
            && self.pwzIssuerFriendlyName == other.pwzIssuerFriendlyName
            && self.pwzSubjectFriendlyName == other.pwzSubjectFriendlyName
            && self.ftValidityStart == other.ftValidityStart
            && self.ftValidityEnd == other.ftValidityEnd
            && self.cRoles == other.cRoles
            && self.pRoles == other.pRoles
            && self.cClassifiers == other.cClassifiers
            && self.ppwzClassifiers == other.ppwzClassifiers
            && self.pSubjectPublicKey == other.pSubjectPublicKey
            && self.authScheme == other.authScheme
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_INVITATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PEER_INVITATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_INVITATION_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("pwzCloudName", &self.pwzCloudName)
            .field("dwScope", &self.dwScope)
            .field("dwCloudFlags", &self.dwCloudFlags)
            .field("pwzGroupPeerName", &self.pwzGroupPeerName)
            .field("pwzIssuerPeerName", &self.pwzIssuerPeerName)
            .field("pwzSubjectPeerName", &self.pwzSubjectPeerName)
            .field("pwzGroupFriendlyName", &self.pwzGroupFriendlyName)
            .field("pwzIssuerFriendlyName", &self.pwzIssuerFriendlyName)
            .field("pwzSubjectFriendlyName", &self.pwzSubjectFriendlyName)
            .field("ftValidityStart", &self.ftValidityStart)
            .field("ftValidityEnd", &self.ftValidityEnd)
            .field("cRoles", &self.cRoles)
            .field("pRoles", &self.pRoles)
            .field("cClassifiers", &self.cClassifiers)
            .field("ppwzClassifiers", &self.ppwzClassifiers)
            .field("pSubjectPublicKey", &self.pSubjectPublicKey)
            .field("authScheme", &self.authScheme)
            .finish()
    }
}
impl ::core::default::Default for PEER_INVITATION_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_INVITATION_RESPONSE {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action && self.pwzMessage == other.pwzMessage && self.hrExtendedInfo == other.hrExtendedInfo
    }
}
impl ::core::cmp::Eq for PEER_INVITATION_RESPONSE {}
impl ::core::fmt::Debug for PEER_INVITATION_RESPONSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_INVITATION_RESPONSE").field("action", &self.action).field("pwzMessage", &self.pwzMessage).field("hrExtendedInfo", &self.hrExtendedInfo).finish()
    }
}
impl ::core::default::Default for PEER_INVITATION_RESPONSE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_INVITATION_RESPONSE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_INVITATION_RESPONSE_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for PEER_MEMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for PEER_MEMBER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.pwzIdentity == other.pwzIdentity && self.pwzAttributes == other.pwzAttributes && self.ullNodeId == other.ullNodeId && self.cAddresses == other.cAddresses && self.pAddresses == other.pAddresses && self.pCredentialInfo == other.pCredentialInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for PEER_MEMBER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for PEER_MEMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_MEMBER").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("pwzIdentity", &self.pwzIdentity).field("pwzAttributes", &self.pwzAttributes).field("ullNodeId", &self.ullNodeId).field("cAddresses", &self.cAddresses).field("pAddresses", &self.pAddresses).field("pCredentialInfo", &self.pCredentialInfo).finish()
    }
}
impl ::core::default::Default for PEER_MEMBER_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_MEMBER_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_MEMBER_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_MEMBER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_MEMBER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_MEMBER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_NAME_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_NAME_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pwzPeerName == other.pwzPeerName && self.pwzFriendlyName == other.pwzFriendlyName
    }
}
impl ::core::cmp::Eq for PEER_NAME_PAIR {}
impl ::core::fmt::Debug for PEER_NAME_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_NAME_PAIR").field("dwSize", &self.dwSize).field("pwzPeerName", &self.pwzPeerName).field("pwzFriendlyName", &self.pwzFriendlyName).finish()
    }
}
impl ::core::default::Default for PEER_NODE_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_NODE_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_NODE_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_NODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for PEER_NODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ullNodeId == other.ullNodeId && self.pwzPeerId == other.pwzPeerId && self.cAddresses == other.cAddresses && self.pAddresses == other.pAddresses && self.pwzAttributes == other.pwzAttributes
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for PEER_NODE_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for PEER_NODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_NODE_INFO").field("dwSize", &self.dwSize).field("ullNodeId", &self.ullNodeId).field("pwzPeerId", &self.pwzPeerId).field("cAddresses", &self.cAddresses).field("pAddresses", &self.pAddresses).field("pwzAttributes", &self.pwzAttributes).finish()
    }
}
impl ::core::default::Default for PEER_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data && self.dwPublicationScope == other.dwPublicationScope
    }
}
impl ::core::cmp::Eq for PEER_OBJECT {}
impl ::core::fmt::Debug for PEER_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_OBJECT").field("id", &self.id).field("data", &self.data).field("dwPublicationScope", &self.dwPublicationScope).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for PEER_PEOPLE_NEAR_ME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_PNRP_CLOUD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_PNRP_CLOUD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwzCloudName == other.pwzCloudName && self.dwScope == other.dwScope && self.dwScopeId == other.dwScopeId
    }
}
impl ::core::cmp::Eq for PEER_PNRP_CLOUD_INFO {}
impl ::core::fmt::Debug for PEER_PNRP_CLOUD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PNRP_CLOUD_INFO").field("pwzCloudName", &self.pwzCloudName).field("dwScope", &self.dwScope).field("dwScopeId", &self.dwScopeId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_PNRP_ENDPOINT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_PNRP_ENDPOINT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwzPeerName == other.pwzPeerName && self.cAddresses == other.cAddresses && self.ppAddresses == other.ppAddresses && self.pwzComment == other.pwzComment && self.payload == other.payload
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_PNRP_ENDPOINT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_PNRP_ENDPOINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PNRP_ENDPOINT_INFO").field("pwzPeerName", &self.pwzPeerName).field("cAddresses", &self.cAddresses).field("ppAddresses", &self.ppAddresses).field("pwzComment", &self.pwzComment).field("payload", &self.payload).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PEER_PNRP_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PEER_PNRP_REGISTRATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwzCloudName == other.pwzCloudName && self.pwzPublishingIdentity == other.pwzPublishingIdentity && self.cAddresses == other.cAddresses && self.ppAddresses == other.ppAddresses && self.wPort == other.wPort && self.pwzComment == other.pwzComment && self.payload == other.payload
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PEER_PNRP_REGISTRATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PEER_PNRP_REGISTRATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PNRP_REGISTRATION_INFO").field("pwzCloudName", &self.pwzCloudName).field("pwzPublishingIdentity", &self.pwzPublishingIdentity).field("cAddresses", &self.cAddresses).field("ppAddresses", &self.ppAddresses).field("wPort", &self.wPort).field("pwzComment", &self.pwzComment).field("payload", &self.payload).finish()
    }
}
impl ::core::default::Default for PEER_PRESENCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_PRESENCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.pwzDescriptiveText == other.pwzDescriptiveText
    }
}
impl ::core::cmp::Eq for PEER_PRESENCE_INFO {}
impl ::core::fmt::Debug for PEER_PRESENCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_PRESENCE_INFO").field("status", &self.status).field("pwzDescriptiveText", &self.pwzDescriptiveText).finish()
    }
}
impl ::core::default::Default for PEER_PRESENCE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_PRESENCE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_PRESENCE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_PUBLICATION_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_PUBLICATION_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_PUBLICATION_SCOPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PEER_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.r#type == other.r#type && self.id == other.id && self.dwVersion == other.dwVersion && self.dwFlags == other.dwFlags && self.pwzCreatorId == other.pwzCreatorId && self.pwzModifiedById == other.pwzModifiedById && self.pwzAttributes == other.pwzAttributes && self.ftCreation == other.ftCreation && self.ftExpiration == other.ftExpiration && self.ftLastModified == other.ftLastModified && self.securityData == other.securityData && self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PEER_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PEER_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_RECORD")
            .field("dwSize", &self.dwSize)
            .field("type", &self.r#type)
            .field("id", &self.id)
            .field("dwVersion", &self.dwVersion)
            .field("dwFlags", &self.dwFlags)
            .field("pwzCreatorId", &self.pwzCreatorId)
            .field("pwzModifiedById", &self.pwzModifiedById)
            .field("pwzAttributes", &self.pwzAttributes)
            .field("ftCreation", &self.ftCreation)
            .field("ftExpiration", &self.ftExpiration)
            .field("ftLastModified", &self.ftLastModified)
            .field("securityData", &self.securityData)
            .field("data", &self.data)
            .finish()
    }
}
impl ::core::default::Default for PEER_RECORD_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_RECORD_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_RECORD_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_RECORD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_RECORD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_RECORD_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PEER_SECURITY_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PEER_SIGNIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_SIGNIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_SIGNIN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PEER_VERSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PEER_VERSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wVersion == other.wVersion && self.wHighestVersion == other.wHighestVersion
    }
}
impl ::core::cmp::Eq for PEER_VERSION_DATA {}
impl ::core::fmt::Debug for PEER_VERSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEER_VERSION_DATA").field("wVersion", &self.wVersion).field("wHighestVersion", &self.wHighestVersion).finish()
    }
}
impl ::core::default::Default for PEER_WATCH_PERMISSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEER_WATCH_PERMISSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEER_WATCH_PERMISSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PNRPCLOUDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PNRPCLOUDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.Cloud == other.Cloud && self.enCloudState == other.enCloudState && self.enCloudFlags == other.enCloudFlags
    }
}
impl ::core::cmp::Eq for PNRPCLOUDINFO {}
impl ::core::fmt::Debug for PNRPCLOUDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PNRPCLOUDINFO").field("dwSize", &self.dwSize).field("Cloud", &self.Cloud).field("enCloudState", &self.enCloudState).field("enCloudFlags", &self.enCloudFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for PNRPINFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for PNRPINFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpwszIdentity == other.lpwszIdentity && self.nMaxResolve == other.nMaxResolve && self.dwTimeout == other.dwTimeout && self.dwLifetime == other.dwLifetime && self.enResolveCriteria == other.enResolveCriteria && self.dwFlags == other.dwFlags && self.saHint == other.saHint && self.enNameState == other.enNameState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for PNRPINFO_V1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for PNRPINFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PNRPINFO_V1").field("dwSize", &self.dwSize).field("lpwszIdentity", &self.lpwszIdentity).field("nMaxResolve", &self.nMaxResolve).field("dwTimeout", &self.dwTimeout).field("dwLifetime", &self.dwLifetime).field("enResolveCriteria", &self.enResolveCriteria).field("dwFlags", &self.dwFlags).field("saHint", &self.saHint).field("enNameState", &self.enNameState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_System_Com"))]
impl ::core::default::Default for PNRPINFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PNRP_CLOUD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PNRP_CLOUD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_CLOUD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PNRP_CLOUD_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PNRP_CLOUD_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.Scope == other.Scope && self.ScopeId == other.ScopeId
    }
}
impl ::core::cmp::Eq for PNRP_CLOUD_ID {}
impl ::core::fmt::Debug for PNRP_CLOUD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PNRP_CLOUD_ID").field("AddressFamily", &self.AddressFamily).field("Scope", &self.Scope).field("ScopeId", &self.ScopeId).finish()
    }
}
impl ::core::default::Default for PNRP_CLOUD_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PNRP_CLOUD_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_CLOUD_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PNRP_EXTENDED_PAYLOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PNRP_EXTENDED_PAYLOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_EXTENDED_PAYLOAD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PNRP_REGISTERED_ID_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PNRP_REGISTERED_ID_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_REGISTERED_ID_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PNRP_RESOLVE_CRITERIA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PNRP_RESOLVE_CRITERIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_RESOLVE_CRITERIA").field(&self.0).finish()
    }
}
impl ::core::default::Default for PNRP_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PNRP_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PNRP_SCOPE").field(&self.0).finish()
    }
}
