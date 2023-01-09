impl ::core::default::Default for AUTHENTICATION_REQUIREMENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTHENTICATION_REQUIREMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATION_REQUIREMENTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BLUETOOTH_AUTHENTICATE_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BLUETOOTH_AUTHENTICATION_CALLBACK_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BLUETOOTH_AUTHENTICATION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BLUETOOTH_AUTHENTICATION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BLUETOOTH_AUTHENTICATION_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_AUTHENTICATION_REQUIREMENTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BLUETOOTH_AUTHENTICATION_REQUIREMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BLUETOOTH_AUTHENTICATION_REQUIREMENTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_COD_PAIRS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_COD_PAIRS {
    fn eq(&self, other: &Self) -> bool {
        self.ulCODMask == other.ulCODMask && self.pcszDescription == other.pcszDescription
    }
}
impl ::core::cmp::Eq for BLUETOOTH_COD_PAIRS {}
impl ::core::fmt::Debug for BLUETOOTH_COD_PAIRS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_COD_PAIRS").field("ulCODMask", &self.ulCODMask).field("pcszDescription", &self.pcszDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BLUETOOTH_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BLUETOOTH_DEVICE_SEARCH_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BLUETOOTH_DEVICE_SEARCH_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.fReturnAuthenticated == other.fReturnAuthenticated && self.fReturnRemembered == other.fReturnRemembered && self.fReturnUnknown == other.fReturnUnknown && self.fReturnConnected == other.fReturnConnected && self.fIssueInquiry == other.fIssueInquiry && self.cTimeoutMultiplier == other.cTimeoutMultiplier && self.hRadio == other.hRadio
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BLUETOOTH_DEVICE_SEARCH_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BLUETOOTH_DEVICE_SEARCH_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_DEVICE_SEARCH_PARAMS").field("dwSize", &self.dwSize).field("fReturnAuthenticated", &self.fReturnAuthenticated).field("fReturnRemembered", &self.fReturnRemembered).field("fReturnUnknown", &self.fReturnUnknown).field("fReturnConnected", &self.fReturnConnected).field("fIssueInquiry", &self.fIssueInquiry).field("cTimeoutMultiplier", &self.cTimeoutMultiplier).field("hRadio", &self.hRadio).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_FIND_RADIO_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_FIND_RADIO_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
    }
}
impl ::core::cmp::Eq for BLUETOOTH_FIND_RADIO_PARAMS {}
impl ::core::fmt::Debug for BLUETOOTH_FIND_RADIO_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_FIND_RADIO_PARAMS").field("dwSize", &self.dwSize).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_GATT_VALUE_CHANGED_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_GATT_VALUE_CHANGED_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.ChangedAttributeHandle == other.ChangedAttributeHandle && self.CharacteristicValueDataSize == other.CharacteristicValueDataSize && self.CharacteristicValue == other.CharacteristicValue
    }
}
impl ::core::cmp::Eq for BLUETOOTH_GATT_VALUE_CHANGED_EVENT {}
impl ::core::fmt::Debug for BLUETOOTH_GATT_VALUE_CHANGED_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_GATT_VALUE_CHANGED_EVENT").field("ChangedAttributeHandle", &self.ChangedAttributeHandle).field("CharacteristicValueDataSize", &self.CharacteristicValueDataSize).field("CharacteristicValue", &self.CharacteristicValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BLUETOOTH_GATT_VALUE_CHANGED_EVENT_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BLUETOOTH_IO_CAPABILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BLUETOOTH_IO_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BLUETOOTH_IO_CAPABILITY").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BLUETOOTH_LOCAL_SERVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BLUETOOTH_NUMERIC_COMPARISON_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_NUMERIC_COMPARISON_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumericValue == other.NumericValue
    }
}
impl ::core::cmp::Eq for BLUETOOTH_NUMERIC_COMPARISON_INFO {}
impl ::core::fmt::Debug for BLUETOOTH_NUMERIC_COMPARISON_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_NUMERIC_COMPARISON_INFO").field("NumericValue", &self.NumericValue).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_OOB_DATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_OOB_DATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.C == other.C && self.R == other.R
    }
}
impl ::core::cmp::Eq for BLUETOOTH_OOB_DATA_INFO {}
impl ::core::fmt::Debug for BLUETOOTH_OOB_DATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_OOB_DATA_INFO").field("C", &self.C).field("R", &self.R).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_PASSKEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_PASSKEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.passkey == other.passkey
    }
}
impl ::core::cmp::Eq for BLUETOOTH_PASSKEY_INFO {}
impl ::core::fmt::Debug for BLUETOOTH_PASSKEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_PASSKEY_INFO").field("passkey", &self.passkey).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_PIN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLUETOOTH_PIN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pin == other.pin && self.pinLength == other.pinLength
    }
}
impl ::core::cmp::Eq for BLUETOOTH_PIN_INFO {}
impl ::core::fmt::Debug for BLUETOOTH_PIN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLUETOOTH_PIN_INFO").field("pin", &self.pin).field("pinLength", &self.pinLength).finish()
    }
}
impl ::core::default::Default for BLUETOOTH_RADIO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BLUETOOTH_SELECT_DEVICE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BTH_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.address == other.address && self.classOfDevice == other.classOfDevice && self.name == other.name
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BTH_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BTH_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BTH_DEVICE_INFO").field("flags", &self.flags).field("address", &self.address).field("classOfDevice", &self.classOfDevice).field("name", &self.name).finish()
    }
}
impl ::core::default::Default for BTH_HCI_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BTH_HCI_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bthAddress == other.bthAddress && self.connectionType == other.connectionType && self.connected == other.connected
    }
}
impl ::core::cmp::Eq for BTH_HCI_EVENT_INFO {}
impl ::core::fmt::Debug for BTH_HCI_EVENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BTH_HCI_EVENT_INFO").field("bthAddress", &self.bthAddress).field("connectionType", &self.connectionType).field("connected", &self.connected).finish()
    }
}
impl ::core::default::Default for BTH_INFO_REQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_INFO_RSP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_L2CAP_EVENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BTH_L2CAP_EVENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.bthAddress == other.bthAddress && self.psm == other.psm && self.connected == other.connected && self.initiated == other.initiated
    }
}
impl ::core::cmp::Eq for BTH_L2CAP_EVENT_INFO {}
impl ::core::fmt::Debug for BTH_L2CAP_EVENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BTH_L2CAP_EVENT_INFO").field("bthAddress", &self.bthAddress).field("psm", &self.psm).field("connected", &self.connected).field("initiated", &self.initiated).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_LE_GATT_CHARACTERISTIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_LE_GATT_CHARACTERISTIC_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BTH_LE_GATT_CHARACTERISTIC_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.DataSize == other.DataSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for BTH_LE_GATT_CHARACTERISTIC_VALUE {}
impl ::core::fmt::Debug for BTH_LE_GATT_CHARACTERISTIC_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BTH_LE_GATT_CHARACTERISTIC_VALUE").field("DataSize", &self.DataSize).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_LE_GATT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_LE_GATT_DESCRIPTOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BTH_LE_GATT_DESCRIPTOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BTH_LE_GATT_DESCRIPTOR_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_LE_GATT_DESCRIPTOR_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_LE_GATT_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BTH_LE_GATT_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BTH_LE_GATT_EVENT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_LE_GATT_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_LE_UUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_PING_REQ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_PING_RSP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BTH_PING_RSP {
    fn eq(&self, other: &Self) -> bool {
        self.dataLen == other.dataLen && self.data == other.data
    }
}
impl ::core::cmp::Eq for BTH_PING_RSP {}
impl ::core::fmt::Debug for BTH_PING_RSP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BTH_PING_RSP").field("dataLen", &self.dataLen).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for BTH_QUERY_DEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BTH_QUERY_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_RADIO_IN_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BTH_RADIO_IN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.deviceInfo == other.deviceInfo && self.previousDeviceFlags == other.previousDeviceFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BTH_RADIO_IN_RANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BTH_RADIO_IN_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BTH_RADIO_IN_RANGE").field("deviceInfo", &self.deviceInfo).field("previousDeviceFlags", &self.previousDeviceFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BTH_SET_SERVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IO_CAPABILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IO_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IO_CAPABILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for NodeContainerType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NodeContainerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NodeContainerType").field(&self.0).finish()
    }
}
impl ::core::default::Default for RFCOMM_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RFCOMM_MSC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFCOMM_MSC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Signals == other.Signals && self.Break == other.Break
    }
}
impl ::core::cmp::Eq for RFCOMM_MSC_DATA {}
impl ::core::fmt::Debug for RFCOMM_MSC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RFCOMM_MSC_DATA").field("Signals", &self.Signals).field("Break", &self.Break).finish()
    }
}
impl ::core::default::Default for RFCOMM_RLS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFCOMM_RLS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.LineStatus == other.LineStatus
    }
}
impl ::core::cmp::Eq for RFCOMM_RLS_DATA {}
impl ::core::fmt::Debug for RFCOMM_RLS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RFCOMM_RLS_DATA").field("LineStatus", &self.LineStatus).finish()
    }
}
impl ::core::default::Default for RFCOMM_RPN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RFCOMM_RPN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Baud == other.Baud && self.Data == other.Data && self.FlowControl == other.FlowControl && self.XonChar == other.XonChar && self.XoffChar == other.XoffChar && self.ParameterMask1 == other.ParameterMask1 && self.ParameterMask2 == other.ParameterMask2
    }
}
impl ::core::cmp::Eq for RFCOMM_RPN_DATA {}
impl ::core::fmt::Debug for RFCOMM_RPN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RFCOMM_RPN_DATA").field("Baud", &self.Baud).field("Data", &self.Data).field("FlowControl", &self.FlowControl).field("XonChar", &self.XonChar).field("XoffChar", &self.XoffChar).field("ParameterMask1", &self.ParameterMask1).field("ParameterMask2", &self.ParameterMask2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SDP_ELEMENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SDP_LARGE_INTEGER_16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SDP_LARGE_INTEGER_16 {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for SDP_LARGE_INTEGER_16 {}
impl ::core::fmt::Debug for SDP_LARGE_INTEGER_16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDP_LARGE_INTEGER_16").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::core::default::Default for SDP_SPECIFICTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SDP_SPECIFICTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SDP_SPECIFICTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SDP_STRING_TYPE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SDP_STRING_TYPE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.encoding == other.encoding && self.mibeNum == other.mibeNum && self.attributeId == other.attributeId
    }
}
impl ::core::cmp::Eq for SDP_STRING_TYPE_DATA {}
impl ::core::fmt::Debug for SDP_STRING_TYPE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDP_STRING_TYPE_DATA").field("encoding", &self.encoding).field("mibeNum", &self.mibeNum).field("attributeId", &self.attributeId).finish()
    }
}
impl ::core::default::Default for SDP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SDP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SDP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SDP_ULARGE_INTEGER_16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SDP_ULARGE_INTEGER_16 {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for SDP_ULARGE_INTEGER_16 {}
impl ::core::fmt::Debug for SDP_ULARGE_INTEGER_16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SDP_ULARGE_INTEGER_16").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::core::default::Default for SOCKADDR_BTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SdpAttributeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SdpAttributeRange {
    fn eq(&self, other: &Self) -> bool {
        self.minAttribute == other.minAttribute && self.maxAttribute == other.maxAttribute
    }
}
impl ::core::cmp::Eq for SdpAttributeRange {}
impl ::core::fmt::Debug for SdpAttributeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SdpAttributeRange").field("minAttribute", &self.minAttribute).field("maxAttribute", &self.maxAttribute).finish()
    }
}
impl ::core::default::Default for SdpQueryUuid {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SdpQueryUuidUnion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
