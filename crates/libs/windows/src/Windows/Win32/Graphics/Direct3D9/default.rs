#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DADAPTER_IDENTIFIER9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DAES_CTR_IV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DAES_CTR_IV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNELTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNELTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DAUTHENTICATEDCHANNELTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.DXVA2DecodeHandle == other.DXVA2DecodeHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.DeviceHandle == other.DeviceHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION").field("Parameters", &self.Parameters).field("DXVA2DecodeHandle", &self.DXVA2DecodeHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.StartSequenceQuery == other.StartSequenceQuery && self.StartSequenceConfigure == other.StartSequenceConfigure
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE").field("Parameters", &self.Parameters).field("StartSequenceQuery", &self.StartSequenceQuery).field("StartSequenceConfigure", &self.StartSequenceConfigure).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.ProcessIdentiferType == other.ProcessIdentiferType && self.ProcessHandle == other.ProcessHandle && self.AllowAccess == other.AllowAccess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE").field("Parameters", &self.Parameters).field("ProcessIdentiferType", &self.ProcessIdentiferType).field("ProcessHandle", &self.ProcessHandle).field("AllowAccess", &self.AllowAccess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.EncryptionGuid == other.EncryptionGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION").field("Parameters", &self.Parameters).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.omac == other.omac && self.ConfigureType == other.ConfigureType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT").field("omac", &self.omac).field("ConfigureType", &self.ConfigureType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.omac == other.omac && self.ConfigureType == other.ConfigureType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber && self.ReturnCode == other.ReturnCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT").field("omac", &self.omac).field("ConfigureType", &self.ConfigureType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).field("ReturnCode", &self.ReturnCode).finish()
    }
}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.ChannelType == other.ChannelType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT").field("Output", &self.Output).field("ChannelType", &self.ChannelType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.DXVA2DecodeHandle == other.DXVA2DecodeHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT").field("Input", &self.Input).field("DXVA2DecodeHandle", &self.DXVA2DecodeHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DXVA2DecodeHandle == other.DXVA2DecodeHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.DeviceHandle == other.DeviceHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT").field("Output", &self.Output).field("DXVA2DecodeHandle", &self.DXVA2DecodeHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DeviceHandle == other.DeviceHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.NumEncryptionGuids == other.NumEncryptionGuids
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT").field("Output", &self.Output).field("NumEncryptionGuids", &self.NumEncryptionGuids).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.EncryptionGuidIndex == other.EncryptionGuidIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT").field("Input", &self.Input).field("EncryptionGuidIndex", &self.EncryptionGuidIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.EncryptionGuidIndex == other.EncryptionGuidIndex && self.EncryptionGuid == other.EncryptionGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT").field("Output", &self.Output).field("EncryptionGuidIndex", &self.EncryptionGuidIndex).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.BusType == other.BusType && self.bAccessibleInContiguousBlocks == other.bAccessibleInContiguousBlocks && self.bAccessibleInNonContiguousBlocks == other.bAccessibleInNonContiguousBlocks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT").field("Output", &self.Output).field("BusType", &self.BusType).field("bAccessibleInContiguousBlocks", &self.bAccessibleInContiguousBlocks).field("bAccessibleInNonContiguousBlocks", &self.bAccessibleInNonContiguousBlocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT").field("Input", &self.Input).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.NumOutputIDs == other.NumOutputIDs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("NumOutputIDs", &self.NumOutputIDs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.OutputIDIndex == other.OutputIDIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT").field("Input", &self.Input).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("OutputIDIndex", &self.OutputIDIndex).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.NumRestrictedSharedResourceProcesses == other.NumRestrictedSharedResourceProcesses
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT").field("Output", &self.Output).field("NumRestrictedSharedResourceProcesses", &self.NumRestrictedSharedResourceProcesses).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.ProcessIndex == other.ProcessIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT").field("Input", &self.Input).field("ProcessIndex", &self.ProcessIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.ProcessIndex == other.ProcessIndex && self.ProcessIdentifer == other.ProcessIdentifer && self.ProcessHandle == other.ProcessHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT").field("Output", &self.Output).field("ProcessIndex", &self.ProcessIndex).field("ProcessIdentifer", &self.ProcessIdentifer).field("ProcessHandle", &self.ProcessHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.EncryptionGuid == other.EncryptionGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT").field("Output", &self.Output).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.NumUnrestrictedProtectedSharedResources == other.NumUnrestrictedProtectedSharedResources
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT").field("Output", &self.Output).field("NumUnrestrictedProtectedSharedResources", &self.NumUnrestrictedProtectedSharedResources).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.QueryType == other.QueryType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERY_INPUT").field("QueryType", &self.QueryType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.omac == other.omac && self.QueryType == other.QueryType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber && self.ReturnCode == other.ReturnCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT").field("omac", &self.omac).field("QueryType", &self.QueryType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).field("ReturnCode", &self.ReturnCode).finish()
    }
}
impl ::core::default::Default for D3DBACKBUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DBACKBUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBACKBUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DBASISTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DBASISTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBASISTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DBLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DBLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBLEND").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DBLENDOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DBLENDOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBLENDOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DBOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DBOX {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Right == other.Right && self.Bottom == other.Bottom && self.Front == other.Front && self.Back == other.Back
    }
}
impl ::core::cmp::Eq for D3DBOX {}
impl ::core::fmt::Debug for D3DBOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DBOX").field("Left", &self.Left).field("Top", &self.Top).field("Right", &self.Right).field("Bottom", &self.Bottom).field("Front", &self.Front).field("Back", &self.Back).finish()
    }
}
impl ::core::default::Default for D3DBUSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DBUSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DBUSTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DCAPS9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DCAPS9 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceType == other.DeviceType
            && self.AdapterOrdinal == other.AdapterOrdinal
            && self.Caps == other.Caps
            && self.Caps2 == other.Caps2
            && self.Caps3 == other.Caps3
            && self.PresentationIntervals == other.PresentationIntervals
            && self.CursorCaps == other.CursorCaps
            && self.DevCaps == other.DevCaps
            && self.PrimitiveMiscCaps == other.PrimitiveMiscCaps
            && self.RasterCaps == other.RasterCaps
            && self.ZCmpCaps == other.ZCmpCaps
            && self.SrcBlendCaps == other.SrcBlendCaps
            && self.DestBlendCaps == other.DestBlendCaps
            && self.AlphaCmpCaps == other.AlphaCmpCaps
            && self.ShadeCaps == other.ShadeCaps
            && self.TextureCaps == other.TextureCaps
            && self.TextureFilterCaps == other.TextureFilterCaps
            && self.CubeTextureFilterCaps == other.CubeTextureFilterCaps
            && self.VolumeTextureFilterCaps == other.VolumeTextureFilterCaps
            && self.TextureAddressCaps == other.TextureAddressCaps
            && self.VolumeTextureAddressCaps == other.VolumeTextureAddressCaps
            && self.LineCaps == other.LineCaps
            && self.MaxTextureWidth == other.MaxTextureWidth
            && self.MaxTextureHeight == other.MaxTextureHeight
            && self.MaxVolumeExtent == other.MaxVolumeExtent
            && self.MaxTextureRepeat == other.MaxTextureRepeat
            && self.MaxTextureAspectRatio == other.MaxTextureAspectRatio
            && self.MaxAnisotropy == other.MaxAnisotropy
            && self.MaxVertexW == other.MaxVertexW
            && self.GuardBandLeft == other.GuardBandLeft
            && self.GuardBandTop == other.GuardBandTop
            && self.GuardBandRight == other.GuardBandRight
            && self.GuardBandBottom == other.GuardBandBottom
            && self.ExtentsAdjust == other.ExtentsAdjust
            && self.StencilCaps == other.StencilCaps
            && self.FVFCaps == other.FVFCaps
            && self.TextureOpCaps == other.TextureOpCaps
            && self.MaxTextureBlendStages == other.MaxTextureBlendStages
            && self.MaxSimultaneousTextures == other.MaxSimultaneousTextures
            && self.VertexProcessingCaps == other.VertexProcessingCaps
            && self.MaxActiveLights == other.MaxActiveLights
            && self.MaxUserClipPlanes == other.MaxUserClipPlanes
            && self.MaxVertexBlendMatrices == other.MaxVertexBlendMatrices
            && self.MaxVertexBlendMatrixIndex == other.MaxVertexBlendMatrixIndex
            && self.MaxPointSize == other.MaxPointSize
            && self.MaxPrimitiveCount == other.MaxPrimitiveCount
            && self.MaxVertexIndex == other.MaxVertexIndex
            && self.MaxStreams == other.MaxStreams
            && self.MaxStreamStride == other.MaxStreamStride
            && self.VertexShaderVersion == other.VertexShaderVersion
            && self.MaxVertexShaderConst == other.MaxVertexShaderConst
            && self.PixelShaderVersion == other.PixelShaderVersion
            && self.PixelShader1xMaxValue == other.PixelShader1xMaxValue
            && self.DevCaps2 == other.DevCaps2
            && self.MaxNpatchTessellationLevel == other.MaxNpatchTessellationLevel
            && self.Reserved5 == other.Reserved5
            && self.MasterAdapterOrdinal == other.MasterAdapterOrdinal
            && self.AdapterOrdinalInGroup == other.AdapterOrdinalInGroup
            && self.NumberOfAdaptersInGroup == other.NumberOfAdaptersInGroup
            && self.DeclTypes == other.DeclTypes
            && self.NumSimultaneousRTs == other.NumSimultaneousRTs
            && self.StretchRectFilterCaps == other.StretchRectFilterCaps
            && self.VS20Caps == other.VS20Caps
            && self.PS20Caps == other.PS20Caps
            && self.VertexTextureFilterCaps == other.VertexTextureFilterCaps
            && self.MaxVShaderInstructionsExecuted == other.MaxVShaderInstructionsExecuted
            && self.MaxPShaderInstructionsExecuted == other.MaxPShaderInstructionsExecuted
            && self.MaxVertexShader30InstructionSlots == other.MaxVertexShader30InstructionSlots
            && self.MaxPixelShader30InstructionSlots == other.MaxPixelShader30InstructionSlots
    }
}
impl ::core::cmp::Eq for D3DCAPS9 {}
impl ::core::fmt::Debug for D3DCAPS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCAPS9")
            .field("DeviceType", &self.DeviceType)
            .field("AdapterOrdinal", &self.AdapterOrdinal)
            .field("Caps", &self.Caps)
            .field("Caps2", &self.Caps2)
            .field("Caps3", &self.Caps3)
            .field("PresentationIntervals", &self.PresentationIntervals)
            .field("CursorCaps", &self.CursorCaps)
            .field("DevCaps", &self.DevCaps)
            .field("PrimitiveMiscCaps", &self.PrimitiveMiscCaps)
            .field("RasterCaps", &self.RasterCaps)
            .field("ZCmpCaps", &self.ZCmpCaps)
            .field("SrcBlendCaps", &self.SrcBlendCaps)
            .field("DestBlendCaps", &self.DestBlendCaps)
            .field("AlphaCmpCaps", &self.AlphaCmpCaps)
            .field("ShadeCaps", &self.ShadeCaps)
            .field("TextureCaps", &self.TextureCaps)
            .field("TextureFilterCaps", &self.TextureFilterCaps)
            .field("CubeTextureFilterCaps", &self.CubeTextureFilterCaps)
            .field("VolumeTextureFilterCaps", &self.VolumeTextureFilterCaps)
            .field("TextureAddressCaps", &self.TextureAddressCaps)
            .field("VolumeTextureAddressCaps", &self.VolumeTextureAddressCaps)
            .field("LineCaps", &self.LineCaps)
            .field("MaxTextureWidth", &self.MaxTextureWidth)
            .field("MaxTextureHeight", &self.MaxTextureHeight)
            .field("MaxVolumeExtent", &self.MaxVolumeExtent)
            .field("MaxTextureRepeat", &self.MaxTextureRepeat)
            .field("MaxTextureAspectRatio", &self.MaxTextureAspectRatio)
            .field("MaxAnisotropy", &self.MaxAnisotropy)
            .field("MaxVertexW", &self.MaxVertexW)
            .field("GuardBandLeft", &self.GuardBandLeft)
            .field("GuardBandTop", &self.GuardBandTop)
            .field("GuardBandRight", &self.GuardBandRight)
            .field("GuardBandBottom", &self.GuardBandBottom)
            .field("ExtentsAdjust", &self.ExtentsAdjust)
            .field("StencilCaps", &self.StencilCaps)
            .field("FVFCaps", &self.FVFCaps)
            .field("TextureOpCaps", &self.TextureOpCaps)
            .field("MaxTextureBlendStages", &self.MaxTextureBlendStages)
            .field("MaxSimultaneousTextures", &self.MaxSimultaneousTextures)
            .field("VertexProcessingCaps", &self.VertexProcessingCaps)
            .field("MaxActiveLights", &self.MaxActiveLights)
            .field("MaxUserClipPlanes", &self.MaxUserClipPlanes)
            .field("MaxVertexBlendMatrices", &self.MaxVertexBlendMatrices)
            .field("MaxVertexBlendMatrixIndex", &self.MaxVertexBlendMatrixIndex)
            .field("MaxPointSize", &self.MaxPointSize)
            .field("MaxPrimitiveCount", &self.MaxPrimitiveCount)
            .field("MaxVertexIndex", &self.MaxVertexIndex)
            .field("MaxStreams", &self.MaxStreams)
            .field("MaxStreamStride", &self.MaxStreamStride)
            .field("VertexShaderVersion", &self.VertexShaderVersion)
            .field("MaxVertexShaderConst", &self.MaxVertexShaderConst)
            .field("PixelShaderVersion", &self.PixelShaderVersion)
            .field("PixelShader1xMaxValue", &self.PixelShader1xMaxValue)
            .field("DevCaps2", &self.DevCaps2)
            .field("MaxNpatchTessellationLevel", &self.MaxNpatchTessellationLevel)
            .field("Reserved5", &self.Reserved5)
            .field("MasterAdapterOrdinal", &self.MasterAdapterOrdinal)
            .field("AdapterOrdinalInGroup", &self.AdapterOrdinalInGroup)
            .field("NumberOfAdaptersInGroup", &self.NumberOfAdaptersInGroup)
            .field("DeclTypes", &self.DeclTypes)
            .field("NumSimultaneousRTs", &self.NumSimultaneousRTs)
            .field("StretchRectFilterCaps", &self.StretchRectFilterCaps)
            .field("VS20Caps", &self.VS20Caps)
            .field("PS20Caps", &self.PS20Caps)
            .field("VertexTextureFilterCaps", &self.VertexTextureFilterCaps)
            .field("MaxVShaderInstructionsExecuted", &self.MaxVShaderInstructionsExecuted)
            .field("MaxPShaderInstructionsExecuted", &self.MaxPShaderInstructionsExecuted)
            .field("MaxVertexShader30InstructionSlots", &self.MaxVertexShader30InstructionSlots)
            .field("MaxPixelShader30InstructionSlots", &self.MaxPixelShader30InstructionSlots)
            .finish()
    }
}
impl ::core::default::Default for D3DCLIPSTATUS9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DCLIPSTATUS9 {
    fn eq(&self, other: &Self) -> bool {
        self.ClipUnion == other.ClipUnion && self.ClipIntersection == other.ClipIntersection
    }
}
impl ::core::cmp::Eq for D3DCLIPSTATUS9 {}
impl ::core::fmt::Debug for D3DCLIPSTATUS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCLIPSTATUS9").field("ClipUnion", &self.ClipUnion).field("ClipIntersection", &self.ClipIntersection).finish()
    }
}
impl ::core::default::Default for D3DCMPFUNC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DCMPFUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCMPFUNC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DCOLORVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DCOLORVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D3DCOLORVALUE {}
impl ::core::fmt::Debug for D3DCOLORVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCOLORVALUE").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::core::default::Default for D3DCOMPOSERECTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DCOMPOSERECTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for D3DCOMPOSERECTDESC {}
impl ::core::fmt::Debug for D3DCOMPOSERECTDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCOMPOSERECTDESC").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for D3DCOMPOSERECTDESTINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DCOMPOSERECTDESTINATION {
    fn eq(&self, other: &Self) -> bool {
        self.SrcRectIndex == other.SrcRectIndex && self.Reserved == other.Reserved && self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for D3DCOMPOSERECTDESTINATION {}
impl ::core::fmt::Debug for D3DCOMPOSERECTDESTINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DCOMPOSERECTDESTINATION").field("SrcRectIndex", &self.SrcRectIndex).field("Reserved", &self.Reserved).field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::default::Default for D3DCOMPOSERECTSOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DCOMPOSERECTSOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCOMPOSERECTSOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DCUBEMAP_FACES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DCUBEMAP_FACES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCUBEMAP_FACES").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DCULL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DCULL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCULL").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DDEBUGMONITORTOKENS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDEBUGMONITORTOKENS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDEBUGMONITORTOKENS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DDECLMETHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDECLMETHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDECLMETHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DDECLTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDECLTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDECLTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DDECLUSAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDECLUSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDECLUSAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DDEGREETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDEGREETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDEGREETYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DDEVICE_CREATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DDEVICE_CREATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.AdapterOrdinal == other.AdapterOrdinal && self.DeviceType == other.DeviceType && self.hFocusWindow == other.hFocusWindow && self.BehaviorFlags == other.BehaviorFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DDEVICE_CREATION_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DDEVICE_CREATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVICE_CREATION_PARAMETERS").field("AdapterOrdinal", &self.AdapterOrdinal).field("DeviceType", &self.DeviceType).field("hFocusWindow", &self.hFocusWindow).field("BehaviorFlags", &self.BehaviorFlags).finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.MaxBandwidthUtilized == other.MaxBandwidthUtilized && self.FrontEndUploadMemoryUtilizedPercent == other.FrontEndUploadMemoryUtilizedPercent && self.VertexRateUtilizedPercent == other.VertexRateUtilizedPercent && self.TriangleSetupRateUtilizedPercent == other.TriangleSetupRateUtilizedPercent && self.FillRateUtilizedPercent == other.FillRateUtilizedPercent
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9BANDWIDTHTIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9BANDWIDTHTIMINGS").field("MaxBandwidthUtilized", &self.MaxBandwidthUtilized).field("FrontEndUploadMemoryUtilizedPercent", &self.FrontEndUploadMemoryUtilizedPercent).field("VertexRateUtilizedPercent", &self.VertexRateUtilizedPercent).field("TriangleSetupRateUtilizedPercent", &self.TriangleSetupRateUtilizedPercent).field("FillRateUtilizedPercent", &self.FillRateUtilizedPercent).finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn eq(&self, other: &Self) -> bool {
        self.TextureCacheHitRate == other.TextureCacheHitRate && self.PostTransformVertexCacheHitRate == other.PostTransformVertexCacheHitRate
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9CACHEUTILIZATION {}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9CACHEUTILIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9CACHEUTILIZATION").field("TextureCacheHitRate", &self.TextureCacheHitRate).field("PostTransformVertexCacheHitRate", &self.PostTransformVertexCacheHitRate).finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.WaitingForGPUToUseApplicationResourceTimePercent == other.WaitingForGPUToUseApplicationResourceTimePercent && self.WaitingForGPUToAcceptMoreCommandsTimePercent == other.WaitingForGPUToAcceptMoreCommandsTimePercent && self.WaitingForGPUToStayWithinLatencyTimePercent == other.WaitingForGPUToStayWithinLatencyTimePercent && self.WaitingForGPUExclusiveResourceTimePercent == other.WaitingForGPUExclusiveResourceTimePercent && self.WaitingForGPUOtherTimePercent == other.WaitingForGPUOtherTimePercent
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9INTERFACETIMINGS {}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9INTERFACETIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9INTERFACETIMINGS")
            .field("WaitingForGPUToUseApplicationResourceTimePercent", &self.WaitingForGPUToUseApplicationResourceTimePercent)
            .field("WaitingForGPUToAcceptMoreCommandsTimePercent", &self.WaitingForGPUToAcceptMoreCommandsTimePercent)
            .field("WaitingForGPUToStayWithinLatencyTimePercent", &self.WaitingForGPUToStayWithinLatencyTimePercent)
            .field("WaitingForGPUExclusiveResourceTimePercent", &self.WaitingForGPUExclusiveResourceTimePercent)
            .field("WaitingForGPUOtherTimePercent", &self.WaitingForGPUOtherTimePercent)
            .finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.VertexProcessingTimePercent == other.VertexProcessingTimePercent && self.PixelProcessingTimePercent == other.PixelProcessingTimePercent && self.OtherGPUProcessingTimePercent == other.OtherGPUProcessingTimePercent && self.GPUIdleTimePercent == other.GPUIdleTimePercent
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9PIPELINETIMINGS {}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9PIPELINETIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9PIPELINETIMINGS").field("VertexProcessingTimePercent", &self.VertexProcessingTimePercent).field("PixelProcessingTimePercent", &self.PixelProcessingTimePercent).field("OtherGPUProcessingTimePercent", &self.OtherGPUProcessingTimePercent).field("GPUIdleTimePercent", &self.GPUIdleTimePercent).finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_D3D9STAGETIMINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3D9STAGETIMINGS {
    fn eq(&self, other: &Self) -> bool {
        self.MemoryProcessingPercent == other.MemoryProcessingPercent && self.ComputationProcessingPercent == other.ComputationProcessingPercent
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3D9STAGETIMINGS {}
impl ::core::fmt::Debug for D3DDEVINFO_D3D9STAGETIMINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3D9STAGETIMINGS").field("MemoryProcessingPercent", &self.MemoryProcessingPercent).field("ComputationProcessingPercent", &self.ComputationProcessingPercent).finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_D3DVERTEXSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_D3DVERTEXSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.NumRenderedTriangles == other.NumRenderedTriangles && self.NumExtraClippingTriangles == other.NumExtraClippingTriangles
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_D3DVERTEXSTATS {}
impl ::core::fmt::Debug for D3DDEVINFO_D3DVERTEXSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_D3DVERTEXSTATS").field("NumRenderedTriangles", &self.NumRenderedTriangles).field("NumExtraClippingTriangles", &self.NumExtraClippingTriangles).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DDEVINFO_RESOURCEMANAGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DDEVINFO_RESOURCEMANAGER {
    fn eq(&self, other: &Self) -> bool {
        self.stats == other.stats
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DDEVINFO_RESOURCEMANAGER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DDEVINFO_RESOURCEMANAGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_RESOURCEMANAGER").field("stats", &self.stats).finish()
    }
}
impl ::core::default::Default for D3DDEVINFO_VCACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDEVINFO_VCACHE {
    fn eq(&self, other: &Self) -> bool {
        self.Pattern == other.Pattern && self.OptMethod == other.OptMethod && self.CacheSize == other.CacheSize && self.MagicNumber == other.MagicNumber
    }
}
impl ::core::cmp::Eq for D3DDEVINFO_VCACHE {}
impl ::core::fmt::Debug for D3DDEVINFO_VCACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDEVINFO_VCACHE").field("Pattern", &self.Pattern).field("OptMethod", &self.OptMethod).field("CacheSize", &self.CacheSize).field("MagicNumber", &self.MagicNumber).finish()
    }
}
impl ::core::default::Default for D3DDEVTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDEVTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDEVTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DDISPLAYMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDISPLAYMODE {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format
    }
}
impl ::core::cmp::Eq for D3DDISPLAYMODE {}
impl ::core::fmt::Debug for D3DDISPLAYMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDISPLAYMODE").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).finish()
    }
}
impl ::core::default::Default for D3DDISPLAYMODEEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDISPLAYMODEEX {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format && self.ScanLineOrdering == other.ScanLineOrdering
    }
}
impl ::core::cmp::Eq for D3DDISPLAYMODEEX {}
impl ::core::fmt::Debug for D3DDISPLAYMODEEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDISPLAYMODEEX").field("Size", &self.Size).field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanLineOrdering", &self.ScanLineOrdering).finish()
    }
}
impl ::core::default::Default for D3DDISPLAYMODEFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DDISPLAYMODEFILTER {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Format == other.Format && self.ScanLineOrdering == other.ScanLineOrdering
    }
}
impl ::core::cmp::Eq for D3DDISPLAYMODEFILTER {}
impl ::core::fmt::Debug for D3DDISPLAYMODEFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DDISPLAYMODEFILTER").field("Size", &self.Size).field("Format", &self.Format).field("ScanLineOrdering", &self.ScanLineOrdering).finish()
    }
}
impl ::core::default::Default for D3DDISPLAYROTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DDISPLAYROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DDISPLAYROTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DENCRYPTED_BLOCK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DENCRYPTED_BLOCK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumEncryptedBytesAtBeginning == other.NumEncryptedBytesAtBeginning && self.NumBytesInSkipPattern == other.NumBytesInSkipPattern && self.NumBytesInEncryptPattern == other.NumBytesInEncryptPattern
    }
}
impl ::core::cmp::Eq for D3DENCRYPTED_BLOCK_INFO {}
impl ::core::fmt::Debug for D3DENCRYPTED_BLOCK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DENCRYPTED_BLOCK_INFO").field("NumEncryptedBytesAtBeginning", &self.NumEncryptedBytesAtBeginning).field("NumBytesInSkipPattern", &self.NumBytesInSkipPattern).field("NumBytesInEncryptPattern", &self.NumBytesInEncryptPattern).finish()
    }
}
impl ::core::default::Default for D3DFILLMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DFILLMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DFILLMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DFOGMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DFOGMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DFOGMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DFORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DGAMMARAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DGAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for D3DGAMMARAMP {}
impl ::core::fmt::Debug for D3DGAMMARAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DGAMMARAMP").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::core::default::Default for D3DINDEXBUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DINDEXBUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Type == other.Type && self.Usage == other.Usage && self.Pool == other.Pool && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for D3DINDEXBUFFER_DESC {}
impl ::core::fmt::Debug for D3DINDEXBUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DINDEXBUFFER_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Size", &self.Size).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3DLIGHT9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3DLIGHT9 {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Diffuse == other.Diffuse && self.Specular == other.Specular && self.Ambient == other.Ambient && self.Position == other.Position && self.Direction == other.Direction && self.Range == other.Range && self.Falloff == other.Falloff && self.Attenuation0 == other.Attenuation0 && self.Attenuation1 == other.Attenuation1 && self.Attenuation2 == other.Attenuation2 && self.Theta == other.Theta && self.Phi == other.Phi
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3DLIGHT9 {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3DLIGHT9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DLIGHT9").field("Type", &self.Type).field("Diffuse", &self.Diffuse).field("Specular", &self.Specular).field("Ambient", &self.Ambient).field("Position", &self.Position).field("Direction", &self.Direction).field("Range", &self.Range).field("Falloff", &self.Falloff).field("Attenuation0", &self.Attenuation0).field("Attenuation1", &self.Attenuation1).field("Attenuation2", &self.Attenuation2).field("Theta", &self.Theta).field("Phi", &self.Phi).finish()
    }
}
impl ::core::default::Default for D3DLIGHTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DLIGHTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DLIGHTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DLOCKED_BOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DLOCKED_BOX {
    fn eq(&self, other: &Self) -> bool {
        self.RowPitch == other.RowPitch && self.SlicePitch == other.SlicePitch && self.pBits == other.pBits
    }
}
impl ::core::cmp::Eq for D3DLOCKED_BOX {}
impl ::core::fmt::Debug for D3DLOCKED_BOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DLOCKED_BOX").field("RowPitch", &self.RowPitch).field("SlicePitch", &self.SlicePitch).field("pBits", &self.pBits).finish()
    }
}
impl ::core::default::Default for D3DLOCKED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DLOCKED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Pitch == other.Pitch && self.pBits == other.pBits
    }
}
impl ::core::cmp::Eq for D3DLOCKED_RECT {}
impl ::core::fmt::Debug for D3DLOCKED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DLOCKED_RECT").field("Pitch", &self.Pitch).field("pBits", &self.pBits).finish()
    }
}
impl ::core::default::Default for D3DMATERIAL9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DMATERIAL9 {
    fn eq(&self, other: &Self) -> bool {
        self.Diffuse == other.Diffuse && self.Ambient == other.Ambient && self.Specular == other.Specular && self.Emissive == other.Emissive && self.Power == other.Power
    }
}
impl ::core::cmp::Eq for D3DMATERIAL9 {}
impl ::core::fmt::Debug for D3DMATERIAL9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DMATERIAL9").field("Diffuse", &self.Diffuse).field("Ambient", &self.Ambient).field("Specular", &self.Specular).field("Emissive", &self.Emissive).field("Power", &self.Power).finish()
    }
}
impl ::core::default::Default for D3DMATERIALCOLORSOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DMATERIALCOLORSOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DMATERIALCOLORSOURCE").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DMEMORYPRESSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DMEMORYPRESSURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3DMULTISAMPLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DMULTISAMPLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DMULTISAMPLE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DPATCHEDGESTYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DPATCHEDGESTYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DPATCHEDGESTYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DPOOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DPOOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DPOOL").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for D3DPRESENTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for D3DPRESENTSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DPRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DPRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.BackBufferWidth == other.BackBufferWidth && self.BackBufferHeight == other.BackBufferHeight && self.BackBufferFormat == other.BackBufferFormat && self.BackBufferCount == other.BackBufferCount && self.MultiSampleType == other.MultiSampleType && self.MultiSampleQuality == other.MultiSampleQuality && self.SwapEffect == other.SwapEffect && self.hDeviceWindow == other.hDeviceWindow && self.Windowed == other.Windowed && self.EnableAutoDepthStencil == other.EnableAutoDepthStencil && self.AutoDepthStencilFormat == other.AutoDepthStencilFormat && self.Flags == other.Flags && self.FullScreen_RefreshRateInHz == other.FullScreen_RefreshRateInHz && self.PresentationInterval == other.PresentationInterval
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DPRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DPRESENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DPRESENT_PARAMETERS")
            .field("BackBufferWidth", &self.BackBufferWidth)
            .field("BackBufferHeight", &self.BackBufferHeight)
            .field("BackBufferFormat", &self.BackBufferFormat)
            .field("BackBufferCount", &self.BackBufferCount)
            .field("MultiSampleType", &self.MultiSampleType)
            .field("MultiSampleQuality", &self.MultiSampleQuality)
            .field("SwapEffect", &self.SwapEffect)
            .field("hDeviceWindow", &self.hDeviceWindow)
            .field("Windowed", &self.Windowed)
            .field("EnableAutoDepthStencil", &self.EnableAutoDepthStencil)
            .field("AutoDepthStencilFormat", &self.AutoDepthStencilFormat)
            .field("Flags", &self.Flags)
            .field("FullScreen_RefreshRateInHz", &self.FullScreen_RefreshRateInHz)
            .field("PresentationInterval", &self.PresentationInterval)
            .finish()
    }
}
impl ::core::default::Default for D3DPRIMITIVETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DPRIMITIVETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DPRIMITIVETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DPSHADERCAPS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DPSHADERCAPS2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Caps == other.Caps && self.DynamicFlowControlDepth == other.DynamicFlowControlDepth && self.NumTemps == other.NumTemps && self.StaticFlowControlDepth == other.StaticFlowControlDepth && self.NumInstructionSlots == other.NumInstructionSlots
    }
}
impl ::core::cmp::Eq for D3DPSHADERCAPS2_0 {}
impl ::core::fmt::Debug for D3DPSHADERCAPS2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DPSHADERCAPS2_0").field("Caps", &self.Caps).field("DynamicFlowControlDepth", &self.DynamicFlowControlDepth).field("NumTemps", &self.NumTemps).field("StaticFlowControlDepth", &self.StaticFlowControlDepth).field("NumInstructionSlots", &self.NumInstructionSlots).finish()
    }
}
impl ::core::default::Default for D3DQUERYTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DQUERYTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DQUERYTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for D3DRANGE {}
impl ::core::fmt::Debug for D3DRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRANGE").field("Offset", &self.Offset).field("Size", &self.Size).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DRASTER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DRASTER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.InVBlank == other.InVBlank && self.ScanLine == other.ScanLine
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DRASTER_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DRASTER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRASTER_STATUS").field("InVBlank", &self.InVBlank).field("ScanLine", &self.ScanLine).finish()
    }
}
impl ::core::default::Default for D3DRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DRECT {
    fn eq(&self, other: &Self) -> bool {
        self.x1 == other.x1 && self.y1 == other.y1 && self.x2 == other.x2 && self.y2 == other.y2
    }
}
impl ::core::cmp::Eq for D3DRECT {}
impl ::core::fmt::Debug for D3DRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRECT").field("x1", &self.x1).field("y1", &self.y1).field("x2", &self.x2).field("y2", &self.y2).finish()
    }
}
impl ::core::default::Default for D3DRECTPATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DRECTPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StartVertexOffsetWidth == other.StartVertexOffsetWidth && self.StartVertexOffsetHeight == other.StartVertexOffsetHeight && self.Width == other.Width && self.Height == other.Height && self.Stride == other.Stride && self.Basis == other.Basis && self.Degree == other.Degree
    }
}
impl ::core::cmp::Eq for D3DRECTPATCH_INFO {}
impl ::core::fmt::Debug for D3DRECTPATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRECTPATCH_INFO").field("StartVertexOffsetWidth", &self.StartVertexOffsetWidth).field("StartVertexOffsetHeight", &self.StartVertexOffsetHeight).field("Width", &self.Width).field("Height", &self.Height).field("Stride", &self.Stride).field("Basis", &self.Basis).field("Degree", &self.Degree).finish()
    }
}
impl ::core::default::Default for D3DRENDERSTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DRENDERSTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DRENDERSTATETYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3DRESOURCESTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3DRESOURCESTATS {
    fn eq(&self, other: &Self) -> bool {
        self.bThrashing == other.bThrashing && self.ApproxBytesDownloaded == other.ApproxBytesDownloaded && self.NumEvicts == other.NumEvicts && self.NumVidCreates == other.NumVidCreates && self.LastPri == other.LastPri && self.NumUsed == other.NumUsed && self.NumUsedInVidMem == other.NumUsedInVidMem && self.WorkingSet == other.WorkingSet && self.WorkingSetBytes == other.WorkingSetBytes && self.TotalManaged == other.TotalManaged && self.TotalBytes == other.TotalBytes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3DRESOURCESTATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3DRESOURCESTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DRESOURCESTATS")
            .field("bThrashing", &self.bThrashing)
            .field("ApproxBytesDownloaded", &self.ApproxBytesDownloaded)
            .field("NumEvicts", &self.NumEvicts)
            .field("NumVidCreates", &self.NumVidCreates)
            .field("LastPri", &self.LastPri)
            .field("NumUsed", &self.NumUsed)
            .field("NumUsedInVidMem", &self.NumUsedInVidMem)
            .field("WorkingSet", &self.WorkingSet)
            .field("WorkingSetBytes", &self.WorkingSetBytes)
            .field("TotalManaged", &self.TotalManaged)
            .field("TotalBytes", &self.TotalBytes)
            .finish()
    }
}
impl ::core::default::Default for D3DRESOURCETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DRESOURCETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DRESOURCETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSAMPLERSTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSAMPLERSTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSAMPLERSTATETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSAMPLER_TEXTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSAMPLER_TEXTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSAMPLER_TEXTURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSCANLINEORDERING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSCANLINEORDERING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSCANLINEORDERING").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADEMODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADEMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADEMODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_ADDRESSMODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_ADDRESSMODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_ADDRESSMODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_COMPARISON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_COMPARISON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_COMPARISON").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_INSTRUCTION_OPCODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_INSTRUCTION_OPCODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_MIN_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_MIN_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_MIN_PRECISION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_MISCTYPE_OFFSETS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_MISCTYPE_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_MISCTYPE_OFFSETS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_PARAM_REGISTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_PARAM_REGISTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_PARAM_REGISTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSHADER_PARAM_SRCMOD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSHADER_PARAM_SRCMOD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSHADER_PARAM_SRCMOD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSTATEBLOCKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSTATEBLOCKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSTATEBLOCKTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSTENCILOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSTENCILOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSTENCILOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DSURFACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DSURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Type == other.Type && self.Usage == other.Usage && self.Pool == other.Pool && self.MultiSampleType == other.MultiSampleType && self.MultiSampleQuality == other.MultiSampleQuality && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for D3DSURFACE_DESC {}
impl ::core::fmt::Debug for D3DSURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DSURFACE_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("MultiSampleType", &self.MultiSampleType).field("MultiSampleQuality", &self.MultiSampleQuality).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for D3DSWAPEFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DSWAPEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DSWAPEFFECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTEXTUREADDRESS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DTEXTUREADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTUREADDRESS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTEXTUREFILTERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DTEXTUREFILTERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTUREFILTERTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTEXTUREOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DTEXTUREOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTUREOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTEXTURESTAGESTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DTEXTURESTAGESTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTURESTAGESTATETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTEXTURETRANSFORMFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DTEXTURETRANSFORMFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTEXTURETRANSFORMFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTRANSFORMSTATETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DTRANSFORMSTATETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DTRANSFORMSTATETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DTRIPATCH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DTRIPATCH_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StartVertexOffset == other.StartVertexOffset && self.NumVertices == other.NumVertices && self.Basis == other.Basis && self.Degree == other.Degree
    }
}
impl ::core::cmp::Eq for D3DTRIPATCH_INFO {}
impl ::core::fmt::Debug for D3DTRIPATCH_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DTRIPATCH_INFO").field("StartVertexOffset", &self.StartVertexOffset).field("NumVertices", &self.NumVertices).field("Basis", &self.Basis).field("Degree", &self.Degree).finish()
    }
}
impl ::core::default::Default for D3DVERTEXBLENDFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DVERTEXBLENDFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DVERTEXBLENDFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DVERTEXBUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DVERTEXBUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Type == other.Type && self.Usage == other.Usage && self.Pool == other.Pool && self.Size == other.Size && self.FVF == other.FVF
    }
}
impl ::core::cmp::Eq for D3DVERTEXBUFFER_DESC {}
impl ::core::fmt::Debug for D3DVERTEXBUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVERTEXBUFFER_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Size", &self.Size).field("FVF", &self.FVF).finish()
    }
}
impl ::core::default::Default for D3DVERTEXELEMENT9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DVERTEXELEMENT9 {
    fn eq(&self, other: &Self) -> bool {
        self.Stream == other.Stream && self.Offset == other.Offset && self.Type == other.Type && self.Method == other.Method && self.Usage == other.Usage && self.UsageIndex == other.UsageIndex
    }
}
impl ::core::cmp::Eq for D3DVERTEXELEMENT9 {}
impl ::core::fmt::Debug for D3DVERTEXELEMENT9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVERTEXELEMENT9").field("Stream", &self.Stream).field("Offset", &self.Offset).field("Type", &self.Type).field("Method", &self.Method).field("Usage", &self.Usage).field("UsageIndex", &self.UsageIndex).finish()
    }
}
impl ::core::default::Default for D3DVIEWPORT9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DVIEWPORT9 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height && self.MinZ == other.MinZ && self.MaxZ == other.MaxZ
    }
}
impl ::core::cmp::Eq for D3DVIEWPORT9 {}
impl ::core::fmt::Debug for D3DVIEWPORT9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVIEWPORT9").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).field("MinZ", &self.MinZ).field("MaxZ", &self.MaxZ).finish()
    }
}
impl ::core::default::Default for D3DVOLUME_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DVOLUME_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Type == other.Type && self.Usage == other.Usage && self.Pool == other.Pool && self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth
    }
}
impl ::core::cmp::Eq for D3DVOLUME_DESC {}
impl ::core::fmt::Debug for D3DVOLUME_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVOLUME_DESC").field("Format", &self.Format).field("Type", &self.Type).field("Usage", &self.Usage).field("Pool", &self.Pool).field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).finish()
    }
}
impl ::core::default::Default for D3DVSHADERCAPS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DVSHADERCAPS2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Caps == other.Caps && self.DynamicFlowControlDepth == other.DynamicFlowControlDepth && self.NumTemps == other.NumTemps && self.StaticFlowControlDepth == other.StaticFlowControlDepth
    }
}
impl ::core::cmp::Eq for D3DVSHADERCAPS2_0 {}
impl ::core::fmt::Debug for D3DVSHADERCAPS2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVSHADERCAPS2_0").field("Caps", &self.Caps).field("DynamicFlowControlDepth", &self.DynamicFlowControlDepth).field("NumTemps", &self.NumTemps).field("StaticFlowControlDepth", &self.StaticFlowControlDepth).finish()
    }
}
impl ::core::default::Default for D3DVS_ADDRESSMODE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DVS_ADDRESSMODE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DVS_ADDRESSMODE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DVS_RASTOUT_OFFSETS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DVS_RASTOUT_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DVS_RASTOUT_OFFSETS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DZBUFFERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DZBUFFERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DZBUFFERTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_OMAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D_OMAC {
    fn eq(&self, other: &Self) -> bool {
        self.Omac == other.Omac
    }
}
impl ::core::cmp::Eq for D3D_OMAC {}
impl ::core::fmt::Debug for D3D_OMAC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D_OMAC").field("Omac", &self.Omac).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3D9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3D9 {}
impl ::core::fmt::Debug for IDirect3D9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3D9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3D9Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3D9Ex {}
impl ::core::fmt::Debug for IDirect3D9Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3D9Ex").field(&self.0).finish()
    }
}
impl IDirect3D9Ex {
    pub unsafe fn RegisterSoftwareDevice(&self, pinitializefunction: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RegisterSoftwareDevice)(::windows::core::Vtable::as_raw(self), pinitializefunction).ok()
    }
    pub unsafe fn GetAdapterCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetAdapterCount)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterIdentifier(&self, adapter: u32, flags: u32, pidentifier: *mut D3DADAPTER_IDENTIFIER9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAdapterIdentifier)(::windows::core::Vtable::as_raw(self), adapter, flags, pidentifier).ok()
    }
    pub unsafe fn GetAdapterModeCount(&self, adapter: u32, format: D3DFORMAT) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetAdapterModeCount)(::windows::core::Vtable::as_raw(self), adapter, format)
    }
    pub unsafe fn EnumAdapterModes(&self, adapter: u32, format: D3DFORMAT, mode: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumAdapterModes)(::windows::core::Vtable::as_raw(self), adapter, format, mode, pmode).ok()
    }
    pub unsafe fn GetAdapterDisplayMode(&self, adapter: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAdapterDisplayMode)(::windows::core::Vtable::as_raw(self), adapter, pmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceType<P0>(&self, adapter: u32, devtype: D3DDEVTYPE, adapterformat: D3DFORMAT, backbufferformat: D3DFORMAT, bwindowed: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CheckDeviceType)(::windows::core::Vtable::as_raw(self), adapter, devtype, adapterformat, backbufferformat, bwindowed.into()).ok()
    }
    pub unsafe fn CheckDeviceFormat(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, usage: u32, rtype: D3DRESOURCETYPE, checkformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckDeviceFormat)(::windows::core::Vtable::as_raw(self), adapter, devicetype, adapterformat, usage, rtype, checkformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceMultiSampleType<P0>(&self, adapter: u32, devicetype: D3DDEVTYPE, surfaceformat: D3DFORMAT, windowed: P0, multisampletype: D3DMULTISAMPLE_TYPE, pqualitylevels: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CheckDeviceMultiSampleType)(::windows::core::Vtable::as_raw(self), adapter, devicetype, surfaceformat, windowed.into(), multisampletype, pqualitylevels).ok()
    }
    pub unsafe fn CheckDepthStencilMatch(&self, adapter: u32, devicetype: D3DDEVTYPE, adapterformat: D3DFORMAT, rendertargetformat: D3DFORMAT, depthstencilformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckDepthStencilMatch)(::windows::core::Vtable::as_raw(self), adapter, devicetype, adapterformat, rendertargetformat, depthstencilformat).ok()
    }
    pub unsafe fn CheckDeviceFormatConversion(&self, adapter: u32, devicetype: D3DDEVTYPE, sourceformat: D3DFORMAT, targetformat: D3DFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckDeviceFormatConversion)(::windows::core::Vtable::as_raw(self), adapter, devicetype, sourceformat, targetformat).ok()
    }
    pub unsafe fn GetDeviceCaps(&self, adapter: u32, devicetype: D3DDEVTYPE, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceCaps)(::windows::core::Vtable::as_raw(self), adapter, devicetype, pcaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetAdapterMonitor(&self, adapter: u32) -> super::Gdi::HMONITOR {
        (::windows::core::Vtable::vtable(self).base__.GetAdapterMonitor)(::windows::core::Vtable::as_raw(self), adapter)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDevice<P0>(&self, adapter: u32, devicetype: D3DDEVTYPE, hfocuswindow: P0, behaviorflags: u32, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, ppreturneddeviceinterface: *mut ::core::option::Option<IDirect3DDevice9>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDevice)(::windows::core::Vtable::as_raw(self), adapter, devicetype, hfocuswindow.into(), behaviorflags, ppresentationparameters, ::core::mem::transmute(ppreturneddeviceinterface)).ok()
    }
}
impl ::core::cmp::PartialEq for IDirect3DBaseTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DBaseTexture9 {}
impl ::core::fmt::Debug for IDirect3DBaseTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DBaseTexture9").field(&self.0).finish()
    }
}
impl IDirect3DBaseTexture9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDirect3DCubeTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DCubeTexture9 {}
impl ::core::fmt::Debug for IDirect3DCubeTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DCubeTexture9").field(&self.0).finish()
    }
}
impl IDirect3DCubeTexture9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetLOD)(::windows::core::Vtable::as_raw(self), lodnew)
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLOD)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLevelCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAutoGenFilterType)(::windows::core::Vtable::as_raw(self), filtertype).ok()
    }
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        (::windows::core::Vtable::vtable(self).base__.GetAutoGenFilterType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Vtable::vtable(self).base__.GenerateMipSubLevels)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9 {}
impl ::core::fmt::Debug for IDirect3DDevice9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice9Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice9Ex {}
impl ::core::fmt::Debug for IDirect3DDevice9Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice9Ex").field(&self.0).finish()
    }
}
impl IDirect3DDevice9Ex {
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TestCooperativeLevel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetAvailableTextureMem(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetAvailableTextureMem)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EvictManagedResources(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EvictManagedResources)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDirect3D(&self) -> ::windows::core::Result<IDirect3D9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDirect3D)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCaps(&self, pcaps: *mut D3DCAPS9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceCaps)(::windows::core::Vtable::as_raw(self), pcaps).ok()
    }
    pub unsafe fn GetDisplayMode(&self, iswapchain: u32, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDisplayMode)(::windows::core::Vtable::as_raw(self), iswapchain, pmode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreationParameters(&self, pparameters: *mut D3DDEVICE_CREATION_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCreationParameters)(::windows::core::Vtable::as_raw(self), pparameters).ok()
    }
    pub unsafe fn SetCursorProperties<P0>(&self, xhotspot: u32, yhotspot: u32, pcursorbitmap: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCursorProperties)(::windows::core::Vtable::as_raw(self), xhotspot, yhotspot, pcursorbitmap.into().abi()).ok()
    }
    pub unsafe fn SetCursorPosition(&self, x: i32, y: i32, flags: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetCursorPosition)(::windows::core::Vtable::as_raw(self), x, y, flags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowCursor<P0>(&self, bshow: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShowCursor)(::windows::core::Vtable::as_raw(self), bshow.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAdditionalSwapChain(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS, pswapchain: *mut ::core::option::Option<IDirect3DSwapChain9>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateAdditionalSwapChain)(::windows::core::Vtable::as_raw(self), ppresentationparameters, ::core::mem::transmute(pswapchain)).ok()
    }
    pub unsafe fn GetSwapChain(&self, iswapchain: u32) -> ::windows::core::Result<IDirect3DSwapChain9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSwapChain)(::windows::core::Vtable::as_raw(self), iswapchain, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNumberOfSwapChains(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNumberOfSwapChains)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Reset(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), ppresentationparameters).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Present<P0>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: P0, pdirtyregion: *const super::Gdi::RGNDATA) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Present)(::windows::core::Vtable::as_raw(self), psourcerect, pdestrect, hdestwindowoverride.into(), pdirtyregion).ok()
    }
    pub unsafe fn GetBackBuffer(&self, iswapchain: u32, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBackBuffer)(::windows::core::Vtable::as_raw(self), iswapchain, ibackbuffer, r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRasterStatus(&self, iswapchain: u32, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRasterStatus)(::windows::core::Vtable::as_raw(self), iswapchain, prasterstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDialogBoxMode<P0>(&self, benabledialogs: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDialogBoxMode)(::windows::core::Vtable::as_raw(self), benabledialogs.into()).ok()
    }
    pub unsafe fn SetGammaRamp(&self, iswapchain: u32, flags: u32, pramp: *const D3DGAMMARAMP) {
        (::windows::core::Vtable::vtable(self).base__.SetGammaRamp)(::windows::core::Vtable::as_raw(self), iswapchain, flags, pramp)
    }
    pub unsafe fn GetGammaRamp(&self, iswapchain: u32, pramp: *mut D3DGAMMARAMP) {
        (::windows::core::Vtable::vtable(self).base__.GetGammaRamp)(::windows::core::Vtable::as_raw(self), iswapchain, pramp)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTexture(&self, width: u32, height: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, pptexture: *mut ::core::option::Option<IDirect3DTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateTexture)(::windows::core::Vtable::as_raw(self), width, height, levels, usage, format, pool, ::core::mem::transmute(pptexture), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVolumeTexture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppvolumetexture: *mut ::core::option::Option<IDirect3DVolumeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateVolumeTexture)(::windows::core::Vtable::as_raw(self), width, height, depth, levels, usage, format, pool, ::core::mem::transmute(ppvolumetexture), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateCubeTexture(&self, edgelength: u32, levels: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppcubetexture: *mut ::core::option::Option<IDirect3DCubeTexture9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateCubeTexture)(::windows::core::Vtable::as_raw(self), edgelength, levels, usage, format, pool, ::core::mem::transmute(ppcubetexture), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateVertexBuffer(&self, length: u32, usage: u32, fvf: u32, pool: D3DPOOL, ppvertexbuffer: *mut ::core::option::Option<IDirect3DVertexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateVertexBuffer)(::windows::core::Vtable::as_raw(self), length, usage, fvf, pool, ::core::mem::transmute(ppvertexbuffer), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateIndexBuffer(&self, length: u32, usage: u32, format: D3DFORMAT, pool: D3DPOOL, ppindexbuffer: *mut ::core::option::Option<IDirect3DIndexBuffer9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateIndexBuffer)(::windows::core::Vtable::as_raw(self), length, usage, format, pool, ::core::mem::transmute(ppindexbuffer), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRenderTarget<P0>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, lockable: P0, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateRenderTarget)(::windows::core::Vtable::as_raw(self), width, height, format, multisample, multisamplequality, lockable.into(), ::core::mem::transmute(ppsurface), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilSurface<P0>(&self, width: u32, height: u32, format: D3DFORMAT, multisample: D3DMULTISAMPLE_TYPE, multisamplequality: u32, discard: P0, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDepthStencilSurface)(::windows::core::Vtable::as_raw(self), width, height, format, multisample, multisamplequality, discard.into(), ::core::mem::transmute(ppsurface), psharedhandle).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateSurface<P0, P1>(&self, psourcesurface: P0, psourcerect: *const super::super::Foundation::RECT, pdestinationsurface: P1, pdestpoint: *const super::super::Foundation::POINT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateSurface)(::windows::core::Vtable::as_raw(self), psourcesurface.into().abi(), psourcerect, pdestinationsurface.into().abi(), pdestpoint).ok()
    }
    pub unsafe fn UpdateTexture<P0, P1>(&self, psourcetexture: P0, pdestinationtexture: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DBaseTexture9>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirect3DBaseTexture9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateTexture)(::windows::core::Vtable::as_raw(self), psourcetexture.into().abi(), pdestinationtexture.into().abi()).ok()
    }
    pub unsafe fn GetRenderTargetData<P0, P1>(&self, prendertarget: P0, pdestsurface: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetRenderTargetData)(::windows::core::Vtable::as_raw(self), prendertarget.into().abi(), pdestsurface.into().abi()).ok()
    }
    pub unsafe fn GetFrontBufferData<P0>(&self, iswapchain: u32, pdestsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetFrontBufferData)(::windows::core::Vtable::as_raw(self), iswapchain, pdestsurface.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StretchRect<P0, P1>(&self, psourcesurface: P0, psourcerect: *const super::super::Foundation::RECT, pdestsurface: P1, pdestrect: *const super::super::Foundation::RECT, filter: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StretchRect)(::windows::core::Vtable::as_raw(self), psourcesurface.into().abi(), psourcerect, pdestsurface.into().abi(), pdestrect, filter).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ColorFill<P0>(&self, psurface: P0, prect: *const super::super::Foundation::RECT, color: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ColorFill)(::windows::core::Vtable::as_raw(self), psurface.into().abi(), prect, color).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateOffscreenPlainSurface(&self, width: u32, height: u32, format: D3DFORMAT, pool: D3DPOOL, ppsurface: *mut ::core::option::Option<IDirect3DSurface9>, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateOffscreenPlainSurface)(::windows::core::Vtable::as_raw(self), width, height, format, pool, ::core::mem::transmute(ppsurface), psharedhandle).ok()
    }
    pub unsafe fn SetRenderTarget<P0>(&self, rendertargetindex: u32, prendertarget: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetindex, prendertarget.into().abi()).ok()
    }
    pub unsafe fn GetRenderTarget(&self, rendertargetindex: u32) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRenderTarget)(::windows::core::Vtable::as_raw(self), rendertargetindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDepthStencilSurface<P0>(&self, pnewzstencil: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDepthStencilSurface)(::windows::core::Vtable::as_raw(self), pnewzstencil.into().abi()).ok()
    }
    pub unsafe fn GetDepthStencilSurface(&self) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDepthStencilSurface)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginScene(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginScene)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndScene(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndScene)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Clear(&self, count: u32, prects: *const D3DRECT, flags: u32, color: u32, z: f32, stencil: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Clear)(::windows::core::Vtable::as_raw(self), count, prects, flags, color, z, stencil).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTransform)(::windows::core::Vtable::as_raw(self), state, pmatrix).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn GetTransform(&self, state: D3DTRANSFORMSTATETYPE, pmatrix: *mut super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), state, pmatrix).ok()
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn MultiplyTransform(&self, param0: D3DTRANSFORMSTATETYPE, param1: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MultiplyTransform)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetViewport(&self, pviewport: *const D3DVIEWPORT9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewport)(::windows::core::Vtable::as_raw(self), pviewport).ok()
    }
    pub unsafe fn GetViewport(&self, pviewport: *mut D3DVIEWPORT9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetViewport)(::windows::core::Vtable::as_raw(self), pviewport).ok()
    }
    pub unsafe fn SetMaterial(&self, pmaterial: *const D3DMATERIAL9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaterial)(::windows::core::Vtable::as_raw(self), pmaterial).ok()
    }
    pub unsafe fn GetMaterial(&self, pmaterial: *mut D3DMATERIAL9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMaterial)(::windows::core::Vtable::as_raw(self), pmaterial).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetLight(&self, index: u32, param1: *const D3DLIGHT9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLight)(::windows::core::Vtable::as_raw(self), index, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetLight(&self, index: u32, param1: *mut D3DLIGHT9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLight)(::windows::core::Vtable::as_raw(self), index, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LightEnable<P0>(&self, index: u32, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LightEnable)(::windows::core::Vtable::as_raw(self), index, enable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLightEnable(&self, index: u32, penable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLightEnable)(::windows::core::Vtable::as_raw(self), index, penable).ok()
    }
    pub unsafe fn SetClipPlane(&self, index: u32, pplane: *const f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClipPlane)(::windows::core::Vtable::as_raw(self), index, pplane).ok()
    }
    pub unsafe fn GetClipPlane(&self, index: u32, pplane: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetClipPlane)(::windows::core::Vtable::as_raw(self), index, pplane).ok()
    }
    pub unsafe fn SetRenderState(&self, state: D3DRENDERSTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRenderState)(::windows::core::Vtable::as_raw(self), state, value).ok()
    }
    pub unsafe fn GetRenderState(&self, state: D3DRENDERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRenderState)(::windows::core::Vtable::as_raw(self), state, pvalue).ok()
    }
    pub unsafe fn CreateStateBlock(&self, r#type: D3DSTATEBLOCKTYPE) -> ::windows::core::Result<IDirect3DStateBlock9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStateBlock)(::windows::core::Vtable::as_raw(self), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginStateBlock(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginStateBlock)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn EndStateBlock(&self) -> ::windows::core::Result<IDirect3DStateBlock9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EndStateBlock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipStatus(&self, pclipstatus: *const D3DCLIPSTATUS9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClipStatus)(::windows::core::Vtable::as_raw(self), pclipstatus).ok()
    }
    pub unsafe fn GetClipStatus(&self, pclipstatus: *mut D3DCLIPSTATUS9) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetClipStatus)(::windows::core::Vtable::as_raw(self), pclipstatus).ok()
    }
    pub unsafe fn GetTexture(&self, stage: u32) -> ::windows::core::Result<IDirect3DBaseTexture9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTexture)(::windows::core::Vtable::as_raw(self), stage, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTexture<P0>(&self, stage: u32, ptexture: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DBaseTexture9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTexture)(::windows::core::Vtable::as_raw(self), stage, ptexture.into().abi()).ok()
    }
    pub unsafe fn GetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTextureStageState)(::windows::core::Vtable::as_raw(self), stage, r#type, pvalue).ok()
    }
    pub unsafe fn SetTextureStageState(&self, stage: u32, r#type: D3DTEXTURESTAGESTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTextureStageState)(::windows::core::Vtable::as_raw(self), stage, r#type, value).ok()
    }
    pub unsafe fn GetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSamplerState)(::windows::core::Vtable::as_raw(self), sampler, r#type, pvalue).ok()
    }
    pub unsafe fn SetSamplerState(&self, sampler: u32, r#type: D3DSAMPLERSTATETYPE, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSamplerState)(::windows::core::Vtable::as_raw(self), sampler, r#type, value).ok()
    }
    pub unsafe fn ValidateDevice(&self, pnumpasses: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ValidateDevice)(::windows::core::Vtable::as_raw(self), pnumpasses).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetPaletteEntries(&self, palettenumber: u32, pentries: *const super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPaletteEntries)(::windows::core::Vtable::as_raw(self), palettenumber, pentries).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetPaletteEntries(&self, palettenumber: u32, pentries: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPaletteEntries)(::windows::core::Vtable::as_raw(self), palettenumber, pentries).ok()
    }
    pub unsafe fn SetCurrentTexturePalette(&self, palettenumber: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCurrentTexturePalette)(::windows::core::Vtable::as_raw(self), palettenumber).ok()
    }
    pub unsafe fn GetCurrentTexturePalette(&self, palettenumber: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentTexturePalette)(::windows::core::Vtable::as_raw(self), palettenumber).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScissorRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetScissorRect)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetScissorRect(&self, prect: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetScissorRect)(::windows::core::Vtable::as_raw(self), prect).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSoftwareVertexProcessing<P0>(&self, bsoftware: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSoftwareVertexProcessing)(::windows::core::Vtable::as_raw(self), bsoftware.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSoftwareVertexProcessing(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetSoftwareVertexProcessing)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetNPatchMode(&self, nsegments: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNPatchMode)(::windows::core::Vtable::as_raw(self), nsegments).ok()
    }
    pub unsafe fn GetNPatchMode(&self) -> f32 {
        (::windows::core::Vtable::vtable(self).base__.GetNPatchMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn DrawPrimitive(&self, primitivetype: D3DPRIMITIVETYPE, startvertex: u32, primitivecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrawPrimitive)(::windows::core::Vtable::as_raw(self), primitivetype, startvertex, primitivecount).ok()
    }
    pub unsafe fn DrawIndexedPrimitive(&self, param0: D3DPRIMITIVETYPE, basevertexindex: i32, minvertexindex: u32, numvertices: u32, startindex: u32, primcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexedPrimitive)(::windows::core::Vtable::as_raw(self), param0, basevertexindex, minvertexindex, numvertices, startindex, primcount).ok()
    }
    pub unsafe fn DrawPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, primitivecount: u32, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrawPrimitiveUP)(::windows::core::Vtable::as_raw(self), primitivetype, primitivecount, pvertexstreamzerodata, vertexstreamzerostride).ok()
    }
    pub unsafe fn DrawIndexedPrimitiveUP(&self, primitivetype: D3DPRIMITIVETYPE, minvertexindex: u32, numvertices: u32, primitivecount: u32, pindexdata: *const ::core::ffi::c_void, indexdataformat: D3DFORMAT, pvertexstreamzerodata: *const ::core::ffi::c_void, vertexstreamzerostride: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexedPrimitiveUP)(::windows::core::Vtable::as_raw(self), primitivetype, minvertexindex, numvertices, primitivecount, pindexdata, indexdataformat, pvertexstreamzerodata, vertexstreamzerostride).ok()
    }
    pub unsafe fn ProcessVertices<P0, P1>(&self, srcstartindex: u32, destindex: u32, vertexcount: u32, pdestbuffer: P0, pvertexdecl: P1, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DVertexBuffer9>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDirect3DVertexDeclaration9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ProcessVertices)(::windows::core::Vtable::as_raw(self), srcstartindex, destindex, vertexcount, pdestbuffer.into().abi(), pvertexdecl.into().abi(), flags).ok()
    }
    pub unsafe fn CreateVertexDeclaration(&self, pvertexelements: *const D3DVERTEXELEMENT9) -> ::windows::core::Result<IDirect3DVertexDeclaration9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVertexDeclaration)(::windows::core::Vtable::as_raw(self), pvertexelements, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVertexDeclaration<P0>(&self, pdecl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DVertexDeclaration9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVertexDeclaration)(::windows::core::Vtable::as_raw(self), pdecl.into().abi()).ok()
    }
    pub unsafe fn GetVertexDeclaration(&self) -> ::windows::core::Result<IDirect3DVertexDeclaration9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVertexDeclaration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFVF(&self, fvf: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFVF)(::windows::core::Vtable::as_raw(self), fvf).ok()
    }
    pub unsafe fn GetFVF(&self, pfvf: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFVF)(::windows::core::Vtable::as_raw(self), pfvf).ok()
    }
    pub unsafe fn CreateVertexShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DVertexShader9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), pfunction, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVertexShader<P0>(&self, pshader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DVertexShader9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetVertexShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi()).ok()
    }
    pub unsafe fn GetVertexShader(&self) -> ::windows::core::Result<IDirect3DVertexShader9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVertexShader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVertexShaderConstantF)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4fcount).ok()
    }
    pub unsafe fn GetVertexShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVertexShaderConstantF)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4fcount).ok()
    }
    pub unsafe fn SetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVertexShaderConstantI)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4icount).ok()
    }
    pub unsafe fn GetVertexShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVertexShaderConstantI)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4icount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVertexShaderConstantB)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, boolcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVertexShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVertexShaderConstantB)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, boolcount).ok()
    }
    pub unsafe fn SetStreamSource<P0>(&self, streamnumber: u32, pstreamdata: P0, offsetinbytes: u32, stride: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DVertexBuffer9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStreamSource)(::windows::core::Vtable::as_raw(self), streamnumber, pstreamdata.into().abi(), offsetinbytes, stride).ok()
    }
    pub unsafe fn GetStreamSource(&self, streamnumber: u32, ppstreamdata: *mut ::core::option::Option<IDirect3DVertexBuffer9>, poffsetinbytes: *mut u32, pstride: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStreamSource)(::windows::core::Vtable::as_raw(self), streamnumber, ::core::mem::transmute(ppstreamdata), poffsetinbytes, pstride).ok()
    }
    pub unsafe fn SetStreamSourceFreq(&self, streamnumber: u32, setting: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStreamSourceFreq)(::windows::core::Vtable::as_raw(self), streamnumber, setting).ok()
    }
    pub unsafe fn GetStreamSourceFreq(&self, streamnumber: u32, psetting: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStreamSourceFreq)(::windows::core::Vtable::as_raw(self), streamnumber, psetting).ok()
    }
    pub unsafe fn SetIndices<P0>(&self, pindexdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DIndexBuffer9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIndices)(::windows::core::Vtable::as_raw(self), pindexdata.into().abi()).ok()
    }
    pub unsafe fn GetIndices(&self) -> ::windows::core::Result<IDirect3DIndexBuffer9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIndices)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePixelShader(&self, pfunction: *const u32) -> ::windows::core::Result<IDirect3DPixelShader9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), pfunction, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPixelShader<P0>(&self, pshader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DPixelShader9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPixelShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi()).ok()
    }
    pub unsafe fn GetPixelShader(&self) -> ::windows::core::Result<IDirect3DPixelShader9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelShader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *const f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPixelShaderConstantF)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4fcount).ok()
    }
    pub unsafe fn GetPixelShaderConstantF(&self, startregister: u32, pconstantdata: *mut f32, vector4fcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPixelShaderConstantF)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4fcount).ok()
    }
    pub unsafe fn SetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *const i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPixelShaderConstantI)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4icount).ok()
    }
    pub unsafe fn GetPixelShaderConstantI(&self, startregister: u32, pconstantdata: *mut i32, vector4icount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPixelShaderConstantI)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, vector4icount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *const super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPixelShaderConstantB)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, boolcount).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPixelShaderConstantB(&self, startregister: u32, pconstantdata: *mut super::super::Foundation::BOOL, boolcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPixelShaderConstantB)(::windows::core::Vtable::as_raw(self), startregister, pconstantdata, boolcount).ok()
    }
    pub unsafe fn DrawRectPatch(&self, handle: u32, pnumsegs: *const f32, prectpatchinfo: *const D3DRECTPATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrawRectPatch)(::windows::core::Vtable::as_raw(self), handle, pnumsegs, prectpatchinfo).ok()
    }
    pub unsafe fn DrawTriPatch(&self, handle: u32, pnumsegs: *const f32, ptripatchinfo: *const D3DTRIPATCH_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DrawTriPatch)(::windows::core::Vtable::as_raw(self), handle, pnumsegs, ptripatchinfo).ok()
    }
    pub unsafe fn DeletePatch(&self, handle: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeletePatch)(::windows::core::Vtable::as_raw(self), handle).ok()
    }
    pub unsafe fn CreateQuery(&self, r#type: D3DQUERYTYPE) -> ::windows::core::Result<IDirect3DQuery9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateQuery)(::windows::core::Vtable::as_raw(self), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDirect3DIndexBuffer9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DIndexBuffer9 {}
impl ::core::fmt::Debug for IDirect3DIndexBuffer9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DIndexBuffer9").field(&self.0).finish()
    }
}
impl IDirect3DIndexBuffer9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDirect3DPixelShader9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DPixelShader9 {}
impl ::core::fmt::Debug for IDirect3DPixelShader9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DPixelShader9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DQuery9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DQuery9 {}
impl ::core::fmt::Debug for IDirect3DQuery9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DQuery9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DResource9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DResource9 {}
impl ::core::fmt::Debug for IDirect3DResource9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DResource9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DStateBlock9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DStateBlock9 {}
impl ::core::fmt::Debug for IDirect3DStateBlock9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DStateBlock9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DSurface9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSurface9 {}
impl ::core::fmt::Debug for IDirect3DSurface9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSurface9").field(&self.0).finish()
    }
}
impl IDirect3DSurface9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDirect3DSwapChain9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSwapChain9 {}
impl ::core::fmt::Debug for IDirect3DSwapChain9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSwapChain9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DSwapChain9Ex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSwapChain9Ex {}
impl ::core::fmt::Debug for IDirect3DSwapChain9Ex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSwapChain9Ex").field(&self.0).finish()
    }
}
impl IDirect3DSwapChain9Ex {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn Present<P0>(&self, psourcerect: *const super::super::Foundation::RECT, pdestrect: *const super::super::Foundation::RECT, hdestwindowoverride: P0, pdirtyregion: *const super::Gdi::RGNDATA, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.Present)(::windows::core::Vtable::as_raw(self), psourcerect, pdestrect, hdestwindowoverride.into(), pdirtyregion, dwflags).ok()
    }
    pub unsafe fn GetFrontBufferData<P0>(&self, pdestsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDirect3DSurface9>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetFrontBufferData)(::windows::core::Vtable::as_raw(self), pdestsurface.into().abi()).ok()
    }
    pub unsafe fn GetBackBuffer(&self, ibackbuffer: u32, r#type: D3DBACKBUFFER_TYPE) -> ::windows::core::Result<IDirect3DSurface9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBackBuffer)(::windows::core::Vtable::as_raw(self), ibackbuffer, r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRasterStatus(&self, prasterstatus: *mut D3DRASTER_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRasterStatus)(::windows::core::Vtable::as_raw(self), prasterstatus).ok()
    }
    pub unsafe fn GetDisplayMode(&self, pmode: *mut D3DDISPLAYMODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDisplayMode)(::windows::core::Vtable::as_raw(self), pmode).ok()
    }
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentParameters(&self, ppresentationparameters: *mut D3DPRESENT_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPresentParameters)(::windows::core::Vtable::as_raw(self), ppresentationparameters).ok()
    }
}
impl ::core::cmp::PartialEq for IDirect3DTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DTexture9 {}
impl ::core::fmt::Debug for IDirect3DTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DTexture9").field(&self.0).finish()
    }
}
impl IDirect3DTexture9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetLOD)(::windows::core::Vtable::as_raw(self), lodnew)
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLOD)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLevelCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAutoGenFilterType)(::windows::core::Vtable::as_raw(self), filtertype).ok()
    }
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        (::windows::core::Vtable::vtable(self).base__.GetAutoGenFilterType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Vtable::vtable(self).base__.GenerateMipSubLevels)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDirect3DVertexBuffer9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVertexBuffer9 {}
impl ::core::fmt::Debug for IDirect3DVertexBuffer9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVertexBuffer9").field(&self.0).finish()
    }
}
impl IDirect3DVertexBuffer9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDirect3DVertexDeclaration9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVertexDeclaration9 {}
impl ::core::fmt::Debug for IDirect3DVertexDeclaration9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVertexDeclaration9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DVertexShader9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVertexShader9 {}
impl ::core::fmt::Debug for IDirect3DVertexShader9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVertexShader9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DVolume9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVolume9 {}
impl ::core::fmt::Debug for IDirect3DVolume9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVolume9").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirect3DVolumeTexture9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DVolumeTexture9 {}
impl ::core::fmt::Debug for IDirect3DVolumeTexture9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DVolumeTexture9").field(&self.0).finish()
    }
}
impl IDirect3DVolumeTexture9 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<IDirect3DDevice9> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *const ::core::ffi::c_void, sizeofdata: u32, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, sizeofdata, flags).ok()
    }
    pub unsafe fn GetPrivateData(&self, refguid: *const ::windows::core::GUID, pdata: *mut ::core::ffi::c_void, psizeofdata: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), refguid, pdata, psizeofdata).ok()
    }
    pub unsafe fn FreePrivateData(&self, refguid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.FreePrivateData)(::windows::core::Vtable::as_raw(self), refguid).ok()
    }
    pub unsafe fn SetPriority(&self, prioritynew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPriority)(::windows::core::Vtable::as_raw(self), prioritynew)
    }
    pub unsafe fn GetPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPriority)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PreLoad(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.PreLoad)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3DRESOURCETYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetLOD(&self, lodnew: u32) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.SetLOD)(::windows::core::Vtable::as_raw(self), lodnew)
    }
    pub unsafe fn GetLOD(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLOD)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLevelCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetLevelCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetAutoGenFilterType(&self, filtertype: D3DTEXTUREFILTERTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetAutoGenFilterType)(::windows::core::Vtable::as_raw(self), filtertype).ok()
    }
    pub unsafe fn GetAutoGenFilterType(&self) -> D3DTEXTUREFILTERTYPE {
        (::windows::core::Vtable::vtable(self).base__.GetAutoGenFilterType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GenerateMipSubLevels(&self) {
        (::windows::core::Vtable::vtable(self).base__.GenerateMipSubLevels)(::windows::core::Vtable::as_raw(self))
    }
}
