impl ::core::default::Default for CH_DESCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CH_DESCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CH_DESCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11EXT_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOT11EXT_IHV_CONNECTION_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11EXT_IHV_CONNECTION_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11EXT_IHV_CONNECTION_PHASE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.pszXmlFragmentIhvConnectivity == other.pszXmlFragmentIhvConnectivity
    }
}
impl ::core::cmp::Eq for DOT11EXT_IHV_CONNECTIVITY_PROFILE {}
impl ::core::fmt::Debug for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_CONNECTIVITY_PROFILE").field("pszXmlFragmentIhvConnectivity", &self.pszXmlFragmentIhvConnectivity).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.IhvConnectivityProfile == other.IhvConnectivityProfile && self.IhvSecurityProfile == other.IhvSecurityProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_IHV_DISCOVERY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_DISCOVERY_PROFILE").field("IhvConnectivityProfile", &self.IhvConnectivityProfile).field("IhvSecurityProfile", &self.IhvSecurityProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.pIhvDiscoveryProfiles == other.pIhvDiscoveryProfiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_DISCOVERY_PROFILE_LIST").field("dwCount", &self.dwCount).field("pIhvDiscoveryProfiles", &self.pIhvDiscoveryProfiles).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::core::default::Default for DOT11EXT_IHV_HANDLERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOT11EXT_IHV_INDICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11EXT_IHV_INDICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11EXT_IHV_INDICATION_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11EXT_IHV_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ExtIhvProfileParams == other.dot11ExtIhvProfileParams && self.wstrProfileName == other.wstrProfileName && self.dwProfileTypeFlags == other.dwProfileTypeFlags && self.interfaceGuid == other.interfaceGuid
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::Eq for DOT11EXT_IHV_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11EXT_IHV_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_PARAMS").field("dot11ExtIhvProfileParams", &self.dot11ExtIhvProfileParams).field("wstrProfileName", &self.wstrProfileName).field("dwProfileTypeFlags", &self.dwProfileTypeFlags).field("interfaceGuid", &self.interfaceGuid).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11EXT_IHV_PROFILE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_PROFILE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pSsidList == other.pSsidList && self.BssType == other.BssType && self.pMSSecuritySettings == other.pMSSecuritySettings
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::Eq for DOT11EXT_IHV_PROFILE_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11EXT_IHV_PROFILE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_PROFILE_PARAMS").field("pSsidList", &self.pSsidList).field("BssType", &self.BssType).field("pMSSecuritySettings", &self.pMSSecuritySettings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_IHV_SECURITY_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_SECURITY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.pszXmlFragmentIhvSecurity == other.pszXmlFragmentIhvSecurity && self.bUseMSOnex == other.bUseMSOnex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_IHV_SECURITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_IHV_SECURITY_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_SECURITY_PROFILE").field("pszXmlFragmentIhvSecurity", &self.pszXmlFragmentIhvSecurity).field("bUseMSOnex", &self.bUseMSOnex).finish()
    }
}
impl ::core::default::Default for DOT11EXT_IHV_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11EXT_IHV_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulCount == other.ulCount && self.SSIDs == other.SSIDs
    }
}
impl ::core::cmp::Eq for DOT11EXT_IHV_SSID_LIST {}
impl ::core::fmt::Debug for DOT11EXT_IHV_SSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_SSID_LIST").field("ulCount", &self.ulCount).field("SSIDs", &self.SSIDs).finish()
    }
}
impl ::core::default::Default for DOT11EXT_IHV_UI_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11EXT_IHV_UI_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwSessionId == other.dwSessionId && self.guidUIRequest == other.guidUIRequest && self.UIPageClsid == other.UIPageClsid && self.dwByteCount == other.dwByteCount && self.pvUIRequest == other.pvUIRequest
    }
}
impl ::core::cmp::Eq for DOT11EXT_IHV_UI_REQUEST {}
impl ::core::fmt::Debug for DOT11EXT_IHV_UI_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_UI_REQUEST").field("dwSessionId", &self.dwSessionId).field("guidUIRequest", &self.guidUIRequest).field("UIPageClsid", &self.UIPageClsid).field("dwByteCount", &self.dwByteCount).field("pvUIRequest", &self.pvUIRequest).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_VIRTUAL_STATION_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.dot11SSID == other.dot11SSID && self.dot11AuthAlgo == other.dot11AuthAlgo && self.dot11CipherAlgo == other.dot11CipherAlgo && self.bIsPassPhrase == other.bIsPassPhrase && self.dwKeyLength == other.dwKeyLength && self.ucKeyData == other.ucKeyData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_VIRTUAL_STATION_AP_PROPERTY").field("dot11SSID", &self.dot11SSID).field("dot11AuthAlgo", &self.dot11AuthAlgo).field("dot11CipherAlgo", &self.dot11CipherAlgo).field("bIsPassPhrase", &self.bIsPassPhrase).field("dwKeyLength", &self.dwKeyLength).field("ucKeyData", &self.ucKeyData).finish()
    }
}
impl ::core::default::Default for DOT11_ACCESSNETWORKOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_ACCESSNETWORKOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.AccessNetworkType == other.AccessNetworkType && self.Internet == other.Internet && self.ASRA == other.ASRA && self.ESR == other.ESR && self.UESA == other.UESA
    }
}
impl ::core::cmp::Eq for DOT11_ACCESSNETWORKOPTIONS {}
impl ::core::fmt::Debug for DOT11_ACCESSNETWORKOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ACCESSNETWORKOPTIONS").field("AccessNetworkType", &self.AccessNetworkType).field("Internet", &self.Internet).field("ASRA", &self.ASRA).field("ESR", &self.ESR).field("UESA", &self.UESA).finish()
    }
}
impl ::core::default::Default for DOT11_AC_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_AC_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_AC_PARAM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_ADAPTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_ADAPTER {
    fn eq(&self, other: &Self) -> bool {
        self.gAdapterId == other.gAdapterId && self.pszDescription == other.pszDescription && self.Dot11CurrentOpMode == other.Dot11CurrentOpMode
    }
}
impl ::core::cmp::Eq for DOT11_ADAPTER {}
impl ::core::fmt::Debug for DOT11_ADAPTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ADAPTER").field("gAdapterId", &self.gAdapterId).field("pszDescription", &self.pszDescription).field("Dot11CurrentOpMode", &self.Dot11CurrentOpMode).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ADDITIONAL_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uBeaconIEsOffset == other.uBeaconIEsOffset && self.uBeaconIEsLength == other.uBeaconIEsLength && self.uResponseIEsOffset == other.uResponseIEsOffset && self.uResponseIEsLength == other.uResponseIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ADDITIONAL_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ADDITIONAL_IE").field("Header", &self.Header).field("uBeaconIEsOffset", &self.uBeaconIEsOffset).field("uBeaconIEsLength", &self.uBeaconIEsLength).field("uResponseIEsOffset", &self.uResponseIEsOffset).field("uResponseIEsLength", &self.uResponseIEsLength).finish()
    }
}
impl ::core::default::Default for DOT11_ADHOC_AUTH_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_ADHOC_AUTH_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_AUTH_ALGORITHM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_ADHOC_CIPHER_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_ADHOC_CIPHER_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_CIPHER_ALGORITHM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_ADHOC_CONNECT_FAIL_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_ADHOC_CONNECT_FAIL_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_CONNECT_FAIL_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_NETWORK_CONNECTION_STATUS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Status == other.Status && self.hContext == other.hContext && self.uResponseLength == other.uResponseLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ANQP_QUERY_COMPLETE_PARAMETERS").field("Header", &self.Header).field("Status", &self.Status).field("hContext", &self.hContext).field("uResponseLength", &self.uResponseLength).finish()
    }
}
impl ::core::default::Default for DOT11_ANQP_QUERY_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_ANQP_QUERY_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ANQP_QUERY_RESULT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_AP_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_AP_JOIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.uJoinFailureTimeout == other.uJoinFailureTimeout && self.OperationalRateSet == other.OperationalRateSet && self.uChCenterFrequency == other.uChCenterFrequency && self.dot11BSSDescription == other.dot11BSSDescription
    }
}
impl ::core::cmp::Eq for DOT11_AP_JOIN_REQUEST {}
impl ::core::fmt::Debug for DOT11_AP_JOIN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AP_JOIN_REQUEST").field("uJoinFailureTimeout", &self.uJoinFailureTimeout).field("OperationalRateSet", &self.OperationalRateSet).field("uChCenterFrequency", &self.uChCenterFrequency).field("dot11BSSDescription", &self.dot11BSSDescription).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.MacAddr == other.MacAddr
            && self.uStatus == other.uStatus
            && self.bReAssocReq == other.bReAssocReq
            && self.bReAssocResp == other.bReAssocResp
            && self.uAssocReqOffset == other.uAssocReqOffset
            && self.uAssocReqSize == other.uAssocReqSize
            && self.uAssocRespOffset == other.uAssocRespOffset
            && self.uAssocRespSize == other.uAssocRespSize
            && self.uBeaconOffset == other.uBeaconOffset
            && self.uBeaconSize == other.uBeaconSize
            && self.uIHVDataOffset == other.uIHVDataOffset
            && self.uIHVDataSize == other.uIHVDataSize
            && self.AuthAlgo == other.AuthAlgo
            && self.UnicastCipher == other.UnicastCipher
            && self.MulticastCipher == other.MulticastCipher
            && self.uActivePhyListOffset == other.uActivePhyListOffset
            && self.uActivePhyListSize == other.uActivePhyListSize
            && self.bFourAddressSupported == other.bFourAddressSupported
            && self.bPortAuthorized == other.bPortAuthorized
            && self.ucActiveQoSProtocol == other.ucActiveQoSProtocol
            && self.DSInfo == other.DSInfo
            && self.uEncapTableOffset == other.uEncapTableOffset
            && self.uEncapTableSize == other.uEncapTableSize
            && self.MulticastMgmtCipher == other.MulticastMgmtCipher
            && self.uAssocComebackTime == other.uAssocComebackTime
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_COMPLETION_PARAMETERS")
            .field("Header", &self.Header)
            .field("MacAddr", &self.MacAddr)
            .field("uStatus", &self.uStatus)
            .field("bReAssocReq", &self.bReAssocReq)
            .field("bReAssocResp", &self.bReAssocResp)
            .field("uAssocReqOffset", &self.uAssocReqOffset)
            .field("uAssocReqSize", &self.uAssocReqSize)
            .field("uAssocRespOffset", &self.uAssocRespOffset)
            .field("uAssocRespSize", &self.uAssocRespSize)
            .field("uBeaconOffset", &self.uBeaconOffset)
            .field("uBeaconSize", &self.uBeaconSize)
            .field("uIHVDataOffset", &self.uIHVDataOffset)
            .field("uIHVDataSize", &self.uIHVDataSize)
            .field("AuthAlgo", &self.AuthAlgo)
            .field("UnicastCipher", &self.UnicastCipher)
            .field("MulticastCipher", &self.MulticastCipher)
            .field("uActivePhyListOffset", &self.uActivePhyListOffset)
            .field("uActivePhyListSize", &self.uActivePhyListSize)
            .field("bFourAddressSupported", &self.bFourAddressSupported)
            .field("bPortAuthorized", &self.bPortAuthorized)
            .field("ucActiveQoSProtocol", &self.ucActiveQoSProtocol)
            .field("DSInfo", &self.DSInfo)
            .field("uEncapTableOffset", &self.uEncapTableOffset)
            .field("uEncapTableSize", &self.uEncapTableSize)
            .field("MulticastMgmtCipher", &self.MulticastMgmtCipher)
            .field("uAssocComebackTime", &self.uAssocComebackTime)
            .finish()
    }
}
impl ::core::default::Default for DOT11_ASSOCIATION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.BSSID == other.BSSID && self.usCapabilityInformation == other.usCapabilityInformation && self.usListenInterval == other.usListenInterval && self.ucPeerSupportedRates == other.ucPeerSupportedRates && self.usAssociationID == other.usAssociationID && self.dot11AssociationState == other.dot11AssociationState && self.dot11PowerMode == other.dot11PowerMode && self.liAssociationUpTime == other.liAssociationUpTime && self.ullNumOfTxPacketSuccesses == other.ullNumOfTxPacketSuccesses && self.ullNumOfTxPacketFailures == other.ullNumOfTxPacketFailures && self.ullNumOfRxPacketSuccesses == other.ullNumOfRxPacketSuccesses && self.ullNumOfRxPacketFailures == other.ullNumOfRxPacketFailures
    }
}
impl ::core::cmp::Eq for DOT11_ASSOCIATION_INFO_EX {}
impl ::core::fmt::Debug for DOT11_ASSOCIATION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_INFO_EX")
            .field("PeerMacAddress", &self.PeerMacAddress)
            .field("BSSID", &self.BSSID)
            .field("usCapabilityInformation", &self.usCapabilityInformation)
            .field("usListenInterval", &self.usListenInterval)
            .field("ucPeerSupportedRates", &self.ucPeerSupportedRates)
            .field("usAssociationID", &self.usAssociationID)
            .field("dot11AssociationState", &self.dot11AssociationState)
            .field("dot11PowerMode", &self.dot11PowerMode)
            .field("liAssociationUpTime", &self.liAssociationUpTime)
            .field("ullNumOfTxPacketSuccesses", &self.ullNumOfTxPacketSuccesses)
            .field("ullNumOfTxPacketFailures", &self.ullNumOfTxPacketFailures)
            .field("ullNumOfRxPacketSuccesses", &self.ullNumOfRxPacketSuccesses)
            .field("ullNumOfRxPacketFailures", &self.ullNumOfRxPacketFailures)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11AssocInfo == other.dot11AssocInfo
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_INFO_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_INFO_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11AssocInfo", &self.dot11AssocInfo).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.BSSID == other.BSSID && self.uAssocRequestIEsOffset == other.uAssocRequestIEsOffset && self.uAssocRequestIEsLength == other.uAssocRequestIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_PARAMS").field("Header", &self.Header).field("BSSID", &self.BSSID).field("uAssocRequestIEsOffset", &self.uAssocRequestIEsOffset).field("uAssocRequestIEsLength", &self.uAssocRequestIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MacAddr == other.MacAddr && self.SSID == other.SSID && self.uIHVDataOffset == other.uIHVDataOffset && self.uIHVDataSize == other.uIHVDataSize
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_START_PARAMETERS").field("Header", &self.Header).field("MacAddr", &self.MacAddr).field("SSID", &self.SSID).field("uIHVDataOffset", &self.uIHVDataOffset).field("uIHVDataSize", &self.uIHVDataSize).finish()
    }
}
impl ::core::default::Default for DOT11_ASSOCIATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_ASSOCIATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ASSOCIATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_AUTH_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_AUTH_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_AUTH_ALGORITHM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AUTH_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AUTH_ALGORITHM_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.AlgorithmIds == other.AlgorithmIds
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AUTH_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AUTH_ALGORITHM_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AUTH_ALGORITHM_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("AlgorithmIds", &self.AlgorithmIds).finish()
    }
}
impl ::core::default::Default for DOT11_AUTH_CIPHER_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_AUTH_CIPHER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.AuthAlgoId == other.AuthAlgoId && self.CipherAlgoId == other.CipherAlgoId
    }
}
impl ::core::cmp::Eq for DOT11_AUTH_CIPHER_PAIR {}
impl ::core::fmt::Debug for DOT11_AUTH_CIPHER_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AUTH_CIPHER_PAIR").field("AuthAlgoId", &self.AuthAlgoId).field("CipherAlgoId", &self.CipherAlgoId).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.AuthCipherPairs == other.AuthCipherPairs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AUTH_CIPHER_PAIR_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AUTH_CIPHER_PAIR_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("AuthCipherPairs", &self.AuthCipherPairs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AVAILABLE_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AVAILABLE_CHANNEL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.uChannelNumber == other.uChannelNumber
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AVAILABLE_CHANNEL_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AVAILABLE_CHANNEL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AVAILABLE_CHANNEL_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("uChannelNumber", &self.uChannelNumber).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.uFrequencyValue == other.uFrequencyValue
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AVAILABLE_FREQUENCY_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AVAILABLE_FREQUENCY_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("uFrequencyValue", &self.uFrequencyValue).finish()
    }
}
impl ::core::default::Default for DOT11_BAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_BAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_BAND").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_BSSID_CANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_BSSID_CANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DOT11_BSSID_CANDIDATE {}
impl ::core::fmt::Debug for DOT11_BSSID_CANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSSID_CANDIDATE").field("BSSID", &self.BSSID).field("uFlags", &self.uFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_BSSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_BSSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.BSSIDs == other.BSSIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_BSSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_BSSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSSID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("BSSIDs", &self.BSSIDs).finish()
    }
}
impl ::core::default::Default for DOT11_BSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_BSS_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11BSSID == other.dot11BSSID && self.dot11BSSType == other.dot11BSSType && self.usBeaconPeriod == other.usBeaconPeriod && self.ullTimestamp == other.ullTimestamp && self.usCapabilityInformation == other.usCapabilityInformation && self.uBufferLength == other.uBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_BSS_DESCRIPTION {}
impl ::core::fmt::Debug for DOT11_BSS_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSS_DESCRIPTION").field("uReserved", &self.uReserved).field("dot11BSSID", &self.dot11BSSID).field("dot11BSSType", &self.dot11BSSType).field("usBeaconPeriod", &self.usBeaconPeriod).field("ullTimestamp", &self.ullTimestamp).field("usCapabilityInformation", &self.usCapabilityInformation).field("uBufferLength", &self.uBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_BSS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOT11_BSS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_BSS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfBytes == other.uNumOfBytes && self.pucBuffer == other.pucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_BSS_LIST {}
impl ::core::fmt::Debug for DOT11_BSS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSS_LIST").field("uNumOfBytes", &self.uNumOfBytes).field("pucBuffer", &self.pucBuffer).finish()
    }
}
impl ::core::default::Default for DOT11_BSS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_BSS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_BSS_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_BYTE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfBytes == other.uNumOfBytes && self.uTotalNumOfBytes == other.uTotalNumOfBytes && self.ucBuffer == other.ucBuffer
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_BYTE_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_BYTE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BYTE_ARRAY").field("Header", &self.Header).field("uNumOfBytes", &self.uNumOfBytes).field("uTotalNumOfBytes", &self.uTotalNumOfBytes).field("ucBuffer", &self.ucBuffer).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ulReason == other.ulReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CAN_SUSTAIN_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CAN_SUSTAIN_AP_PARAMETERS").field("Header", &self.Header).field("ulReason", &self.ulReason).finish()
    }
}
impl ::core::default::Default for DOT11_CHANNEL_HINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_CHANNEL_HINT {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11PhyType == other.Dot11PhyType && self.uChannelNumber == other.uChannelNumber
    }
}
impl ::core::cmp::Eq for DOT11_CHANNEL_HINT {}
impl ::core::fmt::Debug for DOT11_CHANNEL_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CHANNEL_HINT").field("Dot11PhyType", &self.Dot11PhyType).field("uChannelNumber", &self.uChannelNumber).finish()
    }
}
impl ::core::default::Default for DOT11_CIPHER_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_CIPHER_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_CIPHER_ALGORITHM").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CIPHER_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CIPHER_ALGORITHM_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.AlgorithmIds == other.AlgorithmIds
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CIPHER_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CIPHER_ALGORITHM_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CIPHER_ALGORITHM_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("AlgorithmIds", &self.AlgorithmIds).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uKeyIndex == other.uKeyIndex && self.AlgorithmId == other.AlgorithmId && self.MacAddr == other.MacAddr && self.bDelete == other.bDelete && self.bStatic == other.bStatic && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_CIPHER_DEFAULT_KEY_VALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CIPHER_DEFAULT_KEY_VALUE").field("Header", &self.Header).field("uKeyIndex", &self.uKeyIndex).field("AlgorithmId", &self.AlgorithmId).field("MacAddr", &self.MacAddr).field("bDelete", &self.bDelete).field("bStatic", &self.bStatic).field("usKeyLength", &self.usKeyLength).field("ucKey", &self.ucKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddr == other.PeerMacAddr && self.AlgorithmId == other.AlgorithmId && self.Direction == other.Direction && self.bDelete == other.bDelete && self.bStatic == other.bStatic && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CIPHER_KEY_MAPPING_KEY_VALUE").field("PeerMacAddr", &self.PeerMacAddr).field("AlgorithmId", &self.AlgorithmId).field("Direction", &self.Direction).field("bDelete", &self.bDelete).field("bStatic", &self.bStatic).field("usKeyLength", &self.usKeyLength).field("ucKey", &self.ucKey).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uStatus == other.uStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CONNECTION_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CONNECTION_COMPLETION_PARAMETERS").field("Header", &self.Header).field("uStatus", &self.uStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CONNECTION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CONNECTION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.BSSType == other.BSSType && self.AdhocBSSID == other.AdhocBSSID && self.AdhocSSID == other.AdhocSSID
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CONNECTION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CONNECTION_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CONNECTION_START_PARAMETERS").field("Header", &self.Header).field("BSSType", &self.BSSType).field("AdhocBSSID", &self.AdhocBSSID).field("AdhocSSID", &self.AdhocSSID).finish()
    }
}
impl ::core::default::Default for DOT11_COUNTERS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_COUNTERS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.uTransmittedFragmentCount == other.uTransmittedFragmentCount && self.uMulticastTransmittedFrameCount == other.uMulticastTransmittedFrameCount && self.uFailedCount == other.uFailedCount && self.uRetryCount == other.uRetryCount && self.uMultipleRetryCount == other.uMultipleRetryCount && self.uFrameDuplicateCount == other.uFrameDuplicateCount && self.uRTSSuccessCount == other.uRTSSuccessCount && self.uRTSFailureCount == other.uRTSFailureCount && self.uACKFailureCount == other.uACKFailureCount && self.uReceivedFragmentCount == other.uReceivedFragmentCount && self.uMulticastReceivedFrameCount == other.uMulticastReceivedFrameCount && self.uFCSErrorCount == other.uFCSErrorCount && self.uTransmittedFrameCount == other.uTransmittedFrameCount
    }
}
impl ::core::cmp::Eq for DOT11_COUNTERS_ENTRY {}
impl ::core::fmt::Debug for DOT11_COUNTERS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_COUNTERS_ENTRY")
            .field("uTransmittedFragmentCount", &self.uTransmittedFragmentCount)
            .field("uMulticastTransmittedFrameCount", &self.uMulticastTransmittedFrameCount)
            .field("uFailedCount", &self.uFailedCount)
            .field("uRetryCount", &self.uRetryCount)
            .field("uMultipleRetryCount", &self.uMultipleRetryCount)
            .field("uFrameDuplicateCount", &self.uFrameDuplicateCount)
            .field("uRTSSuccessCount", &self.uRTSSuccessCount)
            .field("uRTSFailureCount", &self.uRTSFailureCount)
            .field("uACKFailureCount", &self.uACKFailureCount)
            .field("uReceivedFragmentCount", &self.uReceivedFragmentCount)
            .field("uMulticastReceivedFrameCount", &self.uMulticastReceivedFrameCount)
            .field("uFCSErrorCount", &self.uFCSErrorCount)
            .field("uTransmittedFrameCount", &self.uTransmittedFrameCount)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.CountryOrRegionStrings == other.CountryOrRegionStrings
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_COUNTRY_OR_REGION_STRING_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_COUNTRY_OR_REGION_STRING_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("CountryOrRegionStrings", &self.CountryOrRegionStrings).finish()
    }
}
impl ::core::default::Default for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DOT11_CURRENT_OFFLOAD_CAPABILITY {}
impl ::core::fmt::Debug for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CURRENT_OFFLOAD_CAPABILITY").field("uReserved", &self.uReserved).field("uFlags", &self.uFlags).finish()
    }
}
impl ::core::default::Default for DOT11_CURRENT_OPERATION_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_CURRENT_OPERATION_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uCurrentOpMode == other.uCurrentOpMode
    }
}
impl ::core::cmp::Eq for DOT11_CURRENT_OPERATION_MODE {}
impl ::core::fmt::Debug for DOT11_CURRENT_OPERATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CURRENT_OPERATION_MODE").field("uReserved", &self.uReserved).field("uCurrentOpMode", &self.uCurrentOpMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.bDot11CFPollable == other.bDot11CFPollable && self.bDot11PCF == other.bDot11PCF && self.bDot11PCFMPDUTransferToPC == other.bDot11PCFMPDUTransferToPC && self.bStrictlyOrderedServiceClass == other.bStrictlyOrderedServiceClass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_CURRENT_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CURRENT_OPTIONAL_CAPABILITY").field("uReserved", &self.uReserved).field("bDot11CFPollable", &self.bDot11CFPollable).field("bDot11PCF", &self.bDot11PCF).field("bDot11PCFMPDUTransferToPC", &self.bDot11PCFMPDUTransferToPC).field("bStrictlyOrderedServiceClass", &self.bStrictlyOrderedServiceClass).finish()
    }
}
impl ::core::default::Default for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ucDataRateIndex == other.ucDataRateIndex && self.ucDataRateFlag == other.ucDataRateFlag && self.usDataRateValue == other.usDataRateValue
    }
}
impl ::core::cmp::Eq for DOT11_DATA_RATE_MAPPING_ENTRY {}
impl ::core::fmt::Debug for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DATA_RATE_MAPPING_ENTRY").field("ucDataRateIndex", &self.ucDataRateIndex).field("ucDataRateFlag", &self.ucDataRateFlag).field("usDataRateValue", &self.usDataRateValue).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DATA_RATE_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DATA_RATE_MAPPING_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uDataRateMappingLength == other.uDataRateMappingLength && self.DataRateMappingEntries == other.DataRateMappingEntries
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DATA_RATE_MAPPING_TABLE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_DATA_RATE_MAPPING_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DATA_RATE_MAPPING_TABLE").field("Header", &self.Header).field("uDataRateMappingLength", &self.uDataRateMappingLength).field("DataRateMappingEntries", &self.DataRateMappingEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DEFAULT_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DEFAULT_WEP_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.hOffloadContext == other.hOffloadContext && self.hOffload == other.hOffload && self.dwIndex == other.dwIndex && self.dot11OffloadType == other.dot11OffloadType && self.dwAlgorithm == other.dwAlgorithm && self.uFlags == other.uFlags && self.dot11KeyDirection == other.dot11KeyDirection && self.ucMacAddress == other.ucMacAddress && self.uNumOfRWsOnMe == other.uNumOfRWsOnMe && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DEFAULT_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DEFAULT_WEP_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DEFAULT_WEP_OFFLOAD")
            .field("uReserved", &self.uReserved)
            .field("hOffloadContext", &self.hOffloadContext)
            .field("hOffload", &self.hOffload)
            .field("dwIndex", &self.dwIndex)
            .field("dot11OffloadType", &self.dot11OffloadType)
            .field("dwAlgorithm", &self.dwAlgorithm)
            .field("uFlags", &self.uFlags)
            .field("dot11KeyDirection", &self.dot11KeyDirection)
            .field("ucMacAddress", &self.ucMacAddress)
            .field("uNumOfRWsOnMe", &self.uNumOfRWsOnMe)
            .field("dot11IV48Counters", &self.dot11IV48Counters)
            .field("usDot11RWBitMaps", &self.usDot11RWBitMaps)
            .field("usKeyLength", &self.usKeyLength)
            .field("ucKey", &self.ucKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DEFAULT_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DEFAULT_WEP_UPLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11OffloadType == other.dot11OffloadType && self.hOffload == other.hOffload && self.uNumOfRWsUsed == other.uNumOfRWsUsed && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DEFAULT_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DEFAULT_WEP_UPLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DEFAULT_WEP_UPLOAD").field("uReserved", &self.uReserved).field("dot11OffloadType", &self.dot11OffloadType).field("hOffload", &self.hOffload).field("uNumOfRWsUsed", &self.uNumOfRWsUsed).field("dot11IV48Counters", &self.dot11IV48Counters).field("usDot11RWBitMaps", &self.usDot11RWBitMaps).finish()
    }
}
impl ::core::default::Default for DOT11_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_DIRECTION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.usReason == other.usReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DISASSOCIATE_PEER_REQUEST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DISASSOCIATE_PEER_REQUEST").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("usReason", &self.usReason).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DISASSOCIATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DISASSOCIATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MacAddr == other.MacAddr && self.uReason == other.uReason && self.uIHVDataOffset == other.uIHVDataOffset && self.uIHVDataSize == other.uIHVDataSize
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DISASSOCIATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_DISASSOCIATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DISASSOCIATION_PARAMETERS").field("Header", &self.Header).field("MacAddr", &self.MacAddr).field("uReason", &self.uReason).field("uIHVDataOffset", &self.uIHVDataOffset).field("uIHVDataSize", &self.uIHVDataSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DIVERSITY_SELECTION_RX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DIVERSITY_SELECTION_RX {
    fn eq(&self, other: &Self) -> bool {
        self.uAntennaListIndex == other.uAntennaListIndex && self.bDiversitySelectionRX == other.bDiversitySelectionRX
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DIVERSITY_SELECTION_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DIVERSITY_SELECTION_RX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DIVERSITY_SELECTION_RX").field("uAntennaListIndex", &self.uAntennaListIndex).field("bDiversitySelectionRX", &self.bDiversitySelectionRX).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11DiversitySelectionRx == other.dot11DiversitySelectionRx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DIVERSITY_SELECTION_RX_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DIVERSITY_SELECTION_RX_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11DiversitySelectionRx", &self.dot11DiversitySelectionRx).finish()
    }
}
impl ::core::default::Default for DOT11_DIVERSITY_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_DIVERSITY_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_DIVERSITY_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_DS_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_DS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_DS_INFO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for DOT11_EAP_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for DOT11_EAP_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFailureReasonCode == other.dwFailureReasonCode && self.pAttribArray == other.pAttribArray
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for DOT11_EAP_RESULT {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::fmt::Debug for DOT11_EAP_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EAP_RESULT").field("dwFailureReasonCode", &self.dwFailureReasonCode).field("pAttribArray", &self.pAttribArray).finish()
    }
}
impl ::core::default::Default for DOT11_ENCAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_ENCAP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.usEtherType == other.usEtherType && self.usEncapType == other.usEncapType
    }
}
impl ::core::cmp::Eq for DOT11_ENCAP_ENTRY {}
impl ::core::fmt::Debug for DOT11_ENCAP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ENCAP_ENTRY").field("usEtherType", &self.usEtherType).field("usEncapType", &self.usEncapType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_ERP_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_ERP_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.HRDSSSAttributes == other.HRDSSSAttributes && self.bERPPBCCOptionImplemented == other.bERPPBCCOptionImplemented && self.bDSSSOFDMOptionImplemented == other.bDSSSOFDMOptionImplemented && self.bShortSlotTimeOptionImplemented == other.bShortSlotTimeOptionImplemented
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_ERP_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_ERP_PHY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ERP_PHY_ATTRIBUTES").field("HRDSSSAttributes", &self.HRDSSSAttributes).field("bERPPBCCOptionImplemented", &self.bERPPBCCOptionImplemented).field("bDSSSOFDMOptionImplemented", &self.bDSSSOFDMOptionImplemented).field("bShortSlotTimeOptionImplemented", &self.bShortSlotTimeOptionImplemented).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_EXTAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_EXTAP_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.uScanSSIDListSize == other.uScanSSIDListSize
            && self.uDesiredSSIDListSize == other.uDesiredSSIDListSize
            && self.uPrivacyExemptionListSize == other.uPrivacyExemptionListSize
            && self.uAssociationTableSize == other.uAssociationTableSize
            && self.uDefaultKeyTableSize == other.uDefaultKeyTableSize
            && self.uWEPKeyValueMaxLength == other.uWEPKeyValueMaxLength
            && self.bStrictlyOrderedServiceClassImplemented == other.bStrictlyOrderedServiceClassImplemented
            && self.uNumSupportedCountryOrRegionStrings == other.uNumSupportedCountryOrRegionStrings
            && self.pSupportedCountryOrRegionStrings == other.pSupportedCountryOrRegionStrings
            && self.uInfraNumSupportedUcastAlgoPairs == other.uInfraNumSupportedUcastAlgoPairs
            && self.pInfraSupportedUcastAlgoPairs == other.pInfraSupportedUcastAlgoPairs
            && self.uInfraNumSupportedMcastAlgoPairs == other.uInfraNumSupportedMcastAlgoPairs
            && self.pInfraSupportedMcastAlgoPairs == other.pInfraSupportedMcastAlgoPairs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_EXTAP_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_EXTAP_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTAP_ATTRIBUTES")
            .field("Header", &self.Header)
            .field("uScanSSIDListSize", &self.uScanSSIDListSize)
            .field("uDesiredSSIDListSize", &self.uDesiredSSIDListSize)
            .field("uPrivacyExemptionListSize", &self.uPrivacyExemptionListSize)
            .field("uAssociationTableSize", &self.uAssociationTableSize)
            .field("uDefaultKeyTableSize", &self.uDefaultKeyTableSize)
            .field("uWEPKeyValueMaxLength", &self.uWEPKeyValueMaxLength)
            .field("bStrictlyOrderedServiceClassImplemented", &self.bStrictlyOrderedServiceClassImplemented)
            .field("uNumSupportedCountryOrRegionStrings", &self.uNumSupportedCountryOrRegionStrings)
            .field("pSupportedCountryOrRegionStrings", &self.pSupportedCountryOrRegionStrings)
            .field("uInfraNumSupportedUcastAlgoPairs", &self.uInfraNumSupportedUcastAlgoPairs)
            .field("pInfraSupportedUcastAlgoPairs", &self.pInfraSupportedUcastAlgoPairs)
            .field("uInfraNumSupportedMcastAlgoPairs", &self.uInfraNumSupportedMcastAlgoPairs)
            .field("pInfraSupportedMcastAlgoPairs", &self.pInfraSupportedMcastAlgoPairs)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_EXTSTA_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.uScanSSIDListSize == other.uScanSSIDListSize
            && self.uDesiredBSSIDListSize == other.uDesiredBSSIDListSize
            && self.uDesiredSSIDListSize == other.uDesiredSSIDListSize
            && self.uExcludedMacAddressListSize == other.uExcludedMacAddressListSize
            && self.uPrivacyExemptionListSize == other.uPrivacyExemptionListSize
            && self.uKeyMappingTableSize == other.uKeyMappingTableSize
            && self.uDefaultKeyTableSize == other.uDefaultKeyTableSize
            && self.uWEPKeyValueMaxLength == other.uWEPKeyValueMaxLength
            && self.uPMKIDCacheSize == other.uPMKIDCacheSize
            && self.uMaxNumPerSTADefaultKeyTables == other.uMaxNumPerSTADefaultKeyTables
            && self.bStrictlyOrderedServiceClassImplemented == other.bStrictlyOrderedServiceClassImplemented
            && self.ucSupportedQoSProtocolFlags == other.ucSupportedQoSProtocolFlags
            && self.bSafeModeImplemented == other.bSafeModeImplemented
            && self.uNumSupportedCountryOrRegionStrings == other.uNumSupportedCountryOrRegionStrings
            && self.pSupportedCountryOrRegionStrings == other.pSupportedCountryOrRegionStrings
            && self.uInfraNumSupportedUcastAlgoPairs == other.uInfraNumSupportedUcastAlgoPairs
            && self.pInfraSupportedUcastAlgoPairs == other.pInfraSupportedUcastAlgoPairs
            && self.uInfraNumSupportedMcastAlgoPairs == other.uInfraNumSupportedMcastAlgoPairs
            && self.pInfraSupportedMcastAlgoPairs == other.pInfraSupportedMcastAlgoPairs
            && self.uAdhocNumSupportedUcastAlgoPairs == other.uAdhocNumSupportedUcastAlgoPairs
            && self.pAdhocSupportedUcastAlgoPairs == other.pAdhocSupportedUcastAlgoPairs
            && self.uAdhocNumSupportedMcastAlgoPairs == other.uAdhocNumSupportedMcastAlgoPairs
            && self.pAdhocSupportedMcastAlgoPairs == other.pAdhocSupportedMcastAlgoPairs
            && self.bAutoPowerSaveMode == other.bAutoPowerSaveMode
            && self.uMaxNetworkOffloadListSize == other.uMaxNetworkOffloadListSize
            && self.bMFPCapable == other.bMFPCapable
            && self.uInfraNumSupportedMcastMgmtAlgoPairs == other.uInfraNumSupportedMcastMgmtAlgoPairs
            && self.pInfraSupportedMcastMgmtAlgoPairs == other.pInfraSupportedMcastMgmtAlgoPairs
            && self.bNeighborReportSupported == other.bNeighborReportSupported
            && self.bAPChannelReportSupported == other.bAPChannelReportSupported
            && self.bActionFramesSupported == other.bActionFramesSupported
            && self.bANQPQueryOffloadSupported == other.bANQPQueryOffloadSupported
            && self.bHESSIDConnectionSupported == other.bHESSIDConnectionSupported
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_EXTSTA_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_EXTSTA_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_ATTRIBUTES")
            .field("Header", &self.Header)
            .field("uScanSSIDListSize", &self.uScanSSIDListSize)
            .field("uDesiredBSSIDListSize", &self.uDesiredBSSIDListSize)
            .field("uDesiredSSIDListSize", &self.uDesiredSSIDListSize)
            .field("uExcludedMacAddressListSize", &self.uExcludedMacAddressListSize)
            .field("uPrivacyExemptionListSize", &self.uPrivacyExemptionListSize)
            .field("uKeyMappingTableSize", &self.uKeyMappingTableSize)
            .field("uDefaultKeyTableSize", &self.uDefaultKeyTableSize)
            .field("uWEPKeyValueMaxLength", &self.uWEPKeyValueMaxLength)
            .field("uPMKIDCacheSize", &self.uPMKIDCacheSize)
            .field("uMaxNumPerSTADefaultKeyTables", &self.uMaxNumPerSTADefaultKeyTables)
            .field("bStrictlyOrderedServiceClassImplemented", &self.bStrictlyOrderedServiceClassImplemented)
            .field("ucSupportedQoSProtocolFlags", &self.ucSupportedQoSProtocolFlags)
            .field("bSafeModeImplemented", &self.bSafeModeImplemented)
            .field("uNumSupportedCountryOrRegionStrings", &self.uNumSupportedCountryOrRegionStrings)
            .field("pSupportedCountryOrRegionStrings", &self.pSupportedCountryOrRegionStrings)
            .field("uInfraNumSupportedUcastAlgoPairs", &self.uInfraNumSupportedUcastAlgoPairs)
            .field("pInfraSupportedUcastAlgoPairs", &self.pInfraSupportedUcastAlgoPairs)
            .field("uInfraNumSupportedMcastAlgoPairs", &self.uInfraNumSupportedMcastAlgoPairs)
            .field("pInfraSupportedMcastAlgoPairs", &self.pInfraSupportedMcastAlgoPairs)
            .field("uAdhocNumSupportedUcastAlgoPairs", &self.uAdhocNumSupportedUcastAlgoPairs)
            .field("pAdhocSupportedUcastAlgoPairs", &self.pAdhocSupportedUcastAlgoPairs)
            .field("uAdhocNumSupportedMcastAlgoPairs", &self.uAdhocNumSupportedMcastAlgoPairs)
            .field("pAdhocSupportedMcastAlgoPairs", &self.pAdhocSupportedMcastAlgoPairs)
            .field("bAutoPowerSaveMode", &self.bAutoPowerSaveMode)
            .field("uMaxNetworkOffloadListSize", &self.uMaxNetworkOffloadListSize)
            .field("bMFPCapable", &self.bMFPCapable)
            .field("uInfraNumSupportedMcastMgmtAlgoPairs", &self.uInfraNumSupportedMcastMgmtAlgoPairs)
            .field("pInfraSupportedMcastMgmtAlgoPairs", &self.pInfraSupportedMcastMgmtAlgoPairs)
            .field("bNeighborReportSupported", &self.bNeighborReportSupported)
            .field("bAPChannelReportSupported", &self.bAPChannelReportSupported)
            .field("bActionFramesSupported", &self.bActionFramesSupported)
            .field("bANQPQueryOffloadSupported", &self.bANQPQueryOffloadSupported)
            .field("bHESSIDConnectionSupported", &self.bHESSIDConnectionSupported)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uScanSSIDListSize == other.uScanSSIDListSize && self.uDesiredBSSIDListSize == other.uDesiredBSSIDListSize && self.uDesiredSSIDListSize == other.uDesiredSSIDListSize && self.uExcludedMacAddressListSize == other.uExcludedMacAddressListSize && self.uPrivacyExemptionListSize == other.uPrivacyExemptionListSize && self.uKeyMappingTableSize == other.uKeyMappingTableSize && self.uDefaultKeyTableSize == other.uDefaultKeyTableSize && self.uWEPKeyValueMaxLength == other.uWEPKeyValueMaxLength && self.uPMKIDCacheSize == other.uPMKIDCacheSize && self.uMaxNumPerSTADefaultKeyTables == other.uMaxNumPerSTADefaultKeyTables
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_CAPABILITY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_EXTSTA_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_CAPABILITY")
            .field("Header", &self.Header)
            .field("uScanSSIDListSize", &self.uScanSSIDListSize)
            .field("uDesiredBSSIDListSize", &self.uDesiredBSSIDListSize)
            .field("uDesiredSSIDListSize", &self.uDesiredSSIDListSize)
            .field("uExcludedMacAddressListSize", &self.uExcludedMacAddressListSize)
            .field("uPrivacyExemptionListSize", &self.uPrivacyExemptionListSize)
            .field("uKeyMappingTableSize", &self.uKeyMappingTableSize)
            .field("uDefaultKeyTableSize", &self.uDefaultKeyTableSize)
            .field("uWEPKeyValueMaxLength", &self.uWEPKeyValueMaxLength)
            .field("uPMKIDCacheSize", &self.uPMKIDCacheSize)
            .field("uMaxNumPerSTADefaultKeyTables", &self.uMaxNumPerSTADefaultKeyTables)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_RECV_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_RECV_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uReceiveFlags == other.uReceiveFlags && self.uPhyId == other.uPhyId && self.uChCenterFrequency == other.uChCenterFrequency && self.usNumberOfMPDUsReceived == other.usNumberOfMPDUsReceived && self.lRSSI == other.lRSSI && self.ucDataRate == other.ucDataRate && self.uSizeMediaSpecificInfo == other.uSizeMediaSpecificInfo && self.pvMediaSpecificInfo == other.pvMediaSpecificInfo && self.ullTimestamp == other.ullTimestamp
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_RECV_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_EXTSTA_RECV_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_RECV_CONTEXT")
            .field("Header", &self.Header)
            .field("uReceiveFlags", &self.uReceiveFlags)
            .field("uPhyId", &self.uPhyId)
            .field("uChCenterFrequency", &self.uChCenterFrequency)
            .field("usNumberOfMPDUsReceived", &self.usNumberOfMPDUsReceived)
            .field("lRSSI", &self.lRSSI)
            .field("ucDataRate", &self.ucDataRate)
            .field("uSizeMediaSpecificInfo", &self.uSizeMediaSpecificInfo)
            .field("pvMediaSpecificInfo", &self.pvMediaSpecificInfo)
            .field("ullTimestamp", &self.ullTimestamp)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_SEND_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_SEND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.usExemptionActionType == other.usExemptionActionType && self.uPhyId == other.uPhyId && self.uDelayedSleepValue == other.uDelayedSleepValue && self.pvMediaSpecificInfo == other.pvMediaSpecificInfo && self.uSendFlags == other.uSendFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_SEND_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_EXTSTA_SEND_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_SEND_CONTEXT").field("Header", &self.Header).field("usExemptionActionType", &self.usExemptionActionType).field("uPhyId", &self.uPhyId).field("uDelayedSleepValue", &self.uDelayedSleepValue).field("pvMediaSpecificInfo", &self.pvMediaSpecificInfo).field("uSendFlags", &self.uSendFlags).finish()
    }
}
impl ::core::default::Default for DOT11_FRAGMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_FRAGMENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.uOffset == other.uOffset && self.uLength == other.uLength
    }
}
impl ::core::cmp::Eq for DOT11_FRAGMENT_DESCRIPTOR {}
impl ::core::fmt::Debug for DOT11_FRAGMENT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_FRAGMENT_DESCRIPTOR").field("uOffset", &self.uOffset).field("uLength", &self.uLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
impl ::core::default::Default for DOT11_HOPPING_PATTERN_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_HOPPING_PATTERN_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.uHoppingPatternIndex == other.uHoppingPatternIndex && self.uRandomTableFieldNumber == other.uRandomTableFieldNumber
    }
}
impl ::core::cmp::Eq for DOT11_HOPPING_PATTERN_ENTRY {}
impl ::core::fmt::Debug for DOT11_HOPPING_PATTERN_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_HOPPING_PATTERN_ENTRY").field("uHoppingPatternIndex", &self.uHoppingPatternIndex).field("uRandomTableFieldNumber", &self.uRandomTableFieldNumber).finish()
    }
}
impl ::core::default::Default for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11HoppingPatternEntry == other.dot11HoppingPatternEntry
    }
}
impl ::core::cmp::Eq for DOT11_HOPPING_PATTERN_ENTRY_LIST {}
impl ::core::fmt::Debug for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_HOPPING_PATTERN_ENTRY_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11HoppingPatternEntry", &self.dot11HoppingPatternEntry).finish()
    }
}
impl ::core::default::Default for DOT11_HOP_ALGO_ADOPTED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_HOP_ALGO_ADOPTED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_HOP_ALGO_ADOPTED").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.bShortPreambleOptionImplemented == other.bShortPreambleOptionImplemented && self.bPBCCOptionImplemented == other.bPBCCOptionImplemented && self.bChannelAgilityPresent == other.bChannelAgilityPresent && self.uHRCCAModeSupported == other.uHRCCAModeSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_HRDSSS_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_HRDSSS_PHY_ATTRIBUTES").field("bShortPreambleOptionImplemented", &self.bShortPreambleOptionImplemented).field("bPBCCOptionImplemented", &self.bPBCCOptionImplemented).field("bChannelAgilityPresent", &self.bChannelAgilityPresent).field("uHRCCAModeSupported", &self.uHRCCAModeSupported).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_IBSS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_IBSS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bJoinOnly == other.bJoinOnly && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_IBSS_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_IBSS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_IBSS_PARAMS").field("Header", &self.Header).field("bJoinOnly", &self.bJoinOnly).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
impl ::core::default::Default for DOT11_IHV_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_IHV_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVerMin == other.dwVerMin && self.dwVerMax == other.dwVerMax
    }
}
impl ::core::cmp::Eq for DOT11_IHV_VERSION_INFO {}
impl ::core::fmt::Debug for DOT11_IHV_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_IHV_VERSION_INFO").field("dwVerMin", &self.dwVerMin).field("dwVerMax", &self.dwVerMax).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.uStatus == other.uStatus && self.ucErrorSource == other.ucErrorSource && self.bReAssocReq == other.bReAssocReq && self.bReAssocResp == other.bReAssocResp && self.uAssocReqOffset == other.uAssocReqOffset && self.uAssocReqSize == other.uAssocReqSize && self.uAssocRespOffset == other.uAssocRespOffset && self.uAssocRespSize == other.uAssocRespSize && self.AuthAlgo == other.AuthAlgo && self.UnicastCipher == other.UnicastCipher && self.MulticastCipher == other.MulticastCipher && self.uActivePhyListOffset == other.uActivePhyListOffset && self.uActivePhyListSize == other.uActivePhyListSize && self.uBeaconOffset == other.uBeaconOffset && self.uBeaconSize == other.uBeaconSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerMacAddr", &self.PeerMacAddr)
            .field("uStatus", &self.uStatus)
            .field("ucErrorSource", &self.ucErrorSource)
            .field("bReAssocReq", &self.bReAssocReq)
            .field("bReAssocResp", &self.bReAssocResp)
            .field("uAssocReqOffset", &self.uAssocReqOffset)
            .field("uAssocReqSize", &self.uAssocReqSize)
            .field("uAssocRespOffset", &self.uAssocRespOffset)
            .field("uAssocRespSize", &self.uAssocRespSize)
            .field("AuthAlgo", &self.AuthAlgo)
            .field("UnicastCipher", &self.UnicastCipher)
            .field("MulticastCipher", &self.MulticastCipher)
            .field("uActivePhyListOffset", &self.uActivePhyListOffset)
            .field("uActivePhyListSize", &self.uActivePhyListSize)
            .field("uBeaconOffset", &self.uBeaconOffset)
            .field("uBeaconSize", &self.uBeaconSize)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_DECISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_DECISION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.bAccept == other.bAccept && self.usReasonCode == other.usReasonCode && self.uAssocResponseIEsOffset == other.uAssocResponseIEsOffset && self.uAssocResponseIEsLength == other.uAssocResponseIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_DECISION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_DECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_DECISION").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("bAccept", &self.bAccept).field("usReasonCode", &self.usReasonCode).field("uAssocResponseIEsOffset", &self.uAssocResponseIEsOffset).field("uAssocResponseIEsLength", &self.uAssocResponseIEsLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.bAccept == other.bAccept && self.usReasonCode == other.usReasonCode && self.uAssocResponseIEsOffset == other.uAssocResponseIEsOffset && self.uAssocResponseIEsLength == other.uAssocResponseIEsLength && self.WFDStatus == other.WFDStatus
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_DECISION_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_DECISION_V2").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("bAccept", &self.bAccept).field("usReasonCode", &self.usReasonCode).field("uAssocResponseIEsOffset", &self.uAssocResponseIEsOffset).field("uAssocResponseIEsLength", &self.uAssocResponseIEsLength).field("WFDStatus", &self.WFDStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.bReAssocReq == other.bReAssocReq && self.uAssocReqOffset == other.uAssocReqOffset && self.uAssocReqSize == other.uAssocReqSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("bReAssocReq", &self.bReAssocReq).field("uAssocReqOffset", &self.uAssocReqOffset).field("uAssocReqSize", &self.uAssocReqSize).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_STARTED_PARAMETERS").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.ReceiverAddress == other.ReceiverAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("ReceiverAddress", &self.ReceiverAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
impl ::core::default::Default for DOT11_IV48_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_IV48_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.uIV32Counter == other.uIV32Counter && self.usIV16Counter == other.usIV16Counter
    }
}
impl ::core::cmp::Eq for DOT11_IV48_COUNTER {}
impl ::core::fmt::Debug for DOT11_IV48_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_IV48_COUNTER").field("uIV32Counter", &self.uIV32Counter).field("usIV16Counter", &self.usIV16Counter).finish()
    }
}
impl ::core::default::Default for DOT11_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_JOIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.uJoinFailureTimeout == other.uJoinFailureTimeout && self.OperationalRateSet == other.OperationalRateSet && self.uChCenterFrequency == other.uChCenterFrequency && self.dot11BSSDescription == other.dot11BSSDescription
    }
}
impl ::core::cmp::Eq for DOT11_JOIN_REQUEST {}
impl ::core::fmt::Debug for DOT11_JOIN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_JOIN_REQUEST").field("uJoinFailureTimeout", &self.uJoinFailureTimeout).field("OperationalRateSet", &self.OperationalRateSet).field("uChCenterFrequency", &self.uChCenterFrequency).field("dot11BSSDescription", &self.dot11BSSDescription).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_ALGO_BIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_BIP {
    fn eq(&self, other: &Self) -> bool {
        self.ucIPN == other.ucIPN && self.ulBIPKeyLength == other.ulBIPKeyLength && self.ucBIPKey == other.ucBIPKey
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_BIP {}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_BIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_BIP").field("ucIPN", &self.ucIPN).field("ulBIPKeyLength", &self.ulBIPKeyLength).field("ucBIPKey", &self.ucBIPKey).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn eq(&self, other: &Self) -> bool {
        self.ucIPN == other.ucIPN && self.ulBIPGmac256KeyLength == other.ulBIPGmac256KeyLength && self.ucBIPGmac256Key == other.ucBIPGmac256Key
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_BIP_GMAC_256 {}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_BIP_GMAC_256").field("ucIPN", &self.ucIPN).field("ulBIPGmac256KeyLength", &self.ulBIPGmac256KeyLength).field("ucBIPGmac256Key", &self.ucBIPGmac256Key).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_ALGO_CCMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_CCMP {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulCCMPKeyLength == other.ulCCMPKeyLength && self.ucCCMPKey == other.ucCCMPKey
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_CCMP {}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_CCMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_CCMP").field("ucIV48Counter", &self.ucIV48Counter).field("ulCCMPKeyLength", &self.ulCCMPKeyLength).field("ucCCMPKey", &self.ucCCMPKey).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_ALGO_GCMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_GCMP {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulGCMPKeyLength == other.ulGCMPKeyLength && self.ucGCMPKey == other.ucGCMPKey
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_GCMP {}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_GCMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_GCMP").field("ucIV48Counter", &self.ucIV48Counter).field("ulGCMPKeyLength", &self.ulGCMPKeyLength).field("ucGCMPKey", &self.ucGCMPKey).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_ALGO_GCMP_256 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_GCMP_256 {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulGCMP256KeyLength == other.ulGCMP256KeyLength && self.ucGCMP256Key == other.ucGCMP256Key
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_GCMP_256 {}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_GCMP_256 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_GCMP_256").field("ucIV48Counter", &self.ucIV48Counter).field("ulGCMP256KeyLength", &self.ulGCMP256KeyLength).field("ucGCMP256Key", &self.ucGCMP256Key).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_ALGO_TKIP_MIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_TKIP_MIC {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulTKIPKeyLength == other.ulTKIPKeyLength && self.ulMICKeyLength == other.ulMICKeyLength && self.ucTKIPMICKeys == other.ucTKIPMICKeys
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_TKIP_MIC {}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_TKIP_MIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_TKIP_MIC").field("ucIV48Counter", &self.ucIV48Counter).field("ulTKIPKeyLength", &self.ulTKIPKeyLength).field("ulMICKeyLength", &self.ulMICKeyLength).field("ucTKIPMICKeys", &self.ucTKIPMICKeys).finish()
    }
}
impl ::core::default::Default for DOT11_KEY_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_KEY_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_KEY_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_LINK_QUALITY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_LINK_QUALITY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddr == other.PeerMacAddr && self.ucLinkQuality == other.ucLinkQuality
    }
}
impl ::core::cmp::Eq for DOT11_LINK_QUALITY_ENTRY {}
impl ::core::fmt::Debug for DOT11_LINK_QUALITY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_LINK_QUALITY_ENTRY").field("PeerMacAddr", &self.PeerMacAddr).field("ucLinkQuality", &self.ucLinkQuality).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_LINK_QUALITY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_LINK_QUALITY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uLinkQualityListSize == other.uLinkQualityListSize && self.uLinkQualityListOffset == other.uLinkQualityListOffset
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_LINK_QUALITY_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_LINK_QUALITY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_LINK_QUALITY_PARAMETERS").field("Header", &self.Header).field("uLinkQualityListSize", &self.uLinkQualityListSize).field("uLinkQualityListOffset", &self.uLinkQualityListOffset).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MAC_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MAC_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.MacAddrs == other.MacAddrs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MAC_ADDRESS_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MAC_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_ADDRESS_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("MacAddrs", &self.MacAddrs).finish()
    }
}
impl ::core::default::Default for DOT11_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MAC_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount
            && self.ullReceivedFrameCount == other.ullReceivedFrameCount
            && self.ullTransmittedFailureFrameCount == other.ullTransmittedFailureFrameCount
            && self.ullReceivedFailureFrameCount == other.ullReceivedFailureFrameCount
            && self.ullWEPExcludedCount == other.ullWEPExcludedCount
            && self.ullTKIPLocalMICFailures == other.ullTKIPLocalMICFailures
            && self.ullTKIPReplays == other.ullTKIPReplays
            && self.ullTKIPICVErrorCount == other.ullTKIPICVErrorCount
            && self.ullCCMPReplays == other.ullCCMPReplays
            && self.ullCCMPDecryptErrors == other.ullCCMPDecryptErrors
            && self.ullWEPUndecryptableCount == other.ullWEPUndecryptableCount
            && self.ullWEPICVErrorCount == other.ullWEPICVErrorCount
            && self.ullDecryptSuccessCount == other.ullDecryptSuccessCount
            && self.ullDecryptFailureCount == other.ullDecryptFailureCount
    }
}
impl ::core::cmp::Eq for DOT11_MAC_FRAME_STATISTICS {}
impl ::core::fmt::Debug for DOT11_MAC_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullTransmittedFailureFrameCount", &self.ullTransmittedFailureFrameCount)
            .field("ullReceivedFailureFrameCount", &self.ullReceivedFailureFrameCount)
            .field("ullWEPExcludedCount", &self.ullWEPExcludedCount)
            .field("ullTKIPLocalMICFailures", &self.ullTKIPLocalMICFailures)
            .field("ullTKIPReplays", &self.ullTKIPReplays)
            .field("ullTKIPICVErrorCount", &self.ullTKIPICVErrorCount)
            .field("ullCCMPReplays", &self.ullCCMPReplays)
            .field("ullCCMPDecryptErrors", &self.ullCCMPDecryptErrors)
            .field("ullWEPUndecryptableCount", &self.ullWEPUndecryptableCount)
            .field("ullWEPICVErrorCount", &self.ullWEPICVErrorCount)
            .field("ullDecryptSuccessCount", &self.ullDecryptSuccessCount)
            .field("ullDecryptFailureCount", &self.ullDecryptFailureCount)
            .finish()
    }
}
impl ::core::default::Default for DOT11_MAC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MAC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uNdisPortNumber == other.uNdisPortNumber && self.MacAddr == other.MacAddr
    }
}
impl ::core::cmp::Eq for DOT11_MAC_INFO {}
impl ::core::fmt::Debug for DOT11_MAC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_INFO").field("uReserved", &self.uReserved).field("uNdisPortNumber", &self.uNdisPortNumber).field("MacAddr", &self.MacAddr).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MAC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MAC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uOpmodeMask == other.uOpmodeMask
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MAC_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MAC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_PARAMETERS").field("Header", &self.Header).field("uOpmodeMask", &self.uOpmodeMask).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dot11ManufacturingCallbackType == other.dot11ManufacturingCallbackType && self.uStatus == other.uStatus && self.pvContext == other.pvContext
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_CALLBACK_PARAMETERS").field("Header", &self.Header).field("dot11ManufacturingCallbackType", &self.dot11ManufacturingCallbackType).field("uStatus", &self.uStatus).field("pvContext", &self.pvContext).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_CALLBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_CALLBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MANUFACTURING_CALLBACK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11Band == other.Dot11Band && self.uChannel == other.uChannel && self.ADCPowerLevel == other.ADCPowerLevel
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC").field("Dot11Band", &self.Dot11Band).field("uChannel", &self.uChannel).field("ADCPowerLevel", &self.ADCPowerLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn eq(&self, other: &Self) -> bool {
        self.bEnabled == other.bEnabled && self.Dot11Band == other.Dot11Band && self.uChannel == other.uChannel && self.PowerLevel == other.PowerLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX").field("bEnabled", &self.bEnabled).field("Dot11Band", &self.Dot11Band).field("uChannel", &self.uChannel).field("PowerLevel", &self.PowerLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn eq(&self, other: &Self) -> bool {
        self.bEnable == other.bEnable && self.bOpenLoop == other.bOpenLoop && self.Dot11Band == other.Dot11Band && self.uChannel == other.uChannel && self.uSetPowerLevel == other.uSetPowerLevel && self.ADCPowerLevel == other.ADCPowerLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX").field("bEnable", &self.bEnable).field("bOpenLoop", &self.bOpenLoop).field("Dot11Band", &self.Dot11Band).field("uChannel", &self.uChannel).field("uSetPowerLevel", &self.uSetPowerLevel).field("ADCPowerLevel", &self.ADCPowerLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        self.SelfTestType == other.SelfTestType && self.uTestID == other.uTestID && self.bResult == other.bResult && self.uPinFailedBitMask == other.uPinFailedBitMask && self.pvContext == other.pvContext && self.uBytesWrittenOut == other.uBytesWrittenOut && self.ucBufferOut == other.ucBufferOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS").field("SelfTestType", &self.SelfTestType).field("uTestID", &self.uTestID).field("bResult", &self.bResult).field("uPinFailedBitMask", &self.uPinFailedBitMask).field("pvContext", &self.pvContext).field("uBytesWrittenOut", &self.uBytesWrittenOut).field("ucBufferOut", &self.ucBufferOut).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SelfTestType == other.SelfTestType && self.uTestID == other.uTestID && self.uPinBitMask == other.uPinBitMask && self.pvContext == other.pvContext && self.uBufferLength == other.uBufferLength && self.ucBufferIn == other.ucBufferIn
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS").field("SelfTestType", &self.SelfTestType).field("uTestID", &self.uTestID).field("uPinBitMask", &self.uPinBitMask).field("pvContext", &self.pvContext).field("uBufferLength", &self.uBufferLength).field("ucBufferIn", &self.ucBufferIn).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_SELF_TEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MANUFACTURING_SELF_TEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ManufacturingTestType == other.dot11ManufacturingTestType && self.uBufferLength == other.uBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST {}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST").field("dot11ManufacturingTestType", &self.dot11ManufacturingTestType).field("uBufferLength", &self.uBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uKey == other.uKey && self.uOffset == other.uOffset && self.uBufferLength == other.uBufferLength && self.uBytesRead == other.uBytesRead && self.ucBufferOut == other.ucBufferOut
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_QUERY_DATA {}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST_QUERY_DATA").field("uKey", &self.uKey).field("uOffset", &self.uOffset).field("uBufferLength", &self.uBufferLength).field("uBytesRead", &self.uBytesRead).field("ucBufferOut", &self.ucBufferOut).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uKey == other.uKey && self.uOffset == other.uOffset && self.uBufferLength == other.uBufferLength && self.ucBufferIn == other.ucBufferIn
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_SET_DATA {}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST_SET_DATA").field("uKey", &self.uKey).field("uOffset", &self.uOffset).field("uBufferLength", &self.uBufferLength).field("ucBufferIn", &self.ucBufferIn).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_SLEEP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_SLEEP {
    fn eq(&self, other: &Self) -> bool {
        self.uSleepTime == other.uSleepTime && self.pvContext == other.pvContext
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_SLEEP {}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_SLEEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST_SLEEP").field("uSleepTime", &self.uSleepTime).field("pvContext", &self.pvContext).finish()
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MANUFACTURING_TEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11MDCapabilityEntry == other.dot11MDCapabilityEntry
    }
}
impl ::core::cmp::Eq for DOT11_MD_CAPABILITY_ENTRY_LIST {}
impl ::core::fmt::Debug for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MD_CAPABILITY_ENTRY_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11MDCapabilityEntry", &self.dot11MDCapabilityEntry).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uPhyId == other.uPhyId && self.uMPDUMaxLength == other.uMPDUMaxLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MPDU_MAX_LENGTH_INDICATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MPDU_MAX_LENGTH_INDICATION").field("Header", &self.Header).field("uPhyId", &self.uPhyId).field("uMPDUMaxLength", &self.uMPDUMaxLength).finish()
    }
}
impl ::core::default::Default for DOT11_MSONEX_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_MSONEX_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MSONEX_RESULT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for DOT11_MSONEX_RESULT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for DOT11_MSONEX_RESULT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11OnexAuthStatus == other.Dot11OnexAuthStatus && self.Dot11OneXReasonCode == other.Dot11OneXReasonCode && self.pbMPPESendKey == other.pbMPPESendKey && self.dwMPPESendKeyLen == other.dwMPPESendKeyLen && self.pbMPPERecvKey == other.pbMPPERecvKey && self.dwMPPERecvKeyLen == other.dwMPPERecvKeyLen && self.pDot11EapResult == other.pDot11EapResult
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for DOT11_MSONEX_RESULT_PARAMS {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::fmt::Debug for DOT11_MSONEX_RESULT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MSONEX_RESULT_PARAMS").field("Dot11OnexAuthStatus", &self.Dot11OnexAuthStatus).field("Dot11OneXReasonCode", &self.Dot11OneXReasonCode).field("pbMPPESendKey", &self.pbMPPESendKey).field("dwMPPESendKeyLen", &self.dwMPPESendKeyLen).field("pbMPPERecvKey", &self.pbMPPERecvKey).field("dwMPPERecvKeyLen", &self.dwMPPERecvKeyLen).field("pDot11EapResult", &self.pDot11EapResult).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11_MSSECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::PartialEq for DOT11_MSSECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11AuthAlgorithm == other.dot11AuthAlgorithm && self.dot11CipherAlgorithm == other.dot11CipherAlgorithm && self.fOneXEnabled == other.fOneXEnabled && self.eapMethodType == other.eapMethodType && self.dwEapConnectionDataLen == other.dwEapConnectionDataLen && self.pEapConnectionData == other.pEapConnectionData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::Eq for DOT11_MSSECURITY_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11_MSSECURITY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MSSECURITY_SETTINGS").field("dot11AuthAlgorithm", &self.dot11AuthAlgorithm).field("dot11CipherAlgorithm", &self.dot11CipherAlgorithm).field("fOneXEnabled", &self.fOneXEnabled).field("eapMethodType", &self.eapMethodType).field("dwEapConnectionDataLen", &self.dwEapConnectionDataLen).field("pEapConnectionData", &self.pEapConnectionData).finish()
    }
}
impl ::core::default::Default for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.uMultiDomainCapabilityIndex == other.uMultiDomainCapabilityIndex && self.uFirstChannelNumber == other.uFirstChannelNumber && self.uNumberOfChannels == other.uNumberOfChannels && self.lMaximumTransmitPowerLevel == other.lMaximumTransmitPowerLevel
    }
}
impl ::core::cmp::Eq for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {}
impl ::core::fmt::Debug for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY").field("uMultiDomainCapabilityIndex", &self.uMultiDomainCapabilityIndex).field("uFirstChannelNumber", &self.uFirstChannelNumber).field("uNumberOfChannels", &self.uNumberOfChannels).field("lMaximumTransmitPowerLevel", &self.lMaximumTransmitPowerLevel).finish()
    }
}
impl ::core::default::Default for DOT11_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType
    }
}
impl ::core::cmp::Eq for DOT11_NETWORK {}
impl ::core::fmt::Debug for DOT11_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_NETWORK").field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).finish()
    }
}
impl ::core::default::Default for DOT11_NETWORK_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_NETWORK_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.Network == other.Network
    }
}
impl ::core::cmp::Eq for DOT11_NETWORK_LIST {}
impl ::core::fmt::Debug for DOT11_NETWORK_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_NETWORK_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("Network", &self.Network).finish()
    }
}
impl ::core::default::Default for DOT11_NIC_SPECIFIC_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_NIC_SPECIFIC_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.uBufferLength == other.uBufferLength && self.uTotalBufferLength == other.uTotalBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_NIC_SPECIFIC_EXTENSION {}
impl ::core::fmt::Debug for DOT11_NIC_SPECIFIC_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_NIC_SPECIFIC_EXTENSION").field("uBufferLength", &self.uBufferLength).field("uTotalBufferLength", &self.uTotalBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::core::default::Default for DOT11_OFDM_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_OFDM_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.uFrequencyBandsSupported == other.uFrequencyBandsSupported
    }
}
impl ::core::cmp::Eq for DOT11_OFDM_PHY_ATTRIBUTES {}
impl ::core::fmt::Debug for DOT11_OFDM_PHY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFDM_PHY_ATTRIBUTES").field("uFrequencyBandsSupported", &self.uFrequencyBandsSupported).finish()
    }
}
impl ::core::default::Default for DOT11_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uFlags == other.uFlags && self.uSupportedWEPAlgorithms == other.uSupportedWEPAlgorithms && self.uNumOfReplayWindows == other.uNumOfReplayWindows && self.uMaxWEPKeyMappingLength == other.uMaxWEPKeyMappingLength && self.uSupportedAuthAlgorithms == other.uSupportedAuthAlgorithms && self.uMaxAuthKeyMappingLength == other.uMaxAuthKeyMappingLength
    }
}
impl ::core::cmp::Eq for DOT11_OFFLOAD_CAPABILITY {}
impl ::core::fmt::Debug for DOT11_OFFLOAD_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_CAPABILITY").field("uReserved", &self.uReserved).field("uFlags", &self.uFlags).field("uSupportedWEPAlgorithms", &self.uSupportedWEPAlgorithms).field("uNumOfReplayWindows", &self.uNumOfReplayWindows).field("uMaxWEPKeyMappingLength", &self.uMaxWEPKeyMappingLength).field("uSupportedAuthAlgorithms", &self.uSupportedAuthAlgorithms).field("uMaxAuthKeyMappingLength", &self.uMaxAuthKeyMappingLength).finish()
    }
}
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        self.Ssid == other.Ssid && self.UnicastCipher == other.UnicastCipher && self.AuthAlgo == other.AuthAlgo && self.Dot11ChannelHints == other.Dot11ChannelHints
    }
}
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK {}
impl ::core::fmt::Debug for DOT11_OFFLOAD_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_NETWORK").field("Ssid", &self.Ssid).field("UnicastCipher", &self.UnicastCipher).field("AuthAlgo", &self.AuthAlgo).field("Dot11ChannelHints", &self.Dot11ChannelHints).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ulFlags == other.ulFlags && self.FastScanPeriod == other.FastScanPeriod && self.FastScanIterations == other.FastScanIterations && self.SlowScanPeriod == other.SlowScanPeriod && self.uNumOfEntries == other.uNumOfEntries && self.offloadNetworkList == other.offloadNetworkList
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK_LIST_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_NETWORK_LIST_INFO").field("Header", &self.Header).field("ulFlags", &self.ulFlags).field("FastScanPeriod", &self.FastScanPeriod).field("FastScanIterations", &self.FastScanIterations).field("SlowScanPeriod", &self.SlowScanPeriod).field("uNumOfEntries", &self.uNumOfEntries).field("offloadNetworkList", &self.offloadNetworkList).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Status == other.Status
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS").field("Header", &self.Header).field("Status", &self.Status).finish()
    }
}
impl ::core::default::Default for DOT11_OFFLOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_OFFLOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_OFFLOAD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_OI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_OI {
    fn eq(&self, other: &Self) -> bool {
        self.OILength == other.OILength && self.OI == other.OI
    }
}
impl ::core::cmp::Eq for DOT11_OI {}
impl ::core::fmt::Debug for DOT11_OI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OI").field("OILength", &self.OILength).field("OI", &self.OI).finish()
    }
}
impl ::core::default::Default for DOT11_OPERATION_MODE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_OPERATION_MODE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uMajorVersion == other.uMajorVersion && self.uMinorVersion == other.uMinorVersion && self.uNumOfTXBuffers == other.uNumOfTXBuffers && self.uNumOfRXBuffers == other.uNumOfRXBuffers && self.uOpModeCapability == other.uOpModeCapability
    }
}
impl ::core::cmp::Eq for DOT11_OPERATION_MODE_CAPABILITY {}
impl ::core::fmt::Debug for DOT11_OPERATION_MODE_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OPERATION_MODE_CAPABILITY").field("uReserved", &self.uReserved).field("uMajorVersion", &self.uMajorVersion).field("uMinorVersion", &self.uMinorVersion).field("uNumOfTXBuffers", &self.uNumOfTXBuffers).field("uNumOfRXBuffers", &self.uNumOfRXBuffers).field("uOpModeCapability", &self.uOpModeCapability).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_OPTIONAL_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.bDot11PCF == other.bDot11PCF && self.bDot11PCFMPDUTransferToPC == other.bDot11PCFMPDUTransferToPC && self.bStrictlyOrderedServiceClass == other.bStrictlyOrderedServiceClass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_OPTIONAL_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OPTIONAL_CAPABILITY").field("uReserved", &self.uReserved).field("bDot11PCF", &self.bDot11PCF).field("bDot11PCFMPDUTransferToPC", &self.bDot11PCFMPDUTransferToPC).field("bStrictlyOrderedServiceClass", &self.bStrictlyOrderedServiceClass).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PEER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PEER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MacAddress == other.MacAddress && self.usCapabilityInformation == other.usCapabilityInformation && self.AuthAlgo == other.AuthAlgo && self.UnicastCipherAlgo == other.UnicastCipherAlgo && self.MulticastCipherAlgo == other.MulticastCipherAlgo && self.bWpsEnabled == other.bWpsEnabled && self.usListenInterval == other.usListenInterval && self.ucSupportedRates == other.ucSupportedRates && self.usAssociationID == other.usAssociationID && self.AssociationState == other.AssociationState && self.PowerMode == other.PowerMode && self.liAssociationUpTime == other.liAssociationUpTime && self.Statistics == other.Statistics
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PEER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_PEER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PEER_INFO")
            .field("MacAddress", &self.MacAddress)
            .field("usCapabilityInformation", &self.usCapabilityInformation)
            .field("AuthAlgo", &self.AuthAlgo)
            .field("UnicastCipherAlgo", &self.UnicastCipherAlgo)
            .field("MulticastCipherAlgo", &self.MulticastCipherAlgo)
            .field("bWpsEnabled", &self.bWpsEnabled)
            .field("usListenInterval", &self.usListenInterval)
            .field("ucSupportedRates", &self.ucSupportedRates)
            .field("usAssociationID", &self.usAssociationID)
            .field("AssociationState", &self.AssociationState)
            .field("PowerMode", &self.PowerMode)
            .field("liAssociationUpTime", &self.liAssociationUpTime)
            .field("Statistics", &self.Statistics)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PEER_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PEER_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.PeerInfo == other.PeerInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PEER_INFO_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_PEER_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PEER_INFO_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("PeerInfo", &self.PeerInfo).finish()
    }
}
impl ::core::default::Default for DOT11_PEER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_PEER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullDecryptSuccessCount == other.ullDecryptSuccessCount && self.ullDecryptFailureCount == other.ullDecryptFailureCount && self.ullTxPacketSuccessCount == other.ullTxPacketSuccessCount && self.ullTxPacketFailureCount == other.ullTxPacketFailureCount && self.ullRxPacketSuccessCount == other.ullRxPacketSuccessCount && self.ullRxPacketFailureCount == other.ullRxPacketFailureCount
    }
}
impl ::core::cmp::Eq for DOT11_PEER_STATISTICS {}
impl ::core::fmt::Debug for DOT11_PEER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PEER_STATISTICS").field("ullDecryptSuccessCount", &self.ullDecryptSuccessCount).field("ullDecryptFailureCount", &self.ullDecryptFailureCount).field("ullTxPacketSuccessCount", &self.ullTxPacketSuccessCount).field("ullTxPacketFailureCount", &self.ullTxPacketFailureCount).field("ullRxPacketSuccessCount", &self.ullRxPacketSuccessCount).field("ullRxPacketFailureCount", &self.ullRxPacketFailureCount).finish()
    }
}
impl ::core::default::Default for DOT11_PER_MSDU_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_PER_MSDU_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.uTransmittedFragmentCount == other.uTransmittedFragmentCount && self.uRetryCount == other.uRetryCount && self.uRTSSuccessCount == other.uRTSSuccessCount && self.uRTSFailureCount == other.uRTSFailureCount && self.uACKFailureCount == other.uACKFailureCount
    }
}
impl ::core::cmp::Eq for DOT11_PER_MSDU_COUNTERS {}
impl ::core::fmt::Debug for DOT11_PER_MSDU_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PER_MSDU_COUNTERS").field("uTransmittedFragmentCount", &self.uTransmittedFragmentCount).field("uRetryCount", &self.uRetryCount).field("uRTSSuccessCount", &self.uRTSSuccessCount).field("uRTSFailureCount", &self.uRTSFailureCount).field("uACKFailureCount", &self.uACKFailureCount).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOT11_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_PHY_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount
            && self.ullMulticastTransmittedFrameCount == other.ullMulticastTransmittedFrameCount
            && self.ullFailedCount == other.ullFailedCount
            && self.ullRetryCount == other.ullRetryCount
            && self.ullMultipleRetryCount == other.ullMultipleRetryCount
            && self.ullMaxTXLifetimeExceededCount == other.ullMaxTXLifetimeExceededCount
            && self.ullTransmittedFragmentCount == other.ullTransmittedFragmentCount
            && self.ullRTSSuccessCount == other.ullRTSSuccessCount
            && self.ullRTSFailureCount == other.ullRTSFailureCount
            && self.ullACKFailureCount == other.ullACKFailureCount
            && self.ullReceivedFrameCount == other.ullReceivedFrameCount
            && self.ullMulticastReceivedFrameCount == other.ullMulticastReceivedFrameCount
            && self.ullPromiscuousReceivedFrameCount == other.ullPromiscuousReceivedFrameCount
            && self.ullMaxRXLifetimeExceededCount == other.ullMaxRXLifetimeExceededCount
            && self.ullFrameDuplicateCount == other.ullFrameDuplicateCount
            && self.ullReceivedFragmentCount == other.ullReceivedFragmentCount
            && self.ullPromiscuousReceivedFragmentCount == other.ullPromiscuousReceivedFragmentCount
            && self.ullFCSErrorCount == other.ullFCSErrorCount
    }
}
impl ::core::cmp::Eq for DOT11_PHY_FRAME_STATISTICS {}
impl ::core::fmt::Debug for DOT11_PHY_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullMulticastTransmittedFrameCount", &self.ullMulticastTransmittedFrameCount)
            .field("ullFailedCount", &self.ullFailedCount)
            .field("ullRetryCount", &self.ullRetryCount)
            .field("ullMultipleRetryCount", &self.ullMultipleRetryCount)
            .field("ullMaxTXLifetimeExceededCount", &self.ullMaxTXLifetimeExceededCount)
            .field("ullTransmittedFragmentCount", &self.ullTransmittedFragmentCount)
            .field("ullRTSSuccessCount", &self.ullRTSSuccessCount)
            .field("ullRTSFailureCount", &self.ullRTSFailureCount)
            .field("ullACKFailureCount", &self.ullACKFailureCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullMulticastReceivedFrameCount", &self.ullMulticastReceivedFrameCount)
            .field("ullPromiscuousReceivedFrameCount", &self.ullPromiscuousReceivedFrameCount)
            .field("ullMaxRXLifetimeExceededCount", &self.ullMaxRXLifetimeExceededCount)
            .field("ullFrameDuplicateCount", &self.ullFrameDuplicateCount)
            .field("ullReceivedFragmentCount", &self.ullReceivedFragmentCount)
            .field("ullPromiscuousReceivedFragmentCount", &self.ullPromiscuousReceivedFragmentCount)
            .field("ullFCSErrorCount", &self.ullFCSErrorCount)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_ID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_ID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11PhyId == other.dot11PhyId
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_ID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PHY_ID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_ID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11PhyId", &self.dot11PhyId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PHY_STATE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uPhyId == other.uPhyId && self.bHardwarePhyState == other.bHardwarePhyState && self.bSoftwarePhyState == other.bSoftwarePhyState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PHY_STATE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_PHY_STATE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_STATE_PARAMETERS").field("Header", &self.Header).field("uPhyId", &self.uPhyId).field("bHardwarePhyState", &self.bHardwarePhyState).field("bSoftwarePhyState", &self.bSoftwarePhyState).finish()
    }
}
impl ::core::default::Default for DOT11_PHY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_PHY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_PHY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PHY_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PHY_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PhyType == other.dot11PhyType && self.bUseParameters == other.bUseParameters && self.uProbeDelay == other.uProbeDelay && self.uMinChannelTime == other.uMinChannelTime && self.uMaxChannelTime == other.uMaxChannelTime && self.ChDescriptionType == other.ChDescriptionType && self.uChannelListSize == other.uChannelListSize && self.ucChannelListBuffer == other.ucChannelListBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PHY_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_PHY_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_TYPE_INFO").field("dot11PhyType", &self.dot11PhyType).field("bUseParameters", &self.bUseParameters).field("uProbeDelay", &self.uProbeDelay).field("uMinChannelTime", &self.uMinChannelTime).field("uMaxChannelTime", &self.uMaxChannelTime).field("ChDescriptionType", &self.ChDescriptionType).field("uChannelListSize", &self.uChannelListSize).field("ucChannelListBuffer", &self.ucChannelListBuffer).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11PhyType == other.dot11PhyType
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PHY_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_TYPE_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11PhyType", &self.dot11PhyType).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uCandidateListSize == other.uCandidateListSize && self.uCandidateListOffset == other.uCandidateListOffset
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PMKID_CANDIDATE_LIST_PARAMETERS").field("Header", &self.Header).field("uCandidateListSize", &self.uCandidateListSize).field("uCandidateListOffset", &self.uCandidateListOffset).finish()
    }
}
impl ::core::default::Default for DOT11_PMKID_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_PMKID_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.PMKID == other.PMKID && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DOT11_PMKID_ENTRY {}
impl ::core::fmt::Debug for DOT11_PMKID_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PMKID_ENTRY").field("BSSID", &self.BSSID).field("PMKID", &self.PMKID).field("uFlags", &self.uFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PMKID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PMKID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.PMKIDs == other.PMKIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PMKID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PMKID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PMKID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("PMKIDs", &self.PMKIDs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.uSessionId == other.uSessionId && self.bPortControlled == other.bPortControlled && self.bPortAuthorized == other.bPortAuthorized
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PORT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_PORT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PORT_STATE").field("PeerMacAddress", &self.PeerMacAddress).field("uSessionId", &self.uSessionId).field("bPortControlled", &self.bPortControlled).field("bPortAuthorized", &self.bPortAuthorized).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PORT_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PORT_STATE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMac == other.PeerMac && self.bOpen == other.bOpen
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PORT_STATE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_PORT_STATE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PORT_STATE_NOTIFICATION").field("Header", &self.Header).field("PeerMac", &self.PeerMac).field("bOpen", &self.bOpen).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bEnabled == other.bEnabled
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO").field("Header", &self.Header).field("bEnabled", &self.bEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_POWER_MGMT_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PowerMode == other.dot11PowerMode && self.uPowerSaveLevel == other.uPowerSaveLevel && self.usListenInterval == other.usListenInterval && self.usAID == other.usAID && self.bReceiveDTIMs == other.bReceiveDTIMs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_POWER_MGMT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_POWER_MGMT_MODE").field("dot11PowerMode", &self.dot11PowerMode).field("uPowerSaveLevel", &self.uPowerSaveLevel).field("usListenInterval", &self.usListenInterval).field("usAID", &self.usAID).field("bReceiveDTIMs", &self.bReceiveDTIMs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PowerSaveMode == other.PowerSaveMode && self.uPowerSaveLevel == other.uPowerSaveLevel && self.Reason == other.Reason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_MODE_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_POWER_MGMT_MODE_STATUS_INFO").field("Header", &self.Header).field("PowerSaveMode", &self.PowerSaveMode).field("uPowerSaveLevel", &self.uPowerSaveLevel).field("Reason", &self.Reason).finish()
    }
}
impl ::core::default::Default for DOT11_POWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_POWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_POWER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_POWER_MODE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_POWER_MODE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_POWER_MODE_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_PRIVACY_EXEMPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_PRIVACY_EXEMPTION {
    fn eq(&self, other: &Self) -> bool {
        self.usEtherType == other.usEtherType && self.usExemptionActionType == other.usExemptionActionType && self.usExemptionPacketType == other.usExemptionPacketType
    }
}
impl ::core::cmp::Eq for DOT11_PRIVACY_EXEMPTION {}
impl ::core::fmt::Debug for DOT11_PRIVACY_EXEMPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PRIVACY_EXEMPTION").field("usEtherType", &self.usEtherType).field("usExemptionActionType", &self.usExemptionActionType).field("usExemptionPacketType", &self.usExemptionPacketType).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PRIVACY_EXEMPTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PRIVACY_EXEMPTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.PrivacyExemptionEntries == other.PrivacyExemptionEntries
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PRIVACY_EXEMPTION_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PRIVACY_EXEMPTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PRIVACY_EXEMPTION_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("PrivacyExemptionEntries", &self.PrivacyExemptionEntries).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.ReceiverAddress == other.ReceiverAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("ReceiverAddress", &self.ReceiverAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_QOS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_QOS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ucEnabledQoSProtocolFlags == other.ucEnabledQoSProtocolFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_QOS_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_QOS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_QOS_PARAMS").field("Header", &self.Header).field("ucEnabledQoSProtocolFlags", &self.ucEnabledQoSProtocolFlags).finish()
    }
}
impl ::core::default::Default for DOT11_QOS_TX_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_QOS_TX_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.uNominalMSDUSize == other.uNominalMSDUSize && self.uMinPHYRate == other.uMinPHYRate && self.uDuration == other.uDuration
    }
}
impl ::core::cmp::Eq for DOT11_QOS_TX_DURATION {}
impl ::core::fmt::Debug for DOT11_QOS_TX_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_QOS_TX_DURATION").field("uNominalMSDUSize", &self.uNominalMSDUSize).field("uMinPHYRate", &self.uMinPHYRate).field("uDuration", &self.uDuration).finish()
    }
}
impl ::core::default::Default for DOT11_QOS_TX_MEDIUM_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_QOS_TX_MEDIUM_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PeerAddress == other.dot11PeerAddress && self.ucQoSPriority == other.ucQoSPriority && self.uMediumTimeAdmited == other.uMediumTimeAdmited
    }
}
impl ::core::cmp::Eq for DOT11_QOS_TX_MEDIUM_TIME {}
impl ::core::fmt::Debug for DOT11_QOS_TX_MEDIUM_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_QOS_TX_MEDIUM_TIME").field("dot11PeerAddress", &self.dot11PeerAddress).field("ucQoSPriority", &self.ucQoSPriority).field("uMediumTimeAdmited", &self.uMediumTimeAdmited).finish()
    }
}
impl ::core::default::Default for DOT11_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_RADIO_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_RATE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_RATE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.uRateSetLength == other.uRateSetLength && self.ucRateSet == other.ucRateSet
    }
}
impl ::core::cmp::Eq for DOT11_RATE_SET {}
impl ::core::fmt::Debug for DOT11_RATE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RATE_SET").field("uRateSetLength", &self.uRateSetLength).field("ucRateSet", &self.ucRateSet).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.ResponseContext == other.ResponseContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("ResponseContext", &self.ResponseContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RECV_EXTENSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RECV_EXTENSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion
            && self.pvReserved == other.pvReserved
            && self.dot11PhyType == other.dot11PhyType
            && self.uChCenterFrequency == other.uChCenterFrequency
            && self.lRSSI == other.lRSSI
            && self.lRSSIMin == other.lRSSIMin
            && self.lRSSIMax == other.lRSSIMax
            && self.uRSSI == other.uRSSI
            && self.ucPriority == other.ucPriority
            && self.ucDataRate == other.ucDataRate
            && self.ucPeerMacAddress == other.ucPeerMacAddress
            && self.dwExtendedStatus == other.dwExtendedStatus
            && self.hWEPOffloadContext == other.hWEPOffloadContext
            && self.hAuthOffloadContext == other.hAuthOffloadContext
            && self.usWEPAppliedMask == other.usWEPAppliedMask
            && self.usWPAMSDUPriority == other.usWPAMSDUPriority
            && self.dot11LowestIV48Counter == other.dot11LowestIV48Counter
            && self.usDot11LeftRWBitMap == other.usDot11LeftRWBitMap
            && self.dot11HighestIV48Counter == other.dot11HighestIV48Counter
            && self.usDot11RightRWBitMap == other.usDot11RightRWBitMap
            && self.usNumberOfMPDUsReceived == other.usNumberOfMPDUsReceived
            && self.usNumberOfFragments == other.usNumberOfFragments
            && self.pNdisPackets == other.pNdisPackets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RECV_EXTENSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_RECV_EXTENSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECV_EXTENSION_INFO")
            .field("uVersion", &self.uVersion)
            .field("pvReserved", &self.pvReserved)
            .field("dot11PhyType", &self.dot11PhyType)
            .field("uChCenterFrequency", &self.uChCenterFrequency)
            .field("lRSSI", &self.lRSSI)
            .field("lRSSIMin", &self.lRSSIMin)
            .field("lRSSIMax", &self.lRSSIMax)
            .field("uRSSI", &self.uRSSI)
            .field("ucPriority", &self.ucPriority)
            .field("ucDataRate", &self.ucDataRate)
            .field("ucPeerMacAddress", &self.ucPeerMacAddress)
            .field("dwExtendedStatus", &self.dwExtendedStatus)
            .field("hWEPOffloadContext", &self.hWEPOffloadContext)
            .field("hAuthOffloadContext", &self.hAuthOffloadContext)
            .field("usWEPAppliedMask", &self.usWEPAppliedMask)
            .field("usWPAMSDUPriority", &self.usWPAMSDUPriority)
            .field("dot11LowestIV48Counter", &self.dot11LowestIV48Counter)
            .field("usDot11LeftRWBitMap", &self.usDot11LeftRWBitMap)
            .field("dot11HighestIV48Counter", &self.dot11HighestIV48Counter)
            .field("usDot11RightRWBitMap", &self.usDot11RightRWBitMap)
            .field("usNumberOfMPDUsReceived", &self.usNumberOfMPDUsReceived)
            .field("usNumberOfFragments", &self.usNumberOfFragments)
            .field("pNdisPackets", &self.pNdisPackets)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RECV_EXTENSION_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RECV_EXTENSION_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion
            && self.pvReserved == other.pvReserved
            && self.dot11PhyType == other.dot11PhyType
            && self.uChCenterFrequency == other.uChCenterFrequency
            && self.lRSSI == other.lRSSI
            && self.uRSSI == other.uRSSI
            && self.ucPriority == other.ucPriority
            && self.ucDataRate == other.ucDataRate
            && self.ucPeerMacAddress == other.ucPeerMacAddress
            && self.dwExtendedStatus == other.dwExtendedStatus
            && self.hWEPOffloadContext == other.hWEPOffloadContext
            && self.hAuthOffloadContext == other.hAuthOffloadContext
            && self.usWEPAppliedMask == other.usWEPAppliedMask
            && self.usWPAMSDUPriority == other.usWPAMSDUPriority
            && self.dot11LowestIV48Counter == other.dot11LowestIV48Counter
            && self.usDot11LeftRWBitMap == other.usDot11LeftRWBitMap
            && self.dot11HighestIV48Counter == other.dot11HighestIV48Counter
            && self.usDot11RightRWBitMap == other.usDot11RightRWBitMap
            && self.usNumberOfMPDUsReceived == other.usNumberOfMPDUsReceived
            && self.usNumberOfFragments == other.usNumberOfFragments
            && self.pNdisPackets == other.pNdisPackets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RECV_EXTENSION_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_RECV_EXTENSION_INFO_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECV_EXTENSION_INFO_V2")
            .field("uVersion", &self.uVersion)
            .field("pvReserved", &self.pvReserved)
            .field("dot11PhyType", &self.dot11PhyType)
            .field("uChCenterFrequency", &self.uChCenterFrequency)
            .field("lRSSI", &self.lRSSI)
            .field("uRSSI", &self.uRSSI)
            .field("ucPriority", &self.ucPriority)
            .field("ucDataRate", &self.ucDataRate)
            .field("ucPeerMacAddress", &self.ucPeerMacAddress)
            .field("dwExtendedStatus", &self.dwExtendedStatus)
            .field("hWEPOffloadContext", &self.hWEPOffloadContext)
            .field("hAuthOffloadContext", &self.hAuthOffloadContext)
            .field("usWEPAppliedMask", &self.usWEPAppliedMask)
            .field("usWPAMSDUPriority", &self.usWPAMSDUPriority)
            .field("dot11LowestIV48Counter", &self.dot11LowestIV48Counter)
            .field("usDot11LeftRWBitMap", &self.usDot11LeftRWBitMap)
            .field("dot11HighestIV48Counter", &self.dot11HighestIV48Counter)
            .field("usDot11RightRWBitMap", &self.usDot11RightRWBitMap)
            .field("usNumberOfMPDUsReceived", &self.usNumberOfMPDUsReceived)
            .field("usNumberOfFragments", &self.usNumberOfFragments)
            .field("pNdisPackets", &self.pNdisPackets)
            .finish()
    }
}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_RECV_SENSITIVITY {
    fn eq(&self, other: &Self) -> bool {
        self.ucDataRate == other.ucDataRate && self.lRSSIMin == other.lRSSIMin && self.lRSSIMax == other.lRSSIMax
    }
}
impl ::core::cmp::Eq for DOT11_RECV_SENSITIVITY {}
impl ::core::fmt::Debug for DOT11_RECV_SENSITIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECV_SENSITIVITY").field("ucDataRate", &self.ucDataRate).field("lRSSIMin", &self.lRSSIMin).field("lRSSIMax", &self.lRSSIMax).finish()
    }
}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11RegDomainValue == other.dot11RegDomainValue
    }
}
impl ::core::cmp::Eq for DOT11_REG_DOMAINS_SUPPORT_VALUE {}
impl ::core::fmt::Debug for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_REG_DOMAINS_SUPPORT_VALUE").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11RegDomainValue", &self.dot11RegDomainValue).finish()
    }
}
impl ::core::default::Default for DOT11_REG_DOMAIN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_REG_DOMAIN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.uRegDomainsSupportIndex == other.uRegDomainsSupportIndex && self.uRegDomainsSupportValue == other.uRegDomainsSupportValue
    }
}
impl ::core::cmp::Eq for DOT11_REG_DOMAIN_VALUE {}
impl ::core::fmt::Debug for DOT11_REG_DOMAIN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_REG_DOMAIN_VALUE").field("uRegDomainsSupportIndex", &self.uRegDomainsSupportIndex).field("uRegDomainsSupportValue", &self.uRegDomainsSupportValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RESET_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RESET_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ResetType == other.dot11ResetType && self.dot11MacAddress == other.dot11MacAddress && self.bSetDefaultMIB == other.bSetDefaultMIB
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RESET_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_RESET_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RESET_REQUEST").field("dot11ResetType", &self.dot11ResetType).field("dot11MacAddress", &self.dot11MacAddress).field("bSetDefaultMIB", &self.bSetDefaultMIB).finish()
    }
}
impl ::core::default::Default for DOT11_RESET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_RESET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_RESET_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uStatus == other.uStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ROAMING_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ROAMING_COMPLETION_PARAMETERS").field("Header", &self.Header).field("uStatus", &self.uStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ROAMING_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ROAMING_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.AdhocBSSID == other.AdhocBSSID && self.AdhocSSID == other.AdhocSSID && self.uRoamingReason == other.uRoamingReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ROAMING_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ROAMING_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ROAMING_START_PARAMETERS").field("Header", &self.Header).field("AdhocBSSID", &self.AdhocBSSID).field("AdhocSSID", &self.AdhocSSID).field("uRoamingReason", &self.uRoamingReason).finish()
    }
}
impl ::core::default::Default for DOT11_RSSI_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_RSSI_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PhyType == other.dot11PhyType && self.uRSSIMin == other.uRSSIMin && self.uRSSIMax == other.uRSSIMax
    }
}
impl ::core::cmp::Eq for DOT11_RSSI_RANGE {}
impl ::core::fmt::Debug for DOT11_RSSI_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RSSI_RANGE").field("dot11PhyType", &self.dot11PhyType).field("uRSSIMin", &self.uRSSIMin).field("uRSSIMax", &self.uRSSIMax).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SCAN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SCAN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dot11BSSType == other.dot11BSSType && self.dot11BSSID == other.dot11BSSID && self.dot11SSID == other.dot11SSID && self.dot11ScanType == other.dot11ScanType && self.bRestrictedScan == other.bRestrictedScan && self.bUseRequestIE == other.bUseRequestIE && self.uRequestIDsOffset == other.uRequestIDsOffset && self.uNumOfRequestIDs == other.uNumOfRequestIDs && self.uPhyTypesOffset == other.uPhyTypesOffset && self.uNumOfPhyTypes == other.uNumOfPhyTypes && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength && self.ucBuffer == other.ucBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SCAN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SCAN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SCAN_REQUEST")
            .field("dot11BSSType", &self.dot11BSSType)
            .field("dot11BSSID", &self.dot11BSSID)
            .field("dot11SSID", &self.dot11SSID)
            .field("dot11ScanType", &self.dot11ScanType)
            .field("bRestrictedScan", &self.bRestrictedScan)
            .field("bUseRequestIE", &self.bUseRequestIE)
            .field("uRequestIDsOffset", &self.uRequestIDsOffset)
            .field("uNumOfRequestIDs", &self.uNumOfRequestIDs)
            .field("uPhyTypesOffset", &self.uPhyTypesOffset)
            .field("uNumOfPhyTypes", &self.uNumOfPhyTypes)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .field("ucBuffer", &self.ucBuffer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SCAN_REQUEST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SCAN_REQUEST_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dot11BSSType == other.dot11BSSType && self.dot11BSSID == other.dot11BSSID && self.dot11ScanType == other.dot11ScanType && self.bRestrictedScan == other.bRestrictedScan && self.udot11SSIDsOffset == other.udot11SSIDsOffset && self.uNumOfdot11SSIDs == other.uNumOfdot11SSIDs && self.bUseRequestIE == other.bUseRequestIE && self.uRequestIDsOffset == other.uRequestIDsOffset && self.uNumOfRequestIDs == other.uNumOfRequestIDs && self.uPhyTypeInfosOffset == other.uPhyTypeInfosOffset && self.uNumOfPhyTypeInfos == other.uNumOfPhyTypeInfos && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength && self.ucBuffer == other.ucBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SCAN_REQUEST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SCAN_REQUEST_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SCAN_REQUEST_V2")
            .field("dot11BSSType", &self.dot11BSSType)
            .field("dot11BSSID", &self.dot11BSSID)
            .field("dot11ScanType", &self.dot11ScanType)
            .field("bRestrictedScan", &self.bRestrictedScan)
            .field("udot11SSIDsOffset", &self.udot11SSIDsOffset)
            .field("uNumOfdot11SSIDs", &self.uNumOfdot11SSIDs)
            .field("bUseRequestIE", &self.bUseRequestIE)
            .field("uRequestIDsOffset", &self.uRequestIDsOffset)
            .field("uNumOfRequestIDs", &self.uNumOfRequestIDs)
            .field("uPhyTypeInfosOffset", &self.uPhyTypeInfosOffset)
            .field("uNumOfPhyTypeInfos", &self.uNumOfPhyTypeInfos)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .field("ucBuffer", &self.ucBuffer)
            .finish()
    }
}
impl ::core::default::Default for DOT11_SCAN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_SCAN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_SCAN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_SECURITY_PACKET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.ResponseContext == other.ResponseContext && self.uSendTimeout == other.uSendTimeout && self.Status == other.Status && self.GroupCapability == other.GroupCapability && self.GroupID == other.GroupID && self.bUseGroupID == other.bUseGroupID && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("ResponseContext", &self.ResponseContext)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("Status", &self.Status)
            .field("GroupCapability", &self.GroupCapability)
            .field("GroupID", &self.GroupID)
            .field("bUseGroupID", &self.bUseGroupID)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.uSendTimeout == other.uSendTimeout && self.GroupOwnerIntent == other.GroupOwnerIntent && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.IntendedInterfaceAddress == other.IntendedInterfaceAddress && self.GroupCapability == other.GroupCapability && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("GroupOwnerIntent", &self.GroupOwnerIntent)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("IntendedInterfaceAddress", &self.IntendedInterfaceAddress)
            .field("GroupCapability", &self.GroupCapability)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uSendTimeout == other.uSendTimeout && self.Status == other.Status && self.GroupOwnerIntent == other.GroupOwnerIntent && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.IntendedInterfaceAddress == other.IntendedInterfaceAddress && self.GroupCapability == other.GroupCapability && self.GroupID == other.GroupID && self.bUseGroupID == other.bUseGroupID && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("RequestContext", &self.RequestContext)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("Status", &self.Status)
            .field("GroupOwnerIntent", &self.GroupOwnerIntent)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("IntendedInterfaceAddress", &self.IntendedInterfaceAddress)
            .field("GroupCapability", &self.GroupCapability)
            .field("GroupID", &self.GroupID)
            .field("bUseGroupID", &self.bUseGroupID)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DialogToken == other.DialogToken && self.PeerDeviceAddress == other.PeerDeviceAddress && self.uSendTimeout == other.uSendTimeout && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.InvitationFlags == other.InvitationFlags && self.GroupBSSID == other.GroupBSSID && self.bUseGroupBSSID == other.bUseGroupBSSID && self.OperatingChannel == other.OperatingChannel && self.bUseSpecifiedOperatingChannel == other.bUseSpecifiedOperatingChannel && self.GroupID == other.GroupID && self.bLocalGO == other.bLocalGO && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_INVITATION_REQUEST_PARAMETERS")
            .field("Header", &self.Header)
            .field("DialogToken", &self.DialogToken)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("InvitationFlags", &self.InvitationFlags)
            .field("GroupBSSID", &self.GroupBSSID)
            .field("bUseGroupBSSID", &self.bUseGroupBSSID)
            .field("OperatingChannel", &self.OperatingChannel)
            .field("bUseSpecifiedOperatingChannel", &self.bUseSpecifiedOperatingChannel)
            .field("GroupID", &self.GroupID)
            .field("bLocalGO", &self.bLocalGO)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uSendTimeout == other.uSendTimeout && self.Status == other.Status && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.GroupBSSID == other.GroupBSSID && self.bUseGroupBSSID == other.bUseGroupBSSID && self.OperatingChannel == other.OperatingChannel && self.bUseSpecifiedOperatingChannel == other.bUseSpecifiedOperatingChannel && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_INVITATION_RESPONSE_PARAMETERS")
            .field("Header", &self.Header)
            .field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("RequestContext", &self.RequestContext)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("Status", &self.Status)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("GroupBSSID", &self.GroupBSSID)
            .field("bUseGroupBSSID", &self.bUseGroupBSSID)
            .field("OperatingChannel", &self.OperatingChannel)
            .field("bUseSpecifiedOperatingChannel", &self.bUseSpecifiedOperatingChannel)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DialogToken == other.DialogToken && self.PeerDeviceAddress == other.PeerDeviceAddress && self.uSendTimeout == other.uSendTimeout && self.GroupCapability == other.GroupCapability && self.GroupID == other.GroupID && self.bUseGroupID == other.bUseGroupID && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS").field("Header", &self.Header).field("DialogToken", &self.DialogToken).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("uSendTimeout", &self.uSendTimeout).field("GroupCapability", &self.GroupCapability).field("GroupID", &self.GroupID).field("bUseGroupID", &self.bUseGroupID).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uSendTimeout == other.uSendTimeout && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS").field("Header", &self.Header).field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uSendTimeout", &self.uSendTimeout).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
impl ::core::default::Default for DOT11_SSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SSID {
    fn eq(&self, other: &Self) -> bool {
        self.uSSIDLength == other.uSSIDLength && self.ucSSID == other.ucSSID
    }
}
impl ::core::cmp::Eq for DOT11_SSID {}
impl ::core::fmt::Debug for DOT11_SSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SSID").field("uSSIDLength", &self.uSSIDLength).field("ucSSID", &self.ucSSID).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.SSIDs == other.SSIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_SSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SSID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("SSIDs", &self.SSIDs).finish()
    }
}
impl ::core::default::Default for DOT11_START_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_START_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.uStartFailureTimeout == other.uStartFailureTimeout && self.OperationalRateSet == other.OperationalRateSet && self.uChCenterFrequency == other.uChCenterFrequency && self.dot11BSSDescription == other.dot11BSSDescription
    }
}
impl ::core::cmp::Eq for DOT11_START_REQUEST {}
impl ::core::fmt::Debug for DOT11_START_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_START_REQUEST").field("uStartFailureTimeout", &self.uStartFailureTimeout).field("OperationalRateSet", &self.OperationalRateSet).field("uChCenterFrequency", &self.uChCenterFrequency).field("dot11BSSDescription", &self.dot11BSSDescription).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ullFourWayHandshakeFailures == other.ullFourWayHandshakeFailures && self.ullTKIPCounterMeasuresInvoked == other.ullTKIPCounterMeasuresInvoked && self.ullReserved == other.ullReserved && self.MacUcastCounters == other.MacUcastCounters && self.MacMcastCounters == other.MacMcastCounters && self.PhyCounters == other.PhyCounters
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_STATISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_STATISTICS").field("Header", &self.Header).field("ullFourWayHandshakeFailures", &self.ullFourWayHandshakeFailures).field("ullTKIPCounterMeasuresInvoked", &self.ullTKIPCounterMeasuresInvoked).field("ullReserved", &self.ullReserved).field("MacUcastCounters", &self.MacUcastCounters).field("MacMcastCounters", &self.MacMcastCounters).field("PhyCounters", &self.PhyCounters).finish()
    }
}
impl ::core::default::Default for DOT11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_STATUS_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.uStatusType == other.uStatusType && self.ndisStatus == other.ndisStatus
    }
}
impl ::core::cmp::Eq for DOT11_STATUS_INDICATION {}
impl ::core::fmt::Debug for DOT11_STATUS_INDICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_STATUS_INDICATION").field("uStatusType", &self.uStatusType).field("ndisStatus", &self.ndisStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_STOP_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_STOP_AP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ulReason == other.ulReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_STOP_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_STOP_AP_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_STOP_AP_PARAMETERS").field("Header", &self.Header).field("ulReason", &self.ulReason).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SUPPORTED_ANTENNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_ANTENNA {
    fn eq(&self, other: &Self) -> bool {
        self.uAntennaListIndex == other.uAntennaListIndex && self.bSupportedAntenna == other.bSupportedAntenna
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SUPPORTED_ANTENNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SUPPORTED_ANTENNA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_ANTENNA").field("uAntennaListIndex", &self.uAntennaListIndex).field("bSupportedAntenna", &self.bSupportedAntenna).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SUPPORTED_ANTENNA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_ANTENNA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11SupportedAntenna == other.dot11SupportedAntenna
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SUPPORTED_ANTENNA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SUPPORTED_ANTENNA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_ANTENNA_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11SupportedAntenna", &self.dot11SupportedAntenna).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.ucSupportedTxDataRatesValue == other.ucSupportedTxDataRatesValue && self.ucSupportedRxDataRatesValue == other.ucSupportedRxDataRatesValue
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DATA_RATES_VALUE {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DATA_RATES_VALUE").field("ucSupportedTxDataRatesValue", &self.ucSupportedTxDataRatesValue).field("ucSupportedRxDataRatesValue", &self.ucSupportedRxDataRatesValue).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.ucSupportedTxDataRatesValue == other.ucSupportedTxDataRatesValue && self.ucSupportedRxDataRatesValue == other.ucSupportedRxDataRatesValue
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DATA_RATES_VALUE_V2").field("ucSupportedTxDataRatesValue", &self.ucSupportedTxDataRatesValue).field("ucSupportedRxDataRatesValue", &self.ucSupportedRxDataRatesValue).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.uChannel == other.uChannel
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DSSS_CHANNEL {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DSSS_CHANNEL").field("uChannel", &self.uChannel).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11SupportedDSSSChannel == other.dot11SupportedDSSSChannel
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DSSS_CHANNEL_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11SupportedDSSSChannel", &self.dot11SupportedDSSSChannel).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn eq(&self, other: &Self) -> bool {
        self.uCenterFrequency == other.uCenterFrequency
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_OFDM_FREQUENCY {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_OFDM_FREQUENCY").field("uCenterFrequency", &self.uCenterFrequency).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11SupportedOFDMFrequency == other.dot11SupportedOFDMFrequency
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_OFDM_FREQUENCY_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11SupportedOFDMFrequency", &self.dot11SupportedOFDMFrequency).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_PHY_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_PHY_TYPES {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11PHYType == other.dot11PHYType
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_PHY_TYPES {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_PHY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_PHY_TYPES").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11PHYType", &self.dot11PHYType).finish()
    }
}
impl ::core::default::Default for DOT11_SUPPORTED_POWER_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_POWER_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfSupportedPowerLevels == other.uNumOfSupportedPowerLevels && self.uTxPowerLevelValues == other.uTxPowerLevelValues
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_POWER_LEVELS {}
impl ::core::fmt::Debug for DOT11_SUPPORTED_POWER_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_POWER_LEVELS").field("uNumOfSupportedPowerLevels", &self.uNumOfSupportedPowerLevels).field("uTxPowerLevelValues", &self.uTxPowerLevelValues).finish()
    }
}
impl ::core::default::Default for DOT11_TEMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_TEMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_TEMP_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bDefaultKeyFailure == other.bDefaultKeyFailure && self.uKeyIndex == other.uKeyIndex && self.PeerMac == other.PeerMac
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_TKIPMIC_FAILURE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_TKIPMIC_FAILURE_PARAMETERS").field("Header", &self.Header).field("bDefaultKeyFailure", &self.bDefaultKeyFailure).field("uKeyIndex", &self.uKeyIndex).field("PeerMac", &self.PeerMac).finish()
    }
}
impl ::core::default::Default for DOT11_UPDATE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_UPDATE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11UpdateIEOp == other.dot11UpdateIEOp && self.uBufferLength == other.uBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_UPDATE_IE {}
impl ::core::fmt::Debug for DOT11_UPDATE_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_UPDATE_IE").field("dot11UpdateIEOp", &self.dot11UpdateIEOp).field("uBufferLength", &self.uBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::core::default::Default for DOT11_UPDATE_IE_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_UPDATE_IE_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_UPDATE_IE_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_VENUEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_VENUEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.VenueGroup == other.VenueGroup && self.VenueType == other.VenueType
    }
}
impl ::core::cmp::Eq for DOT11_VENUEINFO {}
impl ::core::fmt::Debug for DOT11_VENUEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VENUEINFO").field("VenueGroup", &self.VenueGroup).field("VenueType", &self.VenueType).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.Combinations == other.Combinations
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_ATTRIBUTES {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_ATTRIBUTES").field("Header", &self.Header).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("Combinations", &self.Combinations).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumInfrastructure == other.uNumInfrastructure && self.uNumAdhoc == other.uNumAdhoc && self.uNumSoftAP == other.uNumSoftAP
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_COMBINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_COMBINATION").field("Header", &self.Header).field("uNumInfrastructure", &self.uNumInfrastructure).field("uNumAdhoc", &self.uNumAdhoc).field("uNumSoftAP", &self.uNumSoftAP).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumInfrastructure == other.uNumInfrastructure && self.uNumAdhoc == other.uNumAdhoc && self.uNumSoftAP == other.uNumSoftAP && self.uNumVirtualStation == other.uNumVirtualStation
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_COMBINATION_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_COMBINATION_V2").field("Header", &self.Header).field("uNumInfrastructure", &self.uNumInfrastructure).field("uNumAdhoc", &self.uNumAdhoc).field("uNumSoftAP", &self.uNumSoftAP).field("uNumVirtualStation", &self.uNumVirtualStation).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumInfrastructure == other.uNumInfrastructure && self.uNumAdhoc == other.uNumAdhoc && self.uNumSoftAP == other.uNumSoftAP && self.uNumVirtualStation == other.uNumVirtualStation && self.uNumWFDGroup == other.uNumWFDGroup
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION_V3 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_COMBINATION_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_COMBINATION_V3").field("Header", &self.Header).field("uNumInfrastructure", &self.uNumInfrastructure).field("uNumAdhoc", &self.uNumAdhoc).field("uNumSoftAP", &self.uNumSoftAP).field("uNumVirtualStation", &self.uNumVirtualStation).field("uNumWFDGroup", &self.uNumWFDGroup).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WEP_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.hOffloadContext == other.hOffloadContext && self.hOffload == other.hOffload && self.dot11OffloadType == other.dot11OffloadType && self.dwAlgorithm == other.dwAlgorithm && self.bRowIsOutbound == other.bRowIsOutbound && self.bUseDefault == other.bUseDefault && self.uFlags == other.uFlags && self.ucMacAddress == other.ucMacAddress && self.uNumOfRWsOnPeer == other.uNumOfRWsOnPeer && self.uNumOfRWsOnMe == other.uNumOfRWsOnMe && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_WEP_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WEP_OFFLOAD")
            .field("uReserved", &self.uReserved)
            .field("hOffloadContext", &self.hOffloadContext)
            .field("hOffload", &self.hOffload)
            .field("dot11OffloadType", &self.dot11OffloadType)
            .field("dwAlgorithm", &self.dwAlgorithm)
            .field("bRowIsOutbound", &self.bRowIsOutbound)
            .field("bUseDefault", &self.bUseDefault)
            .field("uFlags", &self.uFlags)
            .field("ucMacAddress", &self.ucMacAddress)
            .field("uNumOfRWsOnPeer", &self.uNumOfRWsOnPeer)
            .field("uNumOfRWsOnMe", &self.uNumOfRWsOnMe)
            .field("dot11IV48Counters", &self.dot11IV48Counters)
            .field("usDot11RWBitMaps", &self.usDot11RWBitMaps)
            .field("usKeyLength", &self.usKeyLength)
            .field("ucKey", &self.ucKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WEP_UPLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11OffloadType == other.dot11OffloadType && self.hOffload == other.hOffload && self.uNumOfRWsUsed == other.uNumOfRWsUsed && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_WEP_UPLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WEP_UPLOAD").field("uReserved", &self.uReserved).field("dot11OffloadType", &self.dot11OffloadType).field("hOffload", &self.hOffload).field("uNumOfRWsUsed", &self.uNumOfRWsUsed).field("dot11IV48Counters", &self.dot11IV48Counters).field("usDot11RWBitMaps", &self.usDot11RWBitMaps).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_ADDITIONAL_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uBeaconIEsOffset == other.uBeaconIEsOffset && self.uBeaconIEsLength == other.uBeaconIEsLength && self.uProbeResponseIEsOffset == other.uProbeResponseIEsOffset && self.uProbeResponseIEsLength == other.uProbeResponseIEsLength && self.uDefaultRequestIEsOffset == other.uDefaultRequestIEsOffset && self.uDefaultRequestIEsLength == other.uDefaultRequestIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_ADDITIONAL_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADDITIONAL_IE").field("Header", &self.Header).field("uBeaconIEsOffset", &self.uBeaconIEsOffset).field("uBeaconIEsLength", &self.uBeaconIEsLength).field("uProbeResponseIEsOffset", &self.uProbeResponseIEsOffset).field("uProbeResponseIEsLength", &self.uProbeResponseIEsLength).field("uDefaultRequestIEsOffset", &self.uDefaultRequestIEsOffset).field("uDefaultRequestIEsLength", &self.uDefaultRequestIEsLength).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.AdvertisementID == other.AdvertisementID && self.ConfigMethods == other.ConfigMethods && self.ServiceNameLength == other.ServiceNameLength && self.ServiceName == other.ServiceName
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {}
impl ::core::fmt::Debug for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR").field("AdvertisementID", &self.AdvertisementID).field("ConfigMethods", &self.ConfigMethods).field("ServiceNameLength", &self.ServiceNameLength).field("ServiceName", &self.ServiceName).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceCount == other.ServiceCount && self.AdvertisedService == other.AdvertisedService
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISED_SERVICE_LIST {}
impl ::core::fmt::Debug for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADVERTISED_SERVICE_LIST").field("ServiceCount", &self.ServiceCount).field("AdvertisedService", &self.AdvertisedService).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_ADVERTISEMENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISEMENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AdvertisementID == other.AdvertisementID && self.ServiceAddress == other.ServiceAddress
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISEMENT_ID {}
impl ::core::fmt::Debug for DOT11_WFD_ADVERTISEMENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADVERTISEMENT_ID").field("AdvertisementID", &self.AdvertisementID).field("ServiceAddress", &self.ServiceAddress).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.uNumConcurrentGORole == other.uNumConcurrentGORole
            && self.uNumConcurrentClientRole == other.uNumConcurrentClientRole
            && self.WPSVersionsSupported == other.WPSVersionsSupported
            && self.bServiceDiscoverySupported == other.bServiceDiscoverySupported
            && self.bClientDiscoverabilitySupported == other.bClientDiscoverabilitySupported
            && self.bInfrastructureManagementSupported == other.bInfrastructureManagementSupported
            && self.uMaxSecondaryDeviceTypeListSize == other.uMaxSecondaryDeviceTypeListSize
            && self.DeviceAddress == other.DeviceAddress
            && self.uInterfaceAddressListCount == other.uInterfaceAddressListCount
            && self.pInterfaceAddressList == other.pInterfaceAddressList
            && self.uNumSupportedCountryOrRegionStrings == other.uNumSupportedCountryOrRegionStrings
            && self.pSupportedCountryOrRegionStrings == other.pSupportedCountryOrRegionStrings
            && self.uDiscoveryFilterListSize == other.uDiscoveryFilterListSize
            && self.uGORoleClientTableSize == other.uGORoleClientTableSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ATTRIBUTES")
            .field("Header", &self.Header)
            .field("uNumConcurrentGORole", &self.uNumConcurrentGORole)
            .field("uNumConcurrentClientRole", &self.uNumConcurrentClientRole)
            .field("WPSVersionsSupported", &self.WPSVersionsSupported)
            .field("bServiceDiscoverySupported", &self.bServiceDiscoverySupported)
            .field("bClientDiscoverabilitySupported", &self.bClientDiscoverabilitySupported)
            .field("bInfrastructureManagementSupported", &self.bInfrastructureManagementSupported)
            .field("uMaxSecondaryDeviceTypeListSize", &self.uMaxSecondaryDeviceTypeListSize)
            .field("DeviceAddress", &self.DeviceAddress)
            .field("uInterfaceAddressListCount", &self.uInterfaceAddressListCount)
            .field("pInterfaceAddressList", &self.pInterfaceAddressList)
            .field("uNumSupportedCountryOrRegionStrings", &self.uNumSupportedCountryOrRegionStrings)
            .field("pSupportedCountryOrRegionStrings", &self.pSupportedCountryOrRegionStrings)
            .field("uDiscoveryFilterListSize", &self.uDiscoveryFilterListSize)
            .field("uGORoleClientTableSize", &self.uGORoleClientTableSize)
            .finish()
    }
}
impl ::core::default::Default for DOT11_WFD_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.CountryRegionString == other.CountryRegionString && self.OperatingClass == other.OperatingClass && self.ChannelNumber == other.ChannelNumber
    }
}
impl ::core::cmp::Eq for DOT11_WFD_CHANNEL {}
impl ::core::fmt::Debug for DOT11_WFD_CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_CHANNEL").field("CountryRegionString", &self.CountryRegionString).field("OperatingClass", &self.OperatingClass).field("ChannelNumber", &self.ChannelNumber).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn eq(&self, other: &Self) -> bool {
        self.GOTimeout == other.GOTimeout && self.ClientTimeout == other.ClientTimeout
    }
}
impl ::core::cmp::Eq for DOT11_WFD_CONFIGURATION_TIMEOUT {}
impl ::core::fmt::Debug for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_CONFIGURATION_TIMEOUT").field("GOTimeout", &self.GOTimeout).field("ClientTimeout", &self.ClientTimeout).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bServiceDiscoveryEnabled == other.bServiceDiscoveryEnabled && self.bClientDiscoverabilityEnabled == other.bClientDiscoverabilityEnabled && self.bConcurrentOperationSupported == other.bConcurrentOperationSupported && self.bInfrastructureManagementEnabled == other.bInfrastructureManagementEnabled && self.bDeviceLimitReached == other.bDeviceLimitReached && self.bInvitationProcedureEnabled == other.bInvitationProcedureEnabled && self.WPSVersionsEnabled == other.WPSVersionsEnabled
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_CAPABILITY_CONFIG")
            .field("Header", &self.Header)
            .field("bServiceDiscoveryEnabled", &self.bServiceDiscoveryEnabled)
            .field("bClientDiscoverabilityEnabled", &self.bClientDiscoverabilityEnabled)
            .field("bConcurrentOperationSupported", &self.bConcurrentOperationSupported)
            .field("bInfrastructureManagementEnabled", &self.bInfrastructureManagementEnabled)
            .field("bDeviceLimitReached", &self.bDeviceLimitReached)
            .field("bInvitationProcedureEnabled", &self.bInvitationProcedureEnabled)
            .field("WPSVersionsEnabled", &self.WPSVersionsEnabled)
            .finish()
    }
}
impl ::core::default::Default for DOT11_WFD_DEVICE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DeviceAddress == other.DeviceAddress && self.ConfigMethods == other.ConfigMethods && self.PrimaryDeviceType == other.PrimaryDeviceType && self.DeviceName == other.DeviceName
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_INFO").field("Header", &self.Header).field("DeviceAddress", &self.DeviceAddress).field("ConfigMethods", &self.ConfigMethods).field("PrimaryDeviceType", &self.PrimaryDeviceType).field("DeviceName", &self.DeviceName).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ChannelNumber == other.ChannelNumber
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_LISTEN_CHANNEL {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_LISTEN_CHANNEL").field("Header", &self.Header).field("ChannelNumber", &self.ChannelNumber).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.CategoryID == other.CategoryID && self.SubCategoryID == other.SubCategoryID && self.OUI == other.OUI
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_TYPE {}
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_TYPE").field("CategoryID", &self.CategoryID).field("SubCategoryID", &self.SubCategoryID).field("OUI", &self.OUI).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Status == other.Status && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.uListOffset == other.uListOffset && self.uListLength == other.uListLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS").field("Header", &self.Header).field("Status", &self.Status).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("uListOffset", &self.uListOffset).field("uListLength", &self.uListLength).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceID == other.DeviceID && self.ucBitmask == other.ucBitmask && self.GroupSSID == other.GroupSSID
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_DEVICE_FILTER {}
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DISCOVER_DEVICE_FILTER").field("DeviceID", &self.DeviceID).field("ucBitmask", &self.ucBitmask).field("GroupSSID", &self.GroupSSID).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_DISCOVER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DiscoverType == other.DiscoverType && self.ScanType == other.ScanType && self.uDiscoverTimeout == other.uDiscoverTimeout && self.uDeviceFilterListOffset == other.uDeviceFilterListOffset && self.uNumDeviceFilters == other.uNumDeviceFilters && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength && self.bForceScanLegacyNetworks == other.bForceScanLegacyNetworks
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DISCOVER_REQUEST").field("Header", &self.Header).field("DiscoverType", &self.DiscoverType).field("ScanType", &self.ScanType).field("uDiscoverTimeout", &self.uDiscoverTimeout).field("uDeviceFilterListOffset", &self.uDeviceFilterListOffset).field("uNumDeviceFilters", &self.uNumDeviceFilters).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).field("bForceScanLegacyNetworks", &self.bForceScanLegacyNetworks).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_DISCOVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WFD_DISCOVER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_GO_INTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_GO_INTENT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DOT11_WFD_GO_INTENT {}
impl ::core::fmt::Debug for DOT11_WFD_GO_INTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GO_INTENT").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceAddress == other.DeviceAddress && self.SSID == other.SSID
    }
}
impl ::core::cmp::Eq for DOT11_WFD_GROUP_ID {}
impl ::core::fmt::Debug for DOT11_WFD_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_ID").field("DeviceAddress", &self.DeviceAddress).field("SSID", &self.SSID).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.GOOperatingChannel == other.GOOperatingChannel && self.GOConfigTime == other.GOConfigTime && self.bInGroupFormation == other.bInGroupFormation && self.bWaitForWPSReady == other.bWaitForWPSReady
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_JOIN_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_JOIN_PARAMETERS").field("Header", &self.Header).field("GOOperatingChannel", &self.GOOperatingChannel).field("GOConfigTime", &self.GOConfigTime).field("bInGroupFormation", &self.bInGroupFormation).field("bWaitForWPSReady", &self.bWaitForWPSReady).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bPersistentGroupEnabled == other.bPersistentGroupEnabled && self.bIntraBSSDistributionSupported == other.bIntraBSSDistributionSupported && self.bCrossConnectionSupported == other.bCrossConnectionSupported && self.bPersistentReconnectSupported == other.bPersistentReconnectSupported && self.bGroupFormationEnabled == other.bGroupFormationEnabled && self.uMaximumGroupLimit == other.uMaximumGroupLimit
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG")
            .field("Header", &self.Header)
            .field("bPersistentGroupEnabled", &self.bPersistentGroupEnabled)
            .field("bIntraBSSDistributionSupported", &self.bIntraBSSDistributionSupported)
            .field("bCrossConnectionSupported", &self.bCrossConnectionSupported)
            .field("bPersistentReconnectSupported", &self.bPersistentReconnectSupported)
            .field("bGroupFormationEnabled", &self.bGroupFormationEnabled)
            .field("uMaximumGroupLimit", &self.uMaximumGroupLimit)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bPersistentGroupEnabled == other.bPersistentGroupEnabled && self.bIntraBSSDistributionSupported == other.bIntraBSSDistributionSupported && self.bCrossConnectionSupported == other.bCrossConnectionSupported && self.bPersistentReconnectSupported == other.bPersistentReconnectSupported && self.bGroupFormationEnabled == other.bGroupFormationEnabled && self.uMaximumGroupLimit == other.uMaximumGroupLimit && self.bEapolKeyIpAddressAllocationSupported == other.bEapolKeyIpAddressAllocationSupported
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2")
            .field("Header", &self.Header)
            .field("bPersistentGroupEnabled", &self.bPersistentGroupEnabled)
            .field("bIntraBSSDistributionSupported", &self.bIntraBSSDistributionSupported)
            .field("bCrossConnectionSupported", &self.bCrossConnectionSupported)
            .field("bPersistentReconnectSupported", &self.bPersistentReconnectSupported)
            .field("bGroupFormationEnabled", &self.bGroupFormationEnabled)
            .field("uMaximumGroupLimit", &self.uMaximumGroupLimit)
            .field("bEapolKeyIpAddressAllocationSupported", &self.bEapolKeyIpAddressAllocationSupported)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_GROUP_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.AdvertisedOperatingChannel == other.AdvertisedOperatingChannel
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_START_PARAMETERS").field("Header", &self.Header).field("AdvertisedOperatingChannel", &self.AdvertisedOperatingChannel).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_INVITATION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_INVITATION_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DOT11_WFD_INVITATION_FLAGS {}
impl ::core::fmt::Debug for DOT11_WFD_INVITATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_INVITATION_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_SCAN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_WFD_SCAN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WFD_SCAN_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.SecondaryDeviceTypes == other.SecondaryDeviceTypes
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("SecondaryDeviceTypes", &self.SecondaryDeviceTypes).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_SERVICE_HASH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_SERVICE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceHashCount == other.ServiceHashCount && self.ServiceHash == other.ServiceHash
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SERVICE_HASH_LIST {}
impl ::core::fmt::Debug for DOT11_WFD_SERVICE_HASH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SERVICE_HASH_LIST").field("ServiceHashCount", &self.ServiceHashCount).field("ServiceHash", &self.ServiceHash).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionID == other.SessionID && self.SessionAddress == other.SessionAddress
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SESSION_ID {}
impl ::core::fmt::Debug for DOT11_WFD_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SESSION_ID").field("SessionID", &self.SessionID).field("SessionAddress", &self.SessionAddress).finish()
    }
}
impl ::core::default::Default for DOT11_WFD_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WFD_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uSessionInfoLength == other.uSessionInfoLength && self.ucSessionInfo == other.ucSessionInfo
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SESSION_INFO {}
impl ::core::fmt::Debug for DOT11_WFD_SESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SESSION_INFO").field("uSessionInfoLength", &self.uSessionInfoLength).field("ucSessionInfo", &self.ucSessionInfo).finish()
    }
}
impl ::core::default::Default for DOT11_WME_AC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WME_AC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ucAccessCategoryIndex == other.ucAccessCategoryIndex && self.ucAIFSN == other.ucAIFSN && self.ucECWmin == other.ucECWmin && self.ucECWmax == other.ucECWmax && self.usTXOPLimit == other.usTXOPLimit
    }
}
impl ::core::cmp::Eq for DOT11_WME_AC_PARAMETERS {}
impl ::core::fmt::Debug for DOT11_WME_AC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WME_AC_PARAMETERS").field("ucAccessCategoryIndex", &self.ucAccessCategoryIndex).field("ucAIFSN", &self.ucAIFSN).field("ucECWmin", &self.ucECWmin).field("ucECWmax", &self.ucECWmax).field("usTXOPLimit", &self.usTXOPLimit).finish()
    }
}
impl ::core::default::Default for DOT11_WME_AC_PARAMETERS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WME_AC_PARAMETERS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11WMEACParameters == other.dot11WMEACParameters
    }
}
impl ::core::cmp::Eq for DOT11_WME_AC_PARAMETERS_LIST {}
impl ::core::fmt::Debug for DOT11_WME_AC_PARAMETERS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WME_AC_PARAMETERS_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11WMEACParameters", &self.dot11WMEACParameters).finish()
    }
}
impl ::core::default::Default for DOT11_WME_UPDATE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WME_UPDATE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.uParamElemMinBeaconIntervals == other.uParamElemMinBeaconIntervals && self.uWMEInfoElemOffset == other.uWMEInfoElemOffset && self.uWMEInfoElemLength == other.uWMEInfoElemLength && self.uWMEParamElemOffset == other.uWMEParamElemOffset && self.uWMEParamElemLength == other.uWMEParamElemLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_WME_UPDATE_IE {}
impl ::core::fmt::Debug for DOT11_WME_UPDATE_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WME_UPDATE_IE").field("uParamElemMinBeaconIntervals", &self.uParamElemMinBeaconIntervals).field("uWMEInfoElemOffset", &self.uWMEInfoElemOffset).field("uWMEInfoElemLength", &self.uWMEInfoElemLength).field("uWMEParamElemOffset", &self.uWMEParamElemOffset).field("uWMEParamElemLength", &self.uWMEParamElemLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WPA_TSC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WPA_TSC {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11OffloadType == other.dot11OffloadType && self.hOffload == other.hOffload && self.dot11IV48Counter == other.dot11IV48Counter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WPA_TSC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_WPA_TSC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WPA_TSC").field("uReserved", &self.uReserved).field("dot11OffloadType", &self.dot11OffloadType).field("hOffload", &self.hOffload).field("dot11IV48Counter", &self.dot11IV48Counter).finish()
    }
}
impl ::core::default::Default for DOT11_WPS_CONFIG_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_WPS_CONFIG_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WPS_CONFIG_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOT11_WPS_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOT11_WPS_DEVICE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.uDeviceNameLength == other.uDeviceNameLength && self.ucDeviceName == other.ucDeviceName
    }
}
impl ::core::cmp::Eq for DOT11_WPS_DEVICE_NAME {}
impl ::core::fmt::Debug for DOT11_WPS_DEVICE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WPS_DEVICE_NAME").field("uDeviceNameLength", &self.uDeviceNameLength).field("ucDeviceName", &self.ucDeviceName).finish()
    }
}
impl ::core::default::Default for DOT11_WPS_DEVICE_PASSWORD_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DOT11_WPS_DEVICE_PASSWORD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WPS_DEVICE_PASSWORD_ID").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocInterface {}
impl ::core::fmt::Debug for IDot11AdHocInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocInterface").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocInterfaceNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocInterfaceNotificationSink {}
impl ::core::fmt::Debug for IDot11AdHocInterfaceNotificationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocInterfaceNotificationSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocManager {}
impl ::core::fmt::Debug for IDot11AdHocManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocManagerNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocManagerNotificationSink {}
impl ::core::fmt::Debug for IDot11AdHocManagerNotificationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocManagerNotificationSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocNetwork {}
impl ::core::fmt::Debug for IDot11AdHocNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocNetwork").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocNetworkNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocNetworkNotificationSink {}
impl ::core::fmt::Debug for IDot11AdHocNetworkNotificationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocNetworkNotificationSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocSecuritySettings {}
impl ::core::fmt::Debug for IDot11AdHocSecuritySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocSecuritySettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDot11AdHocInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocInterfaces {}
impl ::core::fmt::Debug for IEnumDot11AdHocInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDot11AdHocInterfaces").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDot11AdHocNetworks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocNetworks {}
impl ::core::fmt::Debug for IEnumDot11AdHocNetworks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDot11AdHocNetworks").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumDot11AdHocSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocSecuritySettings {}
impl ::core::fmt::Debug for IEnumDot11AdHocSecuritySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDot11AdHocSecuritySettings").field(&self.0).finish()
    }
}
impl ::core::default::Default for L2_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for L2_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationSource == other.NotificationSource && self.NotificationCode == other.NotificationCode && self.InterfaceGuid == other.InterfaceGuid && self.dwDataSize == other.dwDataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for L2_NOTIFICATION_DATA {}
impl ::core::fmt::Debug for L2_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2_NOTIFICATION_DATA").field("NotificationSource", &self.NotificationSource).field("NotificationCode", &self.NotificationCode).field("InterfaceGuid", &self.InterfaceGuid).field("dwDataSize", &self.dwDataSize).field("pData", &self.pData).finish()
    }
}
impl ::core::default::Default for ONEX_AUTH_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ONEX_AUTH_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_AUTH_IDENTITY").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ONEX_AUTH_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ONEX_AUTH_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.fUpdatePending == other.fUpdatePending && self.oneXConnProfile == other.oneXConnProfile && self.authIdentity == other.authIdentity && self.dwQuarantineState == other.dwQuarantineState && self._bitfield == other._bitfield && self.dwSessionId == other.dwSessionId && self.hUserToken == other.hUserToken && self.OneXUserProfile == other.OneXUserProfile && self.Identity == other.Identity && self.UserName == other.UserName && self.Domain == other.Domain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ONEX_AUTH_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ONEX_AUTH_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_AUTH_PARAMS")
            .field("fUpdatePending", &self.fUpdatePending)
            .field("oneXConnProfile", &self.oneXConnProfile)
            .field("authIdentity", &self.authIdentity)
            .field("dwQuarantineState", &self.dwQuarantineState)
            .field("_bitfield", &self._bitfield)
            .field("dwSessionId", &self.dwSessionId)
            .field("hUserToken", &self.hUserToken)
            .field("OneXUserProfile", &self.OneXUserProfile)
            .field("Identity", &self.Identity)
            .field("UserName", &self.UserName)
            .field("Domain", &self.Domain)
            .finish()
    }
}
impl ::core::default::Default for ONEX_AUTH_RESTART_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ONEX_AUTH_RESTART_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_AUTH_RESTART_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for ONEX_AUTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ONEX_AUTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_AUTH_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for ONEX_EAP_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for ONEX_EAP_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.dwWinError == other.dwWinError && self.r#type == other.r#type && self.dwReasonCode == other.dwReasonCode && self.rootCauseGuid == other.rootCauseGuid && self.repairGuid == other.repairGuid && self.helpLinkGuid == other.helpLinkGuid && self._bitfield == other._bitfield && self.RootCauseString == other.RootCauseString && self.RepairString == other.RepairString
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for ONEX_EAP_ERROR {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::fmt::Debug for ONEX_EAP_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_EAP_ERROR").field("dwWinError", &self.dwWinError).field("type", &self.r#type).field("dwReasonCode", &self.dwReasonCode).field("rootCauseGuid", &self.rootCauseGuid).field("repairGuid", &self.repairGuid).field("helpLinkGuid", &self.helpLinkGuid).field("_bitfield", &self._bitfield).field("RootCauseString", &self.RootCauseString).field("RepairString", &self.RepairString).finish()
    }
}
impl ::core::default::Default for ONEX_EAP_METHOD_BACKEND_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ONEX_EAP_METHOD_BACKEND_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_EAP_METHOD_BACKEND_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for ONEX_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ONEX_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ONEX_REASON_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ONEX_REASON_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_REASON_CODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ONEX_RESULT_UPDATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ONEX_RESULT_UPDATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.oneXStatus == other.oneXStatus && self.BackendSupport == other.BackendSupport && self.fBackendEngaged == other.fBackendEngaged && self._bitfield == other._bitfield && self.authParams == other.authParams && self.eapError == other.eapError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ONEX_RESULT_UPDATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ONEX_RESULT_UPDATE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_RESULT_UPDATE_DATA").field("oneXStatus", &self.oneXStatus).field("BackendSupport", &self.BackendSupport).field("fBackendEngaged", &self.fBackendEngaged).field("_bitfield", &self._bitfield).field("authParams", &self.authParams).field("eapError", &self.eapError).finish()
    }
}
impl ::core::default::Default for ONEX_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ONEX_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.authStatus == other.authStatus && self.dwReason == other.dwReason && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for ONEX_STATUS {}
impl ::core::fmt::Debug for ONEX_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_STATUS").field("authStatus", &self.authStatus).field("dwReason", &self.dwReason).field("dwError", &self.dwError).finish()
    }
}
impl ::core::default::Default for ONEX_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ONEX_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.authIdentity == other.authIdentity && self._bitfield == other._bitfield && self.UserName == other.UserName && self.DomainName == other.DomainName
    }
}
impl ::core::cmp::Eq for ONEX_USER_INFO {}
impl ::core::fmt::Debug for ONEX_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_USER_INFO").field("authIdentity", &self.authIdentity).field("_bitfield", &self._bitfield).field("UserName", &self.UserName).field("DomainName", &self.DomainName).finish()
    }
}
impl ::core::default::Default for ONEX_VARIABLE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ONEX_VARIABLE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for ONEX_VARIABLE_BLOB {}
impl ::core::fmt::Debug for ONEX_VARIABLE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_VARIABLE_BLOB").field("dwSize", &self.dwSize).field("dwOffset", &self.dwOffset).finish()
    }
}
impl ::core::default::Default for WDIAG_IHV_WLAN_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WDIAG_IHV_WLAN_ID {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.Ssid == other.Ssid && self.BssType == other.BssType && self.dwFlags == other.dwFlags && self.dwReasonCode == other.dwReasonCode
    }
}
impl ::core::cmp::Eq for WDIAG_IHV_WLAN_ID {}
impl ::core::fmt::Debug for WDIAG_IHV_WLAN_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDIAG_IHV_WLAN_ID").field("strProfileName", &self.strProfileName).field("Ssid", &self.Ssid).field("BssType", &self.BssType).field("dwFlags", &self.dwFlags).field("dwReasonCode", &self.dwReasonCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WFDSVC_CONNECTION_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WFDSVC_CONNECTION_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.bNew == other.bNew && self.bClient == other.bClient && self.bGO == other.bGO
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WFDSVC_CONNECTION_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WFDSVC_CONNECTION_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WFDSVC_CONNECTION_CAPABILITY").field("bNew", &self.bNew).field("bClient", &self.bClient).field("bGO", &self.bGO).finish()
    }
}
impl ::core::default::Default for WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WFD_GROUP_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceAddress == other.DeviceAddress && self.GroupSSID == other.GroupSSID
    }
}
impl ::core::cmp::Eq for WFD_GROUP_ID {}
impl ::core::fmt::Debug for WFD_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WFD_GROUP_ID").field("DeviceAddress", &self.DeviceAddress).field("GroupSSID", &self.GroupSSID).finish()
    }
}
impl ::core::default::Default for WFD_ROLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WFD_ROLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WFD_ROLE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_ADHOC_NETWORK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_ADHOC_NETWORK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_ADHOC_NETWORK_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_ASSOCIATION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_ASSOCIATION_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.dot11Bssid == other.dot11Bssid && self.dot11PhyType == other.dot11PhyType && self.uDot11PhyIndex == other.uDot11PhyIndex && self.wlanSignalQuality == other.wlanSignalQuality && self.ulRxRate == other.ulRxRate && self.ulTxRate == other.ulTxRate
    }
}
impl ::core::cmp::Eq for WLAN_ASSOCIATION_ATTRIBUTES {}
impl ::core::fmt::Debug for WLAN_ASSOCIATION_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_ASSOCIATION_ATTRIBUTES").field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).field("dot11Bssid", &self.dot11Bssid).field("dot11PhyType", &self.dot11PhyType).field("uDot11PhyIndex", &self.uDot11PhyIndex).field("wlanSignalQuality", &self.wlanSignalQuality).field("ulRxRate", &self.ulRxRate).field("ulTxRate", &self.ulTxRate).finish()
    }
}
impl ::core::default::Default for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.pAuthCipherPairList == other.pAuthCipherPairList
    }
}
impl ::core::cmp::Eq for WLAN_AUTH_CIPHER_PAIR_LIST {}
impl ::core::fmt::Debug for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AUTH_CIPHER_PAIR_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("pAuthCipherPairList", &self.pAuthCipherPairList).finish()
    }
}
impl ::core::default::Default for WLAN_AUTOCONF_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_AUTOCONF_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_AUTOCONF_OPCODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.uNumberOfBssids == other.uNumberOfBssids && self.bNetworkConnectable == other.bNetworkConnectable && self.wlanNotConnectableReason == other.wlanNotConnectableReason && self.uNumberOfPhyTypes == other.uNumberOfPhyTypes && self.dot11PhyTypes == other.dot11PhyTypes && self.bMorePhyTypes == other.bMorePhyTypes && self.wlanSignalQuality == other.wlanSignalQuality && self.bSecurityEnabled == other.bSecurityEnabled && self.dot11DefaultAuthAlgorithm == other.dot11DefaultAuthAlgorithm && self.dot11DefaultCipherAlgorithm == other.dot11DefaultCipherAlgorithm && self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK")
            .field("strProfileName", &self.strProfileName)
            .field("dot11Ssid", &self.dot11Ssid)
            .field("dot11BssType", &self.dot11BssType)
            .field("uNumberOfBssids", &self.uNumberOfBssids)
            .field("bNetworkConnectable", &self.bNetworkConnectable)
            .field("wlanNotConnectableReason", &self.wlanNotConnectableReason)
            .field("uNumberOfPhyTypes", &self.uNumberOfPhyTypes)
            .field("dot11PhyTypes", &self.dot11PhyTypes)
            .field("bMorePhyTypes", &self.bMorePhyTypes)
            .field("wlanSignalQuality", &self.wlanSignalQuality)
            .field("bSecurityEnabled", &self.bSecurityEnabled)
            .field("dot11DefaultAuthAlgorithm", &self.dot11DefaultAuthAlgorithm)
            .field("dot11DefaultCipherAlgorithm", &self.dot11DefaultCipherAlgorithm)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.Network == other.Network
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("Network", &self.Network).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.Network == other.Network
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_LIST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK_LIST_V2").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("Network", &self.Network).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName
            && self.dot11Ssid == other.dot11Ssid
            && self.dot11BssType == other.dot11BssType
            && self.uNumberOfBssids == other.uNumberOfBssids
            && self.bNetworkConnectable == other.bNetworkConnectable
            && self.wlanNotConnectableReason == other.wlanNotConnectableReason
            && self.uNumberOfPhyTypes == other.uNumberOfPhyTypes
            && self.dot11PhyTypes == other.dot11PhyTypes
            && self.bMorePhyTypes == other.bMorePhyTypes
            && self.wlanSignalQuality == other.wlanSignalQuality
            && self.bSecurityEnabled == other.bSecurityEnabled
            && self.dot11DefaultAuthAlgorithm == other.dot11DefaultAuthAlgorithm
            && self.dot11DefaultCipherAlgorithm == other.dot11DefaultCipherAlgorithm
            && self.dwFlags == other.dwFlags
            && self.AccessNetworkOptions == other.AccessNetworkOptions
            && self.dot11HESSID == other.dot11HESSID
            && self.VenueInfo == other.VenueInfo
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK_V2")
            .field("strProfileName", &self.strProfileName)
            .field("dot11Ssid", &self.dot11Ssid)
            .field("dot11BssType", &self.dot11BssType)
            .field("uNumberOfBssids", &self.uNumberOfBssids)
            .field("bNetworkConnectable", &self.bNetworkConnectable)
            .field("wlanNotConnectableReason", &self.wlanNotConnectableReason)
            .field("uNumberOfPhyTypes", &self.uNumberOfPhyTypes)
            .field("dot11PhyTypes", &self.dot11PhyTypes)
            .field("bMorePhyTypes", &self.bMorePhyTypes)
            .field("wlanSignalQuality", &self.wlanSignalQuality)
            .field("bSecurityEnabled", &self.bSecurityEnabled)
            .field("dot11DefaultAuthAlgorithm", &self.dot11DefaultAuthAlgorithm)
            .field("dot11DefaultCipherAlgorithm", &self.dot11DefaultCipherAlgorithm)
            .field("dwFlags", &self.dwFlags)
            .field("AccessNetworkOptions", &self.AccessNetworkOptions)
            .field("dot11HESSID", &self.dot11HESSID)
            .field("VenueInfo", &self.VenueInfo)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_BSS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_BSS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dot11Ssid == other.dot11Ssid && self.uPhyId == other.uPhyId && self.dot11Bssid == other.dot11Bssid && self.dot11BssType == other.dot11BssType && self.dot11BssPhyType == other.dot11BssPhyType && self.lRssi == other.lRssi && self.uLinkQuality == other.uLinkQuality && self.bInRegDomain == other.bInRegDomain && self.usBeaconPeriod == other.usBeaconPeriod && self.ullTimestamp == other.ullTimestamp && self.ullHostTimestamp == other.ullHostTimestamp && self.usCapabilityInformation == other.usCapabilityInformation && self.ulChCenterFrequency == other.ulChCenterFrequency && self.wlanRateSet == other.wlanRateSet && self.ulIeOffset == other.ulIeOffset && self.ulIeSize == other.ulIeSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_BSS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_BSS_ENTRY")
            .field("dot11Ssid", &self.dot11Ssid)
            .field("uPhyId", &self.uPhyId)
            .field("dot11Bssid", &self.dot11Bssid)
            .field("dot11BssType", &self.dot11BssType)
            .field("dot11BssPhyType", &self.dot11BssPhyType)
            .field("lRssi", &self.lRssi)
            .field("uLinkQuality", &self.uLinkQuality)
            .field("bInRegDomain", &self.bInRegDomain)
            .field("usBeaconPeriod", &self.usBeaconPeriod)
            .field("ullTimestamp", &self.ullTimestamp)
            .field("ullHostTimestamp", &self.ullHostTimestamp)
            .field("usCapabilityInformation", &self.usCapabilityInformation)
            .field("ulChCenterFrequency", &self.ulChCenterFrequency)
            .field("wlanRateSet", &self.wlanRateSet)
            .field("ulIeOffset", &self.ulIeOffset)
            .field("ulIeSize", &self.ulIeSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_BSS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_BSS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwNumberOfItems == other.dwNumberOfItems && self.wlanBssEntries == other.wlanBssEntries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_BSS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_BSS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_BSS_LIST").field("dwTotalSize", &self.dwTotalSize).field("dwNumberOfItems", &self.dwNumberOfItems).field("wlanBssEntries", &self.wlanBssEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_CONNECTION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.isState == other.isState && self.wlanConnectionMode == other.wlanConnectionMode && self.strProfileName == other.strProfileName && self.wlanAssociationAttributes == other.wlanAssociationAttributes && self.wlanSecurityAttributes == other.wlanSecurityAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_CONNECTION_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_CONNECTION_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_ATTRIBUTES").field("isState", &self.isState).field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfileName", &self.strProfileName).field("wlanAssociationAttributes", &self.wlanAssociationAttributes).field("wlanSecurityAttributes", &self.wlanSecurityAttributes).finish()
    }
}
impl ::core::default::Default for WLAN_CONNECTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_CONNECTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_CONNECTION_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfileName == other.strProfileName && self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.bSecurityEnabled == other.bSecurityEnabled && self.wlanReasonCode == other.wlanReasonCode && self.dwFlags == other.dwFlags && self.strProfileXml == other.strProfileXml
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_CONNECTION_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_NOTIFICATION_DATA").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfileName", &self.strProfileName).field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).field("bSecurityEnabled", &self.bSecurityEnabled).field("wlanReasonCode", &self.wlanReasonCode).field("dwFlags", &self.dwFlags).field("strProfileXml", &self.strProfileXml).finish()
    }
}
impl ::core::default::Default for WLAN_CONNECTION_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_CONNECTION_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_CONNECTION_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for WLAN_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfile == other.strProfile && self.pDot11Ssid == other.pDot11Ssid && self.pDesiredBssidList == other.pDesiredBssidList && self.dot11BssType == other.dot11BssType && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for WLAN_CONNECTION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for WLAN_CONNECTION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_PARAMETERS").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfile", &self.strProfile).field("pDot11Ssid", &self.pDot11Ssid).field("pDesiredBssidList", &self.pDesiredBssidList).field("dot11BssType", &self.dot11BssType).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for WLAN_CONNECTION_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_PARAMETERS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfile == other.strProfile && self.pDot11Ssid == other.pDot11Ssid && self.pDot11Hessid == other.pDot11Hessid && self.pDesiredBssidList == other.pDesiredBssidList && self.dot11BssType == other.dot11BssType && self.dwFlags == other.dwFlags && self.pDot11AccessNetworkOptions == other.pDot11AccessNetworkOptions
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for WLAN_CONNECTION_PARAMETERS_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for WLAN_CONNECTION_PARAMETERS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_PARAMETERS_V2").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfile", &self.strProfile).field("pDot11Ssid", &self.pDot11Ssid).field("pDot11Hessid", &self.pDot11Hessid).field("pDesiredBssidList", &self.pDesiredBssidList).field("dot11BssType", &self.dot11BssType).field("dwFlags", &self.dwFlags).field("pDot11AccessNetworkOptions", &self.pDot11AccessNetworkOptions).finish()
    }
}
impl ::core::default::Default for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.pCountryOrRegionStringList == other.pCountryOrRegionStringList
    }
}
impl ::core::cmp::Eq for WLAN_COUNTRY_OR_REGION_STRING_LIST {}
impl ::core::fmt::Debug for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_COUNTRY_OR_REGION_STRING_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("pCountryOrRegionStringList", &self.pCountryOrRegionStringList).finish()
    }
}
impl ::core::default::Default for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.DeviceService == other.DeviceService
    }
}
impl ::core::cmp::Eq for WLAN_DEVICE_SERVICE_GUID_LIST {}
impl ::core::fmt::Debug for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_DEVICE_SERVICE_GUID_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("DeviceService", &self.DeviceService).finish()
    }
}
impl ::core::default::Default for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceService == other.DeviceService && self.dwOpCode == other.dwOpCode && self.dwDataSize == other.dwDataSize && self.DataBlob == other.DataBlob
    }
}
impl ::core::cmp::Eq for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {}
impl ::core::fmt::Debug for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_DEVICE_SERVICE_NOTIFICATION_DATA").field("DeviceService", &self.DeviceService).field("dwOpCode", &self.dwOpCode).field("dwDataSize", &self.dwDataSize).field("DataBlob", &self.DataBlob).finish()
    }
}
impl ::core::default::Default for WLAN_FILTER_LIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_FILTER_LIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_FILTER_LIST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.hostedNetworkSSID == other.hostedNetworkSSID && self.dwMaxNumberOfPeers == other.dwMaxNumberOfPeers
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS").field("hostedNetworkSSID", &self.hostedNetworkSSID).field("dwMaxNumberOfPeers", &self.dwMaxNumberOfPeers).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OldState == other.OldState && self.NewState == other.NewState && self.PeerStateChangeReason == other.PeerStateChangeReason
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE").field("OldState", &self.OldState).field("NewState", &self.NewState).field("PeerStateChangeReason", &self.PeerStateChangeReason).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_NOTIFICATION_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_OPCODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_PEER_AUTH_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.PeerAuthState == other.PeerAuthState
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_PEER_STATE {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_PEER_STATE").field("PeerMacAddress", &self.PeerMacAddress).field("PeerAuthState", &self.PeerAuthState).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11SoftwareRadioState == other.dot11SoftwareRadioState && self.dot11HardwareRadioState == other.dot11HardwareRadioState
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_RADIO_STATE {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_RADIO_STATE").field("dot11SoftwareRadioState", &self.dot11SoftwareRadioState).field("dot11HardwareRadioState", &self.dot11HardwareRadioState).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11AuthAlgo == other.dot11AuthAlgo && self.dot11CipherAlgo == other.dot11CipherAlgo
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_SECURITY_SETTINGS").field("dot11AuthAlgo", &self.dot11AuthAlgo).field("dot11CipherAlgo", &self.dot11CipherAlgo).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OldState == other.OldState && self.NewState == other.NewState && self.StateChangeReason == other.StateChangeReason
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_STATE_CHANGE {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_STATE_CHANGE").field("OldState", &self.OldState).field("NewState", &self.NewState).field("StateChangeReason", &self.StateChangeReason).finish()
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.HostedNetworkState == other.HostedNetworkState && self.IPDeviceID == other.IPDeviceID && self.wlanHostedNetworkBSSID == other.wlanHostedNetworkBSSID && self.dot11PhyType == other.dot11PhyType && self.ulChannelFrequency == other.ulChannelFrequency && self.dwNumberOfPeers == other.dwNumberOfPeers && self.PeerList == other.PeerList
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_STATUS {}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_STATUS").field("HostedNetworkState", &self.HostedNetworkState).field("IPDeviceID", &self.IPDeviceID).field("wlanHostedNetworkBSSID", &self.wlanHostedNetworkBSSID).field("dot11PhyType", &self.dot11PhyType).field("ulChannelFrequency", &self.ulChannelFrequency).field("dwNumberOfPeers", &self.dwNumberOfPeers).field("PeerList", &self.PeerList).finish()
    }
}
impl ::core::default::Default for WLAN_IHV_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_IHV_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_IHV_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_INTERFACE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_INTERFACE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.interfaceType == other.interfaceType && self.bDot11DSupported == other.bDot11DSupported && self.dwMaxDesiredSsidListSize == other.dwMaxDesiredSsidListSize && self.dwMaxDesiredBssidListSize == other.dwMaxDesiredBssidListSize && self.dwNumberOfSupportedPhys == other.dwNumberOfSupportedPhys && self.dot11PhyTypes == other.dot11PhyTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_INTERFACE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_INTERFACE_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_INTERFACE_CAPABILITY").field("interfaceType", &self.interfaceType).field("bDot11DSupported", &self.bDot11DSupported).field("dwMaxDesiredSsidListSize", &self.dwMaxDesiredSsidListSize).field("dwMaxDesiredBssidListSize", &self.dwMaxDesiredBssidListSize).field("dwNumberOfSupportedPhys", &self.dwNumberOfSupportedPhys).field("dot11PhyTypes", &self.dot11PhyTypes).finish()
    }
}
impl ::core::default::Default for WLAN_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceGuid == other.InterfaceGuid && self.strInterfaceDescription == other.strInterfaceDescription && self.isState == other.isState
    }
}
impl ::core::cmp::Eq for WLAN_INTERFACE_INFO {}
impl ::core::fmt::Debug for WLAN_INTERFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_INTERFACE_INFO").field("InterfaceGuid", &self.InterfaceGuid).field("strInterfaceDescription", &self.strInterfaceDescription).field("isState", &self.isState).finish()
    }
}
impl ::core::default::Default for WLAN_INTERFACE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_INTERFACE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.InterfaceInfo == other.InterfaceInfo
    }
}
impl ::core::cmp::Eq for WLAN_INTERFACE_INFO_LIST {}
impl ::core::fmt::Debug for WLAN_INTERFACE_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_INTERFACE_INFO_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("InterfaceInfo", &self.InterfaceInfo).finish()
    }
}
impl ::core::default::Default for WLAN_INTERFACE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_INTERFACE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_INTERFACE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_INTERFACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_INTERFACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_INTERFACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_INTF_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_INTF_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_INTF_OPCODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_MAC_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount && self.ullReceivedFrameCount == other.ullReceivedFrameCount && self.ullWEPExcludedCount == other.ullWEPExcludedCount && self.ullTKIPLocalMICFailures == other.ullTKIPLocalMICFailures && self.ullTKIPReplays == other.ullTKIPReplays && self.ullTKIPICVErrorCount == other.ullTKIPICVErrorCount && self.ullCCMPReplays == other.ullCCMPReplays && self.ullCCMPDecryptErrors == other.ullCCMPDecryptErrors && self.ullWEPUndecryptableCount == other.ullWEPUndecryptableCount && self.ullWEPICVErrorCount == other.ullWEPICVErrorCount && self.ullDecryptSuccessCount == other.ullDecryptSuccessCount && self.ullDecryptFailureCount == other.ullDecryptFailureCount
    }
}
impl ::core::cmp::Eq for WLAN_MAC_FRAME_STATISTICS {}
impl ::core::fmt::Debug for WLAN_MAC_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_MAC_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullWEPExcludedCount", &self.ullWEPExcludedCount)
            .field("ullTKIPLocalMICFailures", &self.ullTKIPLocalMICFailures)
            .field("ullTKIPReplays", &self.ullTKIPReplays)
            .field("ullTKIPICVErrorCount", &self.ullTKIPICVErrorCount)
            .field("ullCCMPReplays", &self.ullCCMPReplays)
            .field("ullCCMPDecryptErrors", &self.ullCCMPDecryptErrors)
            .field("ullWEPUndecryptableCount", &self.ullWEPUndecryptableCount)
            .field("ullWEPICVErrorCount", &self.ullWEPICVErrorCount)
            .field("ullDecryptSuccessCount", &self.ullDecryptSuccessCount)
            .field("ullDecryptFailureCount", &self.ullDecryptFailureCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_MSM_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_MSM_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfileName == other.strProfileName && self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.dot11MacAddr == other.dot11MacAddr && self.bSecurityEnabled == other.bSecurityEnabled && self.bFirstPeer == other.bFirstPeer && self.bLastPeer == other.bLastPeer && self.wlanReasonCode == other.wlanReasonCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_MSM_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_MSM_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_MSM_NOTIFICATION_DATA").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfileName", &self.strProfileName).field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).field("dot11MacAddr", &self.dot11MacAddr).field("bSecurityEnabled", &self.bSecurityEnabled).field("bFirstPeer", &self.bFirstPeer).field("bLastPeer", &self.bLastPeer).field("wlanReasonCode", &self.wlanReasonCode).finish()
    }
}
impl ::core::default::Default for WLAN_NOTIFICATION_ACM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_NOTIFICATION_ACM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_NOTIFICATION_ACM").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_NOTIFICATION_MSM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_NOTIFICATION_MSM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_NOTIFICATION_MSM").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_NOTIFICATION_SECURITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_NOTIFICATION_SECURITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_NOTIFICATION_SECURITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_OPCODE_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_OPCODE_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_OPCODE_VALUE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_OPERATIONAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_OPERATIONAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_OPERATIONAL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_PHY_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount
            && self.ullMulticastTransmittedFrameCount == other.ullMulticastTransmittedFrameCount
            && self.ullFailedCount == other.ullFailedCount
            && self.ullRetryCount == other.ullRetryCount
            && self.ullMultipleRetryCount == other.ullMultipleRetryCount
            && self.ullMaxTXLifetimeExceededCount == other.ullMaxTXLifetimeExceededCount
            && self.ullTransmittedFragmentCount == other.ullTransmittedFragmentCount
            && self.ullRTSSuccessCount == other.ullRTSSuccessCount
            && self.ullRTSFailureCount == other.ullRTSFailureCount
            && self.ullACKFailureCount == other.ullACKFailureCount
            && self.ullReceivedFrameCount == other.ullReceivedFrameCount
            && self.ullMulticastReceivedFrameCount == other.ullMulticastReceivedFrameCount
            && self.ullPromiscuousReceivedFrameCount == other.ullPromiscuousReceivedFrameCount
            && self.ullMaxRXLifetimeExceededCount == other.ullMaxRXLifetimeExceededCount
            && self.ullFrameDuplicateCount == other.ullFrameDuplicateCount
            && self.ullReceivedFragmentCount == other.ullReceivedFragmentCount
            && self.ullPromiscuousReceivedFragmentCount == other.ullPromiscuousReceivedFragmentCount
            && self.ullFCSErrorCount == other.ullFCSErrorCount
    }
}
impl ::core::cmp::Eq for WLAN_PHY_FRAME_STATISTICS {}
impl ::core::fmt::Debug for WLAN_PHY_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PHY_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullMulticastTransmittedFrameCount", &self.ullMulticastTransmittedFrameCount)
            .field("ullFailedCount", &self.ullFailedCount)
            .field("ullRetryCount", &self.ullRetryCount)
            .field("ullMultipleRetryCount", &self.ullMultipleRetryCount)
            .field("ullMaxTXLifetimeExceededCount", &self.ullMaxTXLifetimeExceededCount)
            .field("ullTransmittedFragmentCount", &self.ullTransmittedFragmentCount)
            .field("ullRTSSuccessCount", &self.ullRTSSuccessCount)
            .field("ullRTSFailureCount", &self.ullRTSFailureCount)
            .field("ullACKFailureCount", &self.ullACKFailureCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullMulticastReceivedFrameCount", &self.ullMulticastReceivedFrameCount)
            .field("ullPromiscuousReceivedFrameCount", &self.ullPromiscuousReceivedFrameCount)
            .field("ullMaxRXLifetimeExceededCount", &self.ullMaxRXLifetimeExceededCount)
            .field("ullFrameDuplicateCount", &self.ullFrameDuplicateCount)
            .field("ullReceivedFragmentCount", &self.ullReceivedFragmentCount)
            .field("ullPromiscuousReceivedFragmentCount", &self.ullPromiscuousReceivedFragmentCount)
            .field("ullFCSErrorCount", &self.ullFCSErrorCount)
            .finish()
    }
}
impl ::core::default::Default for WLAN_PHY_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_PHY_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwPhyIndex == other.dwPhyIndex && self.dot11SoftwareRadioState == other.dot11SoftwareRadioState && self.dot11HardwareRadioState == other.dot11HardwareRadioState
    }
}
impl ::core::cmp::Eq for WLAN_PHY_RADIO_STATE {}
impl ::core::fmt::Debug for WLAN_PHY_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PHY_RADIO_STATE").field("dwPhyIndex", &self.dwPhyIndex).field("dot11SoftwareRadioState", &self.dot11SoftwareRadioState).field("dot11HardwareRadioState", &self.dot11HardwareRadioState).finish()
    }
}
impl ::core::default::Default for WLAN_POWER_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_POWER_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_POWER_SETTING").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for WLAN_PROFILE_INFO {}
impl ::core::fmt::Debug for WLAN_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PROFILE_INFO").field("strProfileName", &self.strProfileName).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for WLAN_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_PROFILE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.ProfileInfo == other.ProfileInfo
    }
}
impl ::core::cmp::Eq for WLAN_PROFILE_INFO_LIST {}
impl ::core::fmt::Debug for WLAN_PROFILE_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PROFILE_INFO_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("ProfileInfo", &self.ProfileInfo).finish()
    }
}
impl ::core::default::Default for WLAN_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfPhys == other.dwNumberOfPhys && self.PhyRadioState == other.PhyRadioState
    }
}
impl ::core::cmp::Eq for WLAN_RADIO_STATE {}
impl ::core::fmt::Debug for WLAN_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RADIO_STATE").field("dwNumberOfPhys", &self.dwNumberOfPhys).field("PhyRadioState", &self.PhyRadioState).finish()
    }
}
impl ::core::default::Default for WLAN_RATE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_RATE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.uRateSetLength == other.uRateSetLength && self.usRateSet == other.usRateSet
    }
}
impl ::core::cmp::Eq for WLAN_RATE_SET {}
impl ::core::fmt::Debug for WLAN_RATE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RATE_SET").field("uRateSetLength", &self.uRateSetLength).field("usRateSet", &self.usRateSet).finish()
    }
}
impl ::core::default::Default for WLAN_RAW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataSize == other.dwDataSize && self.DataBlob == other.DataBlob
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA {}
impl ::core::fmt::Debug for WLAN_RAW_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RAW_DATA").field("dwDataSize", &self.dwDataSize).field("DataBlob", &self.DataBlob).finish()
    }
}
impl ::core::default::Default for WLAN_RAW_DATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwNumberOfItems == other.dwNumberOfItems && self.DataList == other.DataList
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA_LIST {}
impl ::core::fmt::Debug for WLAN_RAW_DATA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RAW_DATA_LIST").field("dwTotalSize", &self.dwTotalSize).field("dwNumberOfItems", &self.dwNumberOfItems).field("DataList", &self.DataList).finish()
    }
}
impl ::core::default::Default for WLAN_RAW_DATA_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA_LIST_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA_LIST_0 {}
impl ::core::fmt::Debug for WLAN_RAW_DATA_LIST_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RAW_DATA_LIST_0").field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).finish()
    }
}
impl ::core::default::Default for WLAN_SECURABLE_OBJECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_SECURABLE_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_SECURABLE_OBJECT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.bSecurityEnabled == other.bSecurityEnabled && self.bOneXEnabled == other.bOneXEnabled && self.dot11AuthAlgorithm == other.dot11AuthAlgorithm && self.dot11CipherAlgorithm == other.dot11CipherAlgorithm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_SECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_SECURITY_ATTRIBUTES").field("bSecurityEnabled", &self.bSecurityEnabled).field("bOneXEnabled", &self.bOneXEnabled).field("dot11AuthAlgorithm", &self.dot11AuthAlgorithm).field("dot11CipherAlgorithm", &self.dot11CipherAlgorithm).finish()
    }
}
impl ::core::default::Default for WLAN_SET_EAPHOST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WLAN_SET_EAPHOST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_SET_EAPHOST_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WLAN_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WLAN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullFourWayHandshakeFailures == other.ullFourWayHandshakeFailures && self.ullTKIPCounterMeasuresInvoked == other.ullTKIPCounterMeasuresInvoked && self.ullReserved == other.ullReserved && self.MacUcastCounters == other.MacUcastCounters && self.MacMcastCounters == other.MacMcastCounters && self.dwNumberOfPhys == other.dwNumberOfPhys && self.PhyCounters == other.PhyCounters
    }
}
impl ::core::cmp::Eq for WLAN_STATISTICS {}
impl ::core::fmt::Debug for WLAN_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_STATISTICS").field("ullFourWayHandshakeFailures", &self.ullFourWayHandshakeFailures).field("ullTKIPCounterMeasuresInvoked", &self.ullTKIPCounterMeasuresInvoked).field("ullReserved", &self.ullReserved).field("MacUcastCounters", &self.MacUcastCounters).field("MacMcastCounters", &self.MacMcastCounters).field("dwNumberOfPhys", &self.dwNumberOfPhys).field("PhyCounters", &self.PhyCounters).finish()
    }
}
impl ::core::default::Default for WL_DISPLAY_PAGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WL_DISPLAY_PAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WL_DISPLAY_PAGES").field(&self.0).finish()
    }
}
