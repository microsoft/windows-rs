impl ::core::default::Default for D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_1_CREATE_DEVICE_CONTEXT_STATE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_AES_CTR_IV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_AES_CTR_IV {
    fn eq(&self, other: &Self) -> bool {
        self.IV == other.IV && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for D3D11_AES_CTR_IV {}
impl ::core::fmt::Debug for D3D11_AES_CTR_IV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AES_CTR_IV").field("IV", &self.IV).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for D3D11_ASYNC_GETDATA_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_ASYNC_GETDATA_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_ASYNC_GETDATA_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_AUTHENTICATED_CHANNEL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CHANNEL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_AUTHENTICATED_CHANNEL_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.EncryptionGuid == other.EncryptionGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT").field("Parameters", &self.Parameters).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.DecoderHandle == other.DecoderHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.DeviceHandle == other.DeviceHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT").field("Parameters", &self.Parameters).field("DecoderHandle", &self.DecoderHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.StartSequenceQuery == other.StartSequenceQuery && self.StartSequenceConfigure == other.StartSequenceConfigure
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT").field("Parameters", &self.Parameters).field("StartSequenceQuery", &self.StartSequenceQuery).field("StartSequenceConfigure", &self.StartSequenceConfigure).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.omac == other.omac && self.ConfigureType == other.ConfigureType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_CONFIGURE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_CONFIGURE_INPUT").field("omac", &self.omac).field("ConfigureType", &self.ConfigureType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.omac == other.omac && self.ConfigureType == other.ConfigureType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber && self.ReturnCode == other.ReturnCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_CONFIGURE_OUTPUT").field("omac", &self.omac).field("ConfigureType", &self.ConfigureType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).field("ReturnCode", &self.ReturnCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Parameters == other.Parameters && self.ProcessType == other.ProcessType && self.ProcessHandle == other.ProcessHandle && self.AllowAccess == other.AllowAccess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT").field("Parameters", &self.Parameters).field("ProcessType", &self.ProcessType).field("ProcessHandle", &self.ProcessHandle).field("AllowAccess", &self.AllowAccess).finish()
    }
}
impl ::core::default::Default for D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.EncryptionGuidCount == other.EncryptionGuidCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT").field("Output", &self.Output).field("EncryptionGuidCount", &self.EncryptionGuidCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.EncryptionGuidIndex == other.EncryptionGuidIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT").field("Input", &self.Input).field("EncryptionGuidIndex", &self.EncryptionGuidIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.EncryptionGuidIndex == other.EncryptionGuidIndex && self.EncryptionGuid == other.EncryptionGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT").field("Output", &self.Output).field("EncryptionGuidIndex", &self.EncryptionGuidIndex).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.BusType == other.BusType && self.AccessibleInContiguousBlocks == other.AccessibleInContiguousBlocks && self.AccessibleInNonContiguousBlocks == other.AccessibleInNonContiguousBlocks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT").field("Output", &self.Output).field("BusType", &self.BusType).field("AccessibleInContiguousBlocks", &self.AccessibleInContiguousBlocks).field("AccessibleInNonContiguousBlocks", &self.AccessibleInNonContiguousBlocks).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.ChannelType == other.ChannelType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT").field("Output", &self.Output).field("ChannelType", &self.ChannelType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.DecoderHandle == other.DecoderHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT").field("Input", &self.Input).field("DecoderHandle", &self.DecoderHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DecoderHandle == other.DecoderHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.DeviceHandle == other.DeviceHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT").field("Output", &self.Output).field("DecoderHandle", &self.DecoderHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.EncryptionGuid == other.EncryptionGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT").field("Output", &self.Output).field("EncryptionGuid", &self.EncryptionGuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DeviceHandle == other.DeviceHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.QueryType == other.QueryType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_INPUT").field("QueryType", &self.QueryType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.omac == other.omac && self.QueryType == other.QueryType && self.hChannel == other.hChannel && self.SequenceNumber == other.SequenceNumber && self.ReturnCode == other.ReturnCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_OUTPUT").field("omac", &self.omac).field("QueryType", &self.QueryType).field("hChannel", &self.hChannel).field("SequenceNumber", &self.SequenceNumber).field("ReturnCode", &self.ReturnCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT").field("Input", &self.Input).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.OutputIDCount == other.OutputIDCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("OutputIDCount", &self.OutputIDCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.OutputIDIndex == other.OutputIDIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT").field("Input", &self.Input).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("OutputIDIndex", &self.OutputIDIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.DeviceHandle == other.DeviceHandle && self.CryptoSessionHandle == other.CryptoSessionHandle && self.OutputIDIndex == other.OutputIDIndex && self.OutputID == other.OutputID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT").field("Output", &self.Output).field("DeviceHandle", &self.DeviceHandle).field("CryptoSessionHandle", &self.CryptoSessionHandle).field("OutputIDIndex", &self.OutputIDIndex).field("OutputID", &self.OutputID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.RestrictedSharedResourceProcessCount == other.RestrictedSharedResourceProcessCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT").field("Output", &self.Output).field("RestrictedSharedResourceProcessCount", &self.RestrictedSharedResourceProcessCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.ProcessIndex == other.ProcessIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT").field("Input", &self.Input).field("ProcessIndex", &self.ProcessIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.ProcessIndex == other.ProcessIndex && self.ProcessIdentifier == other.ProcessIdentifier && self.ProcessHandle == other.ProcessHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT").field("Output", &self.Output).field("ProcessIndex", &self.ProcessIndex).field("ProcessIdentifier", &self.ProcessIdentifier).field("ProcessHandle", &self.ProcessHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Output == other.Output && self.UnrestrictedProtectedSharedResourceCount == other.UnrestrictedProtectedSharedResourceCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT").field("Output", &self.Output).field("UnrestrictedProtectedSharedResourceCount", &self.UnrestrictedProtectedSharedResourceCount).finish()
    }
}
impl ::core::default::Default for D3D11_BIND_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_BIND_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_BIND_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_BIND_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_BIND_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_BIND_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_BIND_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_BIND_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_BLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_BLEND").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.AlphaToCoverageEnable == other.AlphaToCoverageEnable && self.IndependentBlendEnable == other.IndependentBlendEnable && self.RenderTarget == other.RenderTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_BLEND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_BLEND_DESC").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("IndependentBlendEnable", &self.IndependentBlendEnable).field("RenderTarget", &self.RenderTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_BLEND_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.AlphaToCoverageEnable == other.AlphaToCoverageEnable && self.IndependentBlendEnable == other.IndependentBlendEnable && self.RenderTarget == other.RenderTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_BLEND_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_BLEND_DESC1").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("IndependentBlendEnable", &self.IndependentBlendEnable).field("RenderTarget", &self.RenderTarget).finish()
    }
}
impl ::core::default::Default for D3D11_BLEND_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_BLEND_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_BLEND_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_BOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_BOX {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.front == other.front && self.right == other.right && self.bottom == other.bottom && self.back == other.back
    }
}
impl ::core::cmp::Eq for D3D11_BOX {}
impl ::core::fmt::Debug for D3D11_BOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_BOX").field("left", &self.left).field("top", &self.top).field("front", &self.front).field("right", &self.right).field("bottom", &self.bottom).field("back", &self.back).finish()
    }
}
impl ::core::default::Default for D3D11_BUFFEREX_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_BUFFEREX_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstElement == other.FirstElement && self.NumElements == other.NumElements && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D11_BUFFEREX_SRV {}
impl ::core::fmt::Debug for D3D11_BUFFEREX_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_BUFFEREX_SRV").field("FirstElement", &self.FirstElement).field("NumElements", &self.NumElements).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D11_BUFFEREX_SRV_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_BUFFEREX_SRV_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_BUFFEREX_SRV_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ByteWidth == other.ByteWidth && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags && self.StructureByteStride == other.StructureByteStride
    }
}
impl ::core::cmp::Eq for D3D11_BUFFER_DESC {}
impl ::core::fmt::Debug for D3D11_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_BUFFER_DESC").field("ByteWidth", &self.ByteWidth).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).field("StructureByteStride", &self.StructureByteStride).finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_BUFFER_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_BUFFER_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstElement == other.FirstElement && self.NumElements == other.NumElements && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D11_BUFFER_UAV {}
impl ::core::fmt::Debug for D3D11_BUFFER_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_BUFFER_UAV").field("FirstElement", &self.FirstElement).field("NumElements", &self.NumElements).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D11_BUFFER_UAV_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_BUFFER_UAV_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_BUFFER_UAV_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_BUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_BUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_BUS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_CLASS_INSTANCE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_CLASS_INSTANCE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InstanceId == other.InstanceId && self.InstanceIndex == other.InstanceIndex && self.TypeId == other.TypeId && self.ConstantBuffer == other.ConstantBuffer && self.BaseConstantBufferOffset == other.BaseConstantBufferOffset && self.BaseTexture == other.BaseTexture && self.BaseSampler == other.BaseSampler && self.Created == other.Created
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_CLASS_INSTANCE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_CLASS_INSTANCE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_CLASS_INSTANCE_DESC").field("InstanceId", &self.InstanceId).field("InstanceIndex", &self.InstanceIndex).field("TypeId", &self.TypeId).field("ConstantBuffer", &self.ConstantBuffer).field("BaseConstantBufferOffset", &self.BaseConstantBufferOffset).field("BaseTexture", &self.BaseTexture).field("BaseSampler", &self.BaseSampler).field("Created", &self.Created).finish()
    }
}
impl ::core::default::Default for D3D11_CLEAR_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CLEAR_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CLEAR_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_CLEAR_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_CLEAR_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_CLEAR_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_CLEAR_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_CLEAR_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_COLOR_WRITE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_COMPARISON_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_COMPARISON_FUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_COMPARISON_FUNC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_COMPUTE_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_COMPUTE_SHADER_TRACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Invocation == other.Invocation && self.ThreadIDInGroup == other.ThreadIDInGroup && self.ThreadGroupID == other.ThreadGroupID
    }
}
impl ::core::cmp::Eq for D3D11_COMPUTE_SHADER_TRACE_DESC {}
impl ::core::fmt::Debug for D3D11_COMPUTE_SHADER_TRACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_COMPUTE_SHADER_TRACE_DESC").field("Invocation", &self.Invocation).field("ThreadIDInGroup", &self.ThreadIDInGroup).field("ThreadGroupID", &self.ThreadGroupID).finish()
    }
}
impl ::core::default::Default for D3D11_CONSERVATIVE_RASTERIZATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CONSERVATIVE_RASTERIZATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CONSERVATIVE_RASTERIZATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_CONSERVATIVE_RASTERIZATION_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CONSERVATIVE_RASTERIZATION_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CONSERVATIVE_RASTERIZATION_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_CONTENT_PROTECTION_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CONTENT_PROTECTION_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CONTENT_PROTECTION_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CONTEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_COPY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_COPY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_COPY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_COUNTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_COUNTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_COUNTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_COUNTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Counter == other.Counter && self.MiscFlags == other.MiscFlags
    }
}
impl ::core::cmp::Eq for D3D11_COUNTER_DESC {}
impl ::core::fmt::Debug for D3D11_COUNTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_COUNTER_DESC").field("Counter", &self.Counter).field("MiscFlags", &self.MiscFlags).finish()
    }
}
impl ::core::default::Default for D3D11_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastDeviceDependentCounter == other.LastDeviceDependentCounter && self.NumSimultaneousCounters == other.NumSimultaneousCounters && self.NumDetectableParallelUnits == other.NumDetectableParallelUnits
    }
}
impl ::core::cmp::Eq for D3D11_COUNTER_INFO {}
impl ::core::fmt::Debug for D3D11_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_COUNTER_INFO").field("LastDeviceDependentCounter", &self.LastDeviceDependentCounter).field("NumSimultaneousCounters", &self.NumSimultaneousCounters).field("NumDetectableParallelUnits", &self.NumDetectableParallelUnits).finish()
    }
}
impl ::core::default::Default for D3D11_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_COUNTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_CPU_ACCESS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CPU_ACCESS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CPU_ACCESS_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_CPU_ACCESS_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_CPU_ACCESS_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_CPU_ACCESS_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_CPU_ACCESS_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_CPU_ACCESS_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_CREATE_DEVICE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CREATE_DEVICE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CREATE_DEVICE_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_CREATE_DEVICE_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_CREATE_DEVICE_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_CREATE_DEVICE_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_CREATE_DEVICE_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_CREATE_DEVICE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_CRYPTO_SESSION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CRYPTO_SESSION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CRYPTO_SESSION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_CULL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_CULL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_CULL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_DEPTH_STENCILOP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.StencilFailOp == other.StencilFailOp && self.StencilDepthFailOp == other.StencilDepthFailOp && self.StencilPassOp == other.StencilPassOp && self.StencilFunc == other.StencilFunc
    }
}
impl ::core::cmp::Eq for D3D11_DEPTH_STENCILOP_DESC {}
impl ::core::fmt::Debug for D3D11_DEPTH_STENCILOP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_DEPTH_STENCILOP_DESC").field("StencilFailOp", &self.StencilFailOp).field("StencilDepthFailOp", &self.StencilDepthFailOp).field("StencilPassOp", &self.StencilPassOp).field("StencilFunc", &self.StencilFunc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_DEPTH_STENCIL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DepthEnable == other.DepthEnable && self.DepthWriteMask == other.DepthWriteMask && self.DepthFunc == other.DepthFunc && self.StencilEnable == other.StencilEnable && self.StencilReadMask == other.StencilReadMask && self.StencilWriteMask == other.StencilWriteMask && self.FrontFace == other.FrontFace && self.BackFace == other.BackFace
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_DEPTH_STENCIL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_DEPTH_STENCIL_DESC").field("DepthEnable", &self.DepthEnable).field("DepthWriteMask", &self.DepthWriteMask).field("DepthFunc", &self.DepthFunc).field("StencilEnable", &self.StencilEnable).field("StencilReadMask", &self.StencilReadMask).field("StencilWriteMask", &self.StencilWriteMask).field("FrontFace", &self.FrontFace).field("BackFace", &self.BackFace).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_DEPTH_WRITE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_DEVICE_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_DEVICE_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_DEVICE_CONTEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_DOMAIN_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_DOMAIN_SHADER_TRACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Invocation == other.Invocation
    }
}
impl ::core::cmp::Eq for D3D11_DOMAIN_SHADER_TRACE_DESC {}
impl ::core::fmt::Debug for D3D11_DOMAIN_SHADER_TRACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_DOMAIN_SHADER_TRACE_DESC").field("Invocation", &self.Invocation).finish()
    }
}
impl ::core::default::Default for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    fn eq(&self, other: &Self) -> bool {
        self.IndexCountPerInstance == other.IndexCountPerInstance && self.InstanceCount == other.InstanceCount && self.StartIndexLocation == other.StartIndexLocation && self.BaseVertexLocation == other.BaseVertexLocation && self.StartInstanceLocation == other.StartInstanceLocation
    }
}
impl ::core::cmp::Eq for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {}
impl ::core::fmt::Debug for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS").field("IndexCountPerInstance", &self.IndexCountPerInstance).field("InstanceCount", &self.InstanceCount).field("StartIndexLocation", &self.StartIndexLocation).field("BaseVertexLocation", &self.BaseVertexLocation).field("StartInstanceLocation", &self.StartInstanceLocation).finish()
    }
}
impl ::core::default::Default for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    fn eq(&self, other: &Self) -> bool {
        self.VertexCountPerInstance == other.VertexCountPerInstance && self.InstanceCount == other.InstanceCount && self.StartVertexLocation == other.StartVertexLocation && self.StartInstanceLocation == other.StartInstanceLocation
    }
}
impl ::core::cmp::Eq for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {}
impl ::core::fmt::Debug for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_DRAW_INSTANCED_INDIRECT_ARGS").field("VertexCountPerInstance", &self.VertexCountPerInstance).field("InstanceCount", &self.InstanceCount).field("StartVertexLocation", &self.StartVertexLocation).field("StartInstanceLocation", &self.StartInstanceLocation).finish()
    }
}
impl ::core::default::Default for D3D11_DSV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_DSV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_DSV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_DSV_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_DSV_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_DSV_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_ENCRYPTED_BLOCK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_ENCRYPTED_BLOCK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumEncryptedBytesAtBeginning == other.NumEncryptedBytesAtBeginning && self.NumBytesInSkipPattern == other.NumBytesInSkipPattern && self.NumBytesInEncryptPattern == other.NumBytesInEncryptPattern
    }
}
impl ::core::cmp::Eq for D3D11_ENCRYPTED_BLOCK_INFO {}
impl ::core::fmt::Debug for D3D11_ENCRYPTED_BLOCK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_ENCRYPTED_BLOCK_INFO").field("NumEncryptedBytesAtBeginning", &self.NumEncryptedBytesAtBeginning).field("NumBytesInSkipPattern", &self.NumBytesInSkipPattern).field("NumBytesInEncryptPattern", &self.NumBytesInEncryptPattern).finish()
    }
}
impl ::core::default::Default for D3D11_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FEATURE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TileBasedDeferredRenderer == other.TileBasedDeferredRenderer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_ARCHITECTURE_INFO").field("TileBasedDeferredRenderer", &self.TileBasedDeferredRenderer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x == other.ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS").field("ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x", &self.ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.OutputMergerLogicOp == other.OutputMergerLogicOp
            && self.UAVOnlyRenderingForcedSampleCount == other.UAVOnlyRenderingForcedSampleCount
            && self.DiscardAPIsSeenByDriver == other.DiscardAPIsSeenByDriver
            && self.FlagsForUpdateAndCopySeenByDriver == other.FlagsForUpdateAndCopySeenByDriver
            && self.ClearView == other.ClearView
            && self.CopyWithOverlap == other.CopyWithOverlap
            && self.ConstantBufferPartialUpdate == other.ConstantBufferPartialUpdate
            && self.ConstantBufferOffsetting == other.ConstantBufferOffsetting
            && self.MapNoOverwriteOnDynamicConstantBuffer == other.MapNoOverwriteOnDynamicConstantBuffer
            && self.MapNoOverwriteOnDynamicBufferSRV == other.MapNoOverwriteOnDynamicBufferSRV
            && self.MultisampleRTVWithForcedSampleCountOne == other.MultisampleRTVWithForcedSampleCountOne
            && self.SAD4ShaderInstructions == other.SAD4ShaderInstructions
            && self.ExtendedDoublesShaderInstructions == other.ExtendedDoublesShaderInstructions
            && self.ExtendedResourceSharing == other.ExtendedResourceSharing
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D11_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D11_OPTIONS")
            .field("OutputMergerLogicOp", &self.OutputMergerLogicOp)
            .field("UAVOnlyRenderingForcedSampleCount", &self.UAVOnlyRenderingForcedSampleCount)
            .field("DiscardAPIsSeenByDriver", &self.DiscardAPIsSeenByDriver)
            .field("FlagsForUpdateAndCopySeenByDriver", &self.FlagsForUpdateAndCopySeenByDriver)
            .field("ClearView", &self.ClearView)
            .field("CopyWithOverlap", &self.CopyWithOverlap)
            .field("ConstantBufferPartialUpdate", &self.ConstantBufferPartialUpdate)
            .field("ConstantBufferOffsetting", &self.ConstantBufferOffsetting)
            .field("MapNoOverwriteOnDynamicConstantBuffer", &self.MapNoOverwriteOnDynamicConstantBuffer)
            .field("MapNoOverwriteOnDynamicBufferSRV", &self.MapNoOverwriteOnDynamicBufferSRV)
            .field("MultisampleRTVWithForcedSampleCountOne", &self.MultisampleRTVWithForcedSampleCountOne)
            .field("SAD4ShaderInstructions", &self.SAD4ShaderInstructions)
            .field("ExtendedDoublesShaderInstructions", &self.ExtendedDoublesShaderInstructions)
            .field("ExtendedResourceSharing", &self.ExtendedResourceSharing)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    fn eq(&self, other: &Self) -> bool {
        self.TiledResourcesTier == other.TiledResourcesTier && self.MinMaxFiltering == other.MinMaxFiltering && self.ClearViewAlsoSupportsDepthOnlyFormats == other.ClearViewAlsoSupportsDepthOnlyFormats && self.MapOnDefaultBuffers == other.MapOnDefaultBuffers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D11_OPTIONS1").field("TiledResourcesTier", &self.TiledResourcesTier).field("MinMaxFiltering", &self.MinMaxFiltering).field("ClearViewAlsoSupportsDepthOnlyFormats", &self.ClearViewAlsoSupportsDepthOnlyFormats).field("MapOnDefaultBuffers", &self.MapOnDefaultBuffers).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    fn eq(&self, other: &Self) -> bool {
        self.PSSpecifiedStencilRefSupported == other.PSSpecifiedStencilRefSupported && self.TypedUAVLoadAdditionalFormats == other.TypedUAVLoadAdditionalFormats && self.ROVsSupported == other.ROVsSupported && self.ConservativeRasterizationTier == other.ConservativeRasterizationTier && self.TiledResourcesTier == other.TiledResourcesTier && self.MapOnDefaultTextures == other.MapOnDefaultTextures && self.StandardSwizzle == other.StandardSwizzle && self.UnifiedMemoryArchitecture == other.UnifiedMemoryArchitecture
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D11_OPTIONS2")
            .field("PSSpecifiedStencilRefSupported", &self.PSSpecifiedStencilRefSupported)
            .field("TypedUAVLoadAdditionalFormats", &self.TypedUAVLoadAdditionalFormats)
            .field("ROVsSupported", &self.ROVsSupported)
            .field("ConservativeRasterizationTier", &self.ConservativeRasterizationTier)
            .field("TiledResourcesTier", &self.TiledResourcesTier)
            .field("MapOnDefaultTextures", &self.MapOnDefaultTextures)
            .field("StandardSwizzle", &self.StandardSwizzle)
            .field("UnifiedMemoryArchitecture", &self.UnifiedMemoryArchitecture)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    fn eq(&self, other: &Self) -> bool {
        self.VPAndRTArrayIndexFromAnyShaderFeedingRasterizer == other.VPAndRTArrayIndexFromAnyShaderFeedingRasterizer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D11_OPTIONS3").field("VPAndRTArrayIndexFromAnyShaderFeedingRasterizer", &self.VPAndRTArrayIndexFromAnyShaderFeedingRasterizer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    fn eq(&self, other: &Self) -> bool {
        self.ExtendedNV12SharedTextureSupported == other.ExtendedNV12SharedTextureSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D11_OPTIONS4").field("ExtendedNV12SharedTextureSupported", &self.ExtendedNV12SharedTextureSupported).finish()
    }
}
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    fn eq(&self, other: &Self) -> bool {
        self.SharedResourceTier == other.SharedResourceTier
    }
}
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {}
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D11_OPTIONS5").field("SharedResourceTier", &self.SharedResourceTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.FullNonPow2TextureSupport == other.FullNonPow2TextureSupport
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D9_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D9_OPTIONS").field("FullNonPow2TextureSupport", &self.FullNonPow2TextureSupport).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    fn eq(&self, other: &Self) -> bool {
        self.FullNonPow2TextureSupported == other.FullNonPow2TextureSupported && self.DepthAsTextureWithLessEqualComparisonFilterSupported == other.DepthAsTextureWithLessEqualComparisonFilterSupported && self.SimpleInstancingSupported == other.SimpleInstancingSupported && self.TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported == other.TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D9_OPTIONS1").field("FullNonPow2TextureSupported", &self.FullNonPow2TextureSupported).field("DepthAsTextureWithLessEqualComparisonFilterSupported", &self.DepthAsTextureWithLessEqualComparisonFilterSupported).field("SimpleInstancingSupported", &self.SimpleInstancingSupported).field("TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported", &self.TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.SupportsDepthAsTextureWithLessEqualComparisonFilter == other.SupportsDepthAsTextureWithLessEqualComparisonFilter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT").field("SupportsDepthAsTextureWithLessEqualComparisonFilter", &self.SupportsDepthAsTextureWithLessEqualComparisonFilter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.SimpleInstancingSupported == other.SimpleInstancingSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT").field("SimpleInstancingSupported", &self.SimpleInstancingSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_DISPLAYABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_DISPLAYABLE {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayableTexture == other.DisplayableTexture && self.SharedResourceTier == other.SharedResourceTier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_DISPLAYABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_DISPLAYABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_DISPLAYABLE").field("DisplayableTexture", &self.DisplayableTexture).field("SharedResourceTier", &self.SharedResourceTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_DOUBLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_DOUBLES {
    fn eq(&self, other: &Self) -> bool {
        self.DoublePrecisionFloatShaderOps == other.DoublePrecisionFloatShaderOps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_DOUBLES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_DOUBLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_DOUBLES").field("DoublePrecisionFloatShaderOps", &self.DoublePrecisionFloatShaderOps).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.InFormat == other.InFormat && self.OutFormatSupport == other.OutFormatSupport
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_FORMAT_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_FORMAT_SUPPORT").field("InFormat", &self.InFormat).field("OutFormatSupport", &self.OutFormatSupport).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    fn eq(&self, other: &Self) -> bool {
        self.InFormat == other.InFormat && self.OutFormatSupport2 == other.OutFormatSupport2
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_FORMAT_SUPPORT2").field("InFormat", &self.InFormat).field("OutFormatSupport2", &self.OutFormatSupport2).finish()
    }
}
impl ::core::default::Default for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.MaxGPUVirtualAddressBitsPerResource == other.MaxGPUVirtualAddressBitsPerResource && self.MaxGPUVirtualAddressBitsPerProcess == other.MaxGPUVirtualAddressBitsPerProcess
    }
}
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {}
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT").field("MaxGPUVirtualAddressBitsPerResource", &self.MaxGPUVirtualAddressBitsPerResource).field("MaxGPUVirtualAddressBitsPerProcess", &self.MaxGPUVirtualAddressBitsPerProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.Profile == other.Profile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_MARKER_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_MARKER_SUPPORT").field("Profile", &self.Profile).finish()
    }
}
impl ::core::default::Default for D3D11_FEATURE_DATA_SHADER_CACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_SHADER_CACHE {
    fn eq(&self, other: &Self) -> bool {
        self.SupportFlags == other.SupportFlags
    }
}
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_SHADER_CACHE {}
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_SHADER_CACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_SHADER_CACHE").field("SupportFlags", &self.SupportFlags).finish()
    }
}
impl ::core::default::Default for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.PixelShaderMinPrecision == other.PixelShaderMinPrecision && self.AllOtherShaderStagesMinPrecision == other.AllOtherShaderStagesMinPrecision
    }
}
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {}
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT").field("PixelShaderMinPrecision", &self.PixelShaderMinPrecision).field("AllOtherShaderStagesMinPrecision", &self.AllOtherShaderStagesMinPrecision).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_FEATURE_DATA_THREADING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_THREADING {
    fn eq(&self, other: &Self) -> bool {
        self.DriverConcurrentCreates == other.DriverConcurrentCreates && self.DriverCommandLists == other.DriverCommandLists
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_THREADING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_THREADING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_THREADING").field("DriverConcurrentCreates", &self.DriverConcurrentCreates).field("DriverCommandLists", &self.DriverCommandLists).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    fn eq(&self, other: &Self) -> bool {
        self.DecoderDesc == other.DecoderDesc && self.Components == other.Components && self.BinCount == other.BinCount && self.CounterBitDepth == other.CounterBitDepth
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM").field("DecoderDesc", &self.DecoderDesc).field("Components", &self.Components).field("BinCount", &self.BinCount).field("CounterBitDepth", &self.CounterBitDepth).finish()
    }
}
impl ::core::default::Default for D3D11_FEATURE_VIDEO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FEATURE_VIDEO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FEATURE_VIDEO").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_FENCE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FENCE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FENCE_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_FENCE_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_FENCE_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_FENCE_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_FENCE_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_FENCE_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_FILTER_REDUCTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FILTER_REDUCTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FILTER_REDUCTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FILTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_FORMAT_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_FORMAT_SUPPORT2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_FORMAT_SUPPORT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_FORMAT_SUPPORT2").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D11_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D11_FUNCTION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Creator == other.Creator
            && self.Flags == other.Flags
            && self.ConstantBuffers == other.ConstantBuffers
            && self.BoundResources == other.BoundResources
            && self.InstructionCount == other.InstructionCount
            && self.TempRegisterCount == other.TempRegisterCount
            && self.TempArrayCount == other.TempArrayCount
            && self.DefCount == other.DefCount
            && self.DclCount == other.DclCount
            && self.TextureNormalInstructions == other.TextureNormalInstructions
            && self.TextureLoadInstructions == other.TextureLoadInstructions
            && self.TextureCompInstructions == other.TextureCompInstructions
            && self.TextureBiasInstructions == other.TextureBiasInstructions
            && self.TextureGradientInstructions == other.TextureGradientInstructions
            && self.FloatInstructionCount == other.FloatInstructionCount
            && self.IntInstructionCount == other.IntInstructionCount
            && self.UintInstructionCount == other.UintInstructionCount
            && self.StaticFlowControlCount == other.StaticFlowControlCount
            && self.DynamicFlowControlCount == other.DynamicFlowControlCount
            && self.MacroInstructionCount == other.MacroInstructionCount
            && self.ArrayInstructionCount == other.ArrayInstructionCount
            && self.MovInstructionCount == other.MovInstructionCount
            && self.MovcInstructionCount == other.MovcInstructionCount
            && self.ConversionInstructionCount == other.ConversionInstructionCount
            && self.BitwiseInstructionCount == other.BitwiseInstructionCount
            && self.MinFeatureLevel == other.MinFeatureLevel
            && self.RequiredFeatureFlags == other.RequiredFeatureFlags
            && self.Name == other.Name
            && self.FunctionParameterCount == other.FunctionParameterCount
            && self.HasReturn == other.HasReturn
            && self.Has10Level9VertexShader == other.Has10Level9VertexShader
            && self.Has10Level9PixelShader == other.Has10Level9PixelShader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D11_FUNCTION_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::fmt::Debug for D3D11_FUNCTION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_FUNCTION_DESC")
            .field("Version", &self.Version)
            .field("Creator", &self.Creator)
            .field("Flags", &self.Flags)
            .field("ConstantBuffers", &self.ConstantBuffers)
            .field("BoundResources", &self.BoundResources)
            .field("InstructionCount", &self.InstructionCount)
            .field("TempRegisterCount", &self.TempRegisterCount)
            .field("TempArrayCount", &self.TempArrayCount)
            .field("DefCount", &self.DefCount)
            .field("DclCount", &self.DclCount)
            .field("TextureNormalInstructions", &self.TextureNormalInstructions)
            .field("TextureLoadInstructions", &self.TextureLoadInstructions)
            .field("TextureCompInstructions", &self.TextureCompInstructions)
            .field("TextureBiasInstructions", &self.TextureBiasInstructions)
            .field("TextureGradientInstructions", &self.TextureGradientInstructions)
            .field("FloatInstructionCount", &self.FloatInstructionCount)
            .field("IntInstructionCount", &self.IntInstructionCount)
            .field("UintInstructionCount", &self.UintInstructionCount)
            .field("StaticFlowControlCount", &self.StaticFlowControlCount)
            .field("DynamicFlowControlCount", &self.DynamicFlowControlCount)
            .field("MacroInstructionCount", &self.MacroInstructionCount)
            .field("ArrayInstructionCount", &self.ArrayInstructionCount)
            .field("MovInstructionCount", &self.MovInstructionCount)
            .field("MovcInstructionCount", &self.MovcInstructionCount)
            .field("ConversionInstructionCount", &self.ConversionInstructionCount)
            .field("BitwiseInstructionCount", &self.BitwiseInstructionCount)
            .field("MinFeatureLevel", &self.MinFeatureLevel)
            .field("RequiredFeatureFlags", &self.RequiredFeatureFlags)
            .field("Name", &self.Name)
            .field("FunctionParameterCount", &self.FunctionParameterCount)
            .field("HasReturn", &self.HasReturn)
            .field("Has10Level9VertexShader", &self.Has10Level9VertexShader)
            .field("Has10Level9PixelShader", &self.Has10Level9PixelShader)
            .finish()
    }
}
impl ::core::default::Default for D3D11_GEOMETRY_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_GEOMETRY_SHADER_TRACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Invocation == other.Invocation
    }
}
impl ::core::cmp::Eq for D3D11_GEOMETRY_SHADER_TRACE_DESC {}
impl ::core::fmt::Debug for D3D11_GEOMETRY_SHADER_TRACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_GEOMETRY_SHADER_TRACE_DESC").field("Invocation", &self.Invocation).finish()
    }
}
impl ::core::default::Default for D3D11_HULL_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_HULL_SHADER_TRACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Invocation == other.Invocation
    }
}
impl ::core::cmp::Eq for D3D11_HULL_SHADER_TRACE_DESC {}
impl ::core::fmt::Debug for D3D11_HULL_SHADER_TRACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_HULL_SHADER_TRACE_DESC").field("Invocation", &self.Invocation).finish()
    }
}
impl ::core::default::Default for D3D11_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::core::cmp::Eq for D3D11_INFO_QUEUE_FILTER {}
impl ::core::fmt::Debug for D3D11_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
impl ::core::default::Default for D3D11_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories && self.pCategoryList == other.pCategoryList && self.NumSeverities == other.NumSeverities && self.pSeverityList == other.pSeverityList && self.NumIDs == other.NumIDs && self.pIDList == other.pIDList
    }
}
impl ::core::cmp::Eq for D3D11_INFO_QUEUE_FILTER_DESC {}
impl ::core::fmt::Debug for D3D11_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
impl ::core::default::Default for D3D11_INPUT_CLASSIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.Format == other.Format && self.InputSlot == other.InputSlot && self.AlignedByteOffset == other.AlignedByteOffset && self.InputSlotClass == other.InputSlotClass && self.InstanceDataStepRate == other.InstanceDataStepRate
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_INPUT_ELEMENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_INPUT_ELEMENT_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Format", &self.Format).field("InputSlot", &self.InputSlot).field("AlignedByteOffset", &self.AlignedByteOffset).field("InputSlotClass", &self.InputSlotClass).field("InstanceDataStepRate", &self.InstanceDataStepRate).finish()
    }
}
impl ::core::default::Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.HWProtectionFunctionID == other.HWProtectionFunctionID && self.pInputData == other.pInputData && self.pOutputData == other.pOutputData && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {}
impl ::core::fmt::Debug for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA").field("HWProtectionFunctionID", &self.HWProtectionFunctionID).field("pInputData", &self.pInputData).field("pOutputData", &self.pOutputData).field("Status", &self.Status).finish()
    }
}
impl ::core::default::Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PrivateDataSize == other.PrivateDataSize && self.HWProtectionDataSize == other.HWProtectionDataSize && self.pbInput == other.pbInput
    }
}
impl ::core::cmp::Eq for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {}
impl ::core::fmt::Debug for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA").field("PrivateDataSize", &self.PrivateDataSize).field("HWProtectionDataSize", &self.HWProtectionDataSize).field("pbInput", &self.pbInput).finish()
    }
}
impl ::core::default::Default for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.PrivateDataSize == other.PrivateDataSize && self.MaxHWProtectionDataSize == other.MaxHWProtectionDataSize && self.HWProtectionDataSize == other.HWProtectionDataSize && self.TransportTime == other.TransportTime && self.ExecutionTime == other.ExecutionTime && self.pbOutput == other.pbOutput
    }
}
impl ::core::cmp::Eq for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {}
impl ::core::fmt::Debug for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA").field("PrivateDataSize", &self.PrivateDataSize).field("MaxHWProtectionDataSize", &self.MaxHWProtectionDataSize).field("HWProtectionDataSize", &self.HWProtectionDataSize).field("TransportTime", &self.TransportTime).field("ExecutionTime", &self.ExecutionTime).field("pbOutput", &self.pbOutput).finish()
    }
}
impl ::core::default::Default for D3D11_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_LIBRARY_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Creator == other.Creator && self.Flags == other.Flags && self.FunctionCount == other.FunctionCount
    }
}
impl ::core::cmp::Eq for D3D11_LIBRARY_DESC {}
impl ::core::fmt::Debug for D3D11_LIBRARY_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_LIBRARY_DESC").field("Creator", &self.Creator).field("Flags", &self.Flags).field("FunctionCount", &self.FunctionCount).finish()
    }
}
impl ::core::default::Default for D3D11_LOGIC_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_LOGIC_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_LOGIC_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_MAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_MAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_MAPPED_SUBRESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_MAPPED_SUBRESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.pData == other.pData && self.RowPitch == other.RowPitch && self.DepthPitch == other.DepthPitch
    }
}
impl ::core::cmp::Eq for D3D11_MAPPED_SUBRESOURCE {}
impl ::core::fmt::Debug for D3D11_MAPPED_SUBRESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_MAPPED_SUBRESOURCE").field("pData", &self.pData).field("RowPitch", &self.RowPitch).field("DepthPitch", &self.DepthPitch).finish()
    }
}
impl ::core::default::Default for D3D11_MAP_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_MAP_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_MAP_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Category == other.Category && self.Severity == other.Severity && self.ID == other.ID && self.pDescription == other.pDescription && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::core::cmp::Eq for D3D11_MESSAGE {}
impl ::core::fmt::Debug for D3D11_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_MESSAGE").field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
impl ::core::default::Default for D3D11_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_MESSAGE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_OMAC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_OMAC {
    fn eq(&self, other: &Self) -> bool {
        self.Omac == other.Omac
    }
}
impl ::core::cmp::Eq for D3D11_OMAC {}
impl ::core::fmt::Debug for D3D11_OMAC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_OMAC").field("Omac", &self.Omac).finish()
    }
}
impl ::core::default::Default for D3D11_PACKED_MIP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_PACKED_MIP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumStandardMips == other.NumStandardMips && self.NumPackedMips == other.NumPackedMips && self.NumTilesForPackedMips == other.NumTilesForPackedMips && self.StartTileIndexInOverallResource == other.StartTileIndexInOverallResource
    }
}
impl ::core::cmp::Eq for D3D11_PACKED_MIP_DESC {}
impl ::core::fmt::Debug for D3D11_PACKED_MIP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_PACKED_MIP_DESC").field("NumStandardMips", &self.NumStandardMips).field("NumPackedMips", &self.NumPackedMips).field("NumTilesForPackedMips", &self.NumTilesForPackedMips).field("StartTileIndexInOverallResource", &self.StartTileIndexInOverallResource).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D11_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D11_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.SemanticName == other.SemanticName && self.Type == other.Type && self.Class == other.Class && self.Rows == other.Rows && self.Columns == other.Columns && self.InterpolationMode == other.InterpolationMode && self.Flags == other.Flags && self.FirstInRegister == other.FirstInRegister && self.FirstInComponent == other.FirstInComponent && self.FirstOutRegister == other.FirstOutRegister && self.FirstOutComponent == other.FirstOutComponent
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D11_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D11_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_PARAMETER_DESC")
            .field("Name", &self.Name)
            .field("SemanticName", &self.SemanticName)
            .field("Type", &self.Type)
            .field("Class", &self.Class)
            .field("Rows", &self.Rows)
            .field("Columns", &self.Columns)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("Flags", &self.Flags)
            .field("FirstInRegister", &self.FirstInRegister)
            .field("FirstInComponent", &self.FirstInComponent)
            .field("FirstOutRegister", &self.FirstOutRegister)
            .field("FirstOutComponent", &self.FirstOutComponent)
            .finish()
    }
}
impl ::core::default::Default for D3D11_PIXEL_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_PIXEL_SHADER_TRACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Invocation == other.Invocation && self.X == other.X && self.Y == other.Y && self.SampleMask == other.SampleMask
    }
}
impl ::core::cmp::Eq for D3D11_PIXEL_SHADER_TRACE_DESC {}
impl ::core::fmt::Debug for D3D11_PIXEL_SHADER_TRACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_PIXEL_SHADER_TRACE_DESC").field("Invocation", &self.Invocation).field("X", &self.X).field("Y", &self.Y).field("SampleMask", &self.SampleMask).finish()
    }
}
impl ::core::default::Default for D3D11_QUERY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_QUERY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.IAVertices == other.IAVertices && self.IAPrimitives == other.IAPrimitives && self.VSInvocations == other.VSInvocations && self.GSInvocations == other.GSInvocations && self.GSPrimitives == other.GSPrimitives && self.CInvocations == other.CInvocations && self.CPrimitives == other.CPrimitives && self.PSInvocations == other.PSInvocations && self.HSInvocations == other.HSInvocations && self.DSInvocations == other.DSInvocations && self.CSInvocations == other.CSInvocations
    }
}
impl ::core::cmp::Eq for D3D11_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::fmt::Debug for D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_QUERY_DATA_PIPELINE_STATISTICS")
            .field("IAVertices", &self.IAVertices)
            .field("IAPrimitives", &self.IAPrimitives)
            .field("VSInvocations", &self.VSInvocations)
            .field("GSInvocations", &self.GSInvocations)
            .field("GSPrimitives", &self.GSPrimitives)
            .field("CInvocations", &self.CInvocations)
            .field("CPrimitives", &self.CPrimitives)
            .field("PSInvocations", &self.PSInvocations)
            .field("HSInvocations", &self.HSInvocations)
            .field("DSInvocations", &self.DSInvocations)
            .field("CSInvocations", &self.CSInvocations)
            .finish()
    }
}
impl ::core::default::Default for D3D11_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_QUERY_DATA_SO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.NumPrimitivesWritten == other.NumPrimitivesWritten && self.PrimitivesStorageNeeded == other.PrimitivesStorageNeeded
    }
}
impl ::core::cmp::Eq for D3D11_QUERY_DATA_SO_STATISTICS {}
impl ::core::fmt::Debug for D3D11_QUERY_DATA_SO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_QUERY_DATA_SO_STATISTICS").field("NumPrimitivesWritten", &self.NumPrimitivesWritten).field("PrimitivesStorageNeeded", &self.PrimitivesStorageNeeded).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn eq(&self, other: &Self) -> bool {
        self.Frequency == other.Frequency && self.Disjoint == other.Disjoint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_QUERY_DATA_TIMESTAMP_DISJOINT").field("Frequency", &self.Frequency).field("Disjoint", &self.Disjoint).finish()
    }
}
impl ::core::default::Default for D3D11_QUERY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_QUERY_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Query == other.Query && self.MiscFlags == other.MiscFlags
    }
}
impl ::core::cmp::Eq for D3D11_QUERY_DESC {}
impl ::core::fmt::Debug for D3D11_QUERY_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_QUERY_DESC").field("Query", &self.Query).field("MiscFlags", &self.MiscFlags).finish()
    }
}
impl ::core::default::Default for D3D11_QUERY_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_QUERY_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Query == other.Query && self.MiscFlags == other.MiscFlags && self.ContextType == other.ContextType
    }
}
impl ::core::cmp::Eq for D3D11_QUERY_DESC1 {}
impl ::core::fmt::Debug for D3D11_QUERY_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_QUERY_DESC1").field("Query", &self.Query).field("MiscFlags", &self.MiscFlags).field("ContextType", &self.ContextType).finish()
    }
}
impl ::core::default::Default for D3D11_QUERY_MISC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_QUERY_MISC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_QUERY_MISC_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_RAISE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_RAISE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_RAISE_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_RASTERIZER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FillMode == other.FillMode && self.CullMode == other.CullMode && self.FrontCounterClockwise == other.FrontCounterClockwise && self.DepthBias == other.DepthBias && self.DepthBiasClamp == other.DepthBiasClamp && self.SlopeScaledDepthBias == other.SlopeScaledDepthBias && self.DepthClipEnable == other.DepthClipEnable && self.ScissorEnable == other.ScissorEnable && self.MultisampleEnable == other.MultisampleEnable && self.AntialiasedLineEnable == other.AntialiasedLineEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_RASTERIZER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RASTERIZER_DESC")
            .field("FillMode", &self.FillMode)
            .field("CullMode", &self.CullMode)
            .field("FrontCounterClockwise", &self.FrontCounterClockwise)
            .field("DepthBias", &self.DepthBias)
            .field("DepthBiasClamp", &self.DepthBiasClamp)
            .field("SlopeScaledDepthBias", &self.SlopeScaledDepthBias)
            .field("DepthClipEnable", &self.DepthClipEnable)
            .field("ScissorEnable", &self.ScissorEnable)
            .field("MultisampleEnable", &self.MultisampleEnable)
            .field("AntialiasedLineEnable", &self.AntialiasedLineEnable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_RASTERIZER_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_RASTERIZER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.FillMode == other.FillMode && self.CullMode == other.CullMode && self.FrontCounterClockwise == other.FrontCounterClockwise && self.DepthBias == other.DepthBias && self.DepthBiasClamp == other.DepthBiasClamp && self.SlopeScaledDepthBias == other.SlopeScaledDepthBias && self.DepthClipEnable == other.DepthClipEnable && self.ScissorEnable == other.ScissorEnable && self.MultisampleEnable == other.MultisampleEnable && self.AntialiasedLineEnable == other.AntialiasedLineEnable && self.ForcedSampleCount == other.ForcedSampleCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_RASTERIZER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_RASTERIZER_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RASTERIZER_DESC1")
            .field("FillMode", &self.FillMode)
            .field("CullMode", &self.CullMode)
            .field("FrontCounterClockwise", &self.FrontCounterClockwise)
            .field("DepthBias", &self.DepthBias)
            .field("DepthBiasClamp", &self.DepthBiasClamp)
            .field("SlopeScaledDepthBias", &self.SlopeScaledDepthBias)
            .field("DepthClipEnable", &self.DepthClipEnable)
            .field("ScissorEnable", &self.ScissorEnable)
            .field("MultisampleEnable", &self.MultisampleEnable)
            .field("AntialiasedLineEnable", &self.AntialiasedLineEnable)
            .field("ForcedSampleCount", &self.ForcedSampleCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_RASTERIZER_DESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_RASTERIZER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.FillMode == other.FillMode && self.CullMode == other.CullMode && self.FrontCounterClockwise == other.FrontCounterClockwise && self.DepthBias == other.DepthBias && self.DepthBiasClamp == other.DepthBiasClamp && self.SlopeScaledDepthBias == other.SlopeScaledDepthBias && self.DepthClipEnable == other.DepthClipEnable && self.ScissorEnable == other.ScissorEnable && self.MultisampleEnable == other.MultisampleEnable && self.AntialiasedLineEnable == other.AntialiasedLineEnable && self.ForcedSampleCount == other.ForcedSampleCount && self.ConservativeRaster == other.ConservativeRaster
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_RASTERIZER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_RASTERIZER_DESC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RASTERIZER_DESC2")
            .field("FillMode", &self.FillMode)
            .field("CullMode", &self.CullMode)
            .field("FrontCounterClockwise", &self.FrontCounterClockwise)
            .field("DepthBias", &self.DepthBias)
            .field("DepthBiasClamp", &self.DepthBiasClamp)
            .field("SlopeScaledDepthBias", &self.SlopeScaledDepthBias)
            .field("DepthClipEnable", &self.DepthClipEnable)
            .field("ScissorEnable", &self.ScissorEnable)
            .field("MultisampleEnable", &self.MultisampleEnable)
            .field("AntialiasedLineEnable", &self.AntialiasedLineEnable)
            .field("ForcedSampleCount", &self.ForcedSampleCount)
            .field("ConservativeRaster", &self.ConservativeRaster)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_RENDER_TARGET_BLEND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_RENDER_TARGET_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BlendEnable == other.BlendEnable && self.SrcBlend == other.SrcBlend && self.DestBlend == other.DestBlend && self.BlendOp == other.BlendOp && self.SrcBlendAlpha == other.SrcBlendAlpha && self.DestBlendAlpha == other.DestBlendAlpha && self.BlendOpAlpha == other.BlendOpAlpha && self.RenderTargetWriteMask == other.RenderTargetWriteMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_RENDER_TARGET_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_RENDER_TARGET_BLEND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RENDER_TARGET_BLEND_DESC").field("BlendEnable", &self.BlendEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_RENDER_TARGET_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_RENDER_TARGET_BLEND_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.BlendEnable == other.BlendEnable && self.LogicOpEnable == other.LogicOpEnable && self.SrcBlend == other.SrcBlend && self.DestBlend == other.DestBlend && self.BlendOp == other.BlendOp && self.SrcBlendAlpha == other.SrcBlendAlpha && self.DestBlendAlpha == other.DestBlendAlpha && self.BlendOpAlpha == other.BlendOpAlpha && self.LogicOp == other.LogicOp && self.RenderTargetWriteMask == other.RenderTargetWriteMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_RENDER_TARGET_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_RENDER_TARGET_BLEND_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_RENDER_TARGET_BLEND_DESC1").field("BlendEnable", &self.BlendEnable).field("LogicOpEnable", &self.LogicOpEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("LogicOp", &self.LogicOp).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_RENDER_TARGET_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_RESOURCE_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_RESOURCE_MISC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_RESOURCE_MISC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_RESOURCE_MISC_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_RESOURCE_MISC_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_RESOURCE_MISC_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_RESOURCE_MISC_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_RESOURCE_MISC_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_RESOURCE_MISC_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_RLDO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_RLDO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_RLDO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_RTV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_RTV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_RTV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_SAMPLER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Filter == other.Filter && self.AddressU == other.AddressU && self.AddressV == other.AddressV && self.AddressW == other.AddressW && self.MipLODBias == other.MipLODBias && self.MaxAnisotropy == other.MaxAnisotropy && self.ComparisonFunc == other.ComparisonFunc && self.BorderColor == other.BorderColor && self.MinLOD == other.MinLOD && self.MaxLOD == other.MaxLOD
    }
}
impl ::core::cmp::Eq for D3D11_SAMPLER_DESC {}
impl ::core::fmt::Debug for D3D11_SAMPLER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SAMPLER_DESC").field("Filter", &self.Filter).field("AddressU", &self.AddressU).field("AddressV", &self.AddressV).field("AddressW", &self.AddressW).field("MipLODBias", &self.MipLODBias).field("MaxAnisotropy", &self.MaxAnisotropy).field("ComparisonFunc", &self.ComparisonFunc).field("BorderColor", &self.BorderColor).field("MinLOD", &self.MinLOD).field("MaxLOD", &self.MaxLOD).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D11_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D11_SHADER_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.Variables == other.Variables && self.Size == other.Size && self.uFlags == other.uFlags
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D11_SHADER_BUFFER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D11_SHADER_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SHADER_BUFFER_DESC").field("Name", &self.Name).field("Type", &self.Type).field("Variables", &self.Variables).field("Size", &self.Size).field("uFlags", &self.uFlags).finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_CACHE_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHADER_CACHE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_CACHE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D11_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D11_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Creator == other.Creator
            && self.Flags == other.Flags
            && self.ConstantBuffers == other.ConstantBuffers
            && self.BoundResources == other.BoundResources
            && self.InputParameters == other.InputParameters
            && self.OutputParameters == other.OutputParameters
            && self.InstructionCount == other.InstructionCount
            && self.TempRegisterCount == other.TempRegisterCount
            && self.TempArrayCount == other.TempArrayCount
            && self.DefCount == other.DefCount
            && self.DclCount == other.DclCount
            && self.TextureNormalInstructions == other.TextureNormalInstructions
            && self.TextureLoadInstructions == other.TextureLoadInstructions
            && self.TextureCompInstructions == other.TextureCompInstructions
            && self.TextureBiasInstructions == other.TextureBiasInstructions
            && self.TextureGradientInstructions == other.TextureGradientInstructions
            && self.FloatInstructionCount == other.FloatInstructionCount
            && self.IntInstructionCount == other.IntInstructionCount
            && self.UintInstructionCount == other.UintInstructionCount
            && self.StaticFlowControlCount == other.StaticFlowControlCount
            && self.DynamicFlowControlCount == other.DynamicFlowControlCount
            && self.MacroInstructionCount == other.MacroInstructionCount
            && self.ArrayInstructionCount == other.ArrayInstructionCount
            && self.CutInstructionCount == other.CutInstructionCount
            && self.EmitInstructionCount == other.EmitInstructionCount
            && self.GSOutputTopology == other.GSOutputTopology
            && self.GSMaxOutputVertexCount == other.GSMaxOutputVertexCount
            && self.InputPrimitive == other.InputPrimitive
            && self.PatchConstantParameters == other.PatchConstantParameters
            && self.cGSInstanceCount == other.cGSInstanceCount
            && self.cControlPoints == other.cControlPoints
            && self.HSOutputPrimitive == other.HSOutputPrimitive
            && self.HSPartitioning == other.HSPartitioning
            && self.TessellatorDomain == other.TessellatorDomain
            && self.cBarrierInstructions == other.cBarrierInstructions
            && self.cInterlockedInstructions == other.cInterlockedInstructions
            && self.cTextureStoreInstructions == other.cTextureStoreInstructions
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D11_SHADER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D11_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SHADER_DESC")
            .field("Version", &self.Version)
            .field("Creator", &self.Creator)
            .field("Flags", &self.Flags)
            .field("ConstantBuffers", &self.ConstantBuffers)
            .field("BoundResources", &self.BoundResources)
            .field("InputParameters", &self.InputParameters)
            .field("OutputParameters", &self.OutputParameters)
            .field("InstructionCount", &self.InstructionCount)
            .field("TempRegisterCount", &self.TempRegisterCount)
            .field("TempArrayCount", &self.TempArrayCount)
            .field("DefCount", &self.DefCount)
            .field("DclCount", &self.DclCount)
            .field("TextureNormalInstructions", &self.TextureNormalInstructions)
            .field("TextureLoadInstructions", &self.TextureLoadInstructions)
            .field("TextureCompInstructions", &self.TextureCompInstructions)
            .field("TextureBiasInstructions", &self.TextureBiasInstructions)
            .field("TextureGradientInstructions", &self.TextureGradientInstructions)
            .field("FloatInstructionCount", &self.FloatInstructionCount)
            .field("IntInstructionCount", &self.IntInstructionCount)
            .field("UintInstructionCount", &self.UintInstructionCount)
            .field("StaticFlowControlCount", &self.StaticFlowControlCount)
            .field("DynamicFlowControlCount", &self.DynamicFlowControlCount)
            .field("MacroInstructionCount", &self.MacroInstructionCount)
            .field("ArrayInstructionCount", &self.ArrayInstructionCount)
            .field("CutInstructionCount", &self.CutInstructionCount)
            .field("EmitInstructionCount", &self.EmitInstructionCount)
            .field("GSOutputTopology", &self.GSOutputTopology)
            .field("GSMaxOutputVertexCount", &self.GSMaxOutputVertexCount)
            .field("InputPrimitive", &self.InputPrimitive)
            .field("PatchConstantParameters", &self.PatchConstantParameters)
            .field("cGSInstanceCount", &self.cGSInstanceCount)
            .field("cControlPoints", &self.cControlPoints)
            .field("HSOutputPrimitive", &self.HSOutputPrimitive)
            .field("HSPartitioning", &self.HSPartitioning)
            .field("TessellatorDomain", &self.TessellatorDomain)
            .field("cBarrierInstructions", &self.cBarrierInstructions)
            .field("cInterlockedInstructions", &self.cInterlockedInstructions)
            .field("cTextureStoreInstructions", &self.cTextureStoreInstructions)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D11_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D11_SHADER_INPUT_BIND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.BindPoint == other.BindPoint && self.BindCount == other.BindCount && self.uFlags == other.uFlags && self.ReturnType == other.ReturnType && self.Dimension == other.Dimension && self.NumSamples == other.NumSamples
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D11_SHADER_INPUT_BIND_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D11_SHADER_INPUT_BIND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SHADER_INPUT_BIND_DESC").field("Name", &self.Name).field("Type", &self.Type).field("BindPoint", &self.BindPoint).field("BindCount", &self.BindCount).field("uFlags", &self.uFlags).field("ReturnType", &self.ReturnType).field("Dimension", &self.Dimension).field("NumSamples", &self.NumSamples).finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_MIN_PRECISION_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHADER_MIN_PRECISION_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_MIN_PRECISION_SUPPORT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_SHADER_TRACKING_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHADER_TRACKING_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_TRACKING_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_TRACKING_RESOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHADER_TRACKING_RESOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_TRACKING_RESOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHADER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D11_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D11_SHADER_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Class == other.Class && self.Type == other.Type && self.Rows == other.Rows && self.Columns == other.Columns && self.Elements == other.Elements && self.Members == other.Members && self.Offset == other.Offset && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D11_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D11_SHADER_TYPE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SHADER_TYPE_DESC").field("Class", &self.Class).field("Type", &self.Type).field("Rows", &self.Rows).field("Columns", &self.Columns).field("Elements", &self.Elements).field("Members", &self.Members).field("Offset", &self.Offset).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_SHADER_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.StartOffset == other.StartOffset && self.Size == other.Size && self.uFlags == other.uFlags && self.DefaultValue == other.DefaultValue && self.StartTexture == other.StartTexture && self.TextureSize == other.TextureSize && self.StartSampler == other.StartSampler && self.SamplerSize == other.SamplerSize
    }
}
impl ::core::cmp::Eq for D3D11_SHADER_VARIABLE_DESC {}
impl ::core::fmt::Debug for D3D11_SHADER_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SHADER_VARIABLE_DESC").field("Name", &self.Name).field("StartOffset", &self.StartOffset).field("Size", &self.Size).field("uFlags", &self.uFlags).field("DefaultValue", &self.DefaultValue).field("StartTexture", &self.StartTexture).field("TextureSize", &self.TextureSize).field("StartSampler", &self.StartSampler).field("SamplerSize", &self.SamplerSize).finish()
    }
}
impl ::core::default::Default for D3D11_SHADER_VERSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHADER_VERSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHADER_VERSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_SHARED_RESOURCE_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_SHARED_RESOURCE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_SHARED_RESOURCE_TIER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D11_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D11_SIGNATURE_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.Register == other.Register && self.SystemValueType == other.SystemValueType && self.ComponentType == other.ComponentType && self.Mask == other.Mask && self.ReadWriteMask == other.ReadWriteMask && self.Stream == other.Stream && self.MinPrecision == other.MinPrecision
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D11_SIGNATURE_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D11_SIGNATURE_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SIGNATURE_PARAMETER_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Register", &self.Register).field("SystemValueType", &self.SystemValueType).field("ComponentType", &self.ComponentType).field("Mask", &self.Mask).field("ReadWriteMask", &self.ReadWriteMask).field("Stream", &self.Stream).field("MinPrecision", &self.MinPrecision).finish()
    }
}
impl ::core::default::Default for D3D11_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_SO_DECLARATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Stream == other.Stream && self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.StartComponent == other.StartComponent && self.ComponentCount == other.ComponentCount && self.OutputSlot == other.OutputSlot
    }
}
impl ::core::cmp::Eq for D3D11_SO_DECLARATION_ENTRY {}
impl ::core::fmt::Debug for D3D11_SO_DECLARATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SO_DECLARATION_ENTRY").field("Stream", &self.Stream).field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("StartComponent", &self.StartComponent).field("ComponentCount", &self.ComponentCount).field("OutputSlot", &self.OutputSlot).finish()
    }
}
impl ::core::default::Default for D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_STANDARD_MULTISAMPLE_QUALITY_LEVELS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_STENCIL_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_STENCIL_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_STENCIL_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_SUBRESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pSysMem == other.pSysMem && self.SysMemPitch == other.SysMemPitch && self.SysMemSlicePitch == other.SysMemSlicePitch
    }
}
impl ::core::cmp::Eq for D3D11_SUBRESOURCE_DATA {}
impl ::core::fmt::Debug for D3D11_SUBRESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SUBRESOURCE_DATA").field("pSysMem", &self.pSysMem).field("SysMemPitch", &self.SysMemPitch).field("SysMemSlicePitch", &self.SysMemSlicePitch).finish()
    }
}
impl ::core::default::Default for D3D11_SUBRESOURCE_TILING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_SUBRESOURCE_TILING {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInTiles == other.WidthInTiles && self.HeightInTiles == other.HeightInTiles && self.DepthInTiles == other.DepthInTiles && self.StartTileIndexInOverallResource == other.StartTileIndexInOverallResource
    }
}
impl ::core::cmp::Eq for D3D11_SUBRESOURCE_TILING {}
impl ::core::fmt::Debug for D3D11_SUBRESOURCE_TILING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_SUBRESOURCE_TILING").field("WidthInTiles", &self.WidthInTiles).field("HeightInTiles", &self.HeightInTiles).field("DepthInTiles", &self.DepthInTiles).field("StartTileIndexInOverallResource", &self.StartTileIndexInOverallResource).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_ARRAY_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_ARRAY_UAV {}
impl ::core::fmt::Debug for D3D11_TEX1D_ARRAY_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_ARRAY_UAV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_DSV {}
impl ::core::fmt::Debug for D3D11_TEX1D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_RTV {}
impl ::core::fmt::Debug for D3D11_TEX1D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_SRV {}
impl ::core::fmt::Debug for D3D11_TEX1D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
impl ::core::default::Default for D3D11_TEX1D_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX1D_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX1D_UAV {}
impl ::core::fmt::Debug for D3D11_TEX1D_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX1D_UAV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2DMS_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2DMS_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D11_TEX2DMS_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2DMS_ARRAY_DSV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2DMS_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2DMS_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D11_TEX2DMS_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2DMS_ARRAY_RTV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2DMS_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2DMS_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D11_TEX2DMS_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2DMS_ARRAY_SRV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2DMS_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D11_TEX2DMS_DSV {}
impl ::core::fmt::Debug for D3D11_TEX2DMS_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2DMS_DSV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2DMS_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D11_TEX2DMS_RTV {}
impl ::core::fmt::Debug for D3D11_TEX2DMS_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2DMS_RTV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2DMS_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D11_TEX2DMS_SRV {}
impl ::core::fmt::Debug for D3D11_TEX2DMS_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2DMS_SRV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_RTV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_RTV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_RTV1 {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_RTV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_RTV1").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_SRV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_SRV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_SRV1 {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_SRV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_SRV1").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_UAV {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_UAV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_UAV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_UAV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_UAV1 {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_UAV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_UAV1").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_ARRAY_VPOV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_ARRAY_VPOV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_ARRAY_VPOV {}
impl ::core::fmt::Debug for D3D11_TEX2D_ARRAY_VPOV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_ARRAY_VPOV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_DSV {}
impl ::core::fmt::Debug for D3D11_TEX2D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_RTV {}
impl ::core::fmt::Debug for D3D11_TEX2D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_RTV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_RTV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_RTV1 {}
impl ::core::fmt::Debug for D3D11_TEX2D_RTV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_RTV1").field("MipSlice", &self.MipSlice).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_SRV {}
impl ::core::fmt::Debug for D3D11_TEX2D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_SRV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_SRV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_SRV1 {}
impl ::core::fmt::Debug for D3D11_TEX2D_SRV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_SRV1").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_UAV {}
impl ::core::fmt::Debug for D3D11_TEX2D_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_UAV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_UAV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_UAV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_UAV1 {}
impl ::core::fmt::Debug for D3D11_TEX2D_UAV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_UAV1").field("MipSlice", &self.MipSlice).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_VDOV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_VDOV {
    fn eq(&self, other: &Self) -> bool {
        self.ArraySlice == other.ArraySlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_VDOV {}
impl ::core::fmt::Debug for D3D11_TEX2D_VDOV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_VDOV").field("ArraySlice", &self.ArraySlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_VPIV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_VPIV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.ArraySlice == other.ArraySlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_VPIV {}
impl ::core::fmt::Debug for D3D11_TEX2D_VPIV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_VPIV").field("MipSlice", &self.MipSlice).field("ArraySlice", &self.ArraySlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX2D_VPOV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX2D_VPOV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D11_TEX2D_VPOV {}
impl ::core::fmt::Debug for D3D11_TEX2D_VPOV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX2D_VPOV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D11_TEX3D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX3D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstWSlice == other.FirstWSlice && self.WSize == other.WSize
    }
}
impl ::core::cmp::Eq for D3D11_TEX3D_RTV {}
impl ::core::fmt::Debug for D3D11_TEX3D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX3D_RTV").field("MipSlice", &self.MipSlice).field("FirstWSlice", &self.FirstWSlice).field("WSize", &self.WSize).finish()
    }
}
impl ::core::default::Default for D3D11_TEX3D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX3D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D11_TEX3D_SRV {}
impl ::core::fmt::Debug for D3D11_TEX3D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX3D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
impl ::core::default::Default for D3D11_TEX3D_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEX3D_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstWSlice == other.FirstWSlice && self.WSize == other.WSize
    }
}
impl ::core::cmp::Eq for D3D11_TEX3D_UAV {}
impl ::core::fmt::Debug for D3D11_TEX3D_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEX3D_UAV").field("MipSlice", &self.MipSlice).field("FirstWSlice", &self.FirstWSlice).field("WSize", &self.WSize).finish()
    }
}
impl ::core::default::Default for D3D11_TEXCUBE_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEXCUBE_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.First2DArrayFace == other.First2DArrayFace && self.NumCubes == other.NumCubes
    }
}
impl ::core::cmp::Eq for D3D11_TEXCUBE_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D11_TEXCUBE_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXCUBE_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("First2DArrayFace", &self.First2DArrayFace).field("NumCubes", &self.NumCubes).finish()
    }
}
impl ::core::default::Default for D3D11_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TEXCUBE_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D11_TEXCUBE_SRV {}
impl ::core::fmt::Debug for D3D11_TEXCUBE_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXCUBE_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_TEXTURE1D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_TEXTURE1D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.MipLevels == other.MipLevels && self.ArraySize == other.ArraySize && self.Format == other.Format && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_TEXTURE1D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_TEXTURE1D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXTURE1D_DESC").field("Width", &self.Width).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_TEXTURE2D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_TEXTURE2D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.MipLevels == other.MipLevels && self.ArraySize == other.ArraySize && self.Format == other.Format && self.SampleDesc == other.SampleDesc && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_TEXTURE2D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_TEXTURE2D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXTURE2D_DESC").field("Width", &self.Width).field("Height", &self.Height).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_TEXTURE2D_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_TEXTURE2D_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.MipLevels == other.MipLevels && self.ArraySize == other.ArraySize && self.Format == other.Format && self.SampleDesc == other.SampleDesc && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags && self.TextureLayout == other.TextureLayout
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_TEXTURE2D_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_TEXTURE2D_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXTURE2D_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).field("TextureLayout", &self.TextureLayout).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_TEXTURE3D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_TEXTURE3D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth && self.MipLevels == other.MipLevels && self.Format == other.Format && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_TEXTURE3D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_TEXTURE3D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXTURE3D_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("MipLevels", &self.MipLevels).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_TEXTURE3D_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_TEXTURE3D_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth && self.MipLevels == other.MipLevels && self.Format == other.Format && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags && self.TextureLayout == other.TextureLayout
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_TEXTURE3D_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_TEXTURE3D_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TEXTURE3D_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("MipLevels", &self.MipLevels).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).field("TextureLayout", &self.TextureLayout).finish()
    }
}
impl ::core::default::Default for D3D11_TEXTURECUBE_FACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TEXTURECUBE_FACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TEXTURECUBE_FACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TEXTURE_ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TEXTURE_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TEXTURE_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TEXTURE_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TILED_RESOURCES_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TILED_RESOURCES_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TILED_RESOURCES_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TILED_RESOURCE_COORDINATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TILED_RESOURCE_COORDINATE {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.Subresource == other.Subresource
    }
}
impl ::core::cmp::Eq for D3D11_TILED_RESOURCE_COORDINATE {}
impl ::core::fmt::Debug for D3D11_TILED_RESOURCE_COORDINATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TILED_RESOURCE_COORDINATE").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("Subresource", &self.Subresource).finish()
    }
}
impl ::core::default::Default for D3D11_TILE_COPY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TILE_COPY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TILE_COPY_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TILE_MAPPING_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TILE_MAPPING_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TILE_MAPPING_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TILE_RANGE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TILE_RANGE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TILE_RANGE_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_TILE_REGION_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_TILE_REGION_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.NumTiles == other.NumTiles && self.bUseBox == other.bUseBox && self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_TILE_REGION_SIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_TILE_REGION_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TILE_REGION_SIZE").field("NumTiles", &self.NumTiles).field("bUseBox", &self.bUseBox).field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).finish()
    }
}
impl ::core::default::Default for D3D11_TILE_SHAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TILE_SHAPE {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInTexels == other.WidthInTexels && self.HeightInTexels == other.HeightInTexels && self.DepthInTexels == other.DepthInTexels
    }
}
impl ::core::cmp::Eq for D3D11_TILE_SHAPE {}
impl ::core::fmt::Debug for D3D11_TILE_SHAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TILE_SHAPE").field("WidthInTexels", &self.WidthInTexels).field("HeightInTexels", &self.HeightInTexels).field("DepthInTexels", &self.DepthInTexels).finish()
    }
}
impl ::core::default::Default for D3D11_TRACE_GS_INPUT_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TRACE_GS_INPUT_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TRACE_GS_INPUT_PRIMITIVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_TRACE_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_TRACE_REGISTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_TRACE_REGISTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_TRACE_REGISTER_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_TRACE_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_TRACE_STEP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_TRACE_STEP {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.InstructionActive == other.InstructionActive && self.NumRegistersWritten == other.NumRegistersWritten && self.NumRegistersRead == other.NumRegistersRead && self.MiscOperations == other.MiscOperations && self.OpcodeType == other.OpcodeType && self.CurrentGlobalCycle == other.CurrentGlobalCycle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_TRACE_STEP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_TRACE_STEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TRACE_STEP").field("ID", &self.ID).field("InstructionActive", &self.InstructionActive).field("NumRegistersWritten", &self.NumRegistersWritten).field("NumRegistersRead", &self.NumRegistersRead).field("MiscOperations", &self.MiscOperations).field("OpcodeType", &self.OpcodeType).field("CurrentGlobalCycle", &self.CurrentGlobalCycle).finish()
    }
}
impl ::core::default::Default for D3D11_TRACE_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_TRACE_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Bits == other.Bits && self.ValidMask == other.ValidMask
    }
}
impl ::core::cmp::Eq for D3D11_TRACE_VALUE {}
impl ::core::fmt::Debug for D3D11_TRACE_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_TRACE_VALUE").field("Bits", &self.Bits).field("ValidMask", &self.ValidMask).finish()
    }
}
impl ::core::default::Default for D3D11_UAV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_UAV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_UAV_DIMENSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VDOV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VDOV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VDOV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VERTEX_SHADER_TRACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VERTEX_SHADER_TRACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Invocation == other.Invocation
    }
}
impl ::core::cmp::Eq for D3D11_VERTEX_SHADER_TRACE_DESC {}
impl ::core::fmt::Debug for D3D11_VERTEX_SHADER_TRACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VERTEX_SHADER_TRACE_DESC").field("Invocation", &self.Invocation).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_VIDEO_COLOR_RGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_COLOR_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.R == other.R && self.G == other.G && self.B == other.B && self.A == other.A
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_COLOR_RGBA {}
impl ::core::fmt::Debug for D3D11_VIDEO_COLOR_RGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_COLOR_RGBA").field("R", &self.R).field("G", &self.G).field("B", &self.B).field("A", &self.A).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_COLOR_YCbCrA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_COLOR_YCbCrA {
    fn eq(&self, other: &Self) -> bool {
        self.Y == other.Y && self.Cb == other.Cb && self.Cr == other.Cr && self.A == other.A
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_COLOR_YCbCrA {}
impl ::core::fmt::Debug for D3D11_VIDEO_COLOR_YCbCrA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_COLOR_YCbCrA").field("Y", &self.Y).field("Cb", &self.Cb).field("Cr", &self.Cr).field("A", &self.A).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Caps == other.Caps && self.KeyExchangeTypeCount == other.KeyExchangeTypeCount && self.BlockAlignmentSize == other.BlockAlignmentSize && self.ProtectedMemorySize == other.ProtectedMemorySize
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {}
impl ::core::fmt::Debug for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_CONTENT_PROTECTION_CAPS").field("Caps", &self.Caps).field("KeyExchangeTypeCount", &self.KeyExchangeTypeCount).field("BlockAlignmentSize", &self.BlockAlignmentSize).field("ProtectedMemorySize", &self.ProtectedMemorySize).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn eq(&self, other: &Self) -> bool {
        self.pCryptoSession == other.pCryptoSession && self.BlobSize == other.BlobSize && self.pBlob == other.pBlob && self.pKeyInfoId == other.pKeyInfoId && self.PrivateDataSize == other.PrivateDataSize && self.pPrivateData == other.pPrivateData
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION").field("pCryptoSession", &self.pCryptoSession).field("BlobSize", &self.BlobSize).field("pBlob", &self.pBlob).field("pKeyInfoId", &self.pKeyInfoId).field("PrivateDataSize", &self.PrivateDataSize).field("pPrivateData", &self.pPrivateData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_VIDEO_DECODER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferType == other.BufferType && self.BufferIndex == other.BufferIndex && self.DataOffset == other.DataOffset && self.DataSize == other.DataSize && self.FirstMBaddress == other.FirstMBaddress && self.NumMBsInBuffer == other.NumMBsInBuffer && self.Width == other.Width && self.Height == other.Height && self.Stride == other.Stride && self.ReservedBits == other.ReservedBits && self.pIV == other.pIV && self.IVSize == other.IVSize && self.PartialEncryption == other.PartialEncryption && self.EncryptedBlockInfo == other.EncryptedBlockInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_BUFFER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_BUFFER_DESC")
            .field("BufferType", &self.BufferType)
            .field("BufferIndex", &self.BufferIndex)
            .field("DataOffset", &self.DataOffset)
            .field("DataSize", &self.DataSize)
            .field("FirstMBaddress", &self.FirstMBaddress)
            .field("NumMBsInBuffer", &self.NumMBsInBuffer)
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .field("Stride", &self.Stride)
            .field("ReservedBits", &self.ReservedBits)
            .field("pIV", &self.pIV)
            .field("IVSize", &self.IVSize)
            .field("PartialEncryption", &self.PartialEncryption)
            .field("EncryptedBlockInfo", &self.EncryptedBlockInfo)
            .finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.BufferType == other.BufferType && self.DataOffset == other.DataOffset && self.DataSize == other.DataSize && self.pIV == other.pIV && self.IVSize == other.IVSize && self.pSubSampleMappingBlock == other.pSubSampleMappingBlock && self.SubSampleMappingCount == other.SubSampleMappingCount
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_BUFFER_DESC1 {}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_BUFFER_DESC1").field("BufferType", &self.BufferType).field("DataOffset", &self.DataOffset).field("DataSize", &self.DataSize).field("pIV", &self.pIV).field("IVSize", &self.IVSize).field("pSubSampleMappingBlock", &self.pSubSampleMappingBlock).field("SubSampleMappingCount", &self.SubSampleMappingCount).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.BufferType == other.BufferType && self.DataOffset == other.DataOffset && self.DataSize == other.DataSize && self.pIV == other.pIV && self.IVSize == other.IVSize && self.pSubSampleMappingBlock == other.pSubSampleMappingBlock && self.SubSampleMappingCount == other.SubSampleMappingCount && self.cBlocksStripeEncrypted == other.cBlocksStripeEncrypted && self.cBlocksStripeClear == other.cBlocksStripeClear
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_BUFFER_DESC2 {}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_BUFFER_DESC2").field("BufferType", &self.BufferType).field("DataOffset", &self.DataOffset).field("DataSize", &self.DataSize).field("pIV", &self.pIV).field("IVSize", &self.IVSize).field("pSubSampleMappingBlock", &self.pSubSampleMappingBlock).field("SubSampleMappingCount", &self.SubSampleMappingCount).field("cBlocksStripeEncrypted", &self.cBlocksStripeEncrypted).field("cBlocksStripeClear", &self.cBlocksStripeClear).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_BUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_BUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_BUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.guidConfigBitstreamEncryption == other.guidConfigBitstreamEncryption
            && self.guidConfigMBcontrolEncryption == other.guidConfigMBcontrolEncryption
            && self.guidConfigResidDiffEncryption == other.guidConfigResidDiffEncryption
            && self.ConfigBitstreamRaw == other.ConfigBitstreamRaw
            && self.ConfigMBcontrolRasterOrder == other.ConfigMBcontrolRasterOrder
            && self.ConfigResidDiffHost == other.ConfigResidDiffHost
            && self.ConfigSpatialResid8 == other.ConfigSpatialResid8
            && self.ConfigResid8Subtraction == other.ConfigResid8Subtraction
            && self.ConfigSpatialHost8or9Clipping == other.ConfigSpatialHost8or9Clipping
            && self.ConfigSpatialResidInterleaved == other.ConfigSpatialResidInterleaved
            && self.ConfigIntraResidUnsigned == other.ConfigIntraResidUnsigned
            && self.ConfigResidDiffAccelerator == other.ConfigResidDiffAccelerator
            && self.ConfigHostInverseScan == other.ConfigHostInverseScan
            && self.ConfigSpecificIDCT == other.ConfigSpecificIDCT
            && self.Config4GroupedCoefs == other.Config4GroupedCoefs
            && self.ConfigMinRenderTargetBuffCount == other.ConfigMinRenderTargetBuffCount
            && self.ConfigDecoderSpecific == other.ConfigDecoderSpecific
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_CONFIG {}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_CONFIG")
            .field("guidConfigBitstreamEncryption", &self.guidConfigBitstreamEncryption)
            .field("guidConfigMBcontrolEncryption", &self.guidConfigMBcontrolEncryption)
            .field("guidConfigResidDiffEncryption", &self.guidConfigResidDiffEncryption)
            .field("ConfigBitstreamRaw", &self.ConfigBitstreamRaw)
            .field("ConfigMBcontrolRasterOrder", &self.ConfigMBcontrolRasterOrder)
            .field("ConfigResidDiffHost", &self.ConfigResidDiffHost)
            .field("ConfigSpatialResid8", &self.ConfigSpatialResid8)
            .field("ConfigResid8Subtraction", &self.ConfigResid8Subtraction)
            .field("ConfigSpatialHost8or9Clipping", &self.ConfigSpatialHost8or9Clipping)
            .field("ConfigSpatialResidInterleaved", &self.ConfigSpatialResidInterleaved)
            .field("ConfigIntraResidUnsigned", &self.ConfigIntraResidUnsigned)
            .field("ConfigResidDiffAccelerator", &self.ConfigResidDiffAccelerator)
            .field("ConfigHostInverseScan", &self.ConfigHostInverseScan)
            .field("ConfigSpecificIDCT", &self.ConfigSpecificIDCT)
            .field("Config4GroupedCoefs", &self.Config4GroupedCoefs)
            .field("ConfigMinRenderTargetBuffCount", &self.ConfigMinRenderTargetBuffCount)
            .field("ConfigDecoderSpecific", &self.ConfigDecoderSpecific)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_VIDEO_DECODER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Guid == other.Guid && self.SampleWidth == other.SampleWidth && self.SampleHeight == other.SampleHeight && self.OutputFormat == other.OutputFormat
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_DESC").field("Guid", &self.Guid).field("SampleWidth", &self.SampleWidth).field("SampleHeight", &self.SampleHeight).field("OutputFormat", &self.OutputFormat).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.pPrivateInputData == other.pPrivateInputData && self.PrivateInputDataSize == other.PrivateInputDataSize && self.pPrivateOutputData == other.pPrivateOutputData && self.PrivateOutputDataSize == other.PrivateOutputDataSize && self.ResourceCount == other.ResourceCount && self.ppResourceList == other.ppResourceList
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_EXTENSION {}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_EXTENSION").field("Function", &self.Function).field("pPrivateInputData", &self.pPrivateInputData).field("PrivateInputDataSize", &self.PrivateInputDataSize).field("pPrivateOutputData", &self.pPrivateOutputData).field("PrivateOutputDataSize", &self.PrivateOutputDataSize).field("ResourceCount", &self.ResourceCount).field("ppResourceList", &self.ppResourceList).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.ClearSize == other.ClearSize && self.EncryptedSize == other.EncryptedSize
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {}
impl ::core::fmt::Debug for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK").field("ClearSize", &self.ClearSize).field("EncryptedSize", &self.EncryptedSize).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_FRAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_FRAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_FRAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceCaps == other.DeviceCaps && self.FeatureCaps == other.FeatureCaps && self.FilterCaps == other.FilterCaps && self.InputFormatCaps == other.InputFormatCaps && self.AutoStreamCaps == other.AutoStreamCaps && self.StereoCaps == other.StereoCaps && self.RateConversionCapsCount == other.RateConversionCapsCount && self.MaxInputStreams == other.MaxInputStreams && self.MaxStreamStates == other.MaxStreamStates
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_CAPS {}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_CAPS").field("DeviceCaps", &self.DeviceCaps).field("FeatureCaps", &self.FeatureCaps).field("FilterCaps", &self.FilterCaps).field("InputFormatCaps", &self.InputFormatCaps).field("AutoStreamCaps", &self.AutoStreamCaps).field("StereoCaps", &self.StereoCaps).field("RateConversionCapsCount", &self.RateConversionCapsCount).field("MaxInputStreams", &self.MaxInputStreams).field("MaxStreamStates", &self.MaxStreamStates).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_COLOR_SPACE").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.InputFrameFormat == other.InputFrameFormat && self.InputFrameRate == other.InputFrameRate && self.InputWidth == other.InputWidth && self.InputHeight == other.InputHeight && self.OutputFrameRate == other.OutputFrameRate && self.OutputWidth == other.OutputWidth && self.OutputHeight == other.OutputHeight && self.Usage == other.Usage
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_CONTENT_DESC").field("InputFrameFormat", &self.InputFrameFormat).field("InputFrameRate", &self.InputFrameRate).field("InputWidth", &self.InputWidth).field("InputHeight", &self.InputHeight).field("OutputFrameRate", &self.OutputFrameRate).field("OutputWidth", &self.OutputWidth).field("OutputHeight", &self.OutputHeight).field("Usage", &self.Usage).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    fn eq(&self, other: &Self) -> bool {
        self.CustomRate == other.CustomRate && self.OutputFrames == other.OutputFrames && self.InputInterlaced == other.InputInterlaced && self.InputFramesOrFields == other.InputFramesOrFields
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_CUSTOM_RATE").field("CustomRate", &self.CustomRate).field("OutputFrames", &self.OutputFrames).field("InputInterlaced", &self.InputInterlaced).field("InputFramesOrFields", &self.InputFramesOrFields).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_DEVICE_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_DEVICE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_DEVICE_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_FEATURE_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FEATURE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FEATURE_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_FILTER_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FILTER_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FILTER_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Minimum == other.Minimum && self.Maximum == other.Maximum && self.Default == other.Default && self.Multiplier == other.Multiplier
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_FILTER_RANGE").field("Minimum", &self.Minimum).field("Maximum", &self.Maximum).field("Default", &self.Default).field("Multiplier", &self.Multiplier).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_FORMAT_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FORMAT_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FORMAT_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_OUTPUT_RATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_OUTPUT_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_OUTPUT_RATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.PastFrames == other.PastFrames && self.FutureFrames == other.FutureFrames && self.ProcessorCaps == other.ProcessorCaps && self.ITelecineCaps == other.ITelecineCaps && self.CustomRateCount == other.CustomRateCount
    }
}
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS").field("PastFrames", &self.PastFrames).field("FutureFrames", &self.FutureFrames).field("ProcessorCaps", &self.ProcessorCaps).field("ITelecineCaps", &self.ITelecineCaps).field("CustomRateCount", &self.CustomRateCount).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_ROTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_ROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_ROTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_STEREO_CAPS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STEREO_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_STEREO_CAPS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_STEREO_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STEREO_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_PROCESSOR_STEREO_FORMAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.OutputIndex == other.OutputIndex && self.InputFrameOrField == other.InputFrameOrField && self.PastFrames == other.PastFrames && self.FutureFrames == other.FutureFrames && self.ppPastSurfaces == other.ppPastSurfaces && self.pInputSurface == other.pInputSurface && self.ppFutureSurfaces == other.ppFutureSurfaces && self.ppPastSurfacesRight == other.ppPastSurfacesRight && self.pInputSurfaceRight == other.pInputSurfaceRight && self.ppFutureSurfacesRight == other.ppFutureSurfacesRight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_STREAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_STREAM")
            .field("Enable", &self.Enable)
            .field("OutputIndex", &self.OutputIndex)
            .field("InputFrameOrField", &self.InputFrameOrField)
            .field("PastFrames", &self.PastFrames)
            .field("FutureFrames", &self.FutureFrames)
            .field("ppPastSurfaces", &self.ppPastSurfaces)
            .field("pInputSurface", &self.pInputSurface)
            .field("ppFutureSurfaces", &self.ppFutureSurfaces)
            .field("ppPastSurfacesRight", &self.ppPastSurfacesRight)
            .field("pInputSurfaceRight", &self.pInputSurfaceRight)
            .field("ppFutureSurfacesRight", &self.ppFutureSurfacesRight)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    fn eq(&self, other: &Self) -> bool {
        self.Enable == other.Enable && self.Width == other.Width && self.Height == other.Height && self.Format == other.Format
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT").field("Enable", &self.Enable).field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D11_VIDEO_SAMPLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D11_VIDEO_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.ColorSpace == other.ColorSpace
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D11_VIDEO_SAMPLE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D11_VIDEO_SAMPLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIDEO_SAMPLE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("ColorSpace", &self.ColorSpace).finish()
    }
}
impl ::core::default::Default for D3D11_VIDEO_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VIDEO_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VIDEO_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VIEWPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D11_VIEWPORT {
    fn eq(&self, other: &Self) -> bool {
        self.TopLeftX == other.TopLeftX && self.TopLeftY == other.TopLeftY && self.Width == other.Width && self.Height == other.Height && self.MinDepth == other.MinDepth && self.MaxDepth == other.MaxDepth
    }
}
impl ::core::cmp::Eq for D3D11_VIEWPORT {}
impl ::core::fmt::Debug for D3D11_VIEWPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D11_VIEWPORT").field("TopLeftX", &self.TopLeftX).field("TopLeftY", &self.TopLeftY).field("Width", &self.Width).field("Height", &self.Height).field("MinDepth", &self.MinDepth).field("MaxDepth", &self.MaxDepth).finish()
    }
}
impl ::core::default::Default for D3D11_VPIV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VPIV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VPIV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D11_VPOV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D11_VPOV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D11_VPOV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DX11_FFT_BUFFER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DX11_FFT_BUFFER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumTempBufferSizes == other.NumTempBufferSizes && self.TempBufferFloatSizes == other.TempBufferFloatSizes && self.NumPrecomputeBufferSizes == other.NumPrecomputeBufferSizes && self.PrecomputeBufferFloatSizes == other.PrecomputeBufferFloatSizes
    }
}
impl ::core::cmp::Eq for D3DX11_FFT_BUFFER_INFO {}
impl ::core::fmt::Debug for D3DX11_FFT_BUFFER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DX11_FFT_BUFFER_INFO").field("NumTempBufferSizes", &self.NumTempBufferSizes).field("TempBufferFloatSizes", &self.TempBufferFloatSizes).field("NumPrecomputeBufferSizes", &self.NumPrecomputeBufferSizes).field("PrecomputeBufferFloatSizes", &self.PrecomputeBufferFloatSizes).finish()
    }
}
impl ::core::default::Default for D3DX11_FFT_CREATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DX11_FFT_CREATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DX11_FFT_CREATE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DX11_FFT_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DX11_FFT_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DX11_FFT_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DX11_FFT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DX11_FFT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumDimensions == other.NumDimensions && self.ElementLengths == other.ElementLengths && self.DimensionMask == other.DimensionMask && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for D3DX11_FFT_DESC {}
impl ::core::fmt::Debug for D3DX11_FFT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DX11_FFT_DESC").field("NumDimensions", &self.NumDimensions).field("ElementLengths", &self.ElementLengths).field("DimensionMask", &self.DimensionMask).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for D3DX11_FFT_DIM_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DX11_FFT_DIM_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DX11_FFT_DIM_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DX11_SCAN_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DX11_SCAN_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DX11_SCAN_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DX11_SCAN_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DX11_SCAN_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DX11_SCAN_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3DX11_SCAN_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DX11_SCAN_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DX11_SCAN_OPCODE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11Asynchronous {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Asynchronous {}
impl ::core::fmt::Debug for ID3D11Asynchronous {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Asynchronous").field(&self.0).finish()
    }
}
impl ID3D11Asynchronous {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11AuthenticatedChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11AuthenticatedChannel {}
impl ::core::fmt::Debug for ID3D11AuthenticatedChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11AuthenticatedChannel").field(&self.0).finish()
    }
}
impl ID3D11AuthenticatedChannel {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11BlendState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11BlendState {}
impl ::core::fmt::Debug for ID3D11BlendState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11BlendState").field(&self.0).finish()
    }
}
impl ID3D11BlendState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11BlendState1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11BlendState1 {}
impl ::core::fmt::Debug for ID3D11BlendState1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11BlendState1").field(&self.0).finish()
    }
}
impl ID3D11BlendState1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_BLEND_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Buffer {}
impl ::core::fmt::Debug for ID3D11Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Buffer").field(&self.0).finish()
    }
}
impl ID3D11Buffer {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11ClassInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ClassInstance {}
impl ::core::fmt::Debug for ID3D11ClassInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ClassInstance").field(&self.0).finish()
    }
}
impl ID3D11ClassInstance {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11ClassLinkage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ClassLinkage {}
impl ::core::fmt::Debug for ID3D11ClassLinkage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ClassLinkage").field(&self.0).finish()
    }
}
impl ID3D11ClassLinkage {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11CommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11CommandList {}
impl ::core::fmt::Debug for ID3D11CommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11CommandList").field(&self.0).finish()
    }
}
impl ID3D11CommandList {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11ComputeShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ComputeShader {}
impl ::core::fmt::Debug for ID3D11ComputeShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ComputeShader").field(&self.0).finish()
    }
}
impl ID3D11ComputeShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11Counter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Counter {}
impl ::core::fmt::Debug for ID3D11Counter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Counter").field(&self.0).finish()
    }
}
impl ID3D11Counter {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11CryptoSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11CryptoSession {}
impl ::core::fmt::Debug for ID3D11CryptoSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11CryptoSession").field(&self.0).finish()
    }
}
impl ID3D11CryptoSession {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11Debug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Debug {}
impl ::core::fmt::Debug for ID3D11Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Debug").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11DepthStencilState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DepthStencilState {}
impl ::core::fmt::Debug for ID3D11DepthStencilState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DepthStencilState").field(&self.0).finish()
    }
}
impl ID3D11DepthStencilState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11DepthStencilView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DepthStencilView {}
impl ::core::fmt::Debug for ID3D11DepthStencilView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DepthStencilView").field(&self.0).finish()
    }
}
impl ID3D11DepthStencilView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Device {}
impl ::core::fmt::Debug for ID3D11Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Device").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Device1 {}
impl ::core::fmt::Debug for ID3D11Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Device1").field(&self.0).finish()
    }
}
impl ID3D11Device1 {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateBuffer)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture1D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateTexture1D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture1d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateTexture2D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateTexture3D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: ::core::option::Option<*mut ::core::option::Option<ID3D11InputLayout>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateInputLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len() as _, ::core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(ppinputlayout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppvertexshader: ::core::option::Option<*mut ::core::option::Option<ID3D11VertexShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppvertexshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateGeometryShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P0>(&self, pshaderbytecode: &[u8], psodeclaration: ::core::option::Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: ::core::option::Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateGeometryShaderWithStreamOutput)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(pshaderbytecode.as_ptr()),
            pshaderbytecode.len() as _,
            ::core::mem::transmute(psodeclaration.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            psodeclaration.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pbufferstrides.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            pbufferstrides.as_deref().map_or(0, |slice| slice.len() as _),
            rasterizedstream,
            pclasslinkage.into().abi(),
            ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CreatePixelShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pppixelshader: ::core::option::Option<*mut ::core::option::Option<ID3D11PixelShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pppixelshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateHullShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pphullshader: ::core::option::Option<*mut ::core::option::Option<ID3D11HullShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateHullShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pphullshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDomainShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppdomainshader: ::core::option::Option<*mut ::core::option::Option<ID3D11DomainShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDomainShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppdomainshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateComputeShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppcomputeshader: ::core::option::Option<*mut ::core::option::Option<ID3D11ComputeShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateComputeShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppcomputeshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateClassLinkage(&self) -> ::windows::core::Result<ID3D11ClassLinkage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateClassLinkage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateBlendState)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencildesc, ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateRasterizerState)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11SamplerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSamplerState)(::windows::core::Vtable::as_raw(self), psamplerdesc, ::core::mem::transmute(ppsamplerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: ::core::option::Option<*mut ::core::option::Option<ID3D11Query>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateQuery)(::windows::core::Vtable::as_raw(self), pquerydesc, ::core::mem::transmute(ppquery.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreatePredicate)(::windows::core::Vtable::as_raw(self), ppredicatedesc, ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: ::core::option::Option<*mut ::core::option::Option<ID3D11Counter>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateCounter)(::windows::core::Vtable::as_raw(self), pcounterdesc, ::core::mem::transmute(ppcounter.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDeferredContext)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<P0, T>(&self, hresource: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenSharedResource)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckFormatSupport)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckMultisampleQualityLevels)(::windows::core::Vtable::as_raw(self), format, samplecount, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckCounterInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: ::core::option::Option<*mut u32>, szunits: ::windows::core::PSTR, punitslength: ::core::option::Option<*mut u32>, szdescription: ::windows::core::PSTR, pdescriptionlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckCounter)(::windows::core::Vtable::as_raw(self), pdesc, ptype, pactivecounters, ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        (::windows::core::Vtable::vtable(self).base__.GetFeatureLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetImmediateContext(&self) -> ::windows::core::Result<ID3D11DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImmediateContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetExceptionMode)(::windows::core::Vtable::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetExceptionMode)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11Device2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Device2 {}
impl ::core::fmt::Debug for ID3D11Device2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Device2").field(&self.0).finish()
    }
}
impl ID3D11Device2 {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBuffer)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture1D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTexture1D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture1d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTexture2D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTexture3D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: ::core::option::Option<*mut ::core::option::Option<ID3D11InputLayout>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateInputLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len() as _, ::core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(ppinputlayout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppvertexshader: ::core::option::Option<*mut ::core::option::Option<ID3D11VertexShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppvertexshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGeometryShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P0>(&self, pshaderbytecode: &[u8], psodeclaration: ::core::option::Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: ::core::option::Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGeometryShaderWithStreamOutput)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(pshaderbytecode.as_ptr()),
            pshaderbytecode.len() as _,
            ::core::mem::transmute(psodeclaration.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            psodeclaration.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pbufferstrides.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            pbufferstrides.as_deref().map_or(0, |slice| slice.len() as _),
            rasterizedstream,
            pclasslinkage.into().abi(),
            ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CreatePixelShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pppixelshader: ::core::option::Option<*mut ::core::option::Option<ID3D11PixelShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pppixelshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateHullShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pphullshader: ::core::option::Option<*mut ::core::option::Option<ID3D11HullShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateHullShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pphullshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDomainShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppdomainshader: ::core::option::Option<*mut ::core::option::Option<ID3D11DomainShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDomainShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppdomainshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateComputeShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppcomputeshader: ::core::option::Option<*mut ::core::option::Option<ID3D11ComputeShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateComputeShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppcomputeshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateClassLinkage(&self) -> ::windows::core::Result<ID3D11ClassLinkage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateClassLinkage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBlendState)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencildesc, ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRasterizerState)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11SamplerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSamplerState)(::windows::core::Vtable::as_raw(self), psamplerdesc, ::core::mem::transmute(ppsamplerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: ::core::option::Option<*mut ::core::option::Option<ID3D11Query>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateQuery)(::windows::core::Vtable::as_raw(self), pquerydesc, ::core::mem::transmute(ppquery.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePredicate)(::windows::core::Vtable::as_raw(self), ppredicatedesc, ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: ::core::option::Option<*mut ::core::option::Option<ID3D11Counter>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCounter)(::windows::core::Vtable::as_raw(self), pcounterdesc, ::core::mem::transmute(ppcounter.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeferredContext)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<P0, T>(&self, hresource: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenSharedResource)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckFormatSupport)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckMultisampleQualityLevels)(::windows::core::Vtable::as_raw(self), format, samplecount, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckCounterInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: ::core::option::Option<*mut u32>, szunits: ::windows::core::PSTR, punitslength: ::core::option::Option<*mut u32>, szdescription: ::windows::core::PSTR, pdescriptionlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckCounter)(::windows::core::Vtable::as_raw(self), pdesc, ptype, pactivecounters, ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFeatureLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetImmediateContext(&self) -> ::windows::core::Result<ID3D11DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetImmediateContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetExceptionMode)(::windows::core::Vtable::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetExceptionMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetImmediateContext1(&self) -> ::windows::core::Result<ID3D11DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImmediateContext1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext1 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDeferredContext1)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateBlendState1)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateRasterizerState1)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: ::core::option::Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppcontextstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDeviceContextState)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute(pfeaturelevels.as_ptr()), pfeaturelevels.len() as _, sdkversion, emulatedinterface, ::core::mem::transmute(pchosenfeaturelevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppcontextstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource1<P0, T>(&self, hresource: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenSharedResource1)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenSharedResourceByName<P0, T>(&self, lpname: P0, dwdesiredaccess: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenSharedResourceByName)(::windows::core::Vtable::as_raw(self), lpname.into().abi(), dwdesiredaccess, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D11Device3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Device3 {}
impl ::core::fmt::Debug for ID3D11Device3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Device3").field(&self.0).finish()
    }
}
impl ID3D11Device3 {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBuffer)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture1D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTexture1D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture1d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTexture2D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateTexture3D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: ::core::option::Option<*mut ::core::option::Option<ID3D11InputLayout>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateInputLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len() as _, ::core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(ppinputlayout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppvertexshader: ::core::option::Option<*mut ::core::option::Option<ID3D11VertexShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppvertexshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGeometryShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P0>(&self, pshaderbytecode: &[u8], psodeclaration: ::core::option::Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: ::core::option::Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGeometryShaderWithStreamOutput)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(pshaderbytecode.as_ptr()),
            pshaderbytecode.len() as _,
            ::core::mem::transmute(psodeclaration.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            psodeclaration.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pbufferstrides.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            pbufferstrides.as_deref().map_or(0, |slice| slice.len() as _),
            rasterizedstream,
            pclasslinkage.into().abi(),
            ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CreatePixelShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pppixelshader: ::core::option::Option<*mut ::core::option::Option<ID3D11PixelShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pppixelshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateHullShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pphullshader: ::core::option::Option<*mut ::core::option::Option<ID3D11HullShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateHullShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pphullshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDomainShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppdomainshader: ::core::option::Option<*mut ::core::option::Option<ID3D11DomainShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDomainShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppdomainshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateComputeShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppcomputeshader: ::core::option::Option<*mut ::core::option::Option<ID3D11ComputeShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateComputeShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppcomputeshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateClassLinkage(&self) -> ::windows::core::Result<ID3D11ClassLinkage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateClassLinkage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBlendState)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencildesc, ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRasterizerState)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11SamplerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSamplerState)(::windows::core::Vtable::as_raw(self), psamplerdesc, ::core::mem::transmute(ppsamplerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: ::core::option::Option<*mut ::core::option::Option<ID3D11Query>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateQuery)(::windows::core::Vtable::as_raw(self), pquerydesc, ::core::mem::transmute(ppquery.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePredicate)(::windows::core::Vtable::as_raw(self), ppredicatedesc, ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: ::core::option::Option<*mut ::core::option::Option<ID3D11Counter>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCounter)(::windows::core::Vtable::as_raw(self), pcounterdesc, ::core::mem::transmute(ppcounter.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeferredContext)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<P0, T>(&self, hresource: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenSharedResource)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckFormatSupport)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckMultisampleQualityLevels)(::windows::core::Vtable::as_raw(self), format, samplecount, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckCounterInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: ::core::option::Option<*mut u32>, szunits: ::windows::core::PSTR, punitslength: ::core::option::Option<*mut u32>, szdescription: ::windows::core::PSTR, pdescriptionlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckCounter)(::windows::core::Vtable::as_raw(self), pdesc, ptype, pactivecounters, ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFeatureLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetImmediateContext(&self) -> ::windows::core::Result<ID3D11DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetImmediateContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetExceptionMode)(::windows::core::Vtable::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetExceptionMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetImmediateContext1(&self) -> ::windows::core::Result<ID3D11DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetImmediateContext1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext1 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeferredContext1)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateBlendState1)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRasterizerState1)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: ::core::option::Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppcontextstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeviceContextState)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute(pfeaturelevels.as_ptr()), pfeaturelevels.len() as _, sdkversion, emulatedinterface, ::core::mem::transmute(pchosenfeaturelevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppcontextstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource1<P0, T>(&self, hresource: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OpenSharedResource1)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenSharedResourceByName<P0, T>(&self, lpname: P0, dwdesiredaccess: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OpenSharedResourceByName)(::windows::core::Vtable::as_raw(self), lpname.into().abi(), dwdesiredaccess, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImmediateContext2(&self) -> ::windows::core::Result<ID3D11DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImmediateContext2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext2 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext2>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDeferredContext2)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D11_PACKED_MIP_DESC>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D11_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels1(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckMultisampleQualityLevels1)(::windows::core::Vtable::as_raw(self), format, samplecount, flags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D11Device4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Device4 {}
impl ::core::fmt::Debug for ID3D11Device4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Device4").field(&self.0).finish()
    }
}
impl ID3D11Device4 {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBuffer)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture1D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTexture1D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture1d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTexture2D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateTexture3D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: ::core::option::Option<*mut ::core::option::Option<ID3D11InputLayout>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateInputLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len() as _, ::core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(ppinputlayout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppvertexshader: ::core::option::Option<*mut ::core::option::Option<ID3D11VertexShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppvertexshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGeometryShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P0>(&self, pshaderbytecode: &[u8], psodeclaration: ::core::option::Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: ::core::option::Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGeometryShaderWithStreamOutput)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(pshaderbytecode.as_ptr()),
            pshaderbytecode.len() as _,
            ::core::mem::transmute(psodeclaration.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            psodeclaration.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pbufferstrides.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            pbufferstrides.as_deref().map_or(0, |slice| slice.len() as _),
            rasterizedstream,
            pclasslinkage.into().abi(),
            ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CreatePixelShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pppixelshader: ::core::option::Option<*mut ::core::option::Option<ID3D11PixelShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pppixelshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateHullShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pphullshader: ::core::option::Option<*mut ::core::option::Option<ID3D11HullShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateHullShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pphullshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDomainShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppdomainshader: ::core::option::Option<*mut ::core::option::Option<ID3D11DomainShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDomainShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppdomainshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateComputeShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppcomputeshader: ::core::option::Option<*mut ::core::option::Option<ID3D11ComputeShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateComputeShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppcomputeshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateClassLinkage(&self) -> ::windows::core::Result<ID3D11ClassLinkage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateClassLinkage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBlendState)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencildesc, ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRasterizerState)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11SamplerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSamplerState)(::windows::core::Vtable::as_raw(self), psamplerdesc, ::core::mem::transmute(ppsamplerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: ::core::option::Option<*mut ::core::option::Option<ID3D11Query>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateQuery)(::windows::core::Vtable::as_raw(self), pquerydesc, ::core::mem::transmute(ppquery.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePredicate)(::windows::core::Vtable::as_raw(self), ppredicatedesc, ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: ::core::option::Option<*mut ::core::option::Option<ID3D11Counter>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCounter)(::windows::core::Vtable::as_raw(self), pcounterdesc, ::core::mem::transmute(ppcounter.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDeferredContext)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<P0, T>(&self, hresource: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenSharedResource)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckFormatSupport)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckMultisampleQualityLevels)(::windows::core::Vtable::as_raw(self), format, samplecount, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckCounterInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: ::core::option::Option<*mut u32>, szunits: ::windows::core::PSTR, punitslength: ::core::option::Option<*mut u32>, szdescription: ::windows::core::PSTR, pdescriptionlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckCounter)(::windows::core::Vtable::as_raw(self), pdesc, ptype, pactivecounters, ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFeatureLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetImmediateContext(&self) -> ::windows::core::Result<ID3D11DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetImmediateContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetExceptionMode)(::windows::core::Vtable::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetExceptionMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetImmediateContext1(&self) -> ::windows::core::Result<ID3D11DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetImmediateContext1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext1 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeferredContext1)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateBlendState1)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRasterizerState1)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: ::core::option::Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppcontextstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeviceContextState)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute(pfeaturelevels.as_ptr()), pfeaturelevels.len() as _, sdkversion, emulatedinterface, ::core::mem::transmute(pchosenfeaturelevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppcontextstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource1<P0, T>(&self, hresource: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenSharedResource1)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenSharedResourceByName<P0, T>(&self, lpname: P0, dwdesiredaccess: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenSharedResourceByName)(::windows::core::Vtable::as_raw(self), lpname.into().abi(), dwdesiredaccess, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImmediateContext2(&self) -> ::windows::core::Result<ID3D11DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetImmediateContext2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext2 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext2>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeferredContext2)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D11_PACKED_MIP_DESC>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D11_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels1(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckMultisampleQualityLevels1)(::windows::core::Vtable::as_raw(self), format, samplecount, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateTexture2D1)(::windows::core::Vtable::as_raw(self), pdesc1, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateTexture3D1)(::windows::core::Vtable::as_raw(self), pdesc1, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState2>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateRasterizerState2)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView1<P0>(&self, presource: P0, pdesc1: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC1>, ppsrview1: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView1>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateShaderResourceView1)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView1<P0>(&self, presource: P0, pdesc1: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC1>, ppuaview1: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView1>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateUnorderedAccessView1)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView1<P0>(&self, presource: P0, pdesc1: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC1>, pprtview1: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView1>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateRenderTargetView1)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: ::core::option::Option<*mut ::core::option::Option<ID3D11Query1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateQuery1)(::windows::core::Vtable::as_raw(self), pquerydesc1, ::core::mem::transmute(ppquery1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetImmediateContext3(&self) -> ::windows::core::Result<ID3D11DeviceContext3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetImmediateContext3)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext3 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext3(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext3>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDeferredContext3)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn WriteToSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WriteToSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ReadFromSubresource<P0>(&self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: P0, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReadFromSubresource)(::windows::core::Vtable::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
}
impl ::core::cmp::PartialEq for ID3D11Device5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Device5 {}
impl ::core::fmt::Debug for ID3D11Device5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Device5").field(&self.0).finish()
    }
}
impl ID3D11Device5 {
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D11_BUFFER_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, ppbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBuffer)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D11_TEXTURE1D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture1d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture1D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTexture1D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture1d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D11_TEXTURE2D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTexture2D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D11_TEXTURE3D_DESC, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateTexture3D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>, ppsrview: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>, ppuaview: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC>, pprtview: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: ::core::option::Option<*mut ::core::option::Option<ID3D11InputLayout>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateInputLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len() as _, ::core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(ppinputlayout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppvertexshader: ::core::option::Option<*mut ::core::option::Option<ID3D11VertexShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppvertexshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGeometryShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P0>(&self, pshaderbytecode: &[u8], psodeclaration: ::core::option::Option<&[D3D11_SO_DECLARATION_ENTRY]>, pbufferstrides: ::core::option::Option<&[u32]>, rasterizedstream: u32, pclasslinkage: P0, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D11GeometryShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGeometryShaderWithStreamOutput)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(pshaderbytecode.as_ptr()),
            pshaderbytecode.len() as _,
            ::core::mem::transmute(psodeclaration.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            psodeclaration.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pbufferstrides.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            pbufferstrides.as_deref().map_or(0, |slice| slice.len() as _),
            rasterizedstream,
            pclasslinkage.into().abi(),
            ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    pub unsafe fn CreatePixelShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pppixelshader: ::core::option::Option<*mut ::core::option::Option<ID3D11PixelShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pppixelshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateHullShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, pphullshader: ::core::option::Option<*mut ::core::option::Option<ID3D11HullShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateHullShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(pphullshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDomainShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppdomainshader: ::core::option::Option<*mut ::core::option::Option<ID3D11DomainShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDomainShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppdomainshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateComputeShader<P0>(&self, pshaderbytecode: &[u8], pclasslinkage: P0, ppcomputeshader: ::core::option::Option<*mut ::core::option::Option<ID3D11ComputeShader>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ClassLinkage>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateComputeShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, pclasslinkage.into().abi(), ::core::mem::transmute(ppcomputeshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateClassLinkage(&self) -> ::windows::core::Result<ID3D11ClassLinkage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateClassLinkage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D11_BLEND_DESC, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateBlendState)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencildesc, ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRasterizerState)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D11_SAMPLER_DESC, ppsamplerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11SamplerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSamplerState)(::windows::core::Vtable::as_raw(self), psamplerdesc, ::core::mem::transmute(ppsamplerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D11_QUERY_DESC, ppquery: ::core::option::Option<*mut ::core::option::Option<ID3D11Query>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateQuery)(::windows::core::Vtable::as_raw(self), pquerydesc, ::core::mem::transmute(ppquery.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D11_QUERY_DESC, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePredicate)(::windows::core::Vtable::as_raw(self), ppredicatedesc, ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D11_COUNTER_DESC, ppcounter: ::core::option::Option<*mut ::core::option::Option<ID3D11Counter>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCounter)(::windows::core::Vtable::as_raw(self), pcounterdesc, ::core::mem::transmute(ppcounter.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDeferredContext)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<P0, T>(&self, hresource: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OpenSharedResource)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckFormatSupport)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckMultisampleQualityLevels)(::windows::core::Vtable::as_raw(self), format, samplecount, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckCounterInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D11_COUNTER_DESC, ptype: *mut D3D11_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: ::core::option::Option<*mut u32>, szunits: ::windows::core::PSTR, punitslength: ::core::option::Option<*mut u32>, szdescription: ::windows::core::PSTR, pdescriptionlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckCounter)(::windows::core::Vtable::as_raw(self), pdesc, ptype, pactivecounters, ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D11_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFeatureLevel)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetImmediateContext(&self) -> ::windows::core::Result<ID3D11DeviceContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetImmediateContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetExceptionMode)(::windows::core::Vtable::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetExceptionMode)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetImmediateContext1(&self) -> ::windows::core::Result<ID3D11DeviceContext1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetImmediateContext1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext1 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext1(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDeferredContext1)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState1(&self, pblendstatedesc: *const D3D11_BLEND_DESC1, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateBlendState1)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState1(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC1, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRasterizerState1)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn CreateDeviceContextState(&self, flags: u32, pfeaturelevels: &[super::Direct3D::D3D_FEATURE_LEVEL], sdkversion: u32, emulatedinterface: *const ::windows::core::GUID, pchosenfeaturelevel: ::core::option::Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppcontextstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDeviceContextState)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute(pfeaturelevels.as_ptr()), pfeaturelevels.len() as _, sdkversion, emulatedinterface, ::core::mem::transmute(pchosenfeaturelevel.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppcontextstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource1<P0, T>(&self, hresource: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenSharedResource1)(::windows::core::Vtable::as_raw(self), hresource.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenSharedResourceByName<P0, T>(&self, lpname: P0, dwdesiredaccess: u32) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenSharedResourceByName)(::windows::core::Vtable::as_raw(self), lpname.into().abi(), dwdesiredaccess, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImmediateContext2(&self) -> ::windows::core::Result<ID3D11DeviceContext2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetImmediateContext2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext2 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext2(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext2>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDeferredContext2)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D11_PACKED_MIP_DESC>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D11_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D11_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels1(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32, flags: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckMultisampleQualityLevels1)(::windows::core::Vtable::as_raw(self), format, samplecount, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D1(&self, pdesc1: *const D3D11_TEXTURE2D_DESC1, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture2d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture2D1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTexture2D1)(::windows::core::Vtable::as_raw(self), pdesc1, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture2d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D1(&self, pdesc1: *const D3D11_TEXTURE3D_DESC1, pinitialdata: ::core::option::Option<*const D3D11_SUBRESOURCE_DATA>, pptexture3d: ::core::option::Option<*mut ::core::option::Option<ID3D11Texture3D1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTexture3D1)(::windows::core::Vtable::as_raw(self), pdesc1, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pptexture3d.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState2(&self, prasterizerdesc: *const D3D11_RASTERIZER_DESC2, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D11RasterizerState2>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRasterizerState2)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView1<P0>(&self, presource: P0, pdesc1: ::core::option::Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC1>, ppsrview1: ::core::option::Option<*mut ::core::option::Option<ID3D11ShaderResourceView1>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateShaderResourceView1)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView1<P0>(&self, presource: P0, pdesc1: ::core::option::Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC1>, ppuaview1: ::core::option::Option<*mut ::core::option::Option<ID3D11UnorderedAccessView1>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateUnorderedAccessView1)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppuaview1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView1<P0>(&self, presource: P0, pdesc1: ::core::option::Option<*const D3D11_RENDER_TARGET_VIEW_DESC1>, pprtview1: ::core::option::Option<*mut ::core::option::Option<ID3D11RenderTargetView1>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRenderTargetView1)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc1.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery1(&self, pquerydesc1: *const D3D11_QUERY_DESC1, ppquery1: ::core::option::Option<*mut ::core::option::Option<ID3D11Query1>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateQuery1)(::windows::core::Vtable::as_raw(self), pquerydesc1, ::core::mem::transmute(ppquery1.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetImmediateContext3(&self) -> ::windows::core::Result<ID3D11DeviceContext3> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetImmediateContext3)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11DeviceContext3 as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn CreateDeferredContext3(&self, contextflags: u32, ppdeferredcontext: ::core::option::Option<*mut ::core::option::Option<ID3D11DeviceContext3>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDeferredContext3)(::windows::core::Vtable::as_raw(self), contextflags, ::core::mem::transmute(ppdeferredcontext.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn WriteToSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteToSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ReadFromSubresource<P0>(&self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, psrcresource: P0, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ReadFromSubresource)(::windows::core::Vtable::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterDeviceRemovedEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterDeviceRemovedEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterDeviceRemoved(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.UnregisterDeviceRemoved)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
}
impl ::core::cmp::PartialEq for ID3D11DeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DeviceChild {}
impl ::core::fmt::Debug for ID3D11DeviceChild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DeviceChild").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11DeviceContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DeviceContext {}
impl ::core::fmt::Debug for ID3D11DeviceContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DeviceContext").field(&self.0).finish()
    }
}
impl ID3D11DeviceContext {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11DeviceContext1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DeviceContext1 {}
impl ::core::fmt::Debug for ID3D11DeviceContext1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DeviceContext1").field(&self.0).finish()
    }
}
impl ID3D11DeviceContext1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11PixelShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PSSetShader)(::windows::core::Vtable::as_raw(self), ppixelshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VertexShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VSSetShader)(::windows::core::Vtable::as_raw(self), pvertexshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexed)(::windows::core::Vtable::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.Draw)(::windows::core::Vtable::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn Map<P0>(&self, presource: P0, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: ::core::option::Option<*mut D3D11_MAPPED_SUBRESOURCE>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Map)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, maptype, mapflags, ::core::mem::transmute(pmappedresource.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Unmap)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11InputLayout>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IASetInputLayout)(::windows::core::Vtable::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*const u32>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11GeometryShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GSSetShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Begin)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn GetData<P0>(&self, pasync: P0, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetData)(::windows::core::Vtable::as_raw(self), pasync.into().abi(), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPredication)(::windows::core::Vtable::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetRenderTargetsAndUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi(), uavstartslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: ::core::option::Option<*const f32>, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11BlendState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetBlendState)(::windows::core::Vtable::as_raw(self), pblendstate.into().abi(), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null())), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), numbuffers, ::core::mem::transmute(ppsotargets.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Vtable::vtable(self).base__.DrawAuto)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexedInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn DrawInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DrawInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DispatchIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RasterizerState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RSSetState)(::windows::core::Vtable::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: ::core::option::Option<&[D3D11_VIEWPORT]>) {
        (::windows::core::Vtable::vtable(self).base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviewports.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: ::core::option::Option<&[super::super::Foundation::RECT]>) {
        (::windows::core::Vtable::vtable(self).base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopySubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn CopyStructureCount<P0, P1>(&self, pdstbuffer: P0, dstalignedbyteoffset: u32, psrcview: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyStructureCount)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstalignedbyteoffset, psrcview.into().abi())
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RenderTargetView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), prendertargetview.into().abi(), colorrgba)
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, punorderedaccessview: P0, values: *const u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, punorderedaccessview: P0, values: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ShaderResourceView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GenerateMips)(::windows::core::Vtable::as_raw(self), pshaderresourceview.into().abi())
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi(), minlod)
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteCommandList<P0, P1>(&self, pcommandlist: P0, restorecontextstate: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CommandList>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecuteCommandList)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi(), restorecontextstate.into())
    }
    pub unsafe fn HSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.HSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetShader<P0>(&self, phullshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11HullShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.HSSetShader)(::windows::core::Vtable::as_raw(self), phullshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn HSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.HSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.HSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.DSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShader<P0>(&self, pdomainshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DomainShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DSSetShader)(::windows::core::Vtable::as_raw(self), pdomainshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.DSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.DSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.CSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.CSSetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetShader<P0>(&self, pcomputeshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ComputeShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CSSetShader)(::windows::core::Vtable::as_raw(self), pcomputeshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn CSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.CSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.CSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppixelshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppvertexshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> ::windows::core::Result<ID3D11InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IAGetInputLayout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11InputLayout as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*mut u32>, poffsets: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.IAGetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, format: ::core::option::Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.IAGetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindexbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(format.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(offset.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppgeometryshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IAGetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>, ppredicatevalue: ::core::option::Option<*mut super::super::Foundation::BOOL>) {
        (::windows::core::Vtable::vtable(self).base__.GetPredication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppredicatevalue.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>, uavstartslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetRenderTargetsAndUnorderedAccessViews)(
            ::windows::core::Vtable::as_raw(self),
            pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())),
            uavstartslot,
            ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>, blendfactor: ::core::option::Option<*mut f32>, psamplemask: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetBlendState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psamplemask.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>, pstencilref: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetDepthStencilState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstencilref.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, ppsotargets: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.SOGetTargets)(::windows::core::Vtable::as_raw(self), ppsotargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsotargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSGetState(&self) -> ::windows::core::Result<ID3D11RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RSGetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11RasterizerState as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: ::core::option::Option<*mut D3D11_VIEWPORT>) {
        (::windows::core::Vtable::vtable(self).base__.RSGetViewports)(::windows::core::Vtable::as_raw(self), pnumviewports, ::core::mem::transmute(pviewports.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: ::core::option::Option<*mut super::super::Foundation::RECT>) {
        (::windows::core::Vtable::vtable(self).base__.RSGetScissorRects)(::windows::core::Vtable::as_raw(self), pnumrects, ::core::mem::transmute(prects.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.HSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetShader(&self, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.HSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pphullshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.HSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.HSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.DSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShader(&self, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.DSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdomainshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.DSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.DSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.CSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetUnorderedAccessViews(&self, startslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.CSGetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShader(&self, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.CSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppcomputeshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.CSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.CSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetContextFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FinishCommandList<P0>(&self, restoredeferredcontextstate: P0, ppcommandlist: ::core::option::Option<*mut ::core::option::Option<ID3D11CommandList>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.FinishCommandList)(::windows::core::Vtable::as_raw(self), restoredeferredcontextstate.into(), ::core::mem::transmute(ppcommandlist.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11DeviceContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DeviceContext2 {}
impl ::core::fmt::Debug for ID3D11DeviceContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DeviceContext2").field(&self.0).finish()
    }
}
impl ID3D11DeviceContext2 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11PixelShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.PSSetShader)(::windows::core::Vtable::as_raw(self), ppixelshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VertexShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VSSetShader)(::windows::core::Vtable::as_raw(self), pvertexshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawIndexed)(::windows::core::Vtable::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Draw)(::windows::core::Vtable::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn Map<P0>(&self, presource: P0, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: ::core::option::Option<*mut D3D11_MAPPED_SUBRESOURCE>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Map)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, maptype, mapflags, ::core::mem::transmute(pmappedresource.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Unmap)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11InputLayout>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetInputLayout)(::windows::core::Vtable::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*const u32>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11GeometryShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GSSetShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Begin)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.End)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn GetData<P0>(&self, pasync: P0, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetData)(::windows::core::Vtable::as_raw(self), pasync.into().abi(), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetRenderTargetsAndUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi(), uavstartslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: ::core::option::Option<*const f32>, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11BlendState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetBlendState)(::windows::core::Vtable::as_raw(self), pblendstate.into().abi(), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null())), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), numbuffers, ::core::mem::transmute(ppsotargets.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawAuto)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawIndexedInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn DrawInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DispatchIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RasterizerState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RSSetState)(::windows::core::Vtable::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: ::core::option::Option<&[D3D11_VIEWPORT]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviewports.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: ::core::option::Option<&[super::super::Foundation::RECT]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopySubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn CopyStructureCount<P0, P1>(&self, pdstbuffer: P0, dstalignedbyteoffset: u32, psrcview: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyStructureCount)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstalignedbyteoffset, psrcview.into().abi())
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RenderTargetView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), prendertargetview.into().abi(), colorrgba)
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, punorderedaccessview: P0, values: *const u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, punorderedaccessview: P0, values: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ShaderResourceView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GenerateMips)(::windows::core::Vtable::as_raw(self), pshaderresourceview.into().abi())
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi(), minlod)
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteCommandList<P0, P1>(&self, pcommandlist: P0, restorecontextstate: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CommandList>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ExecuteCommandList)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi(), restorecontextstate.into())
    }
    pub unsafe fn HSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetShader<P0>(&self, phullshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11HullShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.HSSetShader)(::windows::core::Vtable::as_raw(self), phullshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn HSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShader<P0>(&self, pdomainshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DomainShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DSSetShader)(::windows::core::Vtable::as_raw(self), pdomainshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSSetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetShader<P0>(&self, pcomputeshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ComputeShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CSSetShader)(::windows::core::Vtable::as_raw(self), pcomputeshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn CSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppixelshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppvertexshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> ::windows::core::Result<ID3D11InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IAGetInputLayout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11InputLayout as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*mut u32>, poffsets: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.IAGetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, format: ::core::option::Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.IAGetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindexbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(format.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(offset.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppgeometryshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IAGetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>, ppredicatevalue: ::core::option::Option<*mut super::super::Foundation::BOOL>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPredication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppredicatevalue.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMGetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>, uavstartslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMGetRenderTargetsAndUnorderedAccessViews)(
            ::windows::core::Vtable::as_raw(self),
            pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())),
            uavstartslot,
            ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>, blendfactor: ::core::option::Option<*mut f32>, psamplemask: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMGetBlendState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psamplemask.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>, pstencilref: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMGetDepthStencilState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstencilref.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, ppsotargets: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.SOGetTargets)(::windows::core::Vtable::as_raw(self), ppsotargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsotargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSGetState(&self) -> ::windows::core::Result<ID3D11RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RSGetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11RasterizerState as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: ::core::option::Option<*mut D3D11_VIEWPORT>) {
        (::windows::core::Vtable::vtable(self).base__.base__.RSGetViewports)(::windows::core::Vtable::as_raw(self), pnumviewports, ::core::mem::transmute(pviewports.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: ::core::option::Option<*mut super::super::Foundation::RECT>) {
        (::windows::core::Vtable::vtable(self).base__.base__.RSGetScissorRects)(::windows::core::Vtable::as_raw(self), pnumrects, ::core::mem::transmute(prects.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetShader(&self, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pphullshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShader(&self, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdomainshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetUnorderedAccessViews(&self, startslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSGetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShader(&self, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppcomputeshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.Flush)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetContextFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FinishCommandList<P0>(&self, restoredeferredcontextstate: P0, ppcommandlist: ::core::option::Option<*mut ::core::option::Option<ID3D11CommandList>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FinishCommandList)(::windows::core::Vtable::as_raw(self), restoredeferredcontextstate.into(), ::core::mem::transmute(ppcommandlist.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopySubresourceRegion1<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>, copyflags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopySubresourceRegion1)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())), copyflags)
    }
    pub unsafe fn UpdateSubresource1<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateSubresource1)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch, copyflags)
    }
    pub unsafe fn DiscardResource<P0>(&self, presource: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    pub unsafe fn DiscardView<P0>(&self, presourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardView)(::windows::core::Vtable::as_raw(self), presourceview.into().abi())
    }
    pub unsafe fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.HSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.DSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.CSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.HSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.DSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.CSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SwapDeviceContextState<P0>(&self, pstate: P0, pppreviousstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3DDeviceContextState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SwapDeviceContextState)(::windows::core::Vtable::as_raw(self), pstate.into().abi(), ::core::mem::transmute(pppreviousstate.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearView<P0>(&self, pview: P0, color: *const f32, prect: ::core::option::Option<&[super::super::Foundation::RECT]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearView)(::windows::core::Vtable::as_raw(self), pview.into().abi(), color, ::core::mem::transmute(prect.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prect.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardView1<P0>(&self, presourceview: P0, prects: ::core::option::Option<&[super::super::Foundation::RECT]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardView1)(::windows::core::Vtable::as_raw(self), presourceview.into().abi(), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prects.as_deref().map_or(0, |slice| slice.len() as _))
    }
}
impl ::core::cmp::PartialEq for ID3D11DeviceContext3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DeviceContext3 {}
impl ::core::fmt::Debug for ID3D11DeviceContext3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DeviceContext3").field(&self.0).finish()
    }
}
impl ID3D11DeviceContext3 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11PixelShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSSetShader)(::windows::core::Vtable::as_raw(self), ppixelshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VertexShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSSetShader)(::windows::core::Vtable::as_raw(self), pvertexshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawIndexed)(::windows::core::Vtable::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Draw)(::windows::core::Vtable::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn Map<P0>(&self, presource: P0, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: ::core::option::Option<*mut D3D11_MAPPED_SUBRESOURCE>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Map)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, maptype, mapflags, ::core::mem::transmute(pmappedresource.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Unmap)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11InputLayout>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetInputLayout)(::windows::core::Vtable::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*const u32>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11GeometryShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSSetShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Begin)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.End)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn GetData<P0>(&self, pasync: P0, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetData)(::windows::core::Vtable::as_raw(self), pasync.into().abi(), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetRenderTargetsAndUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi(), uavstartslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: ::core::option::Option<*const f32>, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11BlendState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetBlendState)(::windows::core::Vtable::as_raw(self), pblendstate.into().abi(), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null())), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), numbuffers, ::core::mem::transmute(ppsotargets.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawAuto)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawIndexedInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn DrawInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DispatchIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RasterizerState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSSetState)(::windows::core::Vtable::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: ::core::option::Option<&[D3D11_VIEWPORT]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviewports.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: ::core::option::Option<&[super::super::Foundation::RECT]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopySubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UpdateSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn CopyStructureCount<P0, P1>(&self, pdstbuffer: P0, dstalignedbyteoffset: u32, psrcview: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyStructureCount)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstalignedbyteoffset, psrcview.into().abi())
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RenderTargetView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), prendertargetview.into().abi(), colorrgba)
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, punorderedaccessview: P0, values: *const u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, punorderedaccessview: P0, values: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ShaderResourceView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GenerateMips)(::windows::core::Vtable::as_raw(self), pshaderresourceview.into().abi())
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi(), minlod)
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteCommandList<P0, P1>(&self, pcommandlist: P0, restorecontextstate: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CommandList>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ExecuteCommandList)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi(), restorecontextstate.into())
    }
    pub unsafe fn HSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetShader<P0>(&self, phullshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11HullShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSSetShader)(::windows::core::Vtable::as_raw(self), phullshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn HSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShader<P0>(&self, pdomainshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DomainShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSSetShader)(::windows::core::Vtable::as_raw(self), pdomainshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSSetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetShader<P0>(&self, pcomputeshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ComputeShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSSetShader)(::windows::core::Vtable::as_raw(self), pcomputeshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn CSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppixelshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppvertexshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> ::windows::core::Result<ID3D11InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IAGetInputLayout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11InputLayout as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*mut u32>, poffsets: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IAGetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, format: ::core::option::Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IAGetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindexbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(format.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(offset.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppgeometryshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IAGetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>, ppredicatevalue: ::core::option::Option<*mut super::super::Foundation::BOOL>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPredication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppredicatevalue.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMGetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>, uavstartslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMGetRenderTargetsAndUnorderedAccessViews)(
            ::windows::core::Vtable::as_raw(self),
            pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())),
            uavstartslot,
            ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>, blendfactor: ::core::option::Option<*mut f32>, psamplemask: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMGetBlendState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psamplemask.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>, pstencilref: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMGetDepthStencilState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstencilref.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, ppsotargets: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SOGetTargets)(::windows::core::Vtable::as_raw(self), ppsotargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsotargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSGetState(&self) -> ::windows::core::Result<ID3D11RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSGetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11RasterizerState as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: ::core::option::Option<*mut D3D11_VIEWPORT>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSGetViewports)(::windows::core::Vtable::as_raw(self), pnumviewports, ::core::mem::transmute(pviewports.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: ::core::option::Option<*mut super::super::Foundation::RECT>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSGetScissorRects)(::windows::core::Vtable::as_raw(self), pnumrects, ::core::mem::transmute(prects.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetShader(&self, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pphullshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShader(&self, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdomainshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetUnorderedAccessViews(&self, startslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSGetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShader(&self, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppcomputeshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetContextFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FinishCommandList<P0>(&self, restoredeferredcontextstate: P0, ppcommandlist: ::core::option::Option<*mut ::core::option::Option<ID3D11CommandList>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FinishCommandList)(::windows::core::Vtable::as_raw(self), restoredeferredcontextstate.into(), ::core::mem::transmute(ppcommandlist.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopySubresourceRegion1<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>, copyflags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopySubresourceRegion1)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())), copyflags)
    }
    pub unsafe fn UpdateSubresource1<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateSubresource1)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch, copyflags)
    }
    pub unsafe fn DiscardResource<P0>(&self, presource: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    pub unsafe fn DiscardView<P0>(&self, presourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardView)(::windows::core::Vtable::as_raw(self), presourceview.into().abi())
    }
    pub unsafe fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.VSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.HSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.DSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.PSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SwapDeviceContextState<P0>(&self, pstate: P0, pppreviousstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3DDeviceContextState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SwapDeviceContextState)(::windows::core::Vtable::as_raw(self), pstate.into().abi(), ::core::mem::transmute(pppreviousstate.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearView<P0>(&self, pview: P0, color: *const f32, prect: ::core::option::Option<&[super::super::Foundation::RECT]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearView)(::windows::core::Vtable::as_raw(self), pview.into().abi(), color, ::core::mem::transmute(prect.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prect.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardView1<P0>(&self, presourceview: P0, prects: ::core::option::Option<&[super::super::Foundation::RECT]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardView1)(::windows::core::Vtable::as_raw(self), presourceview.into().abi(), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prects.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTileMappings<P0, P1>(&self, ptiledresource: P0, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: ::core::option::Option<*const D3D11_TILED_RESOURCE_COORDINATE>, ptiledresourceregionsizes: ::core::option::Option<*const D3D11_TILE_REGION_SIZE>, ptilepool: P1, numranges: u32, prangeflags: ::core::option::Option<*const u32>, ptilepoolstartoffsets: ::core::option::Option<*const u32>, prangetilecounts: ::core::option::Option<*const u32>, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateTileMappings)(
            ::windows::core::Vtable::as_raw(self),
            ptiledresource.into().abi(),
            numtiledresourceregions,
            ::core::mem::transmute(ptiledresourceregionstartcoordinates.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ptiledresourceregionsizes.unwrap_or(::std::ptr::null())),
            ptilepool.into().abi(),
            numranges,
            ::core::mem::transmute(prangeflags.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ptilepoolstartoffsets.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(prangetilecounts.unwrap_or(::std::ptr::null())),
            flags,
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTileMappings<P0, P1>(&self, pdesttiledresource: P0, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: P1, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTileMappings)(::windows::core::Vtable::as_raw(self), pdesttiledresource.into().abi(), pdestregionstartcoordinate, psourcetiledresource.into().abi(), psourceregionstartcoordinate, ptileregionsize, flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTiles<P0>(&self, pdesttiledresource: P0, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateTiles)(::windows::core::Vtable::as_raw(self), pdesttiledresource.into().abi(), pdesttileregionstartcoordinate, pdesttileregionsize, psourcetiledata, flags)
    }
    pub unsafe fn ResizeTilePool<P0>(&self, ptilepool: P0, newsizeinbytes: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResizeTilePool)(::windows::core::Vtable::as_raw(self), ptilepool.into().abi(), newsizeinbytes).ok()
    }
    pub unsafe fn TiledResourceBarrier<P0, P1>(&self, ptiledresourceorviewaccessbeforebarrier: P0, ptiledresourceorviewaccessafterbarrier: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11DeviceChild>>,
    {
        (::windows::core::Vtable::vtable(self).base__.TiledResourceBarrier)(::windows::core::Vtable::as_raw(self), ptiledresourceorviewaccessbeforebarrier.into().abi(), ptiledresourceorviewaccessafterbarrier.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAnnotationEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsAnnotationEnabled)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetMarkerInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMarkerInt)(::windows::core::Vtable::as_raw(self), plabel.into().abi(), data)
    }
    pub unsafe fn BeginEventInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginEventInt)(::windows::core::Vtable::as_raw(self), plabel.into().abi(), data)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11DeviceContext4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DeviceContext4 {}
impl ::core::fmt::Debug for ID3D11DeviceContext4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DeviceContext4").field(&self.0).finish()
    }
}
impl ID3D11DeviceContext4 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11PixelShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSSetShader)(::windows::core::Vtable::as_raw(self), ppixelshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VertexShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSSetShader)(::windows::core::Vtable::as_raw(self), pvertexshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawIndexed)(::windows::core::Vtable::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Draw)(::windows::core::Vtable::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn Map<P0>(&self, presource: P0, subresource: u32, maptype: D3D11_MAP, mapflags: u32, pmappedresource: ::core::option::Option<*mut D3D11_MAPPED_SUBRESOURCE>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Map)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, maptype, mapflags, ::core::mem::transmute(pmappedresource.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Unmap)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11InputLayout>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetInputLayout)(::windows::core::Vtable::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*const u32>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11GeometryShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSSetShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Begin)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.End)(::windows::core::Vtable::as_raw(self), pasync.into().abi())
    }
    pub unsafe fn GetData<P0>(&self, pasync: P0, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Asynchronous>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetData)(::windows::core::Vtable::as_raw(self), pasync.into().abi(), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D11RenderTargetView]>, pdepthstencilview: P0, uavstartslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetRenderTargetsAndUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi(), uavstartslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: ::core::option::Option<*const f32>, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11BlendState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetBlendState)(::windows::core::Vtable::as_raw(self), pblendstate.into().abi(), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null())), samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), numbuffers, ::core::mem::transmute(ppsotargets.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawAuto)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawIndexedInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn DrawInstancedIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawInstancedIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DispatchIndirect)(::windows::core::Vtable::as_raw(self), pbufferforargs.into().abi(), alignedbyteoffsetforargs)
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RasterizerState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSSetState)(::windows::core::Vtable::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: ::core::option::Option<&[D3D11_VIEWPORT]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviewports.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: ::core::option::Option<&[super::super::Foundation::RECT]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopySubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UpdateSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn CopyStructureCount<P0, P1>(&self, pdstbuffer: P0, dstalignedbyteoffset: u32, psrcview: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyStructureCount)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstalignedbyteoffset, psrcview.into().abi())
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11RenderTargetView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), prendertargetview.into().abi(), colorrgba)
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, punorderedaccessview: P0, values: *const u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, punorderedaccessview: P0, values: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11UnorderedAccessView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), punorderedaccessview.into().abi(), values)
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ShaderResourceView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GenerateMips)(::windows::core::Vtable::as_raw(self), pshaderresourceview.into().abi())
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi(), minlod)
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetResourceMinLOD)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecuteCommandList<P0, P1>(&self, pcommandlist: P0, restorecontextstate: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CommandList>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ExecuteCommandList)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi(), restorecontextstate.into())
    }
    pub unsafe fn HSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetShader<P0>(&self, phullshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11HullShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSSetShader)(::windows::core::Vtable::as_raw(self), phullshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn HSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetShader<P0>(&self, pdomainshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DomainShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSSetShader)(::windows::core::Vtable::as_raw(self), pdomainshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn DSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D11ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetUnorderedAccessViews(&self, startslot: u32, numuavs: u32, ppunorderedaccessviews: ::core::option::Option<*const ::core::option::Option<ID3D11UnorderedAccessView>>, puavinitialcounts: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSSetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, numuavs, ::core::mem::transmute(ppunorderedaccessviews.unwrap_or(::std::ptr::null())), ::core::mem::transmute(puavinitialcounts.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetShader<P0>(&self, pcomputeshader: P0, ppclassinstances: ::core::option::Option<&[ID3D11ClassInstance]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11ComputeShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSSetShader)(::windows::core::Vtable::as_raw(self), pcomputeshader.into().abi(), ::core::mem::transmute(ppclassinstances.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ppclassinstances.as_deref().map_or(0, |slice| slice.len() as _))
    }
    pub unsafe fn CSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D11SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D11Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self, pppixelshader: *mut ::core::option::Option<ID3D11PixelShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppixelshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self, ppvertexshader: *mut ::core::option::Option<ID3D11VertexShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppvertexshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.PSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> ::windows::core::Result<ID3D11InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IAGetInputLayout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11InputLayout as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pstrides: ::core::option::Option<*mut u32>, poffsets: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IAGetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, format: ::core::option::Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IAGetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindexbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(format.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(offset.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self, ppgeometryshader: *mut ::core::option::Option<ID3D11GeometryShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppgeometryshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IAGetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.VSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D11Predicate>>, ppredicatevalue: ::core::option::Option<*mut super::super::Foundation::BOOL>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPredication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppredicatevalue.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMGetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D11RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilView>>, uavstartslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMGetRenderTargetsAndUnorderedAccessViews)(
            ::windows::core::Vtable::as_raw(self),
            pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())),
            uavstartslot,
            ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        )
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D11BlendState>>, blendfactor: ::core::option::Option<*mut f32>, psamplemask: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMGetBlendState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psamplemask.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D11DepthStencilState>>, pstencilref: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMGetDepthStencilState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstencilref.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, ppsotargets: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SOGetTargets)(::windows::core::Vtable::as_raw(self), ppsotargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsotargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn RSGetState(&self) -> ::windows::core::Result<ID3D11RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSGetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11RasterizerState as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn RSGetViewports(&self, pnumviewports: *mut u32, pviewports: ::core::option::Option<*mut D3D11_VIEWPORT>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSGetViewports)(::windows::core::Vtable::as_raw(self), pnumviewports, ::core::mem::transmute(pviewports.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: ::core::option::Option<*mut super::super::Foundation::RECT>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSGetScissorRects)(::windows::core::Vtable::as_raw(self), pnumrects, ::core::mem::transmute(prects.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetShader(&self, pphullshader: *mut ::core::option::Option<ID3D11HullShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pphullshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn HSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetShader(&self, ppdomainshader: *mut ::core::option::Option<ID3D11DomainShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdomainshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn DSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D11ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetUnorderedAccessViews(&self, startslot: u32, ppunorderedaccessviews: ::core::option::Option<&mut [::core::option::Option<ID3D11UnorderedAccessView>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSGetUnorderedAccessViews)(::windows::core::Vtable::as_raw(self), startslot, ppunorderedaccessviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppunorderedaccessviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetShader(&self, ppcomputeshader: *mut ::core::option::Option<ID3D11ComputeShader>, ppclassinstances: ::core::option::Option<*mut ::core::option::Option<ID3D11ClassInstance>>, pnumclassinstances: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSGetShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppcomputeshader), ::core::mem::transmute(ppclassinstances.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumclassinstances.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D11SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D11Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Flush)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetContextFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FinishCommandList<P0>(&self, restoredeferredcontextstate: P0, ppcommandlist: ::core::option::Option<*mut ::core::option::Option<ID3D11CommandList>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FinishCommandList)(::windows::core::Vtable::as_raw(self), restoredeferredcontextstate.into(), ::core::mem::transmute(ppcommandlist.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CopySubresourceRegion1<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D11_BOX>, copyflags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopySubresourceRegion1)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())), copyflags)
    }
    pub unsafe fn UpdateSubresource1<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D11_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32, copyflags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UpdateSubresource1)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch, copyflags)
    }
    pub unsafe fn DiscardResource<P0>(&self, presource: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi())
    }
    pub unsafe fn DiscardView<P0>(&self, presourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DiscardView)(::windows::core::Vtable::as_raw(self), presourceview.into().abi())
    }
    pub unsafe fn VSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn HSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn GSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn PSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CSSetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*const ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*const u32>, pnumconstants: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSSetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn VSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn HSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn PSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CSGetConstantBuffers1(&self, startslot: u32, numbuffers: u32, ppconstantbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D11Buffer>>, pfirstconstant: ::core::option::Option<*mut u32>, pnumconstants: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CSGetConstantBuffers1)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppconstantbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pfirstconstant.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumconstants.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SwapDeviceContextState<P0>(&self, pstate: P0, pppreviousstate: ::core::option::Option<*mut ::core::option::Option<ID3DDeviceContextState>>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3DDeviceContextState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SwapDeviceContextState)(::windows::core::Vtable::as_raw(self), pstate.into().abi(), ::core::mem::transmute(pppreviousstate.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearView<P0>(&self, pview: P0, color: *const f32, prect: ::core::option::Option<&[super::super::Foundation::RECT]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearView)(::windows::core::Vtable::as_raw(self), pview.into().abi(), color, ::core::mem::transmute(prect.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prect.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardView1<P0>(&self, presourceview: P0, prects: ::core::option::Option<&[super::super::Foundation::RECT]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11View>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DiscardView1)(::windows::core::Vtable::as_raw(self), presourceview.into().abi(), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), prects.as_deref().map_or(0, |slice| slice.len() as _))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTileMappings<P0, P1>(&self, ptiledresource: P0, numtiledresourceregions: u32, ptiledresourceregionstartcoordinates: ::core::option::Option<*const D3D11_TILED_RESOURCE_COORDINATE>, ptiledresourceregionsizes: ::core::option::Option<*const D3D11_TILE_REGION_SIZE>, ptilepool: P1, numranges: u32, prangeflags: ::core::option::Option<*const u32>, ptilepoolstartoffsets: ::core::option::Option<*const u32>, prangetilecounts: ::core::option::Option<*const u32>, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateTileMappings)(
            ::windows::core::Vtable::as_raw(self),
            ptiledresource.into().abi(),
            numtiledresourceregions,
            ::core::mem::transmute(ptiledresourceregionstartcoordinates.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ptiledresourceregionsizes.unwrap_or(::std::ptr::null())),
            ptilepool.into().abi(),
            numranges,
            ::core::mem::transmute(prangeflags.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(ptilepoolstartoffsets.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(prangetilecounts.unwrap_or(::std::ptr::null())),
            flags,
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTileMappings<P0, P1>(&self, pdesttiledresource: P0, pdestregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, psourcetiledresource: P1, psourceregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTileMappings)(::windows::core::Vtable::as_raw(self), pdesttiledresource.into().abi(), pdestregionstartcoordinate, psourcetiledresource.into().abi(), psourceregionstartcoordinate, ptileregionsize, flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D11_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateTiles<P0>(&self, pdesttiledresource: P0, pdesttileregionstartcoordinate: *const D3D11_TILED_RESOURCE_COORDINATE, pdesttileregionsize: *const D3D11_TILE_REGION_SIZE, psourcetiledata: *const ::core::ffi::c_void, flags: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.UpdateTiles)(::windows::core::Vtable::as_raw(self), pdesttiledresource.into().abi(), pdesttileregionstartcoordinate, pdesttileregionsize, psourcetiledata, flags)
    }
    pub unsafe fn ResizeTilePool<P0>(&self, ptilepool: P0, newsizeinbytes: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResizeTilePool)(::windows::core::Vtable::as_raw(self), ptilepool.into().abi(), newsizeinbytes).ok()
    }
    pub unsafe fn TiledResourceBarrier<P0, P1>(&self, ptiledresourceorviewaccessbeforebarrier: P0, ptiledresourceorviewaccessafterbarrier: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11DeviceChild>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.TiledResourceBarrier)(::windows::core::Vtable::as_raw(self), ptiledresourceorviewaccessbeforebarrier.into().abi(), ptiledresourceorviewaccessafterbarrier.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAnnotationEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsAnnotationEnabled)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetMarkerInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMarkerInt)(::windows::core::Vtable::as_raw(self), plabel.into().abi(), data)
    }
    pub unsafe fn BeginEventInt<P0>(&self, plabel: P0, data: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginEventInt)(::windows::core::Vtable::as_raw(self), plabel.into().abi(), data)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Flush1<P0>(&self, contexttype: D3D11_CONTEXT_TYPE, hevent: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.Flush1)(::windows::core::Vtable::as_raw(self), contexttype, hevent.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHardwareProtectionState<P0>(&self, hwprotectionenable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHardwareProtectionState)(::windows::core::Vtable::as_raw(self), hwprotectionenable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHardwareProtectionState(&self) -> super::super::Foundation::BOOL {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHardwareProtectionState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
impl ::core::cmp::PartialEq for ID3D11DomainShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11DomainShader {}
impl ::core::fmt::Debug for ID3D11DomainShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11DomainShader").field(&self.0).finish()
    }
}
impl ID3D11DomainShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11Fence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Fence {}
impl ::core::fmt::Debug for ID3D11Fence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Fence").field(&self.0).finish()
    }
}
impl ID3D11Fence {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11FunctionLinkingGraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11FunctionLinkingGraph {}
impl ::core::fmt::Debug for ID3D11FunctionLinkingGraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11FunctionLinkingGraph").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11FunctionParameterReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11FunctionParameterReflection {}
impl ::core::fmt::Debug for ID3D11FunctionParameterReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11FunctionParameterReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11FunctionReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11FunctionReflection {}
impl ::core::fmt::Debug for ID3D11FunctionReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11FunctionReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11GeometryShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11GeometryShader {}
impl ::core::fmt::Debug for ID3D11GeometryShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11GeometryShader").field(&self.0).finish()
    }
}
impl ID3D11GeometryShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11HullShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11HullShader {}
impl ::core::fmt::Debug for ID3D11HullShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11HullShader").field(&self.0).finish()
    }
}
impl ID3D11HullShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11InfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11InfoQueue {}
impl ::core::fmt::Debug for ID3D11InfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11InfoQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11InputLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11InputLayout {}
impl ::core::fmt::Debug for ID3D11InputLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11InputLayout").field(&self.0).finish()
    }
}
impl ID3D11InputLayout {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11LibraryReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11LibraryReflection {}
impl ::core::fmt::Debug for ID3D11LibraryReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11LibraryReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11Linker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Linker {}
impl ::core::fmt::Debug for ID3D11Linker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Linker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11LinkingNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11LinkingNode {}
impl ::core::fmt::Debug for ID3D11LinkingNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11LinkingNode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11Module {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Module {}
impl ::core::fmt::Debug for ID3D11Module {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Module").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11ModuleInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ModuleInstance {}
impl ::core::fmt::Debug for ID3D11ModuleInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ModuleInstance").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11Multithread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Multithread {}
impl ::core::fmt::Debug for ID3D11Multithread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Multithread").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11PixelShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11PixelShader {}
impl ::core::fmt::Debug for ID3D11PixelShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11PixelShader").field(&self.0).finish()
    }
}
impl ID3D11PixelShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11Predicate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Predicate {}
impl ::core::fmt::Debug for ID3D11Predicate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Predicate").field(&self.0).finish()
    }
}
impl ID3D11Predicate {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> D3D11_QUERY_DESC {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
impl ::core::cmp::PartialEq for ID3D11Query {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Query {}
impl ::core::fmt::Debug for ID3D11Query {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Query").field(&self.0).finish()
    }
}
impl ID3D11Query {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11Query1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Query1 {}
impl ::core::fmt::Debug for ID3D11Query1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Query1").field(&self.0).finish()
    }
}
impl ID3D11Query1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> D3D11_QUERY_DESC {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
impl ::core::cmp::PartialEq for ID3D11RasterizerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RasterizerState {}
impl ::core::fmt::Debug for ID3D11RasterizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RasterizerState").field(&self.0).finish()
    }
}
impl ID3D11RasterizerState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11RasterizerState1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RasterizerState1 {}
impl ::core::fmt::Debug for ID3D11RasterizerState1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RasterizerState1").field(&self.0).finish()
    }
}
impl ID3D11RasterizerState1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11RasterizerState2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RasterizerState2 {}
impl ::core::fmt::Debug for ID3D11RasterizerState2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RasterizerState2").field(&self.0).finish()
    }
}
impl ID3D11RasterizerState2 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RASTERIZER_DESC) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut D3D11_RASTERIZER_DESC1) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11RefDefaultTrackingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RefDefaultTrackingOptions {}
impl ::core::fmt::Debug for ID3D11RefDefaultTrackingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RefDefaultTrackingOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11RefTrackingOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RefTrackingOptions {}
impl ::core::fmt::Debug for ID3D11RefTrackingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RefTrackingOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11RenderTargetView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RenderTargetView {}
impl ::core::fmt::Debug for ID3D11RenderTargetView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RenderTargetView").field(&self.0).finish()
    }
}
impl ID3D11RenderTargetView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11RenderTargetView1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11RenderTargetView1 {}
impl ::core::fmt::Debug for ID3D11RenderTargetView1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11RenderTargetView1").field(&self.0).finish()
    }
}
impl ID3D11RenderTargetView1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_RENDER_TARGET_VIEW_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11Resource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Resource {}
impl ::core::fmt::Debug for ID3D11Resource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Resource").field(&self.0).finish()
    }
}
impl ID3D11Resource {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11SamplerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11SamplerState {}
impl ::core::fmt::Debug for ID3D11SamplerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11SamplerState").field(&self.0).finish()
    }
}
impl ID3D11SamplerState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderReflection {}
impl ::core::fmt::Debug for ID3D11ShaderReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderReflectionConstantBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderReflectionConstantBuffer {}
impl ::core::fmt::Debug for ID3D11ShaderReflectionConstantBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderReflectionConstantBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderReflectionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderReflectionType {}
impl ::core::fmt::Debug for ID3D11ShaderReflectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderReflectionType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderReflectionVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderReflectionVariable {}
impl ::core::fmt::Debug for ID3D11ShaderReflectionVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderReflectionVariable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderResourceView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderResourceView {}
impl ::core::fmt::Debug for ID3D11ShaderResourceView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderResourceView").field(&self.0).finish()
    }
}
impl ID3D11ShaderResourceView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderResourceView1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderResourceView1 {}
impl ::core::fmt::Debug for ID3D11ShaderResourceView1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderResourceView1").field(&self.0).finish()
    }
}
impl ID3D11ShaderResourceView1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderTrace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderTrace {}
impl ::core::fmt::Debug for ID3D11ShaderTrace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderTrace").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11ShaderTraceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11ShaderTraceFactory {}
impl ::core::fmt::Debug for ID3D11ShaderTraceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11ShaderTraceFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11SwitchToRef {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11SwitchToRef {}
impl ::core::fmt::Debug for ID3D11SwitchToRef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11SwitchToRef").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11Texture1D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Texture1D {}
impl ::core::fmt::Debug for ID3D11Texture1D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Texture1D").field(&self.0).finish()
    }
}
impl ID3D11Texture1D {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Texture2D {}
impl ::core::fmt::Debug for ID3D11Texture2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Texture2D").field(&self.0).finish()
    }
}
impl ID3D11Texture2D {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11Texture2D1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Texture2D1 {}
impl ::core::fmt::Debug for ID3D11Texture2D1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Texture2D1").field(&self.0).finish()
    }
}
impl ID3D11Texture2D1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11Texture3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Texture3D {}
impl ::core::fmt::Debug for ID3D11Texture3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Texture3D").field(&self.0).finish()
    }
}
impl ID3D11Texture3D {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D11Texture3D1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11Texture3D1 {}
impl ::core::fmt::Debug for ID3D11Texture3D1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11Texture3D1").field(&self.0).finish()
    }
}
impl ID3D11Texture3D1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> D3D11_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE3D_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11TracingDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11TracingDevice {}
impl ::core::fmt::Debug for ID3D11TracingDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11TracingDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11UnorderedAccessView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11UnorderedAccessView {}
impl ::core::fmt::Debug for ID3D11UnorderedAccessView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11UnorderedAccessView").field(&self.0).finish()
    }
}
impl ID3D11UnorderedAccessView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11UnorderedAccessView1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11UnorderedAccessView1 {}
impl ::core::fmt::Debug for ID3D11UnorderedAccessView1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11UnorderedAccessView1").field(&self.0).finish()
    }
}
impl ID3D11UnorderedAccessView1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_UNORDERED_ACCESS_VIEW_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D11VertexShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VertexShader {}
impl ::core::fmt::Debug for ID3D11VertexShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VertexShader").field(&self.0).finish()
    }
}
impl ID3D11VertexShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoContext {}
impl ::core::fmt::Debug for ID3D11VideoContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoContext").field(&self.0).finish()
    }
}
impl ID3D11VideoContext {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoContext1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoContext1 {}
impl ::core::fmt::Debug for ID3D11VideoContext1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoContext1").field(&self.0).finish()
    }
}
impl ID3D11VideoContext1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDecoderBuffer)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), r#type, pbuffersize, ppbuffer).ok()
    }
    pub unsafe fn ReleaseDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReleaseDecoderBuffer)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), r#type).ok()
    }
    pub unsafe fn DecoderBeginFrame<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoderOutputView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecoderBeginFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pview.into().abi(), contentkeysize, ::core::mem::transmute(pcontentkey.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn DecoderEndFrame<P0>(&self, pdecoder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecoderEndFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmitDecoderBuffers<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubmitDecoderBuffers)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pbufferdesc.len() as _, ::core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn DecoderExtension<P0>(&self, pdecoder: P0, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecoderExtension)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pextensiondata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputTargetRect<P0, P1>(&self, pvideoprocessor: P0, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputTargetRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputBackgroundColor<P0, P1>(&self, pvideoprocessor: P0, ycbcr: P1, pcolor: *const D3D11_VIDEO_COLOR)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputBackgroundColor)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), ycbcr.into(), pcolor)
    }
    pub unsafe fn VideoProcessorSetOutputColorSpace<P0>(&self, pvideoprocessor: P0, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pcolorspace)
    }
    pub unsafe fn VideoProcessorSetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputAlphaFillMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), alphafillmode, streamindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputConstriction<P0, P1>(&self, pvideoprocessor: P0, enable: P1, size: super::super::Foundation::SIZE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputConstriction)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into(), ::core::mem::transmute(size))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputStereoMode<P0, P1>(&self, pvideoprocessor: P0, enable: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputStereoMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into())
    }
    pub unsafe fn VideoProcessorSetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pextensionguid, datasize, pdata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputTargetRect<P0>(&self, pvideoprocessor: P0, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputTargetRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputBackgroundColor<P0>(&self, pvideoprocessor: P0, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputBackgroundColor)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pycbcr, pcolor)
    }
    pub unsafe fn VideoProcessorGetOutputColorSpace<P0>(&self, pvideoprocessor: P0) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputAlphaFillMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), palphafillmode, pstreamindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputConstriction<P0>(&self, pvideoprocessor: P0, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputConstriction)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), penabled, psize)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputStereoMode<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputStereoMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorSetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamFrameFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, frameformat)
    }
    pub unsafe fn VideoProcessorSetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pcolorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorSetStreamOutputRate<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: P1, pcustomrate: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamOutputRate)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, outputrate, repeatframe.into(), ::core::mem::transmute(pcustomrate.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamSourceRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamSourceRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamDestRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamDestRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamAlpha<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, alpha: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamAlpha)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), alpha)
    }
    pub unsafe fn VideoProcessorSetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: ::core::option::Option<&[u32]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamPalette)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorSetStreamPixelAspectRatio<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, psourceaspectratio: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>, pdestinationaspectratio: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamPixelAspectRatio)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(psourceaspectratio.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdestinationaspectratio.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamLumaKey<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, lower: f32, upper: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamLumaKey)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), lower, upper)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamStereoFormat<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: P2, baseviewframe0: P3, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamStereoFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), format, leftviewframe0.into(), baseviewframe0.into(), flipmode, monooffset)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamAutoProcessingMode<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamAutoProcessingMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamFilter<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: P1, level: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamFilter)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, filter, enable.into(), level)
    }
    pub unsafe fn VideoProcessorSetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorGetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_FRAME_FORMAT
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamFrameFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetStreamOutputRate<P0>(&self, pvideoprocessor: P0, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamOutputRate)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, poutputrate, prepeatframe, pcustomrate)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamSourceRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamSourceRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamDestRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamDestRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamAlpha<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamAlpha)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, palpha)
    }
    pub unsafe fn VideoProcessorGetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: &mut [u32])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamPalette)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pentries.len() as _, ::core::mem::transmute(pentries.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetStreamPixelAspectRatio<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamPixelAspectRatio)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, psourceaspectratio, pdestinationaspectratio)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamLumaKey<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamLumaKey)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, plower, pupper)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamStereoFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamStereoFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, pformat, pleftviewframe0, pbaseviewframe0, pflipmode, monooffset)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamAutoProcessingMode<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamAutoProcessingMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamFilter<P0>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamFilter)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, filter, penabled, plevel)
    }
    pub unsafe fn VideoProcessorGetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorBlt<P0, P1>(&self, pvideoprocessor: P0, pview: P1, outputframe: u32, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorOutputView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorBlt)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pview.into().abi(), outputframe, pstreams.len() as _, ::core::mem::transmute(pstreams.as_ptr())).ok()
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchange<P0>(&self, pcryptosession: P0, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.NegotiateCryptoSessionKeyExchange)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), datasize, pdata).ok()
    }
    pub unsafe fn EncryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, ivsize: u32, piv: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EncryptionBlt)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), psrcsurface.into().abi(), pdstsurface.into().abi(), ivsize, ::core::mem::transmute(piv.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DecryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, pencryptedblockinfo: ::core::option::Option<*const D3D11_ENCRYPTED_BLOCK_INFO>, contentkeysize: u32, pcontentkey: ::core::option::Option<*const ::core::ffi::c_void>, ivsize: u32, piv: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecryptionBlt)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), psrcsurface.into().abi(), pdstsurface.into().abi(), ::core::mem::transmute(pencryptedblockinfo.unwrap_or(::std::ptr::null())), contentkeysize, ::core::mem::transmute(pcontentkey.unwrap_or(::std::ptr::null())), ivsize, ::core::mem::transmute(piv.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn StartSessionKeyRefresh<P0>(&self, pcryptosession: P0, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartSessionKeyRefresh)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), randomnumbersize, prandomnumber)
    }
    pub unsafe fn FinishSessionKeyRefresh<P0>(&self, pcryptosession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FinishSessionKeyRefresh)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi())
    }
    pub unsafe fn GetEncryptionBltKey<P0>(&self, pcryptosession: P0, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetEncryptionBltKey)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), keysize, preadbackkey).ok()
    }
    pub unsafe fn NegotiateAuthenticatedChannelKeyExchange<P0>(&self, pchannel: P0, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.NegotiateAuthenticatedChannelKeyExchange)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), datasize, pdata).ok()
    }
    pub unsafe fn QueryAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.QueryAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), inputsize, pinput, outputsize, poutput).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ConfigureAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), inputsize, pinput, poutput).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamRotation<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, rotation: D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamRotation)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), rotation)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamRotation<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamRotation)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, protation)
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoContext2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoContext2 {}
impl ::core::fmt::Debug for ID3D11VideoContext2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoContext2").field(&self.0).finish()
    }
}
impl ID3D11VideoContext2 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDecoderBuffer)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), r#type, pbuffersize, ppbuffer).ok()
    }
    pub unsafe fn ReleaseDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseDecoderBuffer)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), r#type).ok()
    }
    pub unsafe fn DecoderBeginFrame<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoderOutputView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecoderBeginFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pview.into().abi(), contentkeysize, ::core::mem::transmute(pcontentkey.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn DecoderEndFrame<P0>(&self, pdecoder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecoderEndFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmitDecoderBuffers<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SubmitDecoderBuffers)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pbufferdesc.len() as _, ::core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn DecoderExtension<P0>(&self, pdecoder: P0, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecoderExtension)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pextensiondata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputTargetRect<P0, P1>(&self, pvideoprocessor: P0, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputTargetRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputBackgroundColor<P0, P1>(&self, pvideoprocessor: P0, ycbcr: P1, pcolor: *const D3D11_VIDEO_COLOR)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputBackgroundColor)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), ycbcr.into(), pcolor)
    }
    pub unsafe fn VideoProcessorSetOutputColorSpace<P0>(&self, pvideoprocessor: P0, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pcolorspace)
    }
    pub unsafe fn VideoProcessorSetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputAlphaFillMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), alphafillmode, streamindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputConstriction<P0, P1>(&self, pvideoprocessor: P0, enable: P1, size: super::super::Foundation::SIZE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputConstriction)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into(), ::core::mem::transmute(size))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputStereoMode<P0, P1>(&self, pvideoprocessor: P0, enable: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputStereoMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into())
    }
    pub unsafe fn VideoProcessorSetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pextensionguid, datasize, pdata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputTargetRect<P0>(&self, pvideoprocessor: P0, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputTargetRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputBackgroundColor<P0>(&self, pvideoprocessor: P0, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputBackgroundColor)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pycbcr, pcolor)
    }
    pub unsafe fn VideoProcessorGetOutputColorSpace<P0>(&self, pvideoprocessor: P0) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputAlphaFillMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), palphafillmode, pstreamindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputConstriction<P0>(&self, pvideoprocessor: P0, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputConstriction)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), penabled, psize)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputStereoMode<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputStereoMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorSetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamFrameFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, frameformat)
    }
    pub unsafe fn VideoProcessorSetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pcolorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorSetStreamOutputRate<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: P1, pcustomrate: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamOutputRate)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, outputrate, repeatframe.into(), ::core::mem::transmute(pcustomrate.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamSourceRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamSourceRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamDestRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamDestRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamAlpha<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, alpha: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamAlpha)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), alpha)
    }
    pub unsafe fn VideoProcessorSetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: ::core::option::Option<&[u32]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamPalette)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorSetStreamPixelAspectRatio<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, psourceaspectratio: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>, pdestinationaspectratio: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamPixelAspectRatio)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(psourceaspectratio.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdestinationaspectratio.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamLumaKey<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, lower: f32, upper: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamLumaKey)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), lower, upper)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamStereoFormat<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: P2, baseviewframe0: P3, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamStereoFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), format, leftviewframe0.into(), baseviewframe0.into(), flipmode, monooffset)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamAutoProcessingMode<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamAutoProcessingMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamFilter<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: P1, level: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamFilter)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, filter, enable.into(), level)
    }
    pub unsafe fn VideoProcessorSetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorGetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_FRAME_FORMAT
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamFrameFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetStreamOutputRate<P0>(&self, pvideoprocessor: P0, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamOutputRate)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, poutputrate, prepeatframe, pcustomrate)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamSourceRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamSourceRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamDestRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamDestRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamAlpha<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamAlpha)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, palpha)
    }
    pub unsafe fn VideoProcessorGetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: &mut [u32])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamPalette)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pentries.len() as _, ::core::mem::transmute(pentries.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetStreamPixelAspectRatio<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamPixelAspectRatio)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, psourceaspectratio, pdestinationaspectratio)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamLumaKey<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamLumaKey)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, plower, pupper)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamStereoFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamStereoFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, pformat, pleftviewframe0, pbaseviewframe0, pflipmode, monooffset)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamAutoProcessingMode<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamAutoProcessingMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamFilter<P0>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamFilter)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, filter, penabled, plevel)
    }
    pub unsafe fn VideoProcessorGetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorBlt<P0, P1>(&self, pvideoprocessor: P0, pview: P1, outputframe: u32, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorOutputView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorBlt)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pview.into().abi(), outputframe, pstreams.len() as _, ::core::mem::transmute(pstreams.as_ptr())).ok()
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchange<P0>(&self, pcryptosession: P0, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.NegotiateCryptoSessionKeyExchange)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), datasize, pdata).ok()
    }
    pub unsafe fn EncryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, ivsize: u32, piv: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EncryptionBlt)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), psrcsurface.into().abi(), pdstsurface.into().abi(), ivsize, ::core::mem::transmute(piv.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DecryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, pencryptedblockinfo: ::core::option::Option<*const D3D11_ENCRYPTED_BLOCK_INFO>, contentkeysize: u32, pcontentkey: ::core::option::Option<*const ::core::ffi::c_void>, ivsize: u32, piv: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecryptionBlt)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), psrcsurface.into().abi(), pdstsurface.into().abi(), ::core::mem::transmute(pencryptedblockinfo.unwrap_or(::std::ptr::null())), contentkeysize, ::core::mem::transmute(pcontentkey.unwrap_or(::std::ptr::null())), ivsize, ::core::mem::transmute(piv.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn StartSessionKeyRefresh<P0>(&self, pcryptosession: P0, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.StartSessionKeyRefresh)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), randomnumbersize, prandomnumber)
    }
    pub unsafe fn FinishSessionKeyRefresh<P0>(&self, pcryptosession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FinishSessionKeyRefresh)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi())
    }
    pub unsafe fn GetEncryptionBltKey<P0>(&self, pcryptosession: P0, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetEncryptionBltKey)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), keysize, preadbackkey).ok()
    }
    pub unsafe fn NegotiateAuthenticatedChannelKeyExchange<P0>(&self, pchannel: P0, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.NegotiateAuthenticatedChannelKeyExchange)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), datasize, pdata).ok()
    }
    pub unsafe fn QueryAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.QueryAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), inputsize, pinput, outputsize, poutput).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ConfigureAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), inputsize, pinput, poutput).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamRotation<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, rotation: D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamRotation)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), rotation)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamRotation<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamRotation)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, protation)
    }
    pub unsafe fn SubmitDecoderBuffers1<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC1]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubmitDecoderBuffers1)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pbufferdesc.len() as _, ::core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn GetDataForNewHardwareKey<P0>(&self, pcryptosession: P0, pprivatinputdata: &[u8]) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDataForNewHardwareKey)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), pprivatinputdata.len() as _, ::core::mem::transmute(pprivatinputdata.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCryptoSessionStatus<P0>(&self, pcryptosession: P0) -> ::windows::core::Result<D3D11_CRYPTO_SESSION_STATUS>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckCryptoSessionStatus)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DecoderEnableDownsampling<P0>(&self, pdecoder: P0, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecoderEnableDownsampling)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), inputcolorspace, poutputdesc, referenceframecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DecoderUpdateDownsampling<P0>(&self, pdecoder: P0, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DecoderUpdateDownsampling)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), poutputdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetOutputColorSpace1<P0>(&self, pvideoprocessor: P0, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), colorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputShaderUsage<P0, P1>(&self, pvideoprocessor: P0, shaderusage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputShaderUsage)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), shaderusage.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetOutputColorSpace1<P0>(&self, pvideoprocessor: P0) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputShaderUsage<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputShaderUsage)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, colorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamMirror<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, fliphorizontal: P2, flipvertical: P3)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamMirror)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), fliphorizontal.into(), flipvertical.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamMirror<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamMirror)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, pfliphorizontal, pflipvertical)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetBehaviorHints<P0>(&self, pvideoprocessor: P0, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT]) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetBehaviorHints)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), outputwidth, outputheight, outputformat, pstreams.len() as _, ::core::mem::transmute(pstreams.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoContext3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoContext3 {}
impl ::core::fmt::Debug for ID3D11VideoContext3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoContext3").field(&self.0).finish()
    }
}
impl ID3D11VideoContext3 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE, pbuffersize: *mut u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDecoderBuffer)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), r#type, pbuffersize, ppbuffer).ok()
    }
    pub unsafe fn ReleaseDecoderBuffer<P0>(&self, pdecoder: P0, r#type: D3D11_VIDEO_DECODER_BUFFER_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReleaseDecoderBuffer)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), r#type).ok()
    }
    pub unsafe fn DecoderBeginFrame<P0, P1>(&self, pdecoder: P0, pview: P1, contentkeysize: u32, pcontentkey: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoderOutputView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DecoderBeginFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pview.into().abi(), contentkeysize, ::core::mem::transmute(pcontentkey.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn DecoderEndFrame<P0>(&self, pdecoder: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DecoderEndFrame)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubmitDecoderBuffers<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SubmitDecoderBuffers)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pbufferdesc.len() as _, ::core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn DecoderExtension<P0>(&self, pdecoder: P0, pextensiondata: *const D3D11_VIDEO_DECODER_EXTENSION) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DecoderExtension)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pextensiondata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputTargetRect<P0, P1>(&self, pvideoprocessor: P0, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputTargetRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputBackgroundColor<P0, P1>(&self, pvideoprocessor: P0, ycbcr: P1, pcolor: *const D3D11_VIDEO_COLOR)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputBackgroundColor)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), ycbcr.into(), pcolor)
    }
    pub unsafe fn VideoProcessorSetOutputColorSpace<P0>(&self, pvideoprocessor: P0, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pcolorspace)
    }
    pub unsafe fn VideoProcessorSetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, alphafillmode: D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, streamindex: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputAlphaFillMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), alphafillmode, streamindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputConstriction<P0, P1>(&self, pvideoprocessor: P0, enable: P1, size: super::super::Foundation::SIZE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputConstriction)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into(), ::core::mem::transmute(size))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputStereoMode<P0, P1>(&self, pvideoprocessor: P0, enable: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputStereoMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enable.into())
    }
    pub unsafe fn VideoProcessorSetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetOutputExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pextensionguid, datasize, pdata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputTargetRect<P0>(&self, pvideoprocessor: P0, enabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputTargetRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), enabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputBackgroundColor<P0>(&self, pvideoprocessor: P0, pycbcr: *mut super::super::Foundation::BOOL, pcolor: *mut D3D11_VIDEO_COLOR)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputBackgroundColor)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pycbcr, pcolor)
    }
    pub unsafe fn VideoProcessorGetOutputColorSpace<P0>(&self, pvideoprocessor: P0) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetOutputAlphaFillMode<P0>(&self, pvideoprocessor: P0, palphafillmode: *mut D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE, pstreamindex: *mut u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputAlphaFillMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), palphafillmode, pstreamindex)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputConstriction<P0>(&self, pvideoprocessor: P0, penabled: *mut super::super::Foundation::BOOL, psize: *mut super::super::Foundation::SIZE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputConstriction)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), penabled, psize)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputStereoMode<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputStereoMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetOutputExtension<P0>(&self, pvideoprocessor: P0, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetOutputExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorSetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, frameformat: D3D11_VIDEO_FRAME_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamFrameFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, frameformat)
    }
    pub unsafe fn VideoProcessorSetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32, pcolorspace: *const D3D11_VIDEO_PROCESSOR_COLOR_SPACE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pcolorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorSetStreamOutputRate<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, outputrate: D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, repeatframe: P1, pcustomrate: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamOutputRate)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, outputrate, repeatframe.into(), ::core::mem::transmute(pcustomrate.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamSourceRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamSourceRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamDestRect<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, prect: ::core::option::Option<*const super::super::Foundation::RECT>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamDestRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(prect.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamAlpha<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, alpha: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamAlpha)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), alpha)
    }
    pub unsafe fn VideoProcessorSetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: ::core::option::Option<&[u32]>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamPalette)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pentries.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pentries.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorSetStreamPixelAspectRatio<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, psourceaspectratio: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>, pdestinationaspectratio: ::core::option::Option<*const super::Dxgi::Common::DXGI_RATIONAL>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamPixelAspectRatio)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), ::core::mem::transmute(psourceaspectratio.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdestinationaspectratio.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamLumaKey<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, lower: f32, upper: f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamLumaKey)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), lower, upper)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamStereoFormat<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, format: D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, leftviewframe0: P2, baseviewframe0: P3, flipmode: D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamStereoFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), format, leftviewframe0.into(), baseviewframe0.into(), flipmode, monooffset)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamAutoProcessingMode<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamAutoProcessingMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamFilter<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, enable: P1, level: i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamFilter)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, filter, enable.into(), level)
    }
    pub unsafe fn VideoProcessorSetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    pub unsafe fn VideoProcessorGetStreamFrameFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_FRAME_FORMAT
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamFrameFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VideoProcessorGetStreamColorSpace<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> D3D11_VIDEO_PROCESSOR_COLOR_SPACE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamColorSpace)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetStreamOutputRate<P0>(&self, pvideoprocessor: P0, streamindex: u32, poutputrate: *mut D3D11_VIDEO_PROCESSOR_OUTPUT_RATE, prepeatframe: *mut super::super::Foundation::BOOL, pcustomrate: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamOutputRate)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, poutputrate, prepeatframe, pcustomrate)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamSourceRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamSourceRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamDestRect<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, prect: *mut super::super::Foundation::RECT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamDestRect)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, prect)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamAlpha<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, palpha: *mut f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamAlpha)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, palpha)
    }
    pub unsafe fn VideoProcessorGetStreamPalette<P0>(&self, pvideoprocessor: P0, streamindex: u32, pentries: &mut [u32])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamPalette)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pentries.len() as _, ::core::mem::transmute(pentries.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetStreamPixelAspectRatio<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, psourceaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL, pdestinationaspectratio: *mut super::Dxgi::Common::DXGI_RATIONAL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamPixelAspectRatio)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, psourceaspectratio, pdestinationaspectratio)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamLumaKey<P0>(&self, pvideoprocessor: P0, streamindex: u32, penabled: *mut super::super::Foundation::BOOL, plower: *mut f32, pupper: *mut f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamLumaKey)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penabled, plower, pupper)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamStereoFormat<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pformat: *mut D3D11_VIDEO_PROCESSOR_STEREO_FORMAT, pleftviewframe0: *mut super::super::Foundation::BOOL, pbaseviewframe0: *mut super::super::Foundation::BOOL, pflipmode: *mut D3D11_VIDEO_PROCESSOR_STEREO_FLIP_MODE, monooffset: *mut i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamStereoFormat)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, pformat, pleftviewframe0, pbaseviewframe0, pflipmode, monooffset)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamAutoProcessingMode<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamAutoProcessingMode)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamFilter<P0>(&self, pvideoprocessor: P0, streamindex: u32, filter: D3D11_VIDEO_PROCESSOR_FILTER, penabled: *mut super::super::Foundation::BOOL, plevel: *mut i32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamFilter)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, filter, penabled, plevel)
    }
    pub unsafe fn VideoProcessorGetStreamExtension<P0>(&self, pvideoprocessor: P0, streamindex: u32, pextensionguid: *const ::windows::core::GUID, datasize: u32, pdata: *mut ::core::ffi::c_void) -> i32
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamExtension)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, pextensionguid, datasize, pdata)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorBlt<P0, P1>(&self, pvideoprocessor: P0, pview: P1, outputframe: u32, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorOutputView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorBlt)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), pview.into().abi(), outputframe, pstreams.len() as _, ::core::mem::transmute(pstreams.as_ptr())).ok()
    }
    pub unsafe fn NegotiateCryptoSessionKeyExchange<P0>(&self, pcryptosession: P0, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NegotiateCryptoSessionKeyExchange)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), datasize, pdata).ok()
    }
    pub unsafe fn EncryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, ivsize: u32, piv: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EncryptionBlt)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), psrcsurface.into().abi(), pdstsurface.into().abi(), ivsize, ::core::mem::transmute(piv.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn DecryptionBlt<P0, P1, P2>(&self, pcryptosession: P0, psrcsurface: P1, pdstsurface: P2, pencryptedblockinfo: ::core::option::Option<*const D3D11_ENCRYPTED_BLOCK_INFO>, contentkeysize: u32, pcontentkey: ::core::option::Option<*const ::core::ffi::c_void>, ivsize: u32, piv: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D11Texture2D>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DecryptionBlt)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), psrcsurface.into().abi(), pdstsurface.into().abi(), ::core::mem::transmute(pencryptedblockinfo.unwrap_or(::std::ptr::null())), contentkeysize, ::core::mem::transmute(pcontentkey.unwrap_or(::std::ptr::null())), ivsize, ::core::mem::transmute(piv.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn StartSessionKeyRefresh<P0>(&self, pcryptosession: P0, randomnumbersize: u32, prandomnumber: *mut ::core::ffi::c_void)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StartSessionKeyRefresh)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), randomnumbersize, prandomnumber)
    }
    pub unsafe fn FinishSessionKeyRefresh<P0>(&self, pcryptosession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FinishSessionKeyRefresh)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi())
    }
    pub unsafe fn GetEncryptionBltKey<P0>(&self, pcryptosession: P0, keysize: u32, preadbackkey: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetEncryptionBltKey)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), keysize, preadbackkey).ok()
    }
    pub unsafe fn NegotiateAuthenticatedChannelKeyExchange<P0>(&self, pchannel: P0, datasize: u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NegotiateAuthenticatedChannelKeyExchange)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), datasize, pdata).ok()
    }
    pub unsafe fn QueryAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const ::core::ffi::c_void, outputsize: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.QueryAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), inputsize, pinput, outputsize, poutput).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureAuthenticatedChannel<P0>(&self, pchannel: P0, inputsize: u32, pinput: *const ::core::ffi::c_void, poutput: *mut D3D11_AUTHENTICATED_CONFIGURE_OUTPUT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11AuthenticatedChannel>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ConfigureAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), pchannel.into().abi(), inputsize, pinput, poutput).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamRotation<P0, P1>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, rotation: D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorSetStreamRotation)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), rotation)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamRotation<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, protation: *mut D3D11_VIDEO_PROCESSOR_ROTATION)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.VideoProcessorGetStreamRotation)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, protation)
    }
    pub unsafe fn SubmitDecoderBuffers1<P0>(&self, pdecoder: P0, pbufferdesc: &[D3D11_VIDEO_DECODER_BUFFER_DESC1]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SubmitDecoderBuffers1)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), pbufferdesc.len() as _, ::core::mem::transmute(pbufferdesc.as_ptr())).ok()
    }
    pub unsafe fn GetDataForNewHardwareKey<P0>(&self, pcryptosession: P0, pprivatinputdata: &[u8]) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataForNewHardwareKey)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), pprivatinputdata.len() as _, ::core::mem::transmute(pprivatinputdata.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCryptoSessionStatus<P0>(&self, pcryptosession: P0) -> ::windows::core::Result<D3D11_CRYPTO_SESSION_STATUS>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11CryptoSession>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckCryptoSessionStatus)(::windows::core::Vtable::as_raw(self), pcryptosession.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DecoderEnableDownsampling<P0>(&self, pdecoder: P0, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, referenceframecount: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecoderEnableDownsampling)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), inputcolorspace, poutputdesc, referenceframecount).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DecoderUpdateDownsampling<P0>(&self, pdecoder: P0, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoDecoder>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DecoderUpdateDownsampling)(::windows::core::Vtable::as_raw(self), pdecoder.into().abi(), poutputdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetOutputColorSpace1<P0>(&self, pvideoprocessor: P0, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), colorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetOutputShaderUsage<P0, P1>(&self, pvideoprocessor: P0, shaderusage: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetOutputShaderUsage)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), shaderusage.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetOutputColorSpace1<P0>(&self, pvideoprocessor: P0) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetOutputShaderUsage<P0>(&self, pvideoprocessor: P0) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetOutputShaderUsage)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorSetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, colorspace)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorSetStreamMirror<P0, P1, P2, P3>(&self, pvideoprocessor: P0, streamindex: u32, enable: P1, fliphorizontal: P2, flipvertical: P3)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorSetStreamMirror)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, enable.into(), fliphorizontal.into(), flipvertical.into())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn VideoProcessorGetStreamColorSpace1<P0>(&self, pvideoprocessor: P0, streamindex: u32) -> super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamColorSpace1)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, result__.as_mut_ptr());
        result__.assume_init()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VideoProcessorGetStreamMirror<P0>(&self, pvideoprocessor: P0, streamindex: u32, penable: *mut super::super::Foundation::BOOL, pfliphorizontal: *mut super::super::Foundation::BOOL, pflipvertical: *mut super::super::Foundation::BOOL)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetStreamMirror)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, penable, pfliphorizontal, pflipvertical)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn VideoProcessorGetBehaviorHints<P0>(&self, pvideoprocessor: P0, outputwidth: u32, outputheight: u32, outputformat: super::Dxgi::Common::DXGI_FORMAT, pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT]) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.VideoProcessorGetBehaviorHints)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), outputwidth, outputheight, outputformat, pstreams.len() as _, ::core::mem::transmute(pstreams.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorSetOutputHDRMetaData<P0>(&self, pvideoprocessor: P0, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: ::core::option::Option<*const ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetOutputHDRMetaData)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), r#type, size, ::core::mem::transmute(phdrmetadata.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorGetOutputHDRMetaData<P0>(&self, pvideoprocessor: P0, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetOutputHDRMetaData)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), ptype, size, ::core::mem::transmute(pmetadata.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorSetStreamHDRMetaData<P0>(&self, pvideoprocessor: P0, streamindex: u32, r#type: super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, phdrmetadata: ::core::option::Option<*const ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorSetStreamHDRMetaData)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, r#type, size, ::core::mem::transmute(phdrmetadata.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn VideoProcessorGetStreamHDRMetaData<P0>(&self, pvideoprocessor: P0, streamindex: u32, ptype: *mut super::Dxgi::DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: ::core::option::Option<*mut ::core::ffi::c_void>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VideoProcessorGetStreamHDRMetaData)(::windows::core::Vtable::as_raw(self), pvideoprocessor.into().abi(), streamindex, ptype, size, ::core::mem::transmute(pmetadata.unwrap_or(::std::ptr::null_mut())))
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoDecoder {}
impl ::core::fmt::Debug for ID3D11VideoDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoDecoder").field(&self.0).finish()
    }
}
impl ID3D11VideoDecoder {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoDecoderOutputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoDecoderOutputView {}
impl ::core::fmt::Debug for ID3D11VideoDecoderOutputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoDecoderOutputView").field(&self.0).finish()
    }
}
impl ID3D11VideoDecoderOutputView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoDevice {}
impl ::core::fmt::Debug for ID3D11VideoDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoDevice1 {}
impl ::core::fmt::Debug for ID3D11VideoDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoDevice1").field(&self.0).finish()
    }
}
impl ID3D11VideoDevice1 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<ID3D11VideoDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoDecoder)(::windows::core::Vtable::as_raw(self), pvideodesc, pconfig, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateVideoProcessor<P0>(&self, penum: P0, rateconversionindex: u32) -> ::windows::core::Result<ID3D11VideoProcessor>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorEnumerator>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoProcessor)(::windows::core::Vtable::as_raw(self), penum.into().abi(), rateconversionindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> ::windows::core::Result<ID3D11AuthenticatedChannel> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), channeltype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCryptoSession(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, pkeyexchangetype: *const ::windows::core::GUID) -> ::windows::core::Result<ID3D11CryptoSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCryptoSession)(::windows::core::Vtable::as_raw(self), pcryptotype, ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), pkeyexchangetype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateVideoDecoderOutputView<P0>(&self, presource: P0, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: ::core::option::Option<*mut ::core::option::Option<ID3D11VideoDecoderOutputView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateVideoDecoderOutputView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pdesc, ::core::mem::transmute(ppvdovview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVideoProcessorInputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: ::core::option::Option<*mut ::core::option::Option<ID3D11VideoProcessorInputView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorEnumerator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateVideoProcessorInputView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), penum.into().abi(), pdesc, ::core::mem::transmute(ppvpiview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVideoProcessorOutputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: ::core::option::Option<*mut ::core::option::Option<ID3D11VideoProcessorOutputView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorEnumerator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateVideoProcessorOutputView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), penum.into().abi(), pdesc, ::core::mem::transmute(ppvpoview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::Result<ID3D11VideoProcessorEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVideoProcessorEnumerator)(::windows::core::Vtable::as_raw(self), pdesc, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVideoDecoderProfileCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetVideoDecoderProfileCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetVideoDecoderProfile(&self, index: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVideoDecoderProfile)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckVideoDecoderFormat)(::windows::core::Vtable::as_raw(self), pdecoderprofile, format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVideoDecoderConfigCount)(::windows::core::Vtable::as_raw(self), pdesc, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoDecoderConfig)(::windows::core::Vtable::as_raw(self), pdesc, index, pconfig).ok()
    }
    pub unsafe fn GetContentProtectionCaps(&self, pcryptotype: ::core::option::Option<*const ::windows::core::GUID>, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetContentProtectionCaps)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pcryptotype.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), pcaps).ok()
    }
    pub unsafe fn CheckCryptoKeyExchange(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, index: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckCryptoKeyExchange)(::windows::core::Vtable::as_raw(self), pcryptotype, ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoDevice2 {}
impl ::core::fmt::Debug for ID3D11VideoDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoDevice2").field(&self.0).finish()
    }
}
impl ID3D11VideoDevice2 {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoDecoder(&self, pvideodesc: *const D3D11_VIDEO_DECODER_DESC, pconfig: *const D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<ID3D11VideoDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoDecoder)(::windows::core::Vtable::as_raw(self), pvideodesc, pconfig, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateVideoProcessor<P0>(&self, penum: P0, rateconversionindex: u32) -> ::windows::core::Result<ID3D11VideoProcessor>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorEnumerator>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoProcessor)(::windows::core::Vtable::as_raw(self), penum.into().abi(), rateconversionindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateAuthenticatedChannel(&self, channeltype: D3D11_AUTHENTICATED_CHANNEL_TYPE) -> ::windows::core::Result<ID3D11AuthenticatedChannel> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateAuthenticatedChannel)(::windows::core::Vtable::as_raw(self), channeltype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCryptoSession(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, pkeyexchangetype: *const ::windows::core::GUID) -> ::windows::core::Result<ID3D11CryptoSession> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCryptoSession)(::windows::core::Vtable::as_raw(self), pcryptotype, ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), pkeyexchangetype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateVideoDecoderOutputView<P0>(&self, presource: P0, pdesc: *const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppvdovview: ::core::option::Option<*mut ::core::option::Option<ID3D11VideoDecoderOutputView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoDecoderOutputView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pdesc, ::core::mem::transmute(ppvdovview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVideoProcessorInputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppvpiview: ::core::option::Option<*mut ::core::option::Option<ID3D11VideoProcessorInputView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorEnumerator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoProcessorInputView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), penum.into().abi(), pdesc, ::core::mem::transmute(ppvpiview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVideoProcessorOutputView<P0, P1>(&self, presource: P0, penum: P1, pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppvpoview: ::core::option::Option<*mut ::core::option::Option<ID3D11VideoProcessorOutputView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D11Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D11VideoProcessorEnumerator>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoProcessorOutputView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), penum.into().abi(), pdesc, ::core::mem::transmute(ppvpoview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVideoProcessorEnumerator(&self, pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::Result<ID3D11VideoProcessorEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateVideoProcessorEnumerator)(::windows::core::Vtable::as_raw(self), pdesc, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVideoDecoderProfileCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVideoDecoderProfileCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetVideoDecoderProfile(&self, index: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVideoDecoderProfile)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CheckVideoDecoderFormat(&self, pdecoderprofile: *const ::windows::core::GUID, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckVideoDecoderFormat)(::windows::core::Vtable::as_raw(self), pdecoderprofile, format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderConfigCount(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVideoDecoderConfigCount)(::windows::core::Vtable::as_raw(self), pdesc, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderConfig(&self, pdesc: *const D3D11_VIDEO_DECODER_DESC, index: u32, pconfig: *mut D3D11_VIDEO_DECODER_CONFIG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVideoDecoderConfig)(::windows::core::Vtable::as_raw(self), pdesc, index, pconfig).ok()
    }
    pub unsafe fn GetContentProtectionCaps(&self, pcryptotype: ::core::option::Option<*const ::windows::core::GUID>, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, pcaps: *mut D3D11_VIDEO_CONTENT_PROTECTION_CAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetContentProtectionCaps)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pcryptotype.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), pcaps).ok()
    }
    pub unsafe fn CheckCryptoKeyExchange(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, index: u32) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckCryptoKeyExchange)(::windows::core::Vtable::as_raw(self), pcryptotype, ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetCryptoSessionPrivateDataSize(&self, pcryptotype: *const ::windows::core::GUID, pdecoderprofile: ::core::option::Option<*const ::windows::core::GUID>, pkeyexchangetype: *const ::windows::core::GUID, pprivateinputsize: *mut u32, pprivateoutputsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCryptoSessionPrivateDataSize)(::windows::core::Vtable::as_raw(self), pcryptotype, ::core::mem::transmute(pdecoderprofile.unwrap_or(::std::ptr::null())), pkeyexchangetype, pprivateinputsize, pprivateoutputsize).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoDecoderCaps(&self, pdecoderprofile: *const ::windows::core::GUID, samplewidth: u32, sampleheight: u32, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, bitrate: u32, pcryptotype: ::core::option::Option<*const ::windows::core::GUID>) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVideoDecoderCaps)(::windows::core::Vtable::as_raw(self), pdecoderprofile, samplewidth, sampleheight, pframerate, bitrate, ::core::mem::transmute(pcryptotype.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CheckVideoDecoderDownsampling(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL, poutputdesc: *const D3D11_VIDEO_SAMPLE_DESC, psupported: *mut super::super::Foundation::BOOL, prealtimehint: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckVideoDecoderDownsampling)(::windows::core::Vtable::as_raw(self), pinputdesc, inputcolorspace, pinputconfig, pframerate, poutputdesc, psupported, prealtimehint).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn RecommendVideoDecoderDownsampleParameters(&self, pinputdesc: *const D3D11_VIDEO_DECODER_DESC, inputcolorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE, pinputconfig: *const D3D11_VIDEO_DECODER_CONFIG, pframerate: *const super::Dxgi::Common::DXGI_RATIONAL) -> ::windows::core::Result<D3D11_VIDEO_SAMPLE_DESC> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RecommendVideoDecoderDownsampleParameters)(::windows::core::Vtable::as_raw(self), pinputdesc, inputcolorspace, pinputconfig, pframerate, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoProcessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoProcessor {}
impl ::core::fmt::Debug for ID3D11VideoProcessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoProcessor").field(&self.0).finish()
    }
}
impl ID3D11VideoProcessor {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoProcessorEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoProcessorEnumerator {}
impl ::core::fmt::Debug for ID3D11VideoProcessorEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoProcessorEnumerator").field(&self.0).finish()
    }
}
impl ID3D11VideoProcessorEnumerator {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoProcessorEnumerator1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoProcessorEnumerator1 {}
impl ::core::fmt::Debug for ID3D11VideoProcessorEnumerator1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoProcessorEnumerator1").field(&self.0).finish()
    }
}
impl ID3D11VideoProcessorEnumerator1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetVideoProcessorContentDesc(&self, pcontentdesc: *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoProcessorContentDesc)(::windows::core::Vtable::as_raw(self), pcontentdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckVideoProcessorFormat(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckVideoProcessorFormat)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVideoProcessorCaps(&self, pcaps: *mut D3D11_VIDEO_PROCESSOR_CAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoProcessorCaps)(::windows::core::Vtable::as_raw(self), pcaps).ok()
    }
    pub unsafe fn GetVideoProcessorRateConversionCaps(&self, typeindex: u32, pcaps: *mut D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoProcessorRateConversionCaps)(::windows::core::Vtable::as_raw(self), typeindex, pcaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetVideoProcessorCustomRate(&self, typeindex: u32, customrateindex: u32, prate: *mut D3D11_VIDEO_PROCESSOR_CUSTOM_RATE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVideoProcessorCustomRate)(::windows::core::Vtable::as_raw(self), typeindex, customrateindex, prate).ok()
    }
    pub unsafe fn GetVideoProcessorFilterRange(&self, filter: D3D11_VIDEO_PROCESSOR_FILTER) -> ::windows::core::Result<D3D11_VIDEO_PROCESSOR_FILTER_RANGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVideoProcessorFilterRange)(::windows::core::Vtable::as_raw(self), filter, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoProcessorInputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoProcessorInputView {}
impl ::core::fmt::Debug for ID3D11VideoProcessorInputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoProcessorInputView").field(&self.0).finish()
    }
}
impl ID3D11VideoProcessorInputView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11VideoProcessorOutputView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11VideoProcessorOutputView {}
impl ::core::fmt::Debug for ID3D11VideoProcessorOutputView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11VideoProcessorOutputView").field(&self.0).finish()
    }
}
impl ID3D11VideoProcessorOutputView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D11Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D11View {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D11View {}
impl ::core::fmt::Debug for ID3D11View {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D11View").field(&self.0).finish()
    }
}
impl ID3D11View {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3DDeviceContextState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DDeviceContextState {}
impl ::core::fmt::Debug for ID3DDeviceContextState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DDeviceContextState").field(&self.0).finish()
    }
}
impl ID3DDeviceContextState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D11Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D11Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3DUserDefinedAnnotation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DUserDefinedAnnotation {}
impl ::core::fmt::Debug for ID3DUserDefinedAnnotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DUserDefinedAnnotation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3DX11FFT {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DX11FFT {}
impl ::core::fmt::Debug for ID3DX11FFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DX11FFT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3DX11Scan {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DX11Scan {}
impl ::core::fmt::Debug for ID3DX11Scan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DX11Scan").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3DX11SegmentedScan {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DX11SegmentedScan {}
impl ::core::fmt::Debug for ID3DX11SegmentedScan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DX11SegmentedScan").field(&self.0).finish()
    }
}
