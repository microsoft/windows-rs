impl ::core::default::Default for BSSID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BSSID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.PMKID == other.PMKID
    }
}
impl ::core::cmp::Eq for BSSID_INFO {}
impl ::core::fmt::Debug for BSSID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BSSID_INFO").field("BSSID", &self.BSSID).field("PMKID", &self.PMKID).finish()
    }
}
impl ::core::default::Default for GEN_GET_NETCARD_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GEN_GET_NETCARD_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.ReadTime == other.ReadTime
    }
}
impl ::core::cmp::Eq for GEN_GET_NETCARD_TIME {}
impl ::core::fmt::Debug for GEN_GET_NETCARD_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GEN_GET_NETCARD_TIME").field("ReadTime", &self.ReadTime).finish()
    }
}
impl ::core::default::Default for GEN_GET_TIME_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GEN_GET_TIME_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.ClockPrecision == other.ClockPrecision
    }
}
impl ::core::cmp::Eq for GEN_GET_TIME_CAPS {}
impl ::core::fmt::Debug for GEN_GET_TIME_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GEN_GET_TIME_CAPS").field("Flags", &self.Flags).field("ClockPrecision", &self.ClockPrecision).finish()
    }
}
impl ::core::default::Default for IF_ADMINISTRATIVE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IF_ADMINISTRATIVE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IF_ADMINISTRATIVE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IF_COUNTED_STRING_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IF_COUNTED_STRING_LH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.String == other.String
    }
}
impl ::core::cmp::Eq for IF_COUNTED_STRING_LH {}
impl ::core::fmt::Debug for IF_COUNTED_STRING_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IF_COUNTED_STRING_LH").field("Length", &self.Length).field("String", &self.String).finish()
    }
}
impl ::core::default::Default for IF_OPER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IF_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IF_OPER_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IF_PHYSICAL_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IF_PHYSICAL_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for IF_PHYSICAL_ADDRESS_LH {}
impl ::core::fmt::Debug for IF_PHYSICAL_ADDRESS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IF_PHYSICAL_ADDRESS_LH").field("Length", &self.Length).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_AI_REQFI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_AI_REQFI {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.ListenInterval == other.ListenInterval && self.CurrentAPAddress == other.CurrentAPAddress
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AI_REQFI {}
impl ::core::fmt::Debug for NDIS_802_11_AI_REQFI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AI_REQFI").field("Capabilities", &self.Capabilities).field("ListenInterval", &self.ListenInterval).field("CurrentAPAddress", &self.CurrentAPAddress).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_AI_RESFI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_AI_RESFI {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.StatusCode == other.StatusCode && self.AssociationId == other.AssociationId
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AI_RESFI {}
impl ::core::fmt::Debug for NDIS_802_11_AI_RESFI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AI_RESFI").field("Capabilities", &self.Capabilities).field("StatusCode", &self.StatusCode).field("AssociationId", &self.AssociationId).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AvailableRequestFixedIEs == other.AvailableRequestFixedIEs && self.RequestFixedIEs == other.RequestFixedIEs && self.RequestIELength == other.RequestIELength && self.OffsetRequestIEs == other.OffsetRequestIEs && self.AvailableResponseFixedIEs == other.AvailableResponseFixedIEs && self.ResponseFixedIEs == other.ResponseFixedIEs && self.ResponseIELength == other.ResponseIELength && self.OffsetResponseIEs == other.OffsetResponseIEs
    }
}
impl ::core::cmp::Eq for NDIS_802_11_ASSOCIATION_INFORMATION {}
impl ::core::fmt::Debug for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_ASSOCIATION_INFORMATION")
            .field("Length", &self.Length)
            .field("AvailableRequestFixedIEs", &self.AvailableRequestFixedIEs)
            .field("RequestFixedIEs", &self.RequestFixedIEs)
            .field("RequestIELength", &self.RequestIELength)
            .field("OffsetRequestIEs", &self.OffsetRequestIEs)
            .field("AvailableResponseFixedIEs", &self.AvailableResponseFixedIEs)
            .field("ResponseFixedIEs", &self.ResponseFixedIEs)
            .field("ResponseIELength", &self.ResponseIELength)
            .field("OffsetResponseIEs", &self.OffsetResponseIEs)
            .finish()
    }
}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn eq(&self, other: &Self) -> bool {
        self.AuthModeSupported == other.AuthModeSupported && self.EncryptStatusSupported == other.EncryptStatusSupported
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AUTHENTICATION_ENCRYPTION {}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AUTHENTICATION_ENCRYPTION").field("AuthModeSupported", &self.AuthModeSupported).field("EncryptStatusSupported", &self.EncryptStatusSupported).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.Request == other.Request
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AUTHENTICATION_EVENT {}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AUTHENTICATION_EVENT").field("Status", &self.Status).field("Request", &self.Request).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_AUTHENTICATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Bssid == other.Bssid && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AUTHENTICATION_REQUEST {}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AUTHENTICATION_REQUEST").field("Length", &self.Length).field("Bssid", &self.Bssid).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_BSSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_BSSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Bssid == other.Bssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_BSSID_LIST {}
impl ::core::fmt::Debug for NDIS_802_11_BSSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_BSSID_LIST").field("NumberOfItems", &self.NumberOfItems).field("Bssid", &self.Bssid).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_BSSID_LIST_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_BSSID_LIST_EX {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Bssid == other.Bssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_BSSID_LIST_EX {}
impl ::core::fmt::Debug for NDIS_802_11_BSSID_LIST_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_BSSID_LIST_EX").field("NumberOfItems", &self.NumberOfItems).field("Bssid", &self.Bssid).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Version == other.Version && self.NoOfPMKIDs == other.NoOfPMKIDs && self.NoOfAuthEncryptPairsSupported == other.NoOfAuthEncryptPairsSupported && self.AuthenticationEncryptionSupported == other.AuthenticationEncryptionSupported
    }
}
impl ::core::cmp::Eq for NDIS_802_11_CAPABILITY {}
impl ::core::fmt::Debug for NDIS_802_11_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_CAPABILITY").field("Length", &self.Length).field("Version", &self.Version).field("NoOfPMKIDs", &self.NoOfPMKIDs).field("NoOfAuthEncryptPairsSupported", &self.NoOfAuthEncryptPairsSupported).field("AuthenticationEncryptionSupported", &self.AuthenticationEncryptionSupported).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.BeaconPeriod == other.BeaconPeriod && self.ATIMWindow == other.ATIMWindow && self.DSConfig == other.DSConfig && self.FHConfig == other.FHConfig
    }
}
impl ::core::cmp::Eq for NDIS_802_11_CONFIGURATION {}
impl ::core::fmt::Debug for NDIS_802_11_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_CONFIGURATION").field("Length", &self.Length).field("BeaconPeriod", &self.BeaconPeriod).field("ATIMWindow", &self.ATIMWindow).field("DSConfig", &self.DSConfig).field("FHConfig", &self.FHConfig).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_CONFIGURATION_FH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_CONFIGURATION_FH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.HopPattern == other.HopPattern && self.HopSet == other.HopSet && self.DwellTime == other.DwellTime
    }
}
impl ::core::cmp::Eq for NDIS_802_11_CONFIGURATION_FH {}
impl ::core::fmt::Debug for NDIS_802_11_CONFIGURATION_FH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_CONFIGURATION_FH").field("Length", &self.Length).field("HopPattern", &self.HopPattern).field("HopSet", &self.HopSet).field("DwellTime", &self.DwellTime).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_FIXED_IEs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_FIXED_IEs {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.BeaconInterval == other.BeaconInterval && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for NDIS_802_11_FIXED_IEs {}
impl ::core::fmt::Debug for NDIS_802_11_FIXED_IEs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_FIXED_IEs").field("Timestamp", &self.Timestamp).field("BeaconInterval", &self.BeaconInterval).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.KeyLength == other.KeyLength && self.BSSID == other.BSSID && self.KeyRSC == other.KeyRSC && self.KeyMaterial == other.KeyMaterial
    }
}
impl ::core::cmp::Eq for NDIS_802_11_KEY {}
impl ::core::fmt::Debug for NDIS_802_11_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_KEY").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("KeyLength", &self.KeyLength).field("BSSID", &self.BSSID).field("KeyRSC", &self.KeyRSC).field("KeyMaterial", &self.KeyMaterial).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_MEDIA_STREAM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_MEDIA_STREAM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_MEDIA_STREAM_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_NETWORK_INFRASTRUCTURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_NETWORK_INFRASTRUCTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_NETWORK_INFRASTRUCTURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_NETWORK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_NETWORK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_NETWORK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_NETWORK_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_NETWORK_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.NetworkType == other.NetworkType
    }
}
impl ::core::cmp::Eq for NDIS_802_11_NETWORK_TYPE_LIST {}
impl ::core::fmt::Debug for NDIS_802_11_NETWORK_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_NETWORK_TYPE_LIST").field("NumberOfItems", &self.NumberOfItems).field("NetworkType", &self.NetworkType).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Non_Bcast_Ssid == other.Non_Bcast_Ssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_NON_BCAST_SSID_LIST {}
impl ::core::fmt::Debug for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_NON_BCAST_SSID_LIST").field("NumberOfItems", &self.NumberOfItems).field("Non_Bcast_Ssid", &self.Non_Bcast_Ssid).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_PMKID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_PMKID {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.BSSIDInfoCount == other.BSSIDInfoCount && self.BSSIDInfo == other.BSSIDInfo
    }
}
impl ::core::cmp::Eq for NDIS_802_11_PMKID {}
impl ::core::fmt::Debug for NDIS_802_11_PMKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_PMKID").field("Length", &self.Length).field("BSSIDInfoCount", &self.BSSIDInfoCount).field("BSSIDInfo", &self.BSSIDInfo).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.NumCandidates == other.NumCandidates && self.CandidateList == other.CandidateList
    }
}
impl ::core::cmp::Eq for NDIS_802_11_PMKID_CANDIDATE_LIST {}
impl ::core::fmt::Debug for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_PMKID_CANDIDATE_LIST").field("Version", &self.Version).field("NumCandidates", &self.NumCandidates).field("CandidateList", &self.CandidateList).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_POWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_POWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_POWER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_PRIVACY_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_PRIVACY_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_PRIVACY_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_RADIO_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_RADIO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_RADIO_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_RELOAD_DEFAULTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_RELOAD_DEFAULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_RELOAD_DEFAULTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_REMOVE_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_REMOVE_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.BSSID == other.BSSID
    }
}
impl ::core::cmp::Eq for NDIS_802_11_REMOVE_KEY {}
impl ::core::fmt::Debug for NDIS_802_11_REMOVE_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_REMOVE_KEY").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("BSSID", &self.BSSID).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_SSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_SSID {
    fn eq(&self, other: &Self) -> bool {
        self.SsidLength == other.SsidLength && self.Ssid == other.Ssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_SSID {}
impl ::core::fmt::Debug for NDIS_802_11_SSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_SSID").field("SsidLength", &self.SsidLength).field("Ssid", &self.Ssid).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.TransmittedFragmentCount == other.TransmittedFragmentCount
            && self.MulticastTransmittedFrameCount == other.MulticastTransmittedFrameCount
            && self.FailedCount == other.FailedCount
            && self.RetryCount == other.RetryCount
            && self.MultipleRetryCount == other.MultipleRetryCount
            && self.RTSSuccessCount == other.RTSSuccessCount
            && self.RTSFailureCount == other.RTSFailureCount
            && self.ACKFailureCount == other.ACKFailureCount
            && self.FrameDuplicateCount == other.FrameDuplicateCount
            && self.ReceivedFragmentCount == other.ReceivedFragmentCount
            && self.MulticastReceivedFrameCount == other.MulticastReceivedFrameCount
            && self.FCSErrorCount == other.FCSErrorCount
            && self.TKIPLocalMICFailures == other.TKIPLocalMICFailures
            && self.TKIPICVErrorCount == other.TKIPICVErrorCount
            && self.TKIPCounterMeasuresInvoked == other.TKIPCounterMeasuresInvoked
            && self.TKIPReplays == other.TKIPReplays
            && self.CCMPFormatErrors == other.CCMPFormatErrors
            && self.CCMPReplays == other.CCMPReplays
            && self.CCMPDecryptErrors == other.CCMPDecryptErrors
            && self.FourWayHandshakeFailures == other.FourWayHandshakeFailures
            && self.WEPUndecryptableCount == other.WEPUndecryptableCount
            && self.WEPICVErrorCount == other.WEPICVErrorCount
            && self.DecryptSuccessCount == other.DecryptSuccessCount
            && self.DecryptFailureCount == other.DecryptFailureCount
    }
}
impl ::core::cmp::Eq for NDIS_802_11_STATISTICS {}
impl ::core::fmt::Debug for NDIS_802_11_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_STATISTICS")
            .field("Length", &self.Length)
            .field("TransmittedFragmentCount", &self.TransmittedFragmentCount)
            .field("MulticastTransmittedFrameCount", &self.MulticastTransmittedFrameCount)
            .field("FailedCount", &self.FailedCount)
            .field("RetryCount", &self.RetryCount)
            .field("MultipleRetryCount", &self.MultipleRetryCount)
            .field("RTSSuccessCount", &self.RTSSuccessCount)
            .field("RTSFailureCount", &self.RTSFailureCount)
            .field("ACKFailureCount", &self.ACKFailureCount)
            .field("FrameDuplicateCount", &self.FrameDuplicateCount)
            .field("ReceivedFragmentCount", &self.ReceivedFragmentCount)
            .field("MulticastReceivedFrameCount", &self.MulticastReceivedFrameCount)
            .field("FCSErrorCount", &self.FCSErrorCount)
            .field("TKIPLocalMICFailures", &self.TKIPLocalMICFailures)
            .field("TKIPICVErrorCount", &self.TKIPICVErrorCount)
            .field("TKIPCounterMeasuresInvoked", &self.TKIPCounterMeasuresInvoked)
            .field("TKIPReplays", &self.TKIPReplays)
            .field("CCMPFormatErrors", &self.CCMPFormatErrors)
            .field("CCMPReplays", &self.CCMPReplays)
            .field("CCMPDecryptErrors", &self.CCMPDecryptErrors)
            .field("FourWayHandshakeFailures", &self.FourWayHandshakeFailures)
            .field("WEPUndecryptableCount", &self.WEPUndecryptableCount)
            .field("WEPICVErrorCount", &self.WEPICVErrorCount)
            .field("DecryptSuccessCount", &self.DecryptSuccessCount)
            .field("DecryptFailureCount", &self.DecryptFailureCount)
            .finish()
    }
}
impl ::core::default::Default for NDIS_802_11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_STATUS_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.StatusType == other.StatusType
    }
}
impl ::core::cmp::Eq for NDIS_802_11_STATUS_INDICATION {}
impl ::core::fmt::Debug for NDIS_802_11_STATUS_INDICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_STATUS_INDICATION").field("StatusType", &self.StatusType).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_STATUS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_TEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDIS_802_11_VARIABLE_IEs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_VARIABLE_IEs {
    fn eq(&self, other: &Self) -> bool {
        self.ElementID == other.ElementID && self.Length == other.Length && self.data == other.data
    }
}
impl ::core::cmp::Eq for NDIS_802_11_VARIABLE_IEs {}
impl ::core::fmt::Debug for NDIS_802_11_VARIABLE_IEs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_VARIABLE_IEs").field("ElementID", &self.ElementID).field("Length", &self.Length).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_WEP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_802_11_WEP {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.KeyLength == other.KeyLength && self.KeyMaterial == other.KeyMaterial
    }
}
impl ::core::cmp::Eq for NDIS_802_11_WEP {}
impl ::core::fmt::Debug for NDIS_802_11_WEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_WEP").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("KeyLength", &self.KeyLength).field("KeyMaterial", &self.KeyMaterial).finish()
    }
}
impl ::core::default::Default for NDIS_802_11_WEP_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_11_WEP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_WEP_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_802_5_RING_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_802_5_RING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_5_RING_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_CO_DEVICE_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_CO_DEVICE_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceDescription == other.DeviceDescription
            && self.DevSpecificInfo == other.DevSpecificInfo
            && self.ulTAPISupplementaryPassThru == other.ulTAPISupplementaryPassThru
            && self.ulAddressModes == other.ulAddressModes
            && self.ulNumAddresses == other.ulNumAddresses
            && self.ulBearerModes == other.ulBearerModes
            && self.ulMaxTxRate == other.ulMaxTxRate
            && self.ulMinTxRate == other.ulMinTxRate
            && self.ulMaxRxRate == other.ulMaxRxRate
            && self.ulMinRxRate == other.ulMinRxRate
            && self.ulMediaModes == other.ulMediaModes
            && self.ulGenerateToneModes == other.ulGenerateToneModes
            && self.ulGenerateToneMaxNumFreq == other.ulGenerateToneMaxNumFreq
            && self.ulGenerateDigitModes == other.ulGenerateDigitModes
            && self.ulMonitorToneMaxNumFreq == other.ulMonitorToneMaxNumFreq
            && self.ulMonitorToneMaxNumEntries == other.ulMonitorToneMaxNumEntries
            && self.ulMonitorDigitModes == other.ulMonitorDigitModes
            && self.ulGatherDigitsMinTimeout == other.ulGatherDigitsMinTimeout
            && self.ulGatherDigitsMaxTimeout == other.ulGatherDigitsMaxTimeout
            && self.ulDevCapFlags == other.ulDevCapFlags
            && self.ulMaxNumActiveCalls == other.ulMaxNumActiveCalls
            && self.ulAnswerMode == other.ulAnswerMode
            && self.ulUUIAcceptSize == other.ulUUIAcceptSize
            && self.ulUUIAnswerSize == other.ulUUIAnswerSize
            && self.ulUUIMakeCallSize == other.ulUUIMakeCallSize
            && self.ulUUIDropSize == other.ulUUIDropSize
            && self.ulUUISendUserUserInfoSize == other.ulUUISendUserUserInfoSize
            && self.ulUUICallInfoSize == other.ulUUICallInfoSize
    }
}
impl ::core::cmp::Eq for NDIS_CO_DEVICE_PROFILE {}
impl ::core::fmt::Debug for NDIS_CO_DEVICE_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_CO_DEVICE_PROFILE")
            .field("DeviceDescription", &self.DeviceDescription)
            .field("DevSpecificInfo", &self.DevSpecificInfo)
            .field("ulTAPISupplementaryPassThru", &self.ulTAPISupplementaryPassThru)
            .field("ulAddressModes", &self.ulAddressModes)
            .field("ulNumAddresses", &self.ulNumAddresses)
            .field("ulBearerModes", &self.ulBearerModes)
            .field("ulMaxTxRate", &self.ulMaxTxRate)
            .field("ulMinTxRate", &self.ulMinTxRate)
            .field("ulMaxRxRate", &self.ulMaxRxRate)
            .field("ulMinRxRate", &self.ulMinRxRate)
            .field("ulMediaModes", &self.ulMediaModes)
            .field("ulGenerateToneModes", &self.ulGenerateToneModes)
            .field("ulGenerateToneMaxNumFreq", &self.ulGenerateToneMaxNumFreq)
            .field("ulGenerateDigitModes", &self.ulGenerateDigitModes)
            .field("ulMonitorToneMaxNumFreq", &self.ulMonitorToneMaxNumFreq)
            .field("ulMonitorToneMaxNumEntries", &self.ulMonitorToneMaxNumEntries)
            .field("ulMonitorDigitModes", &self.ulMonitorDigitModes)
            .field("ulGatherDigitsMinTimeout", &self.ulGatherDigitsMinTimeout)
            .field("ulGatherDigitsMaxTimeout", &self.ulGatherDigitsMaxTimeout)
            .field("ulDevCapFlags", &self.ulDevCapFlags)
            .field("ulMaxNumActiveCalls", &self.ulMaxNumActiveCalls)
            .field("ulAnswerMode", &self.ulAnswerMode)
            .field("ulUUIAcceptSize", &self.ulUUIAcceptSize)
            .field("ulUUIAnswerSize", &self.ulUUIAnswerSize)
            .field("ulUUIMakeCallSize", &self.ulUUIMakeCallSize)
            .field("ulUUIDropSize", &self.ulUUIDropSize)
            .field("ulUUISendUserUserInfoSize", &self.ulUUISendUserUserInfoSize)
            .field("ulUUICallInfoSize", &self.ulUUICallInfoSize)
            .finish()
    }
}
impl ::core::default::Default for NDIS_CO_LINK_SPEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_CO_LINK_SPEED {
    fn eq(&self, other: &Self) -> bool {
        self.Outbound == other.Outbound && self.Inbound == other.Inbound
    }
}
impl ::core::cmp::Eq for NDIS_CO_LINK_SPEED {}
impl ::core::fmt::Debug for NDIS_CO_LINK_SPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_CO_LINK_SPEED").field("Outbound", &self.Outbound).field("Inbound", &self.Inbound).finish()
    }
}
impl ::core::default::Default for NDIS_DEVICE_POWER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_DEVICE_POWER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_DEVICE_POWER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_FDDI_ATTACHMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_FDDI_ATTACHMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_FDDI_ATTACHMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_FDDI_LCONNECTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_FDDI_LCONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_FDDI_LCONNECTION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_FDDI_RING_MGT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_FDDI_RING_MGT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_FDDI_RING_MGT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_GUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.SystemTimestamp1 == other.SystemTimestamp1 && self.HardwareClockTimestamp == other.HardwareClockTimestamp && self.SystemTimestamp2 == other.SystemTimestamp2
    }
}
impl ::core::cmp::Eq for NDIS_HARDWARE_CROSSTIMESTAMP {}
impl ::core::fmt::Debug for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_HARDWARE_CROSSTIMESTAMP").field("Header", &self.Header).field("Flags", &self.Flags).field("SystemTimestamp1", &self.SystemTimestamp1).field("HardwareClockTimestamp", &self.HardwareClockTimestamp).field("SystemTimestamp2", &self.SystemTimestamp2).finish()
    }
}
impl ::core::default::Default for NDIS_HARDWARE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_HARDWARE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_HARDWARE_STATUS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NDIS_INTERFACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NDIS_INTERFACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ifOperStatus == other.ifOperStatus
            && self.ifOperStatusFlags == other.ifOperStatusFlags
            && self.MediaConnectState == other.MediaConnectState
            && self.MediaDuplexState == other.MediaDuplexState
            && self.ifMtu == other.ifMtu
            && self.ifPromiscuousMode == other.ifPromiscuousMode
            && self.ifDeviceWakeUpEnable == other.ifDeviceWakeUpEnable
            && self.XmitLinkSpeed == other.XmitLinkSpeed
            && self.RcvLinkSpeed == other.RcvLinkSpeed
            && self.ifLastChange == other.ifLastChange
            && self.ifCounterDiscontinuityTime == other.ifCounterDiscontinuityTime
            && self.ifInUnknownProtos == other.ifInUnknownProtos
            && self.ifInDiscards == other.ifInDiscards
            && self.ifInErrors == other.ifInErrors
            && self.ifHCInOctets == other.ifHCInOctets
            && self.ifHCInUcastPkts == other.ifHCInUcastPkts
            && self.ifHCInMulticastPkts == other.ifHCInMulticastPkts
            && self.ifHCInBroadcastPkts == other.ifHCInBroadcastPkts
            && self.ifHCOutOctets == other.ifHCOutOctets
            && self.ifHCOutUcastPkts == other.ifHCOutUcastPkts
            && self.ifHCOutMulticastPkts == other.ifHCOutMulticastPkts
            && self.ifHCOutBroadcastPkts == other.ifHCOutBroadcastPkts
            && self.ifOutErrors == other.ifOutErrors
            && self.ifOutDiscards == other.ifOutDiscards
            && self.ifHCInUcastOctets == other.ifHCInUcastOctets
            && self.ifHCInMulticastOctets == other.ifHCInMulticastOctets
            && self.ifHCInBroadcastOctets == other.ifHCInBroadcastOctets
            && self.ifHCOutUcastOctets == other.ifHCOutUcastOctets
            && self.ifHCOutMulticastOctets == other.ifHCOutMulticastOctets
            && self.ifHCOutBroadcastOctets == other.ifHCOutBroadcastOctets
            && self.CompartmentId == other.CompartmentId
            && self.SupportedStatistics == other.SupportedStatistics
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NDIS_INTERFACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NDIS_INTERFACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_INTERFACE_INFORMATION")
            .field("ifOperStatus", &self.ifOperStatus)
            .field("ifOperStatusFlags", &self.ifOperStatusFlags)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("MediaDuplexState", &self.MediaDuplexState)
            .field("ifMtu", &self.ifMtu)
            .field("ifPromiscuousMode", &self.ifPromiscuousMode)
            .field("ifDeviceWakeUpEnable", &self.ifDeviceWakeUpEnable)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("ifLastChange", &self.ifLastChange)
            .field("ifCounterDiscontinuityTime", &self.ifCounterDiscontinuityTime)
            .field("ifInUnknownProtos", &self.ifInUnknownProtos)
            .field("ifInDiscards", &self.ifInDiscards)
            .field("ifInErrors", &self.ifInErrors)
            .field("ifHCInOctets", &self.ifHCInOctets)
            .field("ifHCInUcastPkts", &self.ifHCInUcastPkts)
            .field("ifHCInMulticastPkts", &self.ifHCInMulticastPkts)
            .field("ifHCInBroadcastPkts", &self.ifHCInBroadcastPkts)
            .field("ifHCOutOctets", &self.ifHCOutOctets)
            .field("ifHCOutUcastPkts", &self.ifHCOutUcastPkts)
            .field("ifHCOutMulticastPkts", &self.ifHCOutMulticastPkts)
            .field("ifHCOutBroadcastPkts", &self.ifHCOutBroadcastPkts)
            .field("ifOutErrors", &self.ifOutErrors)
            .field("ifOutDiscards", &self.ifOutDiscards)
            .field("ifHCInUcastOctets", &self.ifHCInUcastOctets)
            .field("ifHCInMulticastOctets", &self.ifHCInMulticastOctets)
            .field("ifHCInBroadcastOctets", &self.ifHCInBroadcastOctets)
            .field("ifHCOutUcastOctets", &self.ifHCOutUcastOctets)
            .field("ifHCOutMulticastOctets", &self.ifHCOutMulticastOctets)
            .field("ifHCOutBroadcastOctets", &self.ifHCOutBroadcastOctets)
            .field("CompartmentId", &self.CompartmentId)
            .field("SupportedStatistics", &self.SupportedStatistics)
            .finish()
    }
}
impl ::core::default::Default for NDIS_INTERRUPT_MODERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_INTERRUPT_MODERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_INTERRUPT_MODERATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.InterruptModeration == other.InterruptModeration
    }
}
impl ::core::cmp::Eq for NDIS_INTERRUPT_MODERATION_PARAMETERS {}
impl ::core::fmt::Debug for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_INTERRUPT_MODERATION_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("InterruptModeration", &self.InterruptModeration).finish()
    }
}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported && self.IPv4AH == other.IPv4AH && self.IPv4ESP == other.IPv4ESP
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1 {}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1").field("Supported", &self.Supported).field("IPv4AH", &self.IPv4AH).field("IPv4ESP", &self.IPv4ESP).finish()
    }
}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_0 {}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_1 {}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1_1").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.AhEspCombined == other.AhEspCombined && self.TransportTunnelCombined == other.TransportTunnelCombined && self.IPv4Options == other.IPv4Options && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_2 {}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1_2").field("Encapsulation", &self.Encapsulation).field("AhEspCombined", &self.AhEspCombined).field("TransportTunnelCombined", &self.TransportTunnelCombined).field("IPv4Options", &self.IPv4Options).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_IP_OPER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IP_OPER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.IpOperationalStatus == other.IpOperationalStatus
    }
}
impl ::core::cmp::Eq for NDIS_IP_OPER_STATE {}
impl ::core::fmt::Debug for NDIS_IP_OPER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IP_OPER_STATE").field("Header", &self.Header).field("Flags", &self.Flags).field("IpOperationalStatus", &self.IpOperationalStatus).finish()
    }
}
impl ::core::default::Default for NDIS_IP_OPER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IP_OPER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.OperationalStatus == other.OperationalStatus && self.OperationalStatusFlags == other.OperationalStatusFlags
    }
}
impl ::core::cmp::Eq for NDIS_IP_OPER_STATUS {}
impl ::core::fmt::Debug for NDIS_IP_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IP_OPER_STATUS").field("AddressFamily", &self.AddressFamily).field("OperationalStatus", &self.OperationalStatus).field("OperationalStatusFlags", &self.OperationalStatusFlags).finish()
    }
}
impl ::core::default::Default for NDIS_IP_OPER_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IP_OPER_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.NumberofAddressFamiliesReturned == other.NumberofAddressFamiliesReturned && self.IpOperationalStatus == other.IpOperationalStatus
    }
}
impl ::core::cmp::Eq for NDIS_IP_OPER_STATUS_INFO {}
impl ::core::fmt::Debug for NDIS_IP_OPER_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IP_OPER_STATUS_INFO").field("Header", &self.Header).field("Flags", &self.Flags).field("NumberofAddressFamiliesReturned", &self.NumberofAddressFamiliesReturned).field("IpOperationalStatus", &self.IpOperationalStatus).finish()
    }
}
impl ::core::default::Default for NDIS_IRDA_PACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_IRDA_PACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExtraBOFs == other.ExtraBOFs && self.MinTurnAroundTime == other.MinTurnAroundTime
    }
}
impl ::core::cmp::Eq for NDIS_IRDA_PACKET_INFO {}
impl ::core::fmt::Debug for NDIS_IRDA_PACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IRDA_PACKET_INFO").field("ExtraBOFs", &self.ExtraBOFs).field("MinTurnAroundTime", &self.MinTurnAroundTime).finish()
    }
}
impl ::core::default::Default for NDIS_LINK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_LINK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaDuplexState == other.MediaDuplexState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.PauseFunctions == other.PauseFunctions && self.AutoNegotiationFlags == other.AutoNegotiationFlags
    }
}
impl ::core::cmp::Eq for NDIS_LINK_PARAMETERS {}
impl ::core::fmt::Debug for NDIS_LINK_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_LINK_PARAMETERS").field("Header", &self.Header).field("MediaDuplexState", &self.MediaDuplexState).field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).field("PauseFunctions", &self.PauseFunctions).field("AutoNegotiationFlags", &self.AutoNegotiationFlags).finish()
    }
}
impl ::core::default::Default for NDIS_LINK_SPEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_LINK_SPEED {
    fn eq(&self, other: &Self) -> bool {
        self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed
    }
}
impl ::core::cmp::Eq for NDIS_LINK_SPEED {}
impl ::core::fmt::Debug for NDIS_LINK_SPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_LINK_SPEED").field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).finish()
    }
}
impl ::core::default::Default for NDIS_LINK_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_LINK_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaConnectState == other.MediaConnectState && self.MediaDuplexState == other.MediaDuplexState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.PauseFunctions == other.PauseFunctions && self.AutoNegotiationFlags == other.AutoNegotiationFlags
    }
}
impl ::core::cmp::Eq for NDIS_LINK_STATE {}
impl ::core::fmt::Debug for NDIS_LINK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_LINK_STATE").field("Header", &self.Header).field("MediaConnectState", &self.MediaConnectState).field("MediaDuplexState", &self.MediaDuplexState).field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).field("PauseFunctions", &self.PauseFunctions).field("AutoNegotiationFlags", &self.AutoNegotiationFlags).finish()
    }
}
impl ::core::default::Default for NDIS_MEDIA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_MEDIA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_MEDIA_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_MEDIUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_MEDIUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_NETWORK_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_NETWORK_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_NETWORK_CHANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Revision == other.Revision && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for NDIS_OBJECT_HEADER {}
impl ::core::fmt::Debug for NDIS_OBJECT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OBJECT_HEADER").field("Type", &self.Type).field("Revision", &self.Revision).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for NDIS_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Checksum == other.Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2 == other.LsoV2 && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_OFFLOAD {}
impl ::core::fmt::Debug for NDIS_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OFFLOAD").field("Header", &self.Header).field("Checksum", &self.Checksum).field("LsoV1", &self.LsoV1).field("IPsecV1", &self.IPsecV1).field("LsoV2", &self.LsoV2).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_OFFLOAD_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_OFFLOAD_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.IPv4Checksum == other.IPv4Checksum && self.TCPIPv4Checksum == other.TCPIPv4Checksum && self.UDPIPv4Checksum == other.UDPIPv4Checksum && self.TCPIPv6Checksum == other.TCPIPv6Checksum && self.UDPIPv6Checksum == other.UDPIPv6Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2IPv4 == other.LsoV2IPv4 && self.LsoV2IPv6 == other.LsoV2IPv6 && self.TcpConnectionIPv4 == other.TcpConnectionIPv4 && self.TcpConnectionIPv6 == other.TcpConnectionIPv6 && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_OFFLOAD_PARAMETERS {}
impl ::core::fmt::Debug for NDIS_OFFLOAD_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OFFLOAD_PARAMETERS")
            .field("Header", &self.Header)
            .field("IPv4Checksum", &self.IPv4Checksum)
            .field("TCPIPv4Checksum", &self.TCPIPv4Checksum)
            .field("UDPIPv4Checksum", &self.UDPIPv4Checksum)
            .field("TCPIPv6Checksum", &self.TCPIPv6Checksum)
            .field("UDPIPv6Checksum", &self.UDPIPv6Checksum)
            .field("LsoV1", &self.LsoV1)
            .field("IPsecV1", &self.IPsecV1)
            .field("LsoV2IPv4", &self.LsoV2IPv4)
            .field("LsoV2IPv6", &self.LsoV2IPv6)
            .field("TcpConnectionIPv4", &self.TcpConnectionIPv4)
            .field("TcpConnectionIPv6", &self.TcpConnectionIPv6)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::default::Default for NDIS_OPER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_OPER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.OperationalStatus == other.OperationalStatus && self.OperationalStatusFlags == other.OperationalStatusFlags
    }
}
impl ::core::cmp::Eq for NDIS_OPER_STATE {}
impl ::core::fmt::Debug for NDIS_OPER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OPER_STATE").field("Header", &self.Header).field("OperationalStatus", &self.OperationalStatus).field("OperationalStatusFlags", &self.OperationalStatusFlags).finish()
    }
}
impl ::core::default::Default for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DeviceType == other.DeviceType && self.CurrentSpeedAndMode == other.CurrentSpeedAndMode && self.CurrentPayloadSize == other.CurrentPayloadSize && self.MaxPayloadSize == other.MaxPayloadSize && self.MaxReadRequestSize == other.MaxReadRequestSize && self.CurrentLinkSpeed == other.CurrentLinkSpeed && self.CurrentLinkWidth == other.CurrentLinkWidth && self.MaxLinkSpeed == other.MaxLinkSpeed && self.MaxLinkWidth == other.MaxLinkWidth && self.PciExpressVersion == other.PciExpressVersion && self.InterruptType == other.InterruptType && self.MaxInterruptMessages == other.MaxInterruptMessages
    }
}
impl ::core::cmp::Eq for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {}
impl ::core::fmt::Debug for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PCI_DEVICE_CUSTOM_PROPERTIES")
            .field("Header", &self.Header)
            .field("DeviceType", &self.DeviceType)
            .field("CurrentSpeedAndMode", &self.CurrentSpeedAndMode)
            .field("CurrentPayloadSize", &self.CurrentPayloadSize)
            .field("MaxPayloadSize", &self.MaxPayloadSize)
            .field("MaxReadRequestSize", &self.MaxReadRequestSize)
            .field("CurrentLinkSpeed", &self.CurrentLinkSpeed)
            .field("CurrentLinkWidth", &self.CurrentLinkWidth)
            .field("MaxLinkSpeed", &self.MaxLinkSpeed)
            .field("MaxLinkWidth", &self.MaxLinkWidth)
            .field("PciExpressVersion", &self.PciExpressVersion)
            .field("InterruptType", &self.InterruptType)
            .field("MaxInterruptMessages", &self.MaxInterruptMessages)
            .finish()
    }
}
impl ::core::default::Default for NDIS_PHYSICAL_MEDIUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_PHYSICAL_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PHYSICAL_MEDIUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_PM_PACKET_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PM_PACKET_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        self.Priority == other.Priority && self.Reserved == other.Reserved && self.MaskSize == other.MaskSize && self.PatternOffset == other.PatternOffset && self.PatternSize == other.PatternSize && self.PatternFlags == other.PatternFlags
    }
}
impl ::core::cmp::Eq for NDIS_PM_PACKET_PATTERN {}
impl ::core::fmt::Debug for NDIS_PM_PACKET_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PM_PACKET_PATTERN").field("Priority", &self.Priority).field("Reserved", &self.Reserved).field("MaskSize", &self.MaskSize).field("PatternOffset", &self.PatternOffset).field("PatternSize", &self.PatternSize).field("PatternFlags", &self.PatternFlags).finish()
    }
}
impl ::core::default::Default for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MinMagicPacketWakeUp == other.MinMagicPacketWakeUp && self.MinPatternWakeUp == other.MinPatternWakeUp && self.MinLinkChangeWakeUp == other.MinLinkChangeWakeUp
    }
}
impl ::core::cmp::Eq for NDIS_PM_WAKE_UP_CAPABILITIES {}
impl ::core::fmt::Debug for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PM_WAKE_UP_CAPABILITIES").field("MinMagicPacketWakeUp", &self.MinMagicPacketWakeUp).field("MinPatternWakeUp", &self.MinPatternWakeUp).field("MinLinkChangeWakeUp", &self.MinLinkChangeWakeUp).finish()
    }
}
impl ::core::default::Default for NDIS_PNP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PNP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.WakeUpCapabilities == other.WakeUpCapabilities
    }
}
impl ::core::cmp::Eq for NDIS_PNP_CAPABILITIES {}
impl ::core::fmt::Debug for NDIS_PNP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PNP_CAPABILITIES").field("Flags", &self.Flags).field("WakeUpCapabilities", &self.WakeUpCapabilities).finish()
    }
}
impl ::core::default::Default for NDIS_PORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PORT {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.NdisReserved == other.NdisReserved && self.MiniportReserved == other.MiniportReserved && self.ProtocolReserved == other.ProtocolReserved && self.PortCharacteristics == other.PortCharacteristics
    }
}
impl ::core::cmp::Eq for NDIS_PORT {}
impl ::core::fmt::Debug for NDIS_PORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT").field("Next", &self.Next).field("NdisReserved", &self.NdisReserved).field("MiniportReserved", &self.MiniportReserved).field("ProtocolReserved", &self.ProtocolReserved).field("PortCharacteristics", &self.PortCharacteristics).finish()
    }
}
impl ::core::default::Default for NDIS_PORT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PORT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.NumberOfPorts == other.NumberOfPorts && self.OffsetFirstPort == other.OffsetFirstPort && self.ElementSize == other.ElementSize && self.Ports == other.Ports
    }
}
impl ::core::cmp::Eq for NDIS_PORT_ARRAY {}
impl ::core::fmt::Debug for NDIS_PORT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_ARRAY").field("Header", &self.Header).field("NumberOfPorts", &self.NumberOfPorts).field("OffsetFirstPort", &self.OffsetFirstPort).field("ElementSize", &self.ElementSize).field("Ports", &self.Ports).finish()
    }
}
impl ::core::default::Default for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState
    }
}
impl ::core::cmp::Eq for NDIS_PORT_AUTHENTICATION_PARAMETERS {}
impl ::core::fmt::Debug for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_AUTHENTICATION_PARAMETERS").field("Header", &self.Header).field("SendControlState", &self.SendControlState).field("RcvControlState", &self.RcvControlState).field("SendAuthorizationState", &self.SendAuthorizationState).field("RcvAuthorizationState", &self.RcvAuthorizationState).finish()
    }
}
impl ::core::default::Default for NDIS_PORT_AUTHORIZATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_PORT_AUTHORIZATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PORT_AUTHORIZATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_PORT_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PORT_CHARACTERISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PortNumber == other.PortNumber && self.Flags == other.Flags && self.Type == other.Type && self.MediaConnectState == other.MediaConnectState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.Direction == other.Direction && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState
    }
}
impl ::core::cmp::Eq for NDIS_PORT_CHARACTERISTICS {}
impl ::core::fmt::Debug for NDIS_PORT_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_CHARACTERISTICS")
            .field("Header", &self.Header)
            .field("PortNumber", &self.PortNumber)
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("Direction", &self.Direction)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .finish()
    }
}
impl ::core::default::Default for NDIS_PORT_CONTROL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_PORT_CONTROL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PORT_CONTROL_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_PORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_PORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaConnectState == other.MediaConnectState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.Direction == other.Direction && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_PORT_STATE {}
impl ::core::fmt::Debug for NDIS_PORT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_STATE")
            .field("Header", &self.Header)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("Direction", &self.Direction)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::default::Default for NDIS_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_PROCESSOR_VENDOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_PROCESSOR_VENDOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PROCESSOR_VENDOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_RECEIVE_HASH_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_RECEIVE_HASH_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.HashInformation == other.HashInformation && self.HashSecretKeySize == other.HashSecretKeySize && self.HashSecretKeyOffset == other.HashSecretKeyOffset
    }
}
impl ::core::cmp::Eq for NDIS_RECEIVE_HASH_PARAMETERS {}
impl ::core::fmt::Debug for NDIS_RECEIVE_HASH_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RECEIVE_HASH_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("HashInformation", &self.HashInformation).field("HashSecretKeySize", &self.HashSecretKeySize).field("HashSecretKeyOffset", &self.HashSecretKeyOffset).finish()
    }
}
impl ::core::default::Default for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.CapabilitiesFlags == other.CapabilitiesFlags && self.NumberOfInterruptMessages == other.NumberOfInterruptMessages && self.NumberOfReceiveQueues == other.NumberOfReceiveQueues
    }
}
impl ::core::cmp::Eq for NDIS_RECEIVE_SCALE_CAPABILITIES {}
impl ::core::fmt::Debug for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RECEIVE_SCALE_CAPABILITIES").field("Header", &self.Header).field("CapabilitiesFlags", &self.CapabilitiesFlags).field("NumberOfInterruptMessages", &self.NumberOfInterruptMessages).field("NumberOfReceiveQueues", &self.NumberOfReceiveQueues).finish()
    }
}
impl ::core::default::Default for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.BaseCpuNumber == other.BaseCpuNumber && self.HashInformation == other.HashInformation && self.IndirectionTableSize == other.IndirectionTableSize && self.IndirectionTableOffset == other.IndirectionTableOffset && self.HashSecretKeySize == other.HashSecretKeySize && self.HashSecretKeyOffset == other.HashSecretKeyOffset
    }
}
impl ::core::cmp::Eq for NDIS_RECEIVE_SCALE_PARAMETERS {}
impl ::core::fmt::Debug for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RECEIVE_SCALE_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("BaseCpuNumber", &self.BaseCpuNumber).field("HashInformation", &self.HashInformation).field("IndirectionTableSize", &self.IndirectionTableSize).field("IndirectionTableOffset", &self.IndirectionTableOffset).field("HashSecretKeySize", &self.HashSecretKeySize).field("HashSecretKeyOffset", &self.HashSecretKeyOffset).finish()
    }
}
impl ::core::default::Default for NDIS_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_REQUEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_STATISTICS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_STATISTICS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.SupportedStatistics == other.SupportedStatistics
            && self.ifInDiscards == other.ifInDiscards
            && self.ifInErrors == other.ifInErrors
            && self.ifHCInOctets == other.ifHCInOctets
            && self.ifHCInUcastPkts == other.ifHCInUcastPkts
            && self.ifHCInMulticastPkts == other.ifHCInMulticastPkts
            && self.ifHCInBroadcastPkts == other.ifHCInBroadcastPkts
            && self.ifHCOutOctets == other.ifHCOutOctets
            && self.ifHCOutUcastPkts == other.ifHCOutUcastPkts
            && self.ifHCOutMulticastPkts == other.ifHCOutMulticastPkts
            && self.ifHCOutBroadcastPkts == other.ifHCOutBroadcastPkts
            && self.ifOutErrors == other.ifOutErrors
            && self.ifOutDiscards == other.ifOutDiscards
            && self.ifHCInUcastOctets == other.ifHCInUcastOctets
            && self.ifHCInMulticastOctets == other.ifHCInMulticastOctets
            && self.ifHCInBroadcastOctets == other.ifHCInBroadcastOctets
            && self.ifHCOutUcastOctets == other.ifHCOutUcastOctets
            && self.ifHCOutMulticastOctets == other.ifHCOutMulticastOctets
            && self.ifHCOutBroadcastOctets == other.ifHCOutBroadcastOctets
    }
}
impl ::core::cmp::Eq for NDIS_STATISTICS_INFO {}
impl ::core::fmt::Debug for NDIS_STATISTICS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_STATISTICS_INFO")
            .field("Header", &self.Header)
            .field("SupportedStatistics", &self.SupportedStatistics)
            .field("ifInDiscards", &self.ifInDiscards)
            .field("ifInErrors", &self.ifInErrors)
            .field("ifHCInOctets", &self.ifHCInOctets)
            .field("ifHCInUcastPkts", &self.ifHCInUcastPkts)
            .field("ifHCInMulticastPkts", &self.ifHCInMulticastPkts)
            .field("ifHCInBroadcastPkts", &self.ifHCInBroadcastPkts)
            .field("ifHCOutOctets", &self.ifHCOutOctets)
            .field("ifHCOutUcastPkts", &self.ifHCOutUcastPkts)
            .field("ifHCOutMulticastPkts", &self.ifHCOutMulticastPkts)
            .field("ifHCOutBroadcastPkts", &self.ifHCOutBroadcastPkts)
            .field("ifOutErrors", &self.ifOutErrors)
            .field("ifOutDiscards", &self.ifOutDiscards)
            .field("ifHCInUcastOctets", &self.ifHCInUcastOctets)
            .field("ifHCInMulticastOctets", &self.ifHCInMulticastOctets)
            .field("ifHCInBroadcastOctets", &self.ifHCInBroadcastOctets)
            .field("ifHCOutUcastOctets", &self.ifHCOutUcastOctets)
            .field("ifHCOutMulticastOctets", &self.ifHCOutMulticastOctets)
            .field("ifHCOutBroadcastOctets", &self.ifHCOutBroadcastOctets)
            .finish()
    }
}
impl ::core::default::Default for NDIS_STATISTICS_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_STATISTICS_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Oid == other.Oid && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for NDIS_STATISTICS_VALUE {}
impl ::core::fmt::Debug for NDIS_STATISTICS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_STATISTICS_VALUE").field("Oid", &self.Oid).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for NDIS_STATISTICS_VALUE_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_STATISTICS_VALUE_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Oid == other.Oid && self.DataLength == other.DataLength && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for NDIS_STATISTICS_VALUE_EX {}
impl ::core::fmt::Debug for NDIS_STATISTICS_VALUE_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_STATISTICS_VALUE_EX").field("Oid", &self.Oid).field("DataLength", &self.DataLength).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for NDIS_SUPPORTED_PAUSE_FUNCTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_SUPPORTED_PAUSE_FUNCTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_SUPPORTED_PAUSE_FUNCTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_CONNECTION_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_CONNECTION_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield && self.TcpConnectionOffloadCapacity == other.TcpConnectionOffloadCapacity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_TCP_CONNECTION_OFFLOAD {}
impl ::core::fmt::Debug for NDIS_TCP_CONNECTION_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_CONNECTION_OFFLOAD").field("Header", &self.Header).field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).field("TcpConnectionOffloadCapacity", &self.TcpConnectionOffloadCapacity).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4Transmit == other.IPv4Transmit && self.IPv4Receive == other.IPv4Receive && self.IPv6Transmit == other.IPv6Transmit && self.IPv6Receive == other.IPv6Receive
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD").field("IPv4Transmit", &self.IPv4Transmit).field("IPv4Receive", &self.IPv4Receive).field("IPv6Transmit", &self.IPv6Transmit).field("IPv6Receive", &self.IPv6Receive).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_0").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_1").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_2").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_3").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V1").field("IPv4", &self.IPv4).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4 && self.IPv6 == other.IPv6
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2").field("IPv4", &self.IPv4).field("IPv6", &self.IPv6).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).finish()
    }
}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.TimeoutArrayLength == other.TimeoutArrayLength && self.TimeoutArray == other.TimeoutArray
    }
}
impl ::core::cmp::Eq for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {}
impl ::core::fmt::Debug for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES").field("Header", &self.Header).field("Flags", &self.Flags).field("TimeoutArrayLength", &self.TimeoutArrayLength).field("TimeoutArray", &self.TimeoutArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NDIS_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NDIS_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.HardwareClockFrequencyHz == other.HardwareClockFrequencyHz && self.CrossTimestamp == other.CrossTimestamp && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.TimestampFlags == other.TimestampFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NDIS_TIMESTAMP_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NDIS_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TIMESTAMP_CAPABILITIES").field("Header", &self.Header).field("HardwareClockFrequencyHz", &self.HardwareClockFrequencyHz).field("CrossTimestamp", &self.CrossTimestamp).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("TimestampFlags", &self.TimestampFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.PtpV2OverUdpIPv4EventMsgReceiveHw == other.PtpV2OverUdpIPv4EventMsgReceiveHw
            && self.PtpV2OverUdpIPv4AllMsgReceiveHw == other.PtpV2OverUdpIPv4AllMsgReceiveHw
            && self.PtpV2OverUdpIPv4EventMsgTransmitHw == other.PtpV2OverUdpIPv4EventMsgTransmitHw
            && self.PtpV2OverUdpIPv4AllMsgTransmitHw == other.PtpV2OverUdpIPv4AllMsgTransmitHw
            && self.PtpV2OverUdpIPv6EventMsgReceiveHw == other.PtpV2OverUdpIPv6EventMsgReceiveHw
            && self.PtpV2OverUdpIPv6AllMsgReceiveHw == other.PtpV2OverUdpIPv6AllMsgReceiveHw
            && self.PtpV2OverUdpIPv6EventMsgTransmitHw == other.PtpV2OverUdpIPv6EventMsgTransmitHw
            && self.PtpV2OverUdpIPv6AllMsgTransmitHw == other.PtpV2OverUdpIPv6AllMsgTransmitHw
            && self.AllReceiveHw == other.AllReceiveHw
            && self.AllTransmitHw == other.AllTransmitHw
            && self.TaggedTransmitHw == other.TaggedTransmitHw
            && self.AllReceiveSw == other.AllReceiveSw
            && self.AllTransmitSw == other.AllTransmitSw
            && self.TaggedTransmitSw == other.TaggedTransmitSw
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NDIS_TIMESTAMP_CAPABILITY_FLAGS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TIMESTAMP_CAPABILITY_FLAGS")
            .field("PtpV2OverUdpIPv4EventMsgReceiveHw", &self.PtpV2OverUdpIPv4EventMsgReceiveHw)
            .field("PtpV2OverUdpIPv4AllMsgReceiveHw", &self.PtpV2OverUdpIPv4AllMsgReceiveHw)
            .field("PtpV2OverUdpIPv4EventMsgTransmitHw", &self.PtpV2OverUdpIPv4EventMsgTransmitHw)
            .field("PtpV2OverUdpIPv4AllMsgTransmitHw", &self.PtpV2OverUdpIPv4AllMsgTransmitHw)
            .field("PtpV2OverUdpIPv6EventMsgReceiveHw", &self.PtpV2OverUdpIPv6EventMsgReceiveHw)
            .field("PtpV2OverUdpIPv6AllMsgReceiveHw", &self.PtpV2OverUdpIPv6AllMsgReceiveHw)
            .field("PtpV2OverUdpIPv6EventMsgTransmitHw", &self.PtpV2OverUdpIPv6EventMsgTransmitHw)
            .field("PtpV2OverUdpIPv6AllMsgTransmitHw", &self.PtpV2OverUdpIPv6AllMsgTransmitHw)
            .field("AllReceiveHw", &self.AllReceiveHw)
            .field("AllTransmitHw", &self.AllTransmitHw)
            .field("TaggedTransmitHw", &self.TaggedTransmitHw)
            .field("AllReceiveSw", &self.AllReceiveSw)
            .field("AllTransmitSw", &self.AllTransmitSw)
            .field("TaggedTransmitSw", &self.TaggedTransmitSw)
            .finish()
    }
}
impl ::core::default::Default for NDIS_VAR_DATA_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_VAR_DATA_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for NDIS_VAR_DATA_DESC {}
impl ::core::fmt::Debug for NDIS_VAR_DATA_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_VAR_DATA_DESC").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for NDIS_WAN_HEADER_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_WAN_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WAN_HEADER_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_WAN_MEDIUM_SUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_WAN_MEDIUM_SUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WAN_MEDIUM_SUBTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_WAN_PROTOCOL_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WAN_PROTOCOL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDIS_WAN_PROTOCOL_CAPS {}
impl ::core::fmt::Debug for NDIS_WAN_PROTOCOL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WAN_PROTOCOL_CAPS").field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for NDIS_WAN_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDIS_WAN_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WAN_QUALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDIS_WLAN_BSSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WLAN_BSSID {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MacAddress == other.MacAddress && self.Reserved == other.Reserved && self.Ssid == other.Ssid && self.Privacy == other.Privacy && self.Rssi == other.Rssi && self.NetworkTypeInUse == other.NetworkTypeInUse && self.Configuration == other.Configuration && self.InfrastructureMode == other.InfrastructureMode && self.SupportedRates == other.SupportedRates
    }
}
impl ::core::cmp::Eq for NDIS_WLAN_BSSID {}
impl ::core::fmt::Debug for NDIS_WLAN_BSSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WLAN_BSSID").field("Length", &self.Length).field("MacAddress", &self.MacAddress).field("Reserved", &self.Reserved).field("Ssid", &self.Ssid).field("Privacy", &self.Privacy).field("Rssi", &self.Rssi).field("NetworkTypeInUse", &self.NetworkTypeInUse).field("Configuration", &self.Configuration).field("InfrastructureMode", &self.InfrastructureMode).field("SupportedRates", &self.SupportedRates).finish()
    }
}
impl ::core::default::Default for NDIS_WLAN_BSSID_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WLAN_BSSID_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MacAddress == other.MacAddress && self.Reserved == other.Reserved && self.Ssid == other.Ssid && self.Privacy == other.Privacy && self.Rssi == other.Rssi && self.NetworkTypeInUse == other.NetworkTypeInUse && self.Configuration == other.Configuration && self.InfrastructureMode == other.InfrastructureMode && self.SupportedRates == other.SupportedRates && self.IELength == other.IELength && self.IEs == other.IEs
    }
}
impl ::core::cmp::Eq for NDIS_WLAN_BSSID_EX {}
impl ::core::fmt::Debug for NDIS_WLAN_BSSID_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WLAN_BSSID_EX")
            .field("Length", &self.Length)
            .field("MacAddress", &self.MacAddress)
            .field("Reserved", &self.Reserved)
            .field("Ssid", &self.Ssid)
            .field("Privacy", &self.Privacy)
            .field("Rssi", &self.Rssi)
            .field("NetworkTypeInUse", &self.NetworkTypeInUse)
            .field("Configuration", &self.Configuration)
            .field("InfrastructureMode", &self.InfrastructureMode)
            .field("SupportedRates", &self.SupportedRates)
            .field("IELength", &self.IELength)
            .field("IEs", &self.IEs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NDIS_WMI_ENUM_ADAPTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDIS_WMI_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported && self.IPv4AH == other.IPv4AH && self.IPv4ESP == other.IPv4ESP
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1 {}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1").field("Supported", &self.Supported).field("IPv4AH", &self.IPv4AH).field("IPv4ESP", &self.IPv4ESP).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Md5 == other.Md5 && self.Sha_1 == other.Sha_1 && self.Transport == other.Transport && self.Tunnel == other.Tunnel && self.Send == other.Send && self.Receive == other.Receive
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1_0").field("Md5", &self.Md5).field("Sha_1", &self.Sha_1).field("Transport", &self.Transport).field("Tunnel", &self.Tunnel).field("Send", &self.Send).field("Receive", &self.Receive).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Des == other.Des && self.Reserved == other.Reserved && self.TripleDes == other.TripleDes && self.NullEsp == other.NullEsp && self.Transport == other.Transport && self.Tunnel == other.Tunnel && self.Send == other.Send && self.Receive == other.Receive
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1_1").field("Des", &self.Des).field("Reserved", &self.Reserved).field("TripleDes", &self.TripleDes).field("NullEsp", &self.NullEsp).field("Transport", &self.Transport).field("Tunnel", &self.Tunnel).field("Send", &self.Send).field("Receive", &self.Receive).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.AhEspCombined == other.AhEspCombined && self.TransportTunnelCombined == other.TransportTunnelCombined && self.IPv4Options == other.IPv4Options && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1_2").field("Encapsulation", &self.Encapsulation).field("AhEspCombined", &self.AhEspCombined).field("TransportTunnelCombined", &self.TransportTunnelCombined).field("IPv4Options", &self.IPv4Options).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_METHOD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDIS_WMI_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Checksum == other.Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2 == other.LsoV2 && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_WMI_OFFLOAD {}
impl ::core::fmt::Debug for NDIS_WMI_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_OFFLOAD").field("Header", &self.Header).field("Checksum", &self.Checksum).field("LsoV1", &self.LsoV1).field("IPsecV1", &self.IPsecV1).field("LsoV2", &self.LsoV2).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_OUTPUT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_OUTPUT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.SupportedRevision == other.SupportedRevision && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for NDIS_WMI_OUTPUT_INFO {}
impl ::core::fmt::Debug for NDIS_WMI_OUTPUT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_OUTPUT_INFO").field("Header", &self.Header).field("Flags", &self.Flags).field("SupportedRevision", &self.SupportedRevision).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_SET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Encapsulation == other.Encapsulation && self.SupportIPv4 == other.SupportIPv4 && self.SupportIPv6 == other.SupportIPv6 && self.SupportIPv6ExtensionHeaders == other.SupportIPv6ExtensionHeaders && self.SupportSack == other.SupportSack && self.TcpConnectionOffloadCapacity == other.TcpConnectionOffloadCapacity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_CONNECTION_OFFLOAD {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_CONNECTION_OFFLOAD").field("Header", &self.Header).field("Encapsulation", &self.Encapsulation).field("SupportIPv4", &self.SupportIPv4).field("SupportIPv6", &self.SupportIPv6).field("SupportIPv6ExtensionHeaders", &self.SupportIPv6ExtensionHeaders).field("SupportSack", &self.SupportSack).field("TcpConnectionOffloadCapacity", &self.TcpConnectionOffloadCapacity).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4Transmit == other.IPv4Transmit && self.IPv4Receive == other.IPv4Receive && self.IPv6Transmit == other.IPv6Transmit && self.IPv6Receive == other.IPv6Receive
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD").field("IPv4Transmit", &self.IPv4Transmit).field("IPv4Receive", &self.IPv4Receive).field("IPv6Transmit", &self.IPv6Transmit).field("IPv6Receive", &self.IPv6Receive).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpOptionsSupported == other.IpOptionsSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum && self.IpChecksum == other.IpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0").field("Encapsulation", &self.Encapsulation).field("IpOptionsSupported", &self.IpOptionsSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).field("IpChecksum", &self.IpChecksum).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpOptionsSupported == other.IpOptionsSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum && self.IpChecksum == other.IpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1").field("Encapsulation", &self.Encapsulation).field("IpOptionsSupported", &self.IpOptionsSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).field("IpChecksum", &self.IpChecksum).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2").field("Encapsulation", &self.Encapsulation).field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3").field("Encapsulation", &self.Encapsulation).field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1").field("IPv4", &self.IPv4).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self.TcpOptions == other.TcpOptions && self.IpOptions == other.IpOptions
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("TcpOptions", &self.TcpOptions).field("IpOptions", &self.IpOptions).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4 && self.IPv6 == other.IPv6
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2").field("IPv4", &self.IPv4).field("IPv6", &self.IPv6).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).finish()
    }
}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).finish()
    }
}
impl ::core::default::Default for NDK_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDK_ADAPTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.VendorId == other.VendorId
            && self.DeviceId == other.DeviceId
            && self.MaxRegistrationSize == other.MaxRegistrationSize
            && self.MaxWindowSize == other.MaxWindowSize
            && self.FRMRPageCount == other.FRMRPageCount
            && self.MaxInitiatorRequestSge == other.MaxInitiatorRequestSge
            && self.MaxReceiveRequestSge == other.MaxReceiveRequestSge
            && self.MaxReadRequestSge == other.MaxReadRequestSge
            && self.MaxTransferLength == other.MaxTransferLength
            && self.MaxInlineDataSize == other.MaxInlineDataSize
            && self.MaxInboundReadLimit == other.MaxInboundReadLimit
            && self.MaxOutboundReadLimit == other.MaxOutboundReadLimit
            && self.MaxReceiveQueueDepth == other.MaxReceiveQueueDepth
            && self.MaxInitiatorQueueDepth == other.MaxInitiatorQueueDepth
            && self.MaxSrqDepth == other.MaxSrqDepth
            && self.MaxCqDepth == other.MaxCqDepth
            && self.LargeRequestThreshold == other.LargeRequestThreshold
            && self.MaxCallerData == other.MaxCallerData
            && self.MaxCalleeData == other.MaxCalleeData
            && self.AdapterFlags == other.AdapterFlags
            && self.RdmaTechnology == other.RdmaTechnology
    }
}
impl ::core::cmp::Eq for NDK_ADAPTER_INFO {}
impl ::core::fmt::Debug for NDK_ADAPTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDK_ADAPTER_INFO")
            .field("Version", &self.Version)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("MaxRegistrationSize", &self.MaxRegistrationSize)
            .field("MaxWindowSize", &self.MaxWindowSize)
            .field("FRMRPageCount", &self.FRMRPageCount)
            .field("MaxInitiatorRequestSge", &self.MaxInitiatorRequestSge)
            .field("MaxReceiveRequestSge", &self.MaxReceiveRequestSge)
            .field("MaxReadRequestSge", &self.MaxReadRequestSge)
            .field("MaxTransferLength", &self.MaxTransferLength)
            .field("MaxInlineDataSize", &self.MaxInlineDataSize)
            .field("MaxInboundReadLimit", &self.MaxInboundReadLimit)
            .field("MaxOutboundReadLimit", &self.MaxOutboundReadLimit)
            .field("MaxReceiveQueueDepth", &self.MaxReceiveQueueDepth)
            .field("MaxInitiatorQueueDepth", &self.MaxInitiatorQueueDepth)
            .field("MaxSrqDepth", &self.MaxSrqDepth)
            .field("MaxCqDepth", &self.MaxCqDepth)
            .field("LargeRequestThreshold", &self.LargeRequestThreshold)
            .field("MaxCallerData", &self.MaxCallerData)
            .field("MaxCalleeData", &self.MaxCalleeData)
            .field("AdapterFlags", &self.AdapterFlags)
            .field("RdmaTechnology", &self.RdmaTechnology)
            .finish()
    }
}
impl ::core::default::Default for NDK_RDMA_TECHNOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NDK_RDMA_TECHNOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDK_RDMA_TECHNOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for NDK_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NDK_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.Major == other.Major && self.Minor == other.Minor
    }
}
impl ::core::cmp::Eq for NDK_VERSION {}
impl ::core::fmt::Debug for NDK_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDK_VERSION").field("Major", &self.Major).field("Minor", &self.Minor).finish()
    }
}
impl ::core::default::Default for NETWORK_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressLength == other.AddressLength && self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS {}
impl ::core::fmt::Debug for NETWORK_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS").field("AddressLength", &self.AddressLength).field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for NETWORK_ADDRESS_IP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_IP {
    fn eq(&self, other: &Self) -> bool {
        self.sin_port == other.sin_port && self.IN_ADDR == other.IN_ADDR && self.sin_zero == other.sin_zero
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_IP {}
impl ::core::fmt::Debug for NETWORK_ADDRESS_IP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_IP").field("sin_port", &self.sin_port).field("IN_ADDR", &self.IN_ADDR).field("sin_zero", &self.sin_zero).finish()
    }
}
impl ::core::default::Default for NETWORK_ADDRESS_IP6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_IP6 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_port == other.sin6_port && self.sin6_flowinfo == other.sin6_flowinfo && self.sin6_addr == other.sin6_addr && self.sin6_scope_id == other.sin6_scope_id
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_IP6 {}
impl ::core::fmt::Debug for NETWORK_ADDRESS_IP6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_IP6").field("sin6_port", &self.sin6_port).field("sin6_flowinfo", &self.sin6_flowinfo).field("sin6_addr", &self.sin6_addr).field("sin6_scope_id", &self.sin6_scope_id).finish()
    }
}
impl ::core::default::Default for NETWORK_ADDRESS_IPX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_IPX {
    fn eq(&self, other: &Self) -> bool {
        self.NetworkAddress == other.NetworkAddress && self.NodeAddress == other.NodeAddress && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_IPX {}
impl ::core::fmt::Debug for NETWORK_ADDRESS_IPX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_IPX").field("NetworkAddress", &self.NetworkAddress).field("NodeAddress", &self.NodeAddress).field("Socket", &self.Socket).finish()
    }
}
impl ::core::default::Default for NETWORK_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AddressCount == other.AddressCount && self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_LIST {}
impl ::core::fmt::Debug for NETWORK_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_LIST").field("AddressCount", &self.AddressCount).field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::core::default::Default for NET_IF_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_ACCESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_ADMIN_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_ADMIN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_ADMIN_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_ALIAS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_IF_ALIAS_LH {
    fn eq(&self, other: &Self) -> bool {
        self.ifAliasLength == other.ifAliasLength && self.ifAliasOffset == other.ifAliasOffset
    }
}
impl ::core::cmp::Eq for NET_IF_ALIAS_LH {}
impl ::core::fmt::Debug for NET_IF_ALIAS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_IF_ALIAS_LH").field("ifAliasLength", &self.ifAliasLength).field("ifAliasOffset", &self.ifAliasOffset).finish()
    }
}
impl ::core::default::Default for NET_IF_CONNECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_CONNECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_CONNECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_DIRECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_DIRECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_DIRECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_MEDIA_CONNECT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_MEDIA_CONNECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_MEDIA_CONNECT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_MEDIA_DUPLEX_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_MEDIA_DUPLEX_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_MEDIA_DUPLEX_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_OPER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_OPER_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_IF_RCV_ADDRESS_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_IF_RCV_ADDRESS_LH {
    fn eq(&self, other: &Self) -> bool {
        self.ifRcvAddressType == other.ifRcvAddressType && self.ifRcvAddressLength == other.ifRcvAddressLength && self.ifRcvAddressOffset == other.ifRcvAddressOffset
    }
}
impl ::core::cmp::Eq for NET_IF_RCV_ADDRESS_LH {}
impl ::core::fmt::Debug for NET_IF_RCV_ADDRESS_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_IF_RCV_ADDRESS_LH").field("ifRcvAddressType", &self.ifRcvAddressType).field("ifRcvAddressLength", &self.ifRcvAddressLength).field("ifRcvAddressOffset", &self.ifRcvAddressOffset).finish()
    }
}
impl ::core::default::Default for NET_IF_RCV_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NET_IF_RCV_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_IF_RCV_ADDRESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for NET_LUID_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for NET_PHYSICAL_LOCATION_LH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_PHYSICAL_LOCATION_LH {
    fn eq(&self, other: &Self) -> bool {
        self.BusNumber == other.BusNumber && self.SlotNumber == other.SlotNumber && self.FunctionNumber == other.FunctionNumber
    }
}
impl ::core::cmp::Eq for NET_PHYSICAL_LOCATION_LH {}
impl ::core::fmt::Debug for NET_PHYSICAL_LOCATION_LH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_PHYSICAL_LOCATION_LH").field("BusNumber", &self.BusNumber).field("SlotNumber", &self.SlotNumber).field("FunctionNumber", &self.FunctionNumber).finish()
    }
}
impl ::core::default::Default for OFFLOAD_ALGO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OFFLOAD_ALGO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier && self.algoKeylen == other.algoKeylen && self.algoRounds == other.algoRounds
    }
}
impl ::core::cmp::Eq for OFFLOAD_ALGO_INFO {}
impl ::core::fmt::Debug for OFFLOAD_ALGO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_ALGO_INFO").field("algoIdentifier", &self.algoIdentifier).field("algoKeylen", &self.algoKeylen).field("algoRounds", &self.algoRounds).finish()
    }
}
impl ::core::default::Default for OFFLOAD_CONF_ALGO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLOAD_CONF_ALGO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLOAD_CONF_ALGO").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLOAD_INTEGRITY_ALGO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLOAD_INTEGRITY_ALGO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLOAD_INTEGRITY_ALGO").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFFLOAD_IPSEC_ADD_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_ADD_SA {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr && self.SrcMask == other.SrcMask && self.DestAddr == other.DestAddr && self.DestMask == other.DestMask && self.Protocol == other.Protocol && self.SrcPort == other.SrcPort && self.DestPort == other.DestPort && self.SrcTunnelAddr == other.SrcTunnelAddr && self.DestTunnelAddr == other.DestTunnelAddr && self.Flags == other.Flags && self.NumSAs == other.NumSAs && self.SecAssoc == other.SecAssoc && self.OffloadHandle == other.OffloadHandle && self.KeyLen == other.KeyLen && self.KeyMat == other.KeyMat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFFLOAD_IPSEC_ADD_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFFLOAD_IPSEC_ADD_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_ADD_SA")
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DestAddr", &self.DestAddr)
            .field("DestMask", &self.DestMask)
            .field("Protocol", &self.Protocol)
            .field("SrcPort", &self.SrcPort)
            .field("DestPort", &self.DestPort)
            .field("SrcTunnelAddr", &self.SrcTunnelAddr)
            .field("DestTunnelAddr", &self.DestTunnelAddr)
            .field("Flags", &self.Flags)
            .field("NumSAs", &self.NumSAs)
            .field("SecAssoc", &self.SecAssoc)
            .field("OffloadHandle", &self.OffloadHandle)
            .field("KeyLen", &self.KeyLen)
            .field("KeyMat", &self.KeyMat)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr && self.SrcMask == other.SrcMask && self.DstAddr == other.DstAddr && self.DstMask == other.DstMask && self.Protocol == other.Protocol && self.SrcPort == other.SrcPort && self.DstPort == other.DstPort && self.SrcTunnelAddr == other.SrcTunnelAddr && self.DstTunnelAddr == other.DstTunnelAddr && self.Flags == other.Flags && self.NumSAs == other.NumSAs && self.SecAssoc == other.SecAssoc && self.OffloadHandle == other.OffloadHandle && self.EncapTypeEntry == other.EncapTypeEntry && self.EncapTypeEntryOffldHandle == other.EncapTypeEntryOffldHandle && self.KeyLen == other.KeyLen && self.KeyMat == other.KeyMat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFFLOAD_IPSEC_ADD_UDPESP_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_ADD_UDPESP_SA")
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DstAddr", &self.DstAddr)
            .field("DstMask", &self.DstMask)
            .field("Protocol", &self.Protocol)
            .field("SrcPort", &self.SrcPort)
            .field("DstPort", &self.DstPort)
            .field("SrcTunnelAddr", &self.SrcTunnelAddr)
            .field("DstTunnelAddr", &self.DstTunnelAddr)
            .field("Flags", &self.Flags)
            .field("NumSAs", &self.NumSAs)
            .field("SecAssoc", &self.SecAssoc)
            .field("OffloadHandle", &self.OffloadHandle)
            .field("EncapTypeEntry", &self.EncapTypeEntry)
            .field("EncapTypeEntryOffldHandle", &self.EncapTypeEntryOffldHandle)
            .field("KeyLen", &self.KeyLen)
            .field("KeyMat", &self.KeyMat)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFFLOAD_IPSEC_DELETE_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_DELETE_SA {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadHandle == other.OffloadHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFFLOAD_IPSEC_DELETE_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFFLOAD_IPSEC_DELETE_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_DELETE_SA").field("OffloadHandle", &self.OffloadHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadHandle == other.OffloadHandle && self.EncapTypeEntryOffldHandle == other.EncapTypeEntryOffldHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OFFLOAD_IPSEC_DELETE_UDPESP_SA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_DELETE_UDPESP_SA").field("OffloadHandle", &self.OffloadHandle).field("EncapTypeEntryOffldHandle", &self.EncapTypeEntryOffldHandle).finish()
    }
}
impl ::core::default::Default for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.UdpEncapType == other.UdpEncapType && self.DstEncapPort == other.DstEncapPort
    }
}
impl ::core::cmp::Eq for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {}
impl ::core::fmt::Debug for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY").field("UdpEncapType", &self.UdpEncapType).field("DstEncapPort", &self.DstEncapPort).finish()
    }
}
impl ::core::default::Default for OFFLOAD_OPERATION_E {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFLOAD_OPERATION_E {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLOAD_OPERATION_E").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFLOAD_SECURITY_ASSOCIATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OFFLOAD_SECURITY_ASSOCIATION {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.SPI == other.SPI && self.IntegrityAlgo == other.IntegrityAlgo && self.ConfAlgo == other.ConfAlgo && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for OFFLOAD_SECURITY_ASSOCIATION {}
impl ::core::fmt::Debug for OFFLOAD_SECURITY_ASSOCIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_SECURITY_ASSOCIATION").field("Operation", &self.Operation).field("SPI", &self.SPI).field("IntegrityAlgo", &self.IntegrityAlgo).field("ConfAlgo", &self.ConfAlgo).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for PMKID_CANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PMKID_CANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PMKID_CANDIDATE {}
impl ::core::fmt::Debug for PMKID_CANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PMKID_CANDIDATE").field("BSSID", &self.BSSID).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for TRANSPORT_HEADER_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRANSPORT_HEADER_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolType == other.ProtocolType && self.HeaderOffset == other.HeaderOffset
    }
}
impl ::core::cmp::Eq for TRANSPORT_HEADER_OFFSET {}
impl ::core::fmt::Debug for TRANSPORT_HEADER_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_HEADER_OFFSET").field("ProtocolType", &self.ProtocolType).field("HeaderOffset", &self.HeaderOffset).finish()
    }
}
impl ::core::default::Default for TUNNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TUNNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TUNNEL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for UDP_ENCAP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UDP_ENCAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDP_ENCAP_TYPE").field(&self.0).finish()
    }
}
