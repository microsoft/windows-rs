#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUTH_VALIDATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.hRasConnection == other.hRasConnection && self.wszUserName == other.wszUserName && self.wszLogonDomain == other.wszLogonDomain && self.AuthInfoSize == other.AuthInfoSize && self.AuthInfo == other.AuthInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUTH_VALIDATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTH_VALIDATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTH_VALIDATION_EX").field("Header", &self.Header).field("hRasConnection", &self.hRasConnection).field("wszUserName", &self.wszUserName).field("wszLogonDomain", &self.wszLogonDomain).field("AuthInfoSize", &self.AuthInfoSize).field("AuthInfo", &self.AuthInfo).finish()
    }
}
impl ::core::default::Default for GRE_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GRE_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags
    }
}
impl ::core::cmp::Eq for GRE_CONFIG_PARAMS0 {}
impl ::core::fmt::Debug for GRE_CONFIG_PARAMS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRE_CONFIG_PARAMS0").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for IKEV2_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for IKEV2_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags && self.dwTunnelConfigParamFlags == other.dwTunnelConfigParamFlags && self.TunnelConfigParams == other.TunnelConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for IKEV2_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for IKEV2_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("dwTunnelConfigParamFlags", &self.dwTunnelConfigParamFlags).field("TunnelConfigParams", &self.TunnelConfigParams).finish()
    }
}
impl ::core::default::Default for IKEV2_ID_PAYLOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEV2_ID_PAYLOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEV2_ID_PAYLOAD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEV2_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwOptions == other.dwOptions
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwEncryptionMethod == other.dwEncryptionMethod
    }
}
impl ::core::cmp::Eq for IKEV2_PROJECTION_INFO {}
impl ::core::fmt::Debug for IKEV2_PROJECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_PROJECTION_INFO")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwOptions", &self.dwOptions)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwEncryptionMethod", &self.dwEncryptionMethod)
            .finish()
    }
}
impl ::core::default::Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEV2_PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwOptions == other.dwOptions
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwEmbeddedEAPTypeId == other.dwEmbeddedEAPTypeId
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwEncryptionMethod == other.dwEncryptionMethod
    }
}
impl ::core::cmp::Eq for IKEV2_PROJECTION_INFO2 {}
impl ::core::fmt::Debug for IKEV2_PROJECTION_INFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_PROJECTION_INFO2")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwOptions", &self.dwOptions)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwEmbeddedEAPTypeId", &self.dwEmbeddedEAPTypeId)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwEncryptionMethod", &self.dwEncryptionMethod)
            .finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwNetworkBlackoutTime == other.dwNetworkBlackoutTime && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.dwConfigOptions == other.dwConfigOptions && self.dwTotalCertificates == other.dwTotalCertificates && self.certificateNames == other.certificateNames && self.machineCertificateName == other.machineCertificateName && self.dwEncryptionType == other.dwEncryptionType && self.customPolicy == other.customPolicy
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS2 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS2")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwNetworkBlackoutTime", &self.dwNetworkBlackoutTime)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("dwConfigOptions", &self.dwConfigOptions)
            .field("dwTotalCertificates", &self.dwTotalCertificates)
            .field("certificateNames", &self.certificateNames)
            .field("machineCertificateName", &self.machineCertificateName)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("customPolicy", &self.customPolicy)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwNetworkBlackoutTime == other.dwNetworkBlackoutTime && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.dwConfigOptions == other.dwConfigOptions && self.dwTotalCertificates == other.dwTotalCertificates && self.certificateNames == other.certificateNames && self.machineCertificateName == other.machineCertificateName && self.dwEncryptionType == other.dwEncryptionType && self.customPolicy == other.customPolicy && self.dwTotalEkus == other.dwTotalEkus && self.certificateEKUs == other.certificateEKUs && self.machineCertificateHash == other.machineCertificateHash
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS3")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwNetworkBlackoutTime", &self.dwNetworkBlackoutTime)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("dwConfigOptions", &self.dwConfigOptions)
            .field("dwTotalCertificates", &self.dwTotalCertificates)
            .field("certificateNames", &self.certificateNames)
            .field("machineCertificateName", &self.machineCertificateName)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("customPolicy", &self.customPolicy)
            .field("dwTotalEkus", &self.dwTotalEkus)
            .field("certificateEKUs", &self.certificateEKUs)
            .field("machineCertificateHash", &self.machineCertificateHash)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwNetworkBlackoutTime == other.dwNetworkBlackoutTime && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.dwConfigOptions == other.dwConfigOptions && self.dwTotalCertificates == other.dwTotalCertificates && self.certificateNames == other.certificateNames && self.machineCertificateName == other.machineCertificateName && self.dwEncryptionType == other.dwEncryptionType && self.customPolicy == other.customPolicy && self.dwTotalEkus == other.dwTotalEkus && self.certificateEKUs == other.certificateEKUs && self.machineCertificateHash == other.machineCertificateHash && self.dwMmSaLifeTime == other.dwMmSaLifeTime
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS4")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwNetworkBlackoutTime", &self.dwNetworkBlackoutTime)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("dwConfigOptions", &self.dwConfigOptions)
            .field("dwTotalCertificates", &self.dwTotalCertificates)
            .field("certificateNames", &self.certificateNames)
            .field("machineCertificateName", &self.machineCertificateName)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("customPolicy", &self.customPolicy)
            .field("dwTotalEkus", &self.dwTotalEkus)
            .field("certificateEKUs", &self.certificateEKUs)
            .field("machineCertificateHash", &self.machineCertificateHash)
            .field("dwMmSaLifeTime", &self.dwMmSaLifeTime)
            .finish()
    }
}
impl ::core::default::Default for L2TP_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for L2TP_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags
    }
}
impl ::core::cmp::Eq for L2TP_CONFIG_PARAMS0 {}
impl ::core::fmt::Debug for L2TP_CONFIG_PARAMS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_CONFIG_PARAMS0").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
impl ::core::default::Default for L2TP_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for L2TP_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags && self.dwTunnelConfigParamFlags == other.dwTunnelConfigParamFlags && self.TunnelConfigParams == other.TunnelConfigParams
    }
}
impl ::core::cmp::Eq for L2TP_CONFIG_PARAMS1 {}
impl ::core::fmt::Debug for L2TP_CONFIG_PARAMS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_CONFIG_PARAMS1").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("dwTunnelConfigParamFlags", &self.dwTunnelConfigParamFlags).field("TunnelConfigParams", &self.TunnelConfigParams).finish()
    }
}
impl ::core::default::Default for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwEncryptionType == other.dwEncryptionType && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.customPolicy == other.customPolicy
    }
}
impl ::core::cmp::Eq for L2TP_TUNNEL_CONFIG_PARAMS1 {}
impl ::core::fmt::Debug for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_TUNNEL_CONFIG_PARAMS1").field("dwIdleTimeout", &self.dwIdleTimeout).field("dwEncryptionType", &self.dwEncryptionType).field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation).field("customPolicy", &self.customPolicy).finish()
    }
}
impl ::core::default::Default for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwEncryptionType == other.dwEncryptionType && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.customPolicy == other.customPolicy && self.dwMmSaLifeTime == other.dwMmSaLifeTime
    }
}
impl ::core::cmp::Eq for L2TP_TUNNEL_CONFIG_PARAMS2 {}
impl ::core::fmt::Debug for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_TUNNEL_CONFIG_PARAMS2").field("dwIdleTimeout", &self.dwIdleTimeout).field("dwEncryptionType", &self.dwEncryptionType).field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation).field("customPolicy", &self.customPolicy).field("dwMmSaLifeTime", &self.dwMmSaLifeTime).finish()
    }
}
impl ::core::default::Default for MGM_ENUM_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MGM_ENUM_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MGM_ENUM_TYPES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MGM_IF_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MGM_IF_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIfIndex == other.dwIfIndex && self.dwIfNextHopAddr == other.dwIfNextHopAddr && self.bIGMP == other.bIGMP && self.bIsEnabled == other.bIsEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MGM_IF_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MGM_IF_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MGM_IF_ENTRY").field("dwIfIndex", &self.dwIfIndex).field("dwIfNextHopAddr", &self.dwIfNextHopAddr).field("bIGMP", &self.bIGMP).field("bIsEnabled", &self.bIsEnabled).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MPRAPI_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPRAPI_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision && self.r#type == other.r#type && self.size == other.size
    }
}
impl ::core::cmp::Eq for MPRAPI_OBJECT_HEADER {}
impl ::core::fmt::Debug for MPRAPI_OBJECT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_OBJECT_HEADER").field("revision", &self.revision).field("type", &self.r#type).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MPRAPI_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MPRAPI_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPRAPI_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        self.IkeConfigParams == other.IkeConfigParams && self.PptpConfigParams == other.PptpConfigParams && self.L2tpConfigParams == other.L2tpConfigParams && self.SstpConfigParams == other.SstpConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPRAPI_TUNNEL_CONFIG_PARAMS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_TUNNEL_CONFIG_PARAMS0").field("IkeConfigParams", &self.IkeConfigParams).field("PptpConfigParams", &self.PptpConfigParams).field("L2tpConfigParams", &self.L2tpConfigParams).field("SstpConfigParams", &self.SstpConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        self.IkeConfigParams == other.IkeConfigParams && self.PptpConfigParams == other.PptpConfigParams && self.L2tpConfigParams == other.L2tpConfigParams && self.SstpConfigParams == other.SstpConfigParams && self.GREConfigParams == other.GREConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPRAPI_TUNNEL_CONFIG_PARAMS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_TUNNEL_CONFIG_PARAMS1").field("IkeConfigParams", &self.IkeConfigParams).field("PptpConfigParams", &self.PptpConfigParams).field("L2tpConfigParams", &self.L2tpConfigParams).field("SstpConfigParams", &self.SstpConfigParams).field("GREConfigParams", &self.GREConfigParams).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_CERT_EKU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_CERT_EKU {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.IsEKUOID == other.IsEKUOID && self.pwszEKU == other.pwszEKU
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_CERT_EKU {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_CERT_EKU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_CERT_EKU").field("dwSize", &self.dwSize).field("IsEKUOID", &self.IsEKUOID).field("pwszEKU", &self.pwszEKU).finish()
    }
}
impl ::core::default::Default for MPR_CREDENTIALSEX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_CREDENTIALSEX_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpbCredentialsInfo == other.lpbCredentialsInfo
    }
}
impl ::core::cmp::Eq for MPR_CREDENTIALSEX_0 {}
impl ::core::fmt::Debug for MPR_CREDENTIALSEX_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_CREDENTIALSEX_0").field("dwSize", &self.dwSize).field("lpbCredentialsInfo", &self.lpbCredentialsInfo).finish()
    }
}
impl ::core::default::Default for MPR_CREDENTIALSEX_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_CREDENTIALSEX_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpbCredentialsInfo == other.lpbCredentialsInfo
    }
}
impl ::core::cmp::Eq for MPR_CREDENTIALSEX_1 {}
impl ::core::fmt::Debug for MPR_CREDENTIALSEX_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_CREDENTIALSEX_1").field("dwSize", &self.dwSize).field("lpbCredentialsInfo", &self.lpbCredentialsInfo).finish()
    }
}
impl ::core::default::Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_DEVICE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName
    }
}
impl ::core::cmp::Eq for MPR_DEVICE_0 {}
impl ::core::fmt::Debug for MPR_DEVICE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_DEVICE_0").field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
impl ::core::default::Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_DEVICE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName && self.szLocalPhoneNumber == other.szLocalPhoneNumber && self.szAlternates == other.szAlternates
    }
}
impl ::core::cmp::Eq for MPR_DEVICE_1 {}
impl ::core::fmt::Debug for MPR_DEVICE_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_DEVICE_1").field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("szAlternates", &self.szAlternates).finish()
    }
}
impl ::core::default::Default for MPR_ET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MPR_ET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_ET").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_FILTER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_FILTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.fEnable == other.fEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_FILTER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_FILTER_0").field("fEnable", &self.fEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_IFTRANSPORT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransportId == other.dwTransportId && self.hIfTransport == other.hIfTransport && self.wszIfTransportName == other.wszIfTransportName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_IFTRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_IFTRANSPORT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IFTRANSPORT_0").field("dwTransportId", &self.dwTransportId).field("hIfTransport", &self.hIfTransport).field("wszIfTransportName", &self.wszIfTransportName).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for MPR_IF_CUSTOMINFOEX0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for MPR_IF_CUSTOMINFOEX0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwFlags == other.dwFlags && self.customIkev2Config == other.customIkev2Config
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for MPR_IF_CUSTOMINFOEX0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for MPR_IF_CUSTOMINFOEX0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IF_CUSTOMINFOEX0").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for MPR_IF_CUSTOMINFOEX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for MPR_IF_CUSTOMINFOEX1 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwFlags == other.dwFlags && self.customIkev2Config == other.customIkev2Config
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for MPR_IF_CUSTOMINFOEX1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for MPR_IF_CUSTOMINFOEX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IF_CUSTOMINFOEX1").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_IF_CUSTOMINFOEX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_IF_CUSTOMINFOEX2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwFlags == other.dwFlags && self.customIkev2Config == other.customIkev2Config
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_IF_CUSTOMINFOEX2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_IF_CUSTOMINFOEX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IF_CUSTOMINFOEX2").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_INTERFACE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wszInterfaceName == other.wszInterfaceName && self.hInterface == other.hInterface && self.fEnabled == other.fEnabled && self.dwIfType == other.dwIfType && self.dwConnectionState == other.dwConnectionState && self.fUnReachabilityReasons == other.fUnReachabilityReasons && self.dwLastError == other.dwLastError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_INTERFACE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_INTERFACE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_INTERFACE_0").field("wszInterfaceName", &self.wszInterfaceName).field("hInterface", &self.hInterface).field("fEnabled", &self.fEnabled).field("dwIfType", &self.dwIfType).field("dwConnectionState", &self.dwConnectionState).field("fUnReachabilityReasons", &self.fUnReachabilityReasons).field("dwLastError", &self.dwLastError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_INTERFACE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.wszInterfaceName == other.wszInterfaceName && self.hInterface == other.hInterface && self.fEnabled == other.fEnabled && self.dwIfType == other.dwIfType && self.dwConnectionState == other.dwConnectionState && self.fUnReachabilityReasons == other.fUnReachabilityReasons && self.dwLastError == other.dwLastError && self.lpwsDialoutHoursRestriction == other.lpwsDialoutHoursRestriction
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_INTERFACE_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_INTERFACE_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_INTERFACE_1").field("wszInterfaceName", &self.wszInterfaceName).field("hInterface", &self.hInterface).field("fEnabled", &self.fEnabled).field("dwIfType", &self.dwIfType).field("dwConnectionState", &self.dwConnectionState).field("fUnReachabilityReasons", &self.fUnReachabilityReasons).field("dwLastError", &self.dwLastError).field("lpwsDialoutHoursRestriction", &self.lpwsDialoutHoursRestriction).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_INTERFACE_2 {
    fn eq(&self, other: &Self) -> bool {
        self.wszInterfaceName == other.wszInterfaceName
            && self.hInterface == other.hInterface
            && self.fEnabled == other.fEnabled
            && self.dwIfType == other.dwIfType
            && self.dwConnectionState == other.dwConnectionState
            && self.fUnReachabilityReasons == other.fUnReachabilityReasons
            && self.dwLastError == other.dwLastError
            && self.dwfOptions == other.dwfOptions
            && self.szLocalPhoneNumber == other.szLocalPhoneNumber
            && self.szAlternates == other.szAlternates
            && self.ipaddr == other.ipaddr
            && self.ipaddrDns == other.ipaddrDns
            && self.ipaddrDnsAlt == other.ipaddrDnsAlt
            && self.ipaddrWins == other.ipaddrWins
            && self.ipaddrWinsAlt == other.ipaddrWinsAlt
            && self.dwfNetProtocols == other.dwfNetProtocols
            && self.szDeviceType == other.szDeviceType
            && self.szDeviceName == other.szDeviceName
            && self.szX25PadType == other.szX25PadType
            && self.szX25Address == other.szX25Address
            && self.szX25Facilities == other.szX25Facilities
            && self.szX25UserData == other.szX25UserData
            && self.dwChannels == other.dwChannels
            && self.dwSubEntries == other.dwSubEntries
            && self.dwDialMode == other.dwDialMode
            && self.dwDialExtraPercent == other.dwDialExtraPercent
            && self.dwDialExtraSampleSeconds == other.dwDialExtraSampleSeconds
            && self.dwHangUpExtraPercent == other.dwHangUpExtraPercent
            && self.dwHangUpExtraSampleSeconds == other.dwHangUpExtraSampleSeconds
            && self.dwIdleDisconnectSeconds == other.dwIdleDisconnectSeconds
            && self.dwType == other.dwType
            && self.dwEncryptionType == other.dwEncryptionType
            && self.dwCustomAuthKey == other.dwCustomAuthKey
            && self.dwCustomAuthDataSize == other.dwCustomAuthDataSize
            && self.lpbCustomAuthData == other.lpbCustomAuthData
            && self.guidId == other.guidId
            && self.dwVpnStrategy == other.dwVpnStrategy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_INTERFACE_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_INTERFACE_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_INTERFACE_2")
            .field("wszInterfaceName", &self.wszInterfaceName)
            .field("hInterface", &self.hInterface)
            .field("fEnabled", &self.fEnabled)
            .field("dwIfType", &self.dwIfType)
            .field("dwConnectionState", &self.dwConnectionState)
            .field("fUnReachabilityReasons", &self.fUnReachabilityReasons)
            .field("dwLastError", &self.dwLastError)
            .field("dwfOptions", &self.dwfOptions)
            .field("szLocalPhoneNumber", &self.szLocalPhoneNumber)
            .field("szAlternates", &self.szAlternates)
            .field("ipaddr", &self.ipaddr)
            .field("ipaddrDns", &self.ipaddrDns)
            .field("ipaddrDnsAlt", &self.ipaddrDnsAlt)
            .field("ipaddrWins", &self.ipaddrWins)
            .field("ipaddrWinsAlt", &self.ipaddrWinsAlt)
            .field("dwfNetProtocols", &self.dwfNetProtocols)
            .field("szDeviceType", &self.szDeviceType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szX25PadType", &self.szX25PadType)
            .field("szX25Address", &self.szX25Address)
            .field("szX25Facilities", &self.szX25Facilities)
            .field("szX25UserData", &self.szX25UserData)
            .field("dwChannels", &self.dwChannels)
            .field("dwSubEntries", &self.dwSubEntries)
            .field("dwDialMode", &self.dwDialMode)
            .field("dwDialExtraPercent", &self.dwDialExtraPercent)
            .field("dwDialExtraSampleSeconds", &self.dwDialExtraSampleSeconds)
            .field("dwHangUpExtraPercent", &self.dwHangUpExtraPercent)
            .field("dwHangUpExtraSampleSeconds", &self.dwHangUpExtraSampleSeconds)
            .field("dwIdleDisconnectSeconds", &self.dwIdleDisconnectSeconds)
            .field("dwType", &self.dwType)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("dwCustomAuthKey", &self.dwCustomAuthKey)
            .field("dwCustomAuthDataSize", &self.dwCustomAuthDataSize)
            .field("lpbCustomAuthData", &self.lpbCustomAuthData)
            .field("guidId", &self.guidId)
            .field("dwVpnStrategy", &self.dwVpnStrategy)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MPR_INTERFACE_DIAL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MPR_INTERFACE_DIAL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_INTERFACE_DIAL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_IPINIP_INTERFACE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wszFriendlyName == other.wszFriendlyName && self.Guid == other.Guid
    }
}
impl ::core::cmp::Eq for MPR_IPINIP_INTERFACE_0 {}
impl ::core::fmt::Debug for MPR_IPINIP_INTERFACE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IPINIP_INTERFACE_0").field("wszFriendlyName", &self.wszFriendlyName).field("Guid", &self.Guid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_SERVER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.fLanOnlyMode == other.fLanOnlyMode && self.dwUpTime == other.dwUpTime && self.dwTotalPorts == other.dwTotalPorts && self.dwPortsInUse == other.dwPortsInUse
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_SERVER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_SERVER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_0").field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).finish()
    }
}
impl ::core::default::Default for MPR_SERVER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_SERVER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPptpPorts == other.dwNumPptpPorts && self.dwPptpPortFlags == other.dwPptpPortFlags && self.dwNumL2tpPorts == other.dwNumL2tpPorts && self.dwL2tpPortFlags == other.dwL2tpPortFlags
    }
}
impl ::core::cmp::Eq for MPR_SERVER_1 {}
impl ::core::fmt::Debug for MPR_SERVER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_1").field("dwNumPptpPorts", &self.dwNumPptpPorts).field("dwPptpPortFlags", &self.dwPptpPortFlags).field("dwNumL2tpPorts", &self.dwNumL2tpPorts).field("dwL2tpPortFlags", &self.dwL2tpPortFlags).finish()
    }
}
impl ::core::default::Default for MPR_SERVER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MPR_SERVER_2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPptpPorts == other.dwNumPptpPorts && self.dwPptpPortFlags == other.dwPptpPortFlags && self.dwNumL2tpPorts == other.dwNumL2tpPorts && self.dwL2tpPortFlags == other.dwL2tpPortFlags && self.dwNumSstpPorts == other.dwNumSstpPorts && self.dwSstpPortFlags == other.dwSstpPortFlags
    }
}
impl ::core::cmp::Eq for MPR_SERVER_2 {}
impl ::core::fmt::Debug for MPR_SERVER_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_2").field("dwNumPptpPorts", &self.dwNumPptpPorts).field("dwPptpPortFlags", &self.dwPptpPortFlags).field("dwNumL2tpPorts", &self.dwNumL2tpPorts).field("dwL2tpPortFlags", &self.dwL2tpPortFlags).field("dwNumSstpPorts", &self.dwNumSstpPorts).field("dwSstpPortFlags", &self.dwSstpPortFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_EX0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_EX0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.fLanOnlyMode == other.fLanOnlyMode && self.dwUpTime == other.dwUpTime && self.dwTotalPorts == other.dwTotalPorts && self.dwPortsInUse == other.dwPortsInUse && self.Reserved == other.Reserved && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_EX0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_EX0").field("Header", &self.Header).field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).field("Reserved", &self.Reserved).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_EX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_EX1 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.fLanOnlyMode == other.fLanOnlyMode && self.dwUpTime == other.dwUpTime && self.dwTotalPorts == other.dwTotalPorts && self.dwPortsInUse == other.dwPortsInUse && self.Reserved == other.Reserved && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_EX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_EX1").field("Header", &self.Header).field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).field("Reserved", &self.Reserved).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_SET_CONFIG_EX0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_SET_CONFIG_EX0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.setConfigForProtocols == other.setConfigForProtocols && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_SET_CONFIG_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_SET_CONFIG_EX0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_SET_CONFIG_EX0").field("Header", &self.Header).field("setConfigForProtocols", &self.setConfigForProtocols).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_SET_CONFIG_EX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_SET_CONFIG_EX1 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.setConfigForProtocols == other.setConfigForProtocols && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_SET_CONFIG_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_SET_CONFIG_EX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_SET_CONFIG_EX1").field("Header", &self.Header).field("setConfigForProtocols", &self.setConfigForProtocols).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_TRANSPORT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransportId == other.dwTransportId && self.hTransport == other.hTransport && self.wszTransportName == other.wszTransportName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_TRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_TRANSPORT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_TRANSPORT_0").field("dwTransportId", &self.dwTransportId).field("hTransport", &self.hTransport).field("wszTransportName", &self.wszTransportName).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MPR_VPN_TRAFFIC_SELECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MPR_VPN_TRAFFIC_SELECTORS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MPR_VPN_TRAFFIC_SELECTORS {
    fn eq(&self, other: &Self) -> bool {
        self.numTsi == other.numTsi && self.numTsr == other.numTsr && self.tsI == other.tsI && self.tsR == other.tsR
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MPR_VPN_TRAFFIC_SELECTORS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for MPR_VPN_TRAFFIC_SELECTORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_VPN_TRAFFIC_SELECTORS").field("numTsi", &self.numTsi).field("numTsr", &self.numTsr).field("tsI", &self.tsI).field("tsR", &self.tsR).finish()
    }
}
impl ::core::default::Default for MPR_VPN_TS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MPR_VPN_TS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_VPN_TS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MPR_VS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MPR_VS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_VS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_ATCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress
    }
}
impl ::core::cmp::Eq for PPP_ATCP_INFO {}
impl ::core::fmt::Debug for PPP_ATCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_ATCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).finish()
    }
}
impl ::core::default::Default for PPP_CCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_CCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm && self.dwOptions == other.dwOptions && self.dwRemoteCompressionAlgorithm == other.dwRemoteCompressionAlgorithm && self.dwRemoteOptions == other.dwRemoteOptions
    }
}
impl ::core::cmp::Eq for PPP_CCP_INFO {}
impl ::core::fmt::Debug for PPP_CCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_CCP_INFO").field("dwError", &self.dwError).field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm).field("dwOptions", &self.dwOptions).field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm).field("dwRemoteOptions", &self.dwRemoteOptions).finish()
    }
}
impl ::core::default::Default for PPP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.nbf == other.nbf && self.ip == other.ip && self.ipx == other.ipx && self.at == other.at
    }
}
impl ::core::cmp::Eq for PPP_INFO {}
impl ::core::fmt::Debug for PPP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_INFO").field("nbf", &self.nbf).field("ip", &self.ip).field("ipx", &self.ipx).field("at", &self.at).finish()
    }
}
impl ::core::default::Default for PPP_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.nbf == other.nbf && self.ip == other.ip && self.ipx == other.ipx && self.at == other.at && self.ccp == other.ccp && self.lcp == other.lcp
    }
}
impl ::core::cmp::Eq for PPP_INFO_2 {}
impl ::core::fmt::Debug for PPP_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_INFO_2").field("nbf", &self.nbf).field("ip", &self.ip).field("ipx", &self.ipx).field("at", &self.at).field("ccp", &self.ccp).field("lcp", &self.lcp).finish()
    }
}
impl ::core::default::Default for PPP_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.nbf == other.nbf && self.ip == other.ip && self.ipv6 == other.ipv6 && self.ccp == other.ccp && self.lcp == other.lcp
    }
}
impl ::core::cmp::Eq for PPP_INFO_3 {}
impl ::core::fmt::Debug for PPP_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_INFO_3").field("nbf", &self.nbf).field("ip", &self.ip).field("ipv6", &self.ipv6).field("ccp", &self.ccp).field("lcp", &self.lcp).finish()
    }
}
impl ::core::default::Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_IPCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress && self.wszRemoteAddress == other.wszRemoteAddress
    }
}
impl ::core::cmp::Eq for PPP_IPCP_INFO {}
impl ::core::fmt::Debug for PPP_IPCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).field("wszRemoteAddress", &self.wszRemoteAddress).finish()
    }
}
impl ::core::default::Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_IPCP_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress && self.wszRemoteAddress == other.wszRemoteAddress && self.dwOptions == other.dwOptions && self.dwRemoteOptions == other.dwRemoteOptions
    }
}
impl ::core::cmp::Eq for PPP_IPCP_INFO2 {}
impl ::core::fmt::Debug for PPP_IPCP_INFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPCP_INFO2").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).field("wszRemoteAddress", &self.wszRemoteAddress).field("dwOptions", &self.dwOptions).field("dwRemoteOptions", &self.dwRemoteOptions).finish()
    }
}
impl ::core::default::Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_IPV6_CP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwSize == other.dwSize && self.dwError == other.dwError && self.bInterfaceIdentifier == other.bInterfaceIdentifier && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier && self.dwOptions == other.dwOptions && self.dwRemoteOptions == other.dwRemoteOptions && self.bPrefix == other.bPrefix && self.dwPrefixLength == other.dwPrefixLength
    }
}
impl ::core::cmp::Eq for PPP_IPV6_CP_INFO {}
impl ::core::fmt::Debug for PPP_IPV6_CP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPV6_CP_INFO").field("dwVersion", &self.dwVersion).field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("bInterfaceIdentifier", &self.bInterfaceIdentifier).field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier).field("dwOptions", &self.dwOptions).field("dwRemoteOptions", &self.dwRemoteOptions).field("bPrefix", &self.bPrefix).field("dwPrefixLength", &self.dwPrefixLength).finish()
    }
}
impl ::core::default::Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_IPXCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress
    }
}
impl ::core::cmp::Eq for PPP_IPXCP_INFO {}
impl ::core::fmt::Debug for PPP_IPXCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPXCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).finish()
    }
}
impl ::core::default::Default for PPP_LCP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PPP_LCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PPP_LCP").field(&self.0).finish()
    }
}
impl ::core::default::Default for PPP_LCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_LCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol && self.dwAuthenticationData == other.dwAuthenticationData && self.dwRemoteAuthenticationProtocol == other.dwRemoteAuthenticationProtocol && self.dwRemoteAuthenticationData == other.dwRemoteAuthenticationData && self.dwTerminateReason == other.dwTerminateReason && self.dwRemoteTerminateReason == other.dwRemoteTerminateReason && self.dwOptions == other.dwOptions && self.dwRemoteOptions == other.dwRemoteOptions && self.dwEapTypeId == other.dwEapTypeId && self.dwRemoteEapTypeId == other.dwRemoteEapTypeId
    }
}
impl ::core::cmp::Eq for PPP_LCP_INFO {}
impl ::core::fmt::Debug for PPP_LCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_LCP_INFO")
            .field("dwError", &self.dwError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwRemoteAuthenticationProtocol", &self.dwRemoteAuthenticationProtocol)
            .field("dwRemoteAuthenticationData", &self.dwRemoteAuthenticationData)
            .field("dwTerminateReason", &self.dwTerminateReason)
            .field("dwRemoteTerminateReason", &self.dwRemoteTerminateReason)
            .field("dwOptions", &self.dwOptions)
            .field("dwRemoteOptions", &self.dwRemoteOptions)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwRemoteEapTypeId", &self.dwRemoteEapTypeId)
            .finish()
    }
}
impl ::core::default::Default for PPP_LCP_INFO_AUTH_DATA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PPP_LCP_INFO_AUTH_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PPP_LCP_INFO_AUTH_DATA").field(&self.0).finish()
    }
}
impl ::core::default::Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_NBFCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszWksta == other.wszWksta
    }
}
impl ::core::cmp::Eq for PPP_NBFCP_INFO {}
impl ::core::fmt::Debug for PPP_NBFCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_NBFCP_INFO").field("dwError", &self.dwError).field("wszWksta", &self.wszWksta).finish()
    }
}
impl ::core::default::Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.dwIPv4Options == other.dwIPv4Options
            && self.dwIPv4RemoteOptions == other.dwIPv4RemoteOptions
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwLcpError == other.dwLcpError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwRemoteAuthenticationProtocol == other.dwRemoteAuthenticationProtocol
            && self.dwRemoteAuthenticationData == other.dwRemoteAuthenticationData
            && self.dwLcpTerminateReason == other.dwLcpTerminateReason
            && self.dwLcpRemoteTerminateReason == other.dwLcpRemoteTerminateReason
            && self.dwLcpOptions == other.dwLcpOptions
            && self.dwLcpRemoteOptions == other.dwLcpRemoteOptions
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwRemoteEapTypeId == other.dwRemoteEapTypeId
            && self.dwCcpError == other.dwCcpError
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwCcpOptions == other.dwCcpOptions
            && self.dwRemoteCompressionAlgorithm == other.dwRemoteCompressionAlgorithm
            && self.dwCcpRemoteOptions == other.dwCcpRemoteOptions
    }
}
impl ::core::cmp::Eq for PPP_PROJECTION_INFO {}
impl ::core::fmt::Debug for PPP_PROJECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_PROJECTION_INFO")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("dwIPv4Options", &self.dwIPv4Options)
            .field("dwIPv4RemoteOptions", &self.dwIPv4RemoteOptions)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwLcpError", &self.dwLcpError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwRemoteAuthenticationProtocol", &self.dwRemoteAuthenticationProtocol)
            .field("dwRemoteAuthenticationData", &self.dwRemoteAuthenticationData)
            .field("dwLcpTerminateReason", &self.dwLcpTerminateReason)
            .field("dwLcpRemoteTerminateReason", &self.dwLcpRemoteTerminateReason)
            .field("dwLcpOptions", &self.dwLcpOptions)
            .field("dwLcpRemoteOptions", &self.dwLcpRemoteOptions)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwRemoteEapTypeId", &self.dwRemoteEapTypeId)
            .field("dwCcpError", &self.dwCcpError)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwCcpOptions", &self.dwCcpOptions)
            .field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm)
            .field("dwCcpRemoteOptions", &self.dwCcpRemoteOptions)
            .finish()
    }
}
impl ::core::default::Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPP_PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.dwIPv4Options == other.dwIPv4Options
            && self.dwIPv4RemoteOptions == other.dwIPv4RemoteOptions
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwLcpError == other.dwLcpError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwRemoteAuthenticationProtocol == other.dwRemoteAuthenticationProtocol
            && self.dwRemoteAuthenticationData == other.dwRemoteAuthenticationData
            && self.dwLcpTerminateReason == other.dwLcpTerminateReason
            && self.dwLcpRemoteTerminateReason == other.dwLcpRemoteTerminateReason
            && self.dwLcpOptions == other.dwLcpOptions
            && self.dwLcpRemoteOptions == other.dwLcpRemoteOptions
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwEmbeddedEAPTypeId == other.dwEmbeddedEAPTypeId
            && self.dwRemoteEapTypeId == other.dwRemoteEapTypeId
            && self.dwCcpError == other.dwCcpError
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwCcpOptions == other.dwCcpOptions
            && self.dwRemoteCompressionAlgorithm == other.dwRemoteCompressionAlgorithm
            && self.dwCcpRemoteOptions == other.dwCcpRemoteOptions
    }
}
impl ::core::cmp::Eq for PPP_PROJECTION_INFO2 {}
impl ::core::fmt::Debug for PPP_PROJECTION_INFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_PROJECTION_INFO2")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("dwIPv4Options", &self.dwIPv4Options)
            .field("dwIPv4RemoteOptions", &self.dwIPv4RemoteOptions)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwLcpError", &self.dwLcpError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwRemoteAuthenticationProtocol", &self.dwRemoteAuthenticationProtocol)
            .field("dwRemoteAuthenticationData", &self.dwRemoteAuthenticationData)
            .field("dwLcpTerminateReason", &self.dwLcpTerminateReason)
            .field("dwLcpRemoteTerminateReason", &self.dwLcpRemoteTerminateReason)
            .field("dwLcpOptions", &self.dwLcpOptions)
            .field("dwLcpRemoteOptions", &self.dwLcpRemoteOptions)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwEmbeddedEAPTypeId", &self.dwEmbeddedEAPTypeId)
            .field("dwRemoteEapTypeId", &self.dwRemoteEapTypeId)
            .field("dwCcpError", &self.dwCcpError)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwCcpOptions", &self.dwCcpOptions)
            .field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm)
            .field("dwCcpRemoteOptions", &self.dwCcpRemoteOptions)
            .finish()
    }
}
impl ::core::default::Default for PPTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PPTP_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags
    }
}
impl ::core::cmp::Eq for PPTP_CONFIG_PARAMS {}
impl ::core::fmt::Debug for PPTP_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPTP_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
impl ::core::default::Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASADPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASAMBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASAMBA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szNetBiosError == other.szNetBiosError && self.bLana == other.bLana
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASAMBA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASAMBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAMBA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szNetBiosError", &self.szNetBiosError).field("bLana", &self.bLana).finish()
    }
}
impl ::core::default::Default for RASAMBW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASAMBW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szNetBiosError == other.szNetBiosError && self.bLana == other.bLana
    }
}
impl ::core::cmp::Eq for RASAMBW {}
impl ::core::fmt::Debug for RASAMBW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAMBW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szNetBiosError", &self.szNetBiosError).field("bLana", &self.bLana).finish()
    }
}
impl ::core::default::Default for RASAPIVERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASAPIVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASAPIVERSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASAUTODIALENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDialingLocation == other.dwDialingLocation && self.szEntry == other.szEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASAUTODIALENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASAUTODIALENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAUTODIALENTRYA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDialingLocation", &self.dwDialingLocation).field("szEntry", &self.szEntry).finish()
    }
}
impl ::core::default::Default for RASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASAUTODIALENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDialingLocation == other.dwDialingLocation && self.szEntry == other.szEntry
    }
}
impl ::core::cmp::Eq for RASAUTODIALENTRYW {}
impl ::core::fmt::Debug for RASAUTODIALENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAUTODIALENTRYW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDialingLocation", &self.dwDialingLocation).field("szEntry", &self.szEntry).finish()
    }
}
impl ::core::default::Default for RASCOMMSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASCOMMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bParity == other.bParity && self.bStop == other.bStop && self.bByteSize == other.bByteSize && self.bAlign == other.bAlign
    }
}
impl ::core::cmp::Eq for RASCOMMSETTINGS {}
impl ::core::fmt::Debug for RASCOMMSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCOMMSETTINGS").field("dwSize", &self.dwSize).field("bParity", &self.bParity).field("bStop", &self.bStop).field("bByteSize", &self.bByteSize).field("bAlign", &self.bAlign).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCONNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASCONNSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASCONNSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASCONNSTATE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASCONNSTATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASCONNSTATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASCONNSUBSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASCONNSUBSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASCONNSUBSTATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCONNW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCREDENTIALSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCREDENTIALSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCREDENTIALSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASCREDENTIALSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCREDENTIALSA").field("dwSize", &self.dwSize).field("dwMask", &self.dwMask).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
impl ::core::default::Default for RASCREDENTIALSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASCREDENTIALSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
impl ::core::cmp::Eq for RASCREDENTIALSW {}
impl ::core::fmt::Debug for RASCREDENTIALSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCREDENTIALSW").field("dwSize", &self.dwSize).field("dwMask", &self.dwMask).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
impl ::core::default::Default for RASCTRYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASCTRYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCountryID == other.dwCountryID && self.dwNextCountryID == other.dwNextCountryID && self.dwCountryCode == other.dwCountryCode && self.dwCountryNameOffset == other.dwCountryNameOffset
    }
}
impl ::core::cmp::Eq for RASCTRYINFO {}
impl ::core::fmt::Debug for RASCTRYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCTRYINFO").field("dwSize", &self.dwSize).field("dwCountryID", &self.dwCountryID).field("dwNextCountryID", &self.dwNextCountryID).field("dwCountryCode", &self.dwCountryCode).field("dwCountryNameOffset", &self.dwCountryNameOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCUSTOMSCRIPTEXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDEVINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASDEVINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASDEVINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASDEVINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASDEVINFOA").field("dwSize", &self.dwSize).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
impl ::core::default::Default for RASDEVINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASDEVINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName
    }
}
impl ::core::cmp::Eq for RASDEVINFOW {}
impl ::core::fmt::Debug for RASDEVINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASDEVINFOW").field("dwSize", &self.dwSize).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
impl ::core::default::Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDIALDLG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDIALEXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASEAPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASEAPUSERIDENTITYA {
    fn eq(&self, other: &Self) -> bool {
        self.szUserName == other.szUserName && self.dwSizeofEapInfo == other.dwSizeofEapInfo && self.pbEapInfo == other.pbEapInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASEAPUSERIDENTITYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASEAPUSERIDENTITYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASEAPUSERIDENTITYA").field("szUserName", &self.szUserName).field("dwSizeofEapInfo", &self.dwSizeofEapInfo).field("pbEapInfo", &self.pbEapInfo).finish()
    }
}
impl ::core::default::Default for RASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASEAPUSERIDENTITYW {
    fn eq(&self, other: &Self) -> bool {
        self.szUserName == other.szUserName && self.dwSizeofEapInfo == other.dwSizeofEapInfo && self.pbEapInfo == other.pbEapInfo
    }
}
impl ::core::cmp::Eq for RASEAPUSERIDENTITYW {}
impl ::core::fmt::Debug for RASEAPUSERIDENTITYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASEAPUSERIDENTITYW").field("szUserName", &self.szUserName).field("dwSizeofEapInfo", &self.dwSizeofEapInfo).field("pbEapInfo", &self.pbEapInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASENTRYNAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASENTRYNAMEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szEntryName == other.szEntryName && self.dwFlags == other.dwFlags && self.szPhonebookPath == other.szPhonebookPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASENTRYNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASENTRYNAMEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASENTRYNAMEA").field("dwSize", &self.dwSize).field("szEntryName", &self.szEntryName).field("dwFlags", &self.dwFlags).field("szPhonebookPath", &self.szPhonebookPath).finish()
    }
}
impl ::core::default::Default for RASENTRYNAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASENTRYNAMEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szEntryName == other.szEntryName && self.dwFlags == other.dwFlags && self.szPhonebookPath == other.szPhonebookPath
    }
}
impl ::core::cmp::Eq for RASENTRYNAMEW {}
impl ::core::fmt::Debug for RASENTRYNAMEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASENTRYNAMEW").field("dwSize", &self.dwSize).field("szEntryName", &self.szEntryName).field("dwFlags", &self.dwFlags).field("szPhonebookPath", &self.szPhonebookPath).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASENTRY_DIAL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASENTRY_DIAL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASENTRY_DIAL_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASIKEV_PROJECTION_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASIKEV_PROJECTION_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASIKEV_PROJECTION_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RASIPADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASIPADDR {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}
impl ::core::cmp::Eq for RASIPADDR {}
impl ::core::fmt::Debug for RASIPADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASIPADDR").field("a", &self.a).field("b", &self.b).field("c", &self.c).field("d", &self.d).finish()
    }
}
impl ::core::default::Default for RASIPXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASIPXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpxAddress == other.szIpxAddress
    }
}
impl ::core::cmp::Eq for RASIPXW {}
impl ::core::fmt::Debug for RASIPXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASIPXW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpxAddress", &self.szIpxAddress).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASNOUSERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASNOUSERA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwTimeoutMs == other.dwTimeoutMs && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASNOUSERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASNOUSERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASNOUSERA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwTimeoutMs", &self.dwTimeoutMs).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
impl ::core::default::Default for RASNOUSERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASNOUSERW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwTimeoutMs == other.dwTimeoutMs && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
impl ::core::cmp::Eq for RASNOUSERW {}
impl ::core::fmt::Debug for RASNOUSERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASNOUSERW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwTimeoutMs", &self.dwTimeoutMs).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPBDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPBDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASPPPCCP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASPPPCCP {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm && self.dwOptions == other.dwOptions && self.dwServerCompressionAlgorithm == other.dwServerCompressionAlgorithm && self.dwServerOptions == other.dwServerOptions
    }
}
impl ::core::cmp::Eq for RASPPPCCP {}
impl ::core::fmt::Debug for RASPPPCCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPCCP").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm).field("dwOptions", &self.dwOptions).field("dwServerCompressionAlgorithm", &self.dwServerCompressionAlgorithm).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPIPA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpAddress == other.szIpAddress && self.szServerIpAddress == other.szServerIpAddress && self.dwOptions == other.dwOptions && self.dwServerOptions == other.dwServerOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpAddress", &self.szIpAddress).field("szServerIpAddress", &self.szServerIpAddress).field("dwOptions", &self.dwOptions).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
impl ::core::default::Default for RASPPPIPV6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASPPPIPV6 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.bLocalInterfaceIdentifier == other.bLocalInterfaceIdentifier && self.bPeerInterfaceIdentifier == other.bPeerInterfaceIdentifier && self.bLocalCompressionProtocol == other.bLocalCompressionProtocol && self.bPeerCompressionProtocol == other.bPeerCompressionProtocol
    }
}
impl ::core::cmp::Eq for RASPPPIPV6 {}
impl ::core::fmt::Debug for RASPPPIPV6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPV6").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("bLocalInterfaceIdentifier", &self.bLocalInterfaceIdentifier).field("bPeerInterfaceIdentifier", &self.bPeerInterfaceIdentifier).field("bLocalCompressionProtocol", &self.bLocalCompressionProtocol).field("bPeerCompressionProtocol", &self.bPeerCompressionProtocol).finish()
    }
}
impl ::core::default::Default for RASPPPIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASPPPIPW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpAddress == other.szIpAddress && self.szServerIpAddress == other.szServerIpAddress && self.dwOptions == other.dwOptions && self.dwServerOptions == other.dwServerOptions
    }
}
impl ::core::cmp::Eq for RASPPPIPW {}
impl ::core::fmt::Debug for RASPPPIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpAddress", &self.szIpAddress).field("szServerIpAddress", &self.szServerIpAddress).field("dwOptions", &self.dwOptions).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPIPXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPIPXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpxAddress == other.szIpxAddress
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPIPXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPIPXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPXA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpxAddress", &self.szIpxAddress).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPLCPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPLCPA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fBundled == other.fBundled && self.dwError == other.dwError && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol && self.dwAuthenticationData == other.dwAuthenticationData && self.dwEapTypeId == other.dwEapTypeId && self.dwServerAuthenticationProtocol == other.dwServerAuthenticationProtocol && self.dwServerAuthenticationData == other.dwServerAuthenticationData && self.dwServerEapTypeId == other.dwServerEapTypeId && self.fMultilink == other.fMultilink && self.dwTerminateReason == other.dwTerminateReason && self.dwServerTerminateReason == other.dwServerTerminateReason && self.szReplyMessage == other.szReplyMessage && self.dwOptions == other.dwOptions && self.dwServerOptions == other.dwServerOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPLCPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPLCPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPLCPA")
            .field("dwSize", &self.dwSize)
            .field("fBundled", &self.fBundled)
            .field("dwError", &self.dwError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwServerAuthenticationProtocol", &self.dwServerAuthenticationProtocol)
            .field("dwServerAuthenticationData", &self.dwServerAuthenticationData)
            .field("dwServerEapTypeId", &self.dwServerEapTypeId)
            .field("fMultilink", &self.fMultilink)
            .field("dwTerminateReason", &self.dwTerminateReason)
            .field("dwServerTerminateReason", &self.dwServerTerminateReason)
            .field("szReplyMessage", &self.szReplyMessage)
            .field("dwOptions", &self.dwOptions)
            .field("dwServerOptions", &self.dwServerOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPLCPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPLCPW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fBundled == other.fBundled && self.dwError == other.dwError && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol && self.dwAuthenticationData == other.dwAuthenticationData && self.dwEapTypeId == other.dwEapTypeId && self.dwServerAuthenticationProtocol == other.dwServerAuthenticationProtocol && self.dwServerAuthenticationData == other.dwServerAuthenticationData && self.dwServerEapTypeId == other.dwServerEapTypeId && self.fMultilink == other.fMultilink && self.dwTerminateReason == other.dwTerminateReason && self.dwServerTerminateReason == other.dwServerTerminateReason && self.szReplyMessage == other.szReplyMessage && self.dwOptions == other.dwOptions && self.dwServerOptions == other.dwServerOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPLCPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPLCPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPLCPW")
            .field("dwSize", &self.dwSize)
            .field("fBundled", &self.fBundled)
            .field("dwError", &self.dwError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwServerAuthenticationProtocol", &self.dwServerAuthenticationProtocol)
            .field("dwServerAuthenticationData", &self.dwServerAuthenticationData)
            .field("dwServerEapTypeId", &self.dwServerEapTypeId)
            .field("fMultilink", &self.fMultilink)
            .field("dwTerminateReason", &self.dwTerminateReason)
            .field("dwServerTerminateReason", &self.dwServerTerminateReason)
            .field("szReplyMessage", &self.szReplyMessage)
            .field("dwOptions", &self.dwOptions)
            .field("dwServerOptions", &self.dwServerOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPNBFA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPNBFA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.dwNetBiosError == other.dwNetBiosError && self.szNetBiosError == other.szNetBiosError && self.szWorkstationName == other.szWorkstationName && self.bLana == other.bLana
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPNBFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPNBFA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPNBFA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwNetBiosError", &self.dwNetBiosError).field("szNetBiosError", &self.szNetBiosError).field("szWorkstationName", &self.szWorkstationName).field("bLana", &self.bLana).finish()
    }
}
impl ::core::default::Default for RASPPPNBFW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASPPPNBFW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.dwNetBiosError == other.dwNetBiosError && self.szNetBiosError == other.szNetBiosError && self.szWorkstationName == other.szWorkstationName && self.bLana == other.bLana
    }
}
impl ::core::cmp::Eq for RASPPPNBFW {}
impl ::core::fmt::Debug for RASPPPNBFW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPNBFW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwNetBiosError", &self.dwNetBiosError).field("szNetBiosError", &self.szNetBiosError).field("szWorkstationName", &self.szWorkstationName).field("bLana", &self.bLana).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASPPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA").field(&self.0).finish()
    }
}
impl ::core::default::Default for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for RASPROJECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASPROJECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPROJECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for RASPROJECTION_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RASPROJECTION_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPROJECTION_INFO_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASSUBENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASSUBENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwfFlags == other.dwfFlags && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName && self.szLocalPhoneNumber == other.szLocalPhoneNumber && self.dwAlternateOffset == other.dwAlternateOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASSUBENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASSUBENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASSUBENTRYA").field("dwSize", &self.dwSize).field("dwfFlags", &self.dwfFlags).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("dwAlternateOffset", &self.dwAlternateOffset).finish()
    }
}
impl ::core::default::Default for RASSUBENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASSUBENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwfFlags == other.dwfFlags && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName && self.szLocalPhoneNumber == other.szLocalPhoneNumber && self.dwAlternateOffset == other.dwAlternateOffset
    }
}
impl ::core::cmp::Eq for RASSUBENTRYW {}
impl ::core::fmt::Debug for RASSUBENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASSUBENTRYW").field("dwSize", &self.dwSize).field("dwfFlags", &self.dwfFlags).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("dwAlternateOffset", &self.dwAlternateOffset).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASUPDATECONN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hConnection == other.hConnection && self.hInterface == other.hInterface && self.dwConnectDuration == other.dwConnectDuration && self.dwInterfaceType == other.dwInterfaceType && self.dwConnectionFlags == other.dwConnectionFlags && self.wszInterfaceName == other.wszInterfaceName && self.wszUserName == other.wszUserName && self.wszLogonDomain == other.wszLogonDomain && self.wszRemoteComputer == other.wszRemoteComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_0").field("hConnection", &self.hConnection).field("hInterface", &self.hInterface).field("dwConnectDuration", &self.dwConnectDuration).field("dwInterfaceType", &self.dwInterfaceType).field("dwConnectionFlags", &self.dwConnectionFlags).field("wszInterfaceName", &self.wszInterfaceName).field("wszUserName", &self.wszUserName).field("wszLogonDomain", &self.wszLogonDomain).field("wszRemoteComputer", &self.wszRemoteComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hConnection == other.hConnection && self.hInterface == other.hInterface && self.PppInfo == other.PppInfo && self.dwBytesXmited == other.dwBytesXmited && self.dwBytesRcved == other.dwBytesRcved && self.dwFramesXmited == other.dwFramesXmited && self.dwFramesRcved == other.dwFramesRcved && self.dwCrcErr == other.dwCrcErr && self.dwTimeoutErr == other.dwTimeoutErr && self.dwAlignmentErr == other.dwAlignmentErr && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr && self.dwFramingErr == other.dwFramingErr && self.dwBufferOverrunErr == other.dwBufferOverrunErr && self.dwCompressionRatioIn == other.dwCompressionRatioIn && self.dwCompressionRatioOut == other.dwCompressionRatioOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_1")
            .field("hConnection", &self.hConnection)
            .field("hInterface", &self.hInterface)
            .field("PppInfo", &self.PppInfo)
            .field("dwBytesXmited", &self.dwBytesXmited)
            .field("dwBytesRcved", &self.dwBytesRcved)
            .field("dwFramesXmited", &self.dwFramesXmited)
            .field("dwFramesRcved", &self.dwFramesRcved)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hConnection == other.hConnection && self.wszUserName == other.wszUserName && self.dwInterfaceType == other.dwInterfaceType && self.guid == other.guid && self.PppInfo2 == other.PppInfo2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_2").field("hConnection", &self.hConnection).field("wszUserName", &self.wszUserName).field("dwInterfaceType", &self.dwInterfaceType).field("guid", &self.guid).field("PppInfo2", &self.PppInfo2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwSize == other.dwSize && self.hConnection == other.hConnection && self.wszUserName == other.wszUserName && self.dwInterfaceType == other.dwInterfaceType && self.guid == other.guid && self.PppInfo3 == other.PppInfo3 && self.rasQuarState == other.rasQuarState && self.timer == other.timer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_3").field("dwVersion", &self.dwVersion).field("dwSize", &self.dwSize).field("hConnection", &self.hConnection).field("wszUserName", &self.wszUserName).field("dwInterfaceType", &self.dwInterfaceType).field("guid", &self.guid).field("PppInfo3", &self.PppInfo3).field("rasQuarState", &self.rasQuarState).field("timer", &self.timer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RAS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for RAS_HARDWARE_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAS_HARDWARE_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_HARDWARE_CONDITION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_PORT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hPort == other.hPort && self.hConnection == other.hConnection && self.dwPortCondition == other.dwPortCondition && self.dwTotalNumberOfCalls == other.dwTotalNumberOfCalls && self.dwConnectDuration == other.dwConnectDuration && self.wszPortName == other.wszPortName && self.wszMediaName == other.wszMediaName && self.wszDeviceName == other.wszDeviceName && self.wszDeviceType == other.wszDeviceType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_PORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_PORT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_PORT_0").field("hPort", &self.hPort).field("hConnection", &self.hConnection).field("dwPortCondition", &self.dwPortCondition).field("dwTotalNumberOfCalls", &self.dwTotalNumberOfCalls).field("dwConnectDuration", &self.dwConnectDuration).field("wszPortName", &self.wszPortName).field("wszMediaName", &self.wszMediaName).field("wszDeviceName", &self.wszDeviceName).field("wszDeviceType", &self.wszDeviceType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_PORT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_PORT_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hPort == other.hPort && self.hConnection == other.hConnection && self.dwHardwareCondition == other.dwHardwareCondition && self.dwLineSpeed == other.dwLineSpeed && self.dwBytesXmited == other.dwBytesXmited && self.dwBytesRcved == other.dwBytesRcved && self.dwFramesXmited == other.dwFramesXmited && self.dwFramesRcved == other.dwFramesRcved && self.dwCrcErr == other.dwCrcErr && self.dwTimeoutErr == other.dwTimeoutErr && self.dwAlignmentErr == other.dwAlignmentErr && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr && self.dwFramingErr == other.dwFramingErr && self.dwBufferOverrunErr == other.dwBufferOverrunErr && self.dwCompressionRatioIn == other.dwCompressionRatioIn && self.dwCompressionRatioOut == other.dwCompressionRatioOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_PORT_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_PORT_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_PORT_1")
            .field("hPort", &self.hPort)
            .field("hConnection", &self.hConnection)
            .field("dwHardwareCondition", &self.dwHardwareCondition)
            .field("dwLineSpeed", &self.dwLineSpeed)
            .field("dwBytesXmited", &self.dwBytesXmited)
            .field("dwBytesRcved", &self.dwBytesRcved)
            .field("dwFramesXmited", &self.dwFramesXmited)
            .field("dwFramesRcved", &self.dwFramesRcved)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_PORT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_PORT_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hPort == other.hPort
            && self.hConnection == other.hConnection
            && self.dwConn_State == other.dwConn_State
            && self.wszPortName == other.wszPortName
            && self.wszMediaName == other.wszMediaName
            && self.wszDeviceName == other.wszDeviceName
            && self.wszDeviceType == other.wszDeviceType
            && self.dwHardwareCondition == other.dwHardwareCondition
            && self.dwLineSpeed == other.dwLineSpeed
            && self.dwCrcErr == other.dwCrcErr
            && self.dwSerialOverRunErrs == other.dwSerialOverRunErrs
            && self.dwTimeoutErr == other.dwTimeoutErr
            && self.dwAlignmentErr == other.dwAlignmentErr
            && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr
            && self.dwFramingErr == other.dwFramingErr
            && self.dwBufferOverrunErr == other.dwBufferOverrunErr
            && self.dwCompressionRatioIn == other.dwCompressionRatioIn
            && self.dwCompressionRatioOut == other.dwCompressionRatioOut
            && self.dwTotalErrors == other.dwTotalErrors
            && self.ullBytesXmited == other.ullBytesXmited
            && self.ullBytesRcved == other.ullBytesRcved
            && self.ullFramesXmited == other.ullFramesXmited
            && self.ullFramesRcved == other.ullFramesRcved
            && self.ullBytesTxUncompressed == other.ullBytesTxUncompressed
            && self.ullBytesTxCompressed == other.ullBytesTxCompressed
            && self.ullBytesRcvUncompressed == other.ullBytesRcvUncompressed
            && self.ullBytesRcvCompressed == other.ullBytesRcvCompressed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_PORT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_PORT_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_PORT_2")
            .field("hPort", &self.hPort)
            .field("hConnection", &self.hConnection)
            .field("dwConn_State", &self.dwConn_State)
            .field("wszPortName", &self.wszPortName)
            .field("wszMediaName", &self.wszMediaName)
            .field("wszDeviceName", &self.wszDeviceName)
            .field("wszDeviceType", &self.wszDeviceType)
            .field("dwHardwareCondition", &self.dwHardwareCondition)
            .field("dwLineSpeed", &self.dwLineSpeed)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwSerialOverRunErrs", &self.dwSerialOverRunErrs)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .field("dwTotalErrors", &self.dwTotalErrors)
            .field("ullBytesXmited", &self.ullBytesXmited)
            .field("ullBytesRcved", &self.ullBytesRcved)
            .field("ullFramesXmited", &self.ullFramesXmited)
            .field("ullFramesRcved", &self.ullFramesRcved)
            .field("ullBytesTxUncompressed", &self.ullBytesTxUncompressed)
            .field("ullBytesTxCompressed", &self.ullBytesTxCompressed)
            .field("ullBytesRcvUncompressed", &self.ullBytesRcvUncompressed)
            .field("ullBytesRcvCompressed", &self.ullBytesRcvCompressed)
            .finish()
    }
}
impl ::core::default::Default for RAS_PORT_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAS_PORT_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_PORT_CONDITION").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RAS_QUARANTINE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAS_QUARANTINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_QUARANTINE_STATE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_SECURITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_SECURITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastError == other.LastError && self.BytesReceived == other.BytesReceived && self.DeviceName == other.DeviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_SECURITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_SECURITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_SECURITY_INFO").field("LastError", &self.LastError).field("BytesReceived", &self.BytesReceived).field("DeviceName", &self.DeviceName).finish()
    }
}
impl ::core::default::Default for RAS_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RAS_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwBytesXmited == other.dwBytesXmited && self.dwBytesRcved == other.dwBytesRcved && self.dwFramesXmited == other.dwFramesXmited && self.dwFramesRcved == other.dwFramesRcved && self.dwCrcErr == other.dwCrcErr && self.dwTimeoutErr == other.dwTimeoutErr && self.dwAlignmentErr == other.dwAlignmentErr && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr && self.dwFramingErr == other.dwFramingErr && self.dwBufferOverrunErr == other.dwBufferOverrunErr && self.dwCompressionRatioIn == other.dwCompressionRatioIn && self.dwCompressionRatioOut == other.dwCompressionRatioOut && self.dwBps == other.dwBps && self.dwConnectDuration == other.dwConnectDuration
    }
}
impl ::core::cmp::Eq for RAS_STATS {}
impl ::core::fmt::Debug for RAS_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_STATS")
            .field("dwSize", &self.dwSize)
            .field("dwBytesXmited", &self.dwBytesXmited)
            .field("dwBytesRcved", &self.dwBytesRcved)
            .field("dwFramesXmited", &self.dwFramesXmited)
            .field("dwFramesRcved", &self.dwFramesRcved)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .field("dwBps", &self.dwBps)
            .field("dwConnectDuration", &self.dwConnectDuration)
            .finish()
    }
}
impl ::core::default::Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RAS_UPDATE_CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwIfIndex == other.dwIfIndex && self.wszLocalEndpointAddress == other.wszLocalEndpointAddress && self.wszRemoteEndpointAddress == other.wszRemoteEndpointAddress
    }
}
impl ::core::cmp::Eq for RAS_UPDATE_CONNECTION {}
impl ::core::fmt::Debug for RAS_UPDATE_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_UPDATE_CONNECTION").field("Header", &self.Header).field("dwIfIndex", &self.dwIfIndex).field("wszLocalEndpointAddress", &self.wszLocalEndpointAddress).field("wszRemoteEndpointAddress", &self.wszRemoteEndpointAddress).finish()
    }
}
impl ::core::default::Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RAS_USER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.bfPrivilege == other.bfPrivilege && self.wszPhoneNumber == other.wszPhoneNumber
    }
}
impl ::core::cmp::Eq for RAS_USER_0 {}
impl ::core::fmt::Debug for RAS_USER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_USER_0").field("bfPrivilege", &self.bfPrivilege).field("wszPhoneNumber", &self.wszPhoneNumber).finish()
    }
}
impl ::core::default::Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RAS_USER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.bfPrivilege == other.bfPrivilege && self.wszPhoneNumber == other.wszPhoneNumber && self.bfPrivilege2 == other.bfPrivilege2
    }
}
impl ::core::cmp::Eq for RAS_USER_1 {}
impl ::core::fmt::Debug for RAS_USER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_USER_1").field("bfPrivilege", &self.bfPrivilege).field("wszPhoneNumber", &self.wszPhoneNumber).field("bfPrivilege2", &self.bfPrivilege2).finish()
    }
}
impl ::core::default::Default for ROUTER_CONNECTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ROUTER_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROUTER_CONNECTION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIntegrityMethod == other.dwIntegrityMethod && self.dwEncryptionMethod == other.dwEncryptionMethod && self.dwCipherTransformConstant == other.dwCipherTransformConstant && self.dwAuthTransformConstant == other.dwAuthTransformConstant && self.dwPfsGroup == other.dwPfsGroup && self.dwDhGroup == other.dwDhGroup
    }
}
impl ::core::cmp::Eq for ROUTER_CUSTOM_IKEv2_POLICY0 {}
impl ::core::fmt::Debug for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_CUSTOM_IKEv2_POLICY0").field("dwIntegrityMethod", &self.dwIntegrityMethod).field("dwEncryptionMethod", &self.dwEncryptionMethod).field("dwCipherTransformConstant", &self.dwCipherTransformConstant).field("dwAuthTransformConstant", &self.dwAuthTransformConstant).field("dwPfsGroup", &self.dwPfsGroup).field("dwDhGroup", &self.dwDhGroup).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSize == other.dwSaDataSize && self.certificateName == other.certificateName && self.customPolicy == other.customPolicy
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG0").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSize == other.dwSaDataSize && self.certificateName == other.certificateName && self.customPolicy == other.customPolicy && self.certificateHash == other.certificateHash
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG1").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).field("certificateHash", &self.certificateHash).finish()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSize == other.dwSaDataSize && self.certificateName == other.certificateName && self.customPolicy == other.customPolicy && self.certificateHash == other.certificateHash && self.dwMmSaLifeTime == other.dwMmSaLifeTime && self.vpnTrafficSelectors == other.vpnTrafficSelectors
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG2").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).field("certificateHash", &self.certificateHash).field("dwMmSaLifeTime", &self.dwMmSaLifeTime).field("vpnTrafficSelectors", &self.vpnTrafficSelectors).finish()
    }
}
impl ::core::default::Default for ROUTER_INTERFACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ROUTER_INTERFACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROUTER_INTERFACE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ROUTING_PROTOCOL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTM_DEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTM_DEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DestHandle == other.DestHandle && self.DestAddress == other.DestAddress && self.LastChanged == other.LastChanged && self.BelongsToViews == other.BelongsToViews && self.NumberOfViews == other.NumberOfViews && self.ViewInfo == other.ViewInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTM_DEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTM_DEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_DEST_INFO").field("DestHandle", &self.DestHandle).field("DestAddress", &self.DestAddress).field("LastChanged", &self.LastChanged).field("BelongsToViews", &self.BelongsToViews).field("NumberOfViews", &self.NumberOfViews).field("ViewInfo", &self.ViewInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTM_DEST_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTM_DEST_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ViewId == other.ViewId && self.NumRoutes == other.NumRoutes && self.Route == other.Route && self.Owner == other.Owner && self.DestFlags == other.DestFlags && self.HoldRoute == other.HoldRoute
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTM_DEST_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTM_DEST_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_DEST_INFO_0").field("ViewId", &self.ViewId).field("NumRoutes", &self.NumRoutes).field("Route", &self.Route).field("Owner", &self.Owner).field("DestFlags", &self.DestFlags).field("HoldRoute", &self.HoldRoute).finish()
    }
}
impl ::core::default::Default for RTM_ENTITY_EXPORT_METHODS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RTM_ENTITY_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RTM_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RTM_ENTITY_METHOD_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_ENTITY_METHOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MethodType == other.MethodType && self.InputSize == other.InputSize && self.InputData == other.InputData
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_METHOD_INPUT {}
impl ::core::fmt::Debug for RTM_ENTITY_METHOD_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ENTITY_METHOD_INPUT").field("MethodType", &self.MethodType).field("InputSize", &self.InputSize).field("InputData", &self.InputData).finish()
    }
}
impl ::core::default::Default for RTM_ENTITY_METHOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_ENTITY_METHOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MethodType == other.MethodType && self.MethodStatus == other.MethodStatus && self.OutputSize == other.OutputSize && self.OutputData == other.OutputData
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_METHOD_OUTPUT {}
impl ::core::fmt::Debug for RTM_ENTITY_METHOD_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ENTITY_METHOD_OUTPUT").field("MethodType", &self.MethodType).field("MethodStatus", &self.MethodStatus).field("OutputSize", &self.OutputSize).field("OutputData", &self.OutputData).finish()
    }
}
impl ::core::default::Default for RTM_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTM_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTM_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RTM_NET_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_NET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.NumBits == other.NumBits && self.AddrBits == other.AddrBits
    }
}
impl ::core::cmp::Eq for RTM_NET_ADDRESS {}
impl ::core::fmt::Debug for RTM_NET_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_NET_ADDRESS").field("AddressFamily", &self.AddressFamily).field("NumBits", &self.NumBits).field("AddrBits", &self.AddrBits).finish()
    }
}
impl ::core::default::Default for RTM_NEXTHOP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_NEXTHOP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextHopAddress == other.NextHopAddress && self.NextHopOwner == other.NextHopOwner && self.InterfaceIndex == other.InterfaceIndex && self.State == other.State && self.Flags == other.Flags && self.EntitySpecificInfo == other.EntitySpecificInfo && self.RemoteNextHop == other.RemoteNextHop
    }
}
impl ::core::cmp::Eq for RTM_NEXTHOP_INFO {}
impl ::core::fmt::Debug for RTM_NEXTHOP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_NEXTHOP_INFO").field("NextHopAddress", &self.NextHopAddress).field("NextHopOwner", &self.NextHopOwner).field("InterfaceIndex", &self.InterfaceIndex).field("State", &self.State).field("Flags", &self.Flags).field("EntitySpecificInfo", &self.EntitySpecificInfo).field("RemoteNextHop", &self.RemoteNextHop).finish()
    }
}
impl ::core::default::Default for RTM_NEXTHOP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_NEXTHOP_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumNextHops == other.NumNextHops && self.NextHops == other.NextHops
    }
}
impl ::core::cmp::Eq for RTM_NEXTHOP_LIST {}
impl ::core::fmt::Debug for RTM_NEXTHOP_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_NEXTHOP_LIST").field("NumNextHops", &self.NumNextHops).field("NextHops", &self.NextHops).finish()
    }
}
impl ::core::default::Default for RTM_PREF_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_PREF_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Metric == other.Metric && self.Preference == other.Preference
    }
}
impl ::core::cmp::Eq for RTM_PREF_INFO {}
impl ::core::fmt::Debug for RTM_PREF_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_PREF_INFO").field("Metric", &self.Metric).field("Preference", &self.Preference).finish()
    }
}
impl ::core::default::Default for RTM_REGN_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_REGN_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.MaxNextHopsInRoute == other.MaxNextHopsInRoute && self.MaxHandlesInEnum == other.MaxHandlesInEnum && self.ViewsSupported == other.ViewsSupported && self.NumberOfViews == other.NumberOfViews
    }
}
impl ::core::cmp::Eq for RTM_REGN_PROFILE {}
impl ::core::fmt::Debug for RTM_REGN_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_REGN_PROFILE").field("MaxNextHopsInRoute", &self.MaxNextHopsInRoute).field("MaxHandlesInEnum", &self.MaxHandlesInEnum).field("ViewsSupported", &self.ViewsSupported).field("NumberOfViews", &self.NumberOfViews).finish()
    }
}
impl ::core::default::Default for RTM_ROUTE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTM_ROUTE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DestHandle == other.DestHandle && self.RouteOwner == other.RouteOwner && self.Neighbour == other.Neighbour && self.State == other.State && self.Flags1 == other.Flags1 && self.Flags == other.Flags && self.PrefInfo == other.PrefInfo && self.BelongsToViews == other.BelongsToViews && self.EntitySpecificInfo == other.EntitySpecificInfo && self.NextHopsList == other.NextHopsList
    }
}
impl ::core::cmp::Eq for RTM_ROUTE_INFO {}
impl ::core::fmt::Debug for RTM_ROUTE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ROUTE_INFO").field("DestHandle", &self.DestHandle).field("RouteOwner", &self.RouteOwner).field("Neighbour", &self.Neighbour).field("State", &self.State).field("Flags1", &self.Flags1).field("Flags", &self.Flags).field("PrefInfo", &self.PrefInfo).field("BelongsToViews", &self.BelongsToViews).field("EntitySpecificInfo", &self.EntitySpecificInfo).field("NextHopsList", &self.NextHopsList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwMsgId == other.dwMsgId && self.hPort == other.hPort && self.dwError == other.dwError && self.UserName == other.UserName && self.Domain == other.Domain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_MESSAGE").field("dwMsgId", &self.dwMsgId).field("hPort", &self.hPort).field("dwError", &self.dwError).field("UserName", &self.UserName).field("Domain", &self.Domain).finish()
    }
}
impl ::core::default::Default for SECURITY_MESSAGE_MSG_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SECURITY_MESSAGE_MSG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_MESSAGE_MSG_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for SOURCE_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SOURCE_GROUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwSourceAddr == other.dwSourceAddr && self.dwSourceMask == other.dwSourceMask && self.dwGroupAddr == other.dwGroupAddr && self.dwGroupMask == other.dwGroupMask
    }
}
impl ::core::cmp::Eq for SOURCE_GROUP_ENTRY {}
impl ::core::fmt::Debug for SOURCE_GROUP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOURCE_GROUP_ENTRY").field("dwSourceAddr", &self.dwSourceAddr).field("dwSourceMask", &self.dwSourceMask).field("dwGroupAddr", &self.dwGroupAddr).field("dwGroupMask", &self.dwGroupMask).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for SSTP_CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for SSTP_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.isDefault == other.isDefault && self.certBlob == other.certBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for SSTP_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for SSTP_CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSTP_CERT_INFO").field("isDefault", &self.isDefault).field("certBlob", &self.certBlob).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for SSTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for SSTP_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags && self.isUseHttps == other.isUseHttps && self.certAlgorithm == other.certAlgorithm && self.sstpCertDetails == other.sstpCertDetails
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for SSTP_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for SSTP_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSTP_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("isUseHttps", &self.isUseHttps).field("certAlgorithm", &self.certAlgorithm).field("sstpCertDetails", &self.sstpCertDetails).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
